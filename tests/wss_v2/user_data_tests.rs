use crate::wss_v2::shared::ParseIncomingTest;
use kraken_async_rs::request_types::{TimeInForce, TriggerType};
use kraken_async_rs::response_types::{BuySell, OrderStatusV2, OrderType};
use kraken_async_rs::wss::{
    Balance, BalanceResponse, ChannelMessage, ExecutionResult, ExecutionType, Fee, FeePreference,
    LedgerCategory, LedgerEntryTypeV2, LedgerUpdate, MakerTaker, PriceType, Response,
    TriggerDescription, TriggerStatus, Wallet, WalletId, WalletType, WssMessage,
};
use rust_decimal_macros::dec;
// TODO: need a realistic partial fill message

#[tokio::test]
async fn test_execution_trades_snapshot() {
    let trades_snapshot = r#"{
        "channel":"executions",
        "type":"snapshot",
        "data":[
            {"order_id":"NG6PUE-C7MXN-CFCAMC","order_userref":0,"exec_id":"B1Y0D9-6JIJG-W1IB7L","exec_type":"trade","trade_id":37496584,"symbol":"BTC/USD","side":"sell","last_qty":0.00016000,"last_price":63377.2,"liquidity_ind":"t","cost":10.12445,"order_status":"filled","order_type":"limit","sender_sub_id":"some-uuid","timestamp":"2024-04-16T10:54:38.243302Z","fee_usd_equiv":0.04050,"fees":[{"asset":"USD","qty":0.04051}]},
            {"order_id":"8G1X9R-F6HH0-R2FYZ0","order_userref":0,"exec_id":"0CVSSH-KVM0J-TCXLSQ","exec_type":"trade","trade_id":2125408,"symbol":"FET/USD","side":"buy","last_qty":25.00000000,"last_price":0.6017,"liquidity_ind":"m","margin":true,"margin_borrow":true,"liquidated":true,"cost":14.013500,"order_status":"filled","order_type":"limit","timestamp":"2024-01-28T21:03:18.167719Z","fee_usd_equiv":0.024028,"fees":[{"asset":"USD","qty":0.024038}]},
            {"order_id":"MQUCYY-SX33Q-KX7KCT","order_userref":0,"exec_id":"QEP2P0-DVAJN-VF1UTF","exec_type":"trade","trade_id":35272682,"symbol":"ETH/USD","side":"sell","last_qty":0.01500000,"last_price":2392.41,"liquidity_ind":"t","cost":35.37130,"order_status":"filled","order_type":"market","timestamp":"2024-01-13T12:24:42.541293Z","fee_usd_equiv":0.09327,"fees":[{"asset":"USD","qty":0.09337}]},
            {"order_id":"MMNB64-U9T0S-U8W0PJ","order_userref":0,"exec_id":"NG6PUE-C7MXN-CFCAMC","exec_type":"trade","trade_id":112396,"symbol":"BRICK/USD","side":"buy","last_qty":153.25931,"last_price":0.06404,"liquidity_ind":"m","cost":9.262299496,"order_status":"filled","order_type":"limit","timestamp":"2024-01-10T07:14:14.485774Z","fee_usd_equiv":0.015460799,"fees":[{"asset":"USD","qty":0.014460799}]}
        ],
        "sequence":1
    }"#.to_string();

    let expected_trades_snapshot = WssMessage::Channel(ChannelMessage::Execution(Response {
        data: vec![
            ExecutionResult {
                amended: None,
                execution_type: ExecutionType::Trade,
                cash_order_quantity: None,
                contingent: None,
                cost: Some(dec!(10.12445)),
                execution_id: Some("B1Y0D9-6JIJG-W1IB7L".to_string()),
                fees: Some(vec![Fee {
                    asset: "USD".to_string(),
                    quantity: dec!(0.04051),
                }]),
                liquidity_indicator: Some(MakerTaker::Taker),
                last_price: Some(dec!(63377.2)),
                last_quantity: Some(dec!(0.00016000)),
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
                fee_usd_equivalent: Some(dec!(0.04050)),
                limit_price: None,
                limit_price_type: None,
                liquidated: None,
                margin: None,
                margin_borrow: None,
                no_market_price_protection: None,
                order_ref_id: None,
                order_id: "NG6PUE-C7MXN-CFCAMC".to_string(),
                order_quantity: None,
                order_type: Some(OrderType::Limit),
                order_status: OrderStatusV2::Filled,
                order_user_ref: Some(0),
                post_only: None,
                position_status: None,
                reduce_only: None,
                sender_sub_id: Some("some-uuid".to_string()),
                side: Some(BuySell::Sell),
                symbol: Some("BTC/USD".to_string()),
                time_in_force: None,
                timestamp: "2024-04-16T10:54:38.243302Z".to_string(),
                trade_id: Some(37496584),
                triggers: None,
                client_order_id: None,
            },
            ExecutionResult {
                amended: None,
                execution_type: ExecutionType::Trade,
                cash_order_quantity: None,
                contingent: None,
                cost: Some(dec!(14.013500)),
                execution_id: Some("0CVSSH-KVM0J-TCXLSQ".to_string()),
                fees: Some(vec![Fee {
                    asset: "USD".to_string(),
                    quantity: dec!(0.024038),
                }]),
                liquidity_indicator: Some(MakerTaker::Maker),
                last_price: Some(dec!(0.6017)),
                last_quantity: Some(dec!(25.00000000)),
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
                fee_usd_equivalent: Some(dec!(0.024028)),
                limit_price: None,
                limit_price_type: None,
                liquidated: Some(true),
                margin: Some(true),
                margin_borrow: Some(true),
                no_market_price_protection: None,
                order_ref_id: None,
                order_id: "8G1X9R-F6HH0-R2FYZ0".to_string(),
                order_quantity: None,
                order_type: Some(OrderType::Limit),
                order_status: OrderStatusV2::Filled,
                order_user_ref: Some(0),
                post_only: None,
                position_status: None,
                reduce_only: None,
                sender_sub_id: None,
                side: Some(BuySell::Buy),
                symbol: Some("FET/USD".to_string()),
                time_in_force: None,
                timestamp: "2024-01-28T21:03:18.167719Z".to_string(),
                trade_id: Some(2125408),
                triggers: None,
                client_order_id: None,
            },
            ExecutionResult {
                amended: None,
                execution_type: ExecutionType::Trade,
                cash_order_quantity: None,
                contingent: None,
                cost: Some(dec!(35.37130)),
                execution_id: Some("QEP2P0-DVAJN-VF1UTF".to_string()),
                fees: Some(vec![Fee {
                    asset: "USD".to_string(),
                    quantity: dec!(0.09337),
                }]),
                liquidity_indicator: Some(MakerTaker::Taker),
                last_price: Some(dec!(2392.41)),
                last_quantity: Some(dec!(0.01500000)),
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
                fee_usd_equivalent: Some(dec!(0.09327)),
                limit_price: None,
                limit_price_type: None,
                liquidated: None,
                margin: None,
                margin_borrow: None,
                no_market_price_protection: None,
                order_ref_id: None,
                order_id: "MQUCYY-SX33Q-KX7KCT".to_string(),
                order_quantity: None,
                order_type: Some(OrderType::Market),
                order_status: OrderStatusV2::Filled,
                order_user_ref: Some(0),
                post_only: None,
                position_status: None,
                reduce_only: None,
                sender_sub_id: None,
                side: Some(BuySell::Sell),
                symbol: Some("ETH/USD".to_string()),
                time_in_force: None,
                timestamp: "2024-01-13T12:24:42.541293Z".to_string(),
                trade_id: Some(35272682),
                triggers: None,
                client_order_id: None,
            },
            ExecutionResult {
                amended: None,
                execution_type: ExecutionType::Trade,
                cash_order_quantity: None,
                contingent: None,
                cost: Some(dec!(9.262299496)),
                execution_id: Some("NG6PUE-C7MXN-CFCAMC".to_string()),
                fees: Some(vec![Fee {
                    asset: "USD".to_string(),
                    quantity: dec!(0.014460799),
                }]),
                liquidity_indicator: Some(MakerTaker::Maker),
                last_price: Some(dec!(0.06404)),
                last_quantity: Some(dec!(153.25931)),
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
                fee_usd_equivalent: Some(dec!(0.015460799)),
                limit_price: None,
                limit_price_type: None,
                liquidated: None,
                margin: None,
                margin_borrow: None,
                no_market_price_protection: None,
                order_ref_id: None,
                order_id: "MMNB64-U9T0S-U8W0PJ".to_string(),
                order_quantity: None,
                order_type: Some(OrderType::Limit),
                order_status: OrderStatusV2::Filled,
                order_user_ref: Some(0),
                post_only: None,
                position_status: None,
                reduce_only: None,
                sender_sub_id: None,
                side: Some(BuySell::Buy),
                symbol: Some("BRICK/USD".to_string()),
                time_in_force: None,
                timestamp: "2024-01-10T07:14:14.485774Z".to_string(),
                trade_id: Some(112396),
                triggers: None,
                client_order_id: None,
            },
        ],
        sequence: 1,
    }));

    ParseIncomingTest::new()
        .with_incoming(trades_snapshot)
        .expect_message(expected_trades_snapshot)
        .test()
        .await;
}

