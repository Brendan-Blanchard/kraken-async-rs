use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
use kraken_async_rs::request_types::{
    AddOrderRequest, CancelOrderRequest, EditOrderRequest, OrderRequest,
};
use kraken_async_rs::response_types::{BuySell, OrderFlag, OrderType};
use kraken_async_rs::secrets::secrets_provider::EnvSecretsProvider;
use rust_decimal_macros::dec;
use std::fs::File;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, Registry};

/// This places an order, queries, edits, then cancels it.
///
/// Of course, *don't use expect and unwrap in production code*, unless you want it to fail.
#[tokio::main]
async fn main() {
    set_up_logging("trading.log");
    // note that this will fail if you don't have your key and secret set to these env vars
    // eg `export KRAKEN_KEY="YOUR-API-KEY"`, ...
    let secrets_provider = Box::new(EnvSecretsProvider::new("KRAKEN_KEY", "KRAKEN_SECRET"));
    let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> =
        Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));

    let mut client = CoreKrakenClient::new(secrets_provider, nonce_provider);

    let request = AddOrderRequest::builder(
        OrderType::Limit,
        BuySell::Buy,
        dec!(5.1234),
        "USDCUSD".into(),
    )
    .price(dec!(0.99))
    // individual OrderFlag variants have a From<OrderFlag> for OrderFlags conversion for convenience
    .order_flags(OrderFlag::Post.into())
    .build();

    let new_order = client
        .add_order(&request)
        .await
        .expect("failed to place order")
        .result
        .unwrap();

    info!("{:?}", new_order);

    let order_id = new_order.tx_id.first().unwrap();

    // there's an impl From<&str/&String/String> conversion for StringCSV when requesting a single id
    let order_query = OrderRequest::builder(order_id.into()).build();

    let order_details = client
        .query_orders_info(&order_query)
        .await
        .expect("failed to query order")
        .result
        .unwrap();

    info!("{:?}", order_details);

    let edit_query = EditOrderRequest::builder(order_id.clone(), dec!(5.0), "USDCUSD".to_string())
        .price(dec!(0.99))
        .build();

    let edit = client
        .edit_order(&edit_query)
        .await
        .expect("failed to edit order")
        .result
        .unwrap();

    info!("{:?}", edit);

    // edit.tx_id.into() uses a convenience impl From<String> for IntOrString
    let cancel_request = CancelOrderRequest::builder(edit.tx_id.into()).build();

    let cancel = client
        .cancel_order(&cancel_request)
        .await
        .unwrap()
        .result
        .unwrap();

    info!("{:?}", cancel);

    assert_eq!(1, cancel.count);
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
