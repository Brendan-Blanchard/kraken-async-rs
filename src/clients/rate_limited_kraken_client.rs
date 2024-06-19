//! A rate-limited [KrakenClient]
use crate::clients::errors::ClientError;
use crate::clients::http_response_types::ResultErrorResponse;
use crate::clients::kraken_client::KrakenClient;
use crate::crypto::nonce_provider::NonceProvider;
use crate::rate_limiting::keyed_rate_limits::KeyedRateLimiter;
use crate::rate_limiting::trading_rate_limits::KrakenTradingRateLimiter;
use crate::request_types::*;
use crate::response_types::*;
use crate::secrets::secrets_provider::SecretsProvider;
use async_rate_limit::limiters::{RateLimiter, VariableCostRateLimiter};
use async_rate_limit::sliding_window::SlidingWindowRateLimiter;
use async_rate_limit::token_bucket::{TokenBucketRateLimiter, TokenBucketState};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use time::OffsetDateTime;
use tokio::sync::Mutex;

/// A [KrakenClient] implementation that decorates a provided client, and applies rate limiting
/// according to the Kraken API specs.
///
/// Loosely, this is:
/// - public endpoints are limited to 1 call per second
/// - private endpoints follow a token-bucket rate limiting scheme, with some endpoints having higher costs
/// - trading endpoints implement the Advanced version of Kraken's rate limiting scheme
///     - this includes tracking order lifetimes and applying penalties to rapid cancels and edits of orders
///
/// The exact rate limit values and replenishment schedule are determined by a user's
/// verification tier. Default new methods assume an `Intermediate` verification, so `Pro` users will
/// want to rely on methods that allow providing a custom verification tier if they want to take full
/// advantage of their increased rate limits (e.g. `new_with_verification_tier`).
///
/// Calls made that violate the rate limiting policy are made to wait asynchronously, but no error handling
/// is in place for receiving rate limit errors, these are to be handled/backed-off by the user.
///
/// Detailed documentation is available from several locations, including the [overview rate-limiting page],
/// [api rate-limiting page] and [trading rate-limiting page]. It's worth noting that the token
/// values used in this library are scaled to be 100x those of Kraken's documentation to keep them
/// as integers using semaphore permits instead of floating-point math.
///
/// [`RateLimitedKrakenClient`]s are cloneable, which results in a new client that shares the same
/// rate limiting state. This is useful for giving many services access to a client while ensuring
/// that all will jointly respect the rate limits of the exchange.
///
/// *Warning: This is not meant to be a comprehensive solution to all rate limiting, but is a best-effort
/// attempt to match the API's specifications. In some cases cloud providers, or server implementations
/// may inject random errors to prevent coordinated attacks or abuse. As such, this cannot anticipate
/// and mitigate all modes of failure.*
///
/// See examples/live_retrieving_recent_trades.rs for usage that relies on rate limiting preventing
/// request failures due to rapidly requesting public trade history.
///
/// [overview rate-limiting page]: https://docs.kraken.com/rest/#section/Rate-Limits/Matching-Engine-Rate-Limits
/// [api rate-limiting page]: https://support.kraken.com/hc/en-us/articles/206548367-What-are-the-API-rate-limits-#3
/// [trading rate-limiting page]: https://support.kraken.com/hc/en-us/articles/360045239571-Trading-rate-limits
#[derive(Debug, Clone)]
pub struct RateLimitedKrakenClient<C>
where
    C: KrakenClient,
{
    core_client: C,
    private_rate_limiter: TokenBucketRateLimiter,
    public_rate_limiter: SlidingWindowRateLimiter,
    trading_rate_limiter: KrakenTradingRateLimiter,
    pair_rate_limiter: KeyedRateLimiter<String>,
}

