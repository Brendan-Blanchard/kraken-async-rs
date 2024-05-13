use crate::response_types::BuySell;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "lowercase")]
pub enum EventTrigger {
    Bbo,
    Trades,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "lowercase")]
pub enum OrderbookEvent {
    Add,
    Modify,
    Delete,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Copy)]
pub enum MarketLimit {
    Market,
    Limit,
}

#[derive(Debug, Serialize)]
pub struct TickerSubscription {
    pub channel: String,
    pub symbol: Vec<String>,
    pub event_trigger: Option<EventTrigger>,
    pub snapshot: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct TickerSubscriptionResponse {
    pub channel: String,
    pub symbol: String,
    pub snapshot: Option<bool>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Ticker {
    pub ask: Decimal,
    pub ask_qty: Decimal,
    pub bid: Decimal,
    pub bid_qty: Decimal,
    pub change: Decimal,
    pub change_pct: Decimal,
    pub high: Decimal,
    pub last: Decimal,
    pub low: Decimal,
    pub symbol: String,
    pub volume: Decimal,
    pub vwap: Decimal,
}

#[derive(Debug, Serialize)]
pub struct BookSubscription {
    pub channel: String,
    pub symbol: Vec<String>,
    pub depth: Option<i32>,
    pub snapshot: Option<bool>,
    /// only needed for L3 subscription
    pub token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BookSubscriptionResponse {
    pub channel: String,
    pub symbol: String,
    pub depth: i32,
    pub snapshot: Option<bool>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct BidAsk {
    pub price: Decimal,
    #[serde(rename = "qty")]
    pub quantity: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct Orderbook {
    pub bids: Vec<BidAsk>,
    pub asks: Vec<BidAsk>,
}

#[derive(Debug, Deserialize)]
pub struct OrderbookSnapshot {
    pub channel: String,
    #[serde(rename = "type")]
    pub message_type: String,
    pub data: [Orderbook; 1],
    pub checksum: i64,
    pub symbol: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderbookUpdate {
    pub channel: String,
    #[serde(rename = "type")]
    pub message_type: String,
    pub data: [Orderbook; 1],
    pub checksum: i64,
    pub symbol: String,
    pub timestamp: String,
}

#[derive(Debug, Deserialize)]
pub struct L3BidAsk {
    pub order_id: String,
    pub limit_price: Decimal,
    #[serde(rename = "order_qty")]
    pub order_quantity: Decimal,
    pub timestamp: String,
}

#[derive(Debug, Deserialize)]
pub struct L3Orderbook {
    pub symbol: String,
    pub bids: Vec<L3BidAsk>,
    pub asks: Vec<L3BidAsk>,
    pub checksum: i64,
}

#[derive(Debug, Deserialize)]
pub struct L3OrderbookUpdateMessage {
    pub channel: String,
    #[serde(rename = "type")]
    pub message_type: String,
    pub data: [L3Orderbook; 1],
}

#[derive(Debug, Deserialize)]
pub struct L3OrderbookUpdate {
    pub symbol: String,
    pub bids: Vec<L3BidAskUpdate>,
    pub asks: Vec<L3BidAskUpdate>,
    pub checksum: i64,
}

#[derive(Debug, Deserialize)]
pub struct L3BidAskUpdate {
    pub event: OrderbookEvent,
    pub order_id: String,
    pub limit_price: Decimal,
    #[serde(rename = "order_qty")]
    pub order_quantity: Decimal,
    pub timestamp: String,
}

#[derive(Debug, Serialize)]
pub struct CandlesSubscription {
    pub channel: String,
    pub symbol: Vec<String>,
    pub interval: i32,
    pub snapshot: Option<bool>,
}

// TODO: applies to several, like Candles and Trades, maybe MarketDataSubscription?
#[derive(Debug, Deserialize)]
pub struct SubscriptionResponse {
    pub channel: String,
    pub symbol: String,
    pub snapshot: Option<bool>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Candle {
    pub symbol: String,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub vwap: Decimal,
    pub trades: Decimal,
    pub volume: Decimal,
    pub interval_begin: String,
    pub interval: i32,
}

#[derive(Debug, Serialize)]
pub struct TradesSubscription {
    pub channel: String,
    pub symbol: Vec<String>,
    pub snapshot: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Trade {
    pub symbol: String,
    pub side: BuySell,
    #[serde(rename = "qty")]
    pub quantity: Decimal,
    pub price: Decimal,
    #[serde(rename = "ord_type")]
    pub order_type: MarketLimit,
    pub trade_id: i64,
    pub timestamp: String,
}

// TODO: Instruments channel, subscriptions and responses
