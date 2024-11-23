use crate::wss::{MethodMessage, PongResponse, WssMessage};
use serde_json::{json, Value};

pub fn get_expected_ping() -> Value {
    json!({"method":"ping","req_id":1})
}

pub fn get_pong() -> String {
    r#"{"method":"pong","req_id":1,"time_in":"2024-05-20T11:08:49.272922Z","time_out":"2024-05-20T11:08:49.272940Z"}"#.to_string()
}

pub fn get_expected_pong_message() -> WssMessage {
    WssMessage::Method(MethodMessage::Pong(PongResponse {
        error: None,
        req_id: 1,
        time_in: "2024-05-20T11:08:49.272922Z".to_string(),
        time_out: "2024-05-20T11:08:49.272940Z".to_string(),
    }))
}
