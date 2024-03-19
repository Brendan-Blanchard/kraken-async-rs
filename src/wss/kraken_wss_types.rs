//! Common types returned in websocket messages
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// A Ping or Pong from Kraken
///
/// Importantly, this denies unknown fields to prevent it from parsing many other similar messages
/// that have extra fields.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PingPong {
    pub event: String,
    #[serde(rename = "reqid")]
    pub req_id: i64,
}

/// A heartbeat from the server
///
/// Importantly, this denies unknown fields to prevent it from parsing many other similar messages
/// that have extra fields.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Heartbeat {
    pub event: String,
}

/// Status of the exchange, given during subscriptions
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Online,
    Maintenance,
    CancelOnly,
    PostOnly,
    LimitOnly,
}

/// A status message sent by the server
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct SystemStatus {
    #[serde(rename = "connectionID")]
    pub connection_id: u64,
    pub event: String,
    pub status: Status,
    pub version: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Sequence {
    sequence: i64,
}

/// Error message type
///
/// Unparsed string messages directly from the server.
///
/// The `meta` field can capture data not expected by this model to aid in debugging or suggesting
/// improvements to the error handling model.
///
/// Examples:
/// `{"errorMessage":"EGeneral:Invalid arguments:volume","event":"addOrderStatus","status":"error"}`
///
/// `{"errorMessage":"Reqid field must be a positive integer less than 18446744073709551616","event":"addOrderStatus","pair":"USDC/USD","reqid":null,"status":"error"}`
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ErrorMessage {
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    #[serde(flatten)] // captures any additional error data as JSON
    pub meta: HashMap<String, Value>,
}