impl<C> KrakenClient for RateLimitedKrakenClient<C>
where
    C: KrakenClient,
{
    fn new(
        secrets_provider: Box<dyn SecretsProvider>,
        nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>>,
    ) -> RateLimitedKrakenClient<C> {
        RateLimitedKrakenClient {
            core_client: C::new(secrets_provider, nonce_provider),
            private_rate_limiter: Self::get_private_rate_limiter(VerificationTier::Intermediate),
            public_rate_limiter: Self::get_public_rate_limiter(),
            trading_rate_limiter: KrakenTradingRateLimiter::new(VerificationTier::Intermediate),
            pair_rate_limiter: KeyedRateLimiter::new(),
        }
    }

    fn new_with_url(
        secrets_provider: Box<dyn SecretsProvider>,
        nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>>,
        url: String,
    ) -> Self {
        RateLimitedKrakenClient {
            core_client: C::new_with_url(secrets_provider, nonce_provider, url),
            private_rate_limiter: Self::get_private_rate_limiter(VerificationTier::Intermediate),
            public_rate_limiter: Self::get_public_rate_limiter(),
            trading_rate_limiter: KrakenTradingRateLimiter::new(VerificationTier::Intermediate),
            pair_rate_limiter: KeyedRateLimiter::new(),
        }
    }

    async fn set_user_agent(&mut self, user_agent: String) {
        self.core_client.set_user_agent(user_agent).await;
    }

    async fn get_server_time(&mut self) -> Result<ResultErrorResponse<SystemTime>, ClientError> {
        self.public_rate_limiter.wait_until_ready().await;
        self.core_client.get_server_time().await
    }

    async fn get_system_status(
        &mut self,
    ) -> Result<ResultErrorResponse<SystemStatusInfo>, ClientError> {
        self.public_rate_limiter.wait_until_ready().await;
        self.core_client.get_system_status().await
    }

    async fn get_asset_info(
        &mut self,
        request: &AssetInfoRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, AssetInfo>>, ClientError> {
        self.public_rate_limiter.wait_until_ready().await;
        self.core_client.get_asset_info(request).await
    }

    async fn get_tradable_asset_pairs(
        &mut self,
        request: &TradableAssetPairsRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, TradableAssetPair>>, ClientError> {
        self.public_rate_limiter.wait_until_ready().await;
        self.core_client.get_tradable_asset_pairs(request).await
    }

    async fn get_ticker_information(
        &mut self,
        request: &TickerRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, RestTickerInfo>>, ClientError> {
        self.public_rate_limiter.wait_until_ready().await;
        self.core_client.get_ticker_information(request).await
    }

    async fn get_ohlc(
        &mut self,
        request: &OHLCRequest,
    ) -> Result<ResultErrorResponse<OhlcResponse>, ClientError> {
        self.pair_rate_limiter
            .wait_until_ready(request.pair.clone())
            .await;
        self.core_client.get_ohlc(request).await
    }

    async fn get_orderbook(
        &mut self,
        request: &OrderbookRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, Orderbook>>, ClientError> {
        self.public_rate_limiter.wait_until_ready().await;
        self.core_client.get_orderbook(request).await
    }

    async fn get_recent_trades(
        &mut self,
        request: &RecentTradesRequest,
    ) -> Result<ResultErrorResponse<RecentTrades>, ClientError> {
        self.pair_rate_limiter
            .wait_until_ready(request.pair.clone())
            .await;
        self.public_rate_limiter.wait_until_ready().await;
        self.core_client.get_recent_trades(request).await
    }

    async fn get_recent_spreads(
        &mut self,
        request: &RecentSpreadsRequest,
    ) -> Result<ResultErrorResponse<RecentSpreads>, ClientError> {
        self.public_rate_limiter.wait_until_ready().await;
        self.core_client.get_recent_spreads(request).await
    }

    async fn get_account_balance(
        &mut self,
    ) -> Result<ResultErrorResponse<AccountBalances>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.get_account_balance().await
    }

    async fn get_extended_balances(
        &mut self,
    ) -> Result<ResultErrorResponse<ExtendedBalances>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.get_extended_balances().await
    }

    async fn get_trade_balances(
        &mut self,
        request: &TradeBalanceRequest,
    ) -> Result<ResultErrorResponse<TradeBalances>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.get_trade_balances(request).await
    }

    async fn get_open_orders(
        &mut self,
        request: &OpenOrdersRequest,
    ) -> Result<ResultErrorResponse<OpenOrders>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.get_open_orders(request).await
    }

    async fn get_closed_orders(
        &mut self,
        request: &ClosedOrdersRequest,
    ) -> Result<ResultErrorResponse<ClosedOrders>, ClientError> {
        self.private_rate_limit(200).await;
        self.core_client.get_closed_orders(request).await
    }

    async fn query_orders_info(
        &mut self,
        request: &OrderRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, Order>>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.query_orders_info(request).await
    }

    async fn get_trades_history(
        &mut self,
        request: &TradesHistoryRequest,
    ) -> Result<ResultErrorResponse<TradesHistory>, ClientError> {
        self.private_rate_limit(200).await;
        self.core_client.get_trades_history(request).await
    }

    async fn query_trades_info(
        &mut self,
        request: &TradeInfoRequest,
    ) -> Result<ResultErrorResponse<TradesInfo>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.query_trades_info(request).await
    }

    async fn get_open_positions(
        &mut self,
        request: &OpenPositionsRequest,
    ) -> Result<ResultErrorResponse<OpenPositions>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.get_open_positions(request).await
    }

    async fn get_ledgers_info(
        &mut self,
        request: &LedgersInfoRequest,
    ) -> Result<ResultErrorResponse<LedgerInfo>, ClientError> {
        self.private_rate_limit(200).await;
        self.core_client.get_ledgers_info(request).await
    }

    async fn query_ledgers(
        &mut self,
        request: &QueryLedgerRequest,
    ) -> Result<ResultErrorResponse<QueryLedgerInfo>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.query_ledgers(request).await
    }

    async fn get_trade_volume(
        &mut self,
        request: &TradeVolumeRequest,
    ) -> Result<ResultErrorResponse<TradeVolume>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.get_trade_volume(request).await
    }

    async fn request_export_report(
        &mut self,
        request: &ExportReportRequest,
    ) -> Result<ResultErrorResponse<ExportReport>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.request_export_report(request).await
    }

    async fn get_export_report_status(
        &mut self,
        request: &ExportReportStatusRequest,
    ) -> Result<ResultErrorResponse<Vec<ExportReportStatus>>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.get_export_report_status(request).await
    }

    async fn retrieve_export_report(
        &mut self,
        request: &RetrieveExportReportRequest,
    ) -> Result<Vec<u8>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.retrieve_export_report(request).await
    }

    async fn delete_export_report(
        &mut self,
        request: &DeleteExportRequest,
    ) -> Result<ResultErrorResponse<DeleteExportReport>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.delete_export_report(request).await
    }

    async fn add_order(
        &mut self,
        request: &AddOrderRequest,
    ) -> Result<ResultErrorResponse<AddOrder>, ClientError> {
        self.trading_rate_limiter.add_order().await;
        let response = self.core_client.add_order(request).await;
        self.notify_add_order(&response, request.user_ref).await;

        response
    }

    async fn add_order_batch(
        &mut self,
        request: &AddBatchedOrderRequest,
    ) -> Result<ResultErrorResponse<AddOrderBatch>, ClientError> {
        self.trading_rate_limiter.add_order_batch(request).await;
        let response = self.core_client.add_order_batch(request).await;
        self.notify_add_order_batched(&response, request).await;

        response
    }

    async fn edit_order(
        &mut self,
        request: &EditOrderRequest,
    ) -> Result<ResultErrorResponse<OrderEdit>, ClientError> {
        self.trading_rate_limiter.edit_order(request).await;
        let response = self.core_client.edit_order(request).await;
        self.notify_edit_order(&response, request.user_ref).await;
        response
    }

    async fn cancel_order(
        &mut self,
        request: &CancelOrderRequest,
    ) -> Result<ResultErrorResponse<CancelOrder>, ClientError> {
        match &request.tx_id {
            IntOrString::Int(i) => {
                self.trading_rate_limiter.cancel_order_user_ref(i).await;
            }
            IntOrString::String(s) => {
                self.trading_rate_limiter.cancel_order_tx_id(s).await;
            }
        }

        self.core_client.cancel_order(request).await
    }

    async fn cancel_all_orders(&mut self) -> Result<ResultErrorResponse<CancelOrder>, ClientError> {
        self.core_client.cancel_all_orders().await
    }

    async fn cancel_all_orders_after(
        &mut self,
        request: &CancelAllOrdersAfterRequest,
    ) -> Result<ResultErrorResponse<CancelAllOrdersAfter>, ClientError> {
        self.core_client.cancel_all_orders_after(request).await
    }

    /// Clients can request to cancel in batches using both ref-ids produced by Kraken (Strings), or
    /// user-refs generated by the user (i64), which are known before the order is placed.
    async fn cancel_order_batch(
        &mut self,
        request: &CancelBatchOrdersRequest,
    ) -> Result<ResultErrorResponse<CancelOrder>, ClientError> {
        for order in &request.orders {
            match order {
                IntOrString::Int(user_ref) => {
                    self.trading_rate_limiter
                        .cancel_order_user_ref(user_ref)
                        .await
                }
                IntOrString::String(tx_id) => {
                    self.trading_rate_limiter.cancel_order_tx_id(tx_id).await
                }
            }
        }

        self.core_client.cancel_order_batch(request).await
    }

    async fn get_deposit_methods(
        &mut self,
        request: &DepositMethodsRequest,
    ) -> Result<ResultErrorResponse<Vec<DepositMethod>>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.get_deposit_methods(request).await
    }

    async fn get_deposit_addresses(
        &mut self,
        request: &DepositAddressesRequest,
    ) -> Result<ResultErrorResponse<Vec<DepositAddress>>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.get_deposit_addresses(request).await
    }

    async fn get_status_of_recent_deposits(
        &mut self,
        request: &StatusOfDepositWithdrawRequest,
    ) -> Result<ResultErrorResponse<DepositWithdrawResponse>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client
            .get_status_of_recent_deposits(request)
            .await
    }

    async fn get_withdrawal_methods(
        &mut self,
        request: &WithdrawalMethodsRequest,
    ) -> Result<ResultErrorResponse<Vec<WithdrawMethod>>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.get_withdrawal_methods(request).await
    }

    async fn get_withdrawal_addresses(
        &mut self,
        request: &WithdrawalAddressesRequest,
    ) -> Result<ResultErrorResponse<Vec<WithdrawalAddress>>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.get_withdrawal_addresses(request).await
    }

    async fn get_withdrawal_info(
        &mut self,
        request: &WithdrawalInfoRequest,
    ) -> Result<ResultErrorResponse<Withdrawal>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.get_withdrawal_info(request).await
    }

    async fn withdraw_funds(
        &mut self,
        request: &WithdrawFundsRequest,
    ) -> Result<ResultErrorResponse<ConfirmationRefId>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.withdraw_funds(request).await
    }

    async fn get_status_of_recent_withdrawals(
        &mut self,
        request: &StatusOfDepositWithdrawRequest,
    ) -> Result<ResultErrorResponse<Vec<DepositWithdrawal>>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client
            .get_status_of_recent_withdrawals(request)
            .await
    }

    async fn request_withdrawal_cancellation(
        &mut self,
        request: &WithdrawCancelRequest,
    ) -> Result<ResultErrorResponse<bool>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client
            .request_withdrawal_cancellation(request)
            .await
    }

    async fn request_wallet_transfer(
        &mut self,
        request: &WalletTransferRequest,
    ) -> Result<ResultErrorResponse<ConfirmationRefId>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.request_wallet_transfer(request).await
    }

    async fn create_sub_account(
        &mut self,
        request: &CreateSubAccountRequest,
    ) -> Result<ResultErrorResponse<bool>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.create_sub_account(request).await
    }

    async fn account_transfer(
        &mut self,
        request: &AccountTransferRequest,
    ) -> Result<ResultErrorResponse<AccountTransfer>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.account_transfer(request).await
    }

    async fn allocate_earn_funds(
        &mut self,
        request: &AllocateEarnFundsRequest,
    ) -> Result<ResultErrorResponse<bool>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.allocate_earn_funds(request).await
    }

    async fn deallocate_earn_funds(
        &mut self,
        request: &AllocateEarnFundsRequest,
    ) -> Result<ResultErrorResponse<bool>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.deallocate_earn_funds(request).await
    }

    async fn get_earn_allocation_status(
        &mut self,
        request: &EarnAllocationStatusRequest,
    ) -> Result<ResultErrorResponse<AllocationStatus>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.get_earn_allocation_status(request).await
    }

    async fn get_earn_deallocation_status(
        &mut self,
        request: &EarnAllocationStatusRequest,
    ) -> Result<ResultErrorResponse<AllocationStatus>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.get_earn_deallocation_status(request).await
    }

    async fn list_earn_strategies(
        &mut self,
        request: &ListEarnStrategiesRequest,
    ) -> Result<ResultErrorResponse<EarnStrategies>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.list_earn_strategies(request).await
    }

    async fn list_earn_allocations(
        &mut self,
        request: &ListEarnAllocationsRequest,
    ) -> Result<ResultErrorResponse<EarnAllocations>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.list_earn_allocations(request).await
    }

    async fn get_websockets_token(
        &mut self,
    ) -> Result<ResultErrorResponse<WebsocketToken>, ClientError> {
        self.private_rate_limit(100).await;
        self.core_client.get_websockets_token().await
    }
}

