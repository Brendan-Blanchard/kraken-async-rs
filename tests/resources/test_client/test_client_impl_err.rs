use crate::resources::test_auth::get_null_secrets_provider;
use kraken_async_rs::clients::errors::ClientError;
use kraken_async_rs::clients::http_response_types::ResultErrorResponse;
use kraken_async_rs::clients::kraken_client::endpoints::KRAKEN_BASE_URL;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::clients::rate_limited_kraken_client::RateLimitedKrakenClient;
use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
use kraken_async_rs::request_types::{
    AccountTransferRequest, AddBatchedOrderRequest, AddOrderRequest, AllocateEarnFundsRequest,
    AssetInfoRequest, CancelAllOrdersAfterRequest, CancelBatchOrdersRequest, CancelOrderRequest,
    ClosedOrdersRequest, CreateSubAccountRequest, DeleteExportRequest, DepositAddressesRequest,
    DepositMethodsRequest, EarnAllocationStatusRequest, EditOrderRequest, ExportReportRequest,
    ExportReportStatusRequest, LedgersInfoRequest, ListEarnAllocationsRequest,
    ListEarnStrategiesRequest, OHLCRequest, OpenOrdersRequest, OpenPositionsRequest, OrderRequest,
    OrderbookRequest, QueryLedgerRequest, RecentSpreadsRequest, RecentTradesRequest,
    RetrieveExportReportRequest, StatusOfDepositWithdrawRequest, TickerRequest,
    TradableAssetPairsRequest, TradeBalanceRequest, TradeInfoRequest, TradeVolumeRequest,
    TradesHistoryRequest, WalletTransferRequest, WithdrawCancelRequest, WithdrawFundsRequest,
    WithdrawalAddressesRequest, WithdrawalInfoRequest, WithdrawalMethodsRequest,
};
use kraken_async_rs::response_types::{
    AccountBalances, AccountTransfer, AddOrder, AddOrderBatch, AllocationStatus, AssetInfo,
    CancelAllOrdersAfter, CancelOrder, ClosedOrders, ConfirmationRefId, DeleteExportReport,
    DepositAddress, DepositMethod, DepositWithdrawResponse, DepositWithdrawal, EarnAllocations,
    EarnStrategies, ExportReport, ExportReportStatus, ExtendedBalances, LedgerInfo, OhlcResponse,
    OpenOrders, OpenPositions, Order, OrderEdit, Orderbook, QueryLedgerInfo, RecentSpreads,
    RecentTrades, RestTickerInfo, SystemStatusInfo, SystemTime, TradableAssetPair, TradeBalances,
    TradeVolume, TradesHistory, TradesInfo, VerificationTier, WebsocketToken, WithdrawMethod,
    Withdrawal, WithdrawalAddress,
};
use kraken_async_rs::secrets::secrets_provider::SecretsProvider;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn get_rate_limit_test_client_err(verification: VerificationTier) -> TestRateLimitedClientErr {
    let secrets_provider = get_null_secrets_provider();
    let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> =
        Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));

    RateLimitedKrakenClient::new_with_verification_tier_and_url(
        secrets_provider,
        nonce_provider,
        KRAKEN_BASE_URL.to_string(),
        verification,
    )
}

pub type TestRateLimitedClientErr = RateLimitedKrakenClient<TestClientErr>;

pub struct TestClientErr {}

impl KrakenClient for TestClientErr {
    fn new(
        _secrets_provider: Box<dyn SecretsProvider>,
        _nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>>,
    ) -> Self {
        Self {}
    }

    fn new_with_url(
        _secrets_provider: Box<dyn SecretsProvider>,
        _nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>>,
        _url: String,
    ) -> Self {
        Self {}
    }

    async fn set_user_agent(&mut self, _user_agent: String) {}

