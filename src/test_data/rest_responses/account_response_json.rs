use serde_json::{json, Value};

pub fn get_account_balance_json() -> Value {
    json!({
        "error": [],
        "result": {
            "ARB": "14.00000",
            "ATOM": "12.50385601",
            "ATOM.S": "0.00000000",
            "AVAX": "4.5430332562",
            "BSX": "180000.00",
            "EOS": "19.9000000000",
            "ETHW": "0.1122822",
            "FET": "45.0000000000",
            "GNO": "0.0000000100",
            "INJ": "4.0000000000",
            "KAVA": "100.44972893",
            "KAVA.S": "0.00000000",
            "KEY": "8500.0000000000",
            "LDO": "8.0000000000",
            "LUNA": "41634.98976651",
            "LUNA2": "0.77441494",
            "MATIC": "18.7054353600",
            "NEAR": "12.99000000",
            "RNDR": "7.5000000000",
            "SAND": "3.0000000000",
            "SOL": "1.3624452700",
            "USDC": "0.00000000",
            "USDT": "0.00000000",
            "XETC": "0.0000000000",
            "XETH": "0.1190976100",
            "XXBT": "0.0022698000",
            "ZUSD": "22.6354"
        }
    })
}

pub fn get_extended_balance_json() -> Value {
    json!({
        "error":[],
        "result": {
            "ARB": {
                "balance": "12.00000",
                "hold_trade": "0.00000"
            },
            "ATOM": {
                "balance": "14.50385601",
                "hold_trade": "3.92433600"
            },
            "ATOM.S": {
                "balance": "0.00000000",
                "hold_trade": "0.00000000"
            },
            "AVAX": {
                "balance": "1.5430332562",
                "hold_trade": "0.0000000000"
            },
            "BSX": {
                "balance": "140000.00",
                "hold_trade": "0.00"
            },
            "EOS": {
                "balance": "12.9000000000",
                "hold_trade": "0.0000000000"
            },
            "ETHW": {
                "balance": "0.1122822",
                "hold_trade": "0.0000000"
            },
            "FET": {
                "balance": "45.0000000000",
                "hold_trade": "0.0000000000"
            },
            "GNO": {
                "balance": "0.0000000100",
                "hold_trade": "0.0000000000"
            },
            "INJ": {
                "balance": "2.0000000000",
                "hold_trade": "0.0000000000"
            },
            "KAVA": {
                "balance": "100.44972893",
                "hold_trade": "0.00000000"
            },
            "KAVA.S": {
                "balance": "0.00000000",
                "hold_trade": "0.00000000"
            },
            "KEY": {
                "balance": "4500.0000000000",
                "hold_trade": "0.0000000000"
            },
            "LDO": {
                "balance": "4.0000000000",
                "hold_trade": "0.0000000000"
            },
            "LUNA": {
                "balance": "41634.98976651",
                "hold_trade": "21650.98976651"
            },
            "LUNA2": {
                "balance": "0.77441494",
                "hold_trade": "0.00000000"
            },
            "MATIC": {
                "balance": "18.7054353600",
                "hold_trade": "0.0000000000"
            },
            "NEAR": {
                "balance": "12.99000000",
                "hold_trade": "0.00000000"
            },
            "RNDR": {
                "balance": "4.5000000000",
                "hold_trade": "0.0000000000"
            },
            "SAND": {
                "balance": "1.0000000000",
                "hold_trade": "0.0000000000"
            },
            "SOL": {
                "balance": "1.3628452600",
                "hold_trade": "0.0000000000"
            },
            "USDC": {
                "balance": "0.00000000",
                "hold_trade": "0.00000000"
            },
            "USDT": {
                "balance": "0.00000000",
                "hold_trade": "0.00000000"
            },
            "XETC": {
                "balance": "0.0000000000",
                "hold_trade": "0.0000000000"
            },
            "XETH": {
                "balance": "0.1690976100",
                "hold_trade": "0.0000000000"
            },
            "XXBT": {
                "balance": "0.0022698000",
                "hold_trade": "0.0000000000"
            },
            "XXLM": {
                "balance": "0.00000000",
                "hold_trade": "0.00000000"
            },
            "ZUSD": {
                "balance": "22.6354",
                "hold_trade": "0.0000"
            }
        }
    })
}

