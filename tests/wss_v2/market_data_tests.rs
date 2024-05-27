use crate::wss_v2::shared::ParseIncomingTest;
use kraken_async_rs::wss::v2::base_messages::{ChannelMessage, SingleResponse, WssMessage};
use kraken_async_rs::wss::v2::market_data_messages::Ticker;
use rust_decimal_macros::dec;

#[tokio::test]
async fn test_ticker_snapshot() {
    let ticker_snapshot = r#"{
        "channel":"ticker",
        "type":"snapshot",
        "data":[{
            "symbol":"BTC/USD",
            "bid":65972.8,
            "bid_qty":0.10000000,
            "ask":65972.9,
            "ask_qty":39.67506683,
            "last":65972.9,
            "volume":4216.61829370,
            "vwap":64275.2,
            "low":61325.4,
            "high":66450.0,
            "change":4412.1,
            "change_pct":7.17
        }]
    }"#
    .to_string();

    let expected_snapshot = WssMessage::Channel(ChannelMessage::Ticker(SingleResponse {
        data: Ticker {
            ask: dec!(65972.9),
            ask_quantity: dec!(39.67506683),
            bid: dec!(65972.8),
            bid_quantity: dec!(0.10000000),
            change: dec!(4412.1),
            change_pct: dec!(7.17),
            high: dec!(66450.0),
            last: dec!(65972.9),
            low: dec!(61325.4),
            symbol: "BTC/USD".to_string(),
            volume: dec!(4216.61829370),
            vwap: dec!(64275.2),
        },
    }));

    ParseIncomingTest::new()
        .with_incoming(ticker_snapshot)
        .expect_message(expected_snapshot)
        .test()
        .await;
}
#[tokio::test]
async fn test_ticker_update() {
    let ticker_update = r#"{
        "channel":"ticker",
        "type":"update",
        "data":[{
            "symbol":"BTC/USD",
            "bid":65843.7,
            "bid_qty":12.31628629,
            "ask":65843.8,
            "ask_qty":0.31232000,
            "last":65843.7,
            "volume":4182.59447976,
            "vwap":64223.4,
            "low":61325.4,
            "high":66450.0,
            "change":4213.8,
            "change_pct":6.84
        }]
    }"#
    .to_string();

    let expected_update = WssMessage::Channel(ChannelMessage::Ticker(SingleResponse {
        data: Ticker {
            ask: dec!(65843.8),
            ask_quantity: dec!(0.31232000),
            bid: dec!(65843.7),
            bid_quantity: dec!(12.31628629),
            change: dec!(4213.8),
            change_pct: dec!(6.84),
            high: dec!(66450.0),
            last: dec!(65843.7),
            low: dec!(61325.4),
            symbol: "BTC/USD".to_string(),
            volume: dec!(4182.59447976),
            vwap: dec!(64223.4),
        },
    }));

    ParseIncomingTest::new()
        .with_incoming(ticker_update)
        .expect_message(expected_update)
        .test()
        .await;
}
