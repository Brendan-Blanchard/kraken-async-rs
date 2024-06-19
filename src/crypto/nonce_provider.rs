//! Trait and implementation for providing request nonces
#[allow(unused)]
use crate::clients::kraken_client::KrakenClient;
use std::fmt::Debug;
use std::time::{SystemTime, UNIX_EPOCH};

/// A nonce generator that should be used to populate the nonce of every request created by a
/// [KrakenClient].
pub trait NonceProvider: Send + Sync + Debug {
    fn get_nonce(&mut self) -> u64;
}

/// A nonce generator that gives the current epoch in milliseconds, except when called in the same
/// millisecond, in which case it increases the nonce by 1 so no duplicates are ever returned.
///
/// You may wish to consider setting a `nonce window` on your API key to allow for out of order nonces
/// within several seconds of each other. Consult the [Kraken docs] for instructions and trade-offs.
///
/// [Kraken docs]: https://support.kraken.com/hc/en-us/articles/360001148023-What-is-a-nonce-window-
#[derive(Debug, Clone, Copy)]
pub struct IncreasingNonceProvider {
    last: u64,
}

impl Default for IncreasingNonceProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl IncreasingNonceProvider {
    pub fn new() -> Self {
        IncreasingNonceProvider { last: 0 }
    }
}

impl NonceProvider for IncreasingNonceProvider {
    /// Returns the current time in milliseconds, or the last nonce + 1 if a duplicate would be
    /// generated.
    fn get_nonce(&mut self) -> u64 {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        if nonce <= self.last {
            self.last += 1;
        } else {
            self.last = nonce;
        }

        self.last
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};

    #[test]
    fn test_increasing_nonce_provider() {
        let mut provider = IncreasingNonceProvider::default();

        let mut last = 0;

        for _ in 0..100 {
            let nonce = provider.get_nonce();
            assert!(nonce > last);
            last = nonce;
        }
    }
}
