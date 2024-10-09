use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
use kraken_async_rs::secrets::secrets_provider::EnvSecretsProvider;
use kraken_async_rs::test_support::set_up_logging;
use kraken_async_rs::wss::v2::base_messages::{Message, WssMessage};
use kraken_async_rs::wss::v2::kraken_wss_client::KrakenWSSClient;
use kraken_async_rs::wss::v2::market_data_messages::BookSubscription;
use kraken_async_rs::wss::v2::user_data_messages::{BalancesSubscription, ExecutionSubscription};
use std::fs::File;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::timeout;
use tokio_stream::StreamExt;
use tracing::{info, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, Registry};

#[tokio::main]
async fn main() {
    set_up_logging("wss_book_v2.log");

    let mut client = KrakenWSSClient::new();
    let mut kraken_stream = client.connect::<WssMessage>().await.unwrap();

    let mut book_params = BookSubscription::new(vec!["BTC/USD".into()]);
    book_params.snapshot = Some(true);
    let subscription = Message::new_subscription(book_params, 0);

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
