//! Common trait and endpoints
#[allow(unused)]
use crate::clients::core_kraken_client::CoreKrakenClient;
use crate::clients::errors::ClientError;
use crate::clients::http_response_types::ResultErrorResponse;
#[allow(unused)]
use crate::clients::rate_limited_kraken_client::RateLimitedKrakenClient;
use crate::crypto::nonce_provider::NonceProvider;
use crate::request_types::*;
use crate::response_types::*;
use crate::secrets::secrets_provider::SecretsProvider;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod endpoints {
    pub const KRAKEN_BASE_URL: &str = "https://api.kraken.com";

    pub const TIME_ENDPOINT: &str = "/0/public/Time";
    pub const STATUS_ENDPOINT: &str = "/0/public/SystemStatus";
    pub const ASSET_INFO_ENDPOINT: &str = "/0/public/Assets";
    pub const TRADABLE_ASSET_PAIRS_ENDPOINT: &str = "/0/public/AssetPairs";
    pub const TICKER_INFO_ENDPOINT: &str = "/0/public/Ticker";
    pub const OHLC_ENDPOINT: &str = "/0/public/OHLC";
    pub const ORDER_BOOK_ENDPOINT: &str = "/0/public/Depth";
    pub const RECENT_TRADES_ENDPOINT: &str = "/0/public/Trades";
    pub const RECENT_SPREADS_ENDPOINT: &str = "/0/public/Spread";

    pub const ACCOUNT_BALANCE_ENDPOINT: &str = "/0/private/Balance";
    pub const ACCOUNT_BALANCE_EXTENDED_ENDPOINT: &str = "/0/private/BalanceEx";
    pub const TRADE_BALANCE_ENDPOINT: &str = "/0/private/TradeBalance";
    pub const OPEN_ORDERS_ENDPOINT: &str = "/0/private/OpenOrders";
    pub const CLOSED_ORDERS_ENDPOINT: &str = "/0/private/ClosedOrders";
    pub const QUERY_ORDERS_ENDPOINT: &str = "/0/private/QueryOrders";
    pub const ORDER_AMENDS_ENDPOINT: &str = "/0/private/OrderAmends";
    pub const TRADES_HISTORY_ENDPOINT: &str = "/0/private/TradesHistory";
    pub const QUERY_TRADES_ENDPOINT: &str = "/0/private/QueryTrades";
    pub const OPEN_POSITIONS_ENDPOINT: &str = "/0/private/OpenPositions";
    pub const LEDGERS_ENDPOINT: &str = "/0/private/Ledgers";
    pub const QUERY_LEDGERS_ENDPOINT: &str = "/0/private/QueryLedgers";
    pub const TRADE_VOLUME_ENDPOINT: &str = "/0/private/TradeVolume";
    pub const ADD_EXPORT_ENDPOINT: &str = "/0/private/AddExport";
    pub const EXPORT_STATUS_ENDPOINT: &str = "/0/private/ExportStatus";
    pub const RETRIEVE_EXPORT_ENDPOINT: &str = "/0/private/RetrieveExport";
    pub const REMOVE_EXPORT_ENDPOINT: &str = "/0/private/RemoveExport";

    pub const ADD_ORDER_ENDPOINT: &str = "/0/private/AddOrder";
    pub const ADD_ORDER_BATCH_ENDPOINT: &str = "/0/private/AddOrderBatch";
    pub const AMEND_ORDER_ENDPOINT: &str = "/0/private/AmendOrder";
    pub const EDIT_ORDER_ENDPOINT: &str = "/0/private/EditOrder";
    pub const CANCEL_ORDER_ENDPOINT: &str = "/0/private/CancelOrder";
    pub const CANCEL_ALL_ORDERS_ENDPOINT: &str = "/0/private/CancelAll";
    pub const CANCEL_ALL_ORDERS_AFTER_ENDPOINT: &str = "/0/private/CancelAllOrdersAfter";
    pub const CANCEL_ORDER_BATCH_ENDPOINT: &str = "/0/private/CancelOrderBatch";

    pub const DEPOSIT_METHODS_ENDPOINT: &str = "/0/private/DepositMethods";
    pub const DEPOSIT_ADDRESSES_ENDPOINT: &str = "/0/private/DepositAddresses";
    pub const DEPOSIT_STATUS_ENDPOINT: &str = "/0/private/DepositStatus";

    pub const WITHDRAW_METHODS_ENDPOINT: &str = "/0/private/WithdrawMethods";
    pub const WITHDRAW_ADDRESSES_ENDPOINT: &str = "/0/private/WithdrawAddresses";
    pub const WITHDRAW_INFO_ENDPOINT: &str = "/0/private/WithdrawInfo";
    pub const WITHDRAW_ENDPOINT: &str = "/0/private/Withdraw";
    pub const WITHDRAW_STATUS_ENDPOINT: &str = "/0/private/WithdrawStatus";
    pub const WITHDRAW_CANCEL_ENDPOINT: &str = "/0/private/WithdrawCancel";

    pub const WALLET_TRANSFER_ENDPOINT: &str = "/0/private/WalletTransfer";

    pub const CREATE_SUB_ACCOUNT_ENDPOINT: &str = "/0/private/CreateSubaccount";
    pub const ACCOUNT_TRANSFER_ENDPOINT: &str = "/0/private/AccountTransfer";

    pub const EARN_ALLOCATE_ENDPOINT: &str = "/0/private/Earn/Allocate";
    pub const EARN_DEALLOCATE_ENDPOINT: &str = "/0/private/Earn/Deallocate";
    pub const EARN_ALLOCATE_STATUS_ENDPOINT: &str = "/0/private/Earn/AllocateStatus";
    pub const EARN_DEALLOCATE_STATUS_ENDPOINT: &str = "/0/private/Earn/DeallocateStatus";
    pub const EARN_STRATEGIES_ENDPOINT: &str = "/0/private/Earn/Strategies";
    pub const EARN_ALLOCATIONS_ENDPOINT: &str = "/0/private/Earn/Allocations";

    pub const GET_WS_TOKEN_ENDPOINT: &str = "/0/private/GetWebSocketsToken";
}

