//! Subscribe and unsubscribe messages for websocket channels
use crate::crypto::secrets::Token;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{Display, Formatter};

/// All available channels for subscription
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SubscriptionName {
    Book,
    Ohlc,
    OpenOrders,
    OwnTrades,
    Spread,
    Ticker,
    Trade,
    #[serde(rename = "*")]
    All,
}

impl Display for SubscriptionName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SubscriptionName::Book => write!(f, "book"),
            SubscriptionName::Ohlc => write!(f, "ohlc"),
            SubscriptionName::OpenOrders => write!(f, "openOrders"),
            SubscriptionName::OwnTrades => write!(f, "ownTrades"),
            SubscriptionName::Spread => write!(f, "spread"),
            SubscriptionName::Ticker => write!(f, "ticker"),
            SubscriptionName::Trade => write!(f, "trade"),
            SubscriptionName::All => write!(f, "*"),
        }
    }
}

/// General struct for subscribing to any channel.
///
/// Not all fields apply to each subscription, see the individual `new_*` methods for constructing
/// well-formed subscriptions for each channel.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subscription {
    pub depth: Option<i64>,
    pub interval: Option<i64>,
    pub name: Option<SubscriptionName>,
    #[serde(rename = "ratecounter")]
    pub rate_counter: Option<bool>,
    pub snapshot: Option<bool>,
    pub token: Option<Token>,
    pub consolidate_taker: Option<bool>,
}

impl Subscription {
    /// Create a new subscription to the orderbook at a given depth.
    pub fn new_book_subscription(depth: Option<i64>) -> Subscription {
        Subscription {
            depth,
            interval: None,
            name: Some(SubscriptionName::Book),
            rate_counter: None,
            snapshot: None,
            token: None,
            consolidate_taker: None,
        }
    }

    /// Create a new subscription to the OHLC channel at the given interval.
    pub fn new_ohlc_subscription(interval: Option<i64>) -> Subscription {
        Subscription {
            depth: None,
            interval,
            name: Some(SubscriptionName::Ohlc),
            rate_counter: None,
            snapshot: None,
            token: None,
            consolidate_taker: None,
        }
    }

    /// Create a new subscription to the OwnTrades channel for a specific token.
    ///
    /// Optionally receive a snapshot of recent orders.
    pub fn new_own_trades_subscription(token: Token, snapshot: Option<bool>) -> Subscription {
        Subscription {
            depth: None,
            interval: None,
            name: Some(SubscriptionName::OwnTrades),
            rate_counter: None,
            snapshot,
            token: Some(token),
            consolidate_taker: None,
        }
    }

    /// Create a new subscription to the public trades channel.
    pub fn new_trades_subscription() -> Subscription {
        Subscription {
            depth: None,
            interval: None,
            name: Some(SubscriptionName::Trade),
            rate_counter: None,
            snapshot: None,
            token: None,
            consolidate_taker: None,
        }
    }

    /// Create a new subscription to the Ticker channel.
    pub fn new_ticker_subscription() -> Subscription {
        Subscription {
            depth: None,
            interval: None,
            name: Some(SubscriptionName::Ticker),
            rate_counter: None,
            snapshot: None,
            token: None,
            consolidate_taker: None,
        }
    }

    /// Create a new subscription to the Spread channel for best bids/asks.
    pub fn new_spread_subscription() -> Subscription {
        Subscription {
            depth: None,
            interval: None,
            name: Some(SubscriptionName::Spread),
            rate_counter: None,
            snapshot: None,
            token: None,
            consolidate_taker: None,
        }
    }

    /// Create a new subscription to all open orders for the user.
    ///
    /// Optionally get rate limiter updates from the server.
    pub fn new_open_orders_subscription(token: Token, rate_counter: Option<bool>) -> Subscription {
        Subscription {
            depth: None,
            interval: None,
            name: Some(SubscriptionName::OpenOrders),
            rate_counter,
            snapshot: None,
            token: Some(token),
            consolidate_taker: None,
        }
    }
}

/// Message for unsubscribing from a given channel.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Unsubscription {
    pub depth: Option<i64>,
    pub interval: Option<i64>,
    pub name: Option<SubscriptionName>,
    pub token: Option<Token>,
}

/// Using a given [Subscription] message, generate the corresponding [Unsubscription] message.
impl From<Subscription> for Unsubscription {
    fn from(value: Subscription) -> Self {
        Unsubscription {
            depth: value.depth,
            interval: value.interval,
            name: value.name,
            token: value.token,
        }
    }
}

/// Struct for subscribing to any websocket channel.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubscribeMessage {
    pub event: String,
    #[serde(rename = "reqid")]
    pub req_id: i64,
    pub pair: Option<Vec<String>>,
    pub subscription: Subscription,
}

