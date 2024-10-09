//! These tests use two mocked versions of KrakenClient (TestClient & TestClientErr) to trigger
//! both cases of all the notify_* methods, one where the result is an error (and no order ids are
//! notified), and the other where all successful orders are notified.

mod resources;

use crate::resources::test_client::test_client_impl::{
    get_rate_limit_test_client, TestRateLimitedClient,
};
use crate::resources::test_client::test_client_impl_err::get_rate_limit_test_client_err;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::request_types::{
    AddBatchedOrderRequest, AddOrderRequest, BatchedOrderRequest, CancelBatchOrdersRequest,
    CancelOrderRequest, EditOrderRequest, IntOrString, OrderFlags,
};
use kraken_async_rs::response_types::VerificationTier::{Intermediate, Pro};
use kraken_async_rs::response_types::{AddOrder, BuySell, OrderFlag, OrderType, VerificationTier};
use rust_decimal_macros::dec;
use std::time::Duration;
use tokio::time::{pause, Instant};

#[tokio::test]
async fn test_adding_order_limits() {
    pause();
    let mut client = get_rate_limit_test_client(Pro);
    let mut client_err = get_rate_limit_test_client_err(Pro);

    let start = Instant::now();

    let request = get_add_order_request();

    // the first 180 orders exhaust all tokens, the remaining 15 require 4s of waiting
    //  since the replenishment rate is 375 tokens/s * 4s = 1500
    for _ in 0..(180 + 15) {
        let _ = client.add_order(&request).await;
        let _ = client_err.add_order(&request).await;
    }

    let end = Instant::now();
    let elapsed = end - start;
    println!("{:?}", elapsed);

    assert!(elapsed > Duration::from_secs(4));
    assert!(elapsed < Duration::from_secs(5));
}

#[tokio::test]
async fn test_add_order_batch_limits() {
    pause();
    let mut client = get_rate_limit_test_client(Pro);
    let mut client_err = get_rate_limit_test_client_err(Pro);

    let start = Instant::now();

    let request = get_batched_order_request(16);

    // batched order of 16 should cost (1 + n / 2) * 100 = 900 each, so 21 * 900 = 18,900
    // replenishing the 900 after the pro limit should take 3s
    for _ in 0..21 {
        let _ = client.add_order_batch(&request).await;
        let _ = client_err.add_order_batch(&request).await;
    }

    let end = Instant::now();
    let elapsed = end - start;
    println!("{:?}", elapsed);

    assert!(elapsed > Duration::from_secs(3));
    assert!(elapsed < Duration::from_secs(4));
}

#[tokio::test]
async fn test_edit_order_max_penalty() {
    pause();
    let verification = Pro;
    let mut client = get_rate_limit_test_client(verification);
    let mut client_err = get_rate_limit_test_client_err(Pro);

    let orders = max_out_rate_limits(&mut client, verification).await;

    let edit_start = Instant::now();

    // 6 instant edits costs 700 each, for 4200 total, 4200 / 375 = ~11.23 (requires 12s wait)
    for i in 0..6 {
        let edit_request = edit_from_order(orders.get(i).unwrap());
        let _ = client.edit_order(&edit_request).await;
    }

    // initiating more edits for the error client has no effect, since each err return did not add
    //  an order id / lifetime
    for i in 0..12 {
        let edit_request = edit_from_order(orders.get(i).unwrap());
        let _ = client_err.edit_order(&edit_request).await;
    }

    let edit_end = Instant::now();
    let edit_elapsed = edit_end - edit_start;
    println!("{:?}", edit_elapsed);

    assert!(edit_elapsed > Duration::from_secs(12));
    assert!(edit_elapsed < Duration::from_secs(13));
}

#[tokio::test]
async fn test_cancel_order_max_penalty() {
    pause();
    let verification = Intermediate;
    let mut client = get_rate_limit_test_client(verification);
    let mut client_err = get_rate_limit_test_client_err(Pro);

    let orders = max_out_rate_limits(&mut client, verification).await;

    let edit_start = Instant::now();

    // 4 instant cancels costs 800 each, for 3200 total, 3200 / 234 = ~13.67 (requires 14s wait)
    for i in 0..4 {
        let cancel_request = cancel_from_order(orders.get(i).unwrap());
        let _ = client.cancel_order(&cancel_request).await;
        let _ = client_err.cancel_order(&cancel_request).await;
    }

    // initiating more cancels for the error client has no effect, since each err return did not add
    //  an order id / lifetime
    for i in 0..12 {
        let cancel_request = cancel_from_order(orders.get(i).unwrap());
        let _ = client_err.cancel_order(&cancel_request).await;
    }

    let edit_end = Instant::now();
    let edit_elapsed = edit_end - edit_start;
    println!("{:?}", edit_elapsed);

    assert!(edit_elapsed > Duration::from_secs(14));
    assert!(edit_elapsed < Duration::from_secs(15));
}

