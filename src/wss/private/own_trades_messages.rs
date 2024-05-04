//! OwnTrade messages (a user's trade stream)
use crate::response_types::{BuySell, OrderType};
use crate::wss::kraken_wss_types::Sequence;
use rust_decimal::Decimal;
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
    price: Decimal,
    cost: Decimal,
    fee: Decimal,
    #[serde(rename(deserialize = "vol"))]
    volume: Decimal,
    margin: Decimal,
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
    pub trade_id: String,
    pub order_tx_id: String,
    pub position_trade_id: String,
    pub pair: String,
    pub time: OffsetDateTime,
    pub trade_type: BuySell,
    pub order_type: OrderType,
    pub price: Decimal,
    pub cost: Decimal,
    pub fee: Decimal,
    pub volume: Decimal,
    pub margin: Decimal,
    pub user_ref: Option<String>,
}

/// A message containing a user's trades
#[derive(Debug, Deserialize_tuple)]
pub struct OwnTradeMessage {
    pub trades: Vec<OwnTrade>,
    #[serde(rename = "channelName")]
    pub channel_name: String,
    pub sequence: Sequence,
}

impl<'de> Deserialize<'de> for OwnTrade {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OwnTradeVisitor;

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
    use rust_decimal_macros::dec;
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
            price: dec!(1.0001),
            cost: dec!(181.248123),
            fee: dec!(0.36249625),
            volume: dec!(181.23),
            margin: dec!(0),
            user_ref: None,
        };

        let own_trade: OwnTrade = serde_json::from_str(OWN_TRADE).unwrap();

        assert_eq!(expected_trade, own_trade);
    }
}
