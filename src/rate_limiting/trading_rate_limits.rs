use crate::rate_limiting::ttl_cache::{TtlCache, TtlEntry};
use crate::request_types::{AddBatchedOrderRequest, EditOrderRequest};
use crate::response_types::VerificationTier;
use async_rate_limit::limiters::VariableCostRateLimiter;
use async_rate_limit::token_bucket::{TokenBucketRateLimiter, TokenBucketState};
use std::sync::Arc;
use std::time::Duration;
use time::OffsetDateTime;
use tokio::sync::Mutex;

// 300 seconds in microseconds
const ORDER_TTL_US: i128 = 300_i128 * 10_i128.pow(6);

/// An implementation of the most accurate trading rate limits given by Kraken
pub struct KrakenTradingRateLimiter {
    ttl_ref_id_cache: TtlCache<String, i64>,
    ttl_user_ref_cache: TtlCache<i64, i64>,
    rate_limiter: TokenBucketRateLimiter,
}

/// Implements the Advanced rate limiting scheme that requires knowing each order's lifetime.
///
/// Detailed documentation is available from several locations, including the [overview rate-limiting page],
/// [api rate-limiting page] and [trading rate-limiting page].
///
/// [overview rate-limiting page]: https://docs.kraken.com/rest/#section/Rate-Limits/Matching-Engine-Rate-Limits
/// [api rate-limiting page]: https://support.kraken.com/hc/en-us/articles/206548367-What-are-the-API-rate-limits-#3
/// [trading rate-limiting page]: https://support.kraken.com/hc/en-us/articles/360045239571-Trading-rate-limits
impl KrakenTradingRateLimiter {
    /// Create a new instance for a user with the given [VerificationTier]
    pub fn new(user_verification: VerificationTier) -> KrakenTradingRateLimiter {
        KrakenTradingRateLimiter {
            ttl_ref_id_cache: Default::default(),
            ttl_user_ref_cache: Default::default(),
            rate_limiter: Self::get_rate_limiter(user_verification),
        }
    }

    /// Wait for the fixed cost of placing an order
    pub async fn add_order(&mut self) {
        self.rate_limiter.wait_with_cost(100).await;
    }

    /// Determine the cost and wait appropriately for the given [AddBatchedOrderRequest].
    ///
    /// The cost of a batch is n / 2, where n is the number of orders in the batch.
    pub async fn add_order_batch(&mut self, add_batched_order_request: &AddBatchedOrderRequest) {
        let cost = 1.0 + (add_batched_order_request.orders.len() as f64 / 2.0);
        self.rate_limiter
            .wait_with_cost((cost * 100.0) as usize)
            .await;
    }

    /// Determine the cost of editing an order and wait if necessary
    ///
    /// This is inclusive of penalties for orders edited soon after creation.
    pub async fn edit_order(&mut self, edit_order_request: &EditOrderRequest) {
        let now_seconds = OffsetDateTime::now_utc().unix_timestamp();
        let request_id = edit_order_request.tx_id.clone();
        let order_lifetime = self
            .ttl_ref_id_cache
            .get(&request_id)
            .map(|ttl_entry| now_seconds - ttl_entry.data)
            .unwrap_or(i64::MAX);

        let penalty = Self::edit_order_penalty(order_lifetime);
        let cost = (penalty + 1) * 100;

        self.rate_limiter.wait_with_cost(cost as usize).await
    }

    /// Determine the cost of cancelling the provided order id and wait appropriately
    ///
    /// This is inclusive of penalties for orders cancelled soon after creation.
    pub async fn cancel_order_tx_id(&mut self, id: &String) {
        let now_seconds = OffsetDateTime::now_utc().unix_timestamp();
        let order_lifetime = self
            .ttl_ref_id_cache
            .get(id)
            .map(|ttl_entry| now_seconds - ttl_entry.data)
            .unwrap_or(i64::MAX);

        self.cancel_with_penalty(order_lifetime).await;
    }

    /// Determine the cost of cancelling the provided user ref and wait appropriately
    ///
    /// This is inclusive of penalties for orders cancelled soon after creation.
    pub async fn cancel_order_user_ref(&mut self, id: &i64) {
        let now_seconds = OffsetDateTime::now_utc().unix_timestamp();
        let order_lifetime = self
            .ttl_user_ref_cache
            .get(id)
            .map(|ttl_entry| now_seconds - ttl_entry.data)
            .unwrap_or(i64::MAX);

        self.cancel_with_penalty(order_lifetime).await;
    }

    async fn cancel_with_penalty(&mut self, order_lifetime: i64) {
        let penalty = Self::cancel_order_penalty(order_lifetime);
        let cost = penalty * 100;

        self.rate_limiter.wait_with_cost(cost as usize).await
    }

    /// Notify the rate limiter of a new order being created -- this is essential to the rate limiting scheme!
    ///
    /// Order lifetimes must be known in order to determine the penalties for editing or cancelling
    /// orders that were placed less than 300s ago.
    pub fn notify_add_order(&mut self, tx_id: String, placement_time: i64, user_ref: Option<i64>) {
        let ttl_ref_entry = TtlEntry::new(tx_id, ORDER_TTL_US, placement_time);
        self.ttl_ref_id_cache.insert(ttl_ref_entry);

        if let Some(user_ref) = user_ref {
            let ttl_user_ref_entry = TtlEntry::new(user_ref, ORDER_TTL_US, placement_time);
            self.ttl_user_ref_cache.insert(ttl_user_ref_entry);
        }
    }

