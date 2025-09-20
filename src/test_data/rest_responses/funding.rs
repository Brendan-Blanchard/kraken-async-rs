use serde_json::{Value, json};

pub fn get_deposit_methods_json() -> Value {
    json!({
        "error":[],
        "result": [
            {
                "method": "Ether (Hex)",
                "limit": false,
                "gen-address": true,
                "minimum": "0.0500000000"
            },
            {
                "method": "Ethereum (ERC20)",
                "limit": false,
                "gen-address": true,
                "minimum": "0.0000100000"
            },
            {
                "method": "Arbitrum One",
                "limit": false,
                "gen-address": true,
                "minimum": "0.0000100000"
            },
            {
                "method": "Ethereum (Polygon)",
                "limit": false,
                "gen-address": true,
                "minimum": "0.0000100000"
            },
            {
                "method": "Arbitrum Nova",
                "limit": false,
                "gen-address": true,
                "minimum": "0.0000100000"
            },
            {
                "method" :"Optimism",
                "limit": false,
                "gen-address": true,
                "minimum": "0.0000100000"
            },
            {
                "method": "zkSync Era",
                "limit": false,
                "gen-address": true,
                "minimum": "0.0200000000"
            }
        ]
    })
}

pub fn get_deposit_addresses_json() -> Value {
    json!({
        "error":[],
        "result": [
            {"address":"17SkEw2md5avVNyYgj6RiXuQKNwkXaxFyQ","expiretm":"0"}
        ]
    })
}

pub fn get_status_of_recent_deposits_json() -> Value {
    json!({
        "error":[],
        "result": [
            {
                "method":"Bitcoin",
                "aclass":"currency",
                "asset":"XXBT",
                "refid":"PXRIYW-S3GP2-R2VOGQ",
                "txid":"a1075db55d416d3ca199f55b6084e2115b9345e16c5cf302fc80e9d5fbf5d48d",
                "info":"17SkEw2md5avVNyYgj6RiXuQKNwkXaxFyQ",
                "amount":"0.0021200000",
                "fee":"0.0000000000",
                "time":1676754614,
                "status":"Success"
            }
        ]
    })
}

pub fn get_withdrawal_methods_json() -> Value {
    json!({
        "error":[],
        "result": [
            {"asset":"ZUSD","method":"MVB Bank (Wire)","network":null,"minimum":"20.00"},
            {"asset":"ZUSD","method":"ACH (Plaid Transfer, via Plaid)","network":null,"minimum":"1.00"},
            {"asset":"ZUSD","method":"Customers Bank (Fedwire)","network":null,"minimum":"20.00"},
            {"asset":"XXBT","method":"Bitcoin","network":"Bitcoin","minimum":"0.0004"},
            {"asset":"XXBT","method":"Bitcoin Lightning","network":"Lightning","minimum":"0.00001"},
            {"asset":"XXRP","method":"Ripple XRP","network":"Ripple","minimum":"25"},
            {"asset":"USDT","method":"Tether USD (TRC20)","network":"Tron","minimum":"6.00"},
            {"asset":"USDT","method":"Tether USD (SPL)","network":"Solana","minimum":"2.00"},
            {"asset":"USDT","method":"Tether USD (Polygon)","network":"Polygon","minimum":"2.00"},
            {"asset":"USDT","method":"Arbitrum One Network","network":"Arbitrum One","minimum":"4.00"},
            {"asset":"USDT","method":"Optimism","network":"Optimism","minimum":"2.50"},
            {"asset":"XMLN","method":"MLN","network":"Ethereum","minimum":"0.52"},
            {"asset":"DASH","method":"Dash","network":null,"minimum":"0.01"},
            {"asset":"DASH","method":"Dash Instant Send","network":null,"minimum":"0.01"},
            {"asset":"GNO","method":"GNO","network":"Ethereum","minimum":"0.10"},
            {"asset":"EOS","method":"EOS","network":"EOS","minimum":"0.5"},
            {"asset":"BCH","method":"Bitcoin Cash","network":"Bitcoin Cash","minimum":"0.0006"},
            {"asset":"USDC","method":"TRC20 (USDC)","network":"Tron","minimum":"6.00"},
            {"asset":"USDC","method":"USDC (Polygon)","network":"Polygon - USDC.e","minimum":"2.00"},
            {"asset":"USDC","method":"Polygon - Native","network":"Polygon","minimum":"2.00"},
            {"asset":"USDC","method":"Arbitrum One Network","network":"Arbitrum One - USDC.e","minimum":"4.00"},
            {"asset":"USDC","method":"Arbitrum One Network - Native","network":"Arbitrum One","minimum":"4.00"},
            {"asset":"USDC","method":"Optimism","network":"Optimism - USDC.e","minimum":"2.50"},
            {"asset":"GHST","method":"Aavegotchi (GHST)","network":"Ethereum","minimum":"11.00"},
            {"asset":"MINA","method":"Mina","network":"Mina","minimum":"7.5"},
            {"asset":"SOL","method":"Solana","network":"Solana","minimum":"0.02"},
            {"asset":"SRM","method":"Serum","network":"Solana","minimum":"20.00"},
            {"asset":"WBTC","method":"Wrapped Bitcoin (WBTC)","network":"Ethereum","minimum":"0.0005"},
            {"asset":"CHZ","method":"Chiliz (CHZ)","network":"Ethereum","minimum":"88.00"},
            {"asset":"1INCH","method":"1inch (1INCH)","network":"Ethereum","minimum":"22.00"},
            {"asset":"AXS","method":"Axie Infinity Shards (AXS)","network":"Ethereum","minimum":"1.34"},
            {"asset":"OGN","method":"Origin Protocol (OGN)","network":"Ethereum","minimum":"92.00"},
            {"asset":"CQT","method":"\tCovalent Query Token (CQT)","network":"Ethereum","minimum":"66.00"},
            {"asset":"UNFI","method":"Unifi Protocol DAO","network":"Ethereum","minimum":"2.40"},
            {"asset":"FET","method":"Fetch.AI (FET)","network":"Fetch.ai","minimum":"50.00"},
            {"asset":"GARI","method":"Gari Network (GARI)","network":"Solana","minimum":"30.00"},
            {"asset":"GST","method":"Green Satoshi Token (GST)","network":"Solana","minimum":"100.00"},
            {"asset":"GMT","method":"STEPN (GMT)","network":"Solana","minimum":"3.00"},
            {"asset":"MSOL","method":"Marinade SOL (MSOL)","network":"Solana","minimum":"0.01"},
            {"asset":"BIT","method":"Bitdao (BIT)","network":"Ethereum","minimum":"22.00"},
            {"asset":"LUNA2","method":"Luna2","network":"Terra 2.0","minimum":"0.5"},
            {"asset":"ETHW","method":"Ethereum PoW","network":"Ethereum PoW","minimum":"0.10"},
        ]
    })
}

