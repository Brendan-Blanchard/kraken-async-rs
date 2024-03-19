use kraken_async_rs::wss::kraken_wss_client::{KrakenMessageStream, KrakenWSSClient};
use kraken_async_rs::wss::public::messages::PublicMessage;
use kraken_async_rs::wss::subscribe_messages::{SubscribeMessage, Subscription};
use std::fs::File;
use tokio_stream::StreamExt;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, Registry};

/// This shows setting up a log file, and listening to all public websocket streams as one.
///
/// *Warning: the log file can grow quite large if this is left running!*
#[tokio::main]
async fn main() {
    set_up_logging("kraken_api.log");

    let subscriptions = [
        // many of these have additional options that could be set
        Subscription::new_trades_subscription(),
        Subscription::new_book_subscription(Some(10)), // use a depth of 10 for simplicity
        Subscription::new_ticker_subscription(),
        Subscription::new_ohlc_subscription(None),
        Subscription::new_spread_subscription(),
    ];

    let mut client = KrakenWSSClient::new();
    let mut kraken_stream: KrakenMessageStream<PublicMessage> = client.connect().await.unwrap();

    for subscription in subscriptions {
        // for more valid pairs for WSS requests, consult the `ws_name` field of `get_tradable_asset_pairs`
        let subscribe_message =
            SubscribeMessage::new(0, Some(vec!["XBT/USD".into()]), subscription);
        kraken_stream.subscribe(&subscribe_message).await.unwrap();
    }

    while let Some(message) = kraken_stream.next().await {
        info!("{:?}", message.unwrap());
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