pub fn get_trade_balance_json() -> Value {
    json!({
        "error":[],
        "result": {
            "eb": "5491.4558",
            "tb": "4204.0545",
            "m": "0.0000",
            "uv": "0.0000",
            "n": "0.0000",
            "c": "0.0000",
            "v": "0.0000",
            "e": "4204.0545",
            "mf": "4204.0545"
        }
    })
}

pub fn get_open_orders_json() -> Value {
    json!({
        "error":[],
        "result": {
            "open": {
                "604X4L-ANXHT-JV0ZQT": {
                    "refid":null,
                    "userref":0,
                    "cl_ord_id": "some-uuid",
                    "status": "open",
                    "opentm": 1676384710.121142,
                    "starttm":0,
                    "expiretm":0,
                    "descr": {
                        "pair": "ATOMETH",
                        "type": "sell",
                        "ordertype": "limit",
                        "price": "0.008770",
                        "price2": "0",
                        "leverage": "none",
                        "order": "sell 1.00000000 ATOMETH @ limit 0.008770",
                        "close": ""
                    },
                    "vol": "1.00000000",
                    "vol_exec": "0.00000000",
                    "cost": "0.00000000",
                    "fee": "0.00000000",
                    "price": "0.00000000",
                    "stopprice": "0.00000000",
                    "limitprice": "0.00000000",
                    "trigger": "index",
                    "margin": false,
                    "misc": "",
                    "oflags": "fciq"
                },
                "R62T2L-0U03M-GJ4MBG": {
                    "refid":null,
                    "link_id": "APCXQQ-SAYXM-3CC6EY",
                    "userref":0,
                    "status": "open",
                    "opentm": 1652451289.7784367,
                    "starttm":0,
                    "expiretm":0,
                    "descr": {
                        "pair": "LUNAUSD",
                        "type": "sell",
                        "ordertype": "limit",
                        "price": "0.00500000",
                        "price2": "0",
                        "leverage": "none",
                        "order": "sell 10000.00000000 LUNAUSD @ limit 0.00500000",
                        "close": ""
                    },
                    "vol": "10000.00000000",
                    "vol_exec": "0.00000000",
                    "cost": "0.00000",
                    "fee": "0.00000",
                    "price": "0.000000000000",
                    "stopprice": "0.000000000000",
                    "limitprice": "0.000000000000",
                    "misc": "",
                    "oflags": "fcib"
                },
                "NWG1I6-WTZ8K-ZSBGTX": {
                    "refid":null,
                    "link_id": "V95Y0A-INMTK-UIN1NQ",
                    "userref":0,
                    "status": "open",
                    "opentm": 1652451255.3537865,
                    "starttm":0,
                    "expiretm":0,
                    "descr": {
                        "pair": "LUNAUSD",
                        "type": "sell",
                        "ordertype": "limit",
                        "price": "0.00200000",
                        "price2": "0",
                        "leverage": "none",
                        "order": "sell 10000.00000000 LUNAUSD @ limit 0.00200000",
                        "close": ""
                    },
                    "vol": "10000.00000000",
                    "vol_exec": "0.00000000",
                    "cost": "0.00000",
                    "fee": "0.00000",
                    "price": "0.000000000000",
                    "stopprice": "0.000000000000",
                    "limitprice": "0.000000000000",
                    "misc": "",
                    "oflags": "fcib"
                },
                "AU251H-0YAWE-UBCP1K": {
                    "refid":null,
                    "link_id": "8NWZZQ-LC1ZS-VBO3FM",
                    "userref":0,
                    "status": "open",
                    "opentm": 1652396227.3224382,
                    "starttm":0,
                    "expiretm":0,
                    "descr": {
                        "pair": "LUNAUSD",
                        "type": "sell",
                        "ordertype": "limit",
                        "price": "0.01340000",
                        "price2": "0",
                        "leverage": "none",
                        "order": "sell 1019.21128070 LUNAUSD @ limit 0.01340000",
                        "close": ""
                    },
                    "vol": "1019.21128070",
                    "vol_exec": "0.00000000",
                    "cost": "0.00000",
                    "fee": "0.00000",
                    "price": "0.000000000000",
                    "stopprice": "0.000000000000",
                    "limitprice": "0.000000000000",
                    "misc": "",
                    "oflags": "fcib"
                },
                "BSKT9F-324Z8-TB8HXX": {
                    "refid":null,
                    "link_id": "9YTV8L-5R3H8-GGT2BQ",
                    "userref":0,
                    "status": "open",
                    "opentm": 1652358142.4768453,
                    "starttm":0,
                    "expiretm":0,
                    "descr": {
                        "pair": "LUNAUSD",
                        "type": "sell",
                        "ordertype": "limit",
                        "price": "0.04000000",
                        "price2": "0",
                        "leverage": "none",
                        "order": "sell 980.12448581 LUNAUSD @ limit 0.04000000",
                        "close": ""
                    },
                    "vol": "980.12448581",
                    "vol_exec": "0.00000000",
                    "cost": "0.00000",
                    "fee": "0.00000",
                    "price": "0.000000000000",
                    "stopprice": "0.000000000000",
                    "limitprice": "0.000000000000",
                    "misc": "",
                    "oflags": "fcib"
                },
                "BI6MMI-IT0HC-W1ZXP2": {
                    "refid":null,
                    "link_id": "196QZ9-50DB2-H1VNJW",
                    "userref":0,
                    "status": "open",
                    "opentm": 1652290509.2748847,
                    "starttm":0,
                    "expiretm":0,
                    "descr": {
                        "pair": "LUNAUSD",
                        "type": "sell",
                        "ordertype": "limit",
                        "price": "4.10000000",
                        "price2": "0",
                        "leverage": "none",
                        "order": "sell 40.00000000 LUNAUSD @ limit 4.10000000",
                        "close": ""
                    },
                    "vol": "40.00000000",
                    "vol_exec": "0.00000000",
                    "cost": "0.00000",
                    "fee": "0.00000",
                    "price": "0.000000000000",
                    "stopprice": "0.000000000000",
                    "limitprice": "0.000000000000",
                    "misc": "",
                    "oflags": "fcib"
                }
            }
        }
    })
}

