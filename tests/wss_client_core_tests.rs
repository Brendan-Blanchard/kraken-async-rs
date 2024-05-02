mod resources;

#[cfg(test)]
mod tests {
    use futures_util::StreamExt;
    use kraken_async_rs::response_types::{
        BuySellChar, LastTrade, MarketLimitChar, TickerBidAsk, TickerDecimal, TickerTrades,
    };
    use kraken_async_rs::wss::kraken_wss_client::KrakenWSSClient;
    use kraken_async_rs::wss::kraken_wss_types::{PingPong, Status, SystemStatus};
    use kraken_async_rs::wss::public::messages::{
        Message, PublicMessage, PublicTrade, Spread, WSSTickerInfo, OHLC,
    };
    use kraken_async_rs::wss::public::orderbooks::{
        BidAsk, BidAskUpdate, Orderbook, OrderbookUpdateMessage,
    };
    use kraken_async_rs::wss::subscribe_messages::{
        SubscribeMessage, Subscription, UnsubscribeMessage,
    };
    use rust_decimal_macros::dec;
    use serde_json::json;
    use std::time::Duration;
    use tokio::sync::mpsc;
    use tokio::time::timeout;
    use ws_mock::matchers::JsonExact;
    use ws_mock::ws_mock_server::{WsMock, WsMockServer};

    use crate::resources::wss::responses::*;

