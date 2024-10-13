//! REST response types
use crate::clients::errors::ClientError;
use crate::crypto::secrets::Token;
use crate::request_types::TriggerType;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_this_or_that::as_i64;
use serde_tuple::Deserialize_tuple;
use serde_with::formats::CommaSeparator;
use serde_with::StringWithSeparator;
use serde_with::{serde_as, DisplayFromStr};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

/// A user's level of KYC verification with Kraken
///
/// Determines rate limits for the user, as well as deposit, withdrawal, and banking limits.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "snake_case")]
pub enum VerificationTier {
    Intermediate,
    Pro,
}

/// Status of the exchange
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "snake_case")]
pub enum SystemStatus {
    Online,
    Maintenance,
    CancelOnly,
    PostOnly,
}

/// Status of a given asset pair for trading (e.g. BTC-USD, ATOM-USD)
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "snake_case")]
pub enum TradableAssetStatus {
    Online,
    CancelOnly,
    PostOnly,
    LimitOnly,
    ReduceOnly,
}

/// Status for an asset (e.g. ETH, ATOM, USDC)
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "snake_case")]
pub enum AssetStatus {
    Enabled,
    DepositOnly,
    WithdrawalOnly,
    FundingTemporarilyDisabled,
}

/// Order side
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "lowercase")]
pub enum BuySell {
    Buy,
    Sell,
}

impl Display for BuySell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BuySell::Buy => write!(f, "buy"),
            BuySell::Sell => write!(f, "sell"),
        }
    }
}

/// Flags that can be applied to order requests.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Copy)]
pub enum OrderFlag {
    /// Post only order will be rejected if it would pay maker fees
    #[serde(rename = "post")]
    Post,
    /// Fees should be taken in the base currency (default for sell)
    #[serde(rename = "fcib")]
    FeesInBase,
    /// Fees should be taken in the quote currency (default for buy)
    #[serde(rename = "fciq")]
    FeesInQuote,
    /// Disable extreme slippage protection for this order
    #[serde(rename = "nompp")]
    NoMarketPriceProtection,
    /// For market orders, give order volume in quote currency
    #[serde(rename = "viqc")]
    OrderVolumeInQuote,
}

impl Display for OrderFlag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderFlag::Post => write!(f, "post"),
            OrderFlag::FeesInBase => write!(f, "fcib"),
            OrderFlag::FeesInQuote => write!(f, "fciq"),
            OrderFlag::NoMarketPriceProtection => write!(f, "nompp"),
            OrderFlag::OrderVolumeInQuote => write!(f, "viqc"),
        }
    }
}

impl FromStr for OrderFlag {
    type Err = ClientError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "post" => Ok(OrderFlag::Post),
            "fcib" => Ok(OrderFlag::FeesInBase),
            "fciq" => Ok(OrderFlag::FeesInQuote),
            "nompp" => Ok(OrderFlag::NoMarketPriceProtection),
            "viqc" => Ok(OrderFlag::OrderVolumeInQuote),
            _ => Err(ClientError::Parse("Failed to parse order flag")),
        }
    }
}

/// Whether a given [BidAsk] is a `Bid` or an `Ask`
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "lowercase")]
pub enum BidOrAsk {
    Bid,
    Ask,
}

/// Single-character enum for buy and sell
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Copy)]
pub enum BuySellChar {
    #[serde(rename(deserialize = "b"))]
    Buy,
    #[serde(rename(deserialize = "s"))]
    Sell,
}

/// Single-character enum for market and limit orders
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Copy)]
pub enum MarketLimitChar {
    #[serde(rename(deserialize = "m"))]
    Market,
    #[serde(rename(deserialize = "l"))]
    Limit,
}

/// Order type, e.g. `Market`, `Limit`, `StopLossLimit`
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum OrderType {
    Limit,
    Market,
    Iceberg, // TODO: maybe not available on WSS AddOrder?
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
    TrailingStop,
    TrailingStopLimit,
    SettlePosition,
}

/// Trade type, separate from [OrderType] due to different serialization semantics
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Copy)]
pub enum TradeType {
    #[serde(rename = "market")]
    Market,
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "stop loss")]
    StopLoss,
    #[serde(rename = "stop limit")]
    StopLimit,
    #[serde(rename = "take profit")]
    TakeProfit,
    #[serde(rename = "stop loss limit")]
    StopLossLimit,
    #[serde(rename = "take profit limit")]
    TakeProfitLimit,
    #[serde(rename = "settle position")]
    SettlePosition,
}

