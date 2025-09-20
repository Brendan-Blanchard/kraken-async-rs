use crate::wss::{
    BalanceSubscriptionResult, BookSubscriptionResponse, EventTrigger,
    ExecutionsSubscriptionResult, InstrumentSubscriptionResult, MethodMessage,
    OhlcSubscriptionResponse, PongResponse, ResultResponse, SubscriptionResult,
    TickerSubscriptionResponse, TradeSubscriptionResponse, WssMessage,
};
use serde_json::{Value, json};

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

pub fn get_expected_execution_subscription() -> Value {
    json!({"method":"subscribe","params":{"channel":"executions","token":"someToken","snap_orders":true,"snap_trades":true},"req_id":0})
}

pub fn get_execution_subscription_response() -> String {
    r#"{"method":"subscribe","req_id":0,"result":{"channel":"executions","maxratecount":180,"snapshot":true,"warnings":["cancel_reason is deprecated, use reason","stop_price is deprecated, use triggers.price","trigger is deprecated use triggers.reference","triggered_price is deprecated use triggers.last_price"]},"success":true,"time_in":"2024-05-19T19:30:36.343170Z","time_out":"2024-05-19T19:30:36.350083Z"}"#.to_string()
}

pub fn get_expected_execution_message() -> WssMessage {
    WssMessage::Method(MethodMessage::Subscription(ResultResponse {
        result: Some(SubscriptionResult::Execution(
            ExecutionsSubscriptionResult {
                max_rate_count: Some(180),
                snapshot: Some(true),
                warnings: Some(vec![
                    "cancel_reason is deprecated, use reason".into(),
                    "stop_price is deprecated, use triggers.price".into(),
                    "trigger is deprecated use triggers.reference".into(),
                    "triggered_price is deprecated use triggers.last_price".into(),
                ]),
            },
        )),
        error: None,
        success: true,
        req_id: 0,
        time_in: "2024-05-19T19:30:36.343170Z".to_string(),
        time_out: "2024-05-19T19:30:36.350083Z".to_string(),
    }))
}

pub fn get_expected_balances_subscription() -> Value {
    json!({"method":"subscribe","params":{"channel":"balances","token":"anotherToken","snapshot":true},"req_id":10312008})
}

pub fn get_balances_subscription_response() -> String {
    r#"{"method":"subscribe","req_id":10312008,"result":{"channel":"balances","snapshot":true},"success":true,"time_in":"2024-05-19T16:25:28.289124Z","time_out":"2024-05-19T16:25:28.293750Z"}"#.to_string()
}

pub fn get_expected_balances_message() -> WssMessage {
    WssMessage::Method(MethodMessage::Subscription(ResultResponse {
        result: Some(SubscriptionResult::Balance(BalanceSubscriptionResult {
            snapshot: Some(true),
            warnings: None,
        })),
        error: None,
        success: true,
        req_id: 10312008,
        time_in: "2024-05-19T16:25:28.289124Z".to_string(),
        time_out: "2024-05-19T16:25:28.293750Z".to_string(),
    }))
}

pub fn get_expected_ticker_subscription() -> Value {
    json!({"method":"subscribe","params":{"channel":"ticker","symbol":["BTC/USD"]},"req_id":42})
}

pub fn get_ticker_subscription_response() -> String {
    r#"{"method":"subscribe","req_id":42,"result":{"channel":"ticker","event_trigger":"trades","snapshot":true,"symbol":"BTC/USD"},"success":true,"time_in":"2024-05-15T11:20:43.013486Z","time_out":"2024-05-15T11:20:43.013545Z"}"#.to_string()
}

pub fn get_expected_ticker_message() -> WssMessage {
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

pub fn get_expected_book_subscription() -> Value {
    json!({"method":"subscribe","params":{"channel":"book","symbol":["BTC/USD"],"depth":10,"snapshot":true},"req_id":11})
}

pub fn get_book_subscription_response() -> String {
    r#"{"method":"subscribe","req_id":11,"result":{"channel":"book","depth":10,"snapshot":true,"symbol":"BTC/USD"},"success":true,"time_in":"2024-05-19T16:27:13.694962Z","time_out":"2024-05-19T16:27:13.695006Z"}"#.to_string()
}

pub fn get_expected_book_message() -> WssMessage {
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

pub fn get_expected_l3_subscription() -> Value {
    json!({"method":"subscribe","params":{"channel":"level3","symbol":["BTC/USD"],"snapshot":true,"token":"someToken"},"req_id":99})
}

pub fn get_l3_subscription_response() -> String {
    r#"{"method":"subscribe","req_id":99,"result":{"channel":"level3","snapshot":true,"symbol":"BTC/USD"},"success":true,"time_in":"2024-05-19T18:51:30.701627Z","time_out":"2024-05-19T18:51:30.708403Z"}"#.to_string()
}

pub fn get_expected_l3_message() -> WssMessage {
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

pub fn get_expected_ohlc_subscription() -> Value {
    json!({"method":"subscribe","params":{"channel":"ohlc","symbol":["ETH/USD"],"interval":60},"req_id":121})
}

pub fn get_ohlc_subscription_response() -> String {
    r#"{"method":"subscribe","req_id":121,"result":{"channel":"ohlc","interval":60,"snapshot":true,"symbol":"ETH/USD","warnings":["timestamp is deprecated, use interval_begin"]},"success":true,"time_in":"2024-05-19T19:06:57.002983Z","time_out":"2024-05-19T19:06:57.003037Z"}"#.to_string()
}

pub fn get_expected_ohlc_message() -> WssMessage {
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

pub fn get_expected_trade_subscription() -> Value {
    json!({"method":"subscribe","params":{"channel":"trade","symbol":["BTC/USD"]},"req_id":0})
}

pub fn get_trade_subscription_response() -> String {
    r#"{"method":"subscribe","req_id":0,"result":{"channel":"trade","snapshot":true,"symbol":"BTC/USD"},"success":true,"time_in":"2024-05-19T19:11:23.034030Z","time_out":"2024-05-19T19:11:23.034073Z"}"#.to_string()
}

pub fn get_expected_trade_message() -> WssMessage {
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

pub fn get_expected_instruments_subscription() -> Value {
    json!({"method":"subscribe","params":{"channel":"instrument","snapshot":true},"req_id":0})
}

pub fn get_instruments_subscription_response() -> String {
    r#"{"method":"subscribe","req_id":0,"result":{"channel":"instrument","snapshot":true,"warnings":["tick_size is deprecated, use price_increment"]},"success":true,"time_in":"2024-05-19T19:44:43.264430Z","time_out":"2024-05-19T19:44:43.264464Z"}"#.to_string()
}

pub fn get_expected_instruments_message() -> WssMessage {
    WssMessage::Method(MethodMessage::Subscription(ResultResponse {
        result: Some(SubscriptionResult::Instrument(
            InstrumentSubscriptionResult {
                snapshot: Some(true),
                warnings: Some(vec!["tick_size is deprecated, use price_increment".into()]),
            },
        )),
        error: None,
        success: true,
        req_id: 0,
        time_in: "2024-05-19T19:44:43.264430Z".to_string(),
        time_out: "2024-05-19T19:44:43.264464Z".to_string(),
    }))
}