#[tokio::test]
async fn test_execution_order_update_cancelled() {
    let cancel = r#"{"channel":"executions","type":"update","data":[{"timestamp":"2024-05-18T12:58:40.165132Z",
    "order_status":"canceled","exec_type":"canceled","cum_qty":0.00000000,"cum_cost":0.000000,"fee_usd_equiv":0.000000,
    "avg_price":0.00000,"order_userref":0,"cancel_reason":"User requested","reason":"User requested",
    "order_id":"KIUEL4-G3PWU-HOJTYU"}],"sequence":143}"#.to_string();

    let expected_update_cancel = WssMessage::Channel(ChannelMessage::Execution(Response {
        data: vec![ExecutionResult {
            amended: None,
            execution_type: ExecutionType::Canceled,
            cash_order_quantity: None,
            contingent: None,
            cost: None,
            execution_id: None,
            fees: None,
            liquidity_indicator: None,
            last_price: None,
            last_quantity: None,
            average_price: Some(dec!(0.0)),
            reason: Some("User requested".to_string()),
            cumulative_cost: Some(dec!(0.0)),
            cumulative_quantity: Some(dec!(0.0)),
            display_quantity: None,
            effective_time: None,
            expire_time: None,
            ext_ord_id: None,
            ext_exec_id: None,
            fee_preference: None,
            fee_usd_equivalent: Some(dec!(0.0)),
            limit_price: None,
            limit_price_type: None,
            liquidated: None,
            margin: None,
            margin_borrow: None,
            no_market_price_protection: None,
            order_ref_id: None,
            order_id: "KIUEL4-G3PWU-HOJTYU".to_string(),
            order_quantity: None,
            order_type: None,
            order_status: OrderStatusV2::Canceled,
            order_user_ref: Some(0),
            post_only: None,
            position_status: None,
            reduce_only: None,
            sender_sub_id: None,
            side: None,
            symbol: None,
            time_in_force: None,
            timestamp: "2024-05-18T12:58:40.165132Z".to_string(),
            trade_id: None,
            triggers: None,
            client_order_id: None,
        }],
        sequence: 143,
    }));

    ParseIncomingTest::new()
        .with_incoming(cancel)
        .expect_message(expected_update_cancel)
        .test()
        .await;
}

