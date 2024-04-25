//! OpenOrder message and sub-types
use crate::request_types::TimeInForce;
use crate::response_types::{BuySell, OrderStatus, OrderType};
use crate::wss::kraken_wss_types::Sequence;
use serde::de::{MapAccess, Visitor};
use serde::{de, Deserialize, Deserializer};
use serde_tuple::Deserialize_tuple;
use serde_with::formats::Strict;
use serde_with::TimestampSecondsWithFrac;
use std::fmt::Formatter;
use time::OffsetDateTime;

const OPEN_ORDER_FIELDS: &[&str] = &[
    "order_id",
    "ref_id",
    "user_ref",
    "status",
    "open_time",
    "start_time",
    "display_volume",
    "display_volume_remain",
    "expire_time",
    "contingent",
    "order_description",
    "last_updated",
    "volume",
    "executed_volume",
    "cost",
    "fee",
    "average_price",
    "stop_price",
    "limit_price",
    "misc",
    "order_flags",
    "time_in_force",
    "cancel_reason",
    "rate_count",
];

/// Message containing a `Vec<OpenOrder>` of all open orders (or updates to them)
#[derive(Debug, Deserialize_tuple)]
pub struct OpenOrdersMessage {
    pub open_orders: Vec<OpenOrder>,
    #[serde(rename = "channelName")]
    pub channel_name: String,
    pub sequence: Sequence,
}

/// Type to deserialize to, missing the order_id field (Kraken API design)
#[derive(Debug, Deserialize, PartialEq)]
struct RawOpenOrder {
    #[serde(rename(deserialize = "refid"))]
    ref_id: Option<String>,
    #[serde(rename(deserialize = "userref"))]
    user_ref: Option<i64>,
    status: Option<OrderStatus>,
    #[serde(rename(deserialize = "opentm"))]
    open_time: Option<String>,
    #[serde(rename(deserialize = "starttm"))]
    start_time: Option<String>,
    display_volume: Option<String>,
    display_volume_remain: Option<String>,
    #[serde(rename(deserialize = "expiretm"))]
    expire_time: Option<String>,
    contingent: Option<OrderContingent>,
    #[serde(rename(deserialize = "descr"))]
    order_description: Option<OrderDescription>,
    #[serde(rename(deserialize = "lastupdated"))]
    last_updated: Option<String>,
    #[serde(rename(deserialize = "vol"))]
    volume: Option<String>,
    #[serde(rename(deserialize = "vol_exec"))]
    executed_volume: Option<String>,
    cost: Option<String>,
    fee: Option<String>,
    #[serde(rename(deserialize = "avg_price"))]
    average_price: Option<String>,
    #[serde(rename(deserialize = "stopprice"))]
    stop_price: Option<String>,
    #[serde(rename(deserialize = "limitprice"))]
    limit_price: Option<String>,
    misc: Option<String>,
    #[serde(rename(deserialize = "oflags"))]
    order_flags: Option<String>,
    #[serde(rename(deserialize = "timeinforce"))]
    time_in_force: Option<TimeInForce>,
    cancel_reason: Option<String>,
    #[serde(rename(deserialize = "ratecount"))]
    rate_count: Option<String>,
}

impl RawOpenOrder {
    pub fn into_open_order(self, order_id: String) -> OpenOrder {
        OpenOrder {
            order_id,
            ref_id: self.ref_id,
            user_ref: self.user_ref,
            status: self.status,
            open_time: self.open_time,
            start_time: self.start_time,
            display_volume: self.display_volume,
            display_volume_remain: self.display_volume_remain,
            expire_time: self.expire_time,
            contingent: self.contingent,
            order_description: self.order_description,
            last_updated: self.last_updated,
            volume: self.volume,
            executed_volume: self.executed_volume,
            cost: self.cost,
            fee: self.fee,
            average_price: self.average_price,
            stop_price: self.stop_price,
            limit_price: self.limit_price,
            misc: self.misc,
            order_flags: self.order_flags,
            time_in_force: self.time_in_force,
            cancel_reason: self.cancel_reason,
            rate_count: self.rate_count,
        }
    }
}

