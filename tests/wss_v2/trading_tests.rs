use crate::wss_v2::shared::CallResponseTest;
use kraken_async_rs::wss::v2::base_messages::MethodMessage::AddOrder;
use kraken_async_rs::wss::v2::base_messages::{Message, ResultResponse, WssMessage};
use kraken_async_rs::wss::v2::trading_messages::AddOrderResult;
use serde_json::json;
use std::str::FromStr;

mod add_order {
    use super::*;
    use kraken_async_rs::request_types::TimeInForceV2;
    use kraken_async_rs::response_types::{BuySell, OrderType};
    use kraken_async_rs::wss::v2::base_messages::PongResponse;
    use kraken_async_rs::wss::v2::market_data_messages::OrderbookEvent::Add;
    use kraken_async_rs::wss::v2::trading_messages::{AddOrderParams, FeePreference};
    use rust_decimal_macros::dec;

    #[tokio::test]
    async fn test_add_order() {
        let expected_request = json!({"method":"add_order","params":{"order_type":"limit","side":"buy","symbol":"USDC/USD","limit_price":0.95,"time_in_force":"ioc","order_qty":5.0,"post_only":false,"fee_preference":"quote","token":"aToken"},"req_id":0});
        let response = r#"{"method":"add_order","req_id":0,"result":{"order_id":"OPS23M-VS41G-DDE5Z2"},"success":true,"time_in":"2024-05-18T12:05:50.293682Z","time_out":"2024-05-18T12:05:50.300542Z"}"#.to_string();
        let expected_response = WssMessage::Method(AddOrder(ResultResponse {
            result: Some(AddOrderResult {
                order_id: "OPS23M-VS41G-DDE5Z2".to_string(),
                order_user_ref: None,
                warning: None,
            }),
            error: None,
            success: true,
            req_id: 0,
            time_in: "2024-05-18T12:05:50.293682Z".to_string(),
            time_out: "2024-05-18T12:05:50.300542Z".to_string(),
        }));

        let add_order: AddOrderParams = AddOrderParams {
            order_type: OrderType::Limit,
            side: BuySell::Buy,
            symbol: "USDC/USD".to_string(),
            limit_price: Some(dec!(0.95)),
            limit_price_type: None,
            triggers: None,
            time_in_force: Some(TimeInForceV2::IOC),
            order_quantity: dec!(5.0),
            margin: None,
            post_only: Some(false),
            reduce_only: None,
            effective_time: None,
            expire_time: None,
            deadline: None,
            order_user_ref: None,
            conditional: None,
            display_quantity: None,
            fee_preference: Some(FeePreference::Quote),
            no_market_price_protection: None,
            stp_type: None,
            cash_order_quantity: None,
            validate: None,
            token: "aToken".to_string(),
        };

        let message = Message {
            method: "add_order".to_string(),
            params: add_order,
            req_id: 0,
        };

        CallResponseTest::builder()
            .match_on(expected_request)
            .respond_with(response)
            .send(message)
            .expect(expected_response)
            .build()
            .test()
            .await;
    }
}
