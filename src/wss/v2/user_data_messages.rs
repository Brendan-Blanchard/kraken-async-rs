use crate::request_types::{TimeInForce, TriggerType};
use crate::response_types::{BuySell, OrderStatusV2, OrderType, PositionStatusV2};
use crate::wss::v2::market_data_messages::{
    BookSubscriptionResponse, OhlcSubscriptionResponse, TickerSubscriptionResponse,
    TradeSubscriptionResponse,
};
use crate::wss::v2::trading_messages::{ConditionalParams, PriceType};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionResponseType {
    Snapshot,
    Update,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MakerTaker {
    #[serde(rename = "m")]
    Maker,
    #[serde(rename = "t")]
    Taker,
}

// TODO: From/Into or a shared Ser/De impl for both FeeCurrencyPreference and FeePreference

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum FeeCurrencyPreference {
    #[serde(rename = "fcib")]
    Base,
    #[serde(rename = "fciq")]
    Quote,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WalletType {
    Spot,
    Earn,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WalletId {
    Main,
    Flex,
    Bonded,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionType {
    PendingNew,
    New,
    Filled,
    Canceled,
    Expired,
    Trade,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TriggerStatus {
    Triggered,
    Untriggered,
}

#[derive(Debug, Serialize, Clone)]
pub struct SubscriptionRequest<T> {
    pub method: String,
    pub params: T,
    pub req_id: Option<i64>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Clone)]
pub struct ExecutionSubscription {
    pub channel: String,
    pub token: String,
    pub snapshot_trades: Option<bool>,
    pub rate_counter: Option<bool>,
    pub snapshot: Option<bool>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InstrumentSubscriptionResult {
    pub snapshot: Option<bool>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ExecutionsSubscriptionResult {
    #[serde(rename = "maxratecount")]
    pub max_rate_count: Option<i64>,
    pub snapshot: Option<bool>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "channel")]
pub enum SubscriptionResult {
    #[serde(alias = "level3")]
    #[serde(rename = "book")]
    Book(BookSubscriptionResponse),
    #[serde(rename = "ticker")]
    Ticker(TickerSubscriptionResponse),
    #[serde(rename = "ohlc")]
    Ohlc(OhlcSubscriptionResponse),
    #[serde(rename = "trade")]
    Trade(TradeSubscriptionResponse),
    #[serde(rename = "executions")]
    Execution(ExecutionsSubscriptionResult),
    #[serde(rename = "instrument")]
    Instrument(InstrumentSubscriptionResult),
}

#[derive(Debug, Deserialize)]
pub struct ExecutionResponse {
    pub channel: String,
    #[serde(rename = "type")]
    pub execution_response_type: ExecutionResponseType,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Fee {
    pub asset: String,
    #[serde(rename = "qty")]
    pub quantity: Decimal,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TriggerDescription {
    pub reference: TriggerType,
    pub price: Decimal,
    pub price_type: PriceType,
    pub actual_price: Decimal,
    pub peak_price: Decimal,
    pub last_price: Decimal,
    pub status: TriggerStatus,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ExecutionResult {
    #[serde(rename = "exec_type")]
    pub execution_type: ExecutionType,
    #[serde(rename = "cash_order_qty")]
    pub cash_order_quantity: Option<Decimal>,
    pub contingent: Option<ConditionalParams>,
    pub cost: Option<Decimal>,
    #[serde(rename = "exec_id")]
    pub execution_id: Option<String>,
    pub fees: Option<Vec<Fee>>,
    #[serde(rename = "liquidity_ind")]
    pub liquidity_indicator: Option<MakerTaker>,
    pub last_price: Option<Decimal>,
    #[serde(rename = "last_qty")]
    pub last_quantity: Option<Decimal>,
    #[serde(rename = "avg_price")]
    pub average_price: Option<Decimal>,
    pub reason: Option<String>,
    #[serde(rename = "cum_cost")]
    pub cumulative_cost: Option<Decimal>,
    #[serde(rename = "cum_qty")]
    pub cumulative_quantity: Option<Decimal>,
    #[serde(rename = "display_qty")]
    pub display_quantity: Option<Decimal>,
    pub effective_time: Option<String>,
    pub expire_time: Option<String>,
    #[serde(rename = "fee_ccy_pref")]
    pub fee_preference: Option<FeeCurrencyPreference>,
    #[serde(rename = "fee_usd_equiv")]
    pub fee_usd_equivalent: Option<Decimal>,
    pub limit_price: Option<Decimal>,
    pub margin: Option<bool>,
    #[serde(rename = "no_mpp")]
    pub no_market_price_protection: Option<bool>,
    #[serde(rename = "ord_ref_id")]
    pub order_ref_id: Option<i64>,
    pub order_id: String,
    #[serde(rename = "order_qty")]
    pub order_quantity: Option<Decimal>,
    pub order_type: Option<OrderType>,
    pub order_status: OrderStatusV2,
    #[serde(rename = "order_userref")]
    pub order_user_ref: i64,
    pub post_only: Option<bool>,
    pub position_status: Option<PositionStatusV2>,
    pub reduce_only: Option<bool>,
    pub side: Option<BuySell>,
    pub symbol: Option<String>,
    pub time_in_force: Option<TimeInForce>,
    pub timestamp: String,
    pub trade_id: Option<i64>,
    pub triggers: Option<TriggerDescription>,
}

#[derive(Debug, Serialize, Clone)]
pub struct BalancesSubscription {
    pub channel: String,
    pub token: String,
    pub snapshot: Option<bool>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Wallet {
    pub balance: Decimal,
    #[serde(rename = "type")]
    pub wallet_type: WalletType,
    pub id: WalletId,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Balance {
    pub asset: String,
    pub balance: Decimal,
    pub wallets: Option<Vec<Wallet>>,
}

#[derive(Debug, Deserialize)]
pub struct BalancesResponse {
    pub channel: String,
    #[serde(rename = "type")]
    pub channel_type: String,
    pub data: Vec<Balance>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_deserializing_execution_trade() {
        let message = r#"{"order_id":"O7IBL5-O2V6X-EEXY4U","order_userref":0,"exec_id":"TJE7HC-DKBTI-5BFVKE","exec_type":"trade","trade_id":365573,"symbol":"KAR/USD","side":"buy","last_qty":105.02014889,"last_price":0.121,"liquidity_ind":"t","cost":12.70744,"order_status":"filled","order_type":"limit","timestamp":"2024-05-18T05:41:33.480251Z","fee_usd_equiv":0.05083,"fees":[{"asset":"USD","qty":0.05083}]}"#;
        let expected = ExecutionResult {
            execution_type: ExecutionType::Trade,
            cash_order_quantity: None,
            contingent: None,
            cost: Some(dec!(12.70744)),
            execution_id: Some("TJE7HC-DKBTI-5BFVKE".to_string()),
            fees: Some(vec![Fee {
                asset: "USD".to_string(),
                quantity: dec!(0.05083),
            }]),
            liquidity_indicator: Some(MakerTaker::Taker),
            last_price: Some(dec!(0.121)),
            last_quantity: Some(dec!(105.02014889)),
            average_price: None,
            reason: None,
            cumulative_cost: None,
            cumulative_quantity: None,
            display_quantity: None,
            effective_time: None,
            expire_time: None,
            fee_preference: None,
            fee_usd_equivalent: Some(dec!(0.05083)),
            limit_price: None,
            margin: None,
            no_market_price_protection: None,
            order_ref_id: None,
            order_id: "O7IBL5-O2V6X-EEXY4U".to_string(),
            order_quantity: None,
            order_type: Some(OrderType::Limit),
            order_status: OrderStatusV2::Filled,
            order_user_ref: 0,
            post_only: None,
            position_status: None,
            reduce_only: None,
            side: Some(BuySell::Buy),
            symbol: Some("KAR/USD".to_string()),
            time_in_force: None,
            timestamp: "2024-05-18T05:41:33.480251Z".to_string(),
            trade_id: Some(365573),
            triggers: None,
        };
        let parsed: ExecutionResult = serde_json::from_str(message).unwrap();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn test_deserializing_execution_new_update() {
        let message = r#"{"timestamp":"2024-05-18T11:00:37.240691Z","order_status":"new","exec_type":"new","order_userref":0,"order_id":"OLADEP-E5D5S-IKEHMF"}"#;
        let expected = ExecutionResult {
            execution_type: ExecutionType::New,
            cash_order_quantity: None,
            contingent: None,
            cost: None,
            execution_id: None,
            fees: None,
            liquidity_indicator: None,
            last_price: None,
            last_quantity: None,
            average_price: None,
            reason: None,
            cumulative_cost: None,
            cumulative_quantity: None,
            display_quantity: None,
            effective_time: None,
            expire_time: None,
            fee_preference: None,
            fee_usd_equivalent: None,
            limit_price: None,
            margin: None,
            no_market_price_protection: None,
            order_ref_id: None,
            order_id: "OLADEP-E5D5S-IKEHMF".to_string(),
            order_quantity: None,
            order_type: None,
            order_status: OrderStatusV2::New,
            order_user_ref: 0,
            post_only: None,
            position_status: None,
            reduce_only: None,
            side: None,
            symbol: None,
            time_in_force: None,
            timestamp: "2024-05-18T11:00:37.240691Z".to_string(),
            trade_id: None,
            triggers: None,
        };
        let parsed: ExecutionResult = serde_json::from_str(message).unwrap();

        assert_eq!(expected, parsed);
    }
}
