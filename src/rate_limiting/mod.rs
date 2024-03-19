//! Implementations of common rate limiting schemes required for using Kraken's API effectively
//!
//! Throughout, Kraken utilizes floating-point rate limit values, but this implementation uses integers
//! that are scaled appropriately. I.e. if the Kraken docs specify a cost of 1.0, it will cost 100
//! in this scheme, and a replenishment rate of 2.34/s in the docs will be 234/s here.
//!
//! This was a simplification that allowed using Semaphore permits as the core rate limiting concept
//! under the hood.
pub mod keyed_rate_limits;
pub mod trading_rate_limits;
pub mod ttl_cache;
