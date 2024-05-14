use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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
