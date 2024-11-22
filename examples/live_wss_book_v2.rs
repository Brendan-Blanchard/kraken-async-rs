use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::test_support::set_up_logging;
use kraken_async_rs::wss::KrakenWSSClient;
use kraken_async_rs::wss::{BookSubscription, Message, WssMessage};
use std::time::Duration;
use tokio::time::timeout;
use tokio_stream::StreamExt;
use tracing::{info, warn};

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
