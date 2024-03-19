//! These are highly repetitive, but get coverage and show the expected behavior of each endpoint's
//! rate limiting.
//!
//! Difference between the Pro and Intermediate verification tiers is not tested 100%, only by
//! alternating them throughout. It's implied at some point that it just works :)
//!
use crate::resources::test_auth::get_null_secrets_provider;

mod resources;

use crate::resources::test_client::test_client_impl::TestRateLimitedClient;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::clients::rate_limited_kraken_client::RateLimitedKrakenClient;
use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
use kraken_async_rs::request_types::{
    ClosedOrdersRequestBuilder, DeleteExportRequest, DeleteExportType, ExportReportRequest,
    ExportReportStatusRequest, LedgersInfoRequest, OpenOrdersRequest, OpenPositionsRequest,
    OrderRequest, QueryLedgerRequest, ReportFormatType, ReportType, RetrieveExportReportRequest,
    StringCSV, TradeBalanceRequest, TradeInfoRequest, TradeVolumeRequest, TradesHistoryRequest,
};
use kraken_async_rs::response_types::VerificationTier::{Intermediate, Pro};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::{pause, Instant};

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

    let request = RetrieveExportReportRequest::builder("HI1M0S-BCRBJ-P01V9R".to_string()).build();

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
