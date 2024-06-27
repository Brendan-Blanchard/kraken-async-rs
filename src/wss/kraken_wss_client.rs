//! Kraken WSS client and message streams
use crate::wss::errors::WSSError;
use crate::wss::private::messages::PrivateMessage;
use crate::wss::public::messages::PublicMessage;
use crate::wss::subscribe_messages::{SubscribeMessage, UnsubscribeMessage};
use futures_util::SinkExt;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::net::TcpStream;
use tokio_stream::Stream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tracing::{debug, trace};
use url::Url;

const WS_KRAKEN: &str = "wss://ws.kraken.com";
const WS_KRAKEN_AUTH: &str = "wss://ws-auth.kraken.com";

type RawStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// A client for connecting to Kraken websockets.
///
/// Connecting to the public (`connect`) or private (`connect_auth`) uris returns a `KrakenMessageStream`
/// that can be awaited for values after sending subscriptions or other messages.
///
/// # Example: Listening to OHLC/Candles
/// Creating a client for public messages requires no token or authentication. All that's needed is
/// to send a valid subscription message for some number of pairs to subscribe to, then listen
/// indefinitely.
/// ```no_run
/// use kraken_async_rs::wss::v2::base_messages::{Message, WssMessage};
/// use kraken_async_rs::wss::v2::kraken_wss_client::KrakenWSSClient;
/// use kraken_async_rs::wss::v2::market_data_messages::OhlcSubscription;
/// use std::time::Duration;
/// use tokio::time::timeout;
/// use tokio_stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let mut client = KrakenWSSClient::new();
///     let mut kraken_stream = client.connect::<WssMessage>().await.unwrap();
///
///     let ohlc_params = OhlcSubscription::new(vec!["ETH/USD".into()], 60);
///     let subscription = Message::new_subscription(ohlc_params, 0);
///
///     let result = kraken_stream.send(&subscription).await;
///     assert!(result.is_ok());
///
///     while let Ok(Some(message)) = timeout(Duration::from_secs(10), kraken_stream.next()).await {
///         if let Ok(response) = message {
///             println!("{:?}", response);
///         } else {
///             println!("Message failed: {:?}", message);
///         }
///     }
/// }
/// ```
#[deprecated(
    since = "0.1.0",
    note = "Please switch to the supported v2 websockets api"
)]
#[derive(Debug, Clone, Copy)]
pub struct KrakenWSSClient<'a> {
    base_url: &'a str,
    auth_url: &'a str,
}

impl<'a> Default for KrakenWSSClient<'a> {
    fn default() -> Self {
        KrakenWSSClient::new()
    }
}

impl<'a> KrakenWSSClient<'a> {
    /// Create a client using the default Kraken URLs.
    pub fn new() -> KrakenWSSClient<'a> {
        KrakenWSSClient {
            base_url: WS_KRAKEN,
            auth_url: WS_KRAKEN_AUTH,
        }
    }

    /// Create a client with custom URLs.
    ///
    /// This is most useful for use with a proxy, or for testing.
    pub fn new_with_urls(base_url: &'a str, auth_url: &'a str) -> KrakenWSSClient<'a> {
        KrakenWSSClient { base_url, auth_url }
    }

    /// Connect to the Kraken public websocket channel, returning a [`Result`] containing a
    /// [`KrakenMessageStream`] of [`PublicMessage`]s.
    pub async fn connect(&mut self) -> Result<KrakenMessageStream<PublicMessage>, WSSError> {
        self._connect(self.base_url).await
    }

    /// Connect to the Kraken private websocket channel, returning a [`Result`] containing a
    /// [`KrakenMessageStream`] of [`PrivateMessage`]s.
    pub async fn connect_auth(&mut self) -> Result<KrakenMessageStream<PrivateMessage>, WSSError> {
        self._connect(self.auth_url).await
    }

    #[tracing::instrument(skip(self))]
    async fn _connect<T>(&mut self, url: &str) -> Result<KrakenMessageStream<T>, WSSError>
    where
        T: for<'d> Deserialize<'d>,
    {
        let url = Url::parse(url)?;
        let (raw_stream, _response) = connect_async(url).await?;

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
    /// Send a subscribe message to the connected channel.
    #[tracing::instrument(skip(self))]
    pub async fn subscribe(
        &mut self,
        subscribe_message: &SubscribeMessage,
    ) -> Result<(), WSSError> {
        Self::send_as_str(&mut self.stream, subscribe_message).await
    }

    /// Send an unsubscribe message to the connected channel.
    #[tracing::instrument(skip(self))]
    pub async fn unsubscribe(
        &mut self,
        unsubscribe_message: &UnsubscribeMessage,
    ) -> Result<(), WSSError> {
        Self::send_as_str(&mut self.stream, unsubscribe_message).await
    }

    /// Send an arbitrary serializable message through the stream.
    #[tracing::instrument(skip(self))]
    pub async fn send<S>(&mut self, message: &S) -> Result<(), WSSError>
    where
        S: Serialize + Debug,
    {
        Self::send_as_str(&mut self.stream, message).await
    }

    #[tracing::instrument(skip(stream))]
    async fn send_as_str<S>(stream: &mut RawStream, message: &S) -> Result<(), WSSError>
    where
        S: Serialize + Debug,
    {
        let message_json = serde_json::to_string(message)?;

        debug!("Sending: {}", message_json);
        stream
            .send(Message::Binary(message_json.as_bytes().to_vec()))
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
            trace!("Received: {}", message.to_string());
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
        let client = KrakenWSSClient::new_with_urls(mock_url, mock_auth_url);
        assert_eq!(mock_url, client.base_url);
        assert_eq!(mock_auth_url, client.auth_url);
    }
}