impl<C> RateLimitedKrakenClient<C>
where
    C: KrakenClient,
{
    /// Notify the trading rate limiter that an order was created at this time, this timestamp is
    /// used for determining the order's lifetime for edit and cancel penalties.
    async fn notify_add_order(
        &mut self,
        order_response: &Result<ResultErrorResponse<AddOrder>, ClientError>,
        user_ref: Option<i64>,
    ) {
        if let Ok(ResultErrorResponse {
            result: Some(result),
            ..
        }) = order_response
        {
            for tx_id in &result.tx_id {
                self.trading_rate_limiter
                    .notify_add_order(
                        tx_id.clone(),
                        OffsetDateTime::now_utc().unix_timestamp(),
                        user_ref,
                    )
                    .await;
            }
        }
    }

    /// Notify the trading rate limiter of all orders created in this batch so it can determine
    /// order lifetimes for edit and cancel penalties.
    async fn notify_add_order_batched(
        &mut self,
        order_response: &Result<ResultErrorResponse<AddOrderBatch>, ClientError>,
        request: &AddBatchedOrderRequest,
    ) {
        if let Ok(ResultErrorResponse {
            result: Some(result),
            ..
        }) = order_response
        {
            for (order, request) in result.orders.iter().zip(request.orders.iter()) {
                self.trading_rate_limiter
                    .notify_add_order(
                        order.tx_id.clone(),
                        OffsetDateTime::now_utc().unix_timestamp(),
                        request.user_ref,
                    )
                    .await
            }
        }
    }

    /// Notify the trading rate limiter of the edited order, since the new order has a fresh order
    /// lifetime.
    async fn notify_edit_order(
        &mut self,
        order_response: &Result<ResultErrorResponse<OrderEdit>, ClientError>,
        user_ref: Option<i64>,
    ) {
        if let Ok(ResultErrorResponse {
            result: Some(result),
            ..
        }) = order_response
        {
            self.trading_rate_limiter
                .notify_add_order(
                    result.tx_id.clone(),
                    OffsetDateTime::now_utc().unix_timestamp(),
                    user_ref,
                )
                .await
        }
    }
}