impl Display for OrderType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderType::Market => write!(f, "market"),
            OrderType::Limit => write!(f, "limit"),
            OrderType::StopLoss => write!(f, "stop-loss"),
            OrderType::TakeProfit => write!(f, "take-profit"),
            OrderType::StopLossLimit => write!(f, "stop-loss-limit"),
            OrderType::TakeProfitLimit => write!(f, "take-profit-limit"),
            OrderType::SettlePosition => write!(f, "settle-position"),
            OrderType::Iceberg => write!(f, "iceberg"),
            OrderType::TrailingStop => write!(f, "trailing-stop"),
            OrderType::TrailingStopLimit => write!(f, "trailing-stop-limit"),
        }
    }
}

/// Status of an order
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Open,
    Closed,
    Canceled,
    Expired,
}

/// Status of an order
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatusV2 {
    PendingNew,
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    Expired,
}

/// Status of a position
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "lowercase")]
pub enum PositionStatus {
    Open,
    Closed,
}

/// Status of a position
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "lowercase")]
pub enum PositionStatusV2 {
    Opened,
    Closing,
    Closed,
}

/// Type of ledger entry in user's ledger
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "lowercase")]
pub enum LedgerEntryType {
    None,
    Trade,
    Credit,
    Deposit,
    Withdrawal,
    Transfer,
    Margin,
    Rollover,
    Spend,
    Receive,
    Settled,
    Adjustment,
    Staking,
    Sale,
    Dividend,
    NftRebate,
    NftTrade,
    NftCreatorFee,
    CustodyTransfer,
}

impl Display for LedgerEntryType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LedgerEntryType::None => write!(f, "none"),
            LedgerEntryType::Trade => write!(f, "trade"),
            LedgerEntryType::Credit => write!(f, "credit"),
            LedgerEntryType::Deposit => write!(f, "deposit"),
            LedgerEntryType::Withdrawal => write!(f, "withdrawal"),
            LedgerEntryType::Transfer => write!(f, "transfer"),
            LedgerEntryType::Margin => write!(f, "margin"),
            LedgerEntryType::Rollover => write!(f, "rollover"),
            LedgerEntryType::Spend => write!(f, "spend"),
            LedgerEntryType::Receive => write!(f, "receive"),
            LedgerEntryType::Settled => write!(f, "settled"),
            LedgerEntryType::Adjustment => write!(f, "adjustment"),
            LedgerEntryType::Staking => write!(f, "staking"),
            LedgerEntryType::Sale => write!(f, "sale"),
            LedgerEntryType::Dividend => write!(f, "dividend"),
            LedgerEntryType::NftRebate => write!(f, "nftrebate"),
            LedgerEntryType::NftTrade => write!(f, "nfttrade"),
            LedgerEntryType::NftCreatorFee => write!(f, "nftcreatorfee"),
            LedgerEntryType::CustodyTransfer => write!(f, "custodytransfer"),
        }
    }
}

/// Status of a requested export report
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Copy)]
pub enum ExportReportStatusType {
    Queued,
    Processing,
    Processed,
}

impl Display for ExportReportStatusType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportReportStatusType::Queued => write!(f, "Queued"),
            ExportReportStatusType::Processing => write!(f, "Processing"),
            ExportReportStatusType::Processed => write!(f, "Processed"),
        }
    }
}

/// Status of an edit requested for an order
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "lowercase")]
pub enum OrderEditStatus {
    Ok,
    Err,
}

/// Wrapper type for odd responses that contain either a `bool` or a `String`
///
/// For example, the limit of a deposit method can be `false` for no limit, or a String value of the
/// numeric limit.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum BoolOrString {
    Bool(bool),
    String(String),
}

/// Additional status properties about a deposit or withdrawal
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum StatusProp {
    CancelPending,
    Canceled,
    CancelDenied,
    Return,
    #[serde(rename = "onhold")]
    OnHold,
}

/// Status of a requested transfer
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum TransferStatus {
    Initial,
    Pending,
    Settled,
    Success,
    Failure,
}

