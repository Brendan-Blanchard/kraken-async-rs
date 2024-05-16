use crate::wss::v2::admin_messages::{Heartbeat, StatusUpdate};
use crate::wss::v2::market_data_messages::{
    L3Orderbook, L3OrderbookUpdate, Orderbook, OrderbookUpdate, Ticker, Trade,
};
use crate::wss::v2::user_data_messages::SubscriptionResult;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PublicMessage {
    Status(Response<Vec<StatusUpdate>>),
    Trade(Response<Vec<Trade>>),
    Ticker(Response<Vec<Ticker>>),
    BookSnapshot(Response<Vec<Orderbook>>),
    BookUpdate(Response<Vec<OrderbookUpdate>>),
    Subscription(ResultResponse<SubscriptionResult>),
    Heartbeat(Heartbeat),
    Error(String),
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PrivateMessage {
    Status(Response<Vec<StatusUpdate>>),
    L3Snapshot(Response<Vec<L3Orderbook>>),
    L3Update(Response<Vec<L3OrderbookUpdate>>),
    Subscription(ResultResponse<SubscriptionResult>),
    Heartbeat(Heartbeat),
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

#[derive(Debug, Deserialize, PartialEq)]
pub struct Response<T> {
    pub channel: String,
    #[serde(rename = "type")]
    pub message_type: String,
    pub data: T,
}

#[derive(Debug, Deserialize, PartialEq)]
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
