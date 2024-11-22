//! Kraken WSS client and message streams
use crate::wss::errors::WSSError;
use crate::wss::Message;
use futures_util::SinkExt;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::net::TcpStream;
use tokio_stream::Stream;
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tracing::{debug, trace, warn};
use url::Url;

pub const WS_KRAKEN: &str = "wss://ws.kraken.com/v2";
pub const WS_KRAKEN_AUTH: &str = "wss://ws-auth.kraken.com/v2";

type RawStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// A client for connecting to Kraken websockets via the V2 protocol.
#[derive(Debug, Clone)]
pub struct KrakenWSSClient {
    base_url: String,
    auth_url: String,
    trace_inbound: bool,
    trace_outbound: bool,
}

impl Default for KrakenWSSClient {
    fn default() -> Self {
        KrakenWSSClient::new()
    }
}

impl KrakenWSSClient {
    /// Create a client using the default Kraken URLs.
    pub fn new() -> KrakenWSSClient {
        if cfg!(feature = "debug-inbound") {
            warn!("Feature `debug-inbound` is deprecated - please use `new_with_tracing` method on `KrakenWSSClient`")
        }

        if cfg!(feature = "debug-outbound") {
            warn!("Feature `debug-outbound` is deprecated - please use `new_with_tracing` method on `KrakenWSSClient`")
        }

        KrakenWSSClient {
            base_url: WS_KRAKEN.to_string(),
            auth_url: WS_KRAKEN_AUTH.to_string(),
            trace_inbound: false,
            trace_outbound: false,
        }
    }

    /// Create a client with custom URLs.
    ///
    /// This is most useful for use with a proxy, or for testing.
    pub fn new_with_urls(base_url: String, auth_url: String) -> KrakenWSSClient {
        KrakenWSSClient {
            base_url,
            auth_url,
            trace_inbound: false,
            trace_outbound: false,
        }
    }

    pub fn new_with_tracing(
        base_url: &str,
        auth_url: &str,
        trace_inbound: bool,
        trace_outbound: bool,
    ) -> KrakenWSSClient {
        KrakenWSSClient {
            base_url: base_url.to_string(),
            auth_url: auth_url.to_string(),
            trace_inbound,
            trace_outbound,
        }
    }

    /// Connect to the Kraken public websocket channel, returning a [`Result`] containing a
    /// [`KrakenMessageStream`] of [`PublicMessage`]s.
    pub async fn connect<T>(&mut self) -> Result<KrakenMessageStream<T>, WSSError>
    where
        T: for<'d> Deserialize<'d>,
    {
        self._connect(&self.base_url.clone()).await
    }

    /// Connect to the Kraken private websocket channel, returning a [`Result`] containing a
    /// [`KrakenMessageStream`] of [`PrivateMessage`]s.
    pub async fn connect_auth<T>(&mut self) -> Result<KrakenMessageStream<T>, WSSError>
    where
        T: for<'d> Deserialize<'d>,
    {
        self._connect(&self.auth_url.clone()).await
    }

    #[tracing::instrument(skip(self))]
    async fn _connect<T>(&mut self, url: &str) -> Result<KrakenMessageStream<T>, WSSError>
    where
        T: for<'d> Deserialize<'d>,
    {
        let url = Url::parse(url)?;
        let (raw_stream, _response) = connect_async(url.as_str()).await?;

        Ok(KrakenMessageStream {
            stream: raw_stream,
            phantom: PhantomData,
            trace_inbound: self.trace_inbound,
            trace_outbound: self.trace_outbound,
        })
    }
}

/// A futures_core::[`Stream`] implementation that returns deserializable messages. Messages can be
/// retrieved by awaiting `someStream.next()`.
///
/// # Example: Listening to Public Messages
/// See the full example including subscribing to channels in examples/live_public_wss_listening.rs.
/// ```ignore
///let mut client = KrakenWSSClient::new();
///let mut kraken_stream: KrakenMessageStream<PublicMessage> = client.connect().await.unwrap();
///
///while let Some(message) = kraken_stream.next().await {
///    println!("{:?}", message.unwrap());
///}
/// ```
pub struct KrakenMessageStream<T>
where
    T: for<'a> Deserialize<'a>,
{
    stream: RawStream,
    phantom: PhantomData<T>,
    trace_inbound: bool,
    trace_outbound: bool,
}

impl<T> Unpin for KrakenMessageStream<T>
where
    T: for<'a> Deserialize<'a>,
{
    // required for stream to be borrow-mutable when polling
}

