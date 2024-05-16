use crate::request_types::{IntOrString, SelfTradePrevention, TimeInForceV2, TriggerType};
use crate::response_types::{BuySell, OrderType};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
pub enum AddOrderStatus {
    Ok,
    Err,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FeePreference {
    Base,
    Quote,
}

/// Type of price given in the `limit_price` field
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum PriceType {
    /// Static or real price, e.g. 65123.29 for BTC/USD
    #[serde(rename = "static")]
    Static,
    /// Percent different from the previous price, e.g. -5.0 (%)
    #[serde(rename = "pct")]
    Percent,
    /// Quote/notional difference from the last traded price, e.g. -500, 150, etc
    #[serde(rename = "quote")]
    Quote,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerParams {
    pub price: Decimal,
    pub price_type: Option<PriceType>,
    pub reference: Option<TriggerType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConditionalParams {
    pub order_type: Option<OrderType>,
    pub limit_price: Option<Decimal>,
    pub limit_price_type: Option<PriceType>,
    pub trigger_price: Option<Decimal>,
    pub trigger_price_type: Option<PriceType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddOrderParams {
    pub order_type: OrderType,
    pub side: BuySell,
    pub symbol: String,
    pub limit_price: Option<Decimal>,
    pub limit_price_type: Option<PriceType>,
    pub triggers: Option<TriggerParams>,
    pub time_in_force: Option<TimeInForceV2>,
    #[serde(rename = "order_qty")]
    pub order_quantity: Decimal,
    pub margin: Option<bool>,
    pub post_only: Option<bool>,
    pub reduce_only: Option<bool>,
    pub effective_time: Option<String>, // RFC3339
    pub expire_time: Option<String>,
    pub deadline: Option<String>,
    #[serde(rename = "order_userref")]
    pub order_user_ref: Option<i64>,
    pub conditional: Option<ConditionalParams>,
    #[serde(rename = "display_qty")]
    pub display_quantity: Option<Decimal>,
    pub fee_preference: Option<FeePreference>,
    #[serde(rename = "no_mpp")]
    pub no_market_price_protection: Option<bool>,
    pub stp_type: Option<SelfTradePrevention>,
    #[serde(rename = "cash_order_qty")]
    pub cash_order_quantity: Option<Decimal>,
    pub validate: Option<bool>,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct AddOrderResult {
    pub order_id: String,
    #[serde(rename = "order_userref")]
    pub order_user_ref: String,
    pub warning: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditOrderParams {
    pub deadline: String,
    #[serde(rename = "display_qty")]
    pub display_quantity: Option<Decimal>,
    pub fee_preference: Option<FeePreference>,
    pub limit_price: Option<Decimal>,
    #[serde(rename = "no_mpp")]
    pub no_market_price_protection: Option<bool>,
    pub order_id: String,
    #[serde(rename = "order_qty")]
    pub order_quantity: Option<Decimal>,
    #[serde(rename = "order_userref")]
    pub order_user_ref: Option<i64>,
    pub post_only: Option<bool>,
    pub reduce_only: Option<bool>,
    pub symbol: String,
    pub triggers: Option<TriggerParams>,
    pub validate: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct EditOrderResult {
    pub order_id: String,
    pub original_order_id: String,
    pub warning: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelOrderParams {
    pub order_id: Vec<String>,
    #[serde(rename = "order_userref")]
    pub order_user_ref: Option<i64>,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct CancelOrderResult {
    pub order_id: String,
    pub warning: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelAllOrdersParams {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct CancelAllOrdersResult {
    pub count: i32,
    pub warning: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelOnDisconnectParams {
    pub timeout: i64,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct CancelOnDisconnectResult {
    #[serde(rename = "currentTime")]
    pub current_time: String,
    #[serde(rename = "triggerTime")]
    pub trigger_time: String,
    pub warning: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchOrder {
    pub order_type: OrderType,
    pub side: BuySell,
    pub limit_price: Option<Decimal>,
    pub limit_price_type: Option<PriceType>,
    pub triggers: Option<TriggerParams>,
    pub time_in_force: Option<TimeInForceV2>,
    #[serde(rename = "order_qty")]
    pub order_quantity: Decimal,
    pub margin: Option<bool>,
    pub post_only: Option<bool>,
    pub reduce_only: Option<bool>,
    pub effective_time: Option<String>, // RFC3339
    pub expire_time: Option<String>,
    #[serde(rename = "order_userref")]
    pub order_user_ref: Option<i64>,
    pub conditional: Option<ConditionalParams>,
    #[serde(rename = "display_qty")]
    pub display_quantity: Option<Decimal>,
    pub fee_preference: Option<FeePreference>,
    #[serde(rename = "no_mpp")]
    pub no_market_price_protection: Option<bool>,
    pub stp_type: Option<SelfTradePrevention>,
    #[serde(rename = "cash_order_qty")]
    pub cash_order_quantity: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchOrderParams {
    pub deadline: Option<String>,
    pub symbol: String,
    pub validate: Option<bool>,
    pub token: String,
    pub orders: Vec<BatchOrder>,
}

pub type BatchOrderResult = Vec<AddOrderResult>;

#[derive(Debug, Serialize)]
pub struct BatchCancelParams {
    pub orders: Vec<IntOrString>,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct BatchCancelResult {
    pub count: i32,
    pub warning: Vec<String>,
}