impl<C> RateLimitedKrakenClient<C>
where
    C: KrakenClient,
{
    /// Create a new rate limited client that delegates calls to any type that implements [KrakenClient].
    pub fn new_with_client(
        client: C,
        verification: VerificationTier,
    ) -> RateLimitedKrakenClient<C> {
        RateLimitedKrakenClient {
            core_client: client,
            private_rate_limiter: Self::get_private_rate_limiter(verification),
            public_rate_limiter: Self::get_public_rate_limiter(),
            trading_rate_limiter: KrakenTradingRateLimiter::new(verification),
            pair_rate_limiter: KeyedRateLimiter::new(),
        }
    }

    /// Create a new rate-limited client using the provided [SecretsProvider] and [NonceProvider]
    pub fn new_with_verification_tier(
        secrets_provider: Box<dyn SecretsProvider>,
        nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>>,
        verification: VerificationTier,
    ) -> Self {
        RateLimitedKrakenClient {
            core_client: C::new(secrets_provider, nonce_provider),
            private_rate_limiter: Self::get_private_rate_limiter(verification),
            public_rate_limiter: Self::get_public_rate_limiter(),
            trading_rate_limiter: KrakenTradingRateLimiter::new(verification),
            pair_rate_limiter: KeyedRateLimiter::new(),
        }
    }

    /// Create a new client, specifying the user's verification tier and the base URL.
    pub fn new_with_verification_tier_and_url(
        secrets_provider: Box<dyn SecretsProvider>,
        nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>>,
        url: String,
        verification: VerificationTier,
    ) -> Self {
        RateLimitedKrakenClient {
            core_client: C::new_with_url(secrets_provider, nonce_provider, url),
            private_rate_limiter: Self::get_private_rate_limiter(verification),
            public_rate_limiter: Self::get_public_rate_limiter(),
            trading_rate_limiter: KrakenTradingRateLimiter::new(verification),
            pair_rate_limiter: KeyedRateLimiter::new(),
        }
    }

    /// Get a private endpoint rate limiter, depending on the user's verification level.
    ///
    /// This implements a more involved scheme.
    pub fn get_private_rate_limiter(user_verification: VerificationTier) -> TokenBucketRateLimiter {
        // tokens are scaled 100x from Kraken's floating-point method to keep as integers
        match user_verification {
            VerificationTier::Intermediate => {
                let token_bucket_state = TokenBucketState::new(2000, 50, Duration::from_secs(1));
                TokenBucketRateLimiter::new(Arc::new(Mutex::new(token_bucket_state)))
            }
            VerificationTier::Pro => {
                let token_bucket_state = TokenBucketState::new(2000, 100, Duration::from_secs(1));
                TokenBucketRateLimiter::new(Arc::new(Mutex::new(token_bucket_state)))
            }
        }
    }

    /// Get a public rate limiter, which limits calls to 1 per second.
    pub fn get_public_rate_limiter() -> SlidingWindowRateLimiter {
        SlidingWindowRateLimiter::new(Duration::from_secs(1), 1)
    }

    async fn private_rate_limit(&mut self, cost: usize) {
        self.private_rate_limiter.wait_with_cost(cost).await
    }
}

#[cfg(test)]
mod tests {
    use crate::clients::core_kraken_client::CoreKrakenClient;
    use crate::clients::kraken_client::endpoints::KRAKEN_BASE_URL;
    use crate::clients::kraken_client::KrakenClient;
    use crate::clients::rate_limited_kraken_client::RateLimitedKrakenClient;
    use crate::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
    use crate::secrets::secrets_provider::StaticSecretsProvider;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[test]
    fn client_creates() {
        let secrets_provider = StaticSecretsProvider::new("", "");
        let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> =
            Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));
        let client: RateLimitedKrakenClient<CoreKrakenClient> =
            RateLimitedKrakenClient::new(Box::new(secrets_provider), nonce_provider);

        assert_eq!(client.core_client.api_url, KRAKEN_BASE_URL);
    }
}
