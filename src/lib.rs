//! A fully-typed API wrapper of the Kraken Pro API for both REST calls and Websockets.
//!
//! This wrapper is meant to be usable on its own if trading solely on Kraken, but has not been
//! overly-tailored for direct use. The ideal use-case is writing generic trading code that works
//! on a layer above this client (whatever your ideal API is), and then uses this library to
//! implement your version on Kraken.
//!
pub mod clients;
pub mod crypto;
pub mod rate_limiting;
pub mod request_types;
pub mod response_types;
pub mod secrets;
#[cfg(test)]
pub mod test_data;
#[cfg(feature = "test-support")]
pub mod test_support;
pub mod wss;
