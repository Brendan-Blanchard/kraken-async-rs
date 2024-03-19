use kraken_async_rs::wss::errors::WSSError;
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