/// The common trait shared by implementations like [CoreKrakenClient] and [RateLimitedKrakenClient]
///
pub trait KrakenClient: Send + Sync {
    /// Creates a new instance with the given [SecretsProvider] and [NonceProvider].
    fn new(
        secrets_provider: Box<Arc<Mutex<dyn SecretsProvider>>>,
        nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>>,
    ) -> Self;

    /// Creates a new instance, allowing a specific URL to be set.
    ///
    /// Useful if using a proxy, testing with a mock-server, or if the URL changes from the default
    /// used in this library.
    fn new_with_url(
        secrets_provider: Box<Arc<Mutex<dyn SecretsProvider>>>,
        nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>>,
        url: impl ToString,
    ) -> Self;

    /// Creates a new instance with the given [SecretsProvider] and [NonceProvider], optionally
    /// enabling tracing for inbound messages.
    fn new_with_tracing(
        secrets_provider: Box<Arc<Mutex<dyn SecretsProvider>>>,
        nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>>,
        trace_inbound: bool,
    ) -> Self;

    /// Set the user-agent that will be sent in HTTP headers to Kraken. This is not required to be
    /// set.
    fn set_user_agent(&mut self, user_agent: impl ToString) -> impl Future<Output = ()>;

    /// Get the server time in two useful formats.
    fn get_server_time(
        &mut self,
    ) -> impl Future<Output = Result<ResultErrorResponse<SystemTime>, ClientError>> + Send;

    /// Get the status of the system, including the current server time.
    fn get_system_status(
        &mut self,
    ) -> impl Future<Output = Result<ResultErrorResponse<SystemStatusInfo>, ClientError>>;

