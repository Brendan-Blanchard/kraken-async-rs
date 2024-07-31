use crate::crypto::secrets::Token;
use crate::request_types::{IntOrString, SelfTradePrevention, TimeInForceV2, TriggerType};
use crate::response_types::{BuySell, OrderType};
use rust_decimal::serde::{float, float_option};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
pub enum AddOrderStatus {
    Ok,
    Err,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FeePreference {
    #[serde(rename(deserialize = "fcib"))]
    Base,
    #[serde(rename(deserialize = "fciq"))]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConditionalParams {
    pub order_type: Option<OrderType>,
    pub limit_price: Option<Decimal>,
    pub limit_price_type: Option<PriceType>,
    pub trigger_price: Option<Decimal>,
    pub trigger_price_type: Option<PriceType>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct AddOrderParams {
    pub order_type: OrderType,
    pub side: BuySell,
    pub symbol: String,
    #[serde(with = "float_option")]
    pub limit_price: Option<Decimal>,
    pub limit_price_type: Option<PriceType>,
    pub triggers: Option<TriggerParams>,
    pub time_in_force: Option<TimeInForceV2>,
    #[serde(with = "float")]
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
    #[serde(with = "float_option")]
    #[serde(rename = "display_qty")]
    pub display_quantity: Option<Decimal>,
    pub fee_preference: Option<FeePreference>,
    #[serde(rename = "no_mpp")]
    pub no_market_price_protection: Option<bool>,
    pub stp_type: Option<SelfTradePrevention>,
    #[serde(with = "float_option")]
    #[serde(rename = "cash_order_qty")]
    pub cash_order_quantity: Option<Decimal>,
    pub validate: Option<bool>,
    pub token: Token,
    #[serde(rename = "cl_ord_id")]
    pub client_order_id: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AddOrderResult {
    pub order_id: String,
    #[serde(rename = "order_userref")]
    pub order_user_ref: Option<i64>,
    pub warning: Option<Vec<String>>,
    #[serde(rename = "cl_ord_id")]
    pub client_order_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct EditOrderParams {
    pub deadline: Option<String>,
    #[serde(with = "float_option")]
    #[serde(rename = "display_qty")]
    pub display_quantity: Option<Decimal>,
    pub fee_preference: Option<FeePreference>,
    #[serde(with = "float_option")]
    pub limit_price: Option<Decimal>,
    #[serde(rename = "no_mpp")]
    pub no_market_price_protection: Option<bool>,
    pub order_id: String,
    #[serde(with = "float_option")]
    #[serde(rename = "order_qty")]
    pub order_quantity: Option<Decimal>,
    #[serde(rename = "order_userref")]
    pub order_user_ref: Option<i64>,
    pub post_only: Option<bool>,
    pub reduce_only: Option<bool>,
    pub symbol: String,
    pub triggers: Option<TriggerParams>,
    pub validate: Option<bool>,
    pub token: Token,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct EditOrderResult {
    pub order_id: String,
    pub original_order_id: String,
    pub warning: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CancelOrderParams {
    pub order_id: Option<Vec<String>>,
    #[serde(rename = "cl_ord_id")]
    pub client_order_id: Option<Vec<String>>,
    #[serde(rename = "order_userref")]
    pub order_user_ref: Option<Vec<i64>>,
    pub token: Token,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CancelOrderResult {
    pub order_id: String,
    pub warning: Option<Vec<String>>,
    #[serde(rename = "cl_ord_id")]
    pub client_order_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelAllOrdersParams {
    pub token: Token,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CancelAllOrdersResult {
    pub count: i32,
    pub warning: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelOnDisconnectParams {
    pub timeout: i64,
    pub token: Token,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CancelOnDisconnectResult {
    #[serde(rename = "currentTime")]
    pub current_time: String,
    #[serde(rename = "triggerTime")]
    pub trigger_time: String,
    pub warning: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchOrder {
    pub order_type: OrderType,
    pub side: BuySell,
    #[serde(with = "float_option")]
    pub limit_price: Option<Decimal>,
    pub limit_price_type: Option<PriceType>,
    pub triggers: Option<TriggerParams>,
    pub time_in_force: Option<TimeInForceV2>,
    #[serde(with = "float")]
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
    #[serde(with = "float_option")]
    #[serde(rename = "display_qty")]
    pub display_quantity: Option<Decimal>,
    pub fee_preference: Option<FeePreference>,
    #[serde(rename = "no_mpp")]
    pub no_market_price_protection: Option<bool>,
    pub stp_type: Option<SelfTradePrevention>,
    #[serde(with = "float_option")]
    #[serde(rename = "cash_order_qty")]
    pub cash_order_quantity: Option<Decimal>,
    #[serde(rename = "cl_ord_id")]
    pub client_order_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchOrderParams {
    pub deadline: Option<String>,
    pub symbol: String,
    pub validate: Option<bool>,
    pub token: Token,
    pub orders: Vec<BatchOrder>,
}

#[derive(Debug, Serialize)]
pub struct BatchCancelParams {
    pub orders: Vec<IntOrString>,
    pub token: Token,
    #[serde(rename = "cl_ord_id")]
    pub client_order_id: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct BatchCancelResult {
    pub count: i32,
    pub warning: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct BatchCancelResponse {
    pub orders_cancelled: i64,
    pub error: Option<String>,
    pub success: bool,
    pub req_id: i64,
    pub time_in: String,
    pub time_out: String,
    #[serde(rename = "cl_ord_id")]
    pub client_order_id: Option<Vec<String>>,
}
