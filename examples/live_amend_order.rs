use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
use kraken_async_rs::request_types::{
    AddOrderRequest, AmendOrderRequest, CancelOrderRequest, EditOrderRequest, OrderRequest,
};
use kraken_async_rs::response_types::{BuySell, OrderFlag, OrderType};
use kraken_async_rs::secrets::secrets_provider::{EnvSecretsProvider, SecretsProvider};
use kraken_async_rs::test_support::set_up_logging;
use rust_decimal_macros::dec;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

/// This places an order, queries, amends, then cancels it.
///
/// Of course, *don't use expect and unwrap in production code*, unless you want it to fail.
#[tokio::main]
async fn main() {
    set_up_logging("trading.log");
    // note that this will fail if you don't have your key and secret set to these env vars
    // eg `export KRAKEN_KEY="YOUR-API-KEY"`, ...
    let secrets_provider: Box<Arc<Mutex<dyn SecretsProvider>>> = Box::new(Arc::new(Mutex::new(
        EnvSecretsProvider::new("KRAKEN_KEY", "KRAKEN_SECRET"),
    )));

    let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> =
        Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));

    let mut client = CoreKrakenClient::new(secrets_provider, nonce_provider);

    let request = AddOrderRequest::builder(
        OrderType::Limit,
        BuySell::Buy,
        dec!(5.1234),
        "USDCUSD".into(),
    )
    .price(dec!(0.95))
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

    let order_query = OrderRequest::builder(order_id.into()).build();

    let order_details = client
        .query_orders_info(&order_query)
        .await
        .expect("failed to query order")
        .result
        .unwrap();

    info!("{:?}", order_details);

    let amend_request = AmendOrderRequest::builder()
        .tx_id(order_id.clone())
        .order_qty(dec!(5.25))
        .limit_price(dec!(0.96).to_string())
        .post_only(true)
        .build();

    let amended_order = client
        .amend_order(&amend_request)
        .await
        .expect("failed to amend order")
        .result
        .unwrap();

    info!("{:?}", amended_order);

    let cancel_request = CancelOrderRequest::builder(order_id.clone().into()).build();

    let cancel = client
        .cancel_order(&cancel_request)
        .await
        .unwrap()
        .result
        .unwrap();

    info!("{:?}", cancel);

    assert_eq!(1, cancel.count);
}
