//! All private messages for websockets
use crate::wss::kraken_wss_types::{ErrorMessage, PingPong, SystemStatus};
use crate::wss::parsing::{get_event_field, get_event_from_vec};
use crate::wss::private::open_orders_messages::OpenOrdersMessage;
use crate::wss::private::own_trades_messages::OwnTradeMessage;
use crate::wss::private::trading_messages::{
    AddOrderResponse, CancelAllAfterResponse, CancelAllResponse, CancelOrderResponse,
    EditOrderResponse,
};
use crate::wss::subscribe_messages::SubscriptionStatus;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

const EXPECTED_PRIVATE_MESSAGES: &[&str] = &[
    "heartbeat",
    "ping",
    "pong",
    "systemStatus",
    "subscriptionStatus",
    "addOrderStatus",
    "editOrderStatus",
    "cancelOrderStatus",
];

/// Contains all possible message types for a private websocket connection
#[derive(Debug)]
pub enum PrivateMessage {
    PingPong(PingPong),
    Heartbeat,
    SystemStatus(SystemStatus),
    SubscriptionStatus(SubscriptionStatus),
    OpenOrders(OpenOrdersMessage),
    OwnTrades(OwnTradeMessage),
    CancelOrderResponse(CancelOrderResponse),
    CancelAllResponse(CancelAllResponse),
    AddOrderResponse(AddOrderResponse),
    EditOrderResponse(EditOrderResponse),
    CancelAllAfterResponse(CancelAllAfterResponse),
    ErrorMessage(ErrorMessage),
}