/// Status of a transfer between accounts
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AccountTransferStatus {
    Pending,
    Complete,
}

/// Wrapper type for loose typing of allocation/earn fees
#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
#[serde(untagged)]
pub enum EarnFee {
    Decimal(Decimal),
    Integer(i64),
    Float(f64),
}

/// Source of yield for a given earn strategy
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum YieldSourceType {
    Staking,
    OffChain,
    OptInRewards,
}

/// Type of compounding for a given strategy
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum AutoCompoundType {
    Enabled,
    Disabled,
    Optional,
}

/// Type of asset lock-up for a given earn strategy
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum LockType {
    Flex,
    Bonded,
    Instant,
}

/// Kraken server time given in both unix timestamp and RFC1123
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct SystemTime {
    #[serde(rename = "unixtime")]
    pub unix_time: i64,
    pub rfc1123: String,
}

/// Kraken server status, including an RFC3339 timestamp.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct SystemStatusInfo {
    pub status: SystemStatus,
    pub timestamp: String,
}

/// Asset details (e.g. for ETH, USDC, BTC, etc)
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct AssetInfo {
    #[serde(rename = "aclass")]
    pub asset_class: String,
    #[serde(rename = "altname")]
    pub alt_name: String,
    pub decimals: i64,
    pub display_decimals: i64,
    pub collateral_value: Option<f64>,
    pub status: AssetStatus,
}

/// Tiered fee description
#[derive(Debug, Deserialize_tuple, PartialEq, Clone)]
pub struct FeeByVolume {
    pub volume: f64,
    pub fee: f64,
}

/// Trading pair details, including all necessary details for formatting orders
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TradableAssetPair {
    #[serde(rename = "altname")]
    pub alt_name: String,
    #[serde(rename = "wsname")]
    pub ws_name: String,
    #[serde(rename = "aclass_base")]
    pub asset_class_base: String,
    pub base: String,
    #[serde(rename = "aclass_quote")]
    pub asset_class_quote: String,
    pub quote: String,
    pub lot: String,
    pub cost_decimals: i64,
    pub pair_decimals: i64,
    pub lot_decimals: i64,
    pub lot_multiplier: i64,
    pub leverage_buy: Vec<i64>,
    pub leverage_sell: Vec<i64>,
    pub fees: Vec<FeeByVolume>,
    pub fees_maker: Vec<FeeByVolume>,
    pub fee_volume_currency: String,
    pub margin_call: i64,
    pub margin_stop: i64,
    #[serde(rename = "ordermin")]
    pub order_min: Decimal,
    #[serde(rename = "costmin")]
    pub cost_min: Decimal,
    pub tick_size: Decimal,
    pub status: TradableAssetStatus,
    pub long_position_limit: Option<i64>,
    pub short_position_limit: Option<i64>,
}

/// Ticker containing trade count data for the last 24 hours
#[derive(Debug, Deserialize_tuple, PartialEq, Clone)]
pub struct TickerTrades {
    pub today: i64,
    pub last_24_h: i64,
}

/// Ticker helper type to serve differently typed data for the last 24 hours.
#[derive(Debug, Deserialize_tuple, PartialEq, Clone)]
pub struct TickerDecimal {
    pub today: Decimal,
    pub last_24_h: Decimal,
}

/// Best bid or ask
///
/// Separate type needed for varying data format from REST API.
#[derive(Debug, Deserialize_tuple, PartialEq, Clone)]
pub struct RestTickerBidAsk {
    pub price: Decimal,
    pub whole_lot_volume: Decimal,
    pub lot_volume: Decimal,
}

/// Best bid or ask
///
/// Separate type needed for different format from WSS API.
#[derive(Debug, Deserialize_tuple, PartialEq, Clone)]
pub struct TickerBidAsk {
    pub price: Decimal,
    #[serde(deserialize_with = "as_i64")]
    pub whole_lot_volume: i64,
    pub lot_volume: Decimal,
}

/// Price and volume for the most recent trade
#[derive(Debug, Deserialize_tuple, PartialEq, Clone)]
pub struct LastTrade {
    pub price: Decimal,
    pub volume: Decimal,
}

