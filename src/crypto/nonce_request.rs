//! Struct for  wrapping arbitrary data with a nonce on serialization
#[allow(unused)]
use crate::clients::core_kraken_client::CoreKrakenClient;
use serde::Serialize;

/// A generic wrapper around any serializable request data that contains the nonce for a request.
///
/// Using `#[serde(flatten)]`, this allows any serializable request to have a nonce field added to
/// its output.
///
/// Users are able to, but unlikely to need to use this at all as nonces are generated in the HTTP
/// methods internal to [`CoreKrakenClient`].
#[derive(Serialize)]
pub struct NonceRequest<'r, R>
where
    R: Serialize,
{
    nonce: u64,
    #[serde(flatten)]
    request: &'r R,
}

impl<'r, R> NonceRequest<'r, R>
where
    R: Serialize,
{
    pub fn new(nonce: u64, request: &'r R) -> Self {
        NonceRequest { nonce, request }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[test]
    fn test_nonce_request() {
        // an arbitrary T: Serialize request should be flattened into an object containing the nonce
        //  when serialized

        #[derive(Serialize)]
        struct Request {
            id: u64,
            name: String,
        }

        let nonce_request = NonceRequest {
            nonce: 999,
            request: &Request {
                name: "SomeRequest".to_string(),
                id: 123,
            },
        };

        assert_eq!(
            "{\"nonce\":999,\"id\":123,\"name\":\"SomeRequest\"}",
            serde_json::to_string(&nonce_request).unwrap()
        );
    }
}
