use serde_json::{Value, json};

pub fn get_websockets_token_json() -> Value {
    json!({
        "result":{
            "token": "nmc39wCfFqn0mirRrpHMFOu0xfq4VVghFy+UPzpVcJo",
            "expires": 900
        },
        "error": []
    })
}
