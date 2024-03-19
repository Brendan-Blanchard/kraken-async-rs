mod resources;

use crate::resources::test_auth::get_null_secrets_provider;

use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::clients::rate_limited_kraken_client::RateLimitedKrakenClient;
use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
use kraken_async_rs::request_types::{
    AllocateEarnFundsRequest, EarnAllocationStatusRequest, ListEarnAllocationsRequest,
    ListEarnStrategiesRequest,
};
use kraken_async_rs::response_types::VerificationTier::{Intermediate, Pro};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{pause, Instant};

use resources::test_client::test_client_impl::TestRateLimitedClient;

use std::time::Duration;

#[tokio::test]
async fn test_allocate_earn_funds() {
    pause();

    let request =
        AllocateEarnFundsRequest::builder("10.123".to_string(), "W38S2C-Y1E0R-DUFM2T".to_string())
            .build();

    // 24 calls costs 2400, requiring 4s to replenish @ 100/s
    test_rate_limited_endpoint!(allocate_earn_funds, 24, 4, 5, Pro, &request);
}

#[tokio::test]
async fn test_deallocate_earn_funds() {
    pause();

    let request =
        AllocateEarnFundsRequest::builder("10.123".to_string(), "W38S2C-Y1E0R-DUFM2T".to_string())
            .build();

    // 24 calls costs 2400, requiring 8s to replenish @ 50/s
    test_rate_limited_endpoint!(deallocate_earn_funds, 24, 8, 9, Intermediate, &request);
}

#[tokio::test]
async fn test_get_allocation_status() {
    pause();

    let request = EarnAllocationStatusRequest::builder("W38S2C-Y1E0R-DUFM2T".to_string()).build();

    // 24 calls costs 2400, requiring 8s to replenish @ 50/s
    test_rate_limited_endpoint!(get_earn_allocation_status, 24, 8, 9, Intermediate, &request);
}

#[tokio::test]
async fn test_get_deallocation_status() {
    pause();

    let request = EarnAllocationStatusRequest::builder("W38S2C-Y1E0R-DUFM2T".to_string()).build();

    // 24 calls costs 2400, requiring 8s to replenish @ 50/s
    test_rate_limited_endpoint!(
        get_earn_deallocation_status,
        24,
        8,
        9,
        Intermediate,
        &request
    );
}

#[tokio::test]
async fn test_list_earn_strategies() {
    pause();

    let request = ListEarnStrategiesRequest::builder()
        .limit(64)
        .ascending(true)
        .build();

    // 24 calls costs 2400, requiring 4s to replenish @ 100/s
    test_rate_limited_endpoint!(list_earn_strategies, 24, 4, 5, Pro, &request);
}

#[tokio::test]
async fn test_list_earn_allocations() {
    pause();

    let request = ListEarnAllocationsRequest::builder()
        .ascending(true)
        .hide_zero_allocations(true)
        .build();

    // 29 calls costs 2900, requiring 18s to replenish @ 500/s
    test_rate_limited_endpoint!(list_earn_allocations, 29, 18, 19, Intermediate, &request);
}
