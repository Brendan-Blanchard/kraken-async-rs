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

impl TickerSubscription {
    pub fn new(symbol: Vec<String>) -> Self {
        TickerSubscription {
            channel: "ticker".to_string(),
            symbol,
            event_trigger: None,
            snapshot: None,
        }
    }
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

impl BookSubscription {
    pub fn new(symbol: Vec<String>) -> Self {
        BookSubscription {
            channel: "book".to_string(),
            symbol,
            depth: None,
            snapshot: None,
            token: None,
        }
    }

    pub fn new_l3(symbol: Vec<String>, token: String) -> Self {
        BookSubscription {
            channel: "level3".to_string(),
            symbol,
            depth: None,
            snapshot: None,
            token: Some(token),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum L2 {
    Orderbook(Orderbook),
    Update(OrderbookUpdate),
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
#[serde(untagged)]
pub enum L3 {
    Orderbook(L3Orderbook),
    Update(L3OrderbookUpdate),
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct L3Orderbook {
    pub symbol: String,
    pub bids: Vec<L3BidAsk>,
    pub asks: Vec<L3BidAsk>,
    pub checksum: i64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct L3OrderbookUpdate {
    pub symbol: String,
    pub bids: Vec<L3BidAskUpdate>,
    pub asks: Vec<L3BidAskUpdate>,
    pub checksum: i64,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct L3BidAsk {
    pub order_id: String,
    pub limit_price: Decimal,
    #[serde(rename = "order_qty")]
    pub order_quantity: Decimal,
    pub timestamp: String,
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
pub struct OhlcSubscription {
    pub channel: String,
    pub symbol: Vec<String>,
    pub interval: i32,
    pub snapshot: Option<bool>,
}

impl OhlcSubscription {
    pub fn new(symbols: Vec<String>, interval: i32) -> Self {
        OhlcSubscription {
            channel: "ohlc".to_string(),
            symbol: symbols,
            interval,
            snapshot: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SubscriptionResponse {
    pub channel: String,
    pub symbol: Option<String>,
    pub snapshot: Option<bool>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TradeSubscriptionResponse {
    pub symbol: Option<String>,
    pub snapshot: Option<bool>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct OhlcSubscriptionResponse {
    pub symbol: Option<String>,
    pub snapshot: Option<bool>,
    pub interval: i64,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct BookSubscriptionResponse {
    pub symbol: String,
    pub depth: Option<i32>,
    pub snapshot: Option<bool>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TickerSubscriptionResponse {
    pub symbol: String,
    pub event_trigger: Option<EventTrigger>,
    pub snapshot: Option<bool>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Ohlc {
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

impl TradesSubscription {
    pub fn new(symbols: Vec<String>) -> Self {
        TradesSubscription {
            channel: "trade".to_string(),
            symbol: symbols,
            snapshot: None,
        }
    }
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

#[derive(Debug, Deserialize, PartialEq)]
pub struct Asset {
    pub id: String,
    pub margin_rate: Option<Decimal>,
    pub precision: i64,
    pub precision_display: i64,
    pub status: AssetStatus,
    pub borrowable: bool,
    pub collateral_value: Decimal,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Pair {
    pub base: String,
    pub quote: String,
    pub cost_min: Decimal,
    pub cost_precision: i64,
    pub has_index: bool,
    pub margin_initial: Option<Decimal>,
    pub marginable: bool,
    pub position_limit_long: Option<i64>,
    pub position_limit_short: Option<i64>,
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

#[derive(Debug, Deserialize, PartialEq)]
pub struct Instruments {
    pub assets: Vec<Asset>,
    pub pairs: Vec<Pair>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use rust_decimal_macros::dec;

    #[test]
    fn test_deserialize_asset() {
        let raw = r#"{"id":"XLM","status":"enabled","precision":8,"precision_display":5,"borrowable":true,"collateral_value":0.00,"margin_rate":0.020000}"#;
        let expected = Asset {
            id: "XLM".to_string(),
            margin_rate: Some(dec!(0.02)),
            precision: 8,
            precision_display: 5,
            status: AssetStatus::Enabled,
            borrowable: true,
            collateral_value: dec!(0),
        };

        let deserialized = serde_json::from_str::<Asset>(raw).unwrap();

        assert_eq!(expected, deserialized);
    }

    #[test]
    fn test_deserialize_pair() {
        let raw = r#"{"symbol":"ETH/BTC","base":"ETH","quote":"BTC","status":"online","qty_precision":8,"qty_increment":0.00000001,"price_precision":5,"cost_precision":10,"marginable":true,"has_index":true,"cost_min":0.00002,"margin_initial":0.20,"position_limit_long":1000,"position_limit_short":600,"tick_size":0.00001,"price_increment":0.00001,"qty_min":0.00200000}"#;
        let expected = Pair {
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
        };

        let deserialized = serde_json::from_str::<Pair>(raw).unwrap();

        assert_eq!(expected, deserialized);
    }
}
