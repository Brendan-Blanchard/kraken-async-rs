use crate::wss_v2::shared::ParseIncomingTest;
use kraken_async_rs::response_types::BuySell;
use kraken_async_rs::wss::{
    Asset, AssetStatus, BidAsk, ChannelMessage, Instruments, L3BidAsk, L3BidAskUpdate, L3Orderbook,
    L3OrderbookUpdate, MarketDataResponse, MarketLimit, Ohlc, Orderbook, OrderbookEvent,
    OrderbookUpdate, Pair, PairStatus, SingleResponse, Ticker, Trade, WssMessage, L2, L3,
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

#[tokio::test]
async fn test_l3_snapshot() {
    let l3_snapshot = r#"{
        "channel":"level3",
        "type":"snapshot",
        "data": [{
        "symbol":"BTC/USD",
        "checksum":1361442827,
        "bids":[
            {"order_id":"OZYA6B-OE3BH-YJ4PY5","limit_price":66579.2,"order_qty":1.35137590,"timestamp":"2024-05-19T18:55:20.910159752Z"},
            {"order_id":"OIOQ7V-JT5S2-QLIEPO","limit_price":66579.2,"order_qty":0.47905712,"timestamp":"2024-05-19T18:55:20.910276406Z"},
            {"order_id":"O34I4J-KIE3I-BOT6VC","limit_price":66579.2,"order_qty":0.03003941,"timestamp":"2024-05-19T18:55:23.001943740Z"},
            {"order_id":"OUOCIK-GA6WX-DSZC2A","limit_price":66574.1,"order_qty":0.45057561,"timestamp":"2024-05-19T18:55:15.431184641Z"}
        ],
        "asks":[
            {"order_id":"OUPTOY-CCUJG-BMAZ5S","limit_price":66579.3,"order_qty":0.07800000,"timestamp":"2024-05-19T18:55:22.531833732Z"},
            {"order_id":"OFUNE7-IGNAY-5UATGI","limit_price":66581.5,"order_qty":1.50192021,"timestamp":"2024-05-19T18:55:25.967603045Z"},
            {"order_id":"ORCUC4-UGIUC-MT5KBA","limit_price":66583.7,"order_qty":0.87745184,"timestamp":"2024-05-19T18:55:18.938264721Z"}
        ]
    }]}"#.to_string();

    let expected_snapshot = WssMessage::Channel(ChannelMessage::L3(SingleResponse {
        data: L3::Orderbook(L3Orderbook {
            symbol: "BTC/USD".to_string(),
            bids: vec![
                L3BidAsk {
                    order_id: "OZYA6B-OE3BH-YJ4PY5".to_string(),
                    limit_price: dec!(66579.2),
                    order_quantity: dec!(1.35137590),
                    timestamp: "2024-05-19T18:55:20.910159752Z".to_string(),
                },
                L3BidAsk {
                    order_id: "OIOQ7V-JT5S2-QLIEPO".to_string(),
                    limit_price: dec!(66579.2),
                    order_quantity: dec!(0.47905712),
                    timestamp: "2024-05-19T18:55:20.910276406Z".to_string(),
                },
                L3BidAsk {
                    order_id: "O34I4J-KIE3I-BOT6VC".to_string(),
                    limit_price: dec!(66579.2),
                    order_quantity: dec!(0.03003941),
                    timestamp: "2024-05-19T18:55:23.001943740Z".to_string(),
                },
                L3BidAsk {
                    order_id: "OUOCIK-GA6WX-DSZC2A".to_string(),
                    limit_price: dec!(66574.1),
                    order_quantity: dec!(0.45057561),
                    timestamp: "2024-05-19T18:55:15.431184641Z".to_string(),
                },
            ],
            asks: vec![
                L3BidAsk {
                    order_id: "OUPTOY-CCUJG-BMAZ5S".to_string(),
                    limit_price: dec!(66579.3),
                    order_quantity: dec!(0.07800000),
                    timestamp: "2024-05-19T18:55:22.531833732Z".to_string(),
                },
                L3BidAsk {
                    order_id: "OFUNE7-IGNAY-5UATGI".to_string(),
                    limit_price: dec!(66581.5),
                    order_quantity: dec!(1.50192021),
                    timestamp: "2024-05-19T18:55:25.967603045Z".to_string(),
                },
                L3BidAsk {
                    order_id: "ORCUC4-UGIUC-MT5KBA".to_string(),
                    limit_price: dec!(66583.7),
                    order_quantity: dec!(0.87745184),
                    timestamp: "2024-05-19T18:55:18.938264721Z".to_string(),
                },
            ],
            checksum: 1361442827,
        }),
    }));

    ParseIncomingTest::new()
        .with_incoming(l3_snapshot)
        .expect_message(expected_snapshot)
        .test()
        .await;
}

