use serde_json::{json, Value};

pub fn get_allocate_earn_funds_json() -> Value {
    json!({
        "error": [],
        "result": true
    })
}

pub fn get_deallocate_earn_funds_json() -> Value {
    json!({
        "error": [],
        "result": true
    })
}

pub fn get_allocation_status_json() -> Value {
    json!({
        "error": [],
        "result": {"pending": false}
    })
}

pub fn get_deallocation_status_json() -> Value {
    json!({
        "error": [],
        "result": {"pending": false}
    })
}

pub fn get_list_earn_strategies_json() -> Value {
    json!({
        "error":
        [],
        "result": {
            "next_cursor":null,
            "items":[
                {"id": "ESQEFMZ-JPPIY-3KXFJ2", "asset": "USD", "lock_type": {"type": "instant", "payout_frequency":302400}, "apr_estimate": {"low": "5.2500", "high": "5.2500"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "opt_in_rewards"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ESSLTY3-TM6GC-G5BWW5", "asset": "EUR", "lock_type": {"type": "instant", "payout_frequency":302400}, "apr_estimate": {"low": "2.5000", "high": "2.5000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "opt_in_rewards"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ES2QQD3-N4GX4-DPB622", "asset": "XBT", "lock_type": {"type": "instant", "payout_frequency":302400}, "apr_estimate": {"low": "0.1500", "high": "0.1500"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "opt_in_rewards"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ESWS7VM-7CJ27-MMO4YN", "asset": "USDT", "lock_type": {"type": "instant", "payout_frequency":302400}, "apr_estimate": {"low": "4.2500", "high": "4.2500"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "opt_in_rewards"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ESEGNA5-RR6AJ-ODBGFW", "asset": "USDC", "lock_type": {"type": "instant", "payout_frequency":302400}, "apr_estimate": {"low": "4.2500", "high": "4.2500"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "opt_in_rewards"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ESNHEMI-SNZ7D-PNH423", "asset": "ADA", "lock_type": {"type": "instant", "payout_frequency":604800}, "apr_estimate": {"low": "3.0000", "high": "6.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ESRYVNB-YPE7I-4J3GQB", "asset": "XTZ", "lock_type": {"type": "instant", "payout_frequency":604800}, "apr_estimate": {"low": "4.0000", "high": "7.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ES3OO4J-FF3XJ-4HFOZK", "asset": "ATOM", "lock_type": {"type": "instant", "payout_frequency":604800}, "apr_estimate": {"low": "4.0000", "high": "8.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ESN4CUK-L4N2K-ZE4S27", "asset": "ATOM", "lock_type": {"type": "bonded", "payout_frequency":604800, "bonding_period":0, "bonding_period_variable":false, "bonding_rewards":false, "unbonding_period":1814400, "unbonding_period_variable":false, "unbonding_rewards":false, "exit_queue_period":0}, "apr_estimate": {"low": "11.0000", "high": "15.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ES6HXY7-6CH3M-BEBALT", "asset": "ALGO", "lock_type": {"type": "instant", "payout_frequency":604800}, "apr_estimate": {"low": "1.0000", "high": "4.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ESVY3DF-E5AIJ-TP6EBD", "asset": "TRX", "lock_type": {"type": "instant", "payout_frequency":604800}, "apr_estimate": {"low": "1.0000", "high": "4.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ESRFUO3-Q62XD-WIOIL7", "asset": "DOT", "lock_type": {"type": "instant", "payout_frequency":604800}, "apr_estimate": {"low": "7.0000", "high": "11.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ESMWVX6-JAPVY-23L3CV", "asset": "DOT", "lock_type": {"type": "bonded", "payout_frequency":604800, "bonding_period":0, "bonding_period_variable":false, "bonding_rewards":false, "unbonding_period":2419200, "unbonding_period_variable":false, "unbonding_rewards":false, "exit_queue_period":0}, "apr_estimate": {"low": "15.0000", "high": "21.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ESZCVV6-CKS62-GJ2YOP", "asset": "KAVA", "lock_type": {"type": "instant", "payout_frequency":604800}, "apr_estimate": {"low": "7.0000", "high": "9.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ES3TCHU-OUPTM-CEQUZB", "asset": "KAVA", "lock_type": {"type": "bonded", "payout_frequency":604800, "bonding_period":0, "bonding_period_variable":false, "bonding_rewards":false, "unbonding_period":1814400, "unbonding_period_variable":false, "unbonding_rewards":false, "exit_queue_period":0}, "apr_estimate": {"low": "13.0000", "high": "18.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ESMPVEI-HNNXJ-DF326M", "asset": "KSM", "lock_type": {"type": "instant", "payout_frequency":604800}, "apr_estimate": {"low": "5.0000", "high": "9.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ESXUM7H-SJHQ6-KOQNNI", "asset": "KSM", "lock_type": {"type": "bonded", "payout_frequency":604800, "bonding_period":0, "bonding_period_variable":false, "bonding_rewards":false, "unbonding_period":604800, "unbonding_period_variable":false, "unbonding_rewards":false, "exit_queue_period":0}, "apr_estimate": {"low": "13.0000", "high": "18.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ESK63SG-H5EJY-BEFW7Y", "asset": "FLOW", "lock_type": {"type": "instant", "payout_frequency":604800}, "apr_estimate": {"low": "1.0000", "high": "4.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ES3TW2S-6OVDU-U2LN4W", "asset": "FLOW", "lock_type": {"type": "bonded", "payout_frequency":604800, "bonding_period":0, "bonding_period_variable":false, "bonding_rewards":false, "unbonding_period":1209600, "unbonding_period_variable":false, "unbonding_rewards":false, "exit_queue_period":0}, "apr_estimate": {"low": "6.0000", "high": "10.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ESXWPHB-6DMRS-ALYR3T", "asset": "GRT", "lock_type": {"type": "instant", "payout_frequency":604800}, "apr_estimate": {"low": "2.0000", "high": "4.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ESIG7QR-O4FFV-PRGQVJ", "asset": "GRT", "lock_type": {"type": "bonded", "payout_frequency":604800, "bonding_period":0, "bonding_period_variable":false, "bonding_rewards":false, "unbonding_period":2419200, "unbonding_period_variable":false, "unbonding_rewards":false, "exit_queue_period":0}, "apr_estimate": {"low": "5.0000", "high": "10.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ESVXUUO-SGM5I-XQ5HOO", "asset": "MATIC", "lock_type": {"type": "instant", "payout_frequency":604800}, "apr_estimate": {"low": "1.0000", "high": "3.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ESOR43Q-2FWLQ-EOARUY", "asset": "MATIC", "lock_type": {"type": "bonded", "payout_frequency":604800, "bonding_period":0, "bonding_period_variable":false, "bonding_rewards":false, "unbonding_period":259200, "unbonding_period_variable":false, "unbonding_rewards":false, "exit_queue_period":0}, "apr_estimate": {"low": "3.0000", "high": "6.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ESLFAYW-3V3B4-UJ42RV", "asset": "MINA", "lock_type": {"type": "instant", "payout_frequency":604800}, "apr_estimate": {"low": "9.0000", "high": "15.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ESPB45U-RZV2F-T6UO3H", "asset": "SOL", "lock_type": {"type": "instant", "payout_frequency":604800}, "apr_estimate": {"low": "2.0000", "high": "4.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ESITPR4-NOX6X-SELCID", "asset": "SOL", "lock_type": {"type": "bonded", "payout_frequency":604800, "bonding_period":0, "bonding_period_variable":false, "bonding_rewards":false, "unbonding_period":259200, "unbonding_period_variable":false, "unbonding_rewards":false, "exit_queue_period":0}, "apr_estimate": {"low": "5.0000", "high": "7.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ESUQUGM-LIFEU-2EJF6T", "asset": "LUNA", "lock_type": {"type": "instant", "payout_frequency":604800}, "apr_estimate": {"low": "0.0000", "high": "0.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":false, "allocation_restriction_info":[]},
                {"id": "ESXI3CZ-JV3YY-CWONEA", "asset": "FLR", "lock_type": {"type": "instant", "payout_frequency":604800}, "apr_estimate": {"low": "0.1000", "high": "2.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ESQ3L5E-5GBYA-OB75IU", "asset": "SCRT", "lock_type": {"type": "instant", "payout_frequency":604800}, "apr_estimate": {"low": "9.0000", "high": "13.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ESSJ6WL-HASM2-47Q6A4", "asset": "SCRT", "lock_type": {"type": "bonded", "payout_frequency":604800, "bonding_period":0, "bonding_period_variable":false, "bonding_rewards":false, "unbonding_period":1814400, "unbonding_period_variable":false, "unbonding_rewards":false, "exit_queue_period":0}, "apr_estimate": {"low": "21.0000", "high": "26.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "enabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ESDQCOL-WTZEU-NU55QF", "asset": "ETH", "lock_type": {"type": "bonded", "payout_frequency":604800, "bonding_period":0, "bonding_period_variable":true, "bonding_rewards":false, "unbonding_period":432145, "unbonding_period_variable":true, "unbonding_rewards":false, "exit_queue_period":59}, "apr_estimate": {"low": "3.0000", "high": "6.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "disabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":true, "allocation_restriction_info":[]},
                {"id": "ES4BY2A-WJLDA-ZI34OX", "asset": "FLOWH", "lock_type": {"type": "bonded", "payout_frequency":604800, "bonding_period":0, "bonding_period_variable":true, "bonding_rewards":false, "unbonding_period":0, "unbonding_period_variable":true, "unbonding_rewards":false, "exit_queue_period":0}, "apr_estimate": {"low": "6.0000", "high": "10.0000"}, "user_min_allocation": "0.01", "allocation_fee": "0.0000", "deallocation_fee": "0.0000", "auto_compound": {"type": "disabled"}, "yield_source": {"type": "staking"}, "can_allocate":false, "can_deallocate":false, "allocation_restriction_info":[]}
            ]
        }
    })
}

pub fn get_list_earn_allocations_json() -> Value {
    json!({
        "error": [],
        "result": {
            "converted_asset": "USD",
            "total_allocated": "0.0000",
            "total_rewarded": "150.9128",
            "next_cursor":null,
            "items": [
                {
                    "strategy_id": "ES3OO4J-FF3XJ-4HFOZK",
                    "native_asset": "ATOM",
                    "amount_allocated": {
                        "total": {"native": "0.00000000", "converted": "0.0000"}
                    },
                    "total_rewarded": {"native": "1.62081900", "converted": "12.8271"}
                },
                {
                    "strategy_id": "ESZCVV6-CKS62-GJ2YOP",
                    "native_asset": "KAVA",
                    "amount_allocated": {
                        "total": {"native": "0.00000000", "converted": "0.0000"}
                    },
                    "total_rewarded": {
                        "native": "4.47424121",
                        "converted": "3.0922"
                    }
                }
            ]
        }
    })
}
