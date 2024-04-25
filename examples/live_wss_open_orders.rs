use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
use kraken_async_rs::secrets::secrets_provider::EnvSecretsProvider;
use kraken_async_rs::wss::kraken_wss_client::{KrakenMessageStream, KrakenWSSClient};
use kraken_async_rs::wss::private::messages::PrivateMessage;
use kraken_async_rs::wss::subscribe_messages::{SubscribeMessage, Subscription};
use std::fs::File;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::timeout;
use tokio_stream::StreamExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, Registry};

/// This creates a [CoreKrakenClient] to retrieve a websocket token, then subscribes to the OpenOrders
/// channel to receive a snapshot of all current orders and get updates on existing orders.
///
/// Assuming no errors, this listens indefinitely since Heartbeats will prevent the 10-second timeout.
#[tokio::main]
async fn main() {
    set_up_logging("live_open_orders.log");

    let secrets_provider = Box::new(EnvSecretsProvider::new("KRAKEN_KEY", "KRAKEN_SECRET"));
    let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> =
        Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));
    let mut kraken_client = CoreKrakenClient::new(secrets_provider, nonce_provider);

    let resp = kraken_client.get_websockets_token().await.unwrap();

    let token = resp.result.unwrap().token;

    let open_orders_subscription = Subscription::new_open_orders_subscription(token, None);
    let subscribe_message = SubscribeMessage::new(0, None, open_orders_subscription);

    let mut client = KrakenWSSClient::new();
    let mut kraken_stream: KrakenMessageStream<PrivateMessage> =
        client.connect_auth().await.unwrap();

    kraken_stream
        .subscribe(&subscribe_message)
        .await
        .expect("failed to send subscribe message");

    while let Some(message) = kraken_stream.next().await {
        if let Ok(response) = message {
            println!("{:?}", response);
        } else {
            println!("Message failed: {:?}", message);
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