#[tokio::test]
async fn test_l3_update() {
    let l3_update = r#"{
        "channel":"level3",
        "type":"update",
        "data":[{
            "checksum":2143854316,
            "symbol":"BTC/USD",
            "bids":[
                {
                    "event":"delete",
                    "order_id":"O7SO4Y-RHRAK-GGAHJE",
                    "limit_price":66567.3,
                    "order_qty":0.22540000,
                    "timestamp":"2024-05-19T18:59:46.541105556Z"
                },
                {
                    "event":"add",
                    "order_id":"OI2XQ5-6JUYI-A5NI6J",
                    "limit_price":66566.9,
                    "order_qty":2.82230268,
                    "timestamp":"2024-05-19T18:59:44.900460701Z"
                }
            ],
            "asks":[]
        }]
    }"#
    .to_string();

    let expected_update = WssMessage::Channel(ChannelMessage::L3(SingleResponse {
        data: L3::Update(L3OrderbookUpdate {
            symbol: "BTC/USD".to_string(),
            bids: vec![
                L3BidAskUpdate {
                    event: OrderbookEvent::Delete,
                    order_id: "O7SO4Y-RHRAK-GGAHJE".to_string(),
                    limit_price: dec!(66567.3),
                    order_quantity: dec!(0.22540000),
                    timestamp: "2024-05-19T18:59:46.541105556Z".to_string(),
                },
                L3BidAskUpdate {
                    event: OrderbookEvent::Add,
                    order_id: "OI2XQ5-6JUYI-A5NI6J".to_string(),
                    limit_price: dec!(66566.9),
                    order_quantity: dec!(2.82230268),
                    timestamp: "2024-05-19T18:59:44.900460701Z".to_string(),
                },
            ],
            asks: vec![],
            checksum: 2143854316,
        }),
    }));

    ParseIncomingTest::new()
        .with_incoming(l3_update)
        .expect_message(expected_update)
        .test()
        .await;
}

#[tokio::test]
async fn test_candles_snapshot() {
    let candles_snapshot = r#"{
        "channel":"ohlc",
        "type":"snapshot",
        "timestamp":"2024-05-17T11:21:16.318303322Z",
        "data":[
            {"symbol":"ETH/USD","open":3027.80,"high":3027.80,"low":3026.13,"close":3026.13,"trades":9,"volume":13.31603062,"vwap":3027.01,"interval_begin":"2024-05-17T11:12:00.000000000Z","interval":1,"timestamp":"2024-05-17T11:13:00.000000Z"},
            {"symbol":"ETH/USD","open":3026.46,"high":3026.47,"low":3026.46,"close":3026.47,"trades":4,"volume":2.14044498,"vwap":3026.46,"interval_begin":"2024-05-17T11:13:00.000000000Z","interval":1,"timestamp":"2024-05-17T11:14:00.000000Z"}
        ]
    }"#
    .to_string();

    let expected_snapshot = WssMessage::Channel(ChannelMessage::Ohlc(MarketDataResponse {
        data: vec![
            Ohlc {
                symbol: "ETH/USD".to_string(),
                open: dec!(3027.80),
                high: dec!(3027.80),
                low: dec!(3026.13),
                close: dec!(3026.13),
                vwap: dec!(3027.01),
                trades: 9,
                volume: dec!(13.31603062),
                interval_begin: "2024-05-17T11:12:00.000000000Z".to_string(),
                interval: 1,
            },
            Ohlc {
                symbol: "ETH/USD".to_string(),
                open: dec!(3026.46),
                high: dec!(3026.47),
                low: dec!(3026.46),
                close: dec!(3026.47),
                vwap: dec!(3026.46),
                trades: 4,
                volume: dec!(2.14044498),
                interval_begin: "2024-05-17T11:13:00.000000000Z".to_string(),
                interval: 1,
            },
        ],
    }));

    ParseIncomingTest::new()
        .with_incoming(candles_snapshot)
        .expect_message(expected_snapshot)
        .test()
        .await;
}

