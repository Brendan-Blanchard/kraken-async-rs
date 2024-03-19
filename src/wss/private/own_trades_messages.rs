//! OwnTrade messages (a user's trade stream)
use crate::response_types::{BuySell, OrderType};
use crate::wss::kraken_wss_types::Sequence;
use serde::de::{MapAccess, Visitor};
use serde::{de, Deserialize, Deserializer};
use serde_tuple::Deserialize_tuple;
use serde_with::formats::Strict;
use serde_with::TimestampSecondsWithFrac;
use std::fmt::Formatter;
use time::OffsetDateTime;

/// Raw trade type to deserialize to, missing the trade's id due to API design
#[serde_with::serde_as]
#[derive(Debug, Deserialize, PartialEq)]
struct RawOwnTrade {
    #[serde(rename(deserialize = "ordertxid"))]
    order_tx_id: String,
    #[serde(rename(deserialize = "postxid"))]
    position_trade_id: String,
    pair: String,
    #[serde_as(as = "TimestampSecondsWithFrac<String, Strict>")]
    time: OffsetDateTime,
    #[serde(rename(deserialize = "type"))]
    trade_type: BuySell,
    #[serde(rename(deserialize = "ordertype"))]
    order_type: OrderType,
    price: String,
    cost: String,
    fee: String,
    #[serde(rename(deserialize = "vol"))]
    volume: String,
    margin: String,
    #[serde(rename(deserialize = "userref"))]
    user_ref: Option<String>,
}

impl RawOwnTrade {
    pub fn into_own_trade(self, trade_id: String) -> OwnTrade {
        OwnTrade {
            trade_id,
            order_tx_id: self.order_tx_id,
            position_trade_id: self.position_trade_id,
            pair: self.pair,
            time: self.time,
            trade_type: self.trade_type,
            order_type: self.order_type,
            price: self.price,
            cost: self.cost,
            fee: self.fee,
            volume: self.volume,
            margin: self.margin,
            user_ref: self.user_ref,
        }
    }
}

/// A user's trade
#[derive(Debug, PartialEq)]
pub struct OwnTrade {
    trade_id: String,
    order_tx_id: String,
    position_trade_id: String,
    pair: String,
    time: OffsetDateTime,
    trade_type: BuySell,
    order_type: OrderType,
    price: String,
    cost: String,
    fee: String,
    volume: String,
    margin: String,
    user_ref: Option<String>,
}

/// A message containing a user's trades
#[derive(Debug, Deserialize_tuple)]
pub struct OwnTradeMessage {
    trades: Vec<OwnTrade>,
    #[serde(rename = "channelName")]
    channel_name: String,
    sequence: Sequence,
}

impl<'de> Deserialize<'de> for OwnTrade {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OwnTradeVisitor;

        #[derive(Debug, Deserialize)]
        enum OwnTradeField {
            String(String),
            Integer(i64),
        }

        impl<'de> Visitor<'de> for OwnTradeVisitor {
            type Value = OwnTrade;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("OwnTrade")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                if let Some((trade_id, raw_trade)) = map.next_entry::<String, RawOwnTrade>()? {
                    Ok(raw_trade.into_own_trade(trade_id))
                } else {
                    Err(de::Error::invalid_length(0, &self))
                }
            }
        }

        const FIELDS: &[&str] = &[
            "trade_id",
            "order_tx_id",
            "position_trade_id",
            "pair",
            "time",
            "trade_type",
            "order_type",
            "price",
            "cost",
            "fee",
            "volume",
            "margin",
            "user_ref",
        ];

        deserializer.deserialize_struct("OwnTrade", FIELDS, OwnTradeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    const OWN_TRADE: &str = "{\
    \"TUCGO2-AXD3U-QAPOWI\":{\"cost\":\"181.24812300\",\"fee\":\"0.36249625\",\
    \"margin\":\"0.00000000\",\"ordertxid\":\"OKOJVE-XTIWX-WB4RPB\",\"ordertype\":\"limit\",\
    \"pair\":\"USDT/USD\",\"postxid\":\"TKH2SE-M7IF5-CFI7LT\",\"price\":\"1.00010000\",\
    \"time\":\"1695932569.208424\",\"type\":\"sell\",\"vol\":\"181.23000000\"}}";

    #[test]
    fn test_deserialize() {
        let expected_trade = OwnTrade {
            trade_id: "TUCGO2-AXD3U-QAPOWI".to_string(),
            order_tx_id: "OKOJVE-XTIWX-WB4RPB".to_string(),
            position_trade_id: "TKH2SE-M7IF5-CFI7LT".to_string(),
            pair: "USDT/USD".to_string(),
            time: datetime!(2023-09-28 20:22:49.208424 UTC),
            trade_type: BuySell::Sell,
            order_type: OrderType::Limit,
            price: "1.00010000".to_string(),
            cost: "181.24812300".to_string(),
            fee: "0.36249625".to_string(),
            volume: "181.23000000".to_string(),
            margin: "0.00000000".to_string(),
            user_ref: None,
        };

        let own_trade: OwnTrade = serde_json::from_str(OWN_TRADE).unwrap();

        assert_eq!(expected_trade, own_trade);
    }
}
