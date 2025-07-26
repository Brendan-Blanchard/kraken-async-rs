//! Client error type and sub-types
use hyper::http::uri::InvalidUri;
use hyper::http::Error as HyperHttpError;
use hyper::Error as HyperError;
use hyper_util::client::legacy::Error as HyperClientError;
use serde::Deserialize;
use serde_json::Error as SerdeError;
use std::error::Error;
use std::fmt::{Display, Formatter};
use url::ParseError as UrlParseError;

/// `ClientError::Kraken` contains all parsed error messages like `PermissionDenied` and
/// `UnknownMethod`, but is not inclusive of all potential error messages that can be present in
/// the `error` field of a response.
///
/// Various dependency errors are exposed, but may be of limited use aside from triggering retries.
#[derive(Debug)]
pub enum ClientError {
    Serde(SerdeError),
    Hyper(HyperError),
    HyperClient(HyperClientError),
    HyperHttp(HyperHttpError),
    HyperUri(InvalidUri),
    HttpStatus(String),
    UrlParse(UrlParseError),
    Parse(&'static str),
    Kraken(KrakenError),
}

impl From<HyperError> for ClientError {
    fn from(value: HyperError) -> Self {
        Self::Hyper(value)
    }
}

impl From<HyperHttpError> for ClientError {
    fn from(value: HyperHttpError) -> Self {
        Self::HyperHttp(value)
    }
}

impl From<HyperClientError> for ClientError {
    fn from(value: HyperClientError) -> Self {
        Self::HyperClient(value)
    }
}

impl From<UrlParseError> for ClientError {
    fn from(value: UrlParseError) -> Self {
        Self::UrlParse(value)
    }
}

impl From<InvalidUri> for ClientError {
    fn from(value: InvalidUri) -> Self {
        Self::HyperUri(value)
    }
}

impl From<SerdeError> for ClientError {
    fn from(value: SerdeError) -> Self {
        Self::Serde(value)
    }
}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::Serde(err) => write!(f, "{err}"),
            ClientError::Hyper(err) => write!(f, "{err}"),
            ClientError::HyperClient(err) => write!(f, "{err}"),
            ClientError::HyperHttp(err) => write!(f, "{err}"),
            ClientError::HyperUri(err) => write!(f, "{err}"),
            ClientError::HttpStatus(body) => write!(f, "Non-successful status with body: {body}"),
            ClientError::UrlParse(err) => write!(f, "{err}"),
            ClientError::Parse(err) => write!(f, "{err}"),
            ClientError::Kraken(err) => write!(f, "{err}"),
        }
    }
}

impl Error for ClientError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ClientError::Serde(e) => Some(e),
            ClientError::Hyper(e) => Some(e),
            ClientError::HyperClient(e) => Some(e),
            ClientError::HyperHttp(e) => Some(e),
            ClientError::HyperUri(e) => Some(e),
            ClientError::HttpStatus(_) => None,
            ClientError::UrlParse(e) => Some(e),
            ClientError::Parse(_) => None,
            ClientError::Kraken(e) => Some(e),
        }
    }
}

/// Enum of all parsed Kraken errors.
///
/// The user still has significant responsibilities in checking error fields!
///
/// Parsing of error messages is done for the general and higher-level errors produced by the API,
/// not including individual trading and request errors. For instance, `PermissionDenied` and
/// `UnknownAssetPair` are parsed because they're broadly applicable to many endpoints, while specific
/// trading errors like "InsufficientMargin" are not.
///
/// More documentation can be found on Kraken's [API error support page].
///
/// [API error support page]: https://support.kraken.com/hc/en-us/articles/360001491786-API-error-messages
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub enum KrakenError {
    PermissionDenied,
    InvalidKey,
    UnknownAssetPair,
    InvalidArguments(String),
    InvalidSignature,
    InvalidNonce,
    InvalidSession,
    BadRequest,
    UnknownMethod,
    RateLimitExceeded,
    TradingRateLimitExceeded,
    TemporaryLockout,
    ServiceUnavailable,
    ServiceBusy,
    InternalError,
    TradeLocked,
    FeatureDisabled,
}

impl Error for KrakenError {}

impl Display for KrakenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            KrakenError::PermissionDenied => write!(f, "PermissionDenied"),
            KrakenError::InvalidKey => write!(f, "InvalidKey"),
            KrakenError::UnknownAssetPair => write!(f, "UnknownAssetPair"),
            KrakenError::InvalidArguments(s) => write!(f, "{s}"),
            KrakenError::InvalidSignature => write!(f, "InvalidSignature"),
            KrakenError::InvalidNonce => write!(f, "InvalidNonce"),
            KrakenError::InvalidSession => write!(f, "InvalidSession"),
            KrakenError::BadRequest => write!(f, "BadRequest"),
            KrakenError::UnknownMethod => write!(f, "UnknownMethod"),
            KrakenError::RateLimitExceeded => write!(f, "RateLimitExceeded"),
            KrakenError::TradingRateLimitExceeded => write!(f, "TradingRateLimitExceeded"),
            KrakenError::TemporaryLockout => write!(f, "TemporaryLockout"),
            KrakenError::ServiceUnavailable => write!(f, "ServiceUnavailable"),
            KrakenError::ServiceBusy => write!(f, "ServiceBusy"),
            KrakenError::InternalError => write!(f, "InternalError"),
            KrakenError::TradeLocked => write!(f, "TradeLocked"),
            KrakenError::FeatureDisabled => write!(f, "FeatureDisabled"),
        }
    }
}

/// Parsing for all supported error types from the raw messages in `ResultErrorResponse.error`.
impl TryFrom<&String> for KrakenError {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        if value.starts_with("EGeneral:Permission denied") {
            Ok(KrakenError::PermissionDenied)
        } else if value.starts_with("EAPI:Invalid key") {
            Ok(KrakenError::InvalidKey)
        } else if value.starts_with("EQuery:Unknown asset pair") {
            Ok(KrakenError::UnknownAssetPair)
        } else if value.starts_with("EGeneral:Invalid arguments") {
            Ok(KrakenError::InvalidArguments(value.clone()))
        } else if value.starts_with("EAPI:Invalid signature") {
            Ok(KrakenError::InvalidSignature)
        } else if value.starts_with("EAPI:Invalid nonce") {
            Ok(KrakenError::InvalidNonce)
        } else if value.starts_with("ESession:Invalid session") {
            Ok(KrakenError::InvalidSession)
        } else if value.starts_with("EAPI:Bad request") {
            Ok(KrakenError::BadRequest)
        } else if value.starts_with("EGeneral:Unknown Method") {
            Ok(KrakenError::UnknownMethod)
        } else if value.starts_with("EAPI:Rate limit exceeded") {
            Ok(KrakenError::RateLimitExceeded)
        } else if value.starts_with("EOrder:Rate limit exceeded") {
            Ok(KrakenError::TradingRateLimitExceeded)
        } else if value.starts_with("EGeneral:Temporary lockout") {
            Ok(KrakenError::TemporaryLockout)
        } else if value.starts_with("EService:Unavailable") {
            Ok(KrakenError::ServiceUnavailable)
        } else if value.starts_with("EService:Busy") {
            Ok(KrakenError::ServiceBusy)
        } else if value.starts_with("EGeneral:Internal error") {
            Ok(KrakenError::InternalError)
        } else if value.starts_with("ETrade:Locked") {
            Ok(KrakenError::TradeLocked)
        } else if value.starts_with("EAPI:Feature disabled") {
            Ok(KrakenError::FeatureDisabled)
        } else {
            Err(())
        }
    }
}
