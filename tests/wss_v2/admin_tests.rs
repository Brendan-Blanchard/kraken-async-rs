use crate::wss_v2::shared::{CallResponseTest, ParseIncomingTest};
use kraken_async_rs::response_types::SystemStatus;
use kraken_async_rs::wss::ChannelMessage::{Heartbeat, Status};
use kraken_async_rs::wss::{SingleResponse, StatusUpdate, WssMessage};
use serde_json::{json, Number, Value};
use std::str::FromStr;

#[tokio::test]
async fn test_admin_messages() {
    let heartbeat = r#"{"channel":"heartbeat"}"#.to_string();
    let status_update = r#"{"channel":"status","data":[{"api_version":"v2","connection_id":12393906104898154338,"system":"online","version":"2.0.4"}],"type":"update"}"#.to_string();

    let status_message = WssMessage::Channel(Status(SingleResponse {
        data: StatusUpdate {
            api_version: "v2".to_string(),
            connection_id: Some(Number::from_str("12393906104898154338").unwrap()),
            system: SystemStatus::Online,
            version: "2.0.4".to_string(),
        },
    }));

    ParseIncomingTest::new()
        .with_incoming(heartbeat)
        .expect_message(WssMessage::Channel(Heartbeat))
        .with_incoming(status_update)
        .expect_message(status_message)
        .test()
        .await;
}

mod ping_pong {
    use super::*;
    use kraken_async_rs::wss::{Message, MethodMessage, PongResponse};

    fn get_expected_ping() -> Value {
        json!({"method":"ping","req_id":1})
    }

    fn get_pong() -> String {
        r#"{"method":"pong","req_id":1,"time_in":"2024-05-20T11:08:49.272922Z","time_out":"2024-05-20T11:08:49.272940Z"}"#.to_string()
    }

    fn get_expected_pong_message() -> WssMessage {
        WssMessage::Method(MethodMessage::Pong(PongResponse {
            error: None,
            req_id: 1,
            time_in: "2024-05-20T11:08:49.272922Z".to_string(),
            time_out: "2024-05-20T11:08:49.272940Z".to_string(),
        }))
    }

    #[tokio::test]
    async fn test_ping_pong() {
        let ping: Option<()> = None;

        let message = Message {
            method: "ping".to_string(),
            params: ping,
            req_id: 1,
        };

        CallResponseTest::builder()
            .match_on(get_expected_ping())
            .respond_with(get_pong())
            .send(message)
            .expect(get_expected_pong_message())
            .build()
            .test()
            .await;
    }
}
