//! Helpers in parsing websocket messages
use serde_json::{Map, Value};

/// Gets the event field from a `&Map<String, Value>` as `Option<&str>`
pub fn get_event_field(o: &Map<String, Value>) -> Option<&str> {
    o.get("event")?.as_str()
}

/// Get the second to last field of an array, expecing it to be an `&str`
pub fn get_event_from_vec(v: &[Value]) -> Option<&str> {
    if v.len() > 1 {
        v.get(v.len() - 2)?.as_str()
    } else {
        None
    }
}