pub fn get_closed_orders_json() -> Value {
    json!({
        "error":[],
        "result": {
            "closed": {
                "KGOWV8-P15FK-YG75U7":{"refid":null,"userref":0,"status":"closed","opentm":1721519777.553314,"starttm":0,"expiretm":0,"descr":{"pair":"ETHUSD","aclass":"forex","type":"buy","ordertype":"limit","price":"750.00","price2":"0","leverage":"none","order":"buy 0.05000000 ETHUSD @ limit 1500.00","close":""},"vol":"0.05000000","vol_exec":"0.05000000","cost":"37.50000","fee":"0.09375","price":"750.00","stopprice":"0.00000","limitprice":"0.00000","misc":"","oflags":"fciq","reason":null,"closetm":1731519777.553314},
                "MSMU5X-PKW32-HQCHHD":{"refid":null,"userref":0,"status":"closed","opentm":1713586800.623244,"starttm":0,"expiretm":0,"descr":{"pair":"ALGOUSD","aclass":"forex","type":"sell","ordertype":"limit","price":"0.00570","price2":"0","leverage":"none","order":"sell 851.00000000 ALGOUSD @ limit 0.00570","close":""},"vol":"851.00000000","vol_exec":"851.00000000","cost":"4.8507000000000","fee":"0.01249","price":"0.00570","stopprice":"0.00000","limitprice":"0.00000","misc":"","oflags":"fciq","reason":null,"closetm":1723586800.623244},
                "1OX8PJ-DOX9B-1TAW4M":{"refid":null,"userref":0,"status":"canceled","opentm":1732443716.701851,"starttm":0,"expiretm":0,"descr":{"pair":"SOLUSD","aclass":"forex","type":"sell","ordertype":"limit","price":"0.01050","price2":"0","leverage":"none","order":"sell 89.90977172 SOLUSD @ limit 0.01050","close":""},"vol":"89.90977172","vol_exec":"0.00000000","cost":"0.00000","fee":"0.00000","price":"0.00000","stopprice":"0.00000","limitprice":"0.00000","misc":"","oflags":"fciq","reason":"User requested","closetm":1742443716.701851}
            },
            "count":99
        }
    })
}

