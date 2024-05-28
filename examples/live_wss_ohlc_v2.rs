use kraken_async_rs::wss::v2::base_messages::{Message, WssMessage};
use kraken_async_rs::wss::v2::kraken_wss_client::KrakenWSSClient;
use kraken_async_rs::wss::v2::market_data_messages::OhlcSubscription;
use std::fs::File;
use std::time::Duration;
use tokio::time::timeout;
use tokio_stream::StreamExt;
use tracing::{info, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, Registry};

#[tokio::main]
async fn main() {
    set_up_logging("wss_ohlc_v2.log");

    let mut client = KrakenWSSClient::new();
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
