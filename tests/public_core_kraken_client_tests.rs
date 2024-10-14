use crate::resources::test_auth::get_null_secrets_provider;

use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};

use std::sync::Arc;
use tokio::sync::Mutex;

mod resources;

use crate::resources::kraken_responses::errors::{
    ERROR_INVALID_KEY, ERROR_PERMISSION_DENIED, ERROR_UNKNOWN_ASSET_PAIR,
};
use crate::resources::kraken_responses::public_response_json::{
    get_asset_info_json, get_ohlc_data_json, get_orderbook_json, get_recent_spreads_json,
    get_recent_trades_json, get_server_time_json, get_system_status_json,
    get_ticker_information_json, get_tradable_asset_pairs_json,
};
use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
use kraken_async_rs::clients::errors::ClientError;
use kraken_async_rs::clients::errors::KrakenError;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::request_types::{
    AssetInfoRequestBuilder, CancelBatchOrdersRequest, CandlestickInterval, IntOrString,
    OHLCRequest, OrderbookRequest, RecentSpreadsRequest, RecentTradesRequest, StringCSV,
    TickerRequest, TradableAssetPairsRequest,
};
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_server_time() {
    let secrets_provider = get_null_secrets_provider();
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/0/public/Time"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_server_time_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, get_server_time);
}

#[tokio::test]
async fn test_get_system_status() {
    let secrets_provider = get_null_secrets_provider();
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("0/public/SystemStatus"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_system_status_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, get_system_status);
}

#[tokio::test]
async fn test_get_asset_info() {
    let secrets_provider = get_null_secrets_provider();
    let pairs = StringCSV::new(vec![
        "XBT".to_string(),
        "ETH".to_string(),
        "ZUSD".to_string(),
    ]);
    let request = AssetInfoRequestBuilder::new()
        .asset(pairs)
        .asset_class("currency".into())
        .build();

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/0/public/Assets"))
        .and(query_param("aclass", "currency"))
        .and(query_param("asset", "XBT,ETH,ZUSD"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_asset_info_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, get_asset_info, &request);
}

#[tokio::test]
async fn test_get_tradable_asset_pairs() {
    let secrets_provider = get_null_secrets_provider();
    let mock_server = MockServer::start().await;

    let pairs = StringCSV::new(vec!["ETHUSD".to_string()]);
    let request = TradableAssetPairsRequest::builder()
        .pair(pairs)
        .country_code("US:TX".to_string())
        .build();

    Mock::given(method("GET"))
        .and(path("/0/public/AssetPairs"))
        .and(query_param("pair", "ETHUSD"))
        .and(query_param("country_code", "US:TX"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_tradable_asset_pairs_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(
        secrets_provider,
        mock_server,
        get_tradable_asset_pairs,
        &request
    );
}

#[tokio::test]
async fn test_get_ticker_information() {
    let secrets_provider = get_null_secrets_provider();
    let mock_server = MockServer::start().await;

    let pairs = StringCSV::new(vec![
        "BTCUSD".to_string(),
        "ETHUSD".to_string(),
        "USDCUSD".to_string(),
    ]);
    let request = TickerRequest::builder().pair(pairs).build();

    Mock::given(method("GET"))
        .and(path("0/public/Ticker"))
        .and(query_param("pair", "BTCUSD,ETHUSD,USDCUSD"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_ticker_information_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(
        secrets_provider,
        mock_server,
        get_ticker_information,
        &request
    );
}

#[tokio::test]
async fn test_get_ohlc_data() {
    let secrets_provider = get_null_secrets_provider();
    let mock_server = MockServer::start().await;

    let request = OHLCRequest::builder("BTCUSD".to_string())
        .interval(CandlestickInterval::Hour)
        .build();

    Mock::given(method("GET"))
        .and(path("0/public/OHLC"))
        .and(query_param("pair", "BTCUSD"))
        .and(query_param("interval", "60"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_ohlc_data_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, get_ohlc, &request);
}

#[tokio::test]
async fn test_get_orderbook() {
    let secrets_provider = get_null_secrets_provider();
    let mock_server = MockServer::start().await;

    let request = OrderbookRequest::builder("XXBTZUSD".to_string())
        .count(10)
        .build();

    Mock::given(method("GET"))
        .and(path("0/public/Depth"))
        .and(query_param("count", "10"))
        .and(query_param("pair", "XXBTZUSD"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_orderbook_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, get_orderbook, &request);
}

#[tokio::test]
async fn test_get_recent_trades() {
    let secrets_provider = get_null_secrets_provider();
    let mock_server = MockServer::start().await;

    let request = RecentTradesRequest::builder("XXBTZUSD".to_string())
        .count(10)
        .build();

    Mock::given(method("GET"))
        .and(path("0/public/Trades"))
        .and(query_param("count", "10"))
        .and(query_param("pair", "XXBTZUSD"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_recent_trades_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, get_recent_trades, &request);
}

#[tokio::test]
async fn test_get_recent_spreads() {
    let secrets_provider = get_null_secrets_provider();
    let mock_server = MockServer::start().await;

    let request = RecentSpreadsRequest::builder("XXBTZUSD".to_string())
        .since(0)
        .build();

    Mock::given(method("GET"))
        .and(path("0/public/Spread"))
        .and(query_param("since", "0"))
        .and(query_param("pair", "XXBTZUSD"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_recent_spreads_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, get_recent_spreads, &request);
}

#[tokio::test]
async fn test_get_asset_info_error() {
    let pairs = StringCSV::new(vec!["TQQQ".to_string()]);
    let request = AssetInfoRequestBuilder::new()
        .asset(pairs)
        .asset_class("currency".into())
        .build();

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/0/public/Assets"))
        .and(query_param("aclass", "currency"))
        .and(query_param("asset", "TQQQ"))
        .respond_with(ResponseTemplate::new(200).set_body_string(ERROR_UNKNOWN_ASSET_PAIR))
        .expect(1)
        .mount(&mock_server)
        .await;

    let mut client = get_test_client(&mock_server);

    let resp = client.get_asset_info(&request).await;

    assert!(resp.is_err());
    assert!(matches!(
        resp,
        Err(ClientError::Kraken(KrakenError::UnknownAssetPair))
    ));
}

#[tokio::test]
async fn test_get_account_balance_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/Balance"))
        .respond_with(ResponseTemplate::new(200).set_body_string(ERROR_INVALID_KEY))
        .expect(1)
        .mount(&mock_server)
        .await;

    let mut client = get_test_client(&mock_server);

    let resp = client.get_account_balance().await;

    assert!(resp.is_err());
    assert!(matches!(
        resp,
        Err(ClientError::Kraken(KrakenError::InvalidKey))
    ));
}

#[tokio::test]
async fn test_cancel_order_batch_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/CancelOrderBatch"))
        .respond_with(ResponseTemplate::new(200).set_body_string(ERROR_PERMISSION_DENIED))
        .expect(1)
        .mount(&mock_server)
        .await;

    let mut client = get_test_client(&mock_server);

    let request = CancelBatchOrdersRequest::builder(vec![
        IntOrString::String("id".into()),
        IntOrString::Int(19),
    ])
    .build();

    let resp = client.cancel_order_batch(&request).await;

    assert!(resp.is_err());
    assert!(matches!(
        resp,
        Err(ClientError::Kraken(KrakenError::PermissionDenied))
    ));
}

fn get_test_client(mock_server: &MockServer) -> CoreKrakenClient {
    let secrets_provider = get_null_secrets_provider();
    let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> =
        Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));

    CoreKrakenClient::new_with_url(secrets_provider, nonce_provider, mock_server.uri())
}