pub fn get_query_order_info_json() -> Value {
    json!({
        "error": [],
        "result": {
            "P8E9L2-ITMSU-UGBBRE": {
                "refid":null,
                "link_id": "2AHH5M-YR054-MCF0Y8",
                "userref":0,
                "status": "canceled",
                "opentm": 1699792641.5845017,
                "starttm":0,
                "expiretm":0,
                "descr":{
                    "pair": "USDCUSD",
                    "type": "buy",
                    "ordertype": "limit",
                    "price": "0.9000",
                    "price2": "0",
                    "leverage": "none",
                    "order": "buy 20.00000000 USDCUSD @ limit 0.9000",
                    "close": ""
                },
                "vol": "20.00000000",
                "vol_exec": "0.00000000",
                "cost": "0.00000000",
                "fee": "0.00000000",
                "price": "0.00000000",
                "stopprice": "0.00000000",
                "limitprice": "0.00000000",
                "misc": "",
                "oflags": "fciq",
                "reason": "Order replaced",
                "closetm": 1699792641.705669}
        }
    })
}

pub fn get_order_amends_json() -> Value {
    serde_json::from_str(r#"{
        "error":[],
        "result": {
            "amends": [
                {"amend_id":"TST2AA-CTCTU-MVJDHR","amend_type":"original","order_qty":"5.12340000","remaining_qty":"5.12340000","limit_price":"0.9500","post_only":true,"timestamp":1728821182545},
                {"amend_id":"TXH3X2-E4ADJ-CH53N2","amend_type":"user","order_qty":"5.25000000","remaining_qty":"5.25000000","limit_price":"0.9600","reason":"User requested","post_only":true,"timestamp":1728821182969}
            ],
            "count": 2
        }
    }"#).unwrap()
}

pub fn get_trades_history_json() -> Value {
    json!({
        "error": [],
        "result": {
            "count":396,
            "trades": {
                "DH9ICY-FXRTM-GESBKK": {
                    "ordertxid": "R0X871-F5PY6-A5707B",
                    "postxid": "BV7YXR-DTHM9-AZRON4",
                    "pair": "ATOMUSD",
                    "time": 1700237532.1690326,
                    "type": "buy",
                    "ordertype": "limit",
                    "price": "8.250000",
                    "cost": "44.250000",
                    "ledgers": ["R0X871-F5PY6-A5707B", "BV7YXR-DTHM9-AZRON4"],
                    "fee": "0.079000",
                    "vol": "4.00000000",
                    "margin": "0.000000",
                    "leverage": "0",
                    "misc": "",
                    "trade_id":6179175,
                    "maker": true
                },

                "VN6IJE-7BQX2-YXRLGM": {
                    "ordertxid": "39HUZX-AYB9W-WK9G32",
                    "postxid": "UN68UU-N0823-UNAW5F",
                    "pair": "XETHZUSD",
                    "time": 1_700_272_061.860_628_6,
                    "type": "buy",
                    "ordertype": "limit",
                    "price": "1912.68000",
                    "cost": "18.67280",
                    "fee": "0.03248",
                    "vol": "0.01200000",
                    "margin": "0.00000",
                    "leverage": "0",
                    "misc": "",
                    "trade_id":46006064,
                    "maker": true
                },
                "6H44OW-RTMBC-VIACPX": {
                    "ordertxid": "3WCTEM-C4M3D-CQU27S",
                    "postxid": "MXKZYL-HOXOX-DUNP1K",
                    "pair": "XXBTZUSD",
                    "time": 1_685_189_574.653_626_4,
                    "type": "sell",
                    "ordertype": "stop limit",
                    "price": "27600.00000",
                    "cost": "43.01683",
                    "fee": "0.07423",
                    "vol": "0.00166745",
                    "margin": "0.00000",
                    "leverage": "0",
                    "misc": "",
                    "trade_id": 12596111,
                    "maker": false
                },
                "GIDTGJ-7RU68-HJQ4BW": {
                    "ordertxid": "VYU6NY-55R66-GTP041",
                    "postxid": "SHT1ZT-08GCQ-E16A71",
                    "pair": "LDOUSD",
                    "time": 1_685_055_943.837_868_5,
                    "type": "buy",
                    "ordertype": "limit",
                    "price": "2.01750",
                    "cost": "16.13000",
                    "fee": "0.03589",
                    "vol": "7.00000000",
                    "margin": "0.00000",
                    "leverage": "0",
                    "misc": "",
                    "trade_id":697350,
                    "maker": true
                },
                "91GDCC-0MSCG-RSTXO2": {
                    "ordertxid": "RBKJUX-1D86E-GNAO4C",
                    "postxid": "FW89UG-NHGVW-F1NKNI",
                    "pair": "ATOMETH",
                    "time": 1_685_053_473.940_939_4,
                    "type": "sell",
                    "ordertype": "limit",
                    "price": "0.00712500",
                    "cost": "0.0889900",
                    "fee": "0.00022568",
                    "vol": "11.00000000",
                    "margin": "0.00000000",
                    "leverage": "0",
                    "misc": "",
                    "trade_id":978550,
                    "maker": false
                },
            }
        }
    })
}

