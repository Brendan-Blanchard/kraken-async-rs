use futures_util::StreamExt;
use kraken_async_rs::wss::v2::base_messages::{Message, WssMessage};
use kraken_async_rs::wss::v2::kraken_wss_client::KrakenWSSClient;
use serde::Serialize;
use serde_json::Value;
use simple_builder::Builder;
use std::fmt::Debug;
use std::time::Duration;
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;
use ws_mock::matchers::JsonExact;
use ws_mock::ws_mock_server::{WsMock, WsMockServer};

pub struct WssTestState {
    pub mock_server: WsMockServer,
    pub ws_client: KrakenWSSClient,
}

impl WssTestState {
    pub async fn new() -> Self {
        let mock_server = WsMockServer::start().await;
        let server_uri = mock_server.uri().await;
        let ws_client = KrakenWSSClient::new_with_urls(server_uri.clone(), server_uri);
        WssTestState {
            mock_server,
            ws_client,
        }
    }
}

#[derive(Debug, Builder)]
pub struct CallResponseTest<T>
where
    T: Debug + Serialize,
{
    match_on: Option<Value>,
    respond_with: Option<String>,
    send: Option<Message<T>>,
    expect: Option<WssMessage>,
}

impl<T> CallResponseTest<T>
where
    T: Debug + Serialize,
{
    pub async fn test(&mut self) {
        assert!(self.match_on.is_some());
        assert!(self.respond_with.is_some());
        assert!(self.send.is_some());
        assert!(self.expect.is_some());

        let mut test_state = WssTestState::new().await;

        WsMock::new()
            .matcher(JsonExact::new(self.match_on.take().unwrap()))
            .expect(1)
            .respond_with(TungsteniteMessage::Text(self.respond_with.take().unwrap()))
            .mount(&test_state.mock_server)
            .await;

        let mut stream = test_state.ws_client.connect::<WssMessage>().await.unwrap();

        stream.send(&self.send.take().unwrap()).await.unwrap();

        let result = timeout(Duration::from_secs(3), stream.next()).await;

        test_state.mock_server.verify().await;

        let response = result.unwrap().unwrap().unwrap();

        println!("{:?}", response);
        assert_eq!(self.expect.take().unwrap(), response);
    }
}
