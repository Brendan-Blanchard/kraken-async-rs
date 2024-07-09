use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct Token(Secret<String>);

impl Token {
    pub fn new(token: impl Into<String>) -> Self {
        Token(Secret::new(token.into()))
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[REDACTED TOKEN]")
    }
}

impl Eq for Token {}

impl PartialEq for Token {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<'de> Deserialize<'de> for Token {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let token = String::deserialize(deserializer)?;
        Ok(Token(Secret::new(token)))
    }
}

impl Serialize for Token {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.expose_secret())
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::secrets::Token;

    #[test]
    fn test_secret_debug() {
        let token = Token::new("shhh");

        let str = format!("{:?}", token);

        assert_eq!("[REDACTED TOKEN]", str);
    }
}
