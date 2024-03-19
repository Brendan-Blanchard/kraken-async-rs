mod resources;

#[cfg(test)]
mod tests {
    use crate::resources::test_serde::test_deserializing_expecting_error;
    use crate::resources::wss::responses::*;
    use kraken_async_rs::wss::private::open_orders_messages::{OpenOrder, OrderStatusChange};
    use kraken_async_rs::wss::private::own_trades_messages::OwnTrade;
    use kraken_async_rs::wss::public::messages::PublicMessage;
    use kraken_async_rs::wss::public::orderbooks::OrderbookUpdateMessage;

    #[test]
    fn test_deserializing_invalid_message() {
        test_deserializing_public_message_produces_error(
            NON_ARRAY_OR_OBJECT_EVENT,
            "Unexpected JSON value that was not an array or object",
        );
    }

    #[test]
    fn test_deserializing_object_with_invalid_event() {
        test_deserializing_public_message_produces_error(
            OBJECT_INVALID_EVENT,
            "unknown variant `emergency`, expected one of `heartbeat`, `ping`, `pong`, `systemStatus`, `subscriptionStatus`"
        );
    }

    #[test]
    fn test_deserializing_object_with_missing_event() {
        test_deserializing_public_message_produces_error(
            OBJECT_MISSING_EVENT,
            "missing field `event`",
        );
    }

    #[test]
    fn test_deserializing_array_with_invalid_event() {
        test_deserializing_public_message_produces_error(
            ARRAY_INVALID_EVENT,
            "unknown variant `incorrect-event`, expected one of `trade`, `spread`, `ticker`, `ohlc`, `book`",
        );
    }

    #[test]
    fn test_deserializing_array_with_missing_event() {
        test_deserializing_public_message_produces_error(
            ARRAY_MISSING_EVENT,
            "event was not second-to-last field of JSON array",
        );
    }

    #[test]
    fn test_deserializing_array_invalid_length() {
        test_deserializing_public_message_produces_error(
            ARRAY_INVALID_LENGTH_FOR_EVENT,
            "event was not second-to-last field of JSON array",
        );
    }

    #[test]
    fn test_deserializing_invalid_open_order() {
        let expected_error = "invalid length 0, expected OpenOrder at line 1 column 2";

        test_deserializing_expecting_error::<OpenOrder>("{}", expected_error);
    }

    #[test]
    fn test_deserializing_invalid_order_status_change() {
        let expected_error = "invalid length 0, expected OrderStatusChange at line 1 column 2";

        test_deserializing_expecting_error::<OrderStatusChange>("{}", expected_error);
    }

    #[test]
    fn test_deserializing_invalid_own_trade() {
        let expected_error = "invalid length 0, expected OwnTrade at line 1 column 2";

        test_deserializing_expecting_error::<OwnTrade>("{}", expected_error);
    }