pub fn get_query_trades_info_json() -> Value {
    json!({
        "error":[],
        "result": {
            "X2Q49U-F3M8J-7HK49O": {
                "ordertxid": "0628QD-9SIQF-UQ72SR",
                "postxid": "2HCNKV-A84MB-UFBO92",
                "pair": "RNDRUSD",
                "time": 1685055903.487446,
                "type": "buy",
                "ordertype": "limit",
                "price": "2.798000",
                "cost": "21.250270",
                "fee": "0.046423",
                "vol": "8.90000000",
                "margin": "0.000000",
                "leverage": "0",
                "misc": "",
                "trade_id": 670058,
                "maker": true,
                "ledgers": ["RBKJUX-1D86E-GNAO4C", "FW89UG-NHGVW-F1NKNI"]
            }
        }
    })
}

pub fn get_open_positions_json() -> Value {
    json!({
        "error": [ ],
        "result": {
            "TF5GVO-T7ZZ2-6NBKBI": {
                "ordertxid": "OLWNFG-LLH4R-D6SFFP",
                "posstatus": "open",
                "pair": "XXBTZUSD",
                "time": 1605280097.8294,
                "type": "buy",
                "ordertype": "limit",
                "cost": "104610.52842",
                "fee": "289.06565",
                "vol": "8.82412861",
                "vol_closed": "0.20200000",
                "margin": "20922.10568",
                "value": "258797.5",
                "net": "+154186.9728",
                "terms": "0.0100% per 4 hours",
                "rollovertm": "1616672637",
                "misc": "",
                "oflags": ""

            },
            "T24DOR-TAFLM-ID3NYP": {
                "ordertxid": "OIVYGZ-M5EHU-ZRUQXX",
                "posstatus": "open",
                "pair": "XXBTZUSD",
                "time": 1607943827.3172,
                "type": "buy",
                "ordertype": "limit",
                "cost": "145756.76856",
                "fee": "335.24057",
                "vol": "8.00000000",
                "vol_closed": "0.00000000",
                "margin": "29151.35371",
                "value": "240124.0",
                "net": "+94367.2314",
                "terms": "0.0100% per 4 hours",
                "rollovertm": "1616672637",
                "misc": "",
                "oflags": ""

            },
            "TYMRFG-URRG5-2ZTQSD": {
                "ordertxid": "OF5WFH-V57DP-QANDAC",
                "posstatus": "open",
                "pair": "XXBTZUSD",
                "time": 1610448039.8374,
                "type": "buy",
                "ordertype": "limit",
                "cost": "0.00240",
                "fee": "0.00000",
                "vol": "0.00000010",
                "vol_closed": "0.00000000",
                "margin": "0.00048",
                "value": "0",
                "net": "+0.0006",
                "terms": "0.0100% per 4 hours",
                "rollovertm": "1616672637",
                "misc": "",
                "oflags": ""
            },
            "TAFGBN-TZNFC-7CCYIM": {
                "ordertxid": "OF5WFH-V57DP-QANDAC",
                "posstatus": "open",
                "pair": "XXBTZUSD",
                "time": 1610448039.8448,
                "type": "buy",
                "ordertype": "limit",
                "cost": "2.40000",
                "fee": "0.00264",
                "vol": "0.00010000",
                "vol_closed": "0.00000000",
                "margin": "0.48000",
                "value": "3.0",
                "net": "+0.6015",
                "terms": "0.0100% per 4 hours",
                "rollovertm": "1616672637",
                "misc": "",
                "oflags": ""
            },
            "T4O5L3-4VGS4-IRU2UL": {
                "ordertxid": "OF5WFH-V57DP-QANDAC",
                "posstatus": "open",
                "pair": "XXBTZUSD",
                "time": 1610448040.7722,
                "type": "buy",
                "ordertype": "limit",
                "cost": "21.59760",
                "fee": "0.02376",
                "vol": "0.00089990",
                "vol_closed": "0.00000000",
                "margin": "4.31952",
                "value": "27.0",
                "net": "+5.4133",
                "terms": "0.0100% per 4 hours",
                "rollovertm": "1616672637",
                "misc": "",
                "oflags": ""
            }
        }
    })
}
pub fn get_open_positions_json_do_calc_optional_fields() -> Value {
    json!({
        "error": [ ],
        "result": {
            "TF5GVO-T7ZZ2-6NBKBI": {
                "ordertxid": "OLWNFG-LLH4R-D6SFFP",
                "posstatus": "open",
                "pair": "XXBTZUSD",
                "time": 1605280097.8294,
                "type": "buy",
                "ordertype": "limit",
                "cost": "104610.52842",
                "fee": "289.06565",
                "vol": "8.82412861",
                "vol_closed": "0.20200000",
                "margin": "20922.10568",
                "terms": "0.0100% per 4 hours",
                "rollovertm": "1616672637",
                "misc": "",
                "oflags": ""
            }
        }
    })
}

