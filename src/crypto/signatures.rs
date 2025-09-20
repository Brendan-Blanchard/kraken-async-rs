//! Core signature implementation for signing messages
use base64::Engine;
use base64::engine::general_purpose::STANDARD as base64;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256, Sha512};

/// Struct containing the encoded message body and finalized signature
pub struct Signature {
    pub body_data: String,
    pub signature: String,
}

/// Generates the signature for an arbitrary request when provided with a nonce, API secret key,
/// the endpoint, and the encoded data being sent.
///
/// This is HMAC-SHA512(uri + sha256(nonce + post_data)), but the exact details are given by
/// [`Kraken's documentation`].
///
/// Errors can occur due to formatting, url-encoding (or not) of specific data, and other details,
/// but this implementation does not specify that `encoded_data` is anything but a [String].
///
/// [`Kraken's documentation`]: https://docs.kraken.com/rest/#section/Authentication/Headers-and-Signature
pub fn generate_signature(
    nonce: u64,
    secret: &str,
    endpoint: &str,
    encoded_data: String,
) -> Signature {
    let mut hmac = Hmac::<Sha512>::new_from_slice(&base64.decode(secret.as_bytes()).unwrap())
        .expect("Could not use private key to create HMAC");

    let mut sha256 = Sha256::new();

    sha256.update(nonce.to_string().as_bytes());
    sha256.update(encoded_data.as_bytes());

    let payload = sha256.finalize();

    hmac.update(endpoint.as_bytes());
    hmac.update(&payload[..]);

    Signature {
        body_data: encoded_data,
        signature: base64.encode(hmac.finalize().into_bytes()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use to_query_params::{QueryParams, ToQueryParams};
    use url::form_urlencoded;

    #[derive(QueryParams, Serialize)]
    struct QueryData {
        #[serde(rename = "ordertype")]
        #[query(required, rename = "ordertype")]
        order_type: String,
        #[query(required)]
        pair: String,
        #[query(required)]
        price: String,
        #[query(required)]
        #[query(rename = "type")]
        #[serde(rename = "type")]
        order_side: String,
        #[query(required)]
        volume: String,
    }

    #[test]
    fn test_generate_signature_form_data() {
        let expected = "4/dpxb3iT4tp/ZCVEwSnEsLxx0bqyhLpdfOpc6fn7OR8+UClSV5n9E6aSS8MPtnRfp32bAb0nmbRn6H8ndwLUQ==";
        let key = "kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg==";

        let nonce = 1616492376594_u64;

        let post_data = QueryData {
            order_type: "limit".into(),
            pair: "XBTUSD".into(),
            price: "37500".into(),
            order_side: "buy".into(),
            volume: "1.25".into(),
        };

        let mut query_params = form_urlencoded::Serializer::new(String::new());
        query_params.append_pair("nonce", &nonce.to_string());

        for (key, value) in post_data.to_query_params().iter() {
            query_params.append_pair(key, value);
        }

        let encoded_data = query_params.finish();

        let signature = generate_signature(nonce, key, "/0/private/AddOrder", encoded_data);

        assert_eq!(expected, signature.signature);
    }

    #[test]
    fn test_generate_signature_json_data() {
        let expected = "oTOXlYtwCD1eL/j45C8gSWB49XQO1Sguv3nnScc8TTNpgmsnDvAA3yu6geyXXjGIsfCUEOzslsv4ugTZNsM7RA==";
        let key = "kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg==";

        let nonce = 1616492376594_u64;

        let post_data = QueryData {
            order_type: "limit".into(),
            pair: "XBTUSD".into(),
            price: "37500".into(),
            order_side: "buy".into(),
            volume: "1.25".into(),
        };

        let encoded_data = serde_json::to_string(&post_data).unwrap();

        let signature = generate_signature(nonce, key, "/0/private/AddOrder", encoded_data);

        assert_eq!(expected, signature.signature);
    }
}