    #[test]
    fn test_deserializing_invalid_orderbook_update_array() {
        let expected_error = "invalid length 2, expected OrderbookUpdateMessage at line 1 column 8";

        test_deserializing_expecting_error::<OrderbookUpdateMessage>(r#"[32, ""]"#, expected_error);
    }

    #[test]
    fn test_deserializing_invalid_orderbook_update_subtype() {
        let expected_error =
            "data did not match any variant of untagged enum MessageField at line 1 column 8";

        test_deserializing_expecting_error::<OrderbookUpdateMessage>(r#"[32, {}]"#, expected_error);
    }

    #[test]
    fn test_deserializing_orderbook_snapshot_err() {
        let expected_error = "Invalid message with event: book (snapshot)";

        test_deserializing_expecting_error::<PublicMessage>(
            BOOK_SNAPSHOT_MISSING_FIELD,
            expected_error,
        );
    }

    #[test]
    fn test_deserializing_orderbook_update_err() {
        let expected_error = "Invalid message with event: book (update)";

        test_deserializing_expecting_error::<PublicMessage>(
            BOOK_BIDS_ONLY_MISSING_FIELD,
            expected_error,
        );
    }

    #[test]
    fn test_deserializing_invalid_channel_name() {
        let expected_error = "unknown variant `airdrop`, expected one of `heartbeat`, `ping`, `pong`, `systemStatus`, `subscriptionStatus`";

        test_deserializing_expecting_error::<PublicMessage>(INVALID_CHANNEL_NAME, expected_error);
    }

    #[test]
    fn test_deserializing_invalid_system_status() {
        let expected_error = "Invalid message with event: systemStatus";

        test_deserializing_expecting_error::<PublicMessage>(INVALID_SYSTEM_STATUS, expected_error);
    }

    #[test]
    fn test_deserializing_invalid_subscription_status() {
        let expected_error = "Invalid message with event: subscriptionStatus";

        test_deserializing_expecting_error::<PublicMessage>(INVALID_UNSUBSCRIBE, expected_error);
    }

    #[test]
    fn test_deserializing_ping_pong() {
        let ping: PublicMessage = serde_json::from_str(PING).unwrap();
        let pong: PublicMessage = serde_json::from_str(PONG).unwrap();

        assert!(matches!(ping, PublicMessage::PingPong { .. }));
        assert!(matches!(pong, PublicMessage::PingPong { .. }));
    }

    #[test]
    fn test_deserializing_heartbeat() {
        let heartbeat: PublicMessage = serde_json::from_str(HEARTBEAT).unwrap();

        assert!(matches!(heartbeat, PublicMessage::Heartbeat));
    }

    #[test]
    fn test_deserializing_system_status() {
        let system_status: PublicMessage = serde_json::from_str(SYSTEM_STATUS).unwrap();

        assert!(matches!(system_status, PublicMessage::SystemStatus { .. }));
    }

    #[test]
    fn test_deserializing_subscriptions() {
        let subscriptions = [
            SUBSCRIBE_SPREAD,
            UNSUBSCRIBE_SPREAD,
            SUBSCRIBE_OHLC,
            UNSUBSCRIBE_OHLC,
            SUBSCRIBE_TICKER,
            TICKER_UNSUBSCRIBE,
            SUBSCRIBE_TRADE,
            UNSUBSCRIBE_TRADE,
            SUBSCRIBE_BOOK,
            UNSUBSCRIBE_BOOK,
        ];

        for subscription in subscriptions {
            let subscribe: PublicMessage = serde_json::from_str(subscription).unwrap();
            assert!(matches!(
                subscribe,
                PublicMessage::SubscriptionStatus { .. }
            ));
        }
    }

    #[test]
    fn test_deserializing_spread() {
        let spread: PublicMessage = serde_json::from_str(SPREAD).unwrap();

        assert!(matches!(spread, PublicMessage::Spread { .. }));
    }

    #[test]
    fn test_deserializing_ohlc() {
        let ohlc: PublicMessage = serde_json::from_str(OHLC).unwrap();

        assert!(matches!(ohlc, PublicMessage::OHLC { .. }));
    }

    #[test]
    fn test_deserializing_ticker() {
        let ticker: PublicMessage = serde_json::from_str(TICKER).unwrap();

        assert!(matches!(ticker, PublicMessage::Ticker { .. }));
    }

    #[test]
    fn test_deserializing_trade() {
        let trade: PublicMessage = serde_json::from_str(TRADE).unwrap();

        assert!(matches!(trade, PublicMessage::Trade { .. }));
    }

    #[test]
    fn test_deserializing_orderbook_snapshot() {
        let book: PublicMessage = serde_json::from_str(BOOK_SNAPSHOT).unwrap();

        assert!(matches!(book, PublicMessage::Orderbook { .. }));
    }

    #[test]
    fn test_deserializing_orderbook_update() {
        let asks: PublicMessage = serde_json::from_str(BOOK_ASKS_ONLY).unwrap();
        let bids: PublicMessage = serde_json::from_str(BOOK_BIDS_ONLY).unwrap();

        assert!(matches!(asks, PublicMessage::OrderbookUpdate { .. }));
        assert!(matches!(bids, PublicMessage::OrderbookUpdate { .. }));
    }

    fn test_deserializing_public_message_produces_error(input: &str, expected_error: &str) {
        let message: Result<PublicMessage, serde_json::Error> = serde_json::from_str(input);

        assert!(message.is_err());
        let err = message.unwrap_err();
        assert_eq!(expected_error, err.to_string())
    }
}
