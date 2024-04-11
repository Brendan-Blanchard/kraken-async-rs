use crate::resources::test_serde::test_display_output;
use kraken_async_rs::clients::errors::KrakenError;
use kraken_async_rs::request_types::{
    AssetPairInfo, CandlestickInterval, CloseTime, Cursor, DeleteExportType, LockType,
    ReportFormatType, SelfTradePrevention, TimeInForce, TradeType, TriggerType,
};
use kraken_async_rs::response_types::{
    BuySell, ExportReportStatusType, LedgerEntryType, OrderFlag, OrderType,
};
use kraken_async_rs::wss::private::trading_messages::OrderRequestStatus;
use kraken_async_rs::wss::subscribe_messages::SubscriptionName;
use std::str::FromStr;

mod resources;

#[test]
fn test_order_status_request_display() {
    test_display_output(OrderRequestStatus::Ok, "ok");
    test_display_output(OrderRequestStatus::Error, "error");
}

#[test]
fn test_subscription_name_display() {
    test_display_output(SubscriptionName::Book, "book");
    test_display_output(SubscriptionName::Ohlc, "ohlc");
    test_display_output(SubscriptionName::OpenOrders, "openOrders");
    test_display_output(SubscriptionName::OwnTrades, "ownTrades");
    test_display_output(SubscriptionName::Spread, "spread");
    test_display_output(SubscriptionName::Ticker, "ticker");
    test_display_output(SubscriptionName::Trade, "trade");
    test_display_output(SubscriptionName::All, "*");
}

#[test]
fn test_close_time_display() {
    test_display_output(CloseTime::Open, "open");
    test_display_output(CloseTime::Close, "close");
    test_display_output(CloseTime::Both, "both");
}

#[test]
fn test_asset_pair_info_display() {
    test_display_output(AssetPairInfo::Info, "info");
    test_display_output(AssetPairInfo::Fees, "fees");
    test_display_output(AssetPairInfo::Leverage, "leverage");
    test_display_output(AssetPairInfo::Margin, "margin");
}

#[test]
fn test_candlestick_interval_display() {
    test_display_output(CandlestickInterval::Minute, "1");
    test_display_output(CandlestickInterval::Minutes5, "5");
    test_display_output(CandlestickInterval::Minutes15, "15");
    test_display_output(CandlestickInterval::Minutes30, "30");
    test_display_output(CandlestickInterval::Hour, "60");
    test_display_output(CandlestickInterval::Hours4, "240");
    test_display_output(CandlestickInterval::Day, "1440");
    test_display_output(CandlestickInterval::Week, "10080");
    test_display_output(CandlestickInterval::Days15, "21600");
}

#[test]
fn test_trade_type_display() {
    test_display_output(TradeType::All, "all");
    test_display_output(TradeType::AnyPosition, "any position");
    test_display_output(TradeType::ClosedPosition, "closed position");
    test_display_output(TradeType::ClosingPosition, "closing position");
    test_display_output(TradeType::NoPosition, "no position");
}

#[test]
fn test_buy_sell_display() {
    test_display_output(BuySell::Buy, "buy");
    test_display_output(BuySell::Sell, "sell");
}

#[test]
fn test_order_flag_display_and_from_str() {
    test_display_output(OrderFlag::Post, "post");
    test_display_output(OrderFlag::FeesInBase, "fcib");
    test_display_output(OrderFlag::FeesInQuote, "fciq");
    test_display_output(OrderFlag::NoMarketPriceProtection, "nompp");
    test_display_output(OrderFlag::OrderVolumeInQuote, "viqc");

    assert!(matches!(OrderFlag::from_str("post"), Ok(OrderFlag::Post)));
    assert!(matches!(
        OrderFlag::from_str("fcib"),
        Ok(OrderFlag::FeesInBase)
    ));
    assert!(matches!(
        OrderFlag::from_str("fciq"),
        Ok(OrderFlag::FeesInQuote)
    ));
    assert!(matches!(
        OrderFlag::from_str("nompp"),
        Ok(OrderFlag::NoMarketPriceProtection)
    ));
    assert!(matches!(
        OrderFlag::from_str("viqc"),
        Ok(OrderFlag::OrderVolumeInQuote)
    ));
}

#[test]
fn test_order_type_display() {
    test_display_output(OrderType::Market, "market");
    test_display_output(OrderType::Limit, "limit");
    test_display_output(OrderType::StopLoss, "stop-loss");
    test_display_output(OrderType::StopLimit, "stop-limit");
    test_display_output(OrderType::TakeProfit, "take-profit");
    test_display_output(OrderType::StopLossLimit, "stop-loss-limit");
    test_display_output(OrderType::TakeProfitLimit, "take-profit-limit");
    test_display_output(OrderType::SettlePosition, "settle-position");
}

