use crate::wss_v2::shared::WssTestState;
use futures_util::StreamExt;
use kraken_async_rs::wss::v2::base_messages::{Message, MethodMessage, ResultResponse, WssMessage};
use kraken_async_rs::wss::v2::market_data_messages::{
    EventTrigger, TickerSubscription, TickerSubscriptionResponse,
};
use kraken_async_rs::wss::v2::user_data_messages::SubscriptionResult;
use serde::Serialize;
use serde_json::{json, Value};
use simple_builder::Builder;
use std::fmt::Debug;
use std::time::Duration;
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;
use ws_mock::matchers::JsonExact;
use ws_mock::ws_mock_server::WsMock;

mod ticker_subscription {
    use super::*;

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
        let ticker_params = TickerSubscription::new(vec!["BTC/USD".into()]);

        let subscription = Message {
            method: "subscribe".to_string(),
            params: ticker_params,
            req_id: 42,
        };

        CallResponseTest::builder()
            .match_on(get_expected_ticker_subscription())
            .respond_with(get_ticker_subscription_response())
            .send(subscription)
            .expect(get_expected_ticker_message())
            .build()
            .test()
            .await;
    }
}

mod book_subscription {
    use super::*;
    use kraken_async_rs::wss::v2::market_data_messages::{
        BookSubscription, BookSubscriptionResponse,
    };

    fn get_expected_book_subscription() -> Value {
        json!({"method":"subscribe","params":{"channel":"book","symbol":["BTC/USD"],"depth":10,"snapshot":true},"req_id":11})
    }

    fn get_book_subscription_response() -> String {
        r#"{"method":"subscribe","req_id":11,"result":{"channel":"book","depth":10,"snapshot":true,"symbol":"BTC/USD"},"success":true,"time_in":"2024-05-19T16:27:13.694962Z","time_out":"2024-05-19T16:27:13.695006Z"}"#.to_string()
    }

    fn get_expected_book_message() -> WssMessage {
        WssMessage::Method(MethodMessage::Subscription(ResultResponse {
            result: Some(SubscriptionResult::Book(BookSubscriptionResponse {
                symbol: "BTC/USD".to_string(),
                snapshot: Some(true),
                depth: Some(10),
                warnings: None,
            })),
            error: None,
            success: true,
            req_id: 11,
            time_in: "2024-05-19T16:27:13.694962Z".to_string(),
            time_out: "2024-05-19T16:27:13.695006Z".to_string(),
        }))
    }

    #[tokio::test]
    async fn test_book_subscription() {
        let mut book_params = BookSubscription::new(vec!["BTC/USD".into()]);
        book_params.depth = Some(10);
        book_params.snapshot = Some(true);

        let subscription = Message {
            method: "subscribe".to_string(),
            params: book_params,
            req_id: 11,
        };

        CallResponseTest::builder()
            .match_on(get_expected_book_subscription())
            .respond_with(get_book_subscription_response())
            .send(subscription)
            .expect(get_expected_book_message())
            .build()
            .test()
            .await;
    }
}

mod l3_subscription {
    use super::*;
    use kraken_async_rs::wss::v2::market_data_messages::{
        BookSubscription, BookSubscriptionResponse,
    };

    fn get_expected_l3_subscription() -> Value {
        json!({"method":"subscribe","params":{"channel":"level3","symbol":["BTC/USD"],"snapshot":true,"token":"someToken"},"req_id":99})
    }

    fn get_l3_subscription_response() -> String {
        r#"{"method":"subscribe","req_id":99,"result":{"channel":"level3","snapshot":true,"symbol":"BTC/USD"},"success":true,"time_in":"2024-05-19T18:51:30.701627Z","time_out":"2024-05-19T18:51:30.708403Z"}"#.to_string()
    }

    fn get_expected_l3_message() -> WssMessage {
        WssMessage::Method(MethodMessage::Subscription(ResultResponse {
            result: Some(SubscriptionResult::L3(BookSubscriptionResponse {
                symbol: "BTC/USD".to_string(),
                snapshot: Some(true),
                depth: None,
                warnings: None,
            })),
            error: None,
            success: true,
            req_id: 99,
            time_in: "2024-05-19T18:51:30.701627Z".to_string(),
            time_out: "2024-05-19T18:51:30.708403Z".to_string(),
        }))
    }

    #[tokio::test]
    async fn test_l3_subscription() {
        let mut book_params =
            BookSubscription::new_l3(vec!["BTC/USD".into()], "someToken".to_string());
        book_params.snapshot = Some(true);

        let subscription = Message {
            method: "subscribe".to_string(),
            params: book_params,
            req_id: 99,
        };

        CallResponseTest::builder()
            .match_on(get_expected_l3_subscription())
            .respond_with(get_l3_subscription_response())
            .send(subscription)
            .expect(get_expected_l3_message())
            .build()
            .test()
            .await;
    }
}

mod ohlc_subscription {
    use super::*;
    use kraken_async_rs::wss::v2::market_data_messages::{
        BookSubscription, BookSubscriptionResponse, OhlcSubscription, OhlcSubscriptionResponse,
    };

