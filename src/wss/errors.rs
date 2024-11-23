//! Top level errors produced by [KrakenWSSClient] and [KrakenMessageStream]
//!
pub use serde_json::Error as SerdeError;
use std::error::Error;
use std::fmt::{Display, Formatter};
pub use tokio_tungstenite::tungstenite::Error as TungsteniteError;
pub use url::ParseError as UrlParseError;

/// Aggregate error type produced by [KrakenWSSClient] and [KrakenMessageStream]
#[derive(Debug)]
pub enum WSSError {
    Serde(SerdeError),
    WSS(TungsteniteError),
    UrlParse(UrlParseError),
}

impl From<SerdeError> for WSSError {
    fn from(value: SerdeError) -> Self {
        Self::Serde(value)
    }
}

impl From<TungsteniteError> for WSSError {
    fn from(value: TungsteniteError) -> Self {
        Self::WSS(value)
    }
}

impl From<UrlParseError> for WSSError {
    fn from(value: UrlParseError) -> Self {
        Self::UrlParse(value)
    }
}

impl Display for WSSError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WSSError::Serde(err) => write!(f, "{err}"),
            WSSError::WSS(err) => write!(f, "{err}"),
            WSSError::UrlParse(err) => write!(f, "{err}"),
        }
    }
}

impl Error for WSSError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            WSSError::Serde(e) => Some(e),
            WSSError::WSS(e) => Some(e),
            WSSError::UrlParse(e) => Some(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::wss::errors::WSSError;
    use serde::de::Error as DeError;
    use serde_json::Error as SerdeError;
    use tokio_tungstenite::tungstenite::Error as TungsteniteError;
    use url::ParseError as UrlParseError;

    #[test]
    fn test_error_conversion() {
        let serde_error = SerdeError::custom("id");
        let tungstenite_error = TungsteniteError::ConnectionClosed;
        let url_parse_error = UrlParseError::InvalidPort;

        let error = WSSError::from(serde_error);
        assert!(matches!(error, WSSError::Serde { .. }));

        let error = WSSError::from(tungstenite_error);
        assert!(matches!(error, WSSError::WSS { .. }));

        let error = WSSError::from(url_parse_error);
        assert!(matches!(error, WSSError::UrlParse { .. }));
    }
}