    #[tokio::test]
    async fn test_subscribe() {
        let subscription = Subscription::new_trades_subscription();
        let subscribe_message =
            SubscribeMessage::new(10312008, Some(vec!["XBT/USD".to_string()]), subscription);

        let expected = json!({
            "event": "subscribe",
            "reqid": 10312008,
            "pair": ["XBT/USD"],
            "subscription": { "name": "trade" }
        });

        let server = WsMockServer::start().await;
        let url = server.uri().await;

        WsMock::new()
            .matcher(JsonExact::new(expected))
            .expect(1)
            .mount(&server)
            .await;

        let mut client = KrakenWSSClient::new_with_urls(url.as_str(), url.as_str());

        let mut message_stream = client.connect().await.unwrap();

        let result = message_stream.subscribe(&subscribe_message).await;
        server.verify().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_unsubscribe() {
        let subscription = Subscription::new_trades_subscription();
        let unsubscribe_message = UnsubscribeMessage::new(
            10312008,
            Some(vec!["ETH/USD".to_string()]),
            subscription.into(),
        );

        let expected = json!({
            "event": "unsubscribe",
            "reqid": 10312008,
            "pair": ["ETH/USD"],
            "subscription": { "name": "trade" }
        });

        let server = WsMockServer::start().await;
        let url = server.uri().await;

        WsMock::new()
            .matcher(JsonExact::new(expected))
            .expect(1)
            .mount(&server)
            .await;

        let mut client = KrakenWSSClient::new_with_urls(url.as_str(), url.as_str());

        let mut message_stream = client.connect().await.unwrap();

        let result = message_stream.unsubscribe(&unsubscribe_message).await;
        server.verify().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_system_public_messages() {
        let test_cases = vec![
            (HEARTBEAT, PublicMessage::Heartbeat),
            (
                PING,
                PublicMessage::PingPong(PingPong {
                    event: "ping".into(),
                    req_id: 42,
                }),
            ),
            (
                PONG,
                PublicMessage::PingPong(PingPong {
                    event: "pong".into(),
                    req_id: 42,
                }),
            ),
            (
                SYSTEM_STATUS,
                PublicMessage::SystemStatus(SystemStatus {
                    connection_id: 7858587364768643506,
                    event: "systemStatus".into(),
                    status: Status::Online,
                    version: "1.9.1".to_string(),
                }),
            ),
        ];

        public_message_parsing_test(test_cases).await;
    }

    #[tokio::test]
    async fn test_market_data_public_messages() {
        let test_cases = vec![
            (
                TRADE,
                PublicMessage::Trade(Message {
                    channel_id: 337,
                    message: vec![PublicTrade {
                        price: "37080.10000".to_string(),
                        volume: "0.00015891".to_string(),
                        time: "1699797222.188887".to_string(),
                        side: BuySellChar::Sell,
                        order_type: MarketLimitChar::Market,
                        misc: "".to_string(),
                    }],
                    channel_name: "trade".to_string(),
                    pair: "XBT/USD".to_string(),
                }),
            ),
            (
                TICKER,
                PublicMessage::Ticker(Box::new(Message {
                    channel_id: 340,
                    message: WSSTickerInfo {
                        asks: TickerBidAsk {
                            price: dec!(37080.20000),
                            whole_lot_volume: 0,
                            lot_volume: dec!(0.49479977),
                        },
                        bids: TickerBidAsk {
                            price: dec!(37080.10000),
                            whole_lot_volume: 24,
                            lot_volume: dec!(24.49109974),
                        },
                        closed: LastTrade {
                            price: dec!(37080.10000),
                            volume: dec!(0.01268510),
                        },
                        volume: TickerDecimal {
                            today: dec!(537.03329406),
                            last_24_h: dec!(1394.36071246),
                        },
                        vwap: TickerDecimal {
                            today: dec!(37012.52371),
                            last_24_h: dec!(37042.48940),
                        },
                        trades: TickerTrades {
                            today: 8495,
                            last_24_h: 21019,
                        },
                        low: TickerDecimal {
                            today: dec!(36727.30000),
                            last_24_h: dec!(36658.00000),
                        },
                        high: TickerDecimal {
                            today: dec!(37185.60000),
                            last_24_h: dec!(37289.70000),
                        },
                        open: TickerDecimal {
                            today: dec!(37139.90000),
                            last_24_h: dec!(37160.10000),
                        },
                    },
                    channel_name: "ticker".to_string(),
                    pair: "XBT/USD".to_string(),
                })),
            ),
            (
                OHLC,
                PublicMessage::OHLC(Message {
                    channel_id: 343,
                    message: OHLC {
                        time: "1699797181.803577".to_string(),
                        end_time: "1699797240.000000".to_string(),
                        open: "37080.20000".to_string(),
                        high: "37080.20000".to_string(),
                        low: "37080.20000".to_string(),
                        close: "37080.20000".to_string(),
                        vwap: "37080.20000".to_string(),
                        volume: "0.01032369".to_string(),
                        count: 2,
                    },
                    channel_name: "ohlc-1".to_string(),
                    pair: "XBT/USD".to_string(),
                }),
            ),
            (
                SPREAD,
                PublicMessage::Spread(Message {
                    channel_id: 341,
                    message: Spread {
                        bid: "37080.10000".to_string(),
                        ask: "37080.20000".to_string(),
                        timestamp: "1699797184.943422".to_string(),
                        bid_volume: "21.82608437".to_string(),
                        ask_volume: "0.50775187".to_string(),
                    },
                    channel_name: "spread".to_string(),
                    pair: "XBT/USD".to_string(),
                }),
            ),
            (
                BOOK_SNAPSHOT,
                PublicMessage::Orderbook(Message {
                    channel_id: 336,
                    message: Orderbook {
                        asks: vec![
                            BidAsk {
                                price: "37080.20000".to_string(),
                                volume: "0.44907155".to_string(),
                                time: "1699797211.976902".to_string(),
                            },
                            BidAsk {
                                price: "37080.50000".to_string(),
                                volume: "0.01086516".to_string(),
                                time: "1699797210.264751".to_string(),
                            },
                            BidAsk {
                                price: "37096.10000".to_string(),
                                volume: "0.00100000".to_string(),
                                time: "1699797210.168531".to_string(),
                            },
                        ],
                        bids: vec![
                            BidAsk {
                                price: "37080.10000".to_string(),
                                volume: "24.49109974".to_string(),
                                time: "1699797200.242011".to_string(),
                            },
                            BidAsk {
                                price: "37079.90000".to_string(),
                                volume: "0.08764809".to_string(),
                                time: "1699797196.230889".to_string(),
                            },
                            BidAsk {
                                price: "37079.80000".to_string(),
                                volume: "0.02789714".to_string(),
                                time: "1699797179.654731".to_string(),
                            },
                        ],
                    },
                    channel_name: "book-10".to_string(),
                    pair: "XBT/USD".to_string(),
                }),
            ),
            (
                BOOK_BIDS_ONLY,
                PublicMessage::OrderbookUpdate(OrderbookUpdateMessage {
                    channel_id: 336,
                    bids: vec![
                        BidAskUpdate {
                            price: "37079.40000".to_string(),
                            volume: "0.36000000".to_string(),
                            timestamp: "1699797212.921034".to_string(),
                            update_type: None,
                        },
                        BidAskUpdate {
                            price: "37080.10000".to_string(),
                            volume: "24.89569974".to_string(),
                            timestamp: "1699797212.921050".to_string(),
                            update_type: None,
                        },
                    ],
                    asks: vec![],
                    channel_name: "book-10".to_string(),
                    pair: "XBT/USD".to_string(),
                    checksum: "2845854188".to_string(),
                }),
            ),
            (
                BOOK_ASKS_ONLY,
                PublicMessage::OrderbookUpdate(OrderbookUpdateMessage {
                    channel_id: 336,
                    bids: vec![],
                    asks: vec![
                        BidAskUpdate {
                            price: "37109.60000".to_string(),
                            volume: "0.00000000".to_string(),
                            timestamp: "1699797213.027747".to_string(),
                            update_type: None,
                        },
                        BidAskUpdate {
                            price: "37110.40000".to_string(),
                            volume: "2.69466902".to_string(),
                            timestamp: "1699797200.313276".to_string(),
                            update_type: Some("r".to_string()),
                        },
                    ],
                    channel_name: "book-10".to_string(),
                    pair: "XBT/USD".to_string(),
                    checksum: "1339898949".to_string(),
                }),
            ),
        ];

        public_message_parsing_test(test_cases).await;
    }

    async fn public_message_parsing_test(test_cases: Vec<(&str, PublicMessage)>) {
        let server = WsMockServer::start().await;
        let url = server.uri().await;

        let (mpsc_send, mpsc_recv) = mpsc::channel::<String>(32);

        WsMock::new()
            .forward_from_channel(mpsc_recv)
            .mount(&server)
            .await;

        let mut client = KrakenWSSClient::new_with_urls(url.as_str(), url.as_str());

        let mut message_stream = client.connect().await.unwrap();

        for (raw_message, expected_parsed_message) in test_cases {
            mpsc_send.send(raw_message.to_string()).await.unwrap();

            let message = timeout(Duration::from_millis(100), message_stream.next())
                .await
                .unwrap();

            assert!(message.is_some());
            assert_eq!(expected_parsed_message, message.unwrap().unwrap());
        }
    }
}
