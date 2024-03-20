use crate::clients::errors::ClientError;
use crate::clients::http_response_types::ResultErrorResponse;
use crate::clients::kraken_client::KrakenClient;
use crate::crypto::nonce_provider::NonceProvider;
use crate::request_types::{
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
use crate::response_types::{
    AccountBalances, AccountTransfer, AddOrder, AddOrderBatch, AllocationStatus, AssetInfo,
    CancelAllOrdersAfter, CancelOrder, ClosedOrders, ConfirmationRefId, DeleteExportReport,
    DepositAddress, DepositMethod, DepositWithdrawResponse, DepositWithdrawal, EarnAllocations,
    EarnStrategies, ExportReport, ExportReportStatus, ExtendedBalances, LedgerInfo, OhlcResponse,
    OpenOrders, OpenPositions, Order, OrderEdit, Orderbook, QueryLedgerInfo, RecentSpreads,
    RecentTrades, RestTickerInfo, SystemStatusInfo, SystemTime, TradableAssetPair, TradeBalances,
    TradeVolume, TradesHistory, TradesInfo, WebsocketToken, WithdrawMethod, Withdrawal,
    WithdrawalAddress,
};
use crate::secrets::secrets_provider::SecretsProvider;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// A fully mocked client for testing upstream systems.
///
/// Methods on [TestClient] simply return from an awaiting channel receiver that is fed via the
/// corresponding `dispatch_*` method implemented for [MockKrakenClient].
///
/// If a method should return immediately, you can first call the dispatch method and then await
/// the method.
///
/// TODO: cleanup, compile?
///
/// ```ignore
/// # use kraken_async_rs::response_types::SystemStatusInfo;
/// # use kraken_async_rs::wss::kraken_wss_types::SystemStatus;
///
/// let mut test_client = ...;
///
/// let mock_response = Ok(
///     SystemStatusInfo {
///         status: SystemStatus::Online,
///         timestamp: "".to_string(),
///     }
/// );
///
/// test_client.dispatch_system_status(mock_response.clone()).await;
///
/// let response = test_client.get_system_status().await;
///
/// assert_eq!(mock_response, response);
/// ```
///
pub struct TestClient {
    // TODO: tokio channel for each method
    // TODO: link each method to return from channel receiver
}

// TODO: impl MockKrakenClient for TestClient
//  dispatch into each channel producer

impl TestClient {
    pub fn new() -> Self {
        TestClient {}
    }
}

impl KrakenClient for TestClient {
    fn new(
        secrets_provider: Box<dyn SecretsProvider>,
        nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>>,
    ) -> Self {
        todo!()
    }

    fn new_with_url(
        secrets_provider: Box<dyn SecretsProvider>,
        nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>>,
        url: String,
    ) -> Self {
        todo!()
    }

    fn set_user_agent(&mut self, user_agent: String) {
        todo!()
    }

    async fn get_server_time(&mut self) -> Result<ResultErrorResponse<SystemTime>, ClientError> {
        todo!()
    }

    async fn get_system_status(
        &mut self,
    ) -> Result<ResultErrorResponse<SystemStatusInfo>, ClientError> {
        todo!()
    }

    async fn get_asset_info(
        &mut self,
        request: &AssetInfoRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, AssetInfo>>, ClientError> {
        todo!()
    }

    async fn get_tradable_asset_pairs(
        &mut self,
        request: &TradableAssetPairsRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, TradableAssetPair>>, ClientError> {
        todo!()
    }

    async fn get_ticker_information(
        &mut self,
        request: &TickerRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, RestTickerInfo>>, ClientError> {
        todo!()
    }

    async fn get_ohlc(
        &mut self,
        request: &OHLCRequest,
    ) -> Result<ResultErrorResponse<OhlcResponse>, ClientError> {
        todo!()
    }

    async fn get_orderbook(
        &mut self,
        request: &OrderbookRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, Orderbook>>, ClientError> {
        todo!()
    }

    async fn get_recent_trades(
        &mut self,
        request: &RecentTradesRequest,
    ) -> Result<ResultErrorResponse<RecentTrades>, ClientError> {
        todo!()
    }

    async fn get_recent_spreads(
        &mut self,
        request: &RecentSpreadsRequest,
    ) -> Result<ResultErrorResponse<RecentSpreads>, ClientError> {
        todo!()
    }

    async fn get_account_balance(
        &mut self,
    ) -> Result<ResultErrorResponse<AccountBalances>, ClientError> {
        todo!()
    }

    async fn get_extended_balances(
        &mut self,
    ) -> Result<ResultErrorResponse<ExtendedBalances>, ClientError> {
        todo!()
    }

    async fn get_trade_balances(
        &mut self,
        request: &TradeBalanceRequest,
    ) -> Result<ResultErrorResponse<TradeBalances>, ClientError> {
        todo!()
    }

    async fn get_open_orders(
        &mut self,
        request: &OpenOrdersRequest,
    ) -> Result<ResultErrorResponse<OpenOrders>, ClientError> {
        todo!()
    }

    async fn get_closed_orders(
        &mut self,
        request: &ClosedOrdersRequest,
    ) -> Result<ResultErrorResponse<ClosedOrders>, ClientError> {
        todo!()
    }

    async fn query_orders_info(
        &mut self,
        request: &OrderRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, Order>>, ClientError> {
        todo!()
    }

    async fn get_trades_history(
        &mut self,
        request: &TradesHistoryRequest,
    ) -> Result<ResultErrorResponse<TradesHistory>, ClientError> {
        todo!()
    }

    async fn query_trades_info(
        &mut self,
        request: &TradeInfoRequest,
    ) -> Result<ResultErrorResponse<TradesInfo>, ClientError> {
        todo!()
    }

    async fn get_open_positions(
        &mut self,
        request: &OpenPositionsRequest,
    ) -> Result<ResultErrorResponse<OpenPositions>, ClientError> {
        todo!()
    }

    async fn get_ledgers_info(
        &mut self,
        request: &LedgersInfoRequest,
    ) -> Result<ResultErrorResponse<LedgerInfo>, ClientError> {
        todo!()
    }

    async fn query_ledgers(
        &mut self,
        request: &QueryLedgerRequest,
    ) -> Result<ResultErrorResponse<QueryLedgerInfo>, ClientError> {
        todo!()
    }

    async fn get_trade_volume(
        &mut self,
        request: &TradeVolumeRequest,
    ) -> Result<ResultErrorResponse<TradeVolume>, ClientError> {
        todo!()
    }

    async fn request_export_report(
        &mut self,
        request: &ExportReportRequest,
    ) -> Result<ResultErrorResponse<ExportReport>, ClientError> {
        todo!()
    }

    async fn get_export_report_status(
        &mut self,
        request: &ExportReportStatusRequest,
    ) -> Result<ResultErrorResponse<Vec<ExportReportStatus>>, ClientError> {
        todo!()
    }

    async fn retrieve_export_report(
        &mut self,
        request: &RetrieveExportReportRequest,
    ) -> Result<Vec<u8>, ClientError> {
        todo!()
    }

    async fn delete_export_report(
        &mut self,
        request: &DeleteExportRequest,
    ) -> Result<ResultErrorResponse<DeleteExportReport>, ClientError> {
        todo!()
    }

    async fn add_order(
        &mut self,
        request: &AddOrderRequest,
    ) -> Result<ResultErrorResponse<AddOrder>, ClientError> {
        todo!()
    }

    async fn add_order_batch(
        &mut self,
        request: &AddBatchedOrderRequest,
    ) -> Result<ResultErrorResponse<AddOrderBatch>, ClientError> {
        todo!()
    }

    async fn edit_order(
        &mut self,
        request: &EditOrderRequest,
    ) -> Result<ResultErrorResponse<OrderEdit>, ClientError> {
        todo!()
    }

    async fn cancel_order(
        &mut self,
        request: &CancelOrderRequest,
    ) -> Result<ResultErrorResponse<CancelOrder>, ClientError> {
        todo!()
    }

    async fn cancel_all_orders(&mut self) -> Result<ResultErrorResponse<CancelOrder>, ClientError> {
        todo!()
    }

    async fn cancel_all_orders_after(
        &mut self,
        request: &CancelAllOrdersAfterRequest,
    ) -> Result<ResultErrorResponse<CancelAllOrdersAfter>, ClientError> {
        todo!()
    }

    async fn cancel_order_batch(
        &mut self,
        request: &CancelBatchOrdersRequest,
    ) -> Result<ResultErrorResponse<CancelOrder>, ClientError> {
        todo!()
    }

    async fn get_deposit_methods(
        &mut self,
        request: &DepositMethodsRequest,
    ) -> Result<ResultErrorResponse<Vec<DepositMethod>>, ClientError> {
        todo!()
    }

    async fn get_deposit_addresses(
        &mut self,
        request: &DepositAddressesRequest,
    ) -> Result<ResultErrorResponse<Vec<DepositAddress>>, ClientError> {
        todo!()
    }

    async fn get_status_of_recent_deposits(
        &mut self,
        request: &StatusOfDepositWithdrawRequest,
    ) -> Result<ResultErrorResponse<DepositWithdrawResponse>, ClientError> {
        todo!()
    }

    async fn get_withdrawal_methods(
        &mut self,
        request: &WithdrawalMethodsRequest,
    ) -> Result<ResultErrorResponse<Vec<WithdrawMethod>>, ClientError> {
        todo!()
    }

    async fn get_withdrawal_addresses(
        &mut self,
        request: &WithdrawalAddressesRequest,
    ) -> Result<ResultErrorResponse<Vec<WithdrawalAddress>>, ClientError> {
        todo!()
    }

    async fn get_withdrawal_info(
        &mut self,
        request: &WithdrawalInfoRequest,
    ) -> Result<ResultErrorResponse<Withdrawal>, ClientError> {
        todo!()
    }

    async fn withdraw_funds(
        &mut self,
        request: &WithdrawFundsRequest,
    ) -> Result<ResultErrorResponse<ConfirmationRefId>, ClientError> {
        todo!()
    }

    async fn get_status_of_recent_withdrawals(
        &mut self,
        request: &StatusOfDepositWithdrawRequest,
    ) -> Result<ResultErrorResponse<Vec<DepositWithdrawal>>, ClientError> {
        todo!()
    }

    async fn request_withdrawal_cancellation(
        &mut self,
        request: &WithdrawCancelRequest,
    ) -> Result<ResultErrorResponse<bool>, ClientError> {
        todo!()
    }

    async fn request_wallet_transfer(
        &mut self,
        request: &WalletTransferRequest,
    ) -> Result<ResultErrorResponse<ConfirmationRefId>, ClientError> {
        todo!()
    }

    async fn create_sub_account(
        &mut self,
        request: &CreateSubAccountRequest,
    ) -> Result<ResultErrorResponse<bool>, ClientError> {
        todo!()
    }

    async fn account_transfer(
        &mut self,
        request: &AccountTransferRequest,
    ) -> Result<ResultErrorResponse<AccountTransfer>, ClientError> {
        todo!()
    }

    async fn allocate_earn_funds(
        &mut self,
        request: &AllocateEarnFundsRequest,
    ) -> Result<ResultErrorResponse<bool>, ClientError> {
        todo!()
    }

    async fn deallocate_earn_funds(
        &mut self,
        request: &AllocateEarnFundsRequest,
    ) -> Result<ResultErrorResponse<bool>, ClientError> {
        todo!()
    }

    async fn get_earn_allocation_status(
        &mut self,
        request: &EarnAllocationStatusRequest,
    ) -> Result<ResultErrorResponse<AllocationStatus>, ClientError> {
        todo!()
    }

    async fn get_earn_deallocation_status(
        &mut self,
        request: &EarnAllocationStatusRequest,
    ) -> Result<ResultErrorResponse<AllocationStatus>, ClientError> {
        todo!()
    }

    async fn list_earn_strategies(
        &mut self,
        request: &ListEarnStrategiesRequest,
    ) -> Result<ResultErrorResponse<EarnStrategies>, ClientError> {
        todo!()
    }

    async fn list_earn_allocations(
        &mut self,
        request: &ListEarnAllocationsRequest,
    ) -> Result<ResultErrorResponse<EarnAllocations>, ClientError> {
        todo!()
    }

    async fn get_websockets_token(
        &mut self,
    ) -> Result<ResultErrorResponse<WebsocketToken>, ClientError> {
        todo!()
    }
}