/// Complete ticker information for an asset
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RestTickerInfo {
    #[serde(rename(deserialize = "a"))]
    pub asks: TickerBidAsk,
    #[serde(rename(deserialize = "b"))]
    pub bids: TickerBidAsk,
    #[serde(rename(deserialize = "c"))]
    pub closed: LastTrade,
    #[serde(rename(deserialize = "v"))]
    pub volume: TickerDecimal,
    #[serde(rename(deserialize = "p"))]
    pub vwap: TickerDecimal,
    #[serde(rename(deserialize = "t"))]
    pub trades: TickerTrades,
    #[serde(rename(deserialize = "l"))]
    pub low: TickerDecimal,
    #[serde(rename(deserialize = "h"))]
    pub high: TickerDecimal,
    #[serde(rename(deserialize = "o"))]
    pub open: Decimal,
}

/// Candlestick data for the given interval
#[derive(Debug, Deserialize_tuple, PartialEq, Clone)]
pub struct OHLC {
    pub time: i64,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub vwap: Decimal,
    pub volume: Decimal,
    pub count: i64,
}

/// OHLC data by pair
///
/// Includes `last` value for use in incremental updates
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct OhlcResponse {
    pub last: i64,
    #[serde(flatten)]
    pub ohlc: HashMap<String, Vec<OHLC>>,
}

/// Bid or Ask
///
/// Identical data for bids and asks, only context determines if it's a bid or ask.
#[derive(Debug, Deserialize_tuple, PartialEq, Clone)]
pub struct BidAsk {
    pub price: Decimal,
    pub volume: Decimal,
    pub time: i64,
}

/// Orderbook containing some depth of bids and asks
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Orderbook {
    pub asks: Vec<BidAsk>,
    pub bids: Vec<BidAsk>,
}

/// A public trade
///
/// The model is the same regardless of if request to be consolidated by taker
#[derive(Debug, Deserialize_tuple, PartialEq, Clone)]
pub struct RecentTrade {
    pub price: Decimal,
    pub volume: Decimal,
    pub time: f64,
    pub buy_sell: BuySellChar,
    pub market_limit: MarketLimitChar,
    pub misc: String,
    pub trade_id: i64,
}

/// Wrapper type for recent trade response
///
/// `last` parameter allows for pagination.
#[serde_as]
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RecentTrades {
    #[serde_as(as = "DisplayFromStr")]
    pub last: i64,
    #[serde(flatten)]
    pub trades: HashMap<String, Vec<RecentTrade>>,
}

/// Bid-ask spread at a given time
#[derive(Debug, Deserialize_tuple, PartialEq, Clone)]
pub struct Spread {
    pub time: i64,
    pub bid: Decimal,
    pub ask: Decimal,
}

/// Spreads for one or many assets
///
/// `last` parameter allows for incremental updates
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RecentSpreads {
    pub last: i64,
    #[serde(flatten)]
    pub spreads: HashMap<String, Vec<Spread>>,
}

/// Convenience type for asset: amount balances
pub type AccountBalances = HashMap<String, Decimal>;

/// Convenience type for asset: extended balances
pub type ExtendedBalances = HashMap<String, ExtendedBalance>;

/// Detailed balance data, including holds and credit (if available)
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct ExtendedBalance {
    pub balance: Decimal,
    pub hold_trade: Decimal,
    pub credit: Option<Decimal>,
    pub credit_used: Option<Decimal>,
}

/// Detailed margin balance data
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TradeBalances {
    #[serde(rename(deserialize = "eb"))]
    pub equivalent_balance: Decimal,
    #[serde(rename(deserialize = "tb"))]
    pub trade_balance: Decimal,
    #[serde(rename(deserialize = "m"))]
    pub margin: Decimal,
    #[serde(rename(deserialize = "n"))]
    pub net_pnl_open: Decimal,
    #[serde(rename(deserialize = "c"))]
    pub cost_basis_open: Decimal,
    #[serde(rename(deserialize = "v"))]
    pub floating_valuation: Decimal,
    #[serde(rename(deserialize = "e"))]
    pub equity: Decimal,
    #[serde(rename(deserialize = "mf"))]
    pub free_margin: Decimal,
    #[serde(rename(deserialize = "ml"))]
    pub margin_level: Option<Decimal>,
    #[serde(rename(deserialize = "uv"))]
    pub unexecuted_value: Option<Decimal>,
}