#[test]
fn test_ledger_entry_type_display() {
    test_display_output(LedgerEntryType::None, "none");
    test_display_output(LedgerEntryType::Trade, "trade");
    test_display_output(LedgerEntryType::Credit, "credit");
    test_display_output(LedgerEntryType::Deposit, "deposit");
    test_display_output(LedgerEntryType::Withdrawal, "withdrawal");
    test_display_output(LedgerEntryType::Transfer, "transfer");
    test_display_output(LedgerEntryType::Margin, "margin");
    test_display_output(LedgerEntryType::Rollover, "rollover");
    test_display_output(LedgerEntryType::Spend, "spend");
    test_display_output(LedgerEntryType::Receive, "receive");
    test_display_output(LedgerEntryType::Settled, "settled");
    test_display_output(LedgerEntryType::Adjustment, "adjustment");
    test_display_output(LedgerEntryType::Staking, "staking");
    test_display_output(LedgerEntryType::Sale, "sale");
    test_display_output(LedgerEntryType::Dividend, "dividend");
    test_display_output(LedgerEntryType::NftRebate, "nftrebate");
    test_display_output(LedgerEntryType::NftCreatorFee, "nftcreatorfee");
    test_display_output(LedgerEntryType::NftTrade, "nfttrade");
    test_display_output(LedgerEntryType::CustodyTransfer, "custodytransfer");
}

#[test]
fn test_report_format_type_display() {
    test_display_output(ReportFormatType::Csv, "CSV");
    test_display_output(ReportFormatType::Tsv, "TSV");
}

#[test]
fn test_delete_export_type_display() {
    test_display_output(DeleteExportType::Cancel, "cancel");
    test_display_output(DeleteExportType::Delete, "delete");
}

#[test]
fn test_trigger_type_display() {
    test_display_output(TriggerType::Index, "index");
    test_display_output(TriggerType::Last, "last");
}

#[test]
fn test_self_trade_prevention_display() {
    test_display_output(SelfTradePrevention::CancelNewest, "cancel-newest");
    test_display_output(SelfTradePrevention::CancelOldest, "cancel-oldest");
    test_display_output(SelfTradePrevention::CancelBoth, "cancel-both");
}

#[test]
fn test_time_in_force_display() {
    test_display_output(TimeInForce::GTC, "GTC");
    test_display_output(TimeInForce::IOC, "IOC");
    test_display_output(TimeInForce::GTD, "GTD");
}

#[test]
fn test_request_lock_type_display() {
    test_display_output(LockType::Flex, "flex");
    test_display_output(LockType::Bonded, "bonded");
    test_display_output(LockType::Timed, "timed");
    test_display_output(LockType::Instant, "instant");
}

#[test]
fn test_cursor_display() {
    test_display_output(Cursor::String("something".to_string()), "something");
    test_display_output(Cursor::Bool(false), "false");
}

#[test]
fn test_kraken_error_display() {
    test_display_output(KrakenError::PermissionDenied, "PermissionDenied");
    test_display_output(KrakenError::InvalidKey, "InvalidKey");
    test_display_output(KrakenError::UnknownAssetPair, "UnknownAssetPair");
    test_display_output(KrakenError::InvalidArguments("InvalidArguments:type".to_string()), "InvalidArguments:type");
    test_display_output(KrakenError::InvalidSignature, "InvalidSignature");
    test_display_output(KrakenError::InvalidNonce, "InvalidNonce");
    test_display_output(KrakenError::InvalidSession, "InvalidSession");
    test_display_output(KrakenError::BadRequest, "BadRequest");
    test_display_output(KrakenError::UnknownMethod, "UnknownMethod");
    test_display_output(KrakenError::RateLimitExceeded, "RateLimitExceeded");
    test_display_output(
        KrakenError::TradingRateLimitExceeded,
        "TradingRateLimitExceeded",
    );
    test_display_output(KrakenError::TemporaryLockout, "TemporaryLockout");
    test_display_output(KrakenError::ServiceUnavailable, "ServiceUnavailable");
    test_display_output(KrakenError::ServiceBusy, "ServiceBusy");
    test_display_output(KrakenError::InternalError, "InternalError");
}

#[test]
fn test_export_report_status_type_display() {
    test_display_output(ExportReportStatusType::Queued, "Queued");
    test_display_output(ExportReportStatusType::Processing, "Processing");
    test_display_output(ExportReportStatusType::Processed, "Processed");
}
