//! Websocket client implementation
//!
//! Connect to public or private messages using [KrakenWSSClient], and send/receive messages using
//! [KrakenMessageStream].
//!
#[allow(unused)]
use crate::wss::kraken_wss_client::KrakenMessageStream;
#[allow(unused)]
use crate::wss::kraken_wss_client::KrakenWSSClient;
pub mod errors;
pub mod kraken_wss_client;
pub mod kraken_wss_types;
mod parsing;
pub mod private;
pub mod public;
pub mod subscribe_messages;
