use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::clients::rate_limited_kraken_client::RateLimitedKrakenClient;
use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
use kraken_async_rs::request_types::TradesHistoryRequest;
use kraken_async_rs::secrets::secrets_provider::{EnvSecretsProvider, SecretsProvider};
use std::sync::Arc;
use time::{Duration, OffsetDateTime};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let secrets_provider: Box<Arc<Mutex<dyn SecretsProvider>>> = Box::new(Arc::new(Mutex::new(
        EnvSecretsProvider::new("KRAKEN_KEY", "KRAKEN_SECRET"),
    )));

    let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> =
        Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));

    let mut client: RateLimitedKrakenClient<CoreKrakenClient> =
        RateLimitedKrakenClient::new(secrets_provider, nonce_provider);

    let since = OffsetDateTime::now_utc()
        .checked_sub(Duration::hours(24))
        .unwrap()
        .unix_timestamp();

    let request = TradesHistoryRequest::builder().start(since).build();

    let trades = client.get_trades_history(&request).await.unwrap();

    println!("{:?}", trades);
    println!("{:?}", trades.result.unwrap().trades.len());
}
