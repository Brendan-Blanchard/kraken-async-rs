use crate::wss::v2::admin_messages::{Heartbeat, StatusUpdate};
use crate::wss::v2::market_data_messages::{
    Instruments, L3Orderbook, L3OrderbookUpdate, Ohlc, Orderbook, OrderbookUpdate, Ticker, Trade,
};
use crate::wss::v2::trading_messages::{
    AddOrderResult, BatchCancelResponse, BatchCancelResult, CancelAllOrdersResult,
    CancelOnDisconnectResult, CancelOrderResult, EditOrderResult,
};
use crate::wss::v2::user_data_messages::{Balance, ExecutionResult, SubscriptionResult};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PublicMessage {
    Status(Response<Vec<StatusUpdate>>),
    Trade(Response<Vec<Trade>>),
    Ticker(Response<Vec<Ticker>>),
    BookSnapshot(Response<Vec<Orderbook>>),
    BookUpdate(Response<Vec<OrderbookUpdate>>),
    Ohlc(Response<Vec<Ohlc>>),
    Instrument(Response<Instruments>),
    Subscription(ResultResponse<SubscriptionResult>),
    Heartbeat(Heartbeat),
    Pong(ResultResponse<Pong>),
    Error(String),
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PrivateMessage {
    Status(Response<Vec<StatusUpdate>>),
    Execution(Response<Vec<ExecutionResult>>),
    Balance(Response<Vec<Balance>>),
    AddOrder(ResultResponse<AddOrderResult>),
    EditOrder(ResultResponse<EditOrderResult>),
    CancelOrder(ResultResponse<CancelOrderResult>),
    CancelAllOrders(ResultResponse<CancelAllOrdersResult>),
    CancelOnDisconnect(ResultResponse<CancelOnDisconnectResult>),
    BatchOrder(ResultResponse<Vec<AddOrderResult>>),
    BatchCancel(BatchCancelResponse),
    L3Snapshot(Response<Vec<L3Orderbook>>),
    L3Update(Response<Vec<L3OrderbookUpdate>>),
    Subscription(ResultResponse<SubscriptionResult>),
    Heartbeat(Heartbeat),
    Pong(ResultResponse<Pong>),
    Error(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message<T>
where
    T: Debug,
{
    pub method: String,
    pub params: T,
    pub req_id: i64,
}

#[derive(Debug, Serialize)]
pub struct Ping {
    pub method: String,
    pub req_id: i64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Pong {
    pub warning: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Response<T> {
    pub channel: String,
    #[serde(rename = "type")]
    pub message_type: String,
    pub data: T,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ResultResponse<T> {
    pub method: String,
    pub result: Option<T>,
    pub error: Option<String>,
    pub success: bool,
    pub req_id: i64,
    pub time_in: String,
    pub time_out: String,
}

#[cfg(test)]
mod tests {
    use crate::response_types::SystemStatus;
    use crate::wss::v2::admin_messages::StatusUpdate;
    use crate::wss::v2::base_messages::{PrivateMessage, Response};
    use serde_json::Number;
    use std::str::FromStr;

    #[test]
    fn test_deserializing_private_status_update() {
        let message = r#"{"channel":"status","data":[{"api_version":"v2","connection_id":18266300427528990701,"system":"online","version":"2.0.4"}],"type":"update"}"#;
        let expected = PrivateMessage::Status(Response {
            channel: "status".to_string(),
            message_type: "update".to_string(),
            data: vec![StatusUpdate {
                api_version: "v2".to_string(),
                connection_id: Number::from_str("18266300427528990701").unwrap(),
                system: SystemStatus::Online,
                version: "2.0.4".to_string(),
            }],
        });

        let parsed = serde_json::from_str::<PrivateMessage>(message).unwrap();

        assert_eq!(expected, parsed);
    }
}
