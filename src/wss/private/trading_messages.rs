//! Trading requests and responses
use crate::request_types::TimeInForce;
use crate::response_types::{BuySell, OrderFlag, OrderType};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::formats::CommaSeparator;
use serde_with::serde_as;
use serde_with::skip_serializing_none;
use serde_with::StringWithSeparator;
use simple_builder::Builder;
use std::fmt::{Display, Formatter};

/// Request send via websocket to add an order
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, PartialEq, Eq, Builder)]
pub struct AddOrderRequest {
    #[builder(required)]
    pub event: String,
    #[builder(required)]
    pub token: String,
    #[builder(required)]
    #[serde(rename = "ordertype")]
    pub order_type: OrderType,
    #[builder(required)]
    #[serde(rename = "type")]
    pub side: BuySell,
    #[builder(required)]
    pub pair: String,
    #[builder(required)]
    pub volume: Decimal,
    #[serde(rename = "reqid")]
    pub req_id: Option<i64>,
    pub price: Option<Decimal>,
    #[serde(rename = "price2")]
    pub price_2: Option<Decimal>,
    pub leverage: Option<i64>,
    pub reduce_only: Option<bool>,
    #[serde(rename = "oflags")]
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, OrderFlag>>")]
    pub order_flags: Option<Vec<OrderFlag>>,
    #[serde(rename = "starttm")]
    pub start_time: Option<String>,
    #[serde(rename = "expiretm")]
    pub expire_time: Option<String>,
    pub deadline: Option<String>,
    #[serde(rename = "userref")]
    pub user_ref: Option<String>,
    pub validate: Option<String>,
    #[serde(rename = "close[ordertype]")]
    pub close_order_type: Option<OrderType>,
    #[serde(rename = "close[price]")]
    pub close_price: Option<Decimal>,
    #[serde(rename = "close[price2]")]
    pub close_price_2: Option<Decimal>,
    #[serde(rename = "timeinforce")]
    pub time_in_force: Option<TimeInForce>,
}

/// Status of an add order request
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum OrderRequestStatus {
    Ok,
    Error,
}

impl Display for OrderRequestStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderRequestStatus::Ok => write!(f, "ok"),
            OrderRequestStatus::Error => write!(f, "error"),
        }
    }
}

/// Response type for adding an order
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct AddOrderResponse {
    pub event: String,
    #[serde(rename = "reqid")]
    pub req_id: Option<i64>,
    pub status: OrderRequestStatus,
    #[serde(rename = "txid")]
    pub tx_id: Option<String>,
    pub descr: Option<String>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}

/// Request for editing an existing order
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, PartialEq, Builder)]
pub struct EditOrderRequest {
    #[builder(required)]
    pub event: String,
    #[builder(required)]
    pub token: String,
    #[serde(rename = "orderid")]
    pub order_id: Option<String>,
    #[serde(rename = "reqid")]
    pub req_id: Option<i64>,
    #[builder(required)]
    pub pair: String,
    pub price: Option<Decimal>,
    #[serde(rename = "price2")]
    pub price_2: Option<Decimal>,
    #[builder(required)]
    pub volume: Decimal,
    #[serde(rename = "oflags")]
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, OrderFlag>>")]
    pub order_flags: Option<Vec<OrderFlag>>,
    #[serde(rename = "newuserref")]
    pub new_user_ref: Option<String>,
    pub validate: Option<String>,
}

/// Response from editing an order
#[derive(Debug, Deserialize, PartialEq)]
pub struct EditOrderResponse {
    pub event: String,
    #[serde(rename = "txid")]
    pub tx_id: Option<String>,
    #[serde(rename = "originaltxid")]
    pub original_tx_id: Option<String>,
    #[serde(rename = "reqid")]
    pub req_id: Option<i64>,
    pub status: OrderRequestStatus,
    pub descr: Option<String>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}

/// Request to cancel an order
#[skip_serializing_none]
#[derive(Debug, Serialize, PartialEq, Builder)]
pub struct CancelOrderRequest {
    #[builder(required)]
    pub event: String,
    #[builder(required)]
    pub token: String,
    #[serde(rename = "txid")]
    pub tx_id: Option<Vec<String>>,
    #[serde(rename = "reqid")]
    pub req_id: Option<i64>,
}

/// Response from cancelling an order
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CancelOrderResponse {
    pub event: String,
    #[serde(rename = "reqid")]
    pub req_id: Option<i64>,
    pub status: OrderRequestStatus,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}

/// Request to cancel all existing orders
#[skip_serializing_none]
#[derive(Debug, Serialize, PartialEq)]
pub struct CancelAllRequest {
    pub event: String,
    pub token: String,
    #[serde(rename = "reqid")]
    pub req_id: Option<i64>,
}

/// Response from requesting to cancel all orders
#[derive(Debug, Deserialize, PartialEq)]
pub struct CancelAllResponse {
    pub event: String,
    #[serde(rename = "reqid")]
    pub req_id: Option<i64>,
    pub count: i64,
    pub status: OrderRequestStatus,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}

/// Request to cancel all orders after a timeout if not re-requested
#[skip_serializing_none]
#[derive(Debug, Serialize, PartialEq)]
pub struct CancelAllAfterRequest {
    pub event: String,
    pub token: String,
    #[serde(rename = "reqid")]
    pub req_id: Option<i64>,
    pub timeout: i64,
}

/// Response for cancelling all orders after a timeout
#[derive(Debug, Deserialize, PartialEq)]
pub struct CancelAllAfterResponse {
    pub event: String,
    #[serde(rename = "reqid")]
    pub req_id: Option<i64>,
    pub status: OrderRequestStatus,
    #[serde(rename = "currentTime")]
    pub current_time: Option<String>,
    #[serde(rename = "triggerTime")]
    pub trigger_time: Option<String>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}
