//! REST request types
//!
use crate::response_types::{BuySell, LedgerEntryType, OrderFlag, OrderType};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::formats::CommaSeparator;
use serde_with::StringWithSeparator;
use serde_with::{serde_as, skip_serializing_none};
use simple_builder::Builder;
use std::fmt::{Display, Formatter};
use to_query_params::{QueryParams, ToQueryParams};

/// Wrapper type for submitting order cancels by Kraken id (String) or user-ref (Int).
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum IntOrString {
    Int(i64),
    String(String),
}

impl From<i64> for IntOrString {
    fn from(value: i64) -> Self {
        IntOrString::Int(value)
    }
}

impl From<&str> for IntOrString {
    fn from(value: &str) -> Self {
        IntOrString::String(value.to_string())
    }
}

impl From<String> for IntOrString {
    fn from(value: String) -> Self {
        IntOrString::String(value)
    }
}

impl Display for IntOrString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IntOrString::Int(i) => write!(f, "{i}"),
            IntOrString::String(s) => write!(f, "{s}"),
        }
    }
}

/// Time to use when searching for closed orders by start and end timestamps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CloseTime {
    Open,
    Close,
    Both,
}

impl Display for CloseTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CloseTime::Open => write!(f, "open"),
            CloseTime::Close => write!(f, "close"),
            CloseTime::Both => write!(f, "both"),
        }
    }
}

/// Type of information to request for asset pairs.
///
/// Defaults to Info, which is all info.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssetPairInfo {
    Info,
    Leverage,
    Fees,
    Margin,
}

impl Display for AssetPairInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetPairInfo::Info => write!(f, "info"),
            AssetPairInfo::Leverage => write!(f, "leverage"),
            AssetPairInfo::Fees => write!(f, "fees"),
            AssetPairInfo::Margin => write!(f, "margin"),
        }
    }
}

/// All possible candlestick intervals for requesting OHLC data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CandlestickInterval {
    Minute,
    Minutes5,
    Minutes15,
    Minutes30,
    Hour,
    Hours4,
    Day,
    Week,
    Days15,
}

impl Display for CandlestickInterval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CandlestickInterval::Minute => write!(f, "1"),
            CandlestickInterval::Minutes5 => write!(f, "5"),
            CandlestickInterval::Minutes15 => write!(f, "15"),
            CandlestickInterval::Minutes30 => write!(f, "30"),
            CandlestickInterval::Hour => write!(f, "60"),
            CandlestickInterval::Hours4 => write!(f, "240"),
            CandlestickInterval::Day => write!(f, "1440"),
            CandlestickInterval::Week => write!(f, "10080"),
            CandlestickInterval::Days15 => write!(f, "21600"),
        }
    }
}

/// Types of trades to filter for when requesting user's trade history.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TradeType {
    All,
    AnyPosition,
    ClosedPosition,
    ClosingPosition,
    NoPosition,
}

impl Display for TradeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeType::All => write!(f, "all"),
            TradeType::AnyPosition => write!(f, "any position"),
            TradeType::ClosedPosition => write!(f, "closed position"),
            TradeType::ClosingPosition => write!(f, "closing position"),
            TradeType::NoPosition => write!(f, "no position"),
        }
    }
}

/// Wrapper type for a `Vec<OrderFlag>` that serializes to a comma-separated string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderFlags(Vec<OrderFlag>);

impl OrderFlags {
    pub fn new(order_flags: Vec<OrderFlag>) -> OrderFlags {
        OrderFlags(order_flags)
    }
}

impl From<OrderFlag> for OrderFlags {
    fn from(value: OrderFlag) -> Self {
        OrderFlags::new(vec![value])
    }
}

impl Display for OrderFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let strings: Vec<String> = self.0.iter().map(|flag| flag.to_string()).collect();
        write!(f, "{}", strings.join(","))
    }
}

/// Type of report to request generation for.
#[derive(Debug, Clone, PartialEq)]
pub enum ReportType {
    Trades,
    Ledgers,
}

impl Display for ReportType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReportType::Trades => write!(f, "trades"),
            ReportType::Ledgers => write!(f, "ledgers"),
        }
    }
}

/// Format of report, either comma or tab separated values.
#[derive(Debug, Clone, PartialEq)]
pub enum ReportFormatType {
    Csv,
    Tsv,
}

