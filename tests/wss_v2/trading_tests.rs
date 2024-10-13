use crate::wss_v2::shared::{CallResponseTest, ParseIncomingTest};
use kraken_async_rs::crypto::secrets::Token;
use kraken_async_rs::request_types::TimeInForceV2;
use kraken_async_rs::response_types::{BuySell, OrderType};
use kraken_async_rs::wss::v2::base_messages::MethodMessage::{
    AddOrder, AmendOrder, CancelOrder, EditOrder,
};
use kraken_async_rs::wss::v2::base_messages::{Message, MethodMessage, ResultResponse, WssMessage};
use kraken_async_rs::wss::v2::trading_messages::{
    AddOrderParams, AddOrderResult, AmendOrderParams, AmendOrderResult, BatchCancelParams,
    BatchCancelResponse, BatchOrder, BatchOrderParams, CancelAllOrdersParams,
    CancelAllOrdersResult, CancelOnDisconnectParams, CancelOnDisconnectResult, CancelOrderParams,
    CancelOrderResult, EditOrderParams, EditOrderResult, FeePreference,
};
use rust_decimal_macros::dec;
use serde_json::json;

#[tokio::test]
async fn test_add_order() {
    let expected_request = json!({"method":"add_order","params":{"order_type":"limit","side":"buy","symbol":"USDC/USD","limit_price":0.95,"time_in_force":"ioc","order_qty":5.0,"post_only":false,"fee_preference":"quote","token":"aToken","cl_ord_id":"client-zero","sender_sub_id":"sender-one"},"req_id":0});
    let response = r#"{"method":"add_order","req_id":0,"result":{"order_id":"OPS23M-VS41G-DDE5Z2","cl_ord_id":"client-zero"},"success":true,"time_in":"2024-05-18T12:05:50.293682Z","time_out":"2024-05-18T12:05:50.300542Z"}"#.to_string();
    let expected_response = WssMessage::Method(AddOrder(ResultResponse {
        result: Some(AddOrderResult {
            order_id: "OPS23M-VS41G-DDE5Z2".to_string(),
            order_user_ref: None,
            warning: None,
            client_order_id: Some("client-zero".to_string()),
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
        sender_sub_id: Some("sender-one".to_string()),
        token: Token::new("aToken".to_string()),
        client_order_id: Some("client-zero".to_string()),
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

#[tokio::test]
async fn test_amend_order() {
    let expected_request = json!({"method":"amend_order","params":{"order_id":"BQS60L-EGW18-UPAK9U","order_qty":5.1,"limit_price":0.96,"post_only":false,"token":"aToken"},"req_id":0});
    let response = r#"{"method":"amend_order","req_id":0,"result":{"amend_id":"1M2JV8-OEJZD-G5GSBF","order_id":"BQS60L-EGW18-UPAK9U"},"success":true,"time_in":"2024-10-11T12:12:21.003873Z","time_out":"2024-10-11T12:12:21.005064Z"}"#.to_string();
    let expected_response = WssMessage::Method(AmendOrder(ResultResponse {
        result: Some(AmendOrderResult {
            amend_id: "1M2JV8-OEJZD-G5GSBF".to_string(),
            order_id: Some("BQS60L-EGW18-UPAK9U".to_string()),
            client_order_id: None,
            warnings: None,
        }),
        error: None,
        success: true,
        req_id: 0,
        time_in: "2024-10-11T12:12:21.003873Z".to_string(),
        time_out: "2024-10-11T12:12:21.005064Z".to_string(),
    }));

    let amend_order = AmendOrderParams {
        order_id: Some("BQS60L-EGW18-UPAK9U".to_string()),
        limit_price: Some(dec!(0.96)),
        limit_price_type: None,
        post_only: Some(false),
        trigger_price: None,
        trigger_price_type: None,
        deadline: None,
        token: Token::new("aToken".to_string()),
        client_order_id: None,
        order_quantity: dec!(5.1),
        display_quantity: None,
    };

    let message = Message {
        method: "amend_order".to_string(),
        params: amend_order,
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

#[tokio::test]
async fn test_amend_order_error_response() {
    let response = r#"{"error":"Limit_price field must be a number_float","method":"amend_order","req_id":0,"success":false,"time_in":"2024-10-13T13:31:28.636431Z","time_out":"2024-10-13T13:31:28.636488Z"}"#;

    let expected_message = WssMessage::Method(AmendOrder(ResultResponse {
        result: None,
        error: Some("Limit_price field must be a number_float".to_string()),
        success: false,
        req_id: 0,
        time_in: "2024-10-13T13:31:28.636431Z".to_string(),
        time_out: "2024-10-13T13:31:28.636488Z".to_string(),
    }));

    ParseIncomingTest::new()
        .with_incoming(response.to_string())
        .expect_message(expected_message)
        .test()
        .await;
}

#[tokio::test]
async fn test_edit_order() {
    let expected_request = json!({"method":"edit_order","params":{"limit_price":0.93,"order_id":"K1FF7H-A13AR-Q1S9Z6","order_qty":6.1,"symbol":"USDC/USD","token":"someToken"},"req_id":0});
    let response = r#"{"method":"edit_order","req_id":0,"result":{"order_id":"7FIK6B-S15X0-DPJJTH","original_order_id":"K1FF7H-A13AR-Q1S9Z6"},"success":true,"time_in":"2024-05-19T12:12:30.171615Z","time_out":"2024-05-19T12:12:30.173877Z"}"#.to_string();
    let expected_response = WssMessage::Method(EditOrder(ResultResponse {
        result: Some(EditOrderResult {
            order_id: "7FIK6B-S15X0-DPJJTH".to_string(),
            original_order_id: "K1FF7H-A13AR-Q1S9Z6".to_string(),
            warning: None,
        }),
        error: None,
        success: true,
        req_id: 0,
        time_in: "2024-05-19T12:12:30.171615Z".to_string(),
        time_out: "2024-05-19T12:12:30.173877Z".to_string(),
    }));

    let edit_order = EditOrderParams {
        symbol: "USDC/USD".to_string(),
        limit_price: Some(dec!(0.93)),
        triggers: None,
        order_quantity: Some(dec!(6.1)),
        post_only: None,
        reduce_only: None,
        deadline: None,
        order_user_ref: None,
        display_quantity: None,
        fee_preference: None,
        no_market_price_protection: None,
        validate: None,
        token: Token::new("someToken".to_string()),
        order_id: "K1FF7H-A13AR-Q1S9Z6".to_string(),
    };

    let message = Message {
        method: "edit_order".to_string(),
        params: edit_order,
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

#[tokio::test]
async fn test_cancel_order() {
    let expected_request = json!({"method":"cancel_order","params":{"order_id":["1V7PZA-L5RIM-RX2G6B"],"token":"thatToken"},"req_id":0});
    let response = r#"{"method":"cancel_order","req_id":0,"result":{"order_id":"1V7PZA-L5RIM-RX2G6B"},"success":true,"time_in":"2024-05-19T19:18:44.987402Z","time_out":"2024-05-19T19:18:44.989756Z"}"#.to_string();
    let expected_response = WssMessage::Method(CancelOrder(ResultResponse {
        result: Some(CancelOrderResult {
            order_id: Some("1V7PZA-L5RIM-RX2G6B".to_string()),
            warning: None,
            client_order_id: None,
        }),
        error: None,
        success: true,
        req_id: 0,
        time_in: "2024-05-19T19:18:44.987402Z".to_string(),
        time_out: "2024-05-19T19:18:44.989756Z".to_string(),
    }));

    let cancel_order = CancelOrderParams {
        order_id: Some(vec!["1V7PZA-L5RIM-RX2G6B".into()]),
        client_order_id: None,
        order_user_ref: None,
        token: Token::new("thatToken".to_string()),
    };

    let message = Message {
        method: "cancel_order".to_string(),
        params: cancel_order,
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

#[tokio::test]
async fn test_cancel_order_by_client_order_id() {
    let expected_request = json!({"method":"cancel_order","params":{"cl_ord_id":["a-uuid"],"token":"thatToken"},"req_id":0});
    let response = r#"{"method":"cancel_order","req_id":0,"result":{"cl_ord_id":"a-uuid"},"success":true,"time_in":"2024-05-19T19:18:44.987402Z","time_out":"2024-05-19T19:18:44.989756Z"}"#.to_string();
    let expected_response = WssMessage::Method(CancelOrder(ResultResponse {
        result: Some(CancelOrderResult {
            order_id: None,
            warning: None,
            client_order_id: Some("a-uuid".to_string()),
        }),
        error: None,
        success: true,
        req_id: 0,
        time_in: "2024-05-19T19:18:44.987402Z".to_string(),
        time_out: "2024-05-19T19:18:44.989756Z".to_string(),
    }));

    let cancel_order = CancelOrderParams {
        order_id: None,
        client_order_id: Some(vec!["a-uuid".to_string()]),
        order_user_ref: None,
        token: Token::new("thatToken".to_string()),
    };

    let message = Message {
        method: "cancel_order".to_string(),
        params: cancel_order,
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

#[tokio::test]
async fn test_cancel_all_orders() {
    let expected_request = json!({"method":"cancel_all","params":{"token":"thisToken"},"req_id":0});
    let response = r#"{"method":"cancel_all","req_id":0,"result":{"count":0},"success":true,"time_in":"2024-05-19T11:42:13.815662Z","time_out":"2024-05-19T11:42:13.824053Z"}"#.to_string();
    let expected_response = WssMessage::Method(MethodMessage::CancelAllOrders(ResultResponse {
        result: Some(CancelAllOrdersResult {
            count: 0,
            warning: None,
        }),
        error: None,
        success: true,
        req_id: 0,
        time_in: "2024-05-19T11:42:13.815662Z".to_string(),
        time_out: "2024-05-19T11:42:13.824053Z".to_string(),
    }));

    let cancel_all = CancelAllOrdersParams {
        token: Token::new("thisToken".to_string()),
    };

    let message = Message {
        method: "cancel_all".to_string(),
        params: cancel_all,
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

#[tokio::test]
async fn test_cancel_on_disconnect() {
    let expected_request = json!({"method":"cancel_all_orders_after","params":{"timeout":5,"token":"yourToken"},"req_id":0});
    let response = r#"{"method":"cancel_all_orders_after","req_id":0,"result":{"currentTime":"2024-05-19T19:22:20Z","triggerTime":"2024-05-19T19:22:25Z"},"success":true,"time_in":"2024-05-19T19:22:19.975239Z","time_out":"2024-05-19T19:22:19.981369Z"}"#.to_string();
    let expected_response = WssMessage::Method(MethodMessage::CancelOnDisconnect(ResultResponse {
        result: Some(CancelOnDisconnectResult {
            current_time: "2024-05-19T19:22:20Z".into(),
            warning: None,
            trigger_time: "2024-05-19T19:22:25Z".into(),
        }),
        error: None,
        success: true,
        req_id: 0,
        time_in: "2024-05-19T19:22:19.975239Z".to_string(),
        time_out: "2024-05-19T19:22:19.981369Z".to_string(),
    }));

    let cancel_on_disconnect = CancelOnDisconnectParams {
        timeout: 5,
        token: Token::new("yourToken".to_string()),
    };

    let message = Message {
        method: "cancel_all_orders_after".to_string(),
        params: cancel_on_disconnect,
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

#[tokio::test]
async fn test_batch_add() {
    let expected_request = json!({"method":"batch_add","params":{"symbol":"USDC/USD","token":"myToken","orders":[{"order_type":"limit","side":"buy","limit_price":0.99,"order_qty":5.0,"post_only":true,"fee_preference":"quote"},{"order_type":"limit","side":"buy","limit_price":0.95,"order_qty":5.0,"post_only":true,"fee_preference":"base"}]},"req_id":0});
    let response = r#"{"method":"batch_add","req_id":0,"result":[{"order_id":"JQDNTT-MZEIZ-OZKUDD"},{"order_id":"X67GEK-3VQWM-HPNQ89"}],"success":true,"time_in":"2024-05-19T19:23:21.134538Z","time_out":"2024-05-19T19:23:21.141229Z"}"#.to_string();
    let expected_response = WssMessage::Method(MethodMessage::BatchOrder(ResultResponse {
        result: Some(vec![
            AddOrderResult {
                order_id: "JQDNTT-MZEIZ-OZKUDD".to_string(),
                order_user_ref: None,
                warning: None,
                client_order_id: None,
            },
            AddOrderResult {
                order_id: "X67GEK-3VQWM-HPNQ89".to_string(),
                order_user_ref: None,
                warning: None,

                client_order_id: None,
            },
        ]),
        error: None,
        success: true,
        req_id: 0,
        time_in: "2024-05-19T19:23:21.134538Z".to_string(),
        time_out: "2024-05-19T19:23:21.141229Z".to_string(),
    }));

    let batch_add = BatchOrderParams {
        deadline: None,
        symbol: "USDC/USD".to_string(),
        token: Token::new("myToken".to_string()),
        validate: None,
        orders: vec![
            BatchOrder {
                order_type: OrderType::Limit,
                side: BuySell::Buy,
                limit_price: Some(dec!(0.99)),
                limit_price_type: None,
                triggers: None,
                time_in_force: None,
                order_quantity: dec!(5.0),
                margin: None,
                post_only: Some(true),
                reduce_only: None,
                effective_time: None,
                expire_time: None,
                order_user_ref: None,
                conditional: None,
                display_quantity: None,
                fee_preference: Some(FeePreference::Quote),
                no_market_price_protection: None,
                stp_type: None,
                cash_order_quantity: None,
                client_order_id: None,
            },
            BatchOrder {
                order_type: OrderType::Limit,
                side: BuySell::Buy,
                limit_price: Some(dec!(0.95)),
                limit_price_type: None,
                triggers: None,
                time_in_force: None,
                order_quantity: dec!(5.0),
                margin: None,
                post_only: Some(true),
                reduce_only: None,
                effective_time: None,
                expire_time: None,
                order_user_ref: None,
                conditional: None,
                display_quantity: None,
                fee_preference: Some(FeePreference::Base),
                no_market_price_protection: None,
                stp_type: None,
                cash_order_quantity: None,
                client_order_id: None,
            },
        ],
    };

    let message = Message {
        method: "batch_add".to_string(),
        params: batch_add,
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

#[tokio::test]
async fn test_batch_cancel() {
    let expected_request = json!({"method":"batch_cancel","params":{"orders":["IY8YF6-Y6LCR-AMZD7P","XR6VND-GLY6K-DL33TB"],"token":"theirToken", "cl_ord_id": null},"req_id":0});
    let response = r#"{"method":"batch_cancel","orders_cancelled":2,"req_id":0,"success":true,"time_in":"2024-05-19T19:29:58.063754Z","time_out":"2024-05-19T19:29:58.071569Z"}"#.to_string();
    let expected_response = WssMessage::Method(MethodMessage::BatchCancel(BatchCancelResponse {
        orders_cancelled: 2,
        error: None,
        success: true,
        req_id: 0,
        time_in: "2024-05-19T19:29:58.063754Z".to_string(),
        time_out: "2024-05-19T19:29:58.071569Z".to_string(),
        client_order_id: None,
    }));

    let batch_cancel = BatchCancelParams {
        orders: vec!["IY8YF6-Y6LCR-AMZD7P".into(), "XR6VND-GLY6K-DL33TB".into()],
        token: Token::new("theirToken".to_string()),
        client_order_id: None,
    };

    let message = Message {
        method: "batch_cancel".to_string(),
        params: batch_cancel,
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