/// OpenOrder type containing the order's id
#[derive(Debug, PartialEq)]
pub struct OpenOrder {
    pub order_id: String,
    pub ref_id: Option<String>,
    pub user_ref: Option<i64>,
    pub status: Option<OrderStatus>,
    pub open_time: Option<String>,
    pub start_time: Option<String>,
    pub display_volume: Option<String>,
    pub display_volume_remain: Option<String>,
    pub expire_time: Option<String>,
    pub contingent: Option<OrderContingent>,
    pub order_description: Option<OrderDescription>,
    pub last_updated: Option<String>,
    pub volume: Option<String>,
    pub executed_volume: Option<String>,
    pub cost: Option<String>,
    pub fee: Option<String>,
    pub average_price: Option<String>,
    pub stop_price: Option<String>,
    pub limit_price: Option<String>,
    pub misc: Option<String>,
    pub order_flags: Option<String>,
    pub time_in_force: Option<TimeInForce>,
    pub cancel_reason: Option<String>,
    pub rate_count: Option<String>,
}

impl<'de> Deserialize<'de> for OpenOrder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OpenOrderVisitor;

        impl<'de> Visitor<'de> for OpenOrderVisitor {
            type Value = OpenOrder;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("OpenOrder")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                if let Some((trade_id, raw_order)) = map.next_entry::<String, RawOpenOrder>()? {
                    Ok(raw_order.into_open_order(trade_id))
                } else {
                    Err(de::Error::invalid_length(0, &self))
                }
            }
        }

        deserializer.deserialize_struct("OpenOrder", OPEN_ORDER_FIELDS, OpenOrderVisitor)
    }
}

/// Contingent leg of an order, e.g. a stop-limit or take-profit
#[derive(Debug, Deserialize, PartialEq)]
pub struct OrderContingent {
    #[serde(rename(deserialize = "ordertype"))]
    pub order_type: OrderType,
    pub price: String,
    #[serde(rename(deserialize = "price2"))]
    pub price_2: String,
    #[serde(rename(deserialize = "oflags"))]
    pub order_flags: Option<String>,
}

/// Details of an individual order
#[derive(Debug, Deserialize, PartialEq)]
pub struct OrderDescription {
    pub pair: String,
    pub position: Option<String>,
    #[serde(rename(deserialize = "type"))]
    pub side: BuySell,
    #[serde(rename(deserialize = "ordertype"))]
    pub order_type: OrderType,
    pub price: String,
    #[serde(rename(deserialize = "price2"))]
    pub price_2: Option<String>,
    pub leverage: Option<String>,
    pub order: String,
    pub close: Option<String>,
}

/// Message for a status change of an order
#[derive(Debug, Deserialize_tuple, PartialEq)]
pub struct OrderStatusMessage {
    pub status_changes: Vec<OrderStatusChange>,
    pub event: String,
    pub sequence: Sequence,
}

/// Order status change to deserialize to, missing order_id due to Kraken API design
#[serde_with::serde_as]
#[derive(Debug, Deserialize, PartialEq)]
struct RawOpenOrderStatusChange {
    status: String,
    #[serde(rename = "userref")]
    user_ref: Option<i64>,
    #[serde(rename = "lastupdated")]
    #[serde_as(as = "Option<TimestampSecondsWithFrac<String, Strict>>")]
    last_updated: Option<OffsetDateTime>,
    #[serde(rename = "vol_exec")]
    volume_executed: Option<String>,
    cost: Option<String>,
    fee: Option<String>,
    #[serde(rename = "avg_price")]
    average_price: Option<String>,
    cancel_reason: Option<String>,
}

impl RawOpenOrderStatusChange {
    fn into_open_order_status_change(self, order_id: String) -> OrderStatusChange {
        OrderStatusChange {
            order_id,
            status: self.status,
            user_ref: self.user_ref,
            last_updated: self.last_updated,
            volume_executed: self.volume_executed,
            cost: self.cost,
            fee: self.fee,
            average_price: self.average_price,
            cancel_reason: self.cancel_reason,
        }
    }
}

/// Order status change containing the order's id and changes to status, fees, or volume executed
#[derive(Debug, PartialEq)]
pub struct OrderStatusChange {
    pub order_id: String,
    pub status: String,
    pub user_ref: Option<i64>,
    pub last_updated: Option<OffsetDateTime>,
    pub volume_executed: Option<String>,
    pub cost: Option<String>,
    pub fee: Option<String>,
    pub average_price: Option<String>,
    pub cancel_reason: Option<String>,
}