impl Display for ReportFormatType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReportFormatType::Csv => write!(f, "CSV"),
            ReportFormatType::Tsv => write!(f, "TSV"),
        }
    }
}

/// Whether to cancel or delete a requested export report.
#[derive(Debug, Clone, PartialEq)]
pub enum DeleteExportType {
    Cancel,
    Delete,
}

impl Display for DeleteExportType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DeleteExportType::Cancel => write!(f, "cancel"),
            DeleteExportType::Delete => write!(f, "delete"),
        }
    }
}

/// Type of price to use for conditional orders.
///
/// `Index` uses an external price feed while `Last` uses the most recent trade on Kraken.
/// `Last` is the default and fallback if external feeds are unavailable.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TriggerType {
    Index,
    Last,
}

impl Display for TriggerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TriggerType::Index => write!(f, "index"),
            TriggerType::Last => write!(f, "last"),
        }
    }
}

/// Strategy for exchange to take when handling a self-crossing order.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelfTradePrevention {
    CancelNewest,
    CancelOldest,
    CancelBoth,
}

impl Display for SelfTradePrevention {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SelfTradePrevention::CancelNewest => write!(f, "cancel_newest"),
            SelfTradePrevention::CancelOldest => write!(f, "cancel_oldest"),
            SelfTradePrevention::CancelBoth => write!(f, "cancel_both"),
        }
    }
}

/// Time in Force for the given order.
///
/// Good 'til Cancelled
/// Immediate or Cancel (aka Fill or Kill)
/// Good 'til Date (must come with an expiration time in the request)
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum TimeInForce {
    GTC,
    IOC,
    GTD,
}

/// Time in Force for the given order.
///
/// Good 'til Cancelled
/// Immediate or Cancel (aka Fill or Kill)
/// Good 'til Date (must come with an expiration time in the request)
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum TimeInForceV2 {
    GTC,
    IOC,
    GTD,
}

impl Display for TimeInForce {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeInForce::GTC => write!(f, "GTC"),
            TimeInForce::IOC => write!(f, "IOC"),
            TimeInForce::GTD => write!(f, "GTD"),
        }
    }
}

/// Type of lock-up for a given Earn strategy.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LockType {
    Flex,
    Bonded,
    Timed,
    Instant,
}

impl Display for LockType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LockType::Flex => write!(f, "flex"),
            LockType::Bonded => write!(f, "bonded"),
            LockType::Timed => write!(f, "timed"),
            LockType::Instant => write!(f, "instant"),
        }
    }
}

/// Wrapper type for a `Vec<String>` that serializes to comma-separated.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct StringCSV(pub Vec<String>);

impl StringCSV {
    pub fn new(strings: Vec<String>) -> StringCSV {
        StringCSV(strings)
    }
}

impl From<&str> for StringCSV {
    fn from(value: &str) -> Self {
        StringCSV::new(vec![value.to_string()])
    }
}

impl From<String> for StringCSV {
    fn from(value: String) -> Self {
        StringCSV::new(vec![value])
    }
}

impl From<&String> for StringCSV {
    fn from(value: &String) -> Self {
        StringCSV::new(vec![value.clone()])
    }
}

impl Display for StringCSV {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join(","))
    }
}

/// A request for details on a particular asset, such as "BTC", "ETH", or "USDC".
///
/// [StringCSV] takes a `Vec<String>` and formats them in queries as comma-separated.
#[derive(Debug, Clone, QueryParams, Builder)]
pub struct AssetInfoRequest {
    pub asset: Option<StringCSV>,
    #[query(rename = "aclass")]
    pub asset_class: Option<String>,
}

/// A request for details on a particular trading pair, such as "BTCUSD", "DOGEUSDT", or "ETHUSD".
///
/// [StringCSV] takes a `Vec<String>` and formats them in queries as comma-separated.
#[derive(Debug, Clone, QueryParams, Builder)]
pub struct TradableAssetPairsRequest {
    pub pair: Option<StringCSV>,
    pub info: Option<AssetPairInfo>,
    pub country_code: Option<String>,
}

/// A request for common ticker info for one or many pairs.
///
/// [StringCSV] takes a `Vec<String>` and formats them in queries as comma-separated.
#[derive(Debug, Clone, QueryParams, Builder)]
pub struct TickerRequest {
    pub pair: Option<StringCSV>,
}

