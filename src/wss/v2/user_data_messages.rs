use crate::request_types::{TimeInForce, TriggerType};
use crate::response_types::{
    BuySell, OrderStatus, OrderStatusV2, OrderType, PositionStatus, PositionStatusV2,
};
use crate::wss::v2::trading_messages::{
    ConditionalParams, FeePreference, PriceType, TriggerParams,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionResponseType {
    Snapshot,
    Update,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WalletType {
    Spot,
    Earn,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WalletId {
    Main,
    Flex,
    Bonded,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionType {
    PendingNew,
    New,
    Filled,
    Canceled,
    Expired,
    Trade,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Clone)]
pub struct ExecutionSubscriptionParams {
    pub channel: String,
    pub token: String,
    pub snapshot_trades: Option<bool>,
    pub rate_counter: Option<bool>,
    pub snapshot: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SubscriptionResult {
    pub channel: String,
    #[serde(rename = "maxratecount")]
    pub max_rate_count: Option<i64>, // TODO: not present on Balances ack, but that's fine to share?
    pub snapshot: Option<bool>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct ExecutionResponse {
    pub channel: String,
    #[serde(rename = "type")]
    pub execution_response_type: ExecutionResponseType,
}

#[derive(Debug, Deserialize)]
pub struct Fee {
    pub asset: String,
    #[serde(rename = "qty")]
    pub quantity: Decimal,
}

#[derive(Debug, Deserialize)]
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

// TODO: many params are optional, need to verify
#[derive(Debug, Deserialize)]
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
    pub average_price: Decimal,
    pub reason: Option<String>,
    #[serde(rename = "cum_cost")]
    pub cumulative_cost: Decimal,
    #[serde(rename = "cum_qty")]
    pub cumulative_quantity: Decimal,
    #[serde(rename = "display_qty")]
    pub display_quantity: Decimal,
    pub effective_time: String,
    pub expire_time: String,
    #[serde(rename = "fee_ccy_pref")]
    pub fee_preference: FeeCurrencyPreference,
    #[serde(rename = "fee_usd_equiv")]
    pub fee_usd_equivalent: Decimal,
    pub limit_price: Decimal,
    pub margin: bool,
    #[serde(rename = "no_mpp")]
    pub no_market_price_protection: bool,
    #[serde(rename = "ord_ref_id")]
    pub order_ref_id: i64,
    pub order_id: String,
    #[serde(rename = "order_qty")]
    pub order_quantity: Decimal,
    pub order_type: OrderType,
    pub order_status: OrderStatusV2,
    #[serde(rename = "order_userref")]
    pub order_user_ref: i64,
    pub post_only: bool,
    pub position_status: PositionStatusV2,
    pub reduce_only: bool,
    pub side: BuySell,
    pub symbol: String,
    pub time_in_force: TimeInForce,
    pub timestamp: String,
    pub trade_id: i64,
    pub triggers: TriggerDescription,
}

#[derive(Debug, Serialize, Clone)]
pub struct BalancesSubscriptionParams {
    pub channel: String,
    pub token: String,
    pub snapshot: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Wallet {
    pub balance: Decimal,
    #[serde(rename = "type")]
    pub wallet_type: WalletType,
    pub id: WalletId,
}

#[derive(Debug, Deserialize)]
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
