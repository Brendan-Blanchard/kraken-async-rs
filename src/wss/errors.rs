//! Top level errors produced by [KrakenWSSClient] and [KrakenMessageStream]
//!
#[allow(unused)]
use crate::wss::kraken_wss_client::KrakenMessageStream;
#[allow(unused)]
#[allow(deprecated)]
use crate::wss::kraken_wss_client::KrakenWSSClient;
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
