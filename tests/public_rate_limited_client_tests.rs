mod resources;

use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};

use crate::resources::test_auth::get_null_secrets_provider;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::response_types::VerificationTier::{Intermediate, Pro};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::{pause, Instant};

use crate::resources::test_client::test_client_impl::TestClient;
use kraken_async_rs::clients::rate_limited_kraken_client::RateLimitedKrakenClient;
use kraken_async_rs::request_types::{
    AssetInfoRequestBuilder, CandlestickInterval, OHLCRequest, OrderbookRequest,
    RecentSpreadsRequest, RecentTradesRequest, StringCSV, TickerRequest, TradableAssetPairsRequest,
};

type TestRateLimitedClient<'a> = RateLimitedKrakenClient<TestClient>;

#[tokio::test]
async fn test_system_public_endpoints() {
    pause();
    let n_calls = 7;

    // n calls are expected to take just over ~n-1 seconds to complete
    test_rate_limited_endpoint!(get_server_time, n_calls, n_calls - 1, n_calls, Intermediate);

    test_rate_limited_endpoint!(
        get_system_status,
        n_calls,
        n_calls - 1,
        n_calls,
        Intermediate
    );
}

#[tokio::test]
async fn test_get_asset_info() {
    pause();
    let n_calls = 7;

    let pairs = StringCSV::new(vec![
        "XBT".to_string(),
        "ETH".to_string(),
        "ZUSD".to_string(),
    ]);
    let request = AssetInfoRequestBuilder::new()
        .asset(pairs)
        .asset_class("currency".into())
        .build();

    // n calls are expected to take just over ~n-1 seconds to complete
    test_rate_limited_endpoint!(
        get_asset_info,
        n_calls,
        n_calls - 1,
        n_calls,
        Intermediate,
        &request
    );
}

#[tokio::test]
async fn test_get_tradable_asset_pairs() {
    pause();
    let n_calls = 7;

    let pairs = StringCSV::new(vec!["ETHUSD".to_string()]);
    let request = TradableAssetPairsRequest::builder().pair(pairs).build();

    // n calls are expected to take just over ~n-1 seconds to complete
    test_rate_limited_endpoint!(
        get_tradable_asset_pairs,
        n_calls,
        n_calls - 1,
        n_calls,
        Intermediate,
        &request
    );
}

#[tokio::test]
async fn test_get_ticker_information() {
    pause();
    let n_calls = 7;

    let pairs = StringCSV::new(vec![
        "BTCUSD".to_string(),
        "ETHUSD".to_string(),
        "USDCUSD".to_string(),
    ]);
    let request = TickerRequest::builder().pair(pairs).build();

    // n calls are expected to take just over ~n-1 seconds to complete
    test_rate_limited_endpoint!(
        get_ticker_information,
        n_calls,
        n_calls - 1,
        n_calls,
        Intermediate,
        &request
    );
}

#[tokio::test]
async fn test_get_ohlc_and_recent_trades() {
    pause();
    let n_calls = 7;

    let ohlc_request = OHLCRequest::builder("XETHZUSD".to_string())
        .interval(CandlestickInterval::Hour)
        .build();

    let trades_request = RecentTradesRequest::builder("XXBTZUSD".to_string())
        .count(10)
        .build();

    let secrets_provider = get_null_secrets_provider();
    let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> =
        Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));

    let mut client: TestRateLimitedClient =
        RateLimitedKrakenClient::new_with_verification_tier(secrets_provider, nonce_provider, Pro);

    let start = Instant::now();

    // calling both in parallel should be fine, since they request different pairs
    for _ in 0..n_calls {
        let _ = client.get_ohlc(&ohlc_request).await;
        let _ = client.get_recent_trades(&trades_request).await;
    }

    let end = Instant::now();
    let elapsed = end - start;

    println!("{:?}", elapsed);

    assert!(elapsed > Duration::from_secs(n_calls - 1));
    assert!(elapsed < Duration::from_secs(n_calls));
}

#[tokio::test]
async fn test_get_orderbook() {
    pause();
    let n_calls = 7;

    let request = OrderbookRequest::builder("XXBTZUSD".to_string())
        .count(10)
        .build();

    // n calls are expected to take just over ~n-1 seconds to complete
    test_rate_limited_endpoint!(
        get_orderbook,
        n_calls,
        n_calls - 1,
        n_calls,
        Intermediate,
        &request
    );
}

#[tokio::test]
async fn test_get_recent_trades() {
    pause();
    let n_calls = 7;

    let request = RecentTradesRequest::builder("XXBTZUSD".to_string())
        .count(10)
        .build();

    // n calls are expected to take just over ~n-1 seconds to complete
    test_rate_limited_endpoint!(
        get_recent_trades,
        n_calls,
        n_calls - 1,
        n_calls,
        Intermediate,
        &request
    );
}

#[tokio::test]
async fn test_get_recent_spreads() {
    pause();
    let n_calls = 7;

    let request = RecentSpreadsRequest::builder("XXBTZUSD".to_string())
        .since(0)
        .build();
    // n calls are expected to take just over ~n-1 seconds to complete
    test_rate_limited_endpoint!(
        get_recent_spreads,
        n_calls,
        n_calls - 1,
        n_calls,
        Intermediate,
        &request
    );
}
