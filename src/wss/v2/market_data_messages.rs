use crate::response_types::BuySell;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

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
#[serde(rename_all = "lowercase")]
pub enum MarketLimit {
    Market,
    Limit,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "lowercase")]
pub enum AssetStatus {
    DepositOnly,
    Disabled,
    Enabled,
    FundingTemporarilyDisabled,
    WithdrawalOnly,
    WorkInProgress,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "snake_case")]
pub enum PairStatus {
    CancelOnly,
    Delisted,
    LimitOnly,
    Maintenance,
    Online,
    PostOnly,
    ReduceOnly,
    WorkInProgress,
}

#[serde_as]
#[skip_serializing_none]
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

#[derive(Debug, Deserialize, PartialEq)]
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

#[serde_as]
#[skip_serializing_none]
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

#[derive(Debug, Deserialize, PartialEq)]
pub struct BidAsk {
    pub price: Decimal,
    #[serde(rename = "qty")]
    pub quantity: Decimal,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Orderbook {
    pub symbol: String,
    pub checksum: i64,
    pub bids: Vec<BidAsk>,
    pub asks: Vec<BidAsk>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct OrderbookUpdate {
    pub symbol: String,
    pub checksum: i64,
    pub timestamp: String,
    pub bids: Vec<BidAsk>,
    pub asks: Vec<BidAsk>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct L3BidAsk {
    pub order_id: String,
    pub limit_price: Decimal,
    #[serde(rename = "order_qty")]
    pub order_quantity: Decimal,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct L3Orderbook {
    pub symbol: String,
    pub bids: Vec<L3BidAsk>,
    pub asks: Vec<L3BidAsk>,
    pub checksum: i64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct L3OrderbookUpdateMessage {
    pub channel: String,
    #[serde(rename = "type")]
    pub message_type: String,
    pub data: [L3Orderbook; 1],
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct L3OrderbookUpdate {
    pub symbol: String,
    pub bids: Vec<L3BidAskUpdate>,
    pub asks: Vec<L3BidAskUpdate>,
    pub checksum: i64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct L3BidAskUpdate {
    pub event: OrderbookEvent,
    pub order_id: String,
    pub limit_price: Decimal,
    #[serde(rename = "order_qty")]
    pub order_quantity: Decimal,
    pub timestamp: String,
}

#[serde_as]
#[skip_serializing_none]
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

#[derive(Debug, Deserialize, PartialEq)]
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

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct TradesSubscription {
    pub channel: String,
    pub symbol: Vec<String>,
    pub snapshot: Option<bool>,
}

#[derive(Debug, Deserialize, PartialEq)]
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

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct InstrumentsSubscription {
    pub channel: String,
    pub snapshot: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct InstrumentsSubscriptionResponse {
    pub channel: String,
    pub snapshot: Option<bool>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Asset {
    pub id: String,
    pub margin_rate: Decimal,
    pub precision: i64,
    pub precision_display: i64,
    pub status: AssetStatus,
    pub borrowable: bool,
    pub collateral_value: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct Pair {
    pub base: String,
    pub cost_min: String,
    pub cost_precision: String,
    pub has_index: bool,
    pub margin_initial: Decimal,
    pub marginable: bool,
    pub position_limit_long: i64,
    pub position_limit_short: i64,
    pub price_increment: Decimal,
    pub price_precision: i64,
    #[serde(rename = "qty_increment")]
    pub quantity_increment: Decimal,
    #[serde(rename = "qty_min")]
    pub quantity_min: Decimal,
    #[serde(rename = "qty_precision")]
    pub quantity_precision: i64,
    pub status: PairStatus,
    pub symbol: String,
}

#[derive(Debug, Deserialize)]
pub struct Instruments {
    pub assets: Vec<Asset>,
    pub pairs: Vec<Pair>,
}
