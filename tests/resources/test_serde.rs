use serde::Deserialize;
use std::fmt::{Debug, Display};

pub fn test_display_output<T>(variant: T, expected: &str)
where
    T: Display,
{
    assert_eq!(expected, variant.to_string());
}