    fn edit_order_penalty(lifetime_seconds: i64) -> i64 {
        if lifetime_seconds < 5 {
            6
        } else if lifetime_seconds < 10 {
            5
        } else if lifetime_seconds < 15 {
            4
        } else if lifetime_seconds < 45 {
            3
        } else if lifetime_seconds < 90 {
            2
        } else {
            0
        }
    }

    fn cancel_order_penalty(lifetime_seconds: i64) -> i64 {
        if lifetime_seconds < 5 {
            8
        } else if lifetime_seconds < 10 {
            6
        } else if lifetime_seconds < 15 {
            5
        } else if lifetime_seconds < 45 {
            4
        } else if lifetime_seconds < 90 {
            2
        } else if lifetime_seconds < 300 {
            1
        } else {
            0
        }
    }

    fn get_rate_limiter(user_verification: VerificationTier) -> TokenBucketRateLimiter {
        // tokens are scaled 100x from Kraken's floating-point method to keep as integers
        match user_verification {
            VerificationTier::Intermediate => {
                let token_bucket_state = TokenBucketState::new(12500, 234, Duration::from_secs(1));
                TokenBucketRateLimiter::new(Arc::new(Mutex::new(token_bucket_state)))
            }
            VerificationTier::Pro => {
                let token_bucket_state = TokenBucketState::new(18000, 375, Duration::from_secs(1));
                TokenBucketRateLimiter::new(Arc::new(Mutex::new(token_bucket_state)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rate_limiting::trading_rate_limits::KrakenTradingRateLimiter;
    /// Tests use Tokio's pause() functionality to have instantaneous testing that relies on Tokio
    /// keeping track of time elapsed by fast-forwarding when there are no pending tasks on the
    /// event loop.
    ///
    /// Tests are done at a high enough level that execution time of Rust is thought to be negligible.
    ///
    use crate::response_types::VerificationTier::{Intermediate, Pro};
    use std::time::Duration;
    use tokio::time::{pause, Instant};

    #[tokio::test]
    async fn test_trading_rate_limiter_intermediate_add_order_limit() {
        pause();

        let mut limiter = KrakenTradingRateLimiter::new(Intermediate);

        let start = Instant::now();
        // 126 calls should push limiter over the 12500 limit, requiring waiting 1s
        for _ in 0..126 {
            limiter.add_order().await;
        }

        let end = Instant::now();
        let elapsed = end - start;

        assert!(elapsed > Duration::from_secs(1));
        assert!(elapsed < Duration::from_secs(2));
    }

    #[tokio::test]
    async fn test_trading_rate_limiter_pro_add_order_limit() {
        pause();

        let mut limiter = KrakenTradingRateLimiter::new(Pro);

        let start = Instant::now();
        // 181 calls should push limiter over the 18000 limit, requiring waiting 1s
        for _ in 0..181 {
            limiter.add_order().await;
        }

        let end = Instant::now();
        let elapsed = end - start;

        assert!(elapsed > Duration::from_secs(1));
        assert!(elapsed < Duration::from_secs(2));
    }

    #[tokio::test]
    async fn test_trading_rate_limiter_intermediate_add_order_limit_replenish() {
        pause();

        let mut limiter = KrakenTradingRateLimiter::new(Intermediate);

        let start = Instant::now();
        // 126 calls should push limiter over the 12500 limit, requiring waiting 1s.
        //  Replenishing at 234/s means that an additional 15 orders (costing 100 * 15 = 1500 total) should
        //  take another (1500 / 234 = ) 6.4s wait
        for _ in 0..(126 + 15) {
            limiter.add_order().await;
        }

        let end = Instant::now();
        let elapsed = end - start;

        // expect that the first 126 orders took 1s
        //  the remaining 15 should take another ~6.4s
        assert!(elapsed > Duration::from_secs(7));
        assert!(elapsed < Duration::from_secs(8));
    }

    #[tokio::test]
    async fn test_trading_rate_limiter_pro_add_order_limit_replenish() {
        pause();

        let mut limiter = KrakenTradingRateLimiter::new(Pro);

        let start = Instant::now();
        // 181 calls should push limiter over the 18000 limit, requiring waiting 1s.
        //  Replenishing at 375/s means that each additional 4 orders (costing 400 total) should
        //  take another 1s wait for that batch of 4
        for _ in 0..(181 + (4 * 3)) {
            limiter.add_order().await;
        }

        let end = Instant::now();
        let elapsed = end - start;

        // expect that the first 181 orders took 1s
        //  the remaining 3 sets of 4 should take another 3s
        assert!(elapsed > Duration::from_secs(4));
        assert!(elapsed < Duration::from_secs(5));
    }

    #[test]
    fn test_edit_order_penalties() {
        let cases = vec![
            (0, 6),
            (4, 6),
            (5, 5),
            (9, 5),
            (10, 4),
            (14, 4),
            (15, 3),
            (44, 3),
            (45, 2),
            (89, 2),
            (90, 0),
            (i64::MAX, 0),
        ];

        for (lifetime, expected) in cases {
            assert_eq!(
                expected,
                KrakenTradingRateLimiter::edit_order_penalty(lifetime)
            );
        }
    }

    #[test]
    fn test_cancel_order_penalties() {
        let cases = vec![
            (0, 8),
            (4, 8),
            (5, 6),
            (9, 6),
            (10, 5),
            (14, 5),
            (15, 4),
            (44, 4),
            (45, 2),
            (89, 2),
            (90, 1),
            (299, 1),
            (300, 0),
            (i64::MAX, 0),
        ];

        for (lifetime, expected) in cases {
            assert_eq!(
                expected,
                KrakenTradingRateLimiter::cancel_order_penalty(lifetime)
            );
        }
    }
}