#[tokio::test]
async fn test_trade_snapshot() {
    let trade_snapshot = r#"{
        "channel":"trade",
        "type":"snapshot",
        "data":[
            {"symbol":"BTC/USD","side":"sell","price":68466.9,"qty":0.01919415,"ord_type":"market","trade_id":70635251,"timestamp":"2024-05-27T12:33:10.826003Z"},
            {"symbol":"BTC/USD","side":"buy","price":68471.2,"qty":0.00007723,"ord_type":"limit","trade_id":70635252,"timestamp":"2024-05-27T12:33:10.980704Z"}
        ]
    }"#.to_string();

    let expected_snapshot = WssMessage::Channel(ChannelMessage::Trade(MarketDataResponse {
        data: vec![
            Trade {
                symbol: "BTC/USD".to_string(),
                side: BuySell::Sell,
                quantity: dec!(0.01919415),
                price: dec!(68466.9),
                order_type: MarketLimit::Market,
                trade_id: 70635251,
                timestamp: "2024-05-27T12:33:10.826003Z".to_string(),
            },
            Trade {
                symbol: "BTC/USD".to_string(),
                side: BuySell::Buy,
                quantity: dec!(0.00007723),
                price: dec!(68471.2),
                order_type: MarketLimit::Limit,
                trade_id: 70635252,
                timestamp: "2024-05-27T12:33:10.980704Z".to_string(),
            },
        ],
    }));

    ParseIncomingTest::new()
        .with_incoming(trade_snapshot)
        .expect_message(expected_snapshot)
        .test()
        .await;
}

#[tokio::test]
async fn test_trade_update() {
    let trade_update = r#"{
        "channel":"trade",
        "type":"update",
        "data":[
            {"symbol":"BTC/USD","side":"buy","price":68500.0,"qty":0.01044926,"ord_type":"limit","trade_id":70635299,"timestamp":"2024-05-27T12:43:11.798009Z"},
            {"symbol":"BTC/USD","side":"buy","price":68500.0,"qty":0.00483192,"ord_type":"limit","trade_id":70635300,"timestamp":"2024-05-27T12:43:11.798009Z"}
        ]
    }"#.to_string();

    let expected_update = WssMessage::Channel(ChannelMessage::Trade(MarketDataResponse {
        data: vec![
            Trade {
                symbol: "BTC/USD".to_string(),
                side: BuySell::Buy,
                quantity: dec!(0.01044926),
                price: dec!(68500.0),
                order_type: MarketLimit::Limit,
                trade_id: 70635299,
                timestamp: "2024-05-27T12:43:11.798009Z".to_string(),
            },
            Trade {
                symbol: "BTC/USD".to_string(),
                side: BuySell::Buy,
                quantity: dec!(0.00483192),
                price: dec!(68500.0),
                order_type: MarketLimit::Limit,
                trade_id: 70635300,
                timestamp: "2024-05-27T12:43:11.798009Z".to_string(),
            },
        ],
    }));

    ParseIncomingTest::new()
        .with_incoming(trade_update)
        .expect_message(expected_update)
        .test()
        .await;
}

