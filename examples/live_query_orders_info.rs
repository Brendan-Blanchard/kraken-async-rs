use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
use kraken_async_rs::request_types::OrderRequest;
use kraken_async_rs::secrets::secrets_provider::{EnvSecretsProvider, SecretsProvider};
use std::fs::File;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, Registry};

/// This retrieves the user's open orders, provided the API key and secret are available in env vars.
#[tokio::main]
async fn main() {
    set_up_logging("tests.log");
    // note that this will fail if you don't have your key and secret set to these env vars
    // eg `export KRAKEN_KEY="YOUR-API-KEY"`, ...
    let secrets_provider: Box<Arc<Mutex<dyn SecretsProvider>>> = Box::new(Arc::new(Mutex::new(
        EnvSecretsProvider::new("KRAKEN_KEY", "KRAKEN_SECRET"),
    )));

    let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> =
        Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));

    let mut client = CoreKrakenClient::new(secrets_provider, nonce_provider);

    let request = OrderRequest::builder("OUIQRR-FSDDA-JDI8D5".into()).build();

    let response = client.query_orders_info(&request).await;

    for (id, order) in response.unwrap().result.unwrap() {
        println!("{id}: {order:?}");
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
