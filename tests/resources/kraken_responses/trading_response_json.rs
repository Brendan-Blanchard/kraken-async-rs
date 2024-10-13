use serde_json::{json, Value};

pub fn get_add_order_json() -> Value {
    json!({
        "error": [],
        "result": {
            "txid":["AKB9L1-XC5U3-CYCTO1"],
            "descr": {
                "order":"buy 5.00000000 USDCUSD @ limit 0.9000"
            }
        }
    })
}

pub fn get_add_order_batch_json() -> Value {
    json!({
        "error": [],
        "result": {
            "orders": [{
                "txid": "OZICHZ-FGB63-156I4K",
                "descr": {
                    "order":"buy 5.10000000 USDCUSD @ limit 0.9000"
                }
            },{
                "txid": "BEGNMD-FEJKF-VC6U8Y",
                "descr": {
                    "order":"buy 5.20000000 USDCUSD @ limit 0.9000"
                }
            }]
        }
    })
}

pub fn get_amend_order_json() -> Value {
    json!({"error":[],"result":{"amend_id":"TVB4ER-X5QP3-ADURMW"}})
}

pub fn get_edit_order_json() -> Value {
    json!({
        "error":[],
        "result": {
            "status": "ok",
            "txid": "7BD466-BKZVM-FT2E2L",
            "originaltxid": "969KIO-6IUMQ-GYFSAI",
            "newuserref": 1234,
            "volume": "5.10000000",
            "price": "0.89",
            "orders_cancelled": 1,
            "descr" : {"order":"buy 5.10000000 USDCUSD @ limit 0.89"}
        }
    })
}

pub fn get_cancel_order_json() -> Value {
    json!({
        "error": [],
        "result": {
            "count":1
        }
    })
}

pub fn get_cancel_all_orders_json() -> Value {
    json!({
        "error": [],
        "result": {
            "count": 4
        }
    })
}

pub fn get_cancel_all_orders_after_json() -> Value {
    json!({
        "error": [],
        "result": {
            "currentTime": "2023-03-24T17:41:56Z",
            "triggerTime": "2023-03-24T17:42:56Z"
        }
    })
}

pub fn get_cancel_order_batch_json() -> Value {
    json!({
        "error": [],
        "result": {
            "count":2
        }
    })
}
