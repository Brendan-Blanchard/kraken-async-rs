use crate::response_types::BidOrAsk;
use rust_decimal::Decimal;
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::{de, Deserialize, Deserializer};
use serde_tuple::Deserialize_tuple;
use std::fmt::Formatter;

/// A bid or ask, depending on context
#[derive(Debug, Deserialize_tuple, PartialEq)]
pub struct BidAsk {
    pub price: Decimal,
    pub volume: Decimal,
    pub time: String,
}

/// Message for an orderbook snapshot
#[derive(Debug, Deserialize, PartialEq)]
pub struct OrderbookMessage {
    #[serde(rename = "channelID")]
    pub channel_id: i64,
    pub orderbook: Orderbook,
    #[serde(rename = "channelName")]
    pub channel_name: String,
    pub pair: String,
}

/// Orderbook containing a `Vec<BidAsk>` for each of bids and asks
#[derive(Debug, Deserialize, PartialEq)]
pub struct Orderbook {
    #[serde(rename = "as")]
    pub asks: Vec<BidAsk>,
    #[serde(rename = "bs")]
    pub bids: Vec<BidAsk>,
}

/// Update to a price point for bids or asks
#[derive(Debug, PartialEq)]
pub struct BidAskUpdate {
    pub price: Decimal,
    pub volume: Decimal,
    pub timestamp: String,
    pub update_type: Option<String>,
}

/// Updates for a particular side of the book, including checksum of final book state
#[derive(Debug, PartialEq)]
struct BidAskUpdates {
    pub side: BidOrAsk,
    pub updates: Vec<BidAskUpdate>,
    pub checksum: Option<String>,
}

/// Orderbook update message containing all updates and the expected checksum of the final book
#[derive(Debug, PartialEq)]
pub struct OrderbookUpdateMessage {
    pub channel_id: i64,
    pub bids: Vec<BidAskUpdate>,
    pub asks: Vec<BidAskUpdate>,
    pub channel_name: String,
    pub pair: String,
    pub checksum: String,
}

impl<'de> Deserialize<'de> for OrderbookUpdateMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize, PartialEq)]
        #[serde(untagged)]
        enum MessageField {
            ChannelId(i64),
            BidAskUpdates(BidAskUpdates),
            String(String),
        }

        struct OrderbookUpdateMessageVisitor;

        impl<'de> Visitor<'de> for OrderbookUpdateMessageVisitor {
            type Value = OrderbookUpdateMessage;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("OrderbookUpdateMessage")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                if let Some(size) = seq.size_hint() {
                    if !(4..=5).contains(&size) {
                        return Err(de::Error::invalid_length(size, &self));
                    }
                }

                let channel_id;
                let mut bids = None;
                let mut asks = None;
                let mut channel_name = None;
                let mut pair = None;
                let mut checksum = None;

                let channel_id_field: MessageField = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let first_update_field: MessageField = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let update_or_channel_name_field: MessageField = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;

                if let MessageField::ChannelId(id) = channel_id_field {
                    channel_id = Some(id);
                } else {
                    return Err(de::Error::custom("first field was not channel id as i64"));
                }

                match update_or_channel_name_field {
                    MessageField::ChannelId(_) => {}
                    MessageField::String(name) => {
                        channel_name = Some(name);

                        let pair_field: MessageField = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(3, &self))?;

                        if let MessageField::String(pair_value) = pair_field {
                            pair = Some(pair_value)
                        }

                        match first_update_field {
                            MessageField::BidAskUpdates(first_updates) => {
                                match first_updates.side {
                                    BidOrAsk::Bid => bids = Some(first_updates.updates),
                                    BidOrAsk::Ask => asks = Some(first_updates.updates),
                                }

                                checksum = first_updates.checksum;
                            }
                            _ => {
                                return Err(de::Error::custom(
                                    "second field could not be deserialized to BidAskUpdates",
                                ));
                            }
                        }
                    }
                    MessageField::BidAskUpdates(second_updates) => {
                        let name_field: MessageField = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(4, &self))?;

                        let pair_field: MessageField = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(5, &self))?;

                        if let MessageField::String(name) = name_field {
                            channel_name = Some(name)
                        }

                        if let MessageField::String(pair_value) = pair_field {
                            pair = Some(pair_value)
                        }

                        if let MessageField::BidAskUpdates(first_updates) = first_update_field {
                            match first_updates.side {
                                BidOrAsk::Bid => bids = Some(first_updates.updates),
                                BidOrAsk::Ask => asks = Some(first_updates.updates),
                            }
                        } else {
                            return Err(de::Error::custom(
                                "second field could not be deserialized to BidAskUpdates",
                            ));
                        }

                        match second_updates.side {
                            BidOrAsk::Bid => bids = Some(second_updates.updates),
                            BidOrAsk::Ask => asks = Some(second_updates.updates),
                        }

                        checksum = second_updates.checksum
                    }
                }

                if let Some(checksum) = checksum {
                    Ok(OrderbookUpdateMessage {
                        channel_id: channel_id.unwrap(),
                        bids: bids.unwrap_or(vec![]),
                        asks: asks.unwrap_or(vec![]),
                        channel_name: channel_name.unwrap(),
                        pair: pair.unwrap(),
                        checksum,
                    })
                } else {
                    Err(de::Error::custom("checksum not present on bids or asks"))
                }
            }
        }

        const FIELDS: &[&str] = &["channel_id", "bids", "asks", "channel_name", "pair"];
        deserializer.deserialize_struct(
            "OrderbookUpdateMessage",
            FIELDS,
            OrderbookUpdateMessageVisitor,
        )
    }
}