#[tokio::test]
async fn test_instruments_snapshot() {
    let instrument_snapshot = r#"{
        "channel":"instrument",
        "type":"snapshot",
        "data":{
            "assets":[
                {"id":"USD","status":"enabled","precision":4,"precision_display":2,"borrowable":true,"collateral_value":1.00,"margin_rate":0.025000},
                {"id":"EUR","status":"enabled","precision":4,"precision_display":2,"borrowable":true,"collateral_value":1.00,"margin_rate":0.020000},
                {"id":"ETH","status":"enabled","precision":10,"precision_display":5,"borrowable":true,"collateral_value":1.00,"margin_rate":0.020000}
            ],
            "pairs": [
                {"symbol":"EUR/USD","base":"EUR","quote":"USD","status":"online","qty_precision":8,"qty_increment":0.00000001,"price_precision":5,"cost_precision":5,"marginable":false,"has_index":true,"cost_min":0.50,"tick_size":0.00001,"price_increment":0.00001,"qty_min":0.50000000},
                {"symbol":"ETH/BTC","base":"ETH","quote":"BTC","status":"online","qty_precision":8,"qty_increment":0.00000001,"price_precision":5,"cost_precision":10,"marginable":true,"has_index":true,"cost_min":0.00002,"margin_initial":0.20,"position_limit_long":1000,"position_limit_short":600,"tick_size":0.00001,"price_increment":0.00001,"qty_min":0.00200000}
            ]
        }
    }"#.to_string();

    let expected_snapshot = WssMessage::Channel(ChannelMessage::Instrument(MarketDataResponse {
        data: Instruments {
            assets: vec![
                Asset {
                    id: "USD".to_string(),
                    margin_rate: Some(dec!(0.025000)),
                    precision: 4,
                    precision_display: 2,
                    status: AssetStatus::Enabled,
                    borrowable: true,
                    collateral_value: dec!(1.0),
                },
                Asset {
                    id: "EUR".to_string(),
                    margin_rate: Some(dec!(0.020000)),
                    precision: 4,
                    precision_display: 2,
                    status: AssetStatus::Enabled,
                    borrowable: true,
                    collateral_value: dec!(1.0),
                },
                Asset {
                    id: "ETH".to_string(),
                    margin_rate: Some(dec!(0.020000)),
                    precision: 10,
                    precision_display: 5,
                    status: AssetStatus::Enabled,
                    borrowable: true,
                    collateral_value: dec!(1.0),
                },
            ],
            pairs: vec![
                Pair {
                    base: "EUR".to_string(),
                    quote: "USD".to_string(),
                    cost_min: dec!(0.50),
                    cost_precision: 5,
                    has_index: true,
                    margin_initial: None,
                    marginable: false,
                    position_limit_long: None,
                    position_limit_short: None,
                    price_increment: dec!(0.00001),
                    price_precision: 5,
                    quantity_increment: dec!(0.00000001),
                    quantity_min: dec!(0.50),
                    quantity_precision: 8,
                    status: PairStatus::Online,
                    symbol: "EUR/USD".to_string(),
                },
                Pair {
                    base: "ETH".to_string(),
                    quote: "BTC".to_string(),
                    cost_min: dec!(0.00002),
                    cost_precision: 10,
                    has_index: true,
                    margin_initial: Some(dec!(0.2)),
                    marginable: true,
                    position_limit_long: Some(1000),
                    position_limit_short: Some(600),
                    price_increment: dec!(0.00001),
                    price_precision: 5,
                    quantity_increment: dec!(0.00000001),
                    quantity_min: dec!(0.002),
                    quantity_precision: 8,
                    status: PairStatus::Online,
                    symbol: "ETH/BTC".to_string(),
                },
            ],
        },
    }));

    ParseIncomingTest::new()
        .with_incoming(instrument_snapshot)
        .expect_message(expected_snapshot)
        .test()
        .await;
}
