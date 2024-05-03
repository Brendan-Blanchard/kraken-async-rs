mod resources;

use crate::resources::test_auth::get_null_secrets_provider;

use crate::resources::kraken_responses::earn_json::{
    get_allocate_earn_funds_json, get_allocation_status_json, get_deallocate_earn_funds_json,
    get_deallocation_status_json, get_list_earn_allocations_json, get_list_earn_strategies_json,
};

use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
use kraken_async_rs::request_types::{
    AllocateEarnFundsRequest, EarnAllocationStatusRequest, ListEarnAllocationsRequest,
    ListEarnStrategiesRequest,
};
use rust_decimal_macros::dec;
use std::sync::Arc;
use tokio::sync::Mutex;
use wiremock::matchers::{body_string_contains, header_exists, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_allocate_earn_funds() {
    let secrets_provider = get_null_secrets_provider();
    let request =
        AllocateEarnFundsRequest::builder(dec!(10.123), "W38S2C-Y1E0R-DUFM2T".to_string()).build();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/Earn/Allocate"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("amount=10.123"))
        .and(body_string_contains("strategy_id=W38S2C-Y1E0R-DUFM2T"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_allocate_earn_funds_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, allocate_earn_funds, &request);
}

#[tokio::test]
async fn test_deallocate_earn_funds() {
    let secrets_provider = get_null_secrets_provider();
    let request =
        AllocateEarnFundsRequest::builder(dec!(10.123), "W38S2C-Y1E0R-DUFM2T".to_string()).build();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/Earn/Deallocate"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("amount=10.123"))
        .and(body_string_contains("strategy_id=W38S2C-Y1E0R-DUFM2T"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_deallocate_earn_funds_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(
        secrets_provider,
        mock_server,
        deallocate_earn_funds,
        &request
    );
}

#[tokio::test]
async fn test_get_allocation_status() {
    let secrets_provider = get_null_secrets_provider();
    let request = EarnAllocationStatusRequest::builder("W38S2C-Y1E0R-DUFM2T".to_string()).build();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/Earn/AllocateStatus"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("strategy_id=W38S2C-Y1E0R-DUFM2T"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_allocation_status_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(
        secrets_provider,
        mock_server,
        get_earn_allocation_status,
        &request
    );
}

#[tokio::test]
async fn test_get_deallocation_status() {
    let secrets_provider = get_null_secrets_provider();
    let request = EarnAllocationStatusRequest::builder("W38S2C-Y1E0R-DUFM2T".to_string()).build();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/Earn/DeallocateStatus"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("strategy_id=W38S2C-Y1E0R-DUFM2T"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_deallocation_status_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(
        secrets_provider,
        mock_server,
        get_earn_deallocation_status,
        &request
    );
}

#[tokio::test]
async fn test_list_earn_strategies() {
    let secrets_provider = get_null_secrets_provider();
    let request = ListEarnStrategiesRequest::builder()
        .limit(64)
        .ascending(true)
        .build();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/Earn/Strategies"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("limit=64"))
        .and(body_string_contains("ascending=true"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_list_earn_strategies_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(
        secrets_provider,
        mock_server,
        list_earn_strategies,
        &request
    );
}

#[tokio::test]
async fn test_list_earn_allocations() {
    let secrets_provider = get_null_secrets_provider();
    let request = ListEarnAllocationsRequest::builder()
        .ascending(true)
        .hide_zero_allocations(true)
        .build();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/Earn/Allocations"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("ascending=true"))
        .and(body_string_contains("hide_zero_allocations=true"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_list_earn_allocations_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(
        secrets_provider,
        mock_server,
        list_earn_allocations,
        &request
    );
}
