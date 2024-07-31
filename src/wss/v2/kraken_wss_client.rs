//! Kraken WSS client and message streams
use crate::wss::errors::WSSError;
use crate::wss::v2::base_messages::Message;
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
use tracing::{debug, trace};
use url::Url;

const WS_KRAKEN: &str = "wss://ws.kraken.com/v2";
const WS_KRAKEN_AUTH: &str = "wss://ws-auth.kraken.com/v2";

type RawStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// A client for connecting to Kraken websockets via the V2 protocol.
#[derive(Debug, Clone)]
pub struct KrakenWSSClient {
    base_url: String,
    auth_url: String,
}

impl Default for KrakenWSSClient {
    fn default() -> Self {
        KrakenWSSClient::new()
    }
}

impl KrakenWSSClient {
    /// Create a client using the default Kraken URLs.
    pub fn new() -> KrakenWSSClient {
        KrakenWSSClient {
            base_url: WS_KRAKEN.to_string(),
            auth_url: WS_KRAKEN_AUTH.to_string(),
        }
    }

    /// Create a client with custom URLs.
    ///
    /// This is most useful for use with a proxy, or for testing.
    pub fn new_with_urls(base_url: String, auth_url: String) -> KrakenWSSClient {
        KrakenWSSClient { base_url, auth_url }
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
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    phantom: PhantomData<T>,
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
        Self::send_as_str(&mut self.stream, message).await
    }

    #[tracing::instrument(skip(stream))]
    async fn send_as_str<M>(stream: &mut RawStream, message: &Message<M>) -> Result<(), WSSError>
    where
        M: Serialize + Debug,
    {
        let message_json = serde_json::to_string(message)?;

        if cfg!(feature = "debug-outbound") {
            debug!("Sending: {}", message_json);
        }

        stream
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
}
