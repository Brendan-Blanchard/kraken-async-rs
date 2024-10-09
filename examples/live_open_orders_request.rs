use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
use kraken_async_rs::clients::http_response_types::ResultErrorResponse;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
use kraken_async_rs::request_types::OpenOrdersRequest;
use kraken_async_rs::secrets::secrets_provider::{EnvSecretsProvider, SecretsProvider};
use kraken_async_rs::test_support::set_up_logging;
use std::sync::Arc;
use tokio::sync::Mutex;

/// This retrieves the user's open orders, provided the API key and secret are available in env vars.
#[tokio::main]
async fn main() {
    set_up_logging("open_orders_request.log");
    // note that this will fail if you don't have your key and secret set to these env vars
    // eg `export KRAKEN_KEY="YOUR-API-KEY"`, ...
    let secrets_provider: Box<Arc<Mutex<dyn SecretsProvider>>> = Box::new(Arc::new(Mutex::new(
        EnvSecretsProvider::new("KRAKEN_KEY", "KRAKEN_SECRET"),
    )));

    let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> =
        Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));

    let mut client = CoreKrakenClient::new(secrets_provider, nonce_provider);

    let request = OpenOrdersRequest::builder().build();

    let open_orders_response = client.get_open_orders(&request).await;

    if let Ok(ResultErrorResponse {
        result: Some(open_orders),
        ..
    }) = open_orders_response
    {
        for (order_id, order) in open_orders.open {
            println!("{order_id}: {order:?}")
        }
    }
}