pub fn get_ledgers_info_json() -> Value {
    json!({
        "error": [],
        "result": {
            "count":324,
            "ledger": {
                "ZKFTQO-BR8G7-08P80H": {
                    "aclass": "currency",
                    "amount": "0.03216614",
                    "asset": "LUNA2",
                    "balance": "0.77441494",
                    "fee": "0.00000000",
                    "refid": "sU8gV7R-9gjgqXS7dKojLDNE07vHRV",
                    "time": 1669369455.5642295,
                    "type": "transfer",
                    "subtype": "spotfromfutures"
                },
                "NX79ER-E401Z-GS13W4": {
                    "aclass": "currency",
                    "amount": "5.00000000",
                    "asset": "ATOM",
                    "balance": "186.50385601",
                    "fee": "0.00000000",
                    "refid": "ORYFVO-2QKEH-N80DLS",
                    "time": 1641488510.5417955,
                    "type": "trade",
                    "subtype": ""
                },
                "JNORYE-CJ718-2PBW5A": {
                    "aclass": "currency",
                    "amount": "-46.2500",
                    "asset": "ZUSD",
                    "balance": "22.6354",
                    "fee": "0.0740",
                    "refid": "315VIC-SWM75-SK45AW",
                    "time": 1690904249.6254137,
                    "type": "trade",
                    "subtype": ""
                },
                "1VRIJ3-YRAC4-ST14ZO": {
                    "aclass": "currency",
                    "amount": "0.0100000000",
                    "asset": "XETH",
                    "balance": "0.1690976100",
                    "fee": "0.0000000000",
                    "refid": "3SF28K-KIQEH-EC9U5L",
                    "time": 1627300547.7191358,
                    "type": "trade",
                    "subtype": ""
                },
                "J777RM-RLSRM-E0YGDD": {
                    "aclass": "currency",
                    "amount": "-19.6768",
                    "asset": "ZUSD",
                    "balance": "67.9684",
                    "fee": "0.0315",
                    "refid": "SYCRCT-X2YZ4-19Y45G",
                    "time": 1623619565.8740494,
                    "type": "trade",
                    "subtype": ""
                },
                "LKA0VV-57H22-I1WYNX": {
                    "aclass": "currency",
                    "amount": "5.00000000",
                    "asset": "ATOM",
                    "balance": "18.51384603",
                    "fee": "0.00000000",
                    "refid": "41RZVA-AH099-F9W5DB",
                    "time": 1636710826.358058,
                    "type": "trade",
                    "subtype": ""
                },
                "CR349H-07OK9-F2Y6HY": {
                    "aclass": "currency",
                    "amount": "-45.0000",
                    "asset": "ZUSD",
                    "balance": "88.6677",
                    "fee": "0.0720",
                    "refid": "R340AX-024WV-D1UB4R",
                    "time": 1608003276.3846428,
                    "type": "trade",
                    "subtype": ""
                },
                "PDVG4B-Q8ILN-XCFWHK": {
                    "aclass": "currency",
                    "amount": "92.55309000",
                    "asset": "USDC",
                    "balance": "104.54408600",
                    "fee": "0.00000000",
                    "refid": "2DZfaYQ-6YUvRTTjBx6S7X8d37Hhv5",
                    "time": 1601356660.6152797,
                    "type": "deposit",
                    "subtype": ""
                },
                "13RBZT-771B9-TV0THC": {
                    "aclass": "currency",
                    "amount": "0.00000050",
                    "asset": "KAVA",
                    "balance": "100.44972893",
                    "fee": "0.00000000",
                    "refid": "A4FVGO-0H6LX-OX0BPV",
                    "time": 1665891991.553499,
                    "type": "transfer",
                    "subtype": "stakingtospot"
                },
                "2VOMAM-8YKCR-J5MR45": {
                    "aclass": "currency",
                    "amount": "-0.0610000000",
                    "asset": "XXBT",
                    "balance": "0.0022698000",
                    "fee": "0.0000500000",
                    "refid": "Hhh0rnC-2vpSSffLijqUDRbxgKG4JV",
                    "time": 1658844473.4411042,
                    "type": "withdrawal",
                    "subtype": ""
                },
                "6SFDWO-BVIXH-5A16E3": {
                    "aclass": "currency",
                    "amount": "500.0000",
                    "asset": "ZUSD",
                    "balance": "1080.8551",
                    "fee": "0.0000",
                    "refid": "S32OSV-P42DI-WC5X34",
                    "time": 1663497440.3613548,
                    "type": "deposit",
                    "subtype": ""
                }
            }
        }
    })
}

