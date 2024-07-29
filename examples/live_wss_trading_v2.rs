use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
use kraken_async_rs::crypto::secrets::Token;
use kraken_async_rs::request_types::{SelfTradePrevention, TimeInForceV2};
use kraken_async_rs::response_types::{BuySell, OrderType};
use kraken_async_rs::secrets::secrets_provider::{EnvSecretsProvider, SecretsProvider};
use kraken_async_rs::wss::v2::base_messages::{Message, MethodMessage, ResultResponse, WssMessage};
use kraken_async_rs::wss::v2::kraken_wss_client::KrakenWSSClient;
use kraken_async_rs::wss::v2::trading_messages::{
    AddOrderParams, AddOrderResult, CancelOrderParams, EditOrderParams, FeePreference,
};
use rust_decimal_macros::dec;
use std::fs::File;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::timeout;
use tokio_stream::StreamExt;
use tracing::{info, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, Registry};

/// This places a new order for 5 USDC at a low price that's unlikely to be filled ($0.95), then
/// edits the order on receipt of its confirmation, and cancels once it's been edited.
#[tokio::main]
async fn main() {
    set_up_logging("wss_trading_v2.log");

    let secrets_provider: Box<Arc<Mutex<dyn SecretsProvider>>> = Box::new(Arc::new(Mutex::new(
        EnvSecretsProvider::new("KRAKEN_KEY", "KRAKEN_SECRET"),
    )));

    let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> =
        Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));
    let mut kraken_client = CoreKrakenClient::new(secrets_provider, nonce_provider);

    let resp = kraken_client.get_websockets_token().await.unwrap();

    let token = resp.result.unwrap().token;

    let mut client = KrakenWSSClient::new();
    let mut kraken_stream = client.connect_auth::<WssMessage>().await.unwrap();

    let new_order = AddOrderParams {
        order_type: OrderType::Limit,
        side: BuySell::Buy,
        symbol: "USDC/USD".to_string(),
        limit_price: Some(dec!(0.95)),
        limit_price_type: None,
        triggers: None,
        time_in_force: Some(TimeInForceV2::GTC),
        order_quantity: dec!(5.0),
        margin: None,
        post_only: Some(true),
        reduce_only: None,
        effective_time: None,
        expire_time: None,
        deadline: None,
        order_user_ref: None,
        conditional: None,
        display_quantity: None,
        fee_preference: Some(FeePreference::Quote),
        no_market_price_protection: None,
        stp_type: Some(SelfTradePrevention::CancelNewest),
        cash_order_quantity: None,
        validate: None,
        token: token.clone(),
    };

    let order_message = Message {
        method: "add_order".to_string(),
        params: new_order,
        req_id: 0,
    };

    let result = kraken_stream.send(&order_message).await;
    assert!(result.is_ok());

    let mut edited = false;

    while let Ok(Some(message)) = timeout(Duration::from_secs(10), kraken_stream.next()).await {
        match message {
            Ok(WssMessage::Method(MethodMessage::AddOrder(response))) => {
                info!("{:?}", response);
                if !edited {
                    edited = true;
                    let order_edit_message =
                        order_edit_from_add_order_result(token.clone(), response);
                    let result = kraken_stream.send(&order_edit_message).await;

                    info!("{:?}", result);
                    assert!(result.is_ok());
                }
            }
            Ok(WssMessage::Method(MethodMessage::EditOrder(response))) => {
                let cancel = CancelOrderParams {
                    order_id: Some(vec![response.result.unwrap().order_id]),
                    client_order_id: None,
                    order_user_ref: None,
                    token: token.clone(),
                };
                let cancel_message = Message {
                    method: "cancel_order".to_string(),
                    params: cancel,
                    req_id: 0,
                };

                let result = kraken_stream.send(&cancel_message).await;

                info!("{:?}", result);
                assert!(result.is_ok());
            }
            Ok(response) => info!("{:?}", response),
            message => warn!("Message failed: {:?}", message),
        }
    }
}

fn order_edit_from_add_order_result(
    token: Token,
    response: ResultResponse<AddOrderResult>,
) -> Message<EditOrderParams> {
    let add_order_result = response.result.unwrap();

    let order_edit = EditOrderParams {
        deadline: None,
        display_quantity: None,
        fee_preference: None,
        limit_price: Some(dec!(0.93)),
        no_market_price_protection: None,
        order_id: add_order_result.order_id,
        order_quantity: Some(dec!(6.1)),
        order_user_ref: None,
        post_only: None,
        reduce_only: None,
        symbol: "USDC/USD".to_string(),
        triggers: None,
        validate: None,
        token,
    };

    Message {
        method: "edit_order".to_string(),
        params: order_edit,
        req_id: 0,
    }
}

fn set_up_logging(filename: &str) {
    let subscriber = Registry::default()
        .with(
            fmt::Layer::default()
                .with_ansi(false)
                .with_writer(get_log_file(filename)),
        )
        .with(fmt::Layer::default().pretty().with_writer(std::io::stdout));

    tracing::subscriber::set_global_default(subscriber).unwrap();
}

fn get_log_file(filename: &str) -> File {
    File::options()
        .append(true)
        .create(true)
        .open(filename)
        .expect("failed to open test log file!")
}