impl<'de> Deserialize<'de> for BidAskUpdates {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BidAskUpdatesVisitor;

        impl<'de> Visitor<'de> for BidAskUpdatesVisitor {
            type Value = BidAskUpdates;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("object containing 'a' or 'b' key")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                #[derive(Debug, Deserialize, PartialEq)]
                #[serde(untagged)]
                enum BidAskUpdatesOrChecksum {
                    BidAskUpdates(Vec<BidAskUpdate>),
                    Checksum(String),
                }

                let mut side = BidOrAsk::Bid;
                let mut updates = None;
                let mut checksum = None;

                if let Some(size) = map.size_hint() {
                    if !(1..=2).contains(&size) {
                        return Err(de::Error::invalid_length(size, &self));
                    }
                }

                while let Some((key, value)) =
                    map.next_entry::<String, BidAskUpdatesOrChecksum>()?
                {
                    match value {
                        BidAskUpdatesOrChecksum::BidAskUpdates(value) => {
                            updates = Some(value);
                            match key.as_str() {
                                "a" => side = BidOrAsk::Ask,
                                "b" => side = BidOrAsk::Bid,
                                field => {
                                    return Err(de::Error::unknown_field(field, &["a", "b", "c"]));
                                }
                            }
                        }
                        BidAskUpdatesOrChecksum::Checksum(value) => checksum = Some(value),
                    }
                }

                let updates = updates.ok_or_else(|| de::Error::missing_field("a | b"))?;

                Ok(BidAskUpdates {
                    side,
                    updates,
                    checksum,
                })
            }
        }

        const FIELDS: &[&str] = &["updates", "checksum"];
        deserializer.deserialize_struct("Bid", FIELDS, BidAskUpdatesVisitor)
    }
}