/// Details of individual order
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct OrderDescription {
    pub pair: String,
    #[serde(rename(deserialize = "type"))]
    pub side: BuySell,
    #[serde(rename(deserialize = "ordertype"))]
    pub order_type: OrderType,
    pub price: Decimal,
    pub price2: Decimal,
    pub leverage: String,
    pub order: String,
    pub close: String,
}

/// Wrapper to map open orders by Kraken ref-id
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct OpenOrders {
    pub open: HashMap<String, Order>,
}

/// Order object for OpenOrders and QueryOrders
#[serde_as]
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Order {
    #[serde(rename = "refid")]
    pub ref_id: Option<String>,
    pub userref: Option<i64>,
    #[serde(rename = "cl_ord_id")]
    pub client_order_id: Option<String>,
    pub status: OrderStatus,
    #[serde(rename = "opentm")]
    pub open_time: f64,
    #[serde(rename = "starttm")]
    pub start_time: f64,
    #[serde(rename = "expiretm")]
    pub expire_time: f64,
    #[serde(rename = "closetm")]
    pub close_time: Option<f64>,
    pub descr: OrderDescription,
    #[serde(rename(deserialize = "vol"))]
    pub volume: Decimal,
    #[serde(rename(deserialize = "vol_exec"))]
    pub volume_executed: Decimal,
    pub cost: Decimal,
    pub fee: Decimal,
    pub price: Decimal,
    #[serde(rename = "stopprice")]
    pub stop_price: Decimal,
    #[serde(rename = "limitprice")]
    pub limit_price: Decimal,
    pub trigger: Option<TriggerType>,
    pub margin: Option<bool>,
    pub misc: String,
    pub sender_sub_id: Option<String>,
    #[serde(rename = "oflags")]
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, OrderFlag>")]
    pub order_flags: Vec<OrderFlag>,
    pub trades: Option<Vec<String>>,
    pub reason: Option<String>,
}

/// Order object for closed orders
#[serde_as]
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ClosedOrder {
    #[serde(rename = "refid")]
    pub ref_id: Option<String>,
    pub userref: Option<i64>,
    #[serde(rename = "cl_ord_id")]
    pub client_order_id: Option<String>,
    pub status: OrderStatus,
    #[serde(rename = "opentm")]
    pub open_time: f64,
    #[serde(rename = "starttm")]
    pub start_time: f64,
    #[serde(rename = "expiretm")]
    pub expire_time: f64,
    #[serde(rename = "closetm")]
    pub close_time: Option<f64>,
    #[serde(rename(deserialize = "vol"))]
    pub volume: Decimal,
    #[serde(rename(deserialize = "vol_exec"))]
    pub volume_executed: Decimal,
    pub cost: Decimal,
    pub fee: Decimal,
    pub price: Decimal,
    #[serde(rename = "stopprice")]
    pub stop_price: Decimal,
    #[serde(rename = "limitprice")]
    pub limit_price: Decimal,
    pub trigger: Option<TriggerType>,
    pub margin: Option<bool>,
    pub misc: String,
    #[serde(rename = "oflags")]
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, OrderFlag>")]
    pub order_flags: Vec<OrderFlag>,
    pub trades: Option<Vec<String>>,
    pub sender_sub_id: Option<String>,
    pub reason: Option<String>,
}

/// Response type for mapping order ids to orders
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ClosedOrders {
    pub closed: HashMap<String, ClosedOrder>,
    pub count: i64,
}

/// A private trade
///
/// Includes fees paid, ledger entries, related order and position ids, etc.
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Trade {
    #[serde(rename = "ordertxid")]
    pub order_tx_id: String,
    #[serde(rename = "postxid")]
    pub post_xid: String,
    pub pair: String,
    pub time: f64,
    #[serde(rename(deserialize = "type"))]
    pub side: BuySell,
    #[serde(rename = "ordertype")]
    pub order_type: TradeType,
    pub price: Decimal,
    pub cost: Decimal,
    pub fee: Decimal,
    #[serde(rename(deserialize = "vol"))]
    pub volume: Decimal,
    pub margin: Decimal,
    pub misc: String,
    pub ledgers: Option<Vec<String>>,
    pub maker: bool,
}

/// Mapping of trade-id: trade object
pub type TradesInfo = HashMap<String, Trade>;

