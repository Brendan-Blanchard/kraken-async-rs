use crate::clients::errors::ClientError;
use crate::clients::http_response_types::ResultErrorResponse;
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
use std::collections::HashMap;

pub trait MockKrakenClient: Send + Sync {
    fn dispatch_server_time(
        &mut self,
        response: Result<ResultErrorResponse<SystemTime>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_system_status(
        &mut self,
        response: Result<ResultErrorResponse<SystemStatusInfo>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_asset_info(
        &mut self,
        response: Result<ResultErrorResponse<HashMap<String, AssetInfo>>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_tradable_asset_pairs(
        &mut self,
        response: Result<ResultErrorResponse<HashMap<String, TradableAssetPair>>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_ticker_information(
        &mut self,
        response: Result<ResultErrorResponse<HashMap<String, RestTickerInfo>>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_ohlc(&mut self, response: Result<ResultErrorResponse<OhlcResponse>, ClientError>) {
        todo!()
    }

    fn dispatch_orderbook(
        &mut self,
        response: Result<ResultErrorResponse<HashMap<String, Orderbook>>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_recent_trades(
        &mut self,
        response: Result<ResultErrorResponse<RecentTrades>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_recent_spreads(
        &mut self,
        response: Result<ResultErrorResponse<RecentSpreads>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_account_balance(
        &mut self,
        response: Result<ResultErrorResponse<AccountBalances>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_extended_balances(
        &mut self,
        response: Result<ResultErrorResponse<ExtendedBalances>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_trade_balances(
        &mut self,
        response: Result<ResultErrorResponse<TradeBalances>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_open_orders(
        &mut self,
        response: Result<ResultErrorResponse<OpenOrders>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_closed_orders(
        &mut self,
        response: Result<ResultErrorResponse<ClosedOrders>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_orders_info(
        &mut self,
        response: Result<ResultErrorResponse<HashMap<String, Order>>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_trades_history(
        &mut self,
        response: Result<ResultErrorResponse<TradesHistory>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_trades_info(
        &mut self,
        response: Result<ResultErrorResponse<TradesInfo>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_open_positions(
        &mut self,
        response: Result<ResultErrorResponse<OpenPositions>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_ledgers_info(
        &mut self,
        response: Result<ResultErrorResponse<LedgerInfo>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_query_ledgers(
        &mut self,
        response: Result<ResultErrorResponse<QueryLedgerInfo>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_trade_volume(
        &mut self,
        response: Result<ResultErrorResponse<TradeVolume>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_export_report(
        &mut self,
        response: Result<ResultErrorResponse<ExportReport>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_export_report_status(
        &mut self,
        response: Result<ResultErrorResponse<Vec<ExportReportStatus>>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_retrieve_export_report(&mut self, response: Result<Vec<u8>, ClientError>) {
        todo!()
    }

    fn dispatch_delete_export_report(
        &mut self,
        response: Result<ResultErrorResponse<DeleteExportReport>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_add_order(&mut self, response: Result<ResultErrorResponse<AddOrder>, ClientError>) {
        todo!()
    }

    fn dispatch_add_order_batch(
        &mut self,
        response: Result<ResultErrorResponse<AddOrderBatch>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_edit_order(
        &mut self,
        response: Result<ResultErrorResponse<OrderEdit>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_cancel_order(
        &mut self,
        response: Result<ResultErrorResponse<CancelOrder>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_cancel_all_orders(
        &mut self,
        response: Result<ResultErrorResponse<CancelOrder>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_cancel_all_orders_after(
        &mut self,
        response: Result<ResultErrorResponse<CancelAllOrdersAfter>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_cancel_order_batch(
        &mut self,
        response: Result<ResultErrorResponse<CancelOrder>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_deposit_methods(
        &mut self,
        response: Result<ResultErrorResponse<Vec<DepositMethod>>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_deposit_addresses(
        &mut self,
        response: Result<ResultErrorResponse<Vec<DepositAddress>>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_status_of_recent_deposits(
        &mut self,
        response: Result<ResultErrorResponse<DepositWithdrawResponse>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_withdrawal_methods(
        &mut self,
        response: Result<ResultErrorResponse<Vec<WithdrawMethod>>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_withdrawal_addresses(
        &mut self,
        response: Result<ResultErrorResponse<Vec<WithdrawalAddress>>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_withdrawal_info(
        &mut self,
        response: Result<ResultErrorResponse<Withdrawal>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_withdraw_funds(
        &mut self,
        response: Result<ResultErrorResponse<ConfirmationRefId>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_status_of_recent_withdrawals(
        &mut self,
        response: Result<ResultErrorResponse<Vec<DepositWithdrawal>>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_request_withdrawal_cancellation(
        &mut self,
        response: Result<ResultErrorResponse<bool>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_request_wallet_transfer(
        &mut self,
        response: Result<ResultErrorResponse<ConfirmationRefId>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_create_sub_account(
        &mut self,
        response: Result<ResultErrorResponse<bool>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_account_transfer(
        &mut self,
        response: Result<ResultErrorResponse<AccountTransfer>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_allocate_earn_funds(
        &mut self,
        response: Result<ResultErrorResponse<bool>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_deallocate_earn_funds(
        &mut self,
        response: Result<ResultErrorResponse<bool>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_earn_allocation_status(
        &mut self,
        response: Result<ResultErrorResponse<AllocationStatus>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_earn_deallocation_status(
        &mut self,
        response: Result<ResultErrorResponse<AllocationStatus>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_list_earn_strategies(
        &mut self,
        response: Result<ResultErrorResponse<EarnStrategies>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_list_earn_allocations(
        &mut self,
        response: Result<ResultErrorResponse<EarnAllocations>, ClientError>,
    ) {
        todo!()
    }

    fn dispatch_get_websockets_token(
        &mut self,
        response: Result<ResultErrorResponse<WebsocketToken>, ClientError>,
    ) {
        todo!()
    }
}
