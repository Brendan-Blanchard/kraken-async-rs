//! HTTP response structure
use serde::Deserialize;

/// The API-wide response type, containing an optional result and maybe-empty list of errors for
/// each response.
#[derive(Debug, Deserialize, Clone)]
pub struct ResultErrorResponse<T> {
    pub result: Option<T>,
    pub error: Vec<String>,
}
