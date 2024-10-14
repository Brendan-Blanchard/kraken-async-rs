use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
use kraken_async_rs::request_types::{AddOrderRequest, CancelBatchOrdersRequest};
use kraken_async_rs::response_types::{BuySell, OrderFlag, OrderType};
use kraken_async_rs::secrets::secrets_provider::{EnvSecretsProvider, SecretsProvider};
use kraken_async_rs::test_support::set_up_logging;
use rust_decimal_macros::dec;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

/// This places two orders for 5 USDC that are unlikely to be filled and immediately cancels them
/// as a batch.
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

    let mut order_ids = vec![];

    for _ in 0..2 {
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

        order_ids.push(order_id.clone());
    }

    let cancel_batch_request = CancelBatchOrdersRequest::from_tx_ids(order_ids);

    let response = client
        .cancel_order_batch(&cancel_batch_request)
        .await
        .unwrap();

    info!("{:?}", response);

    let count = response.result.unwrap().count;
    assert_eq!(2, count);
}
