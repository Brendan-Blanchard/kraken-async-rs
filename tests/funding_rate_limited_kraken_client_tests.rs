use crate::resources::test_auth::get_null_secrets_provider;

mod resources;

use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{pause, Instant};

use crate::resources::test_client::test_client_impl::TestRateLimitedClient;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::clients::rate_limited_kraken_client::RateLimitedKrakenClient;
use kraken_async_rs::request_types::{
    DepositAddressesRequest, DepositMethodsRequest, StatusOfDepositWithdrawRequest,
    WalletTransferRequest, WithdrawCancelRequest, WithdrawFundsRequest, WithdrawalAddressesRequest,
    WithdrawalInfoRequest, WithdrawalMethodsRequest,
};
use kraken_async_rs::response_types::VerificationTier::{Intermediate, Pro};
use rust_decimal_macros::dec;
use std::time::Duration;

#[tokio::test]
async fn test_get_deposit_methods() {
    pause();

    let request = DepositMethodsRequest::builder("ETH".to_string()).build();

    // 24 calls costs 2400, requiring 4s to replenish @ 100/s
    test_rate_limited_endpoint!(get_deposit_methods, 24, 4, 5, Pro, &request);
}

#[tokio::test]
async fn test_get_deposit_addresses() {
    pause();

    let request = DepositAddressesRequest::builder("BTC".to_string(), "Bitcoin".to_string())
        .is_new(true)
        .build();

    // 24 calls costs 2400, requiring 8s to replenish @ 50/s
    test_rate_limited_endpoint!(get_deposit_addresses, 24, 8, 9, Intermediate, &request);
}

#[tokio::test]
async fn test_get_status_of_recent_deposits() {
    pause();

    let request = StatusOfDepositWithdrawRequest::builder()
        .asset_class("currency".to_string())
        .build();

    // 26 calls costs 2600, requiring 6s to replenish @ 100/s
    test_rate_limited_endpoint!(get_status_of_recent_deposits, 26, 6, 7, Pro, &request);
}

#[tokio::test]
async fn test_get_withdrawal_methods() {
    pause();

    let request = WithdrawalMethodsRequest::builder()
        .asset_class("currency".to_string())
        .build();

    // 26 calls costs 2600, requiring 12s to replenish @ 50/s
    test_rate_limited_endpoint!(get_withdrawal_methods, 26, 12, 13, Intermediate, &request);
}

#[tokio::test]
async fn test_get_withdrawal_addresses() {
    pause();

    let request = WithdrawalAddressesRequest::builder()
        .asset_class("currency".to_string())
        .build();

    // 25 calls costs 2500, requiring 5s to replenish @ 100/s
    test_rate_limited_endpoint!(get_withdrawal_addresses, 25, 5, 6, Pro, &request);
}

#[tokio::test]
async fn test_get_withdrawal_info() {
    pause();

    let request = WithdrawalInfoRequest::builder(
        "XBT".to_string(),
        "Greenlisted Address".to_string(),
        dec!(0.1),
    )
    .build();

    // 25 calls costs 2500, requiring 5s to replenish @ 100/s
    test_rate_limited_endpoint!(get_withdrawal_info, 25, 5, 6, Pro, &request);
}

#[tokio::test]
async fn test_withdraw_funds() {
    pause();

    let request = WithdrawFundsRequest::builder(
        "XBT".to_string(),
        "Greenlisted Address".to_string(),
        dec!(0.1),
    )
    .max_fee(dec!(0.00001))
    .build();

    // 25 calls costs 2500, requiring 10s to replenish @ 50/s
    test_rate_limited_endpoint!(withdraw_funds, 25, 10, 11, Intermediate, &request);
}

#[tokio::test]
async fn test_get_status_of_recent_withdrawals() {
    pause();

    let request = StatusOfDepositWithdrawRequest::builder()
        .asset_class("currency".to_string())
        .build();

    // 25 calls costs 2500, requiring 5s to replenish @ 100/s
    test_rate_limited_endpoint!(get_status_of_recent_withdrawals, 25, 5, 6, Pro, &request);
}

#[tokio::test]
async fn test_request_withdrawal_cancellation() {
    pause();

    let request = WithdrawCancelRequest::builder("XBT".to_string(), "uuid".to_string()).build();

    // 27 calls costs 2700, requiring 14s to replenish @ 50/s
    test_rate_limited_endpoint!(
        request_withdrawal_cancellation,
        27,
        14,
        15,
        Intermediate,
        &request
    );
}

#[tokio::test]
async fn test_request_wallet_transfer() {
    pause();

    let request = WalletTransferRequest::builder(
        "XBT".to_string(),
        "Account One".to_string(),
        "Account Two".to_string(),
        dec!(0.25),
    )
    .build();

    // 27 calls costs 2700, requiring 7s to replenish @ 100/s
    test_rate_limited_endpoint!(request_wallet_transfer, 27, 7, 8, Pro, &request);
}
