//! Websocket client implementation
//!
//! Connect to public or private messages using [KrakenWSSClient], and send/receive messages using
//! [KrakenMessageStream].
//!
pub mod errors;
mod kraken_wss_client;
mod messages;

pub use kraken_wss_client::{KrakenMessageStream, KrakenWSSClient, WS_KRAKEN, WS_KRAKEN_AUTH};
pub use messages::*;