/// Response type for user's trade history
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TradesHistory {
    pub trades: TradesInfo,
    pub count: i64,
}

/// Mapping of position id: OpenPosition
pub type OpenPositions = HashMap<String, OpenPosition>;

/// Details of an open margin position
#[serde_as]
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct OpenPosition {
    #[serde(rename = "ordertxid")]
    pub order_tx_id: String,
    #[serde(rename = "posstatus")]
    pub pos_status: PositionStatus,
    pub pair: String,
    pub time: f64,
    #[serde(rename(deserialize = "type"))]
    pub side: BuySell,
    #[serde(rename = "ordertype")]
    pub order_type: OrderType,
    pub cost: Decimal,
    pub fee: Decimal,
    #[serde(rename(deserialize = "vol"))]
    pub volume: Decimal,
    #[serde(rename(deserialize = "vol_closed"))]
    pub volume_closed: Decimal,
    pub margin: Decimal,
    pub value: Option<Decimal>,
    pub net: Option<Decimal>,
    pub terms: String,
    #[serde(rename = "rollovertm")]
    pub rollover_time: String,
    pub misc: String,
    #[serde(rename = "oflags")]
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, OrderFlag>")]
    pub order_flags: Vec<OrderFlag>,
}

/// Entry in the user's ledger
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct LedgerEntry {
    #[serde(rename = "refid")]
    pub ref_id: String,
    pub time: f64,
    #[serde(rename(deserialize = "type"))]
    pub entry_type: LedgerEntryType,
    pub subtype: String,
    #[serde(rename = "aclass")]
    pub asset_class: String,
    pub asset: String,
    pub amount: Decimal,
    pub fee: Decimal,
    pub balance: Decimal,
}

/// Mapping of ledger id: ledger entry
pub type QueryLedgerInfo = HashMap<String, LedgerEntry>;

/// Response type for Ledgers and QueryLedgers
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct LedgerInfo {
    pub ledger: QueryLedgerInfo,
    pub count: i64,
}

/// Description of fee tier
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Fees {
    pub fee: Decimal,
    #[serde(rename = "minfee")]
    pub min_fee: Decimal,
    #[serde(rename = "maxfee")]
    pub max_fee: Decimal,
    #[serde(rename = "nextfee")]
    pub next_fee: Option<Decimal>,
    #[serde(rename = "nextvolume")]
    pub next_volume: Option<Decimal>,
    #[serde(rename = "tiervolume")]
    pub tier_volume: Option<Decimal>,
}

/// Response type for TradeVolume
///
/// In the case of maker-taker fees, `fees` maps trading pairs to taker fees. Otherwise, it
/// represents fees more broadly.
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TradeVolume {
    pub currency: String,
    pub volume: Decimal,
    pub fees: Option<HashMap<String, Fees>>,
    pub fees_maker: Option<HashMap<String, Fees>>,
}

/// Response type for ExportReport
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ExportReport {
    pub id: String,
}

/// Description of an export report
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ExportReportStatus {
    pub id: String,
    pub descr: String,
    pub format: String,
    pub report: String,
    pub subtype: String,
    pub status: ExportReportStatusType,
    pub fields: String,
    #[serde(rename = "createdtm")]
    pub created_time: String,
    #[serde(rename = "starttm")]
    pub start_time: String,
    #[serde(rename = "completedtm")]
    pub completed_time: String,
    #[serde(rename = "datastarttm")]
    pub data_start_time: String,
    #[serde(rename = "dataendtm")]
    pub data_end_time: String,
    pub asset: String,
}

/// Response type for deleting an export report
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct DeleteExportReport {
    pub delete: Option<bool>,
    pub cancel: Option<bool>,
}

/// English description of an added order and closing order instruction (if given)
///
/// Such as "buy 5.00000000 USDCUSD @ limit 1.0000"
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct AddOrderDescription {
    pub order: String,
    pub close: Option<String>,
}

/// Response type for AddOrder
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct AddOrder {
    #[serde(rename = "txid")]
    pub tx_id: Vec<String>,
    pub descr: AddOrderDescription,
    pub error: Option<String>,
}

/// Description of an added batch order, including potential error value.
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct BatchedOrder {
    #[serde(rename = "txid")]
    pub tx_id: String,
    pub descr: AddOrderDescription,
    pub error: Option<String>,
}