#[tokio::test]
async fn test_execution_limit_order_update_pending() {
    let pending_new = r#"{"channel":"executions","type":"update","data":[{"order_id":"AHOJQ8-1E72C-8M2VQH","symbol":"ADX/USD",
    "order_qty":81.36256082,"cum_cost":0.0000000,"time_in_force":"GTC","exec_type":"pending_new","side":"buy","order_type":"limit",
    "order_userref":0,"limit_price_type":"static","limit_price":0.18328,"stop_price":0.00000,"order_status":"pending_new",
    "fee_usd_equiv":0.0000000,"fee_ccy_pref":"fciq","timestamp":"2024-05-18T12:01:56.165888Z"}],"sequence":120}"#.to_string();

    let expected_update_pending = WssMessage::Channel(ChannelMessage::Execution(Response {
        data: vec![ExecutionResult {
            amended: None,
            execution_type: ExecutionType::PendingNew,
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
            cumulative_cost: Some(dec!(0.0)),
            cumulative_quantity: None,
            display_quantity: None,
            effective_time: None,
            expire_time: None,
            ext_ord_id: None,
            ext_exec_id: None,
            fee_preference: Some(FeePreference::Quote),
            fee_usd_equivalent: Some(dec!(0.0)),
            limit_price: Some(dec!(0.18328)),
            limit_price_type: Some(PriceType::Static),
            liquidated: None,
            margin: None,
            margin_borrow: None,
            no_market_price_protection: None,
            order_ref_id: None,
            order_id: "AHOJQ8-1E72C-8M2VQH".to_string(),
            order_quantity: Some(dec!(81.36256082)),
            order_type: Some(OrderType::Limit),
            order_status: OrderStatusV2::PendingNew,
            order_user_ref: Some(0),
            post_only: None,
            position_status: None,
            reduce_only: None,
            sender_sub_id: None,
            side: Some(BuySell::Buy),
            symbol: Some("ADX/USD".to_string()),
            time_in_force: Some(TimeInForce::GTC),
            timestamp: "2024-05-18T12:01:56.165888Z".to_string(),
            trade_id: None,
            triggers: None,
            client_order_id: None,
        }],
        sequence: 120,
    }));

    ParseIncomingTest::new()
        .with_incoming(pending_new)
        .expect_message(expected_update_pending)
        .test()
        .await;
}

