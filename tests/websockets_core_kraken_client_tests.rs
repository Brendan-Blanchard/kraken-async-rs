mod resources;

use crate::resources::test_auth::get_null_secrets_provider;

use crate::resources::kraken_responses::websockets_json::get_websockets_token_json;
use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};

use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::test]
async fn test_get_websockets_token() {
    let secrets_provider = get_null_secrets_provider();
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/GetWebSocketsToken"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_websockets_token_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, get_websockets_token);
}
