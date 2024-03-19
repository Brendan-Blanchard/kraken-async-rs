mod resources;

use crate::resources::test_auth::get_null_secrets_provider;

use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};

use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::clients::rate_limited_kraken_client::RateLimitedKrakenClient;
use kraken_async_rs::response_types::VerificationTier::Pro;
use resources::test_client::test_client_impl::TestRateLimitedClient;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::{pause, Instant};

#[tokio::test]
async fn test_get_websockets_token() {
    pause();

    // 23 calls costs 2300, requiring 3s to replenish @ 100/s
    test_rate_limited_endpoint!(get_websockets_token, 23, 3, 4, Pro);
}
