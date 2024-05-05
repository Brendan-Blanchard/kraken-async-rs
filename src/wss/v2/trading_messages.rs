use crate::request_types::TimeInForce;
use crate::response_types::{BuySell, OrderFlag, OrderType};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::formats::CommaSeparator;
use serde_with::serde_as;
use serde_with::StringWithSeparator;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
pub enum AddOrderStatus {
    Ok,
    Err,
}

/// Enum representing various ways of specifying a real or relative price
#[derive(Debug, Eq, PartialEq, Clone, Serialize)]
pub enum Price {
    /// Real price as a decimal value, e.g. `dec!(65123.20)`
    Decimal(Decimal),
    /// Relative price, specified as a String
    ///
    /// +5.1 - the price should be 5.1 (in quote currency) greater than the last traded price
    /// -5.1 - the price should be 5.1 (in quote currency) less than the last traded price
    /// #5.1 - the price should be 5.1 (in quote currency) less than or greater than the last traded price (set depending on the direction of the order)
    /// +1.2% - the price should be 1.2% greater than the last traded price
    Relative(String),
}

// TODO: impl From<Decimal> for Price
// TODO: impl From<String> for Price

#[serde_as]
#[derive(Debug, Serialize)]
pub struct AddOrderRequest {
    pub event: String,
    #[serde(rename = "ordertype")]
    pub order_type: OrderType,
    #[serde(rename = "type")]
    pub side: BuySell,
    pub pair: String,
    pub price: Price,
    #[serde(rename = "price2")]
    pub price_2: Price,
    pub volume: Decimal,
    pub reduce_only: Option<bool>,
    #[serde(rename = "oflags")]
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, OrderFlag>>")]
    pub order_flags: Option<Vec<OrderFlag>>,
    #[serde(rename = "starttm")]
    pub start_time: Option<String>,
    #[serde(rename = "expiretm")]
    pub expire_time: Option<String>,
    pub deadline: String,
    #[serde(rename = "userref")]
    pub user_ref: String,
    pub validate: String, // TODO: maybe bool? docs say String...
    #[serde(rename = "timeinforce")]
    pub time_in_force: TimeInForce,
    #[serde(rename = "close[ordertype]")]
    pub close_order_type: OrderType,
    #[serde(rename = "close[price2]")]
    pub close_price: Price,
    #[serde(rename = "close[price2]")]
    pub close_price_2: Price,
    #[serde(rename = "reqid")]
    pub req_id: i64,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct AddOrderResponse {
    pub event: String,
    #[serde(rename = "txid")]
    pub tx_id: String,
    pub descr: String,
    pub status: AddOrderStatus,
    #[serde(rename = "reqid")]
    pub req_id: i64,
    #[serde(rename = "errorMessage")]
    pub error_message: String,
}
