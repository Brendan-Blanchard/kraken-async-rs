use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
use kraken_async_rs::request_types::{
    AddOrderRequest, CancelOrderRequest, EditOrderRequest, IntOrString, OrderFlags, OrderRequest,
    StringCSV,
};
use kraken_async_rs::response_types::{BuySell, OrderFlag, OrderType};
use kraken_async_rs::secrets::secrets_provider::EnvSecretsProvider;
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
        "5.1234".into(),
        "USDCUSD".into(),
    )
    .price("0.99".to_string())
    .order_flags(OrderFlags::new(vec![OrderFlag::Post]))
    .build();

    let new_order = client
        .add_order(&request)
        .await
        .expect("failed to place order")
        .result
        .unwrap();

    info!("{:?}", new_order);

    let order_query = OrderRequest::builder(StringCSV::new(vec![new_order
        .tx_id
        .first()
        .unwrap()
        .clone()]))
    .build();

    let order_details = client
        .query_orders_info(&order_query)
        .await
        .expect("failed to query order")
        .result
        .unwrap();

    info!("{:?}", order_details);

    let edit_query = EditOrderRequest::builder(
        new_order.tx_id.first().unwrap().clone(),
        "5.0".to_string(),
        "USDCUSD".to_string(),
    )
    .price("0.99".to_string())
    .build();

    let edit = client
        .edit_order(&edit_query)
        .await
        .expect("failed to edit order")
        .result
        .unwrap();

    info!("{:?}", edit);

    let cancel_request =
        CancelOrderRequest::builder(IntOrString::String(edit.tx_id.clone())).build();

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
