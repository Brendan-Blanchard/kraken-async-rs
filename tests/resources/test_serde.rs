use serde::Deserialize;
use std::fmt::{Debug, Display};

pub fn test_display_output<T>(variant: T, expected: &str)
where
    T: Display,
{
    assert_eq!(expected, variant.to_string());
}

pub fn test_deserializing_expecting_error<'de, T>(data: &'de str, expected_error: &str)
where
    T: Debug + Deserialize<'de>,
{
    let message: Result<T, serde_json::Error> = serde_json::from_str(data);

    assert!(message.is_err());
    let err = message.unwrap_err();
    assert_eq!(expected_error, err.to_string())
}