    async fn get_server_time(&mut self) -> Result<ResultErrorResponse<SystemTime>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_system_status(
        &mut self,
    ) -> Result<ResultErrorResponse<SystemStatusInfo>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_asset_info(
        &mut self,
        _request: &AssetInfoRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, AssetInfo>>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_tradable_asset_pairs(
        &mut self,
        _request: &TradableAssetPairsRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, TradableAssetPair>>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_ticker_information(
        &mut self,
        _request: &TickerRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, RestTickerInfo>>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_ohlc(
        &mut self,
        _request: &OHLCRequest,
    ) -> Result<ResultErrorResponse<OhlcResponse>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_orderbook(
        &mut self,
        _request: &OrderbookRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, Orderbook>>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_recent_trades(
        &mut self,
        _request: &RecentTradesRequest,
    ) -> Result<ResultErrorResponse<RecentTrades>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_recent_spreads(
        &mut self,
        _request: &RecentSpreadsRequest,
    ) -> Result<ResultErrorResponse<RecentSpreads>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_account_balance(
        &mut self,
    ) -> Result<ResultErrorResponse<AccountBalances>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_extended_balances(
        &mut self,
    ) -> Result<ResultErrorResponse<ExtendedBalances>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_trade_balances(
        &mut self,
        _request: &TradeBalanceRequest,
    ) -> Result<ResultErrorResponse<TradeBalances>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_open_orders(
        &mut self,
        _request: &OpenOrdersRequest,
    ) -> Result<ResultErrorResponse<OpenOrders>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_closed_orders(
        &mut self,
        _request: &ClosedOrdersRequest,
    ) -> Result<ResultErrorResponse<ClosedOrders>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn query_orders_info(
        &mut self,
        _request: &OrderRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, Order>>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_trades_history(
        &mut self,
        _request: &TradesHistoryRequest,
    ) -> Result<ResultErrorResponse<TradesHistory>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn query_trades_info(
        &mut self,
        _request: &TradeInfoRequest,
    ) -> Result<ResultErrorResponse<TradesInfo>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_open_positions(
        &mut self,
        _request: &OpenPositionsRequest,
    ) -> Result<ResultErrorResponse<OpenPositions>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_ledgers_info(
        &mut self,
        _request: &LedgersInfoRequest,
    ) -> Result<ResultErrorResponse<LedgerInfo>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn query_ledgers(
        &mut self,
        _request: &QueryLedgerRequest,
    ) -> Result<ResultErrorResponse<QueryLedgerInfo>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_trade_volume(
        &mut self,
        _request: &TradeVolumeRequest,
    ) -> Result<ResultErrorResponse<TradeVolume>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn request_export_report(
        &mut self,
        _request: &ExportReportRequest,
    ) -> Result<ResultErrorResponse<ExportReport>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_export_report_status(
        &mut self,
        _request: &ExportReportStatusRequest,
    ) -> Result<ResultErrorResponse<Vec<ExportReportStatus>>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn retrieve_export_report(
        &mut self,
        _request: &RetrieveExportReportRequest,
    ) -> Result<Vec<u8>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn delete_export_report(
        &mut self,
        _request: &DeleteExportRequest,
    ) -> Result<ResultErrorResponse<DeleteExportReport>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn add_order(
        &mut self,
        _request: &AddOrderRequest,
    ) -> Result<ResultErrorResponse<AddOrder>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn add_order_batch(
        &mut self,
        _request: &AddBatchedOrderRequest,
    ) -> Result<ResultErrorResponse<AddOrderBatch>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn edit_order(
        &mut self,
        _request: &EditOrderRequest,
    ) -> Result<ResultErrorResponse<OrderEdit>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn cancel_order(
        &mut self,
        _request: &CancelOrderRequest,
    ) -> Result<ResultErrorResponse<CancelOrder>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn cancel_all_orders(&mut self) -> Result<ResultErrorResponse<CancelOrder>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn cancel_all_orders_after(
        &mut self,
        _request: &CancelAllOrdersAfterRequest,
    ) -> Result<ResultErrorResponse<CancelAllOrdersAfter>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn cancel_order_batch(
        &mut self,
        _request: &CancelBatchOrdersRequest,
    ) -> Result<ResultErrorResponse<CancelOrder>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_deposit_methods(
        &mut self,
        _request: &DepositMethodsRequest,
    ) -> Result<ResultErrorResponse<Vec<DepositMethod>>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_deposit_addresses(
        &mut self,
        _request: &DepositAddressesRequest,
    ) -> Result<ResultErrorResponse<Vec<DepositAddress>>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_status_of_recent_deposits(
        &mut self,
        _request: &StatusOfDepositWithdrawRequest,
    ) -> Result<ResultErrorResponse<DepositWithdrawResponse>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_withdrawal_methods(
        &mut self,
        _request: &WithdrawalMethodsRequest,
    ) -> Result<ResultErrorResponse<Vec<WithdrawMethod>>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_withdrawal_addresses(
        &mut self,
        _request: &WithdrawalAddressesRequest,
    ) -> Result<ResultErrorResponse<Vec<WithdrawalAddress>>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_withdrawal_info(
        &mut self,
        _request: &WithdrawalInfoRequest,
    ) -> Result<ResultErrorResponse<Withdrawal>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn withdraw_funds(
        &mut self,
        _request: &WithdrawFundsRequest,
    ) -> Result<ResultErrorResponse<ConfirmationRefId>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_status_of_recent_withdrawals(
        &mut self,
        _request: &StatusOfDepositWithdrawRequest,
    ) -> Result<ResultErrorResponse<Vec<DepositWithdrawal>>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn request_withdrawal_cancellation(
        &mut self,
        _request: &WithdrawCancelRequest,
    ) -> Result<ResultErrorResponse<bool>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn request_wallet_transfer(
        &mut self,
        _request: &WalletTransferRequest,
    ) -> Result<ResultErrorResponse<ConfirmationRefId>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn create_sub_account(
        &mut self,
        _request: &CreateSubAccountRequest,
    ) -> Result<ResultErrorResponse<bool>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn account_transfer(
        &mut self,
        _request: &AccountTransferRequest,
    ) -> Result<ResultErrorResponse<AccountTransfer>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn allocate_earn_funds(
        &mut self,
        _request: &AllocateEarnFundsRequest,
    ) -> Result<ResultErrorResponse<bool>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn deallocate_earn_funds(
        &mut self,
        _request: &AllocateEarnFundsRequest,
    ) -> Result<ResultErrorResponse<bool>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_earn_allocation_status(
        &mut self,
        _request: &EarnAllocationStatusRequest,
    ) -> Result<ResultErrorResponse<AllocationStatus>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_earn_deallocation_status(
        &mut self,
        _request: &EarnAllocationStatusRequest,
    ) -> Result<ResultErrorResponse<AllocationStatus>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn list_earn_strategies(
        &mut self,
        _request: &ListEarnStrategiesRequest,
    ) -> Result<ResultErrorResponse<EarnStrategies>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn list_earn_allocations(
        &mut self,
        _request: &ListEarnAllocationsRequest,
    ) -> Result<ResultErrorResponse<EarnAllocations>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }

    async fn get_websockets_token(
        &mut self,
    ) -> Result<ResultErrorResponse<WebsocketToken>, ClientError> {
        Err(ClientError::Parse("StubbedForTesting"))
    }
}
