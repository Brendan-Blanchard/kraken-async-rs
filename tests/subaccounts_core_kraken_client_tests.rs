mod resources;

use crate::resources::test_auth::get_null_secrets_provider;

use crate::resources::kraken_responses::sub_accounts_json::{
    get_account_transfer_json, get_create_sub_account_json,
};
use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::request_types::{AccountTransferRequest, CreateSubAccountRequest};
use wiremock::matchers::{body_string_contains, header_exists, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};

use rust_decimal_macros::dec;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::test]
async fn test_create_subaccount() {
    let secrets_provider = get_null_secrets_provider();
    let request =
        CreateSubAccountRequest::builder("username".to_string(), "user@mail.com".to_string())
            .build();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/CreateSubaccount"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("username=username"))
        .and(body_string_contains("email=user%40mail.com"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_create_sub_account_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, create_sub_account, &request);
}

#[tokio::test]
async fn test_account_transfer() {
    let secrets_provider = get_null_secrets_provider();
    let request = AccountTransferRequest::builder(
        "BTC".to_string(),
        dec!(1031.2008),
        "SourceAccount".to_string(),
        "DestAccount".to_string(),
    )
    .build();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/AccountTransfer"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("asset=BTC"))
        .and(body_string_contains("amount=1031.2008"))
        .and(body_string_contains("from=SourceAccount"))
        .and(body_string_contains("to=DestAccount"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_account_transfer_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, account_transfer, &request);
}