#[tokio::test]
async fn test_execution_stop_loss_limit_order_update_pending() {
    let pending_new = r#"{"channel":"executions","type":"update","data":[{"order_id":"AHOJQ8-1E72C-8M2VQH","symbol":"ADX/USD",
    "order_qty":81.36256082,"cum_cost":0,"time_in_force":"GTC","exec_type":"pending_new","side":"buy","order_type":"stop-loss-limit",
    "order_userref":0,"limit_price_type":"static","triggers":{"price":0.2,"price_type":"static","reference":"index","status":"untriggered"},
    "stop_price":0.2,"limit_price":0.2,"trigger":"index","order_status":"pending_new","fee_usd_equiv":0,"fee_ccy_pref":"fciq",
    "timestamp":"2024-05-18T12:01:56.165888Z"}],"sequence":120}"#.to_string();

    let expected_update_pending = WssMessage::Channel(ChannelMessage::Execution(Response {
        data: vec![ExecutionResult {
            amended: None,
            execution_type: ExecutionType::PendingNew,
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
            cumulative_cost: Some(dec!(0.0)),
            cumulative_quantity: None,
            display_quantity: None,
            effective_time: None,
            expire_time: None,
            ext_ord_id: None,
            ext_exec_id: None,
            fee_preference: Some(FeePreference::Quote),
            fee_usd_equivalent: Some(dec!(0.0)),
            limit_price: Some(dec!(0.2)),
            limit_price_type: Some(PriceType::Static),
            liquidated: None,
            margin: None,
            margin_borrow: None,
            no_market_price_protection: None,
            order_ref_id: None,
            order_id: "AHOJQ8-1E72C-8M2VQH".to_string(),
            order_quantity: Some(dec!(81.36256082)),
            order_type: Some(OrderType::StopLossLimit),
            order_status: OrderStatusV2::PendingNew,
            order_user_ref: Some(0),
            post_only: None,
            position_status: None,
            reduce_only: None,
            sender_sub_id: None,
            side: Some(BuySell::Buy),
            symbol: Some("ADX/USD".to_string()),
            time_in_force: Some(TimeInForce::GTC),
            timestamp: "2024-05-18T12:01:56.165888Z".to_string(),
            trade_id: None,
            triggers: Some(TriggerDescription {
                reference: TriggerType::Index,
                price: dec!(0.2),
                price_type: PriceType::Static,
                actual_price: None,
                peak_price: None,
                last_price: None,
                status: TriggerStatus::Untriggered,
                timestamp: None,
            }),
            client_order_id: None,
        }],
        sequence: 120,
    }));

    ParseIncomingTest::new()
        .with_incoming(pending_new)
        .expect_message(expected_update_pending)
        .test()
        .await;
}

