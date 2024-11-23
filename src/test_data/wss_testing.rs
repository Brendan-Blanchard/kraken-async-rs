use crate::wss::errors::WSSError;
use crate::wss::{KrakenWSSClient, Message, WssMessage};
use serde::Serialize;
use serde_json::Value;
use simple_builder::Builder;
use std::fmt::Debug;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::timeout;
use tokio_stream::StreamExt;
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

#[derive(Debug)]
pub struct ParseIncomingTest {
    incoming_messages: Vec<String>,
    expected_messages: Vec<WssMessage>,
}

impl ParseIncomingTest {
    pub fn new() -> Self {
        ParseIncomingTest {
            incoming_messages: Vec::new(),
            expected_messages: Vec::new(),
        }
    }

    pub fn with_incoming(mut self, message: String) -> Self {
        self.incoming_messages.push(message);
        self
    }

    pub fn expect_message(mut self, message: WssMessage) -> Self {
        self.expected_messages.push(message);
        self
    }

    pub async fn test(self) {
        assert_eq!(self.incoming_messages.len(), self.expected_messages.len());

        let mut test_state = WssTestState::new().await;

        let (mpsc_send, mpsc_recv) = mpsc::channel::<tokio_tungstenite::tungstenite::Message>(8);

        WsMock::new()
            .forward_from_channel(mpsc_recv)
            .mount(&test_state.mock_server)
            .await;

        let mut stream = test_state.ws_client.connect::<WssMessage>().await.unwrap();

        for (message, expected) in self
            .incoming_messages
            .into_iter()
            .zip(self.expected_messages.iter())
        {
            mpsc_send
                .send(TungsteniteMessage::Text(message))
                .await
                .unwrap();

            let result = timeout(Duration::from_secs(1), stream.next())
                .await
                .unwrap()
                .unwrap()
                .unwrap();

            assert_eq!(*expected, result);
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

/// Parse an incoming message by spinning up a test server and forwarding the message to it.
pub async fn parse_for_test(incoming: &str) -> Result<WssMessage, WSSError> {
    let mut test_state = WssTestState::new().await;

    let (mpsc_send, mpsc_recv) = mpsc::channel::<tokio_tungstenite::tungstenite::Message>(8);

    WsMock::new()
        .forward_from_channel(mpsc_recv)
        .mount(&test_state.mock_server)
        .await;

    let mut stream = test_state.ws_client.connect::<WssMessage>().await.unwrap();

    mpsc_send
        .send(TungsteniteMessage::Text(incoming.to_string()))
        .await
        .unwrap();

    timeout(Duration::from_secs(1), stream.next())
        .await
        .unwrap()
        .unwrap()
}
