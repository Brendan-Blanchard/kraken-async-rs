// duplicates of those in core_kraken_client.rs, but can't be shared from here to UTs
pub const ERROR_PERMISSION_DENIED: &str = r#"{"error":["EGeneral:Permission denied"]}"#;
pub const ERROR_INVALID_KEY: &str = r#"{"error":["EAPI:Invalid key"]}"#;
pub const ERROR_UNKNOWN_ASSET_PAIR: &str = r#"{"error":["EQuery:Unknown asset pair"]}"#;
