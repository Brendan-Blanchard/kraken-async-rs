mod resources;

use crate::resources::kraken_responses::public_response_json::get_server_time_json;
use crate::resources::test_auth::get_null_secrets_provider;
use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
use kraken_async_rs::clients::kraken_client::endpoints::KRAKEN_BASE_URL;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
use std::sync::Arc;
use tokio::sync::Mutex;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[test]
fn client_creates() {
    let secrets_provider = get_null_secrets_provider();
    let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> =
        Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));
    let client = CoreKrakenClient::new(secrets_provider, nonce_provider);

    assert_eq!(client.api_url, KRAKEN_BASE_URL);
}

#[tokio::test]
async fn client_user_agent() {
    let secrets_provider = get_null_secrets_provider();
    let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> =
        Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));
    let mock_server = MockServer::start().await;
    let mut client =
        CoreKrakenClient::new_with_url(secrets_provider, nonce_provider, mock_server.uri());

    Mock::given(method("GET"))
        .and(path("/0/public/Time"))
        .and(header("user-agent", "KrakenAsyncRsClient"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_server_time_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    let _resp = client.get_server_time().await;
    mock_server.verify().await;

    client.set_user_agent("Strategy#1".to_string());

    Mock::given(method("GET"))
        .and(path("/0/public/Time"))
        .and(header("user-agent", "Strategy#1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_server_time_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    let _resp = client.get_server_time().await;
    mock_server.verify().await;
}