    fn get_expected_ohlc_subscription() -> Value {
        json!({"method":"subscribe","params":{"channel":"ohlc","symbol":["ETH/USD"],"interval":60},"req_id":121})
    }

    fn get_ohlc_subscription_response() -> String {
        r#"{"method":"subscribe","req_id":121,"result":{"channel":"ohlc","interval":60,"snapshot":true,"symbol":"ETH/USD","warnings":["timestamp is deprecated, use interval_begin"]},"success":true,"time_in":"2024-05-19T19:06:57.002983Z","time_out":"2024-05-19T19:06:57.003037Z"}"#.to_string()
    }

    fn get_expected_ohlc_message() -> WssMessage {
        WssMessage::Method(MethodMessage::Subscription(ResultResponse {
            result: Some(SubscriptionResult::Ohlc(OhlcSubscriptionResponse {
                symbol: Some("ETH/USD".to_string()),
                snapshot: Some(true),
                warnings: Some(vec!["timestamp is deprecated, use interval_begin".into()]),
                interval: 60,
            })),
            error: None,
            success: true,
            req_id: 121,
            time_in: "2024-05-19T19:06:57.002983Z".to_string(),
            time_out: "2024-05-19T19:06:57.003037Z".to_string(),
        }))
    }

    #[tokio::test]
    async fn test_ohlc_subscription() {
        let mut ohlc_params = OhlcSubscription::new(vec!["ETH/USD".into()], 60);

        let subscription = Message {
            method: "subscribe".to_string(),
            params: ohlc_params,
            req_id: 121,
        };

        CallResponseTest::builder()
            .match_on(get_expected_ohlc_subscription())
            .respond_with(get_ohlc_subscription_response())
            .send(subscription)
            .expect(get_expected_ohlc_message())
            .build()
            .test()
            .await;
    }
}

mod trade_subscription {
    use super::*;
    use kraken_async_rs::wss::v2::market_data_messages::{
        BookSubscription, BookSubscriptionResponse, OhlcSubscription, OhlcSubscriptionResponse,
        TradeSubscriptionResponse, TradesSubscription,
    };

    fn get_expected_trade_subscription() -> Value {
        json!({"method":"subscribe","params":{"channel":"trade","symbol":["BTC/USD"]},"req_id":0})
    }

    fn get_trade_subscription_response() -> String {
        r#"{"method":"subscribe","req_id":0,"result":{"channel":"trade","snapshot":true,"symbol":"BTC/USD"},"success":true,"time_in":"2024-05-19T19:11:23.034030Z","time_out":"2024-05-19T19:11:23.034073Z"}"#.to_string()
    }

    fn get_expected_trade_message() -> WssMessage {
        WssMessage::Method(MethodMessage::Subscription(ResultResponse {
            result: Some(SubscriptionResult::Trade(TradeSubscriptionResponse {
                symbol: Some("BTC/USD".to_string()),
                snapshot: Some(true),
                warnings: None,
            })),
            error: None,
            success: true,
            req_id: 0,
            time_in: "2024-05-19T19:11:23.034030Z".to_string(),
            time_out: "2024-05-19T19:11:23.034073Z".to_string(),
        }))
    }

    #[tokio::test]
    async fn test_trade_subscription() {
        let mut trade_params = TradesSubscription::new(vec!["BTC/USD".into()]);

        let subscription = Message {
            method: "subscribe".to_string(),
            params: trade_params,
            req_id: 0,
        };

        CallResponseTest::builder()
            .match_on(get_expected_trade_subscription())
            .respond_with(get_trade_subscription_response())
            .send(subscription)
            .expect(get_expected_trade_message())
            .build()
            .test()
            .await;
    }
}

#[derive(Debug, Builder)]
struct CallResponseTest<T>
where
    T: Debug + Serialize,
{
    match_on: Option<Value>,
    respond_with: Option<String>,
    send: Option<Message<T>>,
    expect: Option<WssMessage>,
}

impl<T> CallResponseTest<T>
where
    T: Debug + Serialize,
{
    pub async fn test(&mut self) {
        assert!(self.match_on.is_some());
        assert!(self.respond_with.is_some());
        assert!(self.send.is_some());
        assert!(self.expect.is_some());

        let mut test_state = WssTestState::new().await;

        WsMock::new()
            .matcher(JsonExact::new(self.match_on.take().unwrap()))
            .expect(1)
            .respond_with(TungsteniteMessage::Text(self.respond_with.take().unwrap()))
            .mount(&test_state.mock_server)
            .await;

        let mut stream = test_state.ws_client.connect::<WssMessage>().await.unwrap();

        stream.send(&self.send.take().unwrap()).await.unwrap();

        let result = timeout(Duration::from_secs(3), stream.next()).await;

        test_state.mock_server.verify().await;

        let response = result.unwrap().unwrap().unwrap();

        println!("{:?}", response);
        assert_eq!(self.expect.take().unwrap(), response);
    }
}