#[tokio::test]
async fn test_cancel_order_batch_with_max_penalty() {
    pause();
    let verification = Intermediate;
    let mut client = get_rate_limit_test_client(verification);
    let mut client_err = get_rate_limit_test_client_err(Pro);

    let mut orders = max_out_rate_limits(&mut client, verification).await;

    let edit_start = Instant::now();

    let mut order_ids = Vec::new();
    for i in 0..4 {
        let id = IntOrString::String(orders.get(i).unwrap().tx_id.first().unwrap().clone());
        order_ids.push(id);
    }

    let user_ref_request = get_add_order_request_user_ref();
    orders.push(
        client
            .add_order(&user_ref_request)
            .await
            .unwrap()
            .result
            .unwrap(),
    );
    order_ids.push(IntOrString::Int(user_ref_request.user_ref.unwrap()));

    let batch_cancel_request = CancelBatchOrdersRequest {
        orders: order_ids,
        client_order_ids: None,
    };

    // 1 additional order w/ user ref costs 100, 5 instant cancels cost 800 each, for 4100 total,
    // making 4100 / 234 = ~17.52 (requires 18s wait)
    let _ = client.cancel_order_batch(&batch_cancel_request).await;

    // failures don't add anything to wait
    for _ in 0..5 {
        let _ = client_err.cancel_order_batch(&batch_cancel_request).await;
    }

    let edit_end = Instant::now();
    let edit_elapsed = edit_end - edit_start;
    println!("{:?}", edit_elapsed);

    assert!(edit_elapsed > Duration::from_secs(18));
    assert!(edit_elapsed < Duration::from_secs(19));
}

/// Depending on the verification tier, submit enough orders to empty the rate limit bucket and
/// return the created orders. Also checks that it has not exceeded the limits (executes in < 10ms).
async fn max_out_rate_limits(
    client: &mut TestRateLimitedClient,
    verification_tier: VerificationTier,
) -> Vec<AddOrder> {
    let start = Instant::now();

    let request = get_add_order_request();

    let n_orders = match verification_tier {
        Intermediate => 125,
        Pro => 180,
    };

    // the first 180 orders exhaust all tokens
    let mut orders = Vec::new();
    for _ in 0..n_orders {
        let order = client.add_order(&request).await.unwrap().result.unwrap();
        orders.push(order);
    }

    let end = Instant::now();
    let elapsed = end - start;
    println!("{:?}", elapsed);

    assert!(elapsed >= Duration::from_secs(0));
    assert!(elapsed < Duration::from_millis(10));
    orders
}

fn get_add_order_request() -> AddOrderRequest {
    let order_flags = OrderFlags::new(vec![OrderFlag::NoMarketPriceProtection, OrderFlag::Post]);

    AddOrderRequest::builder(
        OrderType::Market,
        BuySell::Buy,
        dec!(5.0),
        "USDCUSD".to_string(),
    )
    .order_flags(order_flags)
    .price(dec!(0.90))
    .build()
}

fn get_add_order_request_user_ref() -> AddOrderRequest {
    let order_flags = OrderFlags::new(vec![OrderFlag::NoMarketPriceProtection, OrderFlag::Post]);

    AddOrderRequest::builder(
        OrderType::Market,
        BuySell::Buy,
        dec!(5.0),
        "USDCUSD".to_string(),
    )
    .user_ref(42)
    .order_flags(order_flags)
    .price(dec!(0.90))
    .build()
}

fn get_batched_order_request(n_orders: u64) -> AddBatchedOrderRequest {
    let mut orders = Vec::new();

    for _ in 0..n_orders {
        let order = BatchedOrderRequest::builder(OrderType::Limit, BuySell::Buy, dec!(5.1))
            .price(dec!(0.9))
            .start_time("0".to_string())
            .expire_time("+5".to_string())
            .build();

        orders.push(order);
    }

    AddBatchedOrderRequest::builder(orders, "USDCUSD".to_string()).build()
}

fn edit_from_order(order: &AddOrder) -> EditOrderRequest {
    let edit_request = EditOrderRequest {
        user_ref: None,
        tx_id: order.tx_id.first().unwrap().clone(),
        volume: dec!(0),
        display_volume: None,
        pair: "".to_string(),
        price: None,
        price_2: None,
        order_flags: None,
        deadline: None,
        cancel_response: None,
        validate: None,
    };
    edit_request
}

fn cancel_from_order(order: &AddOrder) -> CancelOrderRequest {
    CancelOrderRequest {
        tx_id: IntOrString::String(order.tx_id.first().unwrap().clone()),
        client_order_id: None,
    }
}
