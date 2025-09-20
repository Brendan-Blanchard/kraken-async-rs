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
        secrets_provider: Box<Arc<Mutex<dyn SecretsProvider>>>,
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
        secrets_provider: Box<Arc<Mutex<dyn SecretsProvider>>>,
        nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>>,
        url: impl ToString,
    ) -> Self {
        RateLimitedKrakenClient {
            core_client: C::new_with_url(secrets_provider, nonce_provider, url),
            private_rate_limiter: Self::get_private_rate_limiter(VerificationTier::Intermediate),
            public_rate_limiter: Self::get_public_rate_limiter(),
            trading_rate_limiter: KrakenTradingRateLimiter::new(VerificationTier::Intermediate),
            pair_rate_limiter: KeyedRateLimiter::new(),
        }
    }

    fn new_with_tracing(
        secrets_provider: Box<Arc<Mutex<dyn SecretsProvider>>>,
        nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>>,
        trace_inbound: bool,
    ) -> Self {
        RateLimitedKrakenClient {
            core_client: C::new_with_tracing(secrets_provider, nonce_provider, trace_inbound),
            private_rate_limiter: Self::get_private_rate_limiter(VerificationTier::Intermediate),
            public_rate_limiter: Self::get_public_rate_limiter(),
            trading_rate_limiter: KrakenTradingRateLimiter::new(VerificationTier::Intermediate),
            pair_rate_limiter: KeyedRateLimiter::new(),
        }
    }

    async fn set_user_agent(&mut self, user_agent: impl ToString) {
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

    async fn get_order_amends(
        &mut self,
        request: &OrderAmendsRequest,
    ) -> Result<ResultErrorResponse<OrderAmends>, ClientError> {
        self.private_rate_limiter.wait_with_cost(100).await;
        self.core_client.get_order_amends(request).await
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
        self.notify_add_order(&response, request.user_ref, &request.client_order_id)
            .await;

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

    async fn amend_order(
        &mut self,
        request: &AmendOrderRequest,
    ) -> Result<ResultErrorResponse<AmendOrder>, ClientError> {
        self.trading_rate_limiter
            .amend_order(&request.tx_id, &request.client_order_id)
            .await;
        let response = self.core_client.amend_order(request).await;
        self.notify_amend_order(&request.tx_id, &request.client_order_id.clone())
            .await;

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
        client_order_id: &Option<String>,
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
                        client_order_id,
                    )
                    .await;
            }
        }
    }

    /// Notify the trading rate limiter that an order was amended at this time, this timestamp is
    /// used for determining the order's lifetime for amend, edit, and cancel penalties.
    async fn notify_amend_order(
        &mut self,
        tx_id: &Option<String>,
        client_order_id: &Option<String>,
    ) {
        self.trading_rate_limiter
            .notify_amend_order(
                tx_id,
                OffsetDateTime::now_utc().unix_timestamp(),
                client_order_id,
            )
            .await;
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
                        &request.client_order_id,
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
                    &None,
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
        secrets_provider: Box<Arc<Mutex<dyn SecretsProvider>>>,
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
        secrets_provider: Box<Arc<Mutex<dyn SecretsProvider>>>,
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
    use crate::clients::kraken_client::KrakenClient;
    use crate::clients::kraken_client::endpoints::KRAKEN_BASE_URL;
    use crate::clients::rate_limited_kraken_client::RateLimitedKrakenClient;
    use crate::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
    use crate::request_types::{
        AccountTransferRequest, AddBatchedOrderRequest, AddOrderRequest, AllocateEarnFundsRequest,
        AmendOrderRequest, AssetInfoRequestBuilder, BatchedOrderRequest, CancelBatchOrdersRequest,
        CancelOrderRequest, CandlestickInterval, ClosedOrdersRequestBuilder,
        CreateSubAccountRequest, DeleteExportRequest, DeleteExportType, DepositAddressesRequest,
        DepositMethodsRequest, EarnAllocationStatusRequest, EditOrderRequest, ExportReportRequest,
        ExportReportStatusRequest, IntOrString, LedgersInfoRequest, ListEarnAllocationsRequest,
        ListEarnStrategiesRequest, OHLCRequest, OpenOrdersRequest, OpenPositionsRequest,
        OrderFlags, OrderRequest, OrderbookRequest, QueryLedgerRequest, RecentSpreadsRequest,
        RecentTradesRequest, ReportFormatType, ReportType, RetrieveExportReportRequest,
        StatusOfDepositWithdrawRequest, StringCSV, TickerRequest, TradableAssetPairsRequest,
        TradeBalanceRequest, TradeInfoRequest, TradeVolumeRequest, TradesHistoryRequest,
        WalletTransferRequest, WithdrawCancelRequest, WithdrawFundsRequest,
        WithdrawalAddressesRequest, WithdrawalInfoRequest, WithdrawalMethodsRequest,
    };
    use crate::response_types::VerificationTier::{Intermediate, Pro};
    use crate::response_types::{AddOrder, BuySell, OrderFlag, OrderType, VerificationTier};
    use crate::secrets::secrets_provider::StaticSecretsProvider;
    use crate::test_data::TestRateLimitedClient;
    use crate::test_data::public_response_json::get_server_time_json;
    use crate::test_data::{
        get_null_secrets_provider, get_rate_limit_test_client, get_rate_limit_test_client_err,
    };
    use crate::test_rate_limited_endpoint;
    use rust_decimal_macros::dec;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::sync::Mutex;
    use tokio::time::Instant;
    use tokio::time::pause;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn client_creates() {
        let secrets_provider = StaticSecretsProvider::new("", "");
        let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> =
            Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));
        let client: RateLimitedKrakenClient<CoreKrakenClient> = RateLimitedKrakenClient::new(
            Box::new(Arc::new(Mutex::new(secrets_provider))),
            nonce_provider,
        );

        assert_eq!(client.core_client.api_url, KRAKEN_BASE_URL);
    }

    #[tokio::test]
    async fn client_user_agent() {
        let secrets_provider = get_null_secrets_provider();
        let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> =
            Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));
        let mock_server = MockServer::start().await;
        let mut client: RateLimitedKrakenClient<CoreKrakenClient> =
            RateLimitedKrakenClient::new_with_url(
                secrets_provider,
                nonce_provider,
                mock_server.uri(),
            );

        Mock::given(method("GET"))
            .and(path("/0/public/Time"))
            .and(header("user-agent", "KrakenAsyncRsClient"))
            .respond_with(ResponseTemplate::new(200).set_body_json(get_server_time_json()))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _resp = client.get_server_time().await;
        mock_server.verify().await;

        client.set_user_agent("Strategy#1".to_string()).await;

        Mock::given(method("GET"))
            .and(path("/0/public/Time"))
            .and(header("user-agent", "Strategy#1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(get_server_time_json()))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _resp = client.get_server_time().await;
        mock_server.verify().await;
    }

    #[tokio::test]
    async fn test_system_public_endpoints() {
        pause();
        let n_calls = 7;

        // n calls are expected to take just over ~n-1 seconds to complete
        test_rate_limited_endpoint!(get_server_time, n_calls, n_calls - 1, n_calls, Intermediate);

        test_rate_limited_endpoint!(
            get_system_status,
            n_calls,
            n_calls - 1,
            n_calls,
            Intermediate
        );
    }

    #[tokio::test]
    async fn test_get_asset_info() {
        pause();
        let n_calls = 7;

        let pairs = StringCSV::new(vec![
            "XBT".to_string(),
            "ETH".to_string(),
            "ZUSD".to_string(),
        ]);
        let request = AssetInfoRequestBuilder::new()
            .asset(pairs)
            .asset_class("currency".into())
            .build();

        // n calls are expected to take just over ~n-1 seconds to complete
        test_rate_limited_endpoint!(
            get_asset_info,
            n_calls,
            n_calls - 1,
            n_calls,
            Intermediate,
            &request
        );
    }

    #[tokio::test]
    async fn test_get_tradable_asset_pairs() {
        pause();
        let n_calls = 7;

        let pairs = StringCSV::new(vec!["ETHUSD".to_string()]);
        let request = TradableAssetPairsRequest::builder().pair(pairs).build();

        // n calls are expected to take just over ~n-1 seconds to complete
        test_rate_limited_endpoint!(
            get_tradable_asset_pairs,
            n_calls,
            n_calls - 1,
            n_calls,
            Intermediate,
            &request
        );
    }

    #[tokio::test]
    async fn test_get_ticker_information() {
        pause();
        let n_calls = 7;

        let pairs = StringCSV::new(vec![
            "BTCUSD".to_string(),
            "ETHUSD".to_string(),
            "USDCUSD".to_string(),
        ]);
        let request = TickerRequest::builder().pair(pairs).build();

        // n calls are expected to take just over ~n-1 seconds to complete
        test_rate_limited_endpoint!(
            get_ticker_information,
            n_calls,
            n_calls - 1,
            n_calls,
            Intermediate,
            &request
        );
    }

    #[tokio::test]
    async fn test_get_ohlc_and_recent_trades() {
        pause();
        let n_calls = 7;

        let ohlc_request = OHLCRequest::builder("XETHZUSD".to_string())
            .interval(CandlestickInterval::Hour)
            .build();

        let trades_request = RecentTradesRequest::builder("XXBTZUSD".to_string())
            .count(10)
            .build();

        let secrets_provider = get_null_secrets_provider();
        let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> =
            Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));

        let mut client: TestRateLimitedClient = RateLimitedKrakenClient::new_with_verification_tier(
            secrets_provider,
            nonce_provider,
            Pro,
        );

        let start = Instant::now();

        // calling both in parallel should be fine, since they request different pairs
        for _ in 0..n_calls {
            let _ = client.get_ohlc(&ohlc_request).await;
            let _ = client.get_recent_trades(&trades_request).await;
        }

        let end = Instant::now();
        let elapsed = end - start;

        println!("{elapsed:?}");

        assert!(elapsed > Duration::from_secs(n_calls - 1));
        assert!(elapsed < Duration::from_secs(n_calls));
    }

    #[tokio::test]
    async fn test_get_orderbook() {
        pause();
        let n_calls = 7;

        let request = OrderbookRequest::builder("XXBTZUSD".to_string())
            .count(10)
            .build();

        // n calls are expected to take just over ~n-1 seconds to complete
        test_rate_limited_endpoint!(
            get_orderbook,
            n_calls,
            n_calls - 1,
            n_calls,
            Intermediate,
            &request
        );
    }

    #[tokio::test]
    async fn test_get_recent_trades() {
        pause();
        let n_calls = 7;

        let request = RecentTradesRequest::builder("XXBTZUSD".to_string())
            .count(10)
            .build();

        // n calls are expected to take just over ~n-1 seconds to complete
        test_rate_limited_endpoint!(
            get_recent_trades,
            n_calls,
            n_calls - 1,
            n_calls,
            Intermediate,
            &request
        );
    }

    #[tokio::test]
    async fn test_get_recent_spreads() {
        pause();
        let n_calls = 7;

        let request = RecentSpreadsRequest::builder("XXBTZUSD".to_string())
            .since(0)
            .build();
        // n calls are expected to take just over ~n-1 seconds to complete
        test_rate_limited_endpoint!(
            get_recent_spreads,
            n_calls,
            n_calls - 1,
            n_calls,
            Intermediate,
            &request
        );
    }

    #[tokio::test]
    async fn test_get_account_balance() {
        pause();

        // 22 calls costs 2200, requiring 4s to replenish @ 50/s
        test_rate_limited_endpoint!(get_account_balance, 22, 4, 5, Intermediate);
    }

    #[tokio::test]
    async fn test_get_extended_balance() {
        pause();

        // 22 calls costs 2200, requiring 2s to replenish @ 100/s
        test_rate_limited_endpoint!(get_extended_balances, 22, 2, 3, Pro);
    }

    #[tokio::test]
    async fn test_get_trade_balances() {
        pause();

        let request = TradeBalanceRequest::builder()
            .asset("XXBTZUSD".to_string())
            .build();

        // 26 calls costs 2600, requiring 6s to replenish @ 100/s
        test_rate_limited_endpoint!(get_trade_balances, 26, 6, 7, Pro, &request);
    }

    #[tokio::test]
    async fn test_get_open_orders() {
        pause();

        let request = OpenOrdersRequest::builder().trades(true).build();

        // 23 calls costs 2300, requiring 6s to replenish @ 50/s
        test_rate_limited_endpoint!(get_open_orders, 23, 6, 7, Intermediate, &request);
    }

    #[tokio::test]
    async fn test_get_closed_orders() {
        pause();

        let request = ClosedOrdersRequestBuilder::new()
            .trades(true)
            .start(12340000)
            .build();

        // 13 calls costs 2600, requiring 6s to replenish @ 100/s
        test_rate_limited_endpoint!(get_closed_orders, 13, 6, 7, Pro, &request);
    }

    #[tokio::test]
    async fn test_query_orders_info() {
        pause();

        let tx_ids = StringCSV::new(vec!["uuid_1".to_string()]);

        let request = OrderRequest::builder(tx_ids)
            .trades(true)
            .consolidate_taker(false)
            .build();

        // 26 calls costs 2600, requiring 12s to replenish @ 50/s
        test_rate_limited_endpoint!(query_orders_info, 26, 12, 13, Intermediate, &request);
    }

    #[tokio::test]
    async fn test_get_trades_history() {
        pause();

        let request = TradesHistoryRequest::builder()
            .start(0)
            .end(1234)
            .trades(true)
            .consolidate_taker(false)
            .build();

        // 14 calls costs 2800, requiring 8s to replenish @ 100/s
        test_rate_limited_endpoint!(get_trades_history, 14, 8, 9, Pro, &request);
    }

    #[tokio::test]
    async fn test_query_trades_info() {
        pause();

        let tx_ids = StringCSV::new(vec!["some-unique-id".to_string()]);

        let request = TradeInfoRequest::builder(tx_ids).trades(true).build();

        // 25 calls costs 2500, requiring 10s to replenish @ 50/s
        test_rate_limited_endpoint!(query_trades_info, 25, 10, 11, Intermediate, &request);
    }

    #[tokio::test]
    async fn test_get_open_positions() {
        pause();

        let request = OpenPositionsRequest::builder()
            .do_calcs(true)
            .consolidation("market".to_string())
            .build();

        // 25 calls costs 2500, requiring 5s to replenish @ 100/s
        test_rate_limited_endpoint!(get_open_positions, 25, 5, 6, Pro, &request);
    }

    #[tokio::test]
    async fn test_get_ledgers_info() {
        pause();

        let request = LedgersInfoRequest::builder()
            .start(0)
            .asset(StringCSV(vec!["all".into()]))
            .build();

        // 12 calls costs 2400, requiring 8s to replenish @ 50/s
        test_rate_limited_endpoint!(get_ledgers_info, 12, 8, 9, Intermediate, &request);
    }

    #[tokio::test]
    async fn test_query_ledgers() {
        pause();

        let request = QueryLedgerRequest::builder(StringCSV(vec!["51AHCZ-XXZ64-YW34UP".into()]))
            .trades(true)
            .build();

        // 24 calls costs 2400, requiring 4s to replenish @ 100/s
        test_rate_limited_endpoint!(query_ledgers, 24, 4, 5, Pro, &request);
    }

    #[tokio::test]
    async fn test_get_trade_volume() {
        pause();

        let request = TradeVolumeRequest::builder()
            .pair(StringCSV(vec!["XXBTZUSD".to_string()]))
            .build();

        // 24 calls costs 2400, requiring 8s to replenish @ 100/s
        test_rate_limited_endpoint!(get_trade_volume, 24, 8, 9, Intermediate, &request);
    }

    #[tokio::test]
    async fn test_request_export_report() {
        pause();

        let request = ExportReportRequest::builder(ReportType::Ledgers, "TestExport".to_string())
            .format(ReportFormatType::Csv)
            .build();

        // 24 calls costs 2400, requiring 4s to replenish @ 100/s
        test_rate_limited_endpoint!(request_export_report, 24, 4, 5, Pro, &request);
    }

    #[tokio::test]
    async fn test_get_export_report_status() {
        pause();

        let request = ExportReportStatusRequest::builder(ReportType::Trades).build();

        // 27 calls costs 2700, requiring 14s to replenish @ 50/s
        test_rate_limited_endpoint!(get_export_report_status, 27, 14, 15, Intermediate, &request);
    }

    #[tokio::test]
    async fn test_retrieve_export_report() {
        pause();

        let request =
            RetrieveExportReportRequest::builder("HI1M0S-BCRBJ-P01V9R".to_string()).build();

        // 24 calls costs 2400, requiring 4s to replenish @ 100/s
        test_rate_limited_endpoint!(retrieve_export_report, 24, 4, 5, Pro, &request);
    }

    #[tokio::test]
    async fn test_delete_export_report() {
        pause();

        let request =
            DeleteExportRequest::builder("54E7".to_string(), DeleteExportType::Delete).build();

        // 24 calls costs 2400, requiring 8s to replenish @ 50/s
        test_rate_limited_endpoint!(delete_export_report, 24, 8, 9, Intermediate, &request);
    }

    #[tokio::test]
    async fn test_adding_order_limits() {
        pause();
        let mut client = get_rate_limit_test_client(Pro);
        let mut client_err = get_rate_limit_test_client_err(Pro);

        let start = Instant::now();

        let request = get_add_order_request();

        // the first 180 orders exhaust all tokens, the remaining 15 require 4s of waiting
        //  since the replenishment rate is 375 tokens/s * 4s = 1500
        for _ in 0..(180 + 15) {
            let _ = client.add_order(&request).await;
            let _ = client_err.add_order(&request).await;
        }

        let end = Instant::now();
        let elapsed = end - start;
        println!("{elapsed:?}");

        assert!(elapsed > Duration::from_secs(4));
        assert!(elapsed < Duration::from_secs(5));
    }

    #[tokio::test]
    async fn test_amend_order_max_penalty() {
        pause();
        let verification = Intermediate;
        let mut client = get_rate_limit_test_client(verification);

        let orders = max_out_rate_limits(&mut client, verification).await;

        let amend_start = Instant::now();

        // 4 instant amends costs 400 each, for 1600 total, 1600 / 234 = ~6.83 (requires 7s wait)
        for i in 0..4 {
            let amend_request = get_amend_for_order(&orders, i);
            let _ = client.amend_order(&amend_request).await;
        }

        let amend_elapsed = amend_start.elapsed();
        println!("{amend_elapsed:?}");

        assert!(amend_elapsed > Duration::from_secs(7));
        assert!(amend_elapsed < Duration::from_secs(8));
    }

    fn get_amend_for_order(orders: &[AddOrder], i: usize) -> AmendOrderRequest {
        AmendOrderRequest::builder()
            .tx_id(orders.get(i).unwrap().tx_id.first().unwrap().clone()) // TODO: cleanup
            .build()
    }

    #[tokio::test]
    async fn test_add_order_batch_limits() {
        pause();
        let mut client = get_rate_limit_test_client(Pro);
        let mut client_err = get_rate_limit_test_client_err(Pro);

        let start = Instant::now();

        let request = get_batched_order_request(16);

        // batched order of 16 should cost (1 + n / 2) * 100 = 900 each, so 21 * 900 = 18,900
        // replenishing the 900 after the pro limit should take 3s
        for _ in 0..21 {
            let _ = client.add_order_batch(&request).await;
            let _ = client_err.add_order_batch(&request).await;
        }

        let end = Instant::now();
        let elapsed = end - start;
        println!("{elapsed:?}");

        assert!(elapsed > Duration::from_secs(3));
        assert!(elapsed < Duration::from_secs(4));
    }

    #[tokio::test]
    async fn test_edit_order_max_penalty() {
        pause();
        let verification = Pro;
        let mut client = get_rate_limit_test_client(verification);
        let mut client_err = get_rate_limit_test_client_err(Pro);

        let orders = max_out_rate_limits(&mut client, verification).await;

        let edit_start = Instant::now();

        // 6 instant edits costs 700 each, for 4200 total, 4200 / 375 = ~11.23 (requires 12s wait)
        for i in 0..6 {
            let edit_request = edit_from_order(orders.get(i).unwrap());
            let _ = client.edit_order(&edit_request).await;
        }

        // initiating more edits for the error client has no effect, since each err return did not add
        //  an order id / lifetime
        for i in 0..12 {
            let edit_request = edit_from_order(orders.get(i).unwrap());
            let _ = client_err.edit_order(&edit_request).await;
        }

        let edit_end = Instant::now();
        let edit_elapsed = edit_end - edit_start;
        println!("{edit_elapsed:?}");

        assert!(edit_elapsed > Duration::from_secs(12));
        assert!(edit_elapsed < Duration::from_secs(13));
    }

    #[tokio::test]
    async fn test_cancel_order_max_penalty() {
        pause();
        let verification = Intermediate;
        let mut client = get_rate_limit_test_client(verification);
        let mut client_err = get_rate_limit_test_client_err(Pro);

        let orders = max_out_rate_limits(&mut client, verification).await;

        let edit_start = Instant::now();

        // 4 instant cancels costs 800 each, for 3200 total, 3200 / 234 = ~13.67 (requires 14s wait)
        for i in 0..4 {
            let cancel_request = cancel_from_order(orders.get(i).unwrap());
            let _ = client.cancel_order(&cancel_request).await;
            let _ = client_err.cancel_order(&cancel_request).await;
        }

        // initiating more cancels for the error client has no effect, since each err return did not add
        //  an order id / lifetime
        for i in 0..12 {
            let cancel_request = cancel_from_order(orders.get(i).unwrap());
            let _ = client_err.cancel_order(&cancel_request).await;
        }

        let edit_end = Instant::now();
        let edit_elapsed = edit_end - edit_start;
        println!("{edit_elapsed:?}");

        assert!(edit_elapsed > Duration::from_secs(14));
        assert!(edit_elapsed < Duration::from_secs(15));
    }

    #[tokio::test]
    async fn test_cancel_order_batch_with_max_penalty() {
        pause();
        let verification = Intermediate;
        let mut client = get_rate_limit_test_client(verification);
        let mut client_err = get_rate_limit_test_client_err(Pro);

        let mut orders = max_out_rate_limits(&mut client, verification).await;

        let edit_start = Instant::now();

        let mut order_ids = Vec::new();
        for i in 0..4 {
            let id = IntOrString::String(orders.get(i).unwrap().tx_id.first().unwrap().clone());
            order_ids.push(id);
        }

        let user_ref_request = get_add_order_request_user_ref();
        orders.push(
            client
                .add_order(&user_ref_request)
                .await
                .unwrap()
                .result
                .unwrap(),
        );
        order_ids.push(IntOrString::Int(user_ref_request.user_ref.unwrap()));

        let batch_cancel_request = CancelBatchOrdersRequest {
            orders: order_ids,
            client_order_ids: None,
        };

        // 1 additional order w/ user ref costs 100, 5 instant cancels cost 800 each, for 4100 total,
        // making 4100 / 234 = ~17.52 (requires 18s wait)
        let _ = client.cancel_order_batch(&batch_cancel_request).await;

        // failures don't add anything to wait
        for _ in 0..5 {
            let _ = client_err.cancel_order_batch(&batch_cancel_request).await;
        }

        let edit_end = Instant::now();
        let edit_elapsed = edit_end - edit_start;
        println!("{edit_elapsed:?}");

        assert!(edit_elapsed > Duration::from_secs(18));
        assert!(edit_elapsed < Duration::from_secs(19));
    }

    /// Depending on the verification tier, submit enough orders to empty the rate limit bucket and
    /// return the created orders. Also checks that it has not exceeded the limits (executes in < 10ms).
    async fn max_out_rate_limits(
        client: &mut TestRateLimitedClient,
        verification_tier: VerificationTier,
    ) -> Vec<AddOrder> {
        let start = Instant::now();

        let request = get_add_order_request();

        let n_orders = match verification_tier {
            Intermediate => 125,
            Pro => 180,
        };

        // the first 180 orders exhaust all tokens
        let mut orders = Vec::new();
        for _ in 0..n_orders {
            let order = client.add_order(&request).await.unwrap().result.unwrap();
            orders.push(order);
        }

        let end = Instant::now();
        let elapsed = end - start;
        println!("{elapsed:?}");

        assert!(elapsed >= Duration::from_secs(0));
        assert!(elapsed < Duration::from_millis(10));
        orders
    }

    fn get_add_order_request() -> AddOrderRequest {
        let order_flags =
            OrderFlags::new(vec![OrderFlag::NoMarketPriceProtection, OrderFlag::Post]);

        AddOrderRequest::builder(
            OrderType::Market,
            BuySell::Buy,
            dec!(5.0),
            "USDCUSD".to_string(),
        )
        .order_flags(order_flags)
        .price(dec!(0.90))
        .build()
    }

    fn get_add_order_request_user_ref() -> AddOrderRequest {
        let order_flags =
            OrderFlags::new(vec![OrderFlag::NoMarketPriceProtection, OrderFlag::Post]);

        AddOrderRequest::builder(
            OrderType::Market,
            BuySell::Buy,
            dec!(5.0),
            "USDCUSD".to_string(),
        )
        .user_ref(42)
        .order_flags(order_flags)
        .price(dec!(0.90))
        .build()
    }

    fn get_batched_order_request(n_orders: u64) -> AddBatchedOrderRequest {
        let mut orders = Vec::new();

        for _ in 0..n_orders {
            let order = BatchedOrderRequest::builder(OrderType::Limit, BuySell::Buy, dec!(5.1))
                .price(dec!(0.9))
                .start_time("0".to_string())
                .expire_time("+5".to_string())
                .build();

            orders.push(order);
        }

        AddBatchedOrderRequest::builder(orders, "USDCUSD".to_string()).build()
    }

    fn edit_from_order(order: &AddOrder) -> EditOrderRequest {
        let edit_request = EditOrderRequest {
            user_ref: None,
            tx_id: order.tx_id.first().unwrap().clone(),
            volume: dec!(0),
            display_volume: None,
            pair: "".to_string(),
            price: None,
            price_2: None,
            order_flags: None,
            deadline: None,
            cancel_response: None,
            validate: None,
        };
        edit_request
    }

    fn cancel_from_order(order: &AddOrder) -> CancelOrderRequest {
        CancelOrderRequest {
            tx_id: IntOrString::String(order.tx_id.first().unwrap().clone()),
            client_order_id: None,
        }
    }

    #[tokio::test]
    async fn test_get_deposit_methods() {
        pause();

        let request = DepositMethodsRequest::builder("ETH".to_string()).build();

        // 24 calls costs 2400, requiring 4s to replenish @ 100/s
        test_rate_limited_endpoint!(get_deposit_methods, 24, 4, 5, Pro, &request);
    }

    #[tokio::test]
    async fn test_get_deposit_addresses() {
        pause();

        let request = DepositAddressesRequest::builder("BTC".to_string(), "Bitcoin".to_string())
            .is_new(true)
            .build();

        // 24 calls costs 2400, requiring 8s to replenish @ 50/s
        test_rate_limited_endpoint!(get_deposit_addresses, 24, 8, 9, Intermediate, &request);
    }

    #[tokio::test]
    async fn test_get_status_of_recent_deposits() {
        pause();

        let request = StatusOfDepositWithdrawRequest::builder()
            .asset_class("currency".to_string())
            .build();

        // 26 calls costs 2600, requiring 6s to replenish @ 100/s
        test_rate_limited_endpoint!(get_status_of_recent_deposits, 26, 6, 7, Pro, &request);
    }

    #[tokio::test]
    async fn test_get_withdrawal_methods() {
        pause();

        let request = WithdrawalMethodsRequest::builder()
            .asset_class("currency".to_string())
            .build();

        // 26 calls costs 2600, requiring 12s to replenish @ 50/s
        test_rate_limited_endpoint!(get_withdrawal_methods, 26, 12, 13, Intermediate, &request);
    }

    #[tokio::test]
    async fn test_get_withdrawal_addresses() {
        pause();

        let request = WithdrawalAddressesRequest::builder()
            .asset_class("currency".to_string())
            .build();

        // 25 calls costs 2500, requiring 5s to replenish @ 100/s
        test_rate_limited_endpoint!(get_withdrawal_addresses, 25, 5, 6, Pro, &request);
    }

    #[tokio::test]
    async fn test_get_withdrawal_info() {
        pause();

        let request = WithdrawalInfoRequest::builder(
            "XBT".to_string(),
            "Greenlisted Address".to_string(),
            dec!(0.1),
        )
        .build();

        // 25 calls costs 2500, requiring 5s to replenish @ 100/s
        test_rate_limited_endpoint!(get_withdrawal_info, 25, 5, 6, Pro, &request);
    }

    #[tokio::test]
    async fn test_withdraw_funds() {
        pause();

        let request = WithdrawFundsRequest::builder(
            "XBT".to_string(),
            "Greenlisted Address".to_string(),
            dec!(0.1),
        )
        .max_fee(dec!(0.00001))
        .build();

        // 25 calls costs 2500, requiring 10s to replenish @ 50/s
        test_rate_limited_endpoint!(withdraw_funds, 25, 10, 11, Intermediate, &request);
    }

    #[tokio::test]
    async fn test_get_status_of_recent_withdrawals() {
        pause();

        let request = StatusOfDepositWithdrawRequest::builder()
            .asset_class("currency".to_string())
            .build();

        // 25 calls costs 2500, requiring 5s to replenish @ 100/s
        test_rate_limited_endpoint!(get_status_of_recent_withdrawals, 25, 5, 6, Pro, &request);
    }

    #[tokio::test]
    async fn test_request_withdrawal_cancellation() {
        pause();

        let request = WithdrawCancelRequest::builder("XBT".to_string(), "uuid".to_string()).build();

        // 27 calls costs 2700, requiring 14s to replenish @ 50/s
        test_rate_limited_endpoint!(
            request_withdrawal_cancellation,
            27,
            14,
            15,
            Intermediate,
            &request
        );
    }

    #[tokio::test]
    async fn test_request_wallet_transfer() {
        pause();

        let request = WalletTransferRequest::builder(
            "XBT".to_string(),
            "Account One".to_string(),
            "Account Two".to_string(),
            dec!(0.25),
        )
        .build();

        // 27 calls costs 2700, requiring 7s to replenish @ 100/s
        test_rate_limited_endpoint!(request_wallet_transfer, 27, 7, 8, Pro, &request);
    }

    #[tokio::test]
    async fn test_create_sub_account() {
        pause();

        let request =
            CreateSubAccountRequest::builder("username".to_string(), "user@mail.com".to_string())
                .build();

        // 24 calls costs 2400, requiring 4s to replenish @ 100/s
        test_rate_limited_endpoint!(create_sub_account, 24, 4, 5, Pro, &request);
    }

    #[tokio::test]
    async fn test_account_transfer() {
        pause();

        let request = AccountTransferRequest::builder(
            "BTC".to_string(),
            dec!(1031.2008),
            "SourceAccount".to_string(),
            "DestAccount".to_string(),
        )
        .build();

        // 24 calls costs 2400, requiring 8s to replenish @ 50/s
        test_rate_limited_endpoint!(account_transfer, 24, 8, 9, Intermediate, &request);
    }

    #[tokio::test]
    async fn test_allocate_earn_funds() {
        pause();

        let request =
            AllocateEarnFundsRequest::builder(dec!(10.123), "W38S2C-Y1E0R-DUFM2T".to_string())
                .build();

        // 24 calls costs 2400, requiring 4s to replenish @ 100/s
        test_rate_limited_endpoint!(allocate_earn_funds, 24, 4, 5, Pro, &request);
    }

    #[tokio::test]
    async fn test_deallocate_earn_funds() {
        pause();

        let request =
            AllocateEarnFundsRequest::builder(dec!(10.123), "W38S2C-Y1E0R-DUFM2T".to_string())
                .build();

        // 24 calls costs 2400, requiring 8s to replenish @ 50/s
        test_rate_limited_endpoint!(deallocate_earn_funds, 24, 8, 9, Intermediate, &request);
    }

    #[tokio::test]
    async fn test_get_allocation_status() {
        pause();

        let request =
            EarnAllocationStatusRequest::builder("W38S2C-Y1E0R-DUFM2T".to_string()).build();

        // 24 calls costs 2400, requiring 8s to replenish @ 50/s
        test_rate_limited_endpoint!(get_earn_allocation_status, 24, 8, 9, Intermediate, &request);
    }

    #[tokio::test]
    async fn test_get_deallocation_status() {
        pause();

        let request =
            EarnAllocationStatusRequest::builder("W38S2C-Y1E0R-DUFM2T".to_string()).build();

        // 24 calls costs 2400, requiring 8s to replenish @ 50/s
        test_rate_limited_endpoint!(
            get_earn_deallocation_status,
            24,
            8,
            9,
            Intermediate,
            &request
        );
    }

    #[tokio::test]
    async fn test_list_earn_strategies() {
        pause();

        let request = ListEarnStrategiesRequest::builder()
            .limit(64)
            .ascending(true)
            .build();

        // 24 calls costs 2400, requiring 4s to replenish @ 100/s
        test_rate_limited_endpoint!(list_earn_strategies, 24, 4, 5, Pro, &request);
    }

    #[tokio::test]
    async fn test_list_earn_allocations() {
        pause();

        let request = ListEarnAllocationsRequest::builder()
            .ascending(true)
            .hide_zero_allocations(true)
            .build();

        // 29 calls costs 2900, requiring 18s to replenish @ 500/s
        test_rate_limited_endpoint!(list_earn_allocations, 29, 18, 19, Intermediate, &request);
    }

    #[tokio::test]
    async fn test_get_websockets_token() {
        pause();

        // 23 calls costs 2300, requiring 3s to replenish @ 100/s
        test_rate_limited_endpoint!(get_websockets_token, 23, 3, 4, Pro);
    }
}
