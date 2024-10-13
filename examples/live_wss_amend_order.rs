use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
use kraken_async_rs::crypto::secrets::Token;
use kraken_async_rs::request_types::{SelfTradePrevention, TimeInForceV2};
use kraken_async_rs::response_types::{BuySell, OrderType};
use kraken_async_rs::secrets::secrets_provider::{EnvSecretsProvider, SecretsProvider};
use kraken_async_rs::test_support::set_up_logging;
use kraken_async_rs::wss::v2::base_messages::{Message, MethodMessage, ResultResponse, WssMessage};
use kraken_async_rs::wss::v2::kraken_wss_client::KrakenWSSClient;
use kraken_async_rs::wss::v2::trading_messages::{
    AddOrderParams, AddOrderResult, AmendOrderParams, CancelOrderParams, EditOrderParams,
    FeePreference,
};
use rust_decimal_macros::dec;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::timeout;
use tokio_stream::StreamExt;
use tracing::{info, warn};

/// This places a new order for 5 USDC at a low price that's unlikely to be filled ($0.95), then
/// amends the order on receipt of its confirmation, and cancels once it's been amended.
#[tokio::main]
async fn main() {
    set_up_logging("wss_trading_amend_order_v2.log");

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
        sender_sub_id: None,
        token: token.clone(),
        client_order_id: None,
    };

    let order_message = Message {
        method: "add_order".to_string(),
        params: new_order,
        req_id: 0,
    };

    let result = kraken_stream.send(&order_message).await;
    assert!(result.is_ok());

    let mut amended = false;

    while let Ok(Some(message)) = timeout(Duration::from_secs(10), kraken_stream.next()).await {
        match message {
            Ok(WssMessage::Method(MethodMessage::AddOrder(response))) => {
                info!("{:?}", response);
                if !amended {
                    amended = true;

                    let amend_params = AmendOrderParams::builder(dec!(5.1), token.clone())
                        .post_only(true)
                        .order_id(response.result.unwrap().order_id)
                        .limit_price(dec!(0.96))
                        .build();

                    let message = Message {
                        method: "amend_order".to_string(),
                        params: amend_params,
                        req_id: 0,
                    };

                    let result = kraken_stream.send(&message).await;

                    info!("{:?}", result);
                    assert!(result.is_ok());
                }
            }
            Ok(WssMessage::Method(MethodMessage::AmendOrder(response))) => {
                let cancel = CancelOrderParams {
                    order_id: Some(vec![response.result.unwrap().order_id.unwrap()]),
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
