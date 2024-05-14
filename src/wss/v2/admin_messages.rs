use crate::response_types::SystemStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct StatusUpdate {
    pub system: SystemStatus,
    pub api_version: String,
    pub connection_id: i64,
    pub version: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Heartbeat {
    pub channel: String,
}

#[derive(Debug, Serialize)]
pub struct Ping {
    pub method: String,
    pub req_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pong {
    pub warning: Vec<String>,
}