pub fn get_query_ledgers_json() -> Value {
    json!({
        "error": [],
        "result": {
            "LMWW7L-EWV66-ZXPC3L": {
                "aclass": "currency",
                "amount": "-90.0000",
                "asset": "USD",
                "balance": "133.7397",
                "fee": "0.1440",
                "refid": "TAUGAY-Z4ZJO-O3POKN",
                "time": 1699961059.3636901,
                "type": "trade",
                "subtype": ""
            }
        }
    })
}

pub fn get_trade_volume_json() -> Value {
    json!({
        "error": [],
        "result": {
            "currency": "ZUSD",
            "volume": "296.0268",
            "fees":null,
            "fees_maker":null
        }
    })
}

pub fn get_trade_volume_per_pair_json() -> Value {
    json!({
        "error":[],
        "result": {
            "currency": "ZUSD",
            "volume": "294.0219",
            "fees": {
                "XXBTZUSD": {
                    "fee": "0.2600",
                    "minfee": "0.1000",
                    "maxfee": "0.2600",
                    "nextfee": "0.2400",
                    "tiervolume": "0.0000",
                    "nextvolume": "50000.0000"
                }
            },
            "fees_maker": {
                "XXBTZUSD": {
                    "fee": "0.1600",
                    "minfee": "0.0000",
                    "maxfee": "0.1600",
                    "nextfee": "0.1400",
                    "tiervolume": "0.0000",
                    "nextvolume": "50000.0000"
                }
            }
        }
    })
}

pub fn get_request_export_report_json() -> Value {
    json!({
        "error": [],
        "result": {"id": "KQMO"}
    })
}

pub fn get_export_report_response() -> Vec<u8> {
    vec![0, 1, 0, 1]
}

pub fn get_export_report_status_json() -> Value {
    json!({
        "error": [],
        "result": [
            {
                "id": "KQMO",
                "descr": "Test Export",
                "format": "CSV",
                "report": "ledgers",
                "status": "Processed",
                "aclass": "currency",
                "fields": "all",
                "asset": "all",
                "subtype": "all",
                "starttm": "1698811200",
                "endtm": "1701406800",
                "createdtm": "1701614189",
                "expiretm": "1702823789",
                "completedtm": "1701614194",
                "datastarttm": "1698811200",
                "dataendtm": "1701406800",
                "flags": "0"
            }
        ]
    })
}

pub fn get_delete_export_report_json() -> Value {
    json!({"error": [], "result": {"delete": true}})
}