    /// Get info about a particular asset, e.g. "XBT" or "ETH".
    fn get_asset_info(
        &mut self,
        request: &AssetInfoRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<HashMap<String, AssetInfo>>, ClientError>>;

    /// Get info about tradable asset pairs, such as USDCUSD, BTCUSD, or XETHZUSD.
    ///
    /// Returns all the information needed to place correctly formatted order volumes, prices, and pairs.
    fn get_tradable_asset_pairs(
        &mut self,
        request: &TradableAssetPairsRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<HashMap<String, TradableAssetPair>>, ClientError>>;

    /// Return some or all ticker data, including the most recent bid, ask, price, and last-24h
    /// stats for each requested pair.
    fn get_ticker_information(
        &mut self,
        request: &TickerRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<HashMap<String, RestTickerInfo>>, ClientError>>;

    /// Retrieve up to the last 720 OHLC candlesticks for a given pair and interval.
    ///
    /// The `since` request parameter allows for getting only data since some timestamp
    /// (the `last` response), but does not allow pagination.
    fn get_ohlc(
        &mut self,
        request: &OHLCRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<OhlcResponse>, ClientError>>;

    /// Get a snapshot of the orderbook for the requested pair and depth-of-book.
    fn get_orderbook(
        &mut self,
        request: &OrderbookRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<HashMap<String, Orderbook>>, ClientError>>;

    /// Retrieve up to 1000 trades at a time from the FULL history of Kraken's exchange for the
    /// requested pair.
    ///
    /// The `since` and `count` parameters allow complete pagination starting from the exchange's
    /// first-ever trade in each pair.
    ///
    /// See /examples/live_retrieving_recent_trades.rs for a full example of pagination with rate
    /// limiting.
    fn get_recent_trades(
        &mut self,
        request: &RecentTradesRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<RecentTrades>, ClientError>>;

    /// Get the last ~200 spread values for the requested pair.
    ///
    /// The `since` parameter allows getting incremental updates, but does not paginate the request
    /// historically.
    fn get_recent_spreads(
        &mut self,
        request: &RecentSpreadsRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<RecentSpreads>, ClientError>>;

    /// Get the raw balances for your account, minus any pending withdrawals.
    fn get_account_balance(
        &mut self,
    ) -> impl Future<Output = Result<ResultErrorResponse<AccountBalances>, ClientError>>;

    /// Get the extended balances for your account, which denotes the balance, any balance on hold,
    /// and lines of credit (if available on your account).
    fn get_extended_balances(
        &mut self,
    ) -> impl Future<Output = Result<ResultErrorResponse<ExtendedBalances>, ClientError>>;

    /// Get balances relevant for futures and margin trading, including equity and margin levels.
    fn get_trade_balances(
        &mut self,
        request: &TradeBalanceRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<TradeBalances>, ClientError>>;

    /// Get all open orders for your account.
    fn get_open_orders(
        &mut self,
        request: &OpenOrdersRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<OpenOrders>, ClientError>>;

    /// Get closed orders from the full history of your account, up to 50 at a time.
    ///
    /// Pagination is done using the `start`, `end`, `ofs` (offset) parameters.
    fn get_closed_orders(
        &mut self,
        request: &ClosedOrdersRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<ClosedOrders>, ClientError>>;

    /// Get the information for up to 50 orders at a time.
    fn query_orders_info(
        &mut self,
        request: &OrderRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<HashMap<String, Order>>, ClientError>>;

    fn get_order_amends(
        &mut self,
        request: &OrderAmendsRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<OrderAmends>, ClientError>>;

    /// Get trades from the full history your account, up to 50 at a time.
    ///
    /// Pagination is done using the `start`, `end` and `ofs` (offset) parameters.
    fn get_trades_history(
        &mut self,
        request: &TradesHistoryRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<TradesHistory>, ClientError>>;

    /// Get trade details for up to 20 specific trades by id at a time.
    fn query_trades_info(
        &mut self,
        request: &TradeInfoRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<TradesInfo>, ClientError>>;

    /// Get information about open margin positions.
    fn get_open_positions(
        &mut self,
        request: &OpenPositionsRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<OpenPositions>, ClientError>>;

    /// Get ledger entries for the full history of your account, up to 50 at a time.
    ///
    /// Pagination is done using the `start`, `end`, `ofs` (offset) parameters.
    fn get_ledgers_info(
        &mut self,
        request: &LedgersInfoRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<LedgerInfo>, ClientError>>;

    /// Get ledger information for up to 20 ids at a time.
    fn query_ledgers(
        &mut self,
        request: &QueryLedgerRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<QueryLedgerInfo>, ClientError>>;

    /// Get the 30-day trading volume for your account, and fee information for any pairs (if requested).
    fn get_trade_volume(
        &mut self,
        request: &TradeVolumeRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<TradeVolume>, ClientError>>;

    /// Request a report for ledgers or trades to be generated asynchronously.
    fn request_export_report(
        &mut self,
        request: &ExportReportRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<ExportReport>, ClientError>>;

    /// Get the status of a report that was requested.
    fn get_export_report_status(
        &mut self,
        request: &ExportReportStatusRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<Vec<ExportReportStatus>>, ClientError>>;

    /// Retrieve an export report once generated.
    fn retrieve_export_report(
        &mut self,
        request: &RetrieveExportReportRequest,
    ) -> impl Future<Output = Result<Vec<u8>, ClientError>>;

    /// Request for an export report to be deleted.
    fn delete_export_report(
        &mut self,
        request: &DeleteExportRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<DeleteExportReport>, ClientError>>;

    /// Add an order of any type (market, limit, trailing stop, etc).
    fn add_order(
        &mut self,
        request: &AddOrderRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<AddOrder>, ClientError>>;

    /// Add up to 15 orders *for a single pair* at once. Orders that fail to place are dropped from
    /// processing and will be returned with errors in the response's `Vec`.
    fn add_order_batch(
        &mut self,
        request: &AddBatchedOrderRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<AddOrderBatch>, ClientError>>;

    fn amend_order(
        &mut self,
        request: &AmendOrderRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<AmendOrder>, ClientError>>;

    /// Edit the volume or price of an existing order, excluding contingent orders like stop/profit orders.
    fn edit_order(
        &mut self,
        request: &EditOrderRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<OrderEdit>, ClientError>>;

    /// Cancel an existing order by ref-id or user-ref.
    fn cancel_order(
        &mut self,
        request: &CancelOrderRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<CancelOrder>, ClientError>>;

    /// Cancel all active orders.
    fn cancel_all_orders(
        &mut self,
    ) -> impl Future<Output = Result<ResultErrorResponse<CancelOrder>, ClientError>>;

    /// Submit a "Dead Man's Switch" that will cancel all orders if not repeatedly updated over time.
    fn cancel_all_orders_after(
        &mut self,
        request: &CancelAllOrdersAfterRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<CancelAllOrdersAfter>, ClientError>>;

    /// Cancel up to 50 orders in a batch by id or user-ref.
    fn cancel_order_batch(
        &mut self,
        request: &CancelBatchOrdersRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<CancelOrder>, ClientError>>;

    /// Get all methods of depositing a specific asset.
    fn get_deposit_methods(
        &mut self,
        request: &DepositMethodsRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<Vec<DepositMethod>>, ClientError>>;

    /// Get all available addresses for a given asset and method.
    fn get_deposit_addresses(
        &mut self,
        request: &DepositAddressesRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<Vec<DepositAddress>>, ClientError>>;

    /// Get the status of recent deposits.
    ///
    /// Pagination is done using the `start`, `end`, `cursor` and `limit` parameters.
    fn get_status_of_recent_deposits(
        &mut self,
        request: &StatusOfDepositWithdrawRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<DepositWithdrawResponse>, ClientError>>;

    /// Get all withdrawal methods, optionally for a given asset.
    fn get_withdrawal_methods(
        &mut self,
        request: &WithdrawalMethodsRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<Vec<WithdrawMethod>>, ClientError>>;

    /// Get all withdrawal addresses, optionally for a specific asset or method.
    fn get_withdrawal_addresses(
        &mut self,
        request: &WithdrawalAddressesRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<Vec<WithdrawalAddress>>, ClientError>>;

    /// Get details about a particular withdrawal.
    fn get_withdrawal_info(
        &mut self,
        request: &WithdrawalInfoRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<Withdrawal>, ClientError>>;

    /// Request a withdrawal for the provided asset and key.
    fn withdraw_funds(
        &mut self,
        request: &WithdrawFundsRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<ConfirmationRefId>, ClientError>>;

    /// Get the status of recent withdrawals.
    ///
    /// Pagination is done using the `start`, `end`, `cursor` and `limit` parameters.
    fn get_status_of_recent_withdrawals(
        &mut self,
        request: &StatusOfDepositWithdrawRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<Vec<DepositWithdrawal>>, ClientError>>;

    /// Request to cancel a particular withdrawal if it has not been fully processed.
    fn request_withdrawal_cancellation(
        &mut self,
        request: &WithdrawCancelRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<bool>, ClientError>>;

    /// Request to transfer from the default Spot wallet to a Futures wallet if available.
    fn request_wallet_transfer(
        &mut self,
        request: &WalletTransferRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<ConfirmationRefId>, ClientError>>;

    /// Create a linked sub-account for the given username and email (Institutional Clients only).
    fn create_sub_account(
        &mut self,
        request: &CreateSubAccountRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<bool>, ClientError>>;

    /// Request to transfer a given asset between sub-accounts (Institutional Clients only).
    fn account_transfer(
        &mut self,
        request: &AccountTransferRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<AccountTransfer>, ClientError>>;

    /// Allocate available funds to a given earn strategy.
    fn allocate_earn_funds(
        &mut self,
        request: &AllocateEarnFundsRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<bool>, ClientError>>;

    /// De-allocate funds from a given earn strategy.
    fn deallocate_earn_funds(
        &mut self,
        request: &AllocateEarnFundsRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<bool>, ClientError>>;

    /// Get the status for the only pending earn allocation request if there is one.
    fn get_earn_allocation_status(
        &mut self,
        request: &EarnAllocationStatusRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<AllocationStatus>, ClientError>>;

    /// Get the status for the only pending earn de-allocation if there is one.
    fn get_earn_deallocation_status(
        &mut self,
        request: &EarnAllocationStatusRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<AllocationStatus>, ClientError>>;

    /// List all earn strategies.
    ///
    /// Pagination is supported through the cursor and limit parameters.
    fn list_earn_strategies(
        &mut self,
        request: &ListEarnStrategiesRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<EarnStrategies>, ClientError>>;

    /// List all current earn allocations.
    fn list_earn_allocations(
        &mut self,
        request: &ListEarnAllocationsRequest,
    ) -> impl Future<Output = Result<ResultErrorResponse<EarnAllocations>, ClientError>>;

    /// Get a token for connecting to private websockets.
    ///
    /// Tokens are valid for 15 minutes for their first use, but do not require being refreshed
    /// once a connection is established.
    fn get_websockets_token(
        &mut self,
    ) -> impl Future<Output = Result<ResultErrorResponse<WebsocketToken>, ClientError>>;
}
