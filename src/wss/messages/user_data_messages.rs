use crate::crypto::secrets::Token;
use crate::request_types::{TimeInForce, TriggerType};
use crate::response_types::{BuySell, OrderStatusV2, OrderType, PositionStatusV2};
use crate::wss::{
    BookSubscriptionResponse, OhlcSubscriptionResponse, TickerSubscriptionResponse,
    TradeSubscriptionResponse,
};
use crate::wss::{ConditionalParams, FeePreference, PriceType};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionResponseType {
    Snapshot,
    Update,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MakerTaker {
    #[serde(rename = "m")]
    Maker,
    #[serde(rename = "t")]
    Taker,
}

/// Type of ledger entry in user's ledger
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "snake_case")]
pub enum LedgerEntryTypeV2 {
    Trade,
    Credit,
    Deposit,
    Withdrawal,
    Transfer,
    Margin,
    Rollover,
    Settled,
    Adjustment,
    Staking,
    Sale,
    Reserve,
    Conversion,
    Dividend,
    Reward,
    CreatorFee,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "lowercase")]
pub enum LedgerEntrySubType {
    SpotFromFutures,
    SpotToFutures,
    StakingFromSpot,
    SpotFromStaking,
    StakingToSpot,
    SpotToStaking,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum LedgerCategory {
    Deposit,
    Withdrawal,
    Trade,
    MarginTrade,
    MarginSettled,
    MarginConversion,
    Conversion,
    Credit,
    MarginRollover,
    StakingRewards,
    Instant,
    EquityTrade,
    Airdrop,
    EquityDividend,
    RewardBonus,
    Nft,
    BlockTrade,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WalletType {
    Spot,
    Earn,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WalletId {
    Main,
    Flex,
    Bonded,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionType {
    PendingNew,
    New,
    Trade,
    Filled,
    Canceled,
    Expired,
    Amended,
    Restated,
    Status,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TriggerStatus {
    Triggered,
    Untriggered,
}

#[derive(Debug, Serialize, Clone)]
pub struct SubscriptionRequest<T> {
    pub method: String,
    pub params: T,
    pub req_id: Option<i64>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Clone)]
pub struct ExecutionSubscription {
    pub channel: String,
    pub token: Token,
    #[serde(rename = "snap_trades")]
    pub snapshot_trades: Option<bool>,
    #[serde(rename = "snap_orders")]
    pub snapshot_orders: Option<bool>,
    pub rate_counter: Option<bool>,
}

impl ExecutionSubscription {
    pub fn new(token: Token) -> Self {
        ExecutionSubscription {
            channel: "executions".to_string(),
            token,
            snapshot_trades: None,
            snapshot_orders: None,
            rate_counter: None,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InstrumentSubscriptionResult {
    pub snapshot: Option<bool>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ExecutionsSubscriptionResult {
    #[serde(rename = "maxratecount")]
    pub max_rate_count: Option<i64>,
    pub snapshot: Option<bool>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct BalanceSubscriptionResult {
    pub snapshot: Option<bool>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "channel")]
pub enum SubscriptionResult {
    #[serde(rename = "level3")]
    L3(BookSubscriptionResponse),
    #[serde(rename = "book")]
    Book(BookSubscriptionResponse),
    #[serde(rename = "ticker")]
    Ticker(TickerSubscriptionResponse),
    #[serde(rename = "ohlc")]
    Ohlc(OhlcSubscriptionResponse),
    #[serde(rename = "trade")]
    Trade(TradeSubscriptionResponse),
    #[serde(rename = "executions")]
    Execution(ExecutionsSubscriptionResult),
    #[serde(rename = "balances")]
    Balance(BalanceSubscriptionResult),
    #[serde(rename = "instrument")]
    Instrument(InstrumentSubscriptionResult),
}

#[derive(Debug, Deserialize)]
pub struct ExecutionResponse {
    pub channel: String,
    #[serde(rename = "type")]
    pub execution_response_type: ExecutionResponseType,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Fee {
    pub asset: String,
    #[serde(rename = "qty")]
    pub quantity: Decimal,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TriggerDescription {
    pub reference: TriggerType,
    pub price: Decimal,
    pub price_type: PriceType,
    pub actual_price: Option<Decimal>,
    pub peak_price: Option<Decimal>,
    pub last_price: Option<Decimal>,
    pub status: TriggerStatus,
    pub timestamp: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ExecutionResult {
    pub amended: Option<bool>,
    #[serde(rename = "exec_type")]
    pub execution_type: ExecutionType,
    #[serde(rename = "cash_order_qty")]
    pub cash_order_quantity: Option<Decimal>,
    #[serde(rename = "cl_ord_id")]
    pub client_order_id: Option<String>,
    pub contingent: Option<ConditionalParams>,
    pub cost: Option<Decimal>,
    #[serde(rename = "exec_id")]
    pub execution_id: Option<String>,
    pub fees: Option<Vec<Fee>>,
    #[serde(rename = "liquidity_ind")]
    pub liquidity_indicator: Option<MakerTaker>,
    pub last_price: Option<Decimal>,
    #[serde(rename = "last_qty")]
    pub last_quantity: Option<Decimal>,
    #[serde(rename = "avg_price")]
    pub average_price: Option<Decimal>,
    pub reason: Option<String>,
    #[serde(rename = "cum_cost")]
    pub cumulative_cost: Option<Decimal>,
    #[serde(rename = "cum_qty")]
    pub cumulative_quantity: Option<Decimal>,
    #[serde(rename = "display_qty")]
    pub display_quantity: Option<Decimal>,
    pub effective_time: Option<String>,
    pub expire_time: Option<String>,
    pub ext_ord_id: Option<String>,
    pub ext_exec_id: Option<String>,
    #[serde(rename = "fee_ccy_pref")]
    pub fee_preference: Option<FeePreference>,
    #[serde(rename = "fee_usd_equiv")]
    pub fee_usd_equivalent: Option<Decimal>,
    pub limit_price: Option<Decimal>,
    pub limit_price_type: Option<PriceType>,
    pub liquidated: Option<bool>,
    pub margin: Option<bool>,
    pub margin_borrow: Option<bool>,
    #[serde(rename = "no_mpp")]
    pub no_market_price_protection: Option<bool>,
    #[serde(rename = "ord_ref_id")]
    pub order_ref_id: Option<i64>,
    pub order_id: String,
    #[serde(rename = "order_qty")]
    pub order_quantity: Option<Decimal>,
    pub order_type: Option<OrderType>,
    pub order_status: OrderStatusV2,
    #[serde(rename = "order_userref")]
    pub order_user_ref: Option<i64>,
    pub post_only: Option<bool>,
    pub position_status: Option<PositionStatusV2>,
    pub reduce_only: Option<bool>,
    pub sender_sub_id: Option<String>,
    pub side: Option<BuySell>,
    pub symbol: Option<String>,
    pub time_in_force: Option<TimeInForce>,
    pub timestamp: String,
    pub trade_id: Option<i64>,
    pub triggers: Option<TriggerDescription>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Clone)]
pub struct BalancesSubscription {
    pub channel: String,
    pub token: Token,
    pub snapshot: Option<bool>,
}

impl BalancesSubscription {
    pub fn new(token: Token) -> Self {
        BalancesSubscription {
            channel: "balances".to_string(),
            token,
            snapshot: None,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Wallet {
    pub balance: Decimal,
    #[serde(rename = "type")]
    pub wallet_type: WalletType,
    pub id: WalletId,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum BalanceResponse {
    Update(Vec<LedgerUpdate>),
    Snapshot(Vec<Balance>),
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Balance {
    pub asset: String,
    pub balance: Decimal,
    pub wallets: Vec<Wallet>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct LedgerUpdate {
    pub asset: String,
    pub amount: Decimal,
    pub balance: Decimal,
    pub fee: Decimal,
    pub ledger_id: String,
    pub ref_id: String,
    pub timestamp: String,
    pub asset_class: String,
    #[serde(rename = "type")]
    pub ledger_type: LedgerEntryTypeV2,
    pub sub_type: Option<LedgerEntrySubType>,
    pub category: LedgerCategory,
    pub wallet_type: WalletType,
    pub wallet_id: WalletId,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_deserializing_execution_trade() {
        let message = r#"{"order_id":"O7IBL5-O2V6X-EEXY4U","exec_id":"TJE7HC-DKBTI-5BFVKE","exec_type":"trade","ext_ord_id":"some-uuid","ext_exec_id":"another-uuid","trade_id":365573,"symbol":"KAR/USD","side":"buy","last_qty":105.02014889,"last_price":0.121,"liquidity_ind":"t","cost":12.70744,"order_status":"filled","order_type":"limit","timestamp":"2024-05-18T05:41:33.480251Z","fee_usd_equiv":0.05083,"fees":[{"asset":"USD","qty":0.05083}]}"#;
        let expected = ExecutionResult {
            amended: None,
            execution_type: ExecutionType::Trade,
            cash_order_quantity: None,
            contingent: None,
            cost: Some(dec!(12.70744)),
            execution_id: Some("TJE7HC-DKBTI-5BFVKE".to_string()),
            fees: Some(vec![Fee {
                asset: "USD".to_string(),
                quantity: dec!(0.05083),
            }]),
            liquidity_indicator: Some(MakerTaker::Taker),
            last_price: Some(dec!(0.121)),
            last_quantity: Some(dec!(105.02014889)),
            average_price: None,
            reason: None,
            cumulative_cost: None,
            cumulative_quantity: None,
            display_quantity: None,
            effective_time: None,
            expire_time: None,
            ext_ord_id: Some("some-uuid".to_string()),
            ext_exec_id: Some("another-uuid".to_string()),
            fee_preference: None,
            fee_usd_equivalent: Some(dec!(0.05083)),
            limit_price: None,
            limit_price_type: None,
            liquidated: None,
            margin: None,
            margin_borrow: None,
            no_market_price_protection: None,
            order_ref_id: None,
            order_id: "O7IBL5-O2V6X-EEXY4U".to_string(),
            order_quantity: None,
            order_type: Some(OrderType::Limit),
            order_status: OrderStatusV2::Filled,
            order_user_ref: None,
            post_only: None,
            position_status: None,
            reduce_only: None,
            sender_sub_id: None,
            side: Some(BuySell::Buy),
            symbol: Some("KAR/USD".to_string()),
            time_in_force: None,
            timestamp: "2024-05-18T05:41:33.480251Z".to_string(),
            trade_id: Some(365573),
            triggers: None,
            client_order_id: None,
        };
        let parsed: ExecutionResult = serde_json::from_str(message).unwrap();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn test_deserializing_execution_new_update() {
        let message = r#"{"timestamp":"2024-05-18T11:00:37.240691Z","order_status":"new","exec_type":"new","order_userref":0,"order_id":"OLADEP-E5D5S-IKEHMF"}"#;
        let expected = ExecutionResult {
            amended: None,
            execution_type: ExecutionType::New,
            cash_order_quantity: None,
            contingent: None,
            cost: None,
            execution_id: None,
            fees: None,
            liquidity_indicator: None,
            last_price: None,
            last_quantity: None,
            average_price: None,
            reason: None,
            cumulative_cost: None,
            cumulative_quantity: None,
            display_quantity: None,
            effective_time: None,
            expire_time: None,
            ext_ord_id: None,
            ext_exec_id: None,
            fee_preference: None,
            fee_usd_equivalent: None,
            limit_price: None,
            limit_price_type: None,
            liquidated: None,
            margin: None,
            margin_borrow: None,
            no_market_price_protection: None,
            order_ref_id: None,
            order_id: "OLADEP-E5D5S-IKEHMF".to_string(),
            order_quantity: None,
            order_type: None,
            order_status: OrderStatusV2::New,
            order_user_ref: Some(0),
            post_only: None,
            position_status: None,
            reduce_only: None,
            sender_sub_id: None,
            side: None,
            symbol: None,
            time_in_force: None,
            timestamp: "2024-05-18T11:00:37.240691Z".to_string(),
            trade_id: None,
            triggers: None,
            client_order_id: None,
        };
        let parsed: ExecutionResult = serde_json::from_str(message).unwrap();

        assert_eq!(expected, parsed);
    }
}