impl<T> KrakenMessageStream<T>
where
    T: for<'a> Deserialize<'a>,
{
    /// Send an arbitrary serializable message through the stream.
    #[tracing::instrument(skip(self))]
    pub async fn send<M>(&mut self, message: &Message<M>) -> Result<(), WSSError>
    where
        M: Serialize + Debug,
    {
        self.send_as_str(message).await
    }

    #[tracing::instrument(skip(self))]
    async fn send_as_str<M>(&mut self, message: &Message<M>) -> Result<(), WSSError>
    where
        M: Serialize + Debug,
    {
        let message_json = serde_json::to_string(message)?;

        if cfg!(feature = "debug-outbound") {
            debug!("Sending: {}", message_json);
        }

        if self.trace_outbound {
            trace!("Sending: {}", message_json);
        }

        self.stream
            .send(TungsteniteMessage::Binary(message_json.as_bytes().to_vec()))
            .await?;
        Ok(())
    }
}

impl<T> Stream for KrakenMessageStream<T>
where
    T: for<'a> Deserialize<'a>,
{
    type Item = Result<T, WSSError>;

    /// returns Poll:Ready with a message if available, otherwise Poll:Pending
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Poll::Ready(Some(message)) = Pin::new(&mut self.stream).poll_next(cx)? {
            if cfg!(feature = "debug-inbound") {
                trace!("Received: {}", message.to_string());
            }
            if self.trace_inbound {
                trace!("Received: {}", message.to_string());
            }
            let parsed: T = serde_json::from_str(message.to_text()?)?;
            Poll::Ready(Some(Ok(parsed)))
        } else {
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::timeout;
    use tokio_stream::StreamExt;
    use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;
    use tracing_test::traced_test;
    use ws_mock::matchers::Any;
    use ws_mock::ws_mock_server::{WsMock, WsMockServer};

    #[test]
    fn test_wss_client_creates() {
        let client = KrakenWSSClient::new();
        assert_eq!(WS_KRAKEN, client.base_url);
        assert_eq!(WS_KRAKEN_AUTH, client.auth_url);
    }

    #[test]
    fn test_wss_default_creates_client() {
        let client = KrakenWSSClient::default();
        assert_eq!(WS_KRAKEN, client.base_url);
        assert_eq!(WS_KRAKEN_AUTH, client.auth_url);
    }

    #[test]
    fn test_wss_client_new_with_urls() {
        let mock_url = "https://trades.com";
        let mock_auth_url = "https://auth.trades.com";
        let client =
            KrakenWSSClient::new_with_urls(mock_url.to_string(), mock_auth_url.to_string());
        assert_eq!(mock_url, client.base_url);
        assert_eq!(mock_auth_url, client.auth_url);
    }

    #[traced_test]
    #[tokio::test]
    async fn test_tracing_flags_disabled_by_default() {
        let mock_server = WsMockServer::start().await;
        let uri = mock_server.uri().await;
        let mut client = KrakenWSSClient::new_with_urls(uri.clone(), uri);

        WsMock::new()
            .matcher(Any::new())
            .respond_with(TungsteniteMessage::text("response"))
            .mount(&mock_server)
            .await;

        let mut stream = client.connect::<String>().await.unwrap();

        stream.send(&Message::new_subscription(0, 0)).await.unwrap();

        let _message = timeout(Duration::from_secs(1), stream.next())
            .await
            .unwrap();

        assert!(!logs_contain("Sending:"));
        assert!(!logs_contain("Received: response"));
    }

    #[traced_test]
    #[tokio::test]
    async fn test_tracing_flags_enabled() {
        let mock_server = WsMockServer::start().await;
        let uri = mock_server.uri().await;
        let mut client = KrakenWSSClient::new_with_urls(uri.clone(), uri);
        client.trace_inbound = true;
        client.trace_outbound = true;

        WsMock::new()
            .matcher(Any::new())
            .respond_with(TungsteniteMessage::text("response"))
            .mount(&mock_server)
            .await;

        let mut stream = client.connect::<String>().await.unwrap();

        stream.send(&Message::new_subscription(0, 0)).await.unwrap();

        let _message = timeout(Duration::from_secs(1), stream.next())
            .await
            .unwrap();

        assert!(logs_contain(
            r#"Sending: {"method":"subscribe","params":0,"req_id":0}"#
        ));
        assert!(logs_contain("Received: response"));
    }
}
