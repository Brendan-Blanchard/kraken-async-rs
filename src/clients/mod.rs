//! REST client implementations
//!
//! Trait:
//! - [KrakenClient]: Core trait defining all supported REST calls to Kraken
//!
//! Implementations:
//! - [CoreKrakenClient]: Basic impl of REST calls with no rate limiting or additional behavior
//! - [RateLimitedKrakenClient]: Rate-limited decorator of arbitrary [KrakenClient] implementations
//!
#[allow(unused)]
use crate::clients::core_kraken_client::CoreKrakenClient;
#[allow(unused)]
use crate::clients::kraken_client::KrakenClient;
#[allow(unused)]
use crate::clients::rate_limited_kraken_client::RateLimitedKrakenClient;

pub mod core_kraken_client;
pub mod errors;
pub mod http_response_types;
pub mod kraken_client;
pub mod rate_limited_kraken_client;