/// A request for OHLC data for a single pair, optionally providing a `since` to retrieve
/// incremental updates.
#[derive(Debug, Clone, QueryParams, Builder)]
pub struct OHLCRequest {
    #[query(required)]
    #[builder(required)]
    pub pair: String,
    pub interval: Option<CandlestickInterval>,
    pub since: Option<i64>,
}

/// A request for the orderbook of a pair, optionally at a given depth of bids and asks
/// (`count` parameter).
#[derive(Debug, Clone, QueryParams, Builder)]
pub struct OrderbookRequest {
    #[query(required)]
    #[builder(required)]
    pub pair: String,
    pub count: Option<i64>,
}

/// A fully-paginated request for trades from a particular pair.
///
/// `since` can be set to 0 to get the very first trades recorded on Kraken, or set to the `last`
/// value provided in the response for full pagination.
///
/// See examples/live_retrieving_recent_traders.rs for an example of completing a paginated request.
#[derive(Debug, Clone, QueryParams, Builder)]
pub struct RecentTradesRequest {
    #[query(required)]
    #[builder(required)]
    pub pair: String,
    pub since: Option<i64>,
    pub count: Option<i64>,
}

/// Retrieve the most recent bid/ask spreads for a given pair, optionally with a `since` parameter
/// to receive only incremental updates.
#[derive(Debug, Clone, QueryParams, Builder)]
pub struct RecentSpreadsRequest {
    #[query(required)]
    #[builder(required)]
    pub pair: String,
    pub since: Option<i64>,
}

/// A request for margin trading data, optionally only for a specific pair.
#[derive(Debug, Clone, QueryParams, Builder)]
pub struct TradeBalanceRequest {
    pub asset: Option<String>,
}

/// A request for all open orders on the account.
///
/// Optionally returns trades associated with each order if `trades` is true, and can be filtered by
/// a provided user ref value.
#[derive(Debug, Clone, QueryParams, Builder)]
pub struct OpenOrdersRequest {
    pub trades: Option<bool>,
    pub userref: Option<i64>,
    #[query(rename = "cl_ord_id")]
    pub client_order_id: Option<String>,
}

/// A request to retrieve historical orders, 50 at a time.
///
/// `start` and `end` provide epoch-time bounds to query, while offset provides pagination within
/// that window.
#[derive(Debug, Clone, QueryParams, Builder)]
pub struct ClosedOrdersRequest {
    pub trades: Option<bool>,
    pub userref: Option<i64>,
    pub start: Option<i64>,
    pub end: Option<i64>,
    #[query(rename = "ofs")]
    pub offset: Option<i64>,
    #[query(rename = "closetime")]
    pub close_time: Option<CloseTime>,
    #[query(rename = "cl_ord_id")]
    pub client_order_id: Option<String>,
}

/// A request for the details of up to 50 orders by id.
///
/// Optionally including trade ids, filtering by user-ref, and consolidating trades by taker.
#[derive(Debug, Clone, QueryParams, Builder)]
pub struct OrderRequest {
    #[builder(required)]
    #[query(required, rename = "txid")]
    pub tx_id: StringCSV,
    pub trades: Option<bool>,
    pub userref: Option<i64>,
    pub consolidate_taker: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Builder)]
pub struct OrderAmendsRequest {
    #[builder(required)]
    order_id: String,
}

/// A request for any historical trades for the account.
///
/// This request is fully paginated by epoch time using the `start` and `end` parameters, in
/// conjunction with the `offset` parameter.
#[derive(Debug, Clone, QueryParams, Builder, PartialEq, Eq)]
pub struct TradesHistoryRequest {
    #[query(rename = "type")]
    pub trade_type: Option<TradeType>,
    pub trades: Option<bool>,
    pub start: Option<i64>,
    pub end: Option<i64>,
    #[query(rename = "ofs")]
    pub offset: Option<i64>,
    pub consolidate_taker: Option<bool>,
    pub ledgers: Option<bool>,
}

/// A request for details of up to 50 trades by ref id.
#[derive(Debug, Clone, QueryParams, Builder)]
pub struct TradeInfoRequest {
    #[builder(required)]
    #[query(required, rename = "txid")]
    pub tx_id: StringCSV,
    pub trades: Option<bool>,
}

