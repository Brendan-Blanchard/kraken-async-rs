use kraken_async_rs::test_support::set_up_logging;
use kraken_async_rs::wss::v2::base_messages::{Message, WssMessage};
use kraken_async_rs::wss::v2::kraken_wss_client::KrakenWSSClient;
use kraken_async_rs::wss::v2::market_data_messages::OhlcSubscription;
use std::time::Duration;
use tokio::time::timeout;
use tokio_stream::StreamExt;
use tracing::{info, warn};

#[tokio::main]
async fn main() {
    set_up_logging("wss_ohlc_v2.log");

    let mut client = KrakenWSSClient::new_with_tracing(true, true);
    let mut kraken_stream = client.connect::<WssMessage>().await.unwrap();

    let ohlc_params = OhlcSubscription::new(vec!["ETH/USD".into()], 60);

    let subscription = Message::new_subscription(ohlc_params, 0);

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
