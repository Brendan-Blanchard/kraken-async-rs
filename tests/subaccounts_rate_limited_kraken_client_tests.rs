mod resources;

use crate::resources::test_auth::get_null_secrets_provider;

use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::request_types::{AccountTransferRequest, CreateSubAccountRequest};

use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};

use crate::resources::test_client::test_client_impl::TestRateLimitedClient;
use kraken_async_rs::clients::rate_limited_kraken_client::RateLimitedKrakenClient;
use kraken_async_rs::response_types::VerificationTier::{Intermediate, Pro};
use rust_decimal_macros::dec;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::{pause, Instant};

#[tokio::test]
async fn test_create_sub_account() {
    pause();

    let request =
        CreateSubAccountRequest::builder("username".to_string(), "user@mail.com".to_string())
            .build();

    // 24 calls costs 2400, requiring 4s to replenish @ 100/s
    test_rate_limited_endpoint!(create_sub_account, 24, 4, 5, Pro, &request);
}

#[tokio::test]
async fn test_account_transfer() {
    pause();

    let request = AccountTransferRequest::builder(
        "BTC".to_string(),
        dec!(1031.2008),
        "SourceAccount".to_string(),
        "DestAccount".to_string(),
    )
    .build();

    // 24 calls costs 2400, requiring 8s to replenish @ 50/s
    test_rate_limited_endpoint!(account_transfer, 24, 8, 9, Intermediate, &request);
}