/// A request for details about an open margin position.
#[derive(Debug, Clone, QueryParams, Builder)]
pub struct OpenPositionsRequest {
    #[query(rename = "txid")]
    pub tx_id: Option<String>,
    #[query(rename = "docalcs")]
    pub do_calcs: Option<bool>,
    pub consolidation: Option<String>,
}

/// A request for 50 ledger entries for the account.
///
/// This request is fully paginated by epoch time using the `start` and `end` parameters, in
/// conjunction with the `offset` parameter.
#[derive(Debug, Clone, QueryParams, Builder)]
pub struct LedgersInfoRequest {
    pub asset: Option<StringCSV>,
    #[query(rename = "aclass")]
    pub asset_class: Option<String>,
    #[query(rename = "type")]
    pub entry_type: Option<LedgerEntryType>,
    pub start: Option<i64>,
    pub end: Option<i64>,
    #[query(rename = "ofs")]
    pub offset: Option<i64>,
    pub without_count: Option<bool>,
}

/// A request for details of up to 20 ledger entries by id.
#[derive(Debug, Clone, QueryParams, Builder)]
pub struct QueryLedgerRequest {
    #[query(required)]
    #[builder(required)]
    pub id: StringCSV,
    pub trades: Option<bool>,
}

/// A request for cumulative 30-day USD trading volume for the account.
///
/// Optionally including fees if a particular pairs are requested.
#[derive(Debug, Clone, QueryParams, Builder)]
pub struct TradeVolumeRequest {
    pub pair: Option<StringCSV>,
}

/// A request for the asynchronous generation of a report of "trades" or "ledgers".
#[derive(Debug, Clone, QueryParams, Builder)]
pub struct ExportReportRequest {
    #[builder(required)]
    #[query(required)]
    pub report: ReportType,
    pub format: Option<ReportFormatType>,
    #[builder(required)]
    #[query(required)]
    pub description: String,
    pub fields: Option<String>,
    #[query(rename = "starttm")]
    pub start_time: Option<i64>,
    #[query(rename = "endtm")]
    pub end_time: Option<i64>,
}

/// A request for the status of a requested export report.
#[derive(Debug, Clone, QueryParams, Builder)]
pub struct ExportReportStatusRequest {
    #[builder(required)]
    #[query(required)]
    pub report: ReportType,
}

/// A request to retrieve a specific export report by id.
#[derive(Debug, Clone, QueryParams, Builder)]
pub struct RetrieveExportReportRequest {
    #[builder(required)]
    #[query(required)]
    pub id: String,
}

/// A request to delete an export report by id.
#[derive(Debug, Clone, QueryParams, Builder)]
pub struct DeleteExportRequest {
    #[builder(required)]
    #[query(required)]
    pub id: String,
    #[builder(required)]
    #[query(required, rename = "type")]
    pub delete_type: DeleteExportType,
}

/// A request to create a new spot order.
#[derive(Debug, Clone, QueryParams, Builder, PartialEq, Eq)]
pub struct AddOrderRequest {
    #[query(rename = "userref")]
    pub user_ref: Option<i64>,
    #[query(rename = "cl_ord_id")]
    pub client_order_id: Option<String>,
    #[builder(required)]
    #[query(required, rename = "ordertype")]
    pub order_type: OrderType,
    #[builder(required)]
    #[query(required, rename = "type")]
    pub side: BuySell,
    #[builder(required)]
    #[query(required)]
    pub volume: Decimal,
    #[query(rename = "displayvol")]
    pub display_volume: Option<Decimal>,
    #[builder(required)]
    #[query(required)]
    pub pair: String,
    #[query(rename = "reqid")]
    pub req_id: Option<i64>,
    pub price: Option<Decimal>,
    #[query(rename = "price2")]
    pub price_2: Option<Decimal>,
    pub trigger: Option<TriggerType>,
    pub leverage: Option<i64>,
    pub reduce_only: Option<bool>,
    #[query(rename = "stptype")]
    pub stp_type: Option<SelfTradePrevention>,
    #[query(rename = "oflags")]
    pub order_flags: Option<OrderFlags>,
    #[query(rename = "timeinforce")]
    pub time_in_force: Option<TimeInForce>,
    #[query(rename = "starttm")]
    pub start_time: Option<String>,
    #[query(rename = "expiretm")]
    pub expire_time: Option<String>,
    #[query(rename = "close[ordertype]")]
    pub close_order_type: Option<String>,
    #[query(rename = "close[price]")]
    pub close_price: Option<Decimal>,
    #[query(rename = "close[price2]")]
    pub close_price_2: Option<Decimal>,
    pub deadline: Option<String>,
    pub validate: Option<bool>,
}