impl<'de> Deserialize<'de> for PrivateMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json = Value::deserialize(deserializer)?;

        match &json {
            Value::Object(o) => match get_event_field(o) {
                Some("heartbeat") => Ok(PrivateMessage::Heartbeat),
                Some("ping") | Some("pong") => {
                    Ok(PrivateMessage::PingPong(PingPong::deserialize(json).or(
                        Err(Error::custom("Invalid message with event: ping | pong")),
                    )?))
                }
                Some("systemStatus") => Ok(PrivateMessage::SystemStatus(
                    SystemStatus::deserialize(json).or(Err(Error::custom(
                        "Invalid message with event: systemStatus",
                    )))?,
                )),
                Some("subscriptionStatus") => Ok(PrivateMessage::SubscriptionStatus(
                    SubscriptionStatus::deserialize(json).or(Err(Error::custom(
                        "Invalid message with event: subscriptionStatus",
                    )))?,
                )),
                Some("addOrderStatus") => Ok(PrivateMessage::AddOrderResponse(
                    AddOrderResponse::deserialize(json).or(Err(Error::custom(
                        "Invalid message with event: addOrderStatus",
                    )))?,
                )),
                Some("editOrderStatus") => Ok(PrivateMessage::EditOrderResponse(
                    EditOrderResponse::deserialize(json).or(Err(Error::custom(
                        "Invalid message with event: editOrderStatus",
                    )))?,
                )),
                Some("cancelOrderStatus") => Ok(PrivateMessage::CancelOrderResponse(
                    CancelOrderResponse::deserialize(json).or(Err(Error::custom(
                        "Invalid message with event: cancelOrderStatus",
                    )))?,
                )),
                Some(field) => Err(Error::unknown_variant(field, EXPECTED_PRIVATE_MESSAGES)),
                None => Err(Error::missing_field("event")),
            },
            Value::Array(v) => match get_event_from_vec(v) {
                Some("ownTrades") => Ok(PrivateMessage::OwnTrades(
                    OwnTradeMessage::deserialize(json)
                        .or(Err(Error::custom("Invalid message with event: ownTrades")))?,
                )),
                Some("openOrders") => Ok(PrivateMessage::OpenOrders(
                    OpenOrdersMessage::deserialize(json)
                        .or(Err(Error::custom("Invalid message with event: openOrders")))?,
                )),
                Some(field) => Err(Error::unknown_variant(field, &["ownTrades", "openOrders"])),
                None => Err(Error::custom(
                    "event was not second-to-last field of JSON array",
                )),
            },
            _ => Err(Error::custom(
                "Unexpected JSON value that was not an array or object",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NON_ARRAY_OR_OBJECT_EVENT: &str = r#""channel-message""#;
    const OBJECT_INVALID_EVENT: &str = r#"{"event":"emergency", "reqid": 42}"#;
    const OBJECT_MISSING_EVENT: &str = r#"{"reqid": 42}"#;
    const ARRAY_INVALID_EVENT: &str = r#"[341,[],"incorrect-event","XBT/USD"]"#;
    const ARRAY_MISSING_EVENT: &str = r#"[341,[],42,"XBT/USD"]"#;
    const ARRAY_INVALID_LENGTH_FOR_EVENT: &str = r#"["channel-message"]"#;

    const HEARTBEAT: &str = r#"{"event":"heartbeat"}"#;
    const PING: &str = r#"{"event":"ping", "reqid": 42}"#;
    const PONG: &str = r#"{"event":"pong", "reqid": 42}"#;
    const SYSTEM_STATUS: &str = r#"{"connectionID":8917087679770238719,"event":"systemStatus","status":"online","version":"1.9.1"}"#;
    const SUBSCRIBE_OPEN_ORDERS: &str = r#"{"channelName":"openOrders","event":"subscriptionStatus","reqid":0,"status":"subscribed","subscription":{"maxratecount":125,"name":"openOrders"}}"#;
    const OPEN_ORDERS_INITIAL: &str = r#"[[{"OKJASZ-554YM-L7YMU7":{"avg_price":"0.000000","cost":"0.000000","descr":{"close":null,"leverage":null,"order":"buy 5.00000000 ATOM/USD @ limit 9.250000","ordertype":"limit","pair":"ATOM/USD","price":"9.250000","price2":"0.000000","type":"buy"},"expiretm":null,"fee":"0.000000","limitprice":"0.000000","misc":"","oflags":"fciq","opentm":"1700076648.883709","refid":null,"starttm":null,"status":"open","stopprice":"0.000000","timeinforce":"GTC","userref":0,"vol":"5.00000000","vol_exec":"0.00000000"}},{"OYEXS2-3M26S-PYLEVF":{"avg_price":"0.00000000","cost":"0.00000000","descr":{"close":null,"leverage":null,"order":"sell 1.00000000 ATOM/ETH @ limit 0.00877000","ordertype":"limit","pair":"ATOM/ETH","price":"0.00877000","price2":"0.00000000","type":"sell"},"expiretm":null,"fee":"0.00000000","limitprice":"0.00000000","misc":"","oflags":"fciq","opentm":"1676384710.121142","refid":null,"starttm":null,"status":"open","stopprice":"0.00000000","timeinforce":"GTC","userref":0,"vol":"1.00000000","vol_exec":"0.00000000"}},{"OVZYZV-IJH2T-QPSLPF":{"avg_price":"0.00000000","cost":"0.00000000","descr":{"close":null,"leverage":null,"order":"sell 0.99000000 ATOM/ETH @ limit 0.00883500","ordertype":"limit","pair":"ATOM/ETH","price":"0.00883500","price2":"0.00000000","type":"sell"},"expiretm":null,"fee":"0.00000000","limitprice":"0.00000000","misc":"","oflags":"fciq","opentm":"1676384605.111791","refid":null,"starttm":null,"status":"open","stopprice":"0.00000000","timeinforce":"GTC","userref":0,"vol":"0.99000000","vol_exec":"0.00000000"}}],"openOrders",{"sequence":1}]"#;
    const OPEN_ORDERS_NEW_ORDER: &str = r#"[[{"O7AIWV-HEBBH-COCEUU":{"avg_price":"0.00000","cost":"0.00000","descr":{"close":null,"leverage":null,"order":"buy 1.00000000 SOL/USD @ limit 25.00000","ordertype":"limit","pair":"SOL/USD","price":"25.00000","price2":"0.00000","type":"buy"},"expiretm":null,"fee":"0.00000","limitprice":"0.00000","misc":"","oflags":"fciq","opentm":"1700220661.985020","refid":null,"starttm":null,"status":"pending","stopprice":"0.00000","timeinforce":"GTC","userref":0,"vol":"1.00000000","vol_exec":"0.00000000"}}],"openOrders",{"sequence":2}]"#;
    const OPEN_ORDERS_OPEN: &str =
        r#"[[{"O7AIWV-HEBBH-COCEUU":{"status":"open","userref":0}}],"openOrders",{"sequence":3}]"#;
    const OPEN_ORDERS_USER_REQUESTED_CANCEL: &str = r#"[[{"O7AIWV-HEBBH-COCEUU":{"lastupdated":"1700220675.012776","status":"canceled","vol_exec":"0.00000000","cost":"0.00000","fee":"0.00000","avg_price":"0.00000","userref":0,"cancel_reason":"User requested"}}],"openOrders",{"sequence":4}]"#;
    const OPEN_ORDERS_TRADE_EXEC: &str = r#"[[{"OX6J4U-3FWTH-NPST2W":{"vol_exec":"0.01000000","cost":"19.67680","fee":"0.03148","avg_price":"1967.68000","userref":0}}],"openOrders",{"sequence":10}]"#;
    const SUBSCRIBE_OWN_TRADES: &str = r#"{"channelName":"ownTrades","event":"subscriptionStatus","reqid":0,"status":"subscribed","subscription":{"name":"ownTrades"}}"#;
    const OPEN_ORDER_EXPIRED: &str = r#"[[{"O7WZ5U-XB4S4-KJSIL5":{"lastupdated":"1700309901.358598","status":"expired","vol_exec":"0.00000000","cost":"0.00000000","fee":"0.00000000","avg_price":"0.00000000","userref":0}}],"openOrders",{"sequence":4}]"#;
    const OWN_TRADES_INITIAL: &str = r#"[[{"T5VKWY-X54ZN-5M7TUQ":{"cost":"45.000000","fee":"0.072000","margin":"0.000000","ordertxid":"OEEGFU-I4ZZT-TUTBCS","ordertype":"limit","pair":"ATOM/USD","postxid":"TKH2SE-M7IF5-CFI7LT","price":"9.000000","time":"1699987398.892228","type":"buy","vol":"5.00000000"}},{"TAUGAY-Z4ZJO-O3POKN":{"cost":"90.000000","fee":"0.144000","margin":"0.000000","ordertxid":"O4ROEJ-XHGST-4L77XX","ordertype":"limit","pair":"ATOM/USD","postxid":"TKH2SE-M7IF5-CFI7LT","price":"9.000000","time":"1699961059.363660","type":"buy","vol":"10.00000000"}}],"ownTrades",{"sequence":1}]"#;
    const OWN_TRADES_EXEC: &str = r#"[[{"TROWH4-DD6XR-2O7DPH":{"cost":"19.67680","fee":"0.03148","margin":"0.00000","ordertxid":"OX6J4U-3FWTH-NPST2W","ordertype":"limit","pair":"ETH/USD","postxid":"TKH2SE-M7IF5-CFI7LT","price":"1967.68000","time":"1700220761.062896","type":"buy","vol":"0.01000000"}}],"ownTrades",{"sequence":2}]"#;

    const ADD_ORDER_RESPONSE: &str = r#"{"descr":"buy 10.00000000 USDCUSD @ limit 0.9000","event":"addOrderStatus","status":"ok","txid":"OA7JUX-OKLO3-M6IEVL"}"#;
    const ADD_ORDER_INVALID_ARGS: &str = r#"{"errorMessage":"EGeneral:Invalid arguments:timeinforce","event":"addOrderStatus","status":"error"}"#;
    const EDIT_ORDER_RESPONSE: &str = r#"{"descr":"buy 20.00000000 USDCUSD @ limit 0.9","event":"editOrderStatus","originaltxid":"OA7JUX-OKLO3-M6IEVL","status":"ok","txid":"O7NE5Y-QUARV-HHHUCA"}"#;
    const EDIT_ORDER_RATE_LIMIT_EXCEEDED: &str = r#"{"errorMessage":"EOrder:Rate limit exceeded","event":"editOrderStatus","status":"error"}"#;
    const CANCEL_ORDER_SUCCESS: &str = r#"{"event":"cancelOrderStatus","status":"ok"}"#;
    const CANCEL_ORDER_ERROR: &str =
        r#"{"errorMessage":"EOrder:Unknown order","event":"cancelOrderStatus","status":"error"}"#;
    const CANCEL_ORDER_REQ_ID: &str = r#"{"event":"cancelOrderStatus","reqid":1234,"status":"ok"}"#;

    #[test]
    fn test_deserializing_invalid_message() {
        test_deserializing_produces_error(
            NON_ARRAY_OR_OBJECT_EVENT,
            "Unexpected JSON value that was not an array or object",
        );
    }

    #[test]
    fn test_deserializing_object_with_invalid_event() {
        test_deserializing_produces_error(
            OBJECT_INVALID_EVENT,
            "unknown variant `emergency`, expected one of `heartbeat`, `ping`, `pong`, `systemStatus`, `subscriptionStatus`, `addOrderStatus`, `editOrderStatus`, `cancelOrderStatus`"
        );
    }

    #[test]
    fn test_deserializing_object_with_missing_event() {
        test_deserializing_produces_error(OBJECT_MISSING_EVENT, "missing field `event`");
    }

    #[test]
    fn test_deserializing_array_with_invalid_event() {
        test_deserializing_produces_error(
            ARRAY_INVALID_EVENT,
            "unknown variant `incorrect-event`, expected `ownTrades` or `openOrders`",
        );
    }

    #[test]
    fn test_deserializing_array_with_missing_event() {
        test_deserializing_produces_error(
            ARRAY_MISSING_EVENT,
            "event was not second-to-last field of JSON array",
        );
    }

    #[test]
    fn test_deserializing_array_invalid_length() {
        test_deserializing_produces_error(
            ARRAY_INVALID_LENGTH_FOR_EVENT,
            "event was not second-to-last field of JSON array",
        );
    }

    fn test_deserializing_produces_error(input: &str, expected_error: &str) {
        let message: Result<PrivateMessage, serde_json::Error> = serde_json::from_str(input);

        assert!(message.is_err());
        let err = message.unwrap_err();
        assert_eq!(expected_error, err.to_string())
    }

    #[test]
    fn test_deserializing_ping_pong() {
        let ping: PrivateMessage = serde_json::from_str(PING).unwrap();
        let pong: PrivateMessage = serde_json::from_str(PONG).unwrap();

        assert!(matches!(ping, PrivateMessage::PingPong { .. }));
        assert!(matches!(pong, PrivateMessage::PingPong { .. }));
    }

    #[test]
    fn test_deserializing_heartbeat() {
        let heartbeat: PrivateMessage = serde_json::from_str(HEARTBEAT).unwrap();

        assert!(matches!(heartbeat, PrivateMessage::Heartbeat));
    }

    #[test]
    fn test_deserializing_system_status() {
        let system_status: PrivateMessage = serde_json::from_str(SYSTEM_STATUS).unwrap();

        assert!(matches!(system_status, PrivateMessage::SystemStatus { .. }));
    }

    #[test]
    fn test_deserializing_subscriptions() {
        let subscriptions = [SUBSCRIBE_OPEN_ORDERS, SUBSCRIBE_OWN_TRADES];

        for subscription in subscriptions {
            let subscribe: PrivateMessage = serde_json::from_str(subscription).unwrap();
            assert!(matches!(
                subscribe,
                PrivateMessage::SubscriptionStatus { .. }
            ));
        }
    }

    #[test]
    fn test_deserializing_open_orders_initial() {
        let open_orders: PrivateMessage = serde_json::from_str(OPEN_ORDERS_INITIAL).unwrap();

        assert!(matches!(open_orders, PrivateMessage::OpenOrders { .. }));
    }

    #[test]
    fn test_deserializing_open_orders_new_order() {
        let open_orders: PrivateMessage = serde_json::from_str(OPEN_ORDERS_NEW_ORDER).unwrap();

        assert!(matches!(open_orders, PrivateMessage::OpenOrders { .. }));
    }

    #[test]
    fn test_deserializing_open_orders_open_order() {
        let open_orders: PrivateMessage = serde_json::from_str(OPEN_ORDERS_OPEN).unwrap();

        assert!(matches!(open_orders, PrivateMessage::OpenOrders { .. }));
    }

    #[test]
    fn test_deserializing_open_orders_exec_trade() {
        let open_orders: PrivateMessage = serde_json::from_str(OPEN_ORDERS_TRADE_EXEC).unwrap();

        assert!(matches!(open_orders, PrivateMessage::OpenOrders { .. }));
    }

    #[test]
    fn test_deserializing_open_orders_expired() {
        let open_orders: PrivateMessage = serde_json::from_str(OPEN_ORDER_EXPIRED).unwrap();

        assert!(matches!(open_orders, PrivateMessage::OpenOrders { .. }));
    }

    #[test]
    fn test_deserializing_own_trades_initial() {
        let own_trades_initial: PrivateMessage = serde_json::from_str(OWN_TRADES_INITIAL).unwrap();

        assert!(matches!(
            own_trades_initial,
            PrivateMessage::OwnTrades { .. }
        ));
    }

    #[test]
    fn test_deserializing_own_trades_exec() {
        let own_trades_exec: PrivateMessage = serde_json::from_str(OWN_TRADES_EXEC).unwrap();

        assert!(matches!(own_trades_exec, PrivateMessage::OwnTrades { .. }));
    }

    #[test]
    fn test_deserializing_add_order_response() {
        let add_order_response: PrivateMessage = serde_json::from_str(ADD_ORDER_RESPONSE).unwrap();

        assert!(matches!(
            add_order_response,
            PrivateMessage::AddOrderResponse { .. }
        ));
    }

    #[test]
    fn test_deserializing_add_order_invalid_args() {
        let add_order_response: PrivateMessage =
            serde_json::from_str(ADD_ORDER_INVALID_ARGS).unwrap();

        assert!(matches!(
            add_order_response,
            PrivateMessage::AddOrderResponse { .. }
        ));
    }

    #[test]
    fn test_deserializing_edit_order_response() {
        let edit_order_response: PrivateMessage =
            serde_json::from_str(EDIT_ORDER_RESPONSE).unwrap();

        assert!(matches!(
            edit_order_response,
            PrivateMessage::EditOrderResponse { .. }
        ));
    }

    #[test]
    fn test_deserializing_edit_order_error() {
        let edit_order_response: PrivateMessage =
            serde_json::from_str(EDIT_ORDER_RATE_LIMIT_EXCEEDED).unwrap();

        assert!(matches!(
            edit_order_response,
            PrivateMessage::EditOrderResponse { .. }
        ));
    }

    #[test]
    fn test_deserializing_cancel_order_response() {
        let cancel_order_response: PrivateMessage =
            serde_json::from_str(CANCEL_ORDER_SUCCESS).unwrap();

        assert!(matches!(
            cancel_order_response,
            PrivateMessage::CancelOrderResponse { .. }
        ));
    }

    #[test]
    fn test_deserializing_cancel_order_with_req_id() {
        let cancel_order_response: PrivateMessage =
            serde_json::from_str(CANCEL_ORDER_REQ_ID).unwrap();

        assert!(matches!(
            cancel_order_response,
            PrivateMessage::CancelOrderResponse { .. }
        ));
    }

    #[test]
    fn test_deserializing_cancel_order_error() {
        let cancel_order_response: PrivateMessage =
            serde_json::from_str(CANCEL_ORDER_ERROR).unwrap();

        assert!(matches!(
            cancel_order_response,
            PrivateMessage::CancelOrderResponse { .. }
        ));
    }
}
