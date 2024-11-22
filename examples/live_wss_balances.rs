use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
use kraken_async_rs::secrets::secrets_provider::{EnvSecretsProvider, SecretsProvider};
use kraken_async_rs::test_support::set_up_logging;
use kraken_async_rs::wss::KrakenWSSClient;
use kraken_async_rs::wss::{BalancesSubscription, Message, WssMessage};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::timeout;
use tokio_stream::StreamExt;
use tracing::{info, warn};

/// This example creates a `CoreKrakenClient` to retrieve a websocket token, then subscribes to
/// the Balances channel, listening for a snapshot and any updates to balances.
#[tokio::main]
async fn main() {
    set_up_logging("wss_balances.log");

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

    let balances_params = BalancesSubscription::new(token);
    let subscription = Message::new_subscription(balances_params, 0);

    let result = kraken_stream.send(&subscription).await;
    assert!(result.is_ok());

    while let Ok(Some(message)) = timeout(Duration::from_secs(10), kraken_stream.next()).await {
        if let Ok(response) = message {
            info!("{:?}", response);
        } else {
            warn!("Message failed: {:?}", message);
        }
    }
}