#[tokio::test]
async fn test_execution_order_update_new() {
    let new = r#"{"channel":"executions","type":"update","data":[{"timestamp":"2024-05-18T12:58:51.121515Z",
    "order_status":"new","exec_type":"new","order_userref":0,"order_id":"7J91XK-XMFEL-348VPM"}],"sequence":148}"#.to_string();

    let expected_update_new = WssMessage::Channel(ChannelMessage::Execution(Response {
        data: vec![ExecutionResult {
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
            order_id: "7J91XK-XMFEL-348VPM".to_string(),
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
            timestamp: "2024-05-18T12:58:51.121515Z".to_string(),
            trade_id: None,
            triggers: None,
            client_order_id: None,
        }],
        sequence: 148,
    }));

    ParseIncomingTest::new()
        .with_incoming(new)
        .expect_message(expected_update_new)
        .test()
        .await;
}

#[tokio::test]
async fn test_execution_order_amended() {
    let amend = r#"{
        "channel":"executions",
        "type":"update",
        "data":[
            {
                "timestamp":"2024-10-13T13:38:39.273886Z",
                "exec_type":"amended",
                "order_status":"new",
                "cum_qty":0.00000000,
                "reason":"User requested",
                "amended":true,
                "order_qty":5.10000000,
                "limit_price":0.9600,
                "limit_price_type":"static",
                "order_userref":0,
                "order_id":"6LYQGW-FH922-U6JTUM"
            }
        ],
        "sequence":20
    }"#;

    let expected_execution = WssMessage::Channel(ChannelMessage::Execution(Response {
        data: vec![ExecutionResult {
            amended: Some(true),
            execution_type: ExecutionType::Amended,
            cash_order_quantity: None,
            client_order_id: None,
            contingent: None,
            cost: None,
            execution_id: None,
            fees: None,
            liquidity_indicator: None,
            last_price: None,
            last_quantity: None,
            average_price: None,
            reason: Some("User requested".to_string()),
            cumulative_cost: None,
            cumulative_quantity: Some(dec!(0)),
            display_quantity: None,
            effective_time: None,
            expire_time: None,
            ext_ord_id: None,
            ext_exec_id: None,
            fee_preference: None,
            fee_usd_equivalent: None,
            limit_price: Some(dec!(0.9600)),
            limit_price_type: Some(PriceType::Static),
            liquidated: None,
            margin: None,
            margin_borrow: None,
            no_market_price_protection: None,
            order_ref_id: None,
            order_id: "6LYQGW-FH922-U6JTUM".to_string(),
            order_quantity: Some(dec!(5.10000000)),
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
            timestamp: "2024-10-13T13:38:39.273886Z".to_string(),
            trade_id: None,
            triggers: None,
        }],
        sequence: 20,
    }));

    ParseIncomingTest::new()
        .with_incoming(amend.to_string())
        .expect_message(expected_execution)
        .test()
        .await;
}

#[tokio::test]
async fn test_balances_snapshot() {
    let balances_snapshot = r#"{
        "channel":"balances",
        "type":"snapshot",
        "data":[
            {"asset":"BRICK","asset_class":"currency","balance":439.9736, "wallets": []},
            {"asset":"KAR","asset_class":"currency","balance":774.6366982600, "wallets": []},
            {"asset":"KEEP","asset_class":"currency","balance":622.3962481300, "wallets": []},
            {"asset":"MULTI","asset_class":"currency","balance":5.5971035500, "wallets": []},
            {"asset":"USD","asset_class":"currency","balance":160.2405, "wallets": [{
                "type": "spot",
                "id": "main",
                "balance": 1.34
            }]}
        ],
        "sequence":1
    }
    "#
    .to_string();

    let expected_snapshot = WssMessage::Channel(ChannelMessage::Balance(Response {
        data: BalanceResponse::Snapshot(vec![
            Balance {
                asset: "BRICK".to_string(),
                balance: dec!(439.9736),
                wallets: vec![],
            },
            Balance {
                asset: "KAR".to_string(),
                balance: dec!(774.6366982600),
                wallets: vec![],
            },
            Balance {
                asset: "KEEP".to_string(),
                balance: dec!(622.3962481300),
                wallets: vec![],
            },
            Balance {
                asset: "MULTI".to_string(),
                balance: dec!(5.5971035500),
                wallets: vec![],
            },
            Balance {
                asset: "USD".to_string(),
                balance: dec!(160.2405),
                wallets: vec![Wallet {
                    balance: dec!(1.34),
                    wallet_type: WalletType::Spot,
                    id: WalletId::Main,
                }],
            },
        ]),
        sequence: 1,
    }));

    ParseIncomingTest::new()
        .with_incoming(balances_snapshot)
        .expect_message(expected_snapshot)
        .test()
        .await;
}

