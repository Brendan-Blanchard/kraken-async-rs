use async_rate_limit::limiters::{RateLimiter, VariableCostRateLimiter};
use async_rate_limit::sliding_window::SlidingWindowRateLimiter;
use std::collections::BTreeMap;
use std::time::Duration;

/// Create a new public rate limiter.
///
/// All tiers for Kraken are limited to on the order of one request per second.
pub fn new_public_rate_limiter() -> SlidingWindowRateLimiter {
    SlidingWindowRateLimiter::new(Duration::from_secs(1), 1)
}

/// A rate limiter that utilizes a [BTreeMap] to map K -> [SlidingWindowRateLimiter], allowing for
/// a per-argument rate limiter.
///
/// This is used for several endpoints that are rate limited by IP and trading pair, so each pair
/// is given a unique rate limiter.
#[derive(Debug, Clone)]
pub struct KeyedRateLimiter<K>
where
    K: Ord,
{
    rate_limiters: BTreeMap<K, SlidingWindowRateLimiter>,
    default: fn() -> SlidingWindowRateLimiter,
}

impl<K> Default for KeyedRateLimiter<K>
where
    K: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K> KeyedRateLimiter<K>
where
    K: Ord,
{
    /// Create an empty instance with no rate limiters.
    pub fn new() -> Self {
        KeyedRateLimiter {
            rate_limiters: Default::default(),
            default: new_public_rate_limiter,
        }
    }

    /// Add a rate limiter implementation for a given key, such that `self.wait_until_ready(key)`
    /// will use this rate limiter.
    ///
    /// This can overwrite previous rate limiters if the key already exists and returns/follows the
    /// same semantics as [BTreeMap]'s insert method.
    pub fn add_rate_limiter(
        &mut self,
        key: K,
        rate_limiter: SlidingWindowRateLimiter,
    ) -> Option<SlidingWindowRateLimiter> {
        self.rate_limiters.insert(key, rate_limiter)
    }

    /// Remove a rate limiter from the internal map. This may result in subsequent usages of this
    /// key using a default rate limiter.
    ///
    /// This follows the same return semantics as [BTreeMap]'s remove method.
    pub fn remove_rate_limiter(&mut self, key: &K) -> Option<SlidingWindowRateLimiter> {
        self.rate_limiters.remove(key)
    }

    /// Follows the same semantics as [SlidingWindowRateLimiter], except it looks up a rate limiter
    /// by key, and creates a rate limiter if none is found.
    pub async fn wait_until_ready(&mut self, key: K) {
        self.rate_limiters
            .entry(key)
            .or_insert((self.default)())
            .wait_until_ready()
            .await
    }

    /// Follows the same semantics as [SlidingWindowRateLimiter], except it looks up a rate limiter
    /// by key, and creates a rate limiter if none is found.
    pub async fn wait_with_cost(&mut self, cost: usize, key: K) {
        self.rate_limiters
            .entry(key)
            .or_insert((self.default)())
            .wait_with_cost(cost)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::rate_limiting::keyed_rate_limits::KeyedRateLimiter;
    use async_rate_limit::sliding_window::SlidingWindowRateLimiter;
    use std::time::Duration;
    use tokio::time::{pause, Instant};

    #[test]
    fn test_add_remove() {
        let mut limiter = KeyedRateLimiter::new();

        let sub_limiter_1 = SlidingWindowRateLimiter::new(Duration::from_secs(1), 1);
        let sub_limiter_2 = SlidingWindowRateLimiter::new(Duration::from_secs(1), 2);

        let added = limiter.add_rate_limiter("k1", sub_limiter_1.clone());
        assert!(added.is_none());

        let added = limiter.add_rate_limiter("k2", sub_limiter_2.clone());
        assert!(added.is_none());

        assert_eq!(2, limiter.rate_limiters.len());

        let removed = limiter.remove_rate_limiter(&"k1");
        assert!(removed.is_some());

        let removed = limiter.remove_rate_limiter(&"k2");
        assert!(removed.is_some());

        assert_eq!(0, limiter.rate_limiters.len());
    }

    #[tokio::test]
    async fn test_waiting_separately() {
        pause();

        let mut limiter = KeyedRateLimiter::new();

        let sub_limiter_1 = SlidingWindowRateLimiter::new(Duration::from_secs(1), 1);
        let sub_limiter_2 = SlidingWindowRateLimiter::new(Duration::from_secs(1), 2);

        limiter.add_rate_limiter("k1", sub_limiter_1.clone());
        limiter.add_rate_limiter("k2", sub_limiter_2.clone());

        let start = Instant::now();

        for _ in 0..3 {
            limiter.wait_until_ready("k1").await;
        }

        let mid = Instant::now();

        for _ in 0..2 {
            limiter.wait_with_cost(2, "k2").await;
        }

        let end = Instant::now();

        // three calls to the first rate limiter should wait twice, taking 2s
        let elapsed_start_mid = mid - start;

        // 2 calls to the second rate limiter should wait once for 1s
        let elapsed_mid_end = end - mid;

        assert!(elapsed_start_mid > Duration::from_secs(2));
        assert!(elapsed_start_mid < Duration::from_millis(3300));

        assert!(elapsed_mid_end > Duration::from_secs(1));
        assert!(elapsed_mid_end < Duration::from_millis(2200));
    }

    #[tokio::test]
    async fn test_waiting_separately_default() {
        pause();

        let mut limiter = KeyedRateLimiter::new();

        let sub_limiter_1 = SlidingWindowRateLimiter::new(Duration::from_secs(2), 1);

        limiter.add_rate_limiter("k1", sub_limiter_1.clone());

        let start = Instant::now();

        for _ in 0..3 {
            limiter.wait_until_ready("k1").await;
        }

        let mid = Instant::now();

        for _ in 0..3 {
            limiter.wait_with_cost(1, "k2").await;
        }

        let end = Instant::now();

        // three calls to the first rate limiter should wait twice, taking 4s
        let elapsed_start_mid = mid - start;

        // 3 calls to the second (default-inserted) rate limiter should wait twice for 2s total
        let elapsed_mid_end = end - mid;

        assert!(elapsed_start_mid > Duration::from_secs(4));
        assert!(elapsed_start_mid < Duration::from_millis(4300));

        println!("{:?}", elapsed_mid_end);
        assert!(elapsed_mid_end > Duration::from_secs(2));
        assert!(elapsed_mid_end < Duration::from_millis(2200));
    }
}
