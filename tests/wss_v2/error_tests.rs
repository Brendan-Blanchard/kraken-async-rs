use crate::wss_v2::shared::{parse_for_test, ParseIncomingTest};
use kraken_async_rs::wss::errors::WSSError;
use kraken_async_rs::wss::{MethodMessage, ResultResponse, WssMessage};

#[tokio::test]
async fn test_cloudflare_error() {
    // a bare string isn't valid JSON, so this fails to parse rather than producing an error
    let cloudflare_restarting = r#"CloudFlare WebSocket proxy restarting"#;

    let result = parse_for_test(cloudflare_restarting).await;

    assert!(matches!(result, Err(WSSError::Serde(..))));
}

#[tokio::test]
async fn test_error_messages() {
    let unsupported_field = r#"{"error":"Unsupported field: 'params' for the given msg type: ping","method":"ping","req_id":0,"success":false,"time_in":"2024-05-19T19:58:40.170724Z","time_out":"2024-05-19T19:58:40.170758Z"}"#.to_string();

    let expected_unsupported_field = WssMessage::Method(MethodMessage::Ping(ResultResponse {
        result: None,
        error: Some("Unsupported field: 'params' for the given msg type: ping".to_string()),
        success: false,
        req_id: 0,
        time_in: "2024-05-19T19:58:40.170724Z".to_string(),
        time_out: "2024-05-19T19:58:40.170758Z".to_string(),
    }));

    let unsupported_event = r#"{"error":"Unsupported event","method":"subscribe","req_id":0,"success":false,"time_in":"2024-05-19T20:02:10.316562Z","time_out":"2024-05-19T20:02:10.316592Z"}"#.to_string();

    let expected_unsupported_event =
        WssMessage::Method(MethodMessage::Subscription(ResultResponse {
            result: None,
            error: Some("Unsupported event".to_string()),
            success: false,
            req_id: 0,
            time_in: "2024-05-19T20:02:10.316562Z".to_string(),
            time_out: "2024-05-19T20:02:10.316592Z".to_string(),
        }));

    let invalid_arguments = r#"{"error":"EGeneral:Invalid arguments:no_mpp order option is only available when ordertype = market","method":"add_order","req_id":0,"success":false,"time_in":"2024-05-18T12:03:08.768086Z","time_out":"2024-05-18T12:03:08.768149Z"}"#.to_string();

    let expected_invalid_arguments =
        WssMessage::Method(MethodMessage::AddOrder(ResultResponse {
            result: None,
            error: Some("EGeneral:Invalid arguments:no_mpp order option is only available when ordertype = market".to_string()),
            success: false,
            req_id: 0,
            time_in: "2024-05-18T12:03:08.768086Z".to_string(),
            time_out: "2024-05-18T12:03:08.768149Z".to_string(),
        }));

    let add_order_failure = r#"{"error":"Cash_order_qty field must be a number_float","method":"add_order","req_id":7,"success":false,"time_in":"2024-05-18T12:00:03.886027Z","time_out":"2024-05-18T12:00:03.886141Z"}"#.to_string();

    let expected_add_order_failure = WssMessage::Method(MethodMessage::AddOrder(ResultResponse {
        result: None,
        error: Some("Cash_order_qty field must be a number_float".to_string()),
        success: false,
        req_id: 7,
        time_in: "2024-05-18T12:00:03.886027Z".to_string(),
        time_out: "2024-05-18T12:00:03.886141Z".to_string(),
    }));

    let permission_denied = r#"{"error":"EGeneral:Permission denied","method":"add_order","req_id":0,"success":false,"time_in":"2024-05-18T12:03:43.466650Z","time_out":"2024-05-18T12:03:43.471987Z"}"#.to_string();

    let expected_permission_denied = WssMessage::Method(MethodMessage::AddOrder(ResultResponse {
        result: None,
        error: Some("EGeneral:Permission denied".to_string()),
        success: false,
        req_id: 0,
        time_in: "2024-05-18T12:03:43.466650Z".to_string(),
        time_out: "2024-05-18T12:03:43.471987Z".to_string(),
    }));

    let no_token = r#"{"error":"Token(s) not found","method":"edit_order","req_id":0,"success":false,"time_in":"2024-05-18T13:04:41.754066Z","time_out":"2024-05-18T13:04:41.754113Z"}"#.to_string();

    let expected_no_token = WssMessage::Method(MethodMessage::EditOrder(ResultResponse {
        result: None,
        error: Some("Token(s) not found".to_string()),
        success: false,
        req_id: 0,
        time_in: "2024-05-18T13:04:41.754066Z".to_string(),
        time_out: "2024-05-18T13:04:41.754113Z".to_string(),
    }));

    ParseIncomingTest::new()
        .with_incoming(unsupported_field)
        .expect_message(expected_unsupported_field)
        .with_incoming(unsupported_event)
        .expect_message(expected_unsupported_event)
        .with_incoming(invalid_arguments)
        .expect_message(expected_invalid_arguments)
        .with_incoming(add_order_failure)
        .expect_message(expected_add_order_failure)
        .with_incoming(permission_denied)
        .expect_message(expected_permission_denied)
        .with_incoming(no_token)
        .expect_message(expected_no_token)
        .test()
        .await;
}
