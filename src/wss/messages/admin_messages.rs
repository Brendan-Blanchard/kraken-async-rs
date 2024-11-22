use crate::response_types::SystemStatus;
use serde::Deserialize;
use serde_json::Number;

#[derive(Debug, Deserialize, PartialEq)]
pub struct StatusUpdate {
    pub api_version: String,
    // true type is i128, but serde does not support it: https://github.com/serde-rs/json/issues/740
    pub connection_id: Option<Number>,
    pub system: SystemStatus,
    pub version: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    /// ['serde_json'] doesn't support i128 natively, so using the Number type allows parsing. If
    /// users need access to it, Number can be converted to i128 with additional serde features.
    #[test]
    fn test_deserialize_status_update() {
        let message = r#"{"api_version":"v2","connection_id":18266300427528990701,"system":"online","version":"2.0.4"}"#;
        let expected = StatusUpdate {
            api_version: "v2".to_string(),
            connection_id: Some(Number::from_str("18266300427528990701").unwrap()),
            system: SystemStatus::Online,
            version: "2.0.4".to_string(),
        };

        let parsed = serde_json::from_str::<StatusUpdate>(message);

        assert_eq!(expected, parsed.unwrap());

        // what's needed after adding features = ["arbitrary_precision"] for serde_json

        // let expected_connection_id: i128 = 18266300427528990701;
        // let connection_id: i128 = i128::from_str(parsed.unwrap().connection_id.as_str()).unwrap();
        // assert_eq!(expected_connection_id, connection_id);
    }
}