impl<'de> Deserialize<'de> for BidAskUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BidAskUpdatesVisitor;

        impl<'de> Visitor<'de> for BidAskUpdatesVisitor {
            type Value = BidAskUpdate;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("sequence BidAskUpdate")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let price = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let volume = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let timestamp = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let update_type = seq.next_element()?.unwrap_or(None);

                Ok(BidAskUpdate {
                    price,
                    volume,
                    timestamp,
                    update_type,
                })
            }
        }

        const FIELDS: &[&str] = &["price", "volume", "timestamp", "update_type"];

        deserializer.deserialize_struct("BidAskUpdates", FIELDS, BidAskUpdatesVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response_types::BidOrAsk::{Ask, Bid};
    use rust_decimal_macros::dec;

    const BID_ASK_UPDATE: &str = "[\"34236.30000\",\"0.20206679\",\"1698231193.410190\"]";
    const BID_ASK_UPDATE_REPUBLISH: &str =
        "[\"34210.00000\",\"2.23575055\",\"1698230978.434643\",\"r\"]";
    const BIDS_ONLY: &str =
        "{\"b\":[[\"34211.00000\",\"2.92303645\",\"1698230979.415494\"]],\"c\":\"976603157\"}";
    const ASKS_ONLY: &str =
        "{\"a\":[[\"34240.10000\",\"0.36250000\",\"1698230979.497975\"]],\"c\":\"2409299850\"}";
    const BIDS_WITHOUT_CHECKSUM: &str =
        "{\"b\":[[\"34211.00000\",\"2.92303645\",\"1698230979.415494\"]]}";

    const ASK_ONLY_MESSAGE: &str = "[336,{\"a\":[[\"34240.10000\",\"0.36250000\",\"1698230979.497975\"]],\"c\":\"2409299850\"},\"book-10\",\"XBT/USD\"]";
    const BID_ONLY_MESSAGE: &str = "[336,{\"b\":[[\"34211.00000\",\"2.92303645\",\"1698230979.415494\"]],\"c\":\"976603157\"},\"book-10\",\"XBT/USD\"]";
    const BID_ASK_MESSAGE: &str = "[336,{\
    \"a\":[[\"34236.30000\",\"0.20206679\",\"1698231193.410190\"]]},\
    {\"b\":[[\"34239.90000\",\"0.24117192\",\"1698231193.409849\"],[\"34208.40000\",\"0.36250000\",\"1698231193.221649\",\"r\"]],\"c\":\"4258113849\"},\
    \"book-10\",\"XBT/USD\"]";

    const BID_ASK_MESSAGE_MISSING_CHECKSUM: &str = "[336,{\
    \"a\":[[\"34236.30000\",\"0.20206679\",\"1698231193.410190\"]]},\
    {\"b\":[[\"34239.90000\",\"0.24117192\",\"1698231193.409849\"],[\"34208.40000\",\"0.36250000\",\"1698231193.221649\",\"r\"]]},\
    \"book-10\",\"XBT/USD\"]";

    const BID_ASK_MESSAGE_INCORRECT_FIRST_FIELD: &str =
        "[336,\"thisShouldn'tBeAString\",\"book-10\",\"XBT/USD\"]";

    #[test]
    fn test_deserializing_update() {
        let expected_bid_ask_update: BidAskUpdate = BidAskUpdate {
            price: dec!(34236.30000),
            volume: dec!(0.20206679),
            timestamp: "1698231193.410190".to_string(),
            update_type: None,
        };

        let bid_ask_update: BidAskUpdate =
            serde_json::from_str(BID_ASK_UPDATE).expect("Deserialize should not fail");

        assert_eq!(expected_bid_ask_update, bid_ask_update);
    }

    #[test]
    fn test_deserializing_update_republish() {
        let expected_bid_ask_update: BidAskUpdate = BidAskUpdate {
            price: dec!(34210.00000),
            volume: dec!(2.23575055),
            timestamp: "1698230978.434643".to_string(),
            update_type: Some("r".to_string()),
        };

        let bid_ask_update: BidAskUpdate = serde_json::from_str(BID_ASK_UPDATE_REPUBLISH).unwrap();

        assert_eq!(expected_bid_ask_update, bid_ask_update);
    }

    #[test]
    fn test_deserializing_bids_only() {
        let expected_bids_update = BidAskUpdates {
            side: Bid,
            updates: vec![BidAskUpdate {
                price: dec!(34211.00000),
                volume: dec!(2.92303645),
                timestamp: "1698230979.415494".to_string(),
                update_type: None,
            }],
            checksum: Some("976603157".into()),
        };

        let bids_update: BidAskUpdates = serde_json::from_str(BIDS_ONLY).unwrap();

        assert_eq!(expected_bids_update, bids_update);
    }

    #[test]
    fn test_deserializing_asks_only() {
        let expected_asks_update = BidAskUpdates {
            side: Ask,
            updates: vec![BidAskUpdate {
                price: dec!(34240.10000),
                volume: dec!(0.36250000),
                timestamp: "1698230979.497975".to_string(),
                update_type: None,
            }],
            checksum: Some("2409299850".into()),
        };

        let bids_update: BidAskUpdates = serde_json::from_str(ASKS_ONLY).unwrap();

        assert_eq!(expected_asks_update, bids_update);
    }

    #[test]
    fn test_deserializing_bids_without_checksum() {
        let expected_asks_update = BidAskUpdates {
            side: Bid,
            updates: vec![BidAskUpdate {
                price: dec!(34211.00000),
                volume: dec!(2.92303645),
                timestamp: "1698230979.415494".to_string(),
                update_type: None,
            }],
            checksum: None,
        };

        let bids_update: BidAskUpdates = serde_json::from_str(BIDS_WITHOUT_CHECKSUM).unwrap();

        assert_eq!(expected_asks_update, bids_update);
    }

    #[test]
    fn test_deserializing_orderbook_message_asks_only() {
        let expected_message = OrderbookUpdateMessage {
            channel_id: 336,
            bids: vec![],
            asks: vec![BidAskUpdate {
                price: dec!(34240.10000),
                volume: dec!(0.36250000),
                timestamp: "1698230979.497975".to_string(),
                update_type: None,
            }],
            channel_name: "book-10".to_string(),
            pair: "XBT/USD".to_string(),
            checksum: "2409299850".into(),
        };

        let message: OrderbookUpdateMessage = serde_json::from_str(ASK_ONLY_MESSAGE).unwrap();

        assert_eq!(expected_message, message);
    }

    #[test]
    fn test_deserializing_orderbook_message_bids_only() {
        let expected_message = OrderbookUpdateMessage {
            channel_id: 336,
            bids: vec![BidAskUpdate {
                price: dec!(34211.00000),
                volume: dec!(2.92303645),
                timestamp: "1698230979.415494".to_string(),
                update_type: None,
            }],
            asks: vec![],
            channel_name: "book-10".to_string(),
            pair: "XBT/USD".to_string(),
            checksum: "976603157".to_string(),
        };

        let message: OrderbookUpdateMessage = serde_json::from_str(BID_ONLY_MESSAGE).unwrap();

        assert_eq!(expected_message, message);
    }

    #[test]
    fn test_deserializing_orderbook_message_bids_and_asks() {
        let expected_message = OrderbookUpdateMessage {
            channel_id: 336,
            bids: vec![
                BidAskUpdate {
                    price: dec!(34239.90000),
                    volume: dec!(0.24117192),
                    timestamp: "1698231193.409849".to_string(),
                    update_type: None,
                },
                BidAskUpdate {
                    price: dec!(34208.40000),
                    volume: dec!(0.36250000),
                    timestamp: "1698231193.221649".to_string(),
                    update_type: Some("r".into()),
                },
            ],
            asks: vec![BidAskUpdate {
                price: dec!(34236.30000),
                volume: dec!(0.20206679),
                timestamp: "1698231193.410190".to_string(),
                update_type: None,
            }],
            channel_name: "book-10".to_string(),
            pair: "XBT/USD".to_string(),
            checksum: "4258113849".to_string(),
        };

        let message: OrderbookUpdateMessage = serde_json::from_str(BID_ASK_MESSAGE).unwrap();

        assert_eq!(expected_message, message);
    }

    #[test]
    #[should_panic(expected = "checksum not present on bids or asks")]
    fn test_deserializing_orderbook_message_missing_checksum() {
        let _message: OrderbookUpdateMessage =
            serde_json::from_str(BID_ASK_MESSAGE_MISSING_CHECKSUM).unwrap();
    }

    #[test]
    #[should_panic(expected = "second field could not be deserialized to BidAskUpdates")]
    fn test_deserializing_orderbook_message_invalid_first_field() {
        let _message: OrderbookUpdateMessage =
            serde_json::from_str(BID_ASK_MESSAGE_INCORRECT_FIRST_FIELD).unwrap();
    }
}
