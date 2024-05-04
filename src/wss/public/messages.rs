//! All public messages for websockets
use crate::response_types::{
    BuySellChar, LastTrade, MarketLimitChar, TickerBidAsk, TickerDecimal, TickerTrades,
};
use crate::wss::kraken_wss_types::{ErrorMessage, PingPong, SystemStatus};
use crate::wss::parsing::{get_event_field, get_event_from_vec};
use crate::wss::public::orderbooks::{Orderbook, OrderbookUpdateMessage};
use crate::wss::subscribe_messages::SubscriptionStatus;
use rust_decimal::Decimal;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use serde_tuple::Deserialize_tuple;

/// Represents all possible public message types
#[derive(Debug, PartialEq)]
pub enum PublicMessage {
    PingPong(PingPong),
    Heartbeat,
    SystemStatus(SystemStatus),
    SubscriptionStatus(SubscriptionStatus),
    ErrorMessage(ErrorMessage),
    Trade(Message<Vec<PublicTrade>>),
    Ticker(Box<Message<WSSTickerInfo>>),
    OHLC(Message<OHLC>),
    Spread(Message<Spread>),
    Orderbook(Message<Orderbook>),
    OrderbookUpdate(OrderbookUpdateMessage),
}

/// Generic websocket message
#[derive(Debug, PartialEq, Deserialize_tuple)]
pub struct Message<T>
where
    T: for<'a> Deserialize<'a>,
{
    #[serde(rename = "channelID")]
    pub channel_id: i64,
    pub message: T,
    #[serde(rename = "channelName")]
    pub channel_name: String,
    pub pair: String,
}

/// Publicly available trade message
#[derive(Debug, PartialEq, Deserialize_tuple)]
pub struct PublicTrade {
    pub price: Decimal,
    pub volume: Decimal,
    pub time: String,
    pub side: BuySellChar,
    #[serde(rename = "orderType")]
    pub order_type: MarketLimitChar,
    pub misc: String,
}

/// OHLC/Candlestick for a given interval
#[derive(Debug, Deserialize_tuple, PartialEq)]
pub struct OHLC {
    pub time: String,
    pub end_time: String,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub vwap: Decimal,
    pub volume: Decimal,
    pub count: i64,
}

/// Best bid and ask and volumes at the recorded timestamp
#[derive(Debug, PartialEq, Deserialize_tuple)]
pub struct Spread {
    pub bid: Decimal,
    pub ask: Decimal,
    pub timestamp: String,
    pub bid_volume: Decimal,
    pub ask_volume: Decimal,
}

/// Last-24h and current ticker stats
#[derive(Debug, Deserialize, PartialEq)]
pub struct WSSTickerInfo {
    #[serde(rename(deserialize = "a"))]
    pub asks: TickerBidAsk,
    #[serde(rename(deserialize = "b"))]
    pub bids: TickerBidAsk,
    #[serde(rename(deserialize = "c"))]
    pub closed: LastTrade,
    #[serde(rename(deserialize = "v"))]
    pub volume: TickerDecimal,
    #[serde(rename(deserialize = "p"))]
    pub vwap: TickerDecimal,
    #[serde(rename(deserialize = "t"))]
    pub trades: TickerTrades,
    #[serde(rename(deserialize = "l"))]
    pub low: TickerDecimal,
    #[serde(rename(deserialize = "h"))]
    pub high: TickerDecimal,
    #[serde(rename(deserialize = "o"))]
    pub open: TickerDecimal,
}

impl<'de> Deserialize<'de> for PublicMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json = Value::deserialize(deserializer)?;

        match &json {
            Value::Object(o) => match get_event_field(o) {
                Some("heartbeat") => Ok(PublicMessage::Heartbeat),
                Some("ping") | Some("pong") => {
                    Ok(PublicMessage::PingPong(PingPong::deserialize(json).or(
                        Err(Error::custom("Invalid message with event: ping | pong")),
                    )?))
                }
                Some("systemStatus") => Ok(PublicMessage::SystemStatus(
                    SystemStatus::deserialize(json).or(Err(Error::custom(
                        "Invalid message with event: systemStatus",
                    )))?,
                )),
                Some("subscriptionStatus") => Ok(PublicMessage::SubscriptionStatus(
                    SubscriptionStatus::deserialize(json).or(Err(Error::custom(
                        "Invalid message with event: subscriptionStatus",
                    )))?,
                )),
                Some(field) => Err(Error::unknown_variant(
                    field,
                    &[
                        "heartbeat",
                        "ping",
                        "pong",
                        "systemStatus",
                        "subscriptionStatus",
                    ],
                )),
                None => Err(Error::missing_field("event")),
            },
            Value::Array(v) => match get_event_from_vec(v) {
                Some("trade") => Ok(PublicMessage::Trade(
                    Message::<Vec<PublicTrade>>::deserialize(json)
                        .or(Err(Error::custom("Invalid message with event: trade")))?,
                )),
                Some("spread") => Ok(PublicMessage::Spread(
                    Message::<Spread>::deserialize(json)
                        .or(Err(Error::custom("Invalid message with event: spread")))?,
                )),
                Some("ticker") => Ok(PublicMessage::Ticker(Box::new(
                    Message::<WSSTickerInfo>::deserialize(json).unwrap(),
                ))),
                Some(name) if name.starts_with("ohlc") => Ok(PublicMessage::OHLC(
                    Message::<OHLC>::deserialize(json)
                        .or(Err(Error::custom("Invalid message with event: ohlc")))?,
                )),
                Some(name) if name.starts_with("book") => {
                    if is_book_snapshot(v) {
                        Ok(PublicMessage::Orderbook(
                            Message::<Orderbook>::deserialize(json).or(Err(Error::custom(
                                "Invalid message with event: book (snapshot)",
                            )))?,
                        ))
                    } else {
                        Ok(PublicMessage::OrderbookUpdate(
                            OrderbookUpdateMessage::deserialize(json).or(Err(Error::custom(
                                "Invalid message with event: book (update)",
                            )))?,
                        ))
                    }
                }
                Some(field) => Err(Error::unknown_variant(
                    field,
                    &["trade", "spread", "ticker", "ohlc", "book"],
                )),
                None => Err(Error::custom(
                    "event was not second-to-last field of JSON array",
                )),
            },
            _ => Err(Error::custom(
                "Unexpected JSON value that was not an array or object",
            )),
        }
    }
}

/// Books are distinguished as a snapshot message if fields "as" or "bs" are present
///
/// Seeing "a", or "b" is an update message
fn is_book_snapshot(v: &[Value]) -> bool {
    let bids = find_book_sub_key(v, "bs");
    let asks = find_book_sub_key(v, "as");

    bids.is_some() || asks.is_some()
}

/// Attempt to pull out the second (index 1) JSON Value and get the requested key from it
fn find_book_sub_key<'a>(v: &'a [Value], key: &str) -> Option<&'a Value> {
    v.get(1)?.get(key)
}
