use crate::wss_v2::shared::ParseIncomingTest;
use kraken_async_rs::wss::v2::base_messages::{ChannelMessage, SingleResponse, WssMessage};
use kraken_async_rs::wss::v2::market_data_messages::{
    BidAsk, Orderbook, OrderbookUpdate, Ticker, L2,
};
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

#[tokio::test]
async fn test_book_snapshot() {
    let book_snapshot = r#"{
        "channel":"book",
        "type":"snapshot",
        "data":[{
            "symbol":"BTC/USD",
            "bids":[
                {"price":66788.0,"qty":3.21926649},
                {"price":66787.5,"qty":0.44916298},
                {"price":66787.4,"qty":0.05992580},
                {"price":66785.3,"qty":0.01496904},
                {"price":66785.2,"qty":0.86989511}
            ],
            "asks":[
                {"price":66788.1,"qty":1.67939137},
                {"price":66788.4,"qty":1.49726637},
                {"price":66790.0,"qty":1.49723133},
                {"price":66791.1,"qty":0.01100000},
                {"price":66792.6,"qty":1.49717197}
            ],
            "checksum":2330500275
        }]
    }"#
    .to_string();

    let expected_snapshot = WssMessage::Channel(ChannelMessage::Orderbook(SingleResponse {
        data: L2::Orderbook(Orderbook {
            symbol: "BTC/USD".to_string(),
            checksum: 2330500275,
            bids: vec![
                BidAsk {
                    price: dec!(66788.0),
                    quantity: dec!(3.21926649),
                },
                BidAsk {
                    price: dec!(66787.5),
                    quantity: dec!(0.44916298),
                },
                BidAsk {
                    price: dec!(66787.4),
                    quantity: dec!(0.05992580),
                },
                BidAsk {
                    price: dec!(66785.3),
                    quantity: dec!(0.01496904),
                },
                BidAsk {
                    price: dec!(66785.2),
                    quantity: dec!(0.86989511),
                },
            ],
            asks: vec![
                BidAsk {
                    price: dec!(66788.1),
                    quantity: dec!(1.67939137),
                },
                BidAsk {
                    price: dec!(66788.4),
                    quantity: dec!(1.49726637),
                },
                BidAsk {
                    price: dec!(66790.0),
                    quantity: dec!(1.49723133),
                },
                BidAsk {
                    price: dec!(66791.1),
                    quantity: dec!(0.01100000),
                },
                BidAsk {
                    price: dec!(66792.6),
                    quantity: dec!(1.49717197),
                },
            ],
        }),
    }));

    ParseIncomingTest::new()
        .with_incoming(book_snapshot)
        .expect_message(expected_snapshot)
        .test()
        .await;
}

#[tokio::test]
async fn test_book_update() {
    let book_update = r#"{
        "channel":"book",
        "type":"update",
        "data":[{
            "symbol":"BTC/USD",
            "bids":[
                {"price":66786.5,"qty":0.00000000},
                {"price":66784.5,"qty":0.01470022},
                {"price":66787.7,"qty":0.12440000}
            ],
            "asks":[],
            "checksum":902440905,
            "timestamp":"2024-05-19T16:45:24.204654Z"
        }]
    }"#
    .to_string();

    let expected_update = WssMessage::Channel(ChannelMessage::Orderbook(SingleResponse {
        data: L2::Update(OrderbookUpdate {
            symbol: "BTC/USD".to_string(),
            checksum: 902440905,
            timestamp: "2024-05-19T16:45:24.204654Z".to_string(),
            bids: vec![
                BidAsk {
                    price: dec!(66786.5),
                    quantity: dec!(0.00000000),
                },
                BidAsk {
                    price: dec!(66784.5),
                    quantity: dec!(0.01470022),
                },
                BidAsk {
                    price: dec!(66787.7),
                    quantity: dec!(0.12440000),
                },
            ],
            asks: vec![],
        }),
    }));

    ParseIncomingTest::new()
        .with_incoming(book_update)
        .expect_message(expected_update)
        .test()
        .await;
}