/// Response type for AddOrderBatch
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct AddOrderBatch {
    pub orders: Vec<BatchedOrder>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct AmendOrder {
    pub amend_id: String,
}

/// Response type for an edited order
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct OrderEdit {
    pub status: OrderEditStatus,
    #[serde(rename = "txid")]
    pub tx_id: String,
    #[serde(rename = "originaltxid")]
    pub original_tx_id: String,
    pub volume: Decimal,
    pub price: Decimal,
    pub price2: Option<Decimal>,
    pub orders_cancelled: i64,
    pub descr: AddOrderDescription,
}

/// Response for CancelOrder
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CancelOrder {
    pub count: i64,
    pub pending: Option<bool>,
}

/// Response for CancelAllOrdersAfter
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersAfter {
    pub current_time: String,
    pub trigger_time: String,
}

/// Description of a deposit method
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct DepositMethod {
    pub method: String,
    pub limit: BoolOrString,
    pub fee: Option<Decimal>,
    pub address_setup_fee: Option<Decimal>,
    pub gen_address: Option<bool>,
    pub minimum: Decimal,
}

/// Description of a withdrawal method
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct WithdrawMethod {
    pub asset: String,
    pub method: String,
    pub network: Option<String>,
    pub minimum: Decimal,
}

/// Description of a deposit address
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct DepositAddress {
    pub address: String,
    #[serde(rename = "expiretm")]
    pub expire_time: String,
    pub new: Option<bool>,
    pub memo: Option<String>,
    pub tag: Option<String>,
}

/// Description of a withdrawal method
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct WithdrawalAddress {
    pub address: String,
    pub asset: String,
    pub method: String,
    pub key: String,
    pub memo: Option<String>,
    pub verified: bool,
}

/// Response type for status of a deposit or withdrawal
///
/// Response can either be bare (Response) or be a wrapper containing a cursor for the next page (Cursor)
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum DepositWithdrawResponse {
    Cursor(DepositWithdrawalCursor),
    Response(Vec<DepositWithdrawal>),
}

/// Cursor response that wraps a deposit
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct DepositWithdrawalCursor {
    deposit: Vec<DepositWithdrawal>,
    cursor: BoolOrString,
}

/// Description of a deposit or withdrawal
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct DepositWithdrawal {
    pub method: String,
    #[serde(rename = "aclass")]
    pub asset_class: String,
    pub asset: String,
    #[serde(rename = "refid")]
    pub ref_id: String,
    #[serde(rename = "txid")]
    pub tx_id: String,
    pub info: String,
    pub amount: Decimal,
    pub fee: Decimal,
    pub time: i64,
    pub status: TransferStatus,
    #[serde(rename = "status-prop")]
    pub status_prop: Option<StatusProp>,
    pub orginators: Option<Vec<String>>,
}

/// Description of a withdrawal
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Withdrawal {
    pub method: String,
    pub limit: BoolOrString,
    pub fee: Decimal,
    pub amount: Decimal,
}

/// Response type containing only a ref id for confirmation
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ConfirmationRefId {
    #[serde(rename = "refid")]
    pub ref_id: String,
}

/// Response type for a transfer to a linked Futures account
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct AccountTransfer {
    pub transfer_id: String,
    pub status: AccountTransferStatus,
}

/// Response type for AllocateStatus
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct AllocationStatus {
    pub pending: bool,
}

/// Paginated response type for /Earn/Strategies
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct EarnStrategies {
    pub items: Vec<EarnStrategy>,
    pub next_cursor: Option<String>,
}

/// Description of an individual earn strategy
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct EarnStrategy {
    pub allocation_fee: EarnFee,
    pub allocation_restriction_info: Vec<String>,
    pub apr_estimate: Option<AprEstimate>,
    pub asset: String,
    pub auto_compound: AutoCompound,
    pub can_allocate: bool,
    pub can_deallocate: bool,
    pub deallocation_fee: EarnFee,
    pub id: String,
    pub lock_type: LockTypeDetail,
    pub user_cap: Option<Decimal>,
    pub user_min_allocation: Option<Decimal>,
    pub yield_source: YieldSource,
}

/// Details of how funds are locked by an earn strategy
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct LockTypeDetail {
    #[serde(rename = "type")]
    pub lock_type: LockType,
    #[serde(flatten)]
    pub bonding: Option<BondingDetail>,
}

/// Details of an earn strategy's commitments and rewards
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct BondingDetail {
    pub payout_frequency: Option<i64>,
    pub bonding_period: Option<i64>,
    pub bonding_period_variable: Option<bool>,
    pub bonding_rewards: Option<bool>,
    pub exit_queue_period: Option<i64>,
    pub unbonding_period: Option<i64>,
    pub unbonding_period_variable: Option<bool>,
    pub unbonding_rewards: Option<bool>,
}

/// Bracketed estimate for a strategy's APR
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct AprEstimate {
    pub low: Decimal,
    pub high: Decimal,
}

/// Wrapper type for compounding nature of a strategy
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct AutoCompound {
    #[serde(rename = "type")]
    pub auto_compound_type: AutoCompoundType,
    pub default: Option<bool>,
}

/// Wrapper type for the origin of rewards from a strategy
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct YieldSource {
    #[serde(rename = "type")]
    pub yield_type: YieldSourceType,
}

/// Response type for Earn/Allocations
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct EarnAllocations {
    pub converted_asset: String,
    pub items: Vec<EarnAllocation>,
    pub total_allocated: Decimal,
    pub total_rewarded: Decimal,
}

/// Description of an allocation to an earn strategy
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct EarnAllocation {
    pub amount_allocated: AmountAllocated,
    pub native_asset: String,
    pub payout: Option<Payout>,
    pub strategy_id: String,
    pub total_rewarded: EarnAmount,
}

/// Details of an allocation to a particular strategy
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct AmountAllocated {
    pub bonding: Option<AllocationState>,
    pub exit_queue: Option<AllocationState>,
    pub pending: Option<EarnAmount>,
    pub total: EarnAmount,
    pub unbonding: Option<AllocationState>,
}

/// State of a single allocation to a strategy
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct AllocationState {
    pub allocation_count: i64,
    pub allocations: Vec<Allocation>,
    pub converted: Decimal,
    pub native: Decimal,
}

/// Description of assets allocated to a strategy
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Allocation {
    pub created_at: String,
    pub expires: String,
    pub converted: Decimal,
    pub native: Decimal,
}

/// Description of the payout for a particular allocation
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Payout {
    pub period_end: String,
    pub period_start: String,
    pub accumulated_reward: EarnAmount,
    pub estimated_reward: EarnAmount,
}

/// Amount earned by an allocation in the requested and native assets
#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
pub struct EarnAmount {
    pub converted: Decimal,
    pub native: Decimal,
}

/// Response type for GetWebSocketsToken
#[derive(Debug, Deserialize, Clone)]
pub struct WebsocketToken {
    pub token: Token,
    pub expires: i64,
}

#[cfg(test)]
mod tests {
    use crate::response_types::ExtendedBalance;
    use rust_decimal_macros::dec;

    #[test]
    fn test_deserializing_extended_balance_full() {
        let balance =
            r#"{"balance": "0.01", "hold_trade": "0.02", "credit": "0.03", "credit_used": "0.04"}"#;

        let expected_balance = ExtendedBalance {
            balance: dec!(0.01),
            hold_trade: dec!(0.02),
            credit: Some(dec!(0.03)),
            credit_used: Some(dec!(0.04)),
        };

        assert_eq!(expected_balance, serde_json::from_str(balance).unwrap());
    }

    #[test]
    fn test_deserializing_extended_balance_some_none() {
        let balance_missing = r#"{"balance": "0.01", "hold_trade": "0.02"}"#;

        let expected_balance = ExtendedBalance {
            balance: dec!(0.01),
            hold_trade: dec!(0.02),
            credit: None,
            credit_used: None,
        };

        assert_eq!(
            expected_balance,
            serde_json::from_str(balance_missing).unwrap()
        );
    }

    #[test]
    fn test_deserializing_extended_balance_some_gibberish() {
        let gibberish = r#"{"balance": "0.01", "hold_trade": "0.02", "credit": "soNotANumber"}"#;

        assert!(serde_json::from_str::<ExtendedBalance>(gibberish).is_err())
    }
}