#[tokio::test]
async fn test_balances_updates() {
    let usd_update = r#"{
        "channel":"balances",
        "type":"update",
        "data":[{
            "ledger_id":"DATKX6-PEHL1-HZKND8",
            "ref_id":"LKAKN2-N0N12-VKQNLN",
            "timestamp":"2024-05-24T14:01:53.526524Z",
            "type":"trade",
            "asset":"USD",
            "asset_class":"currency",
            "category":"trade",
            "wallet_type":"spot",
            "wallet_id":"main",
            "amount":-19.9743,
            "fee":0.0499,
            "balance":118.0677
        }],
        "sequence":4
    }"#
    .to_string();

    let expected_usd_update = WssMessage::Channel(ChannelMessage::Balance(Response {
        data: BalanceResponse::Update(vec![LedgerUpdate {
            asset: "USD".to_string(),
            amount: dec!(-19.9743),
            balance: dec!(118.0677),
            fee: dec!(0.0499),
            ledger_id: "DATKX6-PEHL1-HZKND8".to_string(),
            ref_id: "LKAKN2-N0N12-VKQNLN".to_string(),
            timestamp: "2024-05-24T14:01:53.526524Z".to_string(),
            asset_class: "currency".to_string(),
            ledger_type: LedgerEntryTypeV2::Trade,
            sub_type: None,
            category: LedgerCategory::Trade,
            wallet_type: WalletType::Spot,
            wallet_id: WalletId::Main,
        }]),
        sequence: 4,
    }));

    let base_update = r#"{
        "channel":"balances",
        "type":"update",
        "data":[{
            "ledger_id":"9K6IR4-X9PQJ-OMBG73",
            "ref_id":"WLINKJ-1TZZW-M3HCOY",
            "timestamp":"2024-05-12T12:11:57.525134Z",
            "type":"trade",
            "asset":"ADX",
            "asset_class":"currency",
            "category":"trade",
            "wallet_type":"spot",
            "wallet_id":"main",
            "amount":111.0857412800,
            "fee":0.0000000000,
            "balance":147.1906006900
        }],
        "sequence":5
    }"#
    .to_string();

    let expected_base_update = WssMessage::Channel(ChannelMessage::Balance(Response {
        data: BalanceResponse::Update(vec![LedgerUpdate {
            asset: "ADX".to_string(),
            amount: dec!(111.0857412800),
            balance: dec!(147.1906006900),
            fee: dec!(0.0),
            ledger_id: "9K6IR4-X9PQJ-OMBG73".to_string(),
            ref_id: "WLINKJ-1TZZW-M3HCOY".to_string(),
            timestamp: "2024-05-12T12:11:57.525134Z".to_string(),
            asset_class: "currency".to_string(),
            ledger_type: LedgerEntryTypeV2::Trade,
            sub_type: None,
            category: LedgerCategory::Trade,
            wallet_type: WalletType::Spot,
            wallet_id: WalletId::Main,
        }]),
        sequence: 5,
    }));

    ParseIncomingTest::new()
        .with_incoming(usd_update)
        .expect_message(expected_usd_update)
        .with_incoming(base_update)
        .expect_message(expected_base_update)
        .test()
        .await;
}
