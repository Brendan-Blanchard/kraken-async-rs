use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::wss::kraken_wss_client::{KrakenMessageStream, KrakenWSSClient};
use kraken_async_rs::wss::public::messages::PublicMessage;
use kraken_async_rs::wss::subscribe_messages::{SubscribeMessage, Subscription};
use std::time::Duration;
use tokio::time::timeout;
use tokio_stream::StreamExt;

/// This subscribes to all trades for BTC/USD and ETH/USD.
///
/// Assuming no errors, this listens indefinitely since Heartbeats will prevent the 10-second timeout.
#[tokio::main]
async fn main() {
    let mut client = KrakenWSSClient::new();
    let mut kraken_stream: KrakenMessageStream<PublicMessage> = client.connect().await.unwrap();

    let trades_subscription = Subscription::new_trades_subscription();
    let subscribe_message = SubscribeMessage::new(
        0,
        Some(vec!["BTC/USD".into(), "ETH/USD".into()]),
        trades_subscription,
    );

    kraken_stream.subscribe(&subscribe_message).await.unwrap();

    while let Ok(Some(message)) = timeout(Duration::from_secs(10), kraken_stream.next()).await {
        if let Ok(response) = message {
            println!("{:?}", response);
        } else {
            println!("Message failed: {:?}", message);
        }
    }
}
