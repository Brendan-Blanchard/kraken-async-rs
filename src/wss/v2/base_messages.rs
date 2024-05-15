use crate::wss::v2::admin_messages::{Heartbeat, StatusUpdate};
use crate::wss::v2::market_data_messages::{Orderbook, OrderbookUpdate, Ticker, Trade};
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

#[derive(Debug, Deserialize)]
pub struct Response<T>
where
    T: Debug,
{
    pub channel: String,
    #[serde(rename = "type")]
    pub message_type: String,
    pub data: T,
}
