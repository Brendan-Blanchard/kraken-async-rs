use crate::resources::test_auth::get_null_secrets_provider;
use serde_json::json;

mod resources;

use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
use kraken_async_rs::clients::kraken_client::KrakenClient;

use crate::resources::kraken_responses::trading_response_json::{
    get_add_order_batch_json, get_add_order_json, get_amend_order_json,
    get_cancel_all_orders_after_json, get_cancel_all_orders_json, get_cancel_order_batch_json,
    get_cancel_order_json, get_edit_order_json,
};
use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
use kraken_async_rs::request_types::{
    AddBatchedOrderRequest, AddOrderRequest, AmendOrderRequest, BatchedOrderRequest,
    CancelAllOrdersAfterRequest, CancelBatchOrdersRequest, CancelOrderRequest, EditOrderRequest,
    IntOrString, OrderFlags,
};
use kraken_async_rs::response_types::{BuySell, OrderFlag, OrderType};
use wiremock::matchers::{body_partial_json, body_string_contains, header_exists, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use rust_decimal_macros::dec;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::test]
async fn test_add_order() {
    let secrets_provider = get_null_secrets_provider();

    let order_flags = OrderFlags::new(vec![OrderFlag::NoMarketPriceProtection, OrderFlag::Post]);
    let request = AddOrderRequest::builder(
        OrderType::Market,
        BuySell::Buy,
        dec!(5.0),
        "USDCUSD".to_string(),
    )
    .order_flags(order_flags)
    .price(dec!(0.90))
    .build();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/AddOrder"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("price=0.90"))
        .and(body_string_contains("ordertype=market"))
        .and(body_string_contains("type=buy"))
        .and(body_string_contains("volume=5.0"))
        .and(body_string_contains("pair=USDCUSD"))
        .and(body_string_contains("oflags=nompp%2Cpost"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_add_order_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, add_order, &request);
}

#[tokio::test]
async fn test_add_order_batch() {
    let secrets_provider = get_null_secrets_provider();
    let order_1 = BatchedOrderRequest::builder(OrderType::Limit, BuySell::Buy, dec!(5.1))
        .price(dec!(0.9))
        .start_time("0".to_string())
        .expire_time("+5".to_string())
        .build();

    let order_2 = BatchedOrderRequest::builder(OrderType::Limit, BuySell::Sell, dec!(5.2))
        .price(dec!(0.9))
        .order_flags(vec![OrderFlag::Post])
        .build();

    let request =
        AddBatchedOrderRequest::builder(vec![order_1, order_2], "USDCUSD".to_string()).build();

    let mock_server = MockServer::start().await;

    let expected_json = json!({
        "orders": [
            {"ordertype": "limit", "type": "buy", "volume": "5.1", "price": "0.9", "starttm": "0", "expiretm": "+5"},
            {"ordertype": "limit", "type": "sell", "volume": "5.2", "price": "0.9", "oflags": "post"}
        ],
        "pair":"USDCUSD"
    });

    Mock::given(method("POST"))
        .and(path("/0/private/AddOrderBatch"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_partial_json(expected_json))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_add_order_batch_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, add_order_batch, &request);
}

#[tokio::test]
async fn test_amend_order() {
    let secrets_provider = get_null_secrets_provider();

    let amend_request = AmendOrderRequest::builder()
        .tx_id("tx-id".to_string())
        .order_quantity(dec!(5.25))
        .limit_price(dec!(0.96).to_string())
        .post_only(true)
        .build();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/AmendOrder"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains(r#""txid":"tx-id""#))
        .and(body_string_contains(r#""order_qty":"5.25""#))
        .and(body_string_contains(r#""limit_price":"0.96""#))
        .and(body_string_contains(r#""post_only":true"#))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_amend_order_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, amend_order, &amend_request);
}

#[tokio::test]
async fn test_edit_order() {
    let secrets_provider = get_null_secrets_provider();
    let request = EditOrderRequest::builder(
        "7BD466-BKZVM-FT2E2L".to_string(),
        dec!(5.1),
        "USDCUSD".to_string(),
    )
    .price(dec!(0.89))
    .user_ref(1234)
    .build();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/EditOrder"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("price=0.89"))
        .and(body_string_contains("volume=5.1"))
        .and(body_string_contains("userref=1234"))
        .and(body_string_contains("txid=7BD466-BKZVM-FT2E2L"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_edit_order_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, edit_order, &request);
}

#[tokio::test]
async fn test_cancel_order() {
    let secrets_provider = get_null_secrets_provider();

    let txid = IntOrString::String("7BD466-BKZVM-FT2E2L".to_string());
    let request = CancelOrderRequest::builder(txid).build();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/CancelOrder"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("txid=7BD466-BKZVM-FT2E2L"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_cancel_order_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, cancel_order, &request);
}

#[tokio::test]
async fn test_cancel_all_orders() {
    let secrets_provider = get_null_secrets_provider();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/CancelAll"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_cancel_all_orders_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, cancel_all_orders);
}

#[tokio::test]
async fn test_cancel_all_orders_after() {
    let secrets_provider = get_null_secrets_provider();

    let request = CancelAllOrdersAfterRequest::builder(180).build();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/0/private/CancelAllOrdersAfter"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("timeout=180"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_cancel_all_orders_after_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(
        secrets_provider,
        mock_server,
        cancel_all_orders_after,
        &request
    );
}

#[tokio::test]
async fn test_cancel_order_batch() {
    let secrets_provider = get_null_secrets_provider();
    let tx_ids = vec![
        "OZICHZ-FGB63-156I4K".to_string(),
        "BEGNMD-FEJKF-VC6U8Y".to_string(),
    ];
    let request = CancelBatchOrdersRequest::from_tx_ids(tx_ids);

    let mock_server = MockServer::start().await;

    let expected_json = json!({
        "orders": ["OZICHZ-FGB63-156I4K", "BEGNMD-FEJKF-VC6U8Y"],
    });

    Mock::given(method("POST"))
        .and(path("/0/private/CancelOrderBatch"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_partial_json(expected_json))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_cancel_order_batch_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, cancel_order_batch, &request);
}