/// A request to create up to 15 spot orders in a batch.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Builder)]
pub struct AddBatchedOrderRequest {
    #[builder(required)]
    pub orders: Vec<BatchedOrderRequest>,
    #[builder(required)]
    pub pair: String,
    pub deadline: Option<String>,
    pub validate: Option<bool>,
}

/// An individual order request to be placed in a batch.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
pub struct BatchedOrderRequest {
    #[serde(rename = "userref")]
    pub user_ref: Option<i64>,
    #[serde(rename = "cl_ord_id")]
    pub client_order_id: Option<String>,
    #[builder(required)]
    #[serde(rename = "ordertype")]
    pub order_type: OrderType,
    #[builder(required)]
    #[serde(rename = "type")]
    pub side: BuySell,
    #[builder(required)]
    pub volume: Decimal,
    #[serde(rename = "displayvol")]
    pub display_volume: Option<Decimal>,
    pub price: Option<Decimal>,
    #[serde(rename = "price2")]
    pub price_2: Option<Decimal>,
    pub trigger: Option<TriggerType>,
    pub leverage: Option<i64>,
    pub reduce_only: Option<bool>,
    #[serde(rename = "stptype")]
    pub stp_type: Option<String>,
    #[serde(rename = "oflags")]
    #[serde(default)]
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, OrderFlag>>")]
    pub order_flags: Option<Vec<OrderFlag>>,
    #[serde(rename = "timeinforce")]
    pub time_in_force: Option<TimeInForce>,
    #[serde(rename = "starttm")]
    pub start_time: Option<String>,
    #[serde(rename = "expiretm")]
    pub expire_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Builder)]
pub struct AmendOrderRequest {
    #[serde(rename = "txid")]
    pub tx_id: Option<String>,
    #[serde(rename = "cl_ord_id")]
    pub client_order_id: Option<String>,
    #[serde(rename = "order_qty")]
    pub order_quantity: Option<Decimal>,
    #[serde(rename = "display_qty")]
    pub display_quantity: Option<Decimal>,
    pub limit_price: Option<String>,
    pub trigger_price: Option<String>,
    pub post_only: Option<bool>,
    pub deadline: Option<String>, // RFC-3339
}

/// A request to edit an existing order.
#[derive(Debug, Clone, QueryParams, Builder)]
pub struct EditOrderRequest {
    #[query(rename = "userref")]
    pub user_ref: Option<i64>,
    #[query(required, rename = "txid")]
    #[builder(required)]
    pub tx_id: String,
    #[builder(required)]
    #[query(required)]
    pub volume: Decimal,
    #[query(rename = "displayvol")]
    pub display_volume: Option<Decimal>,
    #[builder(required)]
    #[query(required)]
    pub pair: String,
    pub price: Option<Decimal>,
    #[query(rename = "price2")]
    pub price_2: Option<Decimal>,
    #[query(rename = "oflags")]
    pub order_flags: Option<OrderFlags>,
    pub deadline: Option<String>,
    pub cancel_response: Option<bool>,
    pub validate: Option<bool>,
}

/// A request to cancel an order by txid (String) or userref (Int).
#[derive(Debug, Clone, QueryParams, Builder)]
pub struct CancelOrderRequest {
    #[query(required, rename = "txid")]
    #[builder(required)]
    pub tx_id: IntOrString,
    #[query(rename = "cl_ord_id")]
    pub client_order_id: Option<String>,
}

/// A "dead man's switch" for all active orders.
///
/// Once set to a timestamp, this must be continually called to prevent all orders from being
/// cancelled.
#[derive(Debug, Clone, QueryParams, Builder)]
pub struct CancelAllOrdersAfterRequest {
    #[builder(required)]
    #[query(required)]
    pub timeout: i64,
}

/// A request to cancel up to 50 orders in a batch by tx id or user ref.
#[derive(Debug, Clone, Builder, Serialize)]
pub struct CancelBatchOrdersRequest {
    #[builder(required)]
    pub orders: Vec<IntOrString>,
    #[serde(rename = "cl_ord_ids")]
    pub client_order_ids: Option<Vec<String>>,
}

