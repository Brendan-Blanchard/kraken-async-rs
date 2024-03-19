mod resources;
use crate::resources::test_auth::get_null_secrets_provider;
use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
use std::sync::Arc;
use tokio::sync::Mutex;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_uri_parsing() {
    let secrets_provider = get_null_secrets_provider();
    let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> =
        Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));
    let mut client =
        CoreKrakenClient::new_with_url(secrets_provider, nonce_provider, "badUrl".to_string());

    let resp = client.get_websockets_token().await;
    assert_eq!("relative URL without a base", resp.unwrap_err().to_string());
}

#[tokio::test]
async fn test_invalid_response() {
    let secrets_provider = get_null_secrets_provider();
    let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> =
        Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));
    let mock_server = MockServer::start().await;
    let mut client =
        CoreKrakenClient::new_with_url(secrets_provider, nonce_provider, mock_server.uri());

    Mock::given(method("POST"))
        .and(path("/0/private/GetWebSocketsToken"))
        .respond_with(ResponseTemplate::new(200).set_body_json(""))
        .expect(1)
        .mount(&mock_server)
        .await;

    let resp = client.get_websockets_token().await;
    assert_eq!(
        "invalid type: string \"\", expected struct ResultErrorResponse at line 1 column 2",
        resp.unwrap_err().to_string()
    );
}

#[tokio::test]
async fn test_invalid_status_code() {
    let secrets_provider = get_null_secrets_provider();
    let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> =
        Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));
    let mock_server = MockServer::start().await;
    let mut client =
        CoreKrakenClient::new_with_url(secrets_provider, nonce_provider, mock_server.uri());

    Mock::given(method("POST"))
        .and(path("/0/private/GetWebSocketsToken"))
        .respond_with(ResponseTemplate::new(424).set_body_json(""))
        .expect(1)
        .mount(&mock_server)
        .await;

    let resp = client.get_websockets_token().await;
    assert_eq!(
        "Non-successful status with body: \"\"",
        resp.unwrap_err().to_string()
    );
}
