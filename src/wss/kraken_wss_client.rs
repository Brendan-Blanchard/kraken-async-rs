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
    use crate::crypto::secrets::Token;
    use crate::request_types::TimeInForceV2;
    use crate::response_types::{BuySell, OrderType, SystemStatus};
    use crate::test_data::{
        get_balances_subscription_response, get_book_subscription_response,
        get_execution_subscription_response, get_expected_balances_message,
        get_expected_balances_subscription, get_expected_book_message,
        get_expected_book_subscription, get_expected_execution_message,
        get_expected_execution_subscription, get_expected_instruments_message,
        get_expected_instruments_subscription, get_expected_l3_message,
        get_expected_l3_subscription, get_expected_ohlc_message, get_expected_ohlc_subscription,
        get_expected_ping, get_expected_pong_message, get_expected_ticker_message,
        get_expected_ticker_subscription, get_expected_trade_message,
        get_expected_trade_subscription, get_instruments_subscription_response,
        get_l3_subscription_response, get_ohlc_subscription_response, get_pong,
        get_ticker_subscription_response, get_trade_subscription_response, parse_for_test,
        CallResponseTest, ParseIncomingTest,
    };
    use crate::wss::ChannelMessage::{Heartbeat, Status};
    use crate::wss::MethodMessage::{AddOrder, AmendOrder, CancelOrder, EditOrder};
    use crate::wss::{
        AddOrderParams, AddOrderResult, AmendOrderParams, AmendOrderResult, Asset, AssetStatus,
        BalancesSubscription, BatchCancelParams, BatchCancelResponse, BatchOrder, BatchOrderParams,
        BidAsk, BookSubscription, CancelAllOrdersParams, CancelAllOrdersResult,
        CancelOnDisconnectParams, CancelOnDisconnectResult, CancelOrderParams, CancelOrderResult,
        ChannelMessage, EditOrderParams, EditOrderResult, ExecutionSubscription, FeePreference,
        Instruments, InstrumentsSubscription, L3BidAsk, L3BidAskUpdate, L3Orderbook,
        L3OrderbookUpdate, MarketDataResponse, MarketLimit, MethodMessage, Ohlc, OhlcSubscription,
        Orderbook, OrderbookEvent, OrderbookUpdate, Pair, PairStatus, ResultResponse,
        SingleResponse, StatusUpdate, Ticker, TickerSubscription, Trade, TradesSubscription,
        WssMessage, L2, L3,
    };
    use rust_decimal_macros::dec;
    use serde_json::{json, Number};
    use std::str::FromStr;
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

    #[tokio::test]
    async fn test_admin_messages() {
        let heartbeat = r#"{"channel":"heartbeat"}"#.to_string();
        let status_update = r#"{"channel":"status","data":[{"api_version":"v2","connection_id":12393906104898154338,"system":"online","version":"2.0.4"}],"type":"update"}"#.to_string();

        let status_message = WssMessage::Channel(Status(SingleResponse {
            data: StatusUpdate {
                api_version: "v2".to_string(),
                connection_id: Some(Number::from_str("12393906104898154338").unwrap()),
                system: SystemStatus::Online,
                version: "2.0.4".to_string(),
            },
        }));

        ParseIncomingTest::new()
            .with_incoming(heartbeat)
            .expect_message(WssMessage::Channel(Heartbeat))
            .with_incoming(status_update)
            .expect_message(status_message)
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_ping_pong() {
        let ping: Option<()> = None;

        let message = Message {
            method: "ping".to_string(),
            params: ping,
            req_id: 1,
        };

        CallResponseTest::builder()
            .match_on(get_expected_ping())
            .respond_with(get_pong())
            .send(message)
            .expect(get_expected_pong_message())
            .build()
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_cloudflare_error() {
        // a bare string isn't valid JSON, so this fails to parse rather than producing an error
        let cloudflare_restarting = r#"CloudFlare WebSocket proxy restarting"#;

        let result = parse_for_test(cloudflare_restarting).await;

        assert!(matches!(result, Err(WSSError::Serde(..))));
    }

    #[tokio::test]
    async fn test_error_messages() {
        let unsupported_field = r#"{"error":"Unsupported field: 'params' for the given msg type: ping","method":"ping","req_id":0,"success":false,"time_in":"2024-05-19T19:58:40.170724Z","time_out":"2024-05-19T19:58:40.170758Z"}"#.to_string();

        let expected_unsupported_field = WssMessage::Method(MethodMessage::Ping(ResultResponse {
            result: None,
            error: Some("Unsupported field: 'params' for the given msg type: ping".to_string()),
            success: false,
            req_id: 0,
            time_in: "2024-05-19T19:58:40.170724Z".to_string(),
            time_out: "2024-05-19T19:58:40.170758Z".to_string(),
        }));

        let unsupported_event = r#"{"error":"Unsupported event","method":"subscribe","req_id":0,"success":false,"time_in":"2024-05-19T20:02:10.316562Z","time_out":"2024-05-19T20:02:10.316592Z"}"#.to_string();

        let expected_unsupported_event =
            WssMessage::Method(MethodMessage::Subscription(ResultResponse {
                result: None,
                error: Some("Unsupported event".to_string()),
                success: false,
                req_id: 0,
                time_in: "2024-05-19T20:02:10.316562Z".to_string(),
                time_out: "2024-05-19T20:02:10.316592Z".to_string(),
            }));

        let invalid_arguments = r#"{"error":"EGeneral:Invalid arguments:no_mpp order option is only available when ordertype = market","method":"add_order","req_id":0,"success":false,"time_in":"2024-05-18T12:03:08.768086Z","time_out":"2024-05-18T12:03:08.768149Z"}"#.to_string();

        let expected_invalid_arguments =
            WssMessage::Method(MethodMessage::AddOrder(ResultResponse {
                result: None,
                error: Some("EGeneral:Invalid arguments:no_mpp order option is only available when ordertype = market".to_string()),
                success: false,
                req_id: 0,
                time_in: "2024-05-18T12:03:08.768086Z".to_string(),
                time_out: "2024-05-18T12:03:08.768149Z".to_string(),
            }));

        let add_order_failure = r#"{"error":"Cash_order_qty field must be a number_float","method":"add_order","req_id":7,"success":false,"time_in":"2024-05-18T12:00:03.886027Z","time_out":"2024-05-18T12:00:03.886141Z"}"#.to_string();

        let expected_add_order_failure =
            WssMessage::Method(MethodMessage::AddOrder(ResultResponse {
                result: None,
                error: Some("Cash_order_qty field must be a number_float".to_string()),
                success: false,
                req_id: 7,
                time_in: "2024-05-18T12:00:03.886027Z".to_string(),
                time_out: "2024-05-18T12:00:03.886141Z".to_string(),
            }));

        let permission_denied = r#"{"error":"EGeneral:Permission denied","method":"add_order","req_id":0,"success":false,"time_in":"2024-05-18T12:03:43.466650Z","time_out":"2024-05-18T12:03:43.471987Z"}"#.to_string();

        let expected_permission_denied =
            WssMessage::Method(MethodMessage::AddOrder(ResultResponse {
                result: None,
                error: Some("EGeneral:Permission denied".to_string()),
                success: false,
                req_id: 0,
                time_in: "2024-05-18T12:03:43.466650Z".to_string(),
                time_out: "2024-05-18T12:03:43.471987Z".to_string(),
            }));

        let no_token = r#"{"error":"Token(s) not found","method":"edit_order","req_id":0,"success":false,"time_in":"2024-05-18T13:04:41.754066Z","time_out":"2024-05-18T13:04:41.754113Z"}"#.to_string();

        let expected_no_token = WssMessage::Method(MethodMessage::EditOrder(ResultResponse {
            result: None,
            error: Some("Token(s) not found".to_string()),
            success: false,
            req_id: 0,
            time_in: "2024-05-18T13:04:41.754066Z".to_string(),
            time_out: "2024-05-18T13:04:41.754113Z".to_string(),
        }));

        ParseIncomingTest::new()
            .with_incoming(unsupported_field)
            .expect_message(expected_unsupported_field)
            .with_incoming(unsupported_event)
            .expect_message(expected_unsupported_event)
            .with_incoming(invalid_arguments)
            .expect_message(expected_invalid_arguments)
            .with_incoming(add_order_failure)
            .expect_message(expected_add_order_failure)
            .with_incoming(permission_denied)
            .expect_message(expected_permission_denied)
            .with_incoming(no_token)
            .expect_message(expected_no_token)
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_ticker_snapshot() {
        let ticker_snapshot = r#"{
        "channel":"ticker",
        "type":"snapshot",
        "data":[{
            "symbol":"BTC/USD",
            "bid":65972.8,
            "bid_qty":0.10000000,
            "ask":65972.9,
            "ask_qty":39.67506683,
            "last":65972.9,
            "volume":4216.61829370,
            "vwap":64275.2,
            "low":61325.4,
            "high":66450.0,
            "change":4412.1,
            "change_pct":7.17
        }]
    }"#
        .to_string();

        let expected_snapshot = WssMessage::Channel(ChannelMessage::Ticker(SingleResponse {
            data: Ticker {
                ask: dec!(65972.9),
                ask_quantity: dec!(39.67506683),
                bid: dec!(65972.8),
                bid_quantity: dec!(0.10000000),
                change: dec!(4412.1),
                change_pct: dec!(7.17),
                high: dec!(66450.0),
                last: dec!(65972.9),
                low: dec!(61325.4),
                symbol: "BTC/USD".to_string(),
                volume: dec!(4216.61829370),
                vwap: dec!(64275.2),
            },
        }));

        ParseIncomingTest::new()
            .with_incoming(ticker_snapshot)
            .expect_message(expected_snapshot)
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_ticker_update() {
        let ticker_update = r#"{
        "channel":"ticker",
        "type":"update",
        "data":[{
            "symbol":"BTC/USD",
            "bid":65843.7,
            "bid_qty":12.31628629,
            "ask":65843.8,
            "ask_qty":0.31232000,
            "last":65843.7,
            "volume":4182.59447976,
            "vwap":64223.4,
            "low":61325.4,
            "high":66450.0,
            "change":4213.8,
            "change_pct":6.84
        }]
    }"#
        .to_string();

        let expected_update = WssMessage::Channel(ChannelMessage::Ticker(SingleResponse {
            data: Ticker {
                ask: dec!(65843.8),
                ask_quantity: dec!(0.31232000),
                bid: dec!(65843.7),
                bid_quantity: dec!(12.31628629),
                change: dec!(4213.8),
                change_pct: dec!(6.84),
                high: dec!(66450.0),
                last: dec!(65843.7),
                low: dec!(61325.4),
                symbol: "BTC/USD".to_string(),
                volume: dec!(4182.59447976),
                vwap: dec!(64223.4),
            },
        }));

        ParseIncomingTest::new()
            .with_incoming(ticker_update)
            .expect_message(expected_update)
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_book_snapshot() {
        let book_snapshot = r#"{
        "channel":"book",
        "type":"snapshot",
        "data":[{
            "symbol":"BTC/USD",
            "bids":[
                {"price":66788.0,"qty":3.21926649},
                {"price":66787.5,"qty":0.44916298},
                {"price":66787.4,"qty":0.05992580},
                {"price":66785.3,"qty":0.01496904},
                {"price":66785.2,"qty":0.86989511}
            ],
            "asks":[
                {"price":66788.1,"qty":1.67939137},
                {"price":66788.4,"qty":1.49726637},
                {"price":66790.0,"qty":1.49723133},
                {"price":66791.1,"qty":0.01100000},
                {"price":66792.6,"qty":1.49717197}
            ],
            "checksum":2330500275
        }]
    }"#
        .to_string();

        let expected_snapshot = WssMessage::Channel(ChannelMessage::Orderbook(SingleResponse {
            data: L2::Orderbook(Orderbook {
                symbol: "BTC/USD".to_string(),
                checksum: 2330500275,
                bids: vec![
                    BidAsk {
                        price: dec!(66788.0),
                        quantity: dec!(3.21926649),
                    },
                    BidAsk {
                        price: dec!(66787.5),
                        quantity: dec!(0.44916298),
                    },
                    BidAsk {
                        price: dec!(66787.4),
                        quantity: dec!(0.05992580),
                    },
                    BidAsk {
                        price: dec!(66785.3),
                        quantity: dec!(0.01496904),
                    },
                    BidAsk {
                        price: dec!(66785.2),
                        quantity: dec!(0.86989511),
                    },
                ],
                asks: vec![
                    BidAsk {
                        price: dec!(66788.1),
                        quantity: dec!(1.67939137),
                    },
                    BidAsk {
                        price: dec!(66788.4),
                        quantity: dec!(1.49726637),
                    },
                    BidAsk {
                        price: dec!(66790.0),
                        quantity: dec!(1.49723133),
                    },
                    BidAsk {
                        price: dec!(66791.1),
                        quantity: dec!(0.01100000),
                    },
                    BidAsk {
                        price: dec!(66792.6),
                        quantity: dec!(1.49717197),
                    },
                ],
            }),
        }));

        ParseIncomingTest::new()
            .with_incoming(book_snapshot)
            .expect_message(expected_snapshot)
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_book_update() {
        let book_update = r#"{
        "channel":"book",
        "type":"update",
        "data":[{
            "symbol":"BTC/USD",
            "bids":[
                {"price":66786.5,"qty":0.00000000},
                {"price":66784.5,"qty":0.01470022},
                {"price":66787.7,"qty":0.12440000}
            ],
            "asks":[],
            "checksum":902440905,
            "timestamp":"2024-05-19T16:45:24.204654Z"
        }]
    }"#
        .to_string();

        let expected_update = WssMessage::Channel(ChannelMessage::Orderbook(SingleResponse {
            data: L2::Update(OrderbookUpdate {
                symbol: "BTC/USD".to_string(),
                checksum: 902440905,
                timestamp: "2024-05-19T16:45:24.204654Z".to_string(),
                bids: vec![
                    BidAsk {
                        price: dec!(66786.5),
                        quantity: dec!(0.00000000),
                    },
                    BidAsk {
                        price: dec!(66784.5),
                        quantity: dec!(0.01470022),
                    },
                    BidAsk {
                        price: dec!(66787.7),
                        quantity: dec!(0.12440000),
                    },
                ],
                asks: vec![],
            }),
        }));

        ParseIncomingTest::new()
            .with_incoming(book_update)
            .expect_message(expected_update)
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_l3_snapshot() {
        let l3_snapshot = r#"{
        "channel":"level3",
        "type":"snapshot",
        "data": [{
        "symbol":"BTC/USD",
        "checksum":1361442827,
        "bids":[
            {"order_id":"OZYA6B-OE3BH-YJ4PY5","limit_price":66579.2,"order_qty":1.35137590,"timestamp":"2024-05-19T18:55:20.910159752Z"},
            {"order_id":"OIOQ7V-JT5S2-QLIEPO","limit_price":66579.2,"order_qty":0.47905712,"timestamp":"2024-05-19T18:55:20.910276406Z"},
            {"order_id":"O34I4J-KIE3I-BOT6VC","limit_price":66579.2,"order_qty":0.03003941,"timestamp":"2024-05-19T18:55:23.001943740Z"},
            {"order_id":"OUOCIK-GA6WX-DSZC2A","limit_price":66574.1,"order_qty":0.45057561,"timestamp":"2024-05-19T18:55:15.431184641Z"}
        ],
        "asks":[
            {"order_id":"OUPTOY-CCUJG-BMAZ5S","limit_price":66579.3,"order_qty":0.07800000,"timestamp":"2024-05-19T18:55:22.531833732Z"},
            {"order_id":"OFUNE7-IGNAY-5UATGI","limit_price":66581.5,"order_qty":1.50192021,"timestamp":"2024-05-19T18:55:25.967603045Z"},
            {"order_id":"ORCUC4-UGIUC-MT5KBA","limit_price":66583.7,"order_qty":0.87745184,"timestamp":"2024-05-19T18:55:18.938264721Z"}
        ]
    }]}"#.to_string();

        let expected_snapshot = WssMessage::Channel(ChannelMessage::L3(SingleResponse {
            data: L3::Orderbook(L3Orderbook {
                symbol: "BTC/USD".to_string(),
                bids: vec![
                    L3BidAsk {
                        order_id: "OZYA6B-OE3BH-YJ4PY5".to_string(),
                        limit_price: dec!(66579.2),
                        order_quantity: dec!(1.35137590),
                        timestamp: "2024-05-19T18:55:20.910159752Z".to_string(),
                    },
                    L3BidAsk {
                        order_id: "OIOQ7V-JT5S2-QLIEPO".to_string(),
                        limit_price: dec!(66579.2),
                        order_quantity: dec!(0.47905712),
                        timestamp: "2024-05-19T18:55:20.910276406Z".to_string(),
                    },
                    L3BidAsk {
                        order_id: "O34I4J-KIE3I-BOT6VC".to_string(),
                        limit_price: dec!(66579.2),
                        order_quantity: dec!(0.03003941),
                        timestamp: "2024-05-19T18:55:23.001943740Z".to_string(),
                    },
                    L3BidAsk {
                        order_id: "OUOCIK-GA6WX-DSZC2A".to_string(),
                        limit_price: dec!(66574.1),
                        order_quantity: dec!(0.45057561),
                        timestamp: "2024-05-19T18:55:15.431184641Z".to_string(),
                    },
                ],
                asks: vec![
                    L3BidAsk {
                        order_id: "OUPTOY-CCUJG-BMAZ5S".to_string(),
                        limit_price: dec!(66579.3),
                        order_quantity: dec!(0.07800000),
                        timestamp: "2024-05-19T18:55:22.531833732Z".to_string(),
                    },
                    L3BidAsk {
                        order_id: "OFUNE7-IGNAY-5UATGI".to_string(),
                        limit_price: dec!(66581.5),
                        order_quantity: dec!(1.50192021),
                        timestamp: "2024-05-19T18:55:25.967603045Z".to_string(),
                    },
                    L3BidAsk {
                        order_id: "ORCUC4-UGIUC-MT5KBA".to_string(),
                        limit_price: dec!(66583.7),
                        order_quantity: dec!(0.87745184),
                        timestamp: "2024-05-19T18:55:18.938264721Z".to_string(),
                    },
                ],
                checksum: 1361442827,
            }),
        }));

        ParseIncomingTest::new()
            .with_incoming(l3_snapshot)
            .expect_message(expected_snapshot)
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_l3_update() {
        let l3_update = r#"{
        "channel":"level3",
        "type":"update",
        "data":[{
            "checksum":2143854316,
            "symbol":"BTC/USD",
            "bids":[
                {
                    "event":"delete",
                    "order_id":"O7SO4Y-RHRAK-GGAHJE",
                    "limit_price":66567.3,
                    "order_qty":0.22540000,
                    "timestamp":"2024-05-19T18:59:46.541105556Z"
                },
                {
                    "event":"add",
                    "order_id":"OI2XQ5-6JUYI-A5NI6J",
                    "limit_price":66566.9,
                    "order_qty":2.82230268,
                    "timestamp":"2024-05-19T18:59:44.900460701Z"
                }
            ],
            "asks":[]
        }]
    }"#
        .to_string();

        let expected_update = WssMessage::Channel(ChannelMessage::L3(SingleResponse {
            data: L3::Update(L3OrderbookUpdate {
                symbol: "BTC/USD".to_string(),
                bids: vec![
                    L3BidAskUpdate {
                        event: OrderbookEvent::Delete,
                        order_id: "O7SO4Y-RHRAK-GGAHJE".to_string(),
                        limit_price: dec!(66567.3),
                        order_quantity: dec!(0.22540000),
                        timestamp: "2024-05-19T18:59:46.541105556Z".to_string(),
                    },
                    L3BidAskUpdate {
                        event: OrderbookEvent::Add,
                        order_id: "OI2XQ5-6JUYI-A5NI6J".to_string(),
                        limit_price: dec!(66566.9),
                        order_quantity: dec!(2.82230268),
                        timestamp: "2024-05-19T18:59:44.900460701Z".to_string(),
                    },
                ],
                asks: vec![],
                checksum: 2143854316,
            }),
        }));

        ParseIncomingTest::new()
            .with_incoming(l3_update)
            .expect_message(expected_update)
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_candles_snapshot() {
        let candles_snapshot = r#"{
        "channel":"ohlc",
        "type":"snapshot",
        "timestamp":"2024-05-17T11:21:16.318303322Z",
        "data":[
            {"symbol":"ETH/USD","open":3027.80,"high":3027.80,"low":3026.13,"close":3026.13,"trades":9,"volume":13.31603062,"vwap":3027.01,"interval_begin":"2024-05-17T11:12:00.000000000Z","interval":1,"timestamp":"2024-05-17T11:13:00.000000Z"},
            {"symbol":"ETH/USD","open":3026.46,"high":3026.47,"low":3026.46,"close":3026.47,"trades":4,"volume":2.14044498,"vwap":3026.46,"interval_begin":"2024-05-17T11:13:00.000000000Z","interval":1,"timestamp":"2024-05-17T11:14:00.000000Z"}
        ]
    }"#
            .to_string();

        let expected_snapshot = WssMessage::Channel(ChannelMessage::Ohlc(MarketDataResponse {
            data: vec![
                Ohlc {
                    symbol: "ETH/USD".to_string(),
                    open: dec!(3027.80),
                    high: dec!(3027.80),
                    low: dec!(3026.13),
                    close: dec!(3026.13),
                    vwap: dec!(3027.01),
                    trades: 9,
                    volume: dec!(13.31603062),
                    interval_begin: "2024-05-17T11:12:00.000000000Z".to_string(),
                    interval: 1,
                },
                Ohlc {
                    symbol: "ETH/USD".to_string(),
                    open: dec!(3026.46),
                    high: dec!(3026.47),
                    low: dec!(3026.46),
                    close: dec!(3026.47),
                    vwap: dec!(3026.46),
                    trades: 4,
                    volume: dec!(2.14044498),
                    interval_begin: "2024-05-17T11:13:00.000000000Z".to_string(),
                    interval: 1,
                },
            ],
        }));

        ParseIncomingTest::new()
            .with_incoming(candles_snapshot)
            .expect_message(expected_snapshot)
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_trade_snapshot() {
        let trade_snapshot = r#"{
        "channel":"trade",
        "type":"snapshot",
        "data":[
            {"symbol":"BTC/USD","side":"sell","price":68466.9,"qty":0.01919415,"ord_type":"market","trade_id":70635251,"timestamp":"2024-05-27T12:33:10.826003Z"},
            {"symbol":"BTC/USD","side":"buy","price":68471.2,"qty":0.00007723,"ord_type":"limit","trade_id":70635252,"timestamp":"2024-05-27T12:33:10.980704Z"}
        ]
    }"#.to_string();

        let expected_snapshot = WssMessage::Channel(ChannelMessage::Trade(MarketDataResponse {
            data: vec![
                Trade {
                    symbol: "BTC/USD".to_string(),
                    side: BuySell::Sell,
                    quantity: dec!(0.01919415),
                    price: dec!(68466.9),
                    order_type: MarketLimit::Market,
                    trade_id: 70635251,
                    timestamp: "2024-05-27T12:33:10.826003Z".to_string(),
                },
                Trade {
                    symbol: "BTC/USD".to_string(),
                    side: BuySell::Buy,
                    quantity: dec!(0.00007723),
                    price: dec!(68471.2),
                    order_type: MarketLimit::Limit,
                    trade_id: 70635252,
                    timestamp: "2024-05-27T12:33:10.980704Z".to_string(),
                },
            ],
        }));

        ParseIncomingTest::new()
            .with_incoming(trade_snapshot)
            .expect_message(expected_snapshot)
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_trade_update() {
        let trade_update = r#"{
        "channel":"trade",
        "type":"update",
        "data":[
            {"symbol":"BTC/USD","side":"buy","price":68500.0,"qty":0.01044926,"ord_type":"limit","trade_id":70635299,"timestamp":"2024-05-27T12:43:11.798009Z"},
            {"symbol":"BTC/USD","side":"buy","price":68500.0,"qty":0.00483192,"ord_type":"limit","trade_id":70635300,"timestamp":"2024-05-27T12:43:11.798009Z"}
        ]
    }"#.to_string();

        let expected_update = WssMessage::Channel(ChannelMessage::Trade(MarketDataResponse {
            data: vec![
                Trade {
                    symbol: "BTC/USD".to_string(),
                    side: BuySell::Buy,
                    quantity: dec!(0.01044926),
                    price: dec!(68500.0),
                    order_type: MarketLimit::Limit,
                    trade_id: 70635299,
                    timestamp: "2024-05-27T12:43:11.798009Z".to_string(),
                },
                Trade {
                    symbol: "BTC/USD".to_string(),
                    side: BuySell::Buy,
                    quantity: dec!(0.00483192),
                    price: dec!(68500.0),
                    order_type: MarketLimit::Limit,
                    trade_id: 70635300,
                    timestamp: "2024-05-27T12:43:11.798009Z".to_string(),
                },
            ],
        }));

        ParseIncomingTest::new()
            .with_incoming(trade_update)
            .expect_message(expected_update)
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_instruments_snapshot() {
        let instrument_snapshot = r#"{
        "channel":"instrument",
        "type":"snapshot",
        "data":{
            "assets":[
                {"id":"USD","status":"enabled","precision":4,"precision_display":2,"borrowable":true,"collateral_value":1.00,"margin_rate":0.025000},
                {"id":"EUR","status":"enabled","precision":4,"precision_display":2,"borrowable":true,"collateral_value":1.00,"margin_rate":0.020000},
                {"id":"ETH","status":"enabled","precision":10,"precision_display":5,"borrowable":true,"collateral_value":1.00,"margin_rate":0.020000}
            ],
            "pairs": [
                {"symbol":"EUR/USD","base":"EUR","quote":"USD","status":"online","qty_precision":8,"qty_increment":0.00000001,"price_precision":5,"cost_precision":5,"marginable":false,"has_index":true,"cost_min":0.50,"tick_size":0.00001,"price_increment":0.00001,"qty_min":0.50000000},
                {"symbol":"ETH/BTC","base":"ETH","quote":"BTC","status":"online","qty_precision":8,"qty_increment":0.00000001,"price_precision":5,"cost_precision":10,"marginable":true,"has_index":true,"cost_min":0.00002,"margin_initial":0.20,"position_limit_long":1000,"position_limit_short":600,"tick_size":0.00001,"price_increment":0.00001,"qty_min":0.00200000}
            ]
        }
    }"#.to_string();

        let expected_snapshot =
            WssMessage::Channel(ChannelMessage::Instrument(MarketDataResponse {
                data: Instruments {
                    assets: vec![
                        Asset {
                            id: "USD".to_string(),
                            margin_rate: Some(dec!(0.025000)),
                            precision: 4,
                            precision_display: 2,
                            status: AssetStatus::Enabled,
                            borrowable: true,
                            collateral_value: dec!(1.0),
                        },
                        Asset {
                            id: "EUR".to_string(),
                            margin_rate: Some(dec!(0.020000)),
                            precision: 4,
                            precision_display: 2,
                            status: AssetStatus::Enabled,
                            borrowable: true,
                            collateral_value: dec!(1.0),
                        },
                        Asset {
                            id: "ETH".to_string(),
                            margin_rate: Some(dec!(0.020000)),
                            precision: 10,
                            precision_display: 5,
                            status: AssetStatus::Enabled,
                            borrowable: true,
                            collateral_value: dec!(1.0),
                        },
                    ],
                    pairs: vec![
                        Pair {
                            base: "EUR".to_string(),
                            quote: "USD".to_string(),
                            cost_min: dec!(0.50),
                            cost_precision: 5,
                            has_index: true,
                            margin_initial: None,
                            marginable: false,
                            position_limit_long: None,
                            position_limit_short: None,
                            price_increment: dec!(0.00001),
                            price_precision: 5,
                            quantity_increment: dec!(0.00000001),
                            quantity_min: dec!(0.50),
                            quantity_precision: 8,
                            status: PairStatus::Online,
                            symbol: "EUR/USD".to_string(),
                        },
                        Pair {
                            base: "ETH".to_string(),
                            quote: "BTC".to_string(),
                            cost_min: dec!(0.00002),
                            cost_precision: 10,
                            has_index: true,
                            margin_initial: Some(dec!(0.2)),
                            marginable: true,
                            position_limit_long: Some(1000),
                            position_limit_short: Some(600),
                            price_increment: dec!(0.00001),
                            price_precision: 5,
                            quantity_increment: dec!(0.00000001),
                            quantity_min: dec!(0.002),
                            quantity_precision: 8,
                            status: PairStatus::Online,
                            symbol: "ETH/BTC".to_string(),
                        },
                    ],
                },
            }));

        ParseIncomingTest::new()
            .with_incoming(instrument_snapshot)
            .expect_message(expected_snapshot)
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_execution_subscription() {
        let mut execution_params = ExecutionSubscription::new(Token::new("someToken".to_string()));
        execution_params.snapshot_trades = Some(true);
        execution_params.snapshot_orders = Some(true);

        let subscription = Message::new_subscription(execution_params, 0);

        CallResponseTest::builder()
            .match_on(get_expected_execution_subscription())
            .respond_with(get_execution_subscription_response())
            .send(subscription)
            .expect(get_expected_execution_message())
            .build()
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_balances_subscription() {
        let mut balances_params = BalancesSubscription::new(Token::new("anotherToken".to_string()));
        balances_params.snapshot = Some(true);

        let subscription = Message::new_subscription(balances_params, 10312008);

        CallResponseTest::builder()
            .match_on(get_expected_balances_subscription())
            .respond_with(get_balances_subscription_response())
            .send(subscription)
            .expect(get_expected_balances_message())
            .build()
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_ticker_subscription() {
        let ticker_params = TickerSubscription::new(vec!["BTC/USD".into()]);

        let subscription = Message::new_subscription(ticker_params, 42);

        CallResponseTest::builder()
            .match_on(get_expected_ticker_subscription())
            .respond_with(get_ticker_subscription_response())
            .send(subscription)
            .expect(get_expected_ticker_message())
            .build()
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_book_subscription() {
        let mut book_params = BookSubscription::new(vec!["BTC/USD".into()]);
        book_params.depth = Some(10);
        book_params.snapshot = Some(true);

        let subscription = Message::new_subscription(book_params, 11);

        CallResponseTest::builder()
            .match_on(get_expected_book_subscription())
            .respond_with(get_book_subscription_response())
            .send(subscription)
            .expect(get_expected_book_message())
            .build()
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_l3_subscription() {
        let mut book_params =
            BookSubscription::new_l3(vec!["BTC/USD".into()], Token::new("someToken".to_string()));
        book_params.snapshot = Some(true);

        let subscription = Message::new_subscription(book_params, 99);

        CallResponseTest::builder()
            .match_on(get_expected_l3_subscription())
            .respond_with(get_l3_subscription_response())
            .send(subscription)
            .expect(get_expected_l3_message())
            .build()
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_ohlc_subscription() {
        let ohlc_params = OhlcSubscription::new(vec!["ETH/USD".into()], 60);

        let subscription = Message::new_subscription(ohlc_params, 121);

        CallResponseTest::builder()
            .match_on(get_expected_ohlc_subscription())
            .respond_with(get_ohlc_subscription_response())
            .send(subscription)
            .expect(get_expected_ohlc_message())
            .build()
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_trade_subscription() {
        let trade_params = TradesSubscription::new(vec!["BTC/USD".into()]);

        let subscription = Message::new_subscription(trade_params, 0);

        CallResponseTest::builder()
            .match_on(get_expected_trade_subscription())
            .respond_with(get_trade_subscription_response())
            .send(subscription)
            .expect(get_expected_trade_message())
            .build()
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_instruments_subscription() {
        let instruments_params = InstrumentsSubscription::new(true);

        let subscription = Message::new_subscription(instruments_params, 0);

        CallResponseTest::builder()
            .match_on(get_expected_instruments_subscription())
            .respond_with(get_instruments_subscription_response())
            .send(subscription)
            .expect(get_expected_instruments_message())
            .build()
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_add_order() {
        let expected_request = json!({"method":"add_order","params":{"order_type":"limit","side":"buy","symbol":"USDC/USD","limit_price":0.95,"time_in_force":"ioc","order_qty":5.0,"post_only":false,"fee_preference":"quote","token":"aToken","cl_ord_id":"client-zero","sender_sub_id":"sender-one"},"req_id":0});
        let response = r#"{"method":"add_order","req_id":0,"result":{"order_id":"OPS23M-VS41G-DDE5Z2","cl_ord_id":"client-zero"},"success":true,"time_in":"2024-05-18T12:05:50.293682Z","time_out":"2024-05-18T12:05:50.300542Z"}"#.to_string();
        let expected_response = WssMessage::Method(AddOrder(ResultResponse {
            result: Some(AddOrderResult {
                order_id: "OPS23M-VS41G-DDE5Z2".to_string(),
                order_user_ref: None,
                warning: None,
                client_order_id: Some("client-zero".to_string()),
            }),
            error: None,
            success: true,
            req_id: 0,
            time_in: "2024-05-18T12:05:50.293682Z".to_string(),
            time_out: "2024-05-18T12:05:50.300542Z".to_string(),
        }));

        let add_order: AddOrderParams = AddOrderParams {
            order_type: OrderType::Limit,
            side: BuySell::Buy,
            symbol: "USDC/USD".to_string(),
            limit_price: Some(dec!(0.95)),
            limit_price_type: None,
            triggers: None,
            time_in_force: Some(TimeInForceV2::IOC),
            order_quantity: dec!(5.0),
            margin: None,
            post_only: Some(false),
            reduce_only: None,
            effective_time: None,
            expire_time: None,
            deadline: None,
            order_user_ref: None,
            conditional: None,
            display_quantity: None,
            fee_preference: Some(FeePreference::Quote),
            no_market_price_protection: None,
            stp_type: None,
            cash_order_quantity: None,
            validate: None,
            sender_sub_id: Some("sender-one".to_string()),
            token: Token::new("aToken".to_string()),
            client_order_id: Some("client-zero".to_string()),
        };

        let message = Message {
            method: "add_order".to_string(),
            params: add_order,
            req_id: 0,
        };

        CallResponseTest::builder()
            .match_on(expected_request)
            .respond_with(response)
            .send(message)
            .expect(expected_response)
            .build()
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_amend_order() {
        let expected_request = json!({"method":"amend_order","params":{"order_id":"BQS60L-EGW18-UPAK9U","order_qty":5.1,"limit_price":0.96,"post_only":false,"token":"aToken"},"req_id":0});
        let response = r#"{"method":"amend_order","req_id":0,"result":{"amend_id":"1M2JV8-OEJZD-G5GSBF","order_id":"BQS60L-EGW18-UPAK9U"},"success":true,"time_in":"2024-10-11T12:12:21.003873Z","time_out":"2024-10-11T12:12:21.005064Z"}"#.to_string();
        let expected_response = WssMessage::Method(AmendOrder(ResultResponse {
            result: Some(AmendOrderResult {
                amend_id: "1M2JV8-OEJZD-G5GSBF".to_string(),
                order_id: Some("BQS60L-EGW18-UPAK9U".to_string()),
                client_order_id: None,
                warnings: None,
            }),
            error: None,
            success: true,
            req_id: 0,
            time_in: "2024-10-11T12:12:21.003873Z".to_string(),
            time_out: "2024-10-11T12:12:21.005064Z".to_string(),
        }));

        let amend_order = AmendOrderParams {
            order_id: Some("BQS60L-EGW18-UPAK9U".to_string()),
            limit_price: Some(dec!(0.96)),
            limit_price_type: None,
            post_only: Some(false),
            trigger_price: None,
            trigger_price_type: None,
            deadline: None,
            token: Token::new("aToken".to_string()),
            client_order_id: None,
            order_quantity: dec!(5.1),
            display_quantity: None,
        };

        let message = Message {
            method: "amend_order".to_string(),
            params: amend_order,
            req_id: 0,
        };

        CallResponseTest::builder()
            .match_on(expected_request)
            .respond_with(response)
            .send(message)
            .expect(expected_response)
            .build()
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_amend_order_error_response() {
        let response = r#"{"error":"Limit_price field must be a number_float","method":"amend_order","req_id":0,"success":false,"time_in":"2024-10-13T13:31:28.636431Z","time_out":"2024-10-13T13:31:28.636488Z"}"#;

        let expected_message = WssMessage::Method(AmendOrder(ResultResponse {
            result: None,
            error: Some("Limit_price field must be a number_float".to_string()),
            success: false,
            req_id: 0,
            time_in: "2024-10-13T13:31:28.636431Z".to_string(),
            time_out: "2024-10-13T13:31:28.636488Z".to_string(),
        }));

        ParseIncomingTest::new()
            .with_incoming(response.to_string())
            .expect_message(expected_message)
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_edit_order() {
        let expected_request = json!({"method":"edit_order","params":{"limit_price":0.93,"order_id":"K1FF7H-A13AR-Q1S9Z6","order_qty":6.1,"symbol":"USDC/USD","token":"someToken"},"req_id":0});
        let response = r#"{"method":"edit_order","req_id":0,"result":{"order_id":"7FIK6B-S15X0-DPJJTH","original_order_id":"K1FF7H-A13AR-Q1S9Z6"},"success":true,"time_in":"2024-05-19T12:12:30.171615Z","time_out":"2024-05-19T12:12:30.173877Z"}"#.to_string();
        let expected_response = WssMessage::Method(EditOrder(ResultResponse {
            result: Some(EditOrderResult {
                order_id: "7FIK6B-S15X0-DPJJTH".to_string(),
                original_order_id: "K1FF7H-A13AR-Q1S9Z6".to_string(),
                warning: None,
            }),
            error: None,
            success: true,
            req_id: 0,
            time_in: "2024-05-19T12:12:30.171615Z".to_string(),
            time_out: "2024-05-19T12:12:30.173877Z".to_string(),
        }));

        let edit_order = EditOrderParams {
            symbol: "USDC/USD".to_string(),
            limit_price: Some(dec!(0.93)),
            triggers: None,
            order_quantity: Some(dec!(6.1)),
            post_only: None,
            reduce_only: None,
            deadline: None,
            order_user_ref: None,
            display_quantity: None,
            fee_preference: None,
            no_market_price_protection: None,
            validate: None,
            token: Token::new("someToken".to_string()),
            order_id: "K1FF7H-A13AR-Q1S9Z6".to_string(),
        };

        let message = Message {
            method: "edit_order".to_string(),
            params: edit_order,
            req_id: 0,
        };

        CallResponseTest::builder()
            .match_on(expected_request)
            .respond_with(response)
            .send(message)
            .expect(expected_response)
            .build()
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_cancel_order() {
        let expected_request = json!({"method":"cancel_order","params":{"order_id":["1V7PZA-L5RIM-RX2G6B"],"token":"thatToken"},"req_id":0});
        let response = r#"{"method":"cancel_order","req_id":0,"result":{"order_id":"1V7PZA-L5RIM-RX2G6B"},"success":true,"time_in":"2024-05-19T19:18:44.987402Z","time_out":"2024-05-19T19:18:44.989756Z"}"#.to_string();
        let expected_response = WssMessage::Method(CancelOrder(ResultResponse {
            result: Some(CancelOrderResult {
                order_id: Some("1V7PZA-L5RIM-RX2G6B".to_string()),
                warning: None,
                client_order_id: None,
            }),
            error: None,
            success: true,
            req_id: 0,
            time_in: "2024-05-19T19:18:44.987402Z".to_string(),
            time_out: "2024-05-19T19:18:44.989756Z".to_string(),
        }));

        let cancel_order = CancelOrderParams {
            order_id: Some(vec!["1V7PZA-L5RIM-RX2G6B".into()]),
            client_order_id: None,
            order_user_ref: None,
            token: Token::new("thatToken".to_string()),
        };

        let message = Message {
            method: "cancel_order".to_string(),
            params: cancel_order,
            req_id: 0,
        };

        CallResponseTest::builder()
            .match_on(expected_request)
            .respond_with(response)
            .send(message)
            .expect(expected_response)
            .build()
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_cancel_order_by_client_order_id() {
        let expected_request = json!({"method":"cancel_order","params":{"cl_ord_id":["a-uuid"],"token":"thatToken"},"req_id":0});
        let response = r#"{"method":"cancel_order","req_id":0,"result":{"cl_ord_id":"a-uuid"},"success":true,"time_in":"2024-05-19T19:18:44.987402Z","time_out":"2024-05-19T19:18:44.989756Z"}"#.to_string();
        let expected_response = WssMessage::Method(CancelOrder(ResultResponse {
            result: Some(CancelOrderResult {
                order_id: None,
                warning: None,
                client_order_id: Some("a-uuid".to_string()),
            }),
            error: None,
            success: true,
            req_id: 0,
            time_in: "2024-05-19T19:18:44.987402Z".to_string(),
            time_out: "2024-05-19T19:18:44.989756Z".to_string(),
        }));

        let cancel_order = CancelOrderParams {
            order_id: None,
            client_order_id: Some(vec!["a-uuid".to_string()]),
            order_user_ref: None,
            token: Token::new("thatToken".to_string()),
        };

        let message = Message {
            method: "cancel_order".to_string(),
            params: cancel_order,
            req_id: 0,
        };

        CallResponseTest::builder()
            .match_on(expected_request)
            .respond_with(response)
            .send(message)
            .expect(expected_response)
            .build()
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_cancel_all_orders() {
        let expected_request =
            json!({"method":"cancel_all","params":{"token":"thisToken"},"req_id":0});
        let response = r#"{"method":"cancel_all","req_id":0,"result":{"count":0},"success":true,"time_in":"2024-05-19T11:42:13.815662Z","time_out":"2024-05-19T11:42:13.824053Z"}"#.to_string();
        let expected_response =
            WssMessage::Method(MethodMessage::CancelAllOrders(ResultResponse {
                result: Some(CancelAllOrdersResult {
                    count: 0,
                    warning: None,
                }),
                error: None,
                success: true,
                req_id: 0,
                time_in: "2024-05-19T11:42:13.815662Z".to_string(),
                time_out: "2024-05-19T11:42:13.824053Z".to_string(),
            }));

        let cancel_all = CancelAllOrdersParams {
            token: Token::new("thisToken".to_string()),
        };

        let message = Message {
            method: "cancel_all".to_string(),
            params: cancel_all,
            req_id: 0,
        };

        CallResponseTest::builder()
            .match_on(expected_request)
            .respond_with(response)
            .send(message)
            .expect(expected_response)
            .build()
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_cancel_on_disconnect() {
        let expected_request = json!({"method":"cancel_all_orders_after","params":{"timeout":5,"token":"yourToken"},"req_id":0});
        let response = r#"{"method":"cancel_all_orders_after","req_id":0,"result":{"currentTime":"2024-05-19T19:22:20Z","triggerTime":"2024-05-19T19:22:25Z"},"success":true,"time_in":"2024-05-19T19:22:19.975239Z","time_out":"2024-05-19T19:22:19.981369Z"}"#.to_string();
        let expected_response =
            WssMessage::Method(MethodMessage::CancelOnDisconnect(ResultResponse {
                result: Some(CancelOnDisconnectResult {
                    current_time: "2024-05-19T19:22:20Z".into(),
                    warning: None,
                    trigger_time: "2024-05-19T19:22:25Z".into(),
                }),
                error: None,
                success: true,
                req_id: 0,
                time_in: "2024-05-19T19:22:19.975239Z".to_string(),
                time_out: "2024-05-19T19:22:19.981369Z".to_string(),
            }));

        let cancel_on_disconnect = CancelOnDisconnectParams {
            timeout: 5,
            token: Token::new("yourToken".to_string()),
        };

        let message = Message {
            method: "cancel_all_orders_after".to_string(),
            params: cancel_on_disconnect,
            req_id: 0,
        };

        CallResponseTest::builder()
            .match_on(expected_request)
            .respond_with(response)
            .send(message)
            .expect(expected_response)
            .build()
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_batch_add() {
        let expected_request = json!({"method":"batch_add","params":{"symbol":"USDC/USD","token":"myToken","orders":[{"order_type":"limit","side":"buy","limit_price":0.99,"order_qty":5.0,"post_only":true,"fee_preference":"quote"},{"order_type":"limit","side":"buy","limit_price":0.95,"order_qty":5.0,"post_only":true,"fee_preference":"base"}]},"req_id":0});
        let response = r#"{"method":"batch_add","req_id":0,"result":[{"order_id":"JQDNTT-MZEIZ-OZKUDD"},{"order_id":"X67GEK-3VQWM-HPNQ89"}],"success":true,"time_in":"2024-05-19T19:23:21.134538Z","time_out":"2024-05-19T19:23:21.141229Z"}"#.to_string();
        let expected_response = WssMessage::Method(MethodMessage::BatchOrder(ResultResponse {
            result: Some(vec![
                AddOrderResult {
                    order_id: "JQDNTT-MZEIZ-OZKUDD".to_string(),
                    order_user_ref: None,
                    warning: None,
                    client_order_id: None,
                },
                AddOrderResult {
                    order_id: "X67GEK-3VQWM-HPNQ89".to_string(),
                    order_user_ref: None,
                    warning: None,

                    client_order_id: None,
                },
            ]),
            error: None,
            success: true,
            req_id: 0,
            time_in: "2024-05-19T19:23:21.134538Z".to_string(),
            time_out: "2024-05-19T19:23:21.141229Z".to_string(),
        }));

        let batch_add = BatchOrderParams {
            deadline: None,
            symbol: "USDC/USD".to_string(),
            token: Token::new("myToken".to_string()),
            validate: None,
            orders: vec![
                BatchOrder {
                    order_type: OrderType::Limit,
                    side: BuySell::Buy,
                    limit_price: Some(dec!(0.99)),
                    limit_price_type: None,
                    triggers: None,
                    time_in_force: None,
                    order_quantity: dec!(5.0),
                    margin: None,
                    post_only: Some(true),
                    reduce_only: None,
                    effective_time: None,
                    expire_time: None,
                    order_user_ref: None,
                    conditional: None,
                    display_quantity: None,
                    fee_preference: Some(FeePreference::Quote),
                    no_market_price_protection: None,
                    stp_type: None,
                    cash_order_quantity: None,
                    client_order_id: None,
                },
                BatchOrder {
                    order_type: OrderType::Limit,
                    side: BuySell::Buy,
                    limit_price: Some(dec!(0.95)),
                    limit_price_type: None,
                    triggers: None,
                    time_in_force: None,
                    order_quantity: dec!(5.0),
                    margin: None,
                    post_only: Some(true),
                    reduce_only: None,
                    effective_time: None,
                    expire_time: None,
                    order_user_ref: None,
                    conditional: None,
                    display_quantity: None,
                    fee_preference: Some(FeePreference::Base),
                    no_market_price_protection: None,
                    stp_type: None,
                    cash_order_quantity: None,
                    client_order_id: None,
                },
            ],
        };

        let message = Message {
            method: "batch_add".to_string(),
            params: batch_add,
            req_id: 0,
        };

        CallResponseTest::builder()
            .match_on(expected_request)
            .respond_with(response)
            .send(message)
            .expect(expected_response)
            .build()
            .test()
            .await;
    }

    #[tokio::test]
    async fn test_batch_cancel() {
        let expected_request = json!({"method":"batch_cancel","params":{"orders":["IY8YF6-Y6LCR-AMZD7P","XR6VND-GLY6K-DL33TB"],"token":"theirToken"},"req_id":0});
        let response = r#"{"method":"batch_cancel","orders_cancelled":2,"req_id":0,"success":true,"time_in":"2024-05-19T19:29:58.063754Z","time_out":"2024-05-19T19:29:58.071569Z"}"#.to_string();
        let expected_response =
            WssMessage::Method(MethodMessage::BatchCancel(BatchCancelResponse {
                orders_cancelled: 2,
                error: None,
                success: true,
                req_id: 0,
                time_in: "2024-05-19T19:29:58.063754Z".to_string(),
                time_out: "2024-05-19T19:29:58.071569Z".to_string(),
                client_order_id: None,
            }));

        let batch_cancel = BatchCancelParams {
            orders: vec!["IY8YF6-Y6LCR-AMZD7P".into(), "XR6VND-GLY6K-DL33TB".into()],
            token: Token::new("theirToken".to_string()),
            client_order_id: None,
        };

        let message = Message {
            method: "batch_cancel".to_string(),
            params: batch_cancel,
            req_id: 0,
        };

        CallResponseTest::builder()
            .match_on(expected_request)
            .respond_with(response)
            .send(message)
            .expect(expected_response)
            .build()
            .test()
            .await;
    }
}
