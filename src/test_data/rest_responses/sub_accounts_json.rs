use serde_json::{Value, json};

pub fn get_create_sub_account_json() -> Value {
    json!({
        "error": [],
        "result": true
    })
}
pub fn get_account_transfer_json() -> Value {
    json!({
        "error": [],
        "result": {
            "transfer_id": "TOH3AS2-LPCWR8-JDQGEU",
            "status": "complete"
        }
    })
}