impl<'de> Deserialize<'de> for OrderStatusChange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OrderStatusChangeVisitor;

        impl<'de> Visitor<'de> for OrderStatusChangeVisitor {
            type Value = OrderStatusChange;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("OrderStatusChange")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                if let Some((order_id, raw_status)) =
                    map.next_entry::<String, RawOpenOrderStatusChange>()?
                {
                    Ok(raw_status.into_open_order_status_change(order_id))
                } else {
                    Err(de::Error::invalid_length(0, &self))
                }
            }
        }

        const ORDER_STATUS_CHANGE_FIELDS: &[&str] = &["order_id", "status", "user_ref"];

        deserializer.deserialize_struct(
            "OpenOrderStatusChange",
            ORDER_STATUS_CHANGE_FIELDS,
            OrderStatusChangeVisitor,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    const OPEN_ORDER: &str = "{\
    \"OSJN7R-5G4OK-T2KYTD\":{\"avg_price\":\"0.00000000\",\"cost\":\"0.00000000\",\"descr\":\
    {\"close\":null,\"leverage\":null,\"order\":\"sell 106.53408600 USDC/USD @ limit 1.00150000\",\
    \"ordertype\":\"limit\",\"pair\":\"USDC/USD\",\"price\":\"1.00150000\",\"price2\":\"0.00000000\",\
    \"type\":\"sell\"},\"expiretm\":null,\"fee\":\"0.00000000\",\"limitprice\":\"0.00000000\",\
    \"misc\":\"\",\"oflags\":\"fcib,post\",\"opentm\":\"1697801466.817843\",\"refid\":null,\
    \"starttm\":null,\"status\":\"open\",\"stopprice\":\"0.00000000\",\"timeinforce\":\"GTC\",\
    \"userref\":0,\"vol\":\"106.53408600\",\"vol_exec\":\"0.00000000\"}}";

    const OPEN_CONTINGENT_ORDER: &str = "{\
    \"OBA7Z7-XQVOQ-3NZRDS\":{\"avg_price\":\"0.00000000\",\"cost\":\"0.00000000\",\"descr\":\
    {\"close\":null,\"leverage\":null,\"order\":\
    \"buy 5.00000000 USDC/USD @ take-profit-limit 0.99000000, limit 0.99100000\",\"ordertype\":\
    \"take-profit-limit\",\"pair\":\"USDC/USD\",\"price\":\"0.99000000\",\"price2\":\"0.99100000\",\
    \"type\":\"buy\"},\"expiretm\":null,\"fee\":\"0.00000000\",\"limitprice\":\"0.00000000\",\
    \"misc\":\"\",\"oflags\":\"fciq\",\"opentm\":\"1698761179.336974\",\"refid\":null,\
    \"starttm\":null,\"status\":\"pending\",\"stopprice\":\"0.00000000\",\"timeinforce\":\"GTC\",\
    \"trigger\":\"index\",\"userref\":0,\"vol\":\"5.00000000\",\"vol_exec\":\"0.00000000\"}}";

    const STATUS_CHANGE: &str = r#"{"OBA7Z7-XQVOQ-3NZRDS":{"status":"open","userref":0}}"#;

    const STATUS_CHANGE_USER_CANCEL: &str = "{\"OBA7Z7-XQVOQ-3NZRDS\":{\"lastupdated\":\
    \"1698762486.391070\",\"status\":\"canceled\",\"vol_exec\":\"0.00000000\",\"cost\":\
    \"0.00000000\",\"fee\":\"0.00000000\",\"avg_price\":\"0.00000000\",\"userref\":0,\
    \"cancel_reason\":\"User requested\"}}";

    #[test]
    fn test_deserialize_open_order() {
        let expected_open_order = OpenOrder {
            order_id: "OSJN7R-5G4OK-T2KYTD".to_string(),
            ref_id: None,
            user_ref: Some(0),
            status: Some(OrderStatus::Open),
            open_time: Some("1697801466.817843".to_string()),
            start_time: None,
            display_volume: None,
            display_volume_remain: None,
            expire_time: None,
            contingent: None,
            order_description: Some(OrderDescription {
                pair: "USDC/USD".to_string(),
                position: None,
                side: BuySell::Sell,
                order_type: OrderType::Limit,
                price: "1.00150000".to_string(),
                price_2: Some("0.00000000".to_string()),
                leverage: None,
                order: "sell 106.53408600 USDC/USD @ limit 1.00150000".to_string(),
                close: None,
            }),
            last_updated: None,
            volume: Some("106.53408600".to_string()),
            executed_volume: Some("0.00000000".to_string()),
            cost: Some("0.00000000".to_string()),
            fee: Some("0.00000000".to_string()),
            average_price: Some("0.00000000".to_string()),
            stop_price: Some("0.00000000".to_string()),
            limit_price: Some("0.00000000".to_string()),
            misc: Some("".to_string()),
            order_flags: Some("fcib,post".to_string()),
            time_in_force: Some(TimeInForce::GTC),
            cancel_reason: None,
            rate_count: None,
        };

        let open_order: OpenOrder = serde_json::from_str(OPEN_ORDER).unwrap();

        assert_eq!(expected_open_order, open_order);
    }

    #[test]
    fn test_deserialize_open_contingent_order() {
        let expected_open_contingent_order = OpenOrder {
            order_id: "OBA7Z7-XQVOQ-3NZRDS".to_string(),
            ref_id: None,
            user_ref: Some(0),
            status: Some(OrderStatus::Pending),
            open_time: Some("1698761179.336974".to_string()),
            start_time: None,
            display_volume: None,
            display_volume_remain: None,
            expire_time: None,
            contingent: None,
            order_description: Some(OrderDescription {
                pair: "USDC/USD".to_string(),
                position: None,
                side: BuySell::Buy,
                order_type: OrderType::TakeProfitLimit,
                price: "0.99000000".to_string(),
                price_2: Some("0.99100000".to_string()),
                leverage: None,
                order: "buy 5.00000000 USDC/USD @ take-profit-limit 0.99000000, limit 0.99100000"
                    .to_string(),
                close: None,
            }),
            last_updated: None,
            volume: Some("5.00000000".to_string()),
            executed_volume: Some("0.00000000".to_string()),
            cost: Some("0.00000000".to_string()),
            fee: Some("0.00000000".to_string()),
            average_price: Some("0.00000000".to_string()),
            stop_price: Some("0.00000000".to_string()),
            limit_price: Some("0.00000000".to_string()),
            misc: Some("".to_string()),
            order_flags: Some("fciq".to_string()),
            time_in_force: Some(TimeInForce::GTC),
            cancel_reason: None,
            rate_count: None,
        };

        let open_order: OpenOrder = serde_json::from_str(OPEN_CONTINGENT_ORDER).unwrap();

        assert_eq!(expected_open_contingent_order, open_order);
    }

    #[test]
    fn test_deserialize_status_change() {
        let expected_open_order_status_change = OrderStatusChange {
            order_id: "OBA7Z7-XQVOQ-3NZRDS".to_string(),
            user_ref: Some(0),
            last_updated: None,
            volume_executed: None,
            cost: None,
            fee: None,
            average_price: None,
            status: "open".to_string(),
            cancel_reason: None,
        };

        let status_change: OrderStatusChange = serde_json::from_str(STATUS_CHANGE).unwrap();

        assert_eq!(expected_open_order_status_change, status_change);
    }

    #[test]
    fn test_deserialize_user_cancel_status_change() {
        let expected_open_order_status_change = OrderStatusChange {
            order_id: "OBA7Z7-XQVOQ-3NZRDS".to_string(),
            user_ref: Some(0),
            last_updated: Some(datetime!(2023-10-31 14:28:06.39107 UTC)),
            volume_executed: Some("0.00000000".to_string()),
            cost: Some("0.00000000".to_string()),
            fee: Some("0.00000000".to_string()),
            average_price: Some("0.00000000".to_string()),
            status: "canceled".to_string(),
            cancel_reason: Some("User requested".to_string()),
        };

        let status_change: OrderStatusChange =
            serde_json::from_str(STATUS_CHANGE_USER_CANCEL).unwrap();

        assert_eq!(expected_open_order_status_change, status_change);
    }
}
