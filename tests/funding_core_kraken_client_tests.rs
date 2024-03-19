use crate::resources::test_auth::get_null_secrets_provider;

mod resources;

use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::resources::kraken_responses::funding::{
    get_deposit_addresses_json, get_deposit_methods_json, get_request_wallet_transfer_json,
    get_request_withdrawal_cancellation_json, get_status_of_recent_deposits_json,
    get_status_of_recent_withdrawals_json, get_withdraw_funds_json, get_withdrawal_addresses_json,
    get_withdrawal_info_json, get_withdrawal_methods_json,
};
use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::request_types::{
    DepositAddressesRequest, DepositMethodsRequest, StatusOfDepositWithdrawRequest,
    WalletTransferRequest, WithdrawCancelRequest, WithdrawFundsRequest, WithdrawalAddressesRequest,
    WithdrawalInfoRequest, WithdrawalMethodsRequest,
};
use wiremock::matchers::{body_string_contains, header_exists, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_deposit_methods() {
    let secrets_provider = get_null_secrets_provider();
    let request = DepositMethodsRequest::builder("ETH".to_string()).build();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/DepositMethods"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("asset=ETH"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_deposit_methods_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, get_deposit_methods, &request);
}

#[tokio::test]
async fn test_get_deposit_addresses() {
    let secrets_provider = get_null_secrets_provider();
    let request = DepositAddressesRequest::builder("BTC".to_string(), "Bitcoin".to_string())
        .is_new(true)
        .build();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/DepositAddresses"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("asset=BTC"))
        .and(body_string_contains("method=Bitcoin"))
        .and(body_string_contains("new=true"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_deposit_addresses_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(
        secrets_provider,
        mock_server,
        get_deposit_addresses,
        &request
    );
}

#[tokio::test]
async fn test_get_status_of_recent_deposits() {
    let secrets_provider = get_null_secrets_provider();
    let request = StatusOfDepositWithdrawRequest::builder()
        .asset_class("currency".to_string())
        .build();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/DepositStatus"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("aclass=currency"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(get_status_of_recent_deposits_json()),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(
        secrets_provider,
        mock_server,
        get_status_of_recent_deposits,
        &request
    );
}

#[tokio::test]
async fn test_get_withdrawal_methods() {
    let secrets_provider = get_null_secrets_provider();
    let request = WithdrawalMethodsRequest::builder()
        .asset_class("currency".to_string())
        .build();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/WithdrawMethods"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("aclass=currency"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_withdrawal_methods_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(
        secrets_provider,
        mock_server,
        get_withdrawal_methods,
        &request
    );
}

#[tokio::test]
async fn test_get_withdrawal_addresses() {
    let secrets_provider = get_null_secrets_provider();
    let request = WithdrawalAddressesRequest::builder()
        .asset_class("currency".to_string())
        .build();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/WithdrawAddresses"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("aclass=currency"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_withdrawal_addresses_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(
        secrets_provider,
        mock_server,
        get_withdrawal_addresses,
        &request
    );
}

#[tokio::test]
async fn test_get_withdrawal_info() {
    let secrets_provider = get_null_secrets_provider();
    let request = WithdrawalInfoRequest::builder(
        "XBT".to_string(),
        "Greenlisted Address".to_string(),
        "0.1".to_string(),
    )
    .build();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/WithdrawInfo"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("asset=XBT"))
        .and(body_string_contains("key=Greenlisted+Address"))
        .and(body_string_contains("amount=0.1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_withdrawal_info_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, get_withdrawal_info, &request);
}

#[tokio::test]
async fn test_withdraw_funds() {
    let secrets_provider = get_null_secrets_provider();
    let request = WithdrawFundsRequest::builder(
        "XBT".to_string(),
        "Greenlisted Address".to_string(),
        "0.1".to_string(),
    )
    .max_fee("0.00001".to_string())
    .build();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/Withdraw"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("asset=XBT"))
        .and(body_string_contains("key=Greenlisted+Address"))
        .and(body_string_contains("amount=0.1"))
        .and(body_string_contains("max_fee=0.00001"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_withdraw_funds_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, withdraw_funds, &request);
}

#[tokio::test]
async fn test_get_status_of_recent_withdrawals() {
    let secrets_provider = get_null_secrets_provider();
    let request = StatusOfDepositWithdrawRequest::builder()
        .asset_class("currency".to_string())
        .build();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/WithdrawStatus"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("aclass=currency"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(get_status_of_recent_withdrawals_json()),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(
        secrets_provider,
        mock_server,
        get_status_of_recent_withdrawals,
        &request
    );
}

#[tokio::test]
async fn test_request_withdrawal_cancellation() {
    let secrets_provider = get_null_secrets_provider();
    let request = WithdrawCancelRequest::builder("XBT".to_string(), "uuid".to_string()).build();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/WithdrawCancel"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("asset=XBT"))
        .and(body_string_contains("refid=uuid"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(get_request_withdrawal_cancellation_json()),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(
        secrets_provider,
        mock_server,
        request_withdrawal_cancellation,
        &request
    );
}

#[tokio::test]
async fn test_request_wallet_transfer() {
    let secrets_provider = get_null_secrets_provider();
    let request = WalletTransferRequest::builder(
        "XBT".to_string(),
        "Account One".to_string(),
        "Account Two".to_string(),
        "0.25".to_string(),
    )
    .build();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/WalletTransfer"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("asset=XBT"))
        .and(body_string_contains("from=Account+One"))
        .and(body_string_contains("to=Account+Two"))
        .and(body_string_contains("amount=0.25"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_request_wallet_transfer_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(
        secrets_provider,
        mock_server,
        request_wallet_transfer,
        &request
    );
}
