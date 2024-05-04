use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
use kraken_async_rs::request_types::TimeInForce;
use kraken_async_rs::response_types::{BuySell, OrderFlag, OrderType};
use kraken_async_rs::secrets::secrets_provider::EnvSecretsProvider;
use kraken_async_rs::wss::kraken_wss_client::{KrakenMessageStream, KrakenWSSClient};
use kraken_async_rs::wss::private::messages::PrivateMessage;
use kraken_async_rs::wss::private::trading_messages::AddOrderRequest;
use rust_decimal_macros::dec;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::timeout;
use tokio_stream::StreamExt;

/// This places an unlikely order via websockets, provided KRAKEN_KEY and KRAKEN_SECRET are properly
/// configured env vars.
///
/// The order is for 10 USDC at a 10% discount, and expires in 5 seconds.
///
/// For private streams, a token is required to connect (within 15 minutes of creating the token).
/// The token is retrieved using `KrakenClient`'s `get_websockets_token` parameter, the result of
/// which goes in any request made via the websocket connection.
#[tokio::main]
async fn main() {
    let secrets_provider = Box::new(EnvSecretsProvider::new("KRAKEN_KEY", "KRAKEN_SECRET"));
    let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> =
        Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));
    let mut kraken_client = CoreKrakenClient::new(secrets_provider, nonce_provider);

    let resp = kraken_client.get_websockets_token().await.unwrap();

    let mut client = KrakenWSSClient::new();
    let mut kraken_stream: KrakenMessageStream<PrivateMessage> =
        client.connect_auth().await.unwrap();

    let request = AddOrderRequest::builder(
        "addOrder".to_string(),
        resp.result.unwrap().token.clone(),
        OrderType::Limit,
        BuySell::Buy,
        "USDC/USD".to_string(),
        dec!(10.00),
    )
    .price(dec!(0.90))
    .order_flags(vec![OrderFlag::Post])
    .expire_time("+5".to_string())
    .time_in_force(TimeInForce::GTD)
    .validate("true".to_string())
    .build();

    kraken_stream.send(&request).await.unwrap();

    while let Ok(Some(message)) = timeout(Duration::from_secs(10), kraken_stream.next()).await {
        if let Ok(response) = message {
            println!("{:?}", response);
        } else {
            println!("Message failed: {:?}", message);
        }
    }
}
