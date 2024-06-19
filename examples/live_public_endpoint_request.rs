use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
use kraken_async_rs::clients::http_response_types::ResultErrorResponse;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
use kraken_async_rs::request_types::TradableAssetPairsRequest;
use kraken_async_rs::secrets::secrets_provider::{SecretsProvider, StaticSecretsProvider};
use std::sync::Arc;
use tokio::sync::Mutex;

/// This retrieves the asset pair details for BTC-USD, showing a simple public endpoint request.
#[tokio::main]
async fn main() {
    // credentials aren't needed for public endpoints
    let secrets_provider: Box<Arc<Mutex<dyn SecretsProvider>>> =
        Box::new(Arc::new(Mutex::new(StaticSecretsProvider::new("", ""))));
    let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> =
        Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));
    let mut client = CoreKrakenClient::new(secrets_provider, nonce_provider);

    let request = TradableAssetPairsRequest::builder()
        .pair("BTCUSD".into())
        .build();

    let response = client.get_tradable_asset_pairs(&request).await;

    // Note that Kraken will return assets in their own naming scheme, e.g. a request for
    // "BTCUSD" will return as "XXBTZUSD"
    // For a reasonable understanding of their mappings, see: https://gist.github.com/brendano257/975a395d73a6d7bb53e53d292534d6af
    if let Ok(ResultErrorResponse {
        result: Some(tradable_assets),
        ..
    }) = response
    {
        for (asset, details) in tradable_assets {
            println!("{asset}: {details:?}")
        }
    }
}