impl SubscribeMessage {
    /// Return a new [SubscribeMessage], potentially for a specific set of pairs.
    pub fn new(
        req_id: i64,
        pair: Option<Vec<String>>,
        subscription: Subscription,
    ) -> SubscribeMessage {
        SubscribeMessage {
            event: "subscribe".into(),
            req_id,
            pair,
            subscription,
        }
    }
}

/// A message to unsubscribe from the given channel and optionally for a specific pair.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnsubscribeMessage {
    pub event: String,
    #[serde(rename = "reqid")]
    pub req_id: i64,
    pub pair: Option<Vec<String>>,
    pub subscription: Unsubscription,
}

impl UnsubscribeMessage {
    pub fn new(
        req_id: i64,
        pair: Option<Vec<String>>,
        unsubscription: Unsubscription,
    ) -> UnsubscribeMessage {
        UnsubscribeMessage {
            event: "unsubscribe".into(),
            req_id,
            pair,
            subscription: unsubscription,
        }
    }
}

impl From<SubscribeMessage> for UnsubscribeMessage {
    /// Create the corresponding [UnsubscribeMessage] for this [SubscribeMessage]
    fn from(value: SubscribeMessage) -> Self {
        UnsubscribeMessage {
            event: "unsubscribe".to_string(),
            req_id: value.req_id,
            pair: value.pair,
            subscription: value.subscription.into(),
        }
    }
}

/// Message to unsubscribe from a given channel or pair
#[derive(Debug, Deserialize)]
pub struct Unsubscribe {
    pub event: String,
    #[serde(rename = "reqid")]
    pub req_id: i64,
    pub pair: Vec<String>,
    pub subscription: Unsubscription,
}

/// Generic response for any subscription
///
/// This optionally contains fields relevant to each type of subscription.
#[derive(Debug, PartialEq, Deserialize)]
pub struct SubscriptionResponse {
    pub depth: Option<i64>,
    pub interval: Option<i64>,
    #[serde(rename = "maxratecount")]
    pub max_rate_count: Option<i64>,
    pub name: Option<SubscriptionName>,
    pub token: Option<Token>,
}

/// Status message returned after a subscription
#[derive(Debug, PartialEq, Deserialize)]
pub struct SubscriptionStatus {
    #[serde(rename = "channelID")]
    pub channel_id: Option<i64>,
    #[serde(rename = "channelName")]
    pub channel_name: String,
    pub event: String,
    pub pair: Option<String>,
    #[serde(rename = "reqid")]
    pub req_id: i64,
    pub status: String,
    pub subscription: SubscriptionResponse,
    #[serde(rename = "OneOf")]
    pub one_of: Option<OneOf>,
}

/// Wrapper type for an error message or the successfully subscribed channel id
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct OneOf {
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    #[serde(rename = "channelID")]
    pub channel_id: i64,
}

#[cfg(test)]
mod tests {
    use crate::crypto::secrets::Token;
    use crate::wss::subscribe_messages::{Subscription, SubscriptionName};

    #[test]
    fn test_new_book_subscription() {
        let book_subscription = Subscription::new_book_subscription(Some(10));
        assert_eq!(Some(10), book_subscription.depth);
        assert_eq!(Some(SubscriptionName::Book), book_subscription.name);
    }

    #[test]
    fn test_new_ohlc_subscription() {
        let ohlc_subscription = Subscription::new_ohlc_subscription(Some(60));
        assert_eq!(Some(60), ohlc_subscription.interval);
        assert_eq!(Some(SubscriptionName::Ohlc), ohlc_subscription.name);
    }

    #[test]
    fn test_new_own_trades_subscription() {
        let ohlc_subscription = Subscription::new_own_trades_subscription(
            Token::new("someToken".to_string()),
            Some(true),
        );
        assert_eq!(
            Some(Token::new("someToken".to_string())),
            ohlc_subscription.token
        );
        assert_eq!(Some(true), ohlc_subscription.snapshot);
        assert_eq!(Some(SubscriptionName::OwnTrades), ohlc_subscription.name);
    }

    #[test]
    fn test_new_ticker_subscription() {
        let ohlc_subscription = Subscription::new_ticker_subscription();
        assert_eq!(Some(SubscriptionName::Ticker), ohlc_subscription.name);
    }

    #[test]
    fn test_new_spread_subscription() {
        let ohlc_subscription = Subscription::new_spread_subscription();
        assert_eq!(Some(SubscriptionName::Spread), ohlc_subscription.name);
    }

    #[test]
    fn test_new_open_orders_subscription() {
        let ohlc_subscription = Subscription::new_open_orders_subscription(
            Token::new("someToken".to_string()),
            Some(false),
        );
        assert_eq!(
            Some(Token::new("someToken".to_string())),
            ohlc_subscription.token
        );
        assert_eq!(Some(false), ohlc_subscription.rate_counter);
        assert_eq!(Some(SubscriptionName::OpenOrders), ohlc_subscription.name);
    }
}