pub fn get_withdrawal_addresses_json() -> Value {
    json!({
        "error":[],
        "result":[
            {"address":"cosmos1c4k24jzduc365kywrsvf5ujz4ya6mwymy8vq4q","asset":"ATOM","method":"Cosmos","key":"Coinbase Custody","verified":true,"memo":"12349999"},
            {"address":"0x758ce03A82Bf1433A5b38aE78c2F97F4977694Ab","asset":"USDC","method":"USDC","key":"Some Other Metamask","verified":true},
            {"address":"17SkEw2md5avVNyYgj6RiXuQKNwkXaxFyQ","asset":"XBT","method":"Bitcoin","key":"Bitcoin Pizza","verified":true},
            {"address":"0x12aD4b2E9A11dD98664A4E033aD4658C39a896d6","asset":"ETH","method":"Ether","key":"Some Metamask","verified":true}
        ]
    })
}

pub fn get_withdrawal_info_json() -> Value {
    json!({
        "error": [ ],
        "result": {
            "method": "Bitcoin",
            "limit": "332.00956139",
            "amount": "0.72480000",
            "fee": "0.00020000"
        }
    })
}

pub fn get_withdraw_funds_json() -> Value {
    json!({
        "error": [],
        "result": {
            "refid": "FTQcuak-V6Za8qrWnhzTx67yYHz8Tg"
        }
    })
}

pub fn get_status_of_recent_withdrawals_json() -> Value {
    json!({
      "error": [],
      "result": [
        {
          "method": "Bitcoin",
          "aclass": "currency",
          "asset": "XXBT",
          "refid": "FTQcuak-V6Za8qrWnhzTx67yYHz8Tg",
          "txid": "THVRQM-33VKH-UCI7BS",
          "info": "mzp6yUVMRxfasyfwzTZjjy38dHqMX7Z3GR",
          "amount": "0.72485000",
          "fee": "0.00020000",
          "time": 1688014586,
          "status": "Pending",
          "key": "btc-wallet-1"
        },
        {
          "method": "Bitcoin",
          "aclass": "currency",
          "asset": "XXBT",
          "refid": "FTQcuak-V6Za8qrPnhsTx47yYLz8Tg",
          "txid": "KLETXZ-33VKH-UCI7BS",
          "info": "mzp6yUVMRxfasyfwzTZjjy38dHqMX7Z3GR",
          "amount": "0.72485000",
          "fee": "0.00020000",
          "time": 1688015423,
          "status": "Failure",
          "status-prop": "canceled",
          "key": "btc-wallet-2"
        }
      ]
    })
}

pub fn get_request_withdrawal_cancellation_json() -> Value {
    json!({
        "error": [],
        "result": true
    })
}

pub fn get_request_wallet_transfer_json() -> Value {
    json!({
        "error": [],
        "result": {
            "refid": "FTQcuak-V6Za8qrWnhzTx67yYHz8Tg"
        }
    })
}
