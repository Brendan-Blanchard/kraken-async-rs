use crate::wss_v2::shared::WssTestState;
use futures_util::StreamExt;
use kraken_async_rs::wss::v2::base_messages::{Message, MethodMessage, ResultResponse, WssMessage};
use kraken_async_rs::wss::v2::market_data_messages::{
    EventTrigger, TickerSubscription, TickerSubscriptionResponse,
};
use kraken_async_rs::wss::v2::user_data_messages::SubscriptionResult;
use serde_json::{json, Value};
use std::time::Duration;
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;
use ws_mock::matchers::JsonExact;
use ws_mock::ws_mock_server::WsMock;

fn get_expected_ticker_subscription() -> Value {
    json!({"method":"subscribe","params":{"channel":"ticker","symbol":["BTC/USD"]},"req_id":42})
}

fn get_ticker_subscription_response() -> String {
    r#"{"method":"subscribe","req_id":42,"result":{"channel":"ticker","event_trigger":"trades","snapshot":true,"symbol":"BTC/USD"},"success":true,"time_in":"2024-05-15T11:20:43.013486Z","time_out":"2024-05-15T11:20:43.013545Z"}"#.to_string()
}

fn get_expected_ticker_message() -> WssMessage {
    WssMessage::Method(MethodMessage::Subscription(ResultResponse {
        result: Some(SubscriptionResult::Ticker(TickerSubscriptionResponse {
            symbol: "BTC/USD".to_string(),
            event_trigger: Some(EventTrigger::Trades),
            snapshot: Some(true),
        })),
        error: None,
        success: true,
        req_id: 42,
        time_in: "2024-05-15T11:20:43.013486Z".to_string(),
        time_out: "2024-05-15T11:20:43.013545Z".to_string(),
    }))
}

#[tokio::test]
async fn test_ticker_subscription() {
    let mut test_state = WssTestState::new().await;

    WsMock::new()
        .matcher(JsonExact::new(get_expected_ticker_subscription()))
        .expect(1)
        .respond_with(TungsteniteMessage::Text(get_ticker_subscription_response()))
        .mount(&test_state.mock_server)
        .await;

    let mut stream = test_state.ws_client.connect::<WssMessage>().await.unwrap();

    let ticker_params = TickerSubscription {
        channel: "ticker".to_string(),
        symbol: vec!["BTC/USD".into()],
        event_trigger: None,
        snapshot: None,
    };

    let subscription = Message {
        method: "subscribe".to_string(),
        params: ticker_params,
        req_id: 42,
    };

    stream.send(&subscription).await.unwrap();

    let subscription = timeout(Duration::from_secs(3), stream.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();

    println!("{:?}", subscription);
    assert_eq!(get_expected_ticker_message(), subscription);
    test_state.mock_server.verify().await;
}