impl CancelBatchOrdersRequest {
    pub fn from_user_refs(refs: Vec<i64>) -> CancelBatchOrdersRequest {
        CancelBatchOrdersRequest {
            orders: refs.into_iter().map(IntOrString::Int).collect(),
            client_order_ids: None,
        }
    }

    pub fn from_tx_ids(ids: Vec<String>) -> CancelBatchOrdersRequest {
        CancelBatchOrdersRequest {
            orders: ids.into_iter().map(IntOrString::String).collect(),
            client_order_ids: None,
        }
    }

    pub fn from_client_order_ids(ids: Vec<String>) -> CancelBatchOrdersRequest {
        CancelBatchOrdersRequest {
            orders: vec![],
            client_order_ids: Some(ids),
        }
    }
}

/// A request for all available deposit methods for a given asset.
#[derive(Debug, Clone, Builder, QueryParams)]
pub struct DepositMethodsRequest {
    #[builder(required)]
    #[query(required)]
    pub asset: String,
    pub aclass: Option<String>,
}

/// A request to retrieve or generate a deposit address for a particular asset and method.
#[derive(Debug, Clone, Builder, QueryParams)]
pub struct DepositAddressesRequest {
    #[query(required)]
    #[builder(required)]
    pub asset: String,
    #[query(required)]
    #[builder(required)]
    pub method: String,
    #[query(rename = "new")]
    pub is_new: Option<bool>,
    pub amount: Option<Decimal>, // only for Lightning network
}

/// A request for all available withdrawal methods for the user.
#[derive(Debug, Clone, Builder, QueryParams)]
pub struct WithdrawalMethodsRequest {
    pub asset: Option<String>,
    #[query(rename = "aclass")]
    pub asset_class: Option<String>,
    pub network: Option<String>,
}

/// A request to retrieve or generate a withdrawal address for a particular asset and method.
#[derive(Debug, Clone, Builder, QueryParams)]
pub struct WithdrawalAddressesRequest {
    pub asset: Option<String>,
    #[query(rename = "aclass")]
    pub asset_class: Option<String>,
    pub method: Option<String>,
    pub key: Option<String>,
    pub verified: Option<bool>,
}

/// A sub-type for specifying if paginating (Bool), or providing a cursor for the next page (String).
#[derive(Debug, Clone)]
pub enum Cursor {
    String(String),
    Bool(bool),
}

impl Display for Cursor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cursor::String(str) => write!(f, "{}", str),
            Cursor::Bool(b) => write!(f, "{}", b),
        }
    }
}

/// A request for the status of a deposit or withdrawal request.
#[derive(Debug, Clone, Builder, QueryParams)]
pub struct StatusOfDepositWithdrawRequest {
    pub asset: Option<String>,
    #[query(rename = "aclass")]
    pub asset_class: Option<String>,
    pub method: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub cursor: Option<Cursor>,
    pub limit: Option<i64>,
}

/// A request for the limit, amount and fee to withdraw asset.
#[derive(Debug, Clone, Builder, QueryParams)]
pub struct WithdrawalInfoRequest {
    #[builder(required)]
    #[query(required)]
    pub asset: String,
    #[builder(required)]
    #[query(required)]
    pub key: String,
    #[builder(required)]
    #[query(required)]
    pub amount: Decimal,
}

/// A request to withdraw funds.
#[derive(Debug, Clone, Builder, QueryParams)]
pub struct WithdrawFundsRequest {
    #[builder(required)]
    #[query(required)]
    pub asset: String,
    #[builder(required)]
    #[query(required)]
    pub key: String,
    #[builder(required)]
    #[query(required)]
    pub amount: Decimal,
    pub address: Option<String>,
    pub max_fee: Option<Decimal>,
}

/// A request to cancel an active withdrawal.
#[derive(Debug, Clone, Builder, QueryParams)]
pub struct WithdrawCancelRequest {
    #[builder(required)]
    #[query(required)]
    pub asset: String,
    #[builder(required)]
    #[query(required, rename = "refid")]
    pub ref_id: String,
}

/// A request to transfer from the account's Spot wallet to Future's wallet.
#[derive(Debug, Clone, Builder, QueryParams)]
pub struct WalletTransferRequest {
    #[builder(required)]
    #[query(required)]
    pub asset: String,
    #[builder(required)]
    #[query(required)]
    pub from: String,
    #[builder(required)]
    #[query(required)]
    pub to: String,
    #[builder(required)]
    #[query(required)]
    pub amount: Decimal,
}

/// A request to create a sub-account for trading.
#[derive(Debug, Clone, Builder, QueryParams)]
pub struct CreateSubAccountRequest {
    #[builder(required)]
    #[query(required)]
    pub username: String,
    #[builder(required)]
    #[query(required)]
    pub email: String,
}

/// A request to transfer assets between sub-accounts.
#[derive(Debug, Clone, Builder, QueryParams)]
pub struct AccountTransferRequest {
    #[builder(required)]
    #[query(required)]
    pub asset: String,
    #[builder(required)]
    #[query(required)]
    pub amount: Decimal,
    #[builder(required)]
    #[query(required)]
    pub from: String,
    #[builder(required)]
    #[query(required)]
    pub to: String,
}

/// A request to allocate funds to a particular Earn strategy.
#[derive(Debug, Clone, Builder, QueryParams)]
pub struct AllocateEarnFundsRequest {
    #[builder(required)]
    #[query(required)]
    pub amount: Decimal,
    #[builder(required)]
    #[query(required)]
    pub strategy_id: String,
}

/// A request for the allocation status for a given strategy.
#[derive(Debug, Clone, Builder, QueryParams)]
pub struct EarnAllocationStatusRequest {
    #[builder(required)]
    #[query(required)]
    pub strategy_id: String,
}

/// A request for all earn strategies.
///
/// Pagination is available via the `cursor` and `limit` parameters.
#[derive(Debug, Clone, Builder, QueryParams)]
pub struct ListEarnStrategiesRequest {
    pub ascending: Option<bool>,
    pub asset: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<u16>,
    pub lock_type: Option<LockType>,
}

/// A request to list all current earn strategy allocations.
#[derive(Debug, Clone, Builder, QueryParams)]
pub struct ListEarnAllocationsRequest {
    pub ascending: Option<bool>,
    pub converted_asset: Option<String>,
    pub hide_zero_allocations: Option<bool>,
}

#[cfg(test)]
mod tests {
    use crate::request_types::{CancelBatchOrdersRequest, IntOrString, OrderFlags, StringCSV};
    use crate::response_types::OrderFlag;

    #[test]
    fn test_cancel_batch_order_request_ids() {
        let request =
            CancelBatchOrdersRequest::from_tx_ids(vec!["M97YKE-HHCTY-2GRVXU".to_string()]);

        let expected = vec![IntOrString::String("M97YKE-HHCTY-2GRVXU".to_string())];
        assert_eq!(expected, request.orders);
    }

    #[test]
    fn test_cancel_batch_order_request_user_refs() {
        let request = CancelBatchOrdersRequest::from_user_refs(vec![42]);

        let expected = vec![IntOrString::Int(42)];
        assert_eq!(expected, request.orders);
    }

    #[test]
    fn test_string_csv_conversions() {
        let expected_string_csv = StringCSV::new(vec!["post".to_string()]);

        let from_str: StringCSV = "post".into();
        let from_string: StringCSV = "post".to_string().into();

        let string_ref: &String = &("post".to_string());
        let from_string_ref: StringCSV = string_ref.into();

        assert_eq!(expected_string_csv, from_str);
        assert_eq!(expected_string_csv, from_string);
        assert_eq!(expected_string_csv, from_string_ref);
    }

    #[test]
    fn test_order_flag_conversions() {
        let expected_order_flag = OrderFlags::new(vec![OrderFlag::NoMarketPriceProtection]);

        let order_flags: OrderFlags = OrderFlag::NoMarketPriceProtection.into();

        assert_eq!(expected_order_flag, order_flags);
    }

    #[test]
    fn test_int_or_string_conversions() {
        let expected_int = IntOrString::Int(42);
        let expected_string = IntOrString::String("someString".to_string());

        let int: IntOrString = 42.into();
        let str: IntOrString = "someString".into();
        let string: IntOrString = "someString".to_string().into();

        assert_eq!(expected_int, int);
        assert_eq!(expected_string, str);
        assert_eq!(expected_string, string);
    }
}
