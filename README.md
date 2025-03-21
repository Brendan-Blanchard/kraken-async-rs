# Kraken-Async-Rs

![badge](https://github.com/Brendan-Blanchard/kraken-async-rs/actions/workflows/main.yml/badge.svg) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![codecov](https://codecov.io/gh/Brendan-Blanchard/kraken-async-rs/graph/badge.svg?token=30Y7BIDSNK)](https://codecov.io/gh/Brendan-Blanchard/kraken-async-rs)

A complete[^3] wrapper of the Kraken Pro trading API (v2 websockets), written in asynchronous Rust.

It's not expected that you'll be able to use Kraken-Async-Rs without consulting
the [Kraken REST API](https://docs.kraken.com/api/docs/rest-api/get-server-time) docs,
and [Kraken Websockets V2](https://docs.kraken.com/api/docs/websocket-v2/add_order) docs.

There are many details and interdependencies[^1] to each request that are not documented or enforced in
the library since they're outside this library's control and subject to change.

### Example: Calling a Public Endpoint

Public endpoint calls are as easy as creating a client object and awaiting a request. Since no API secrets are required,
a blank static set of credentials is provided using `StaticSecretsProvider` with empty `&str` values. See
the [full example](examples/live_public_endpoint_request.rs) for imports.

```rust
#[tokio::main]
async fn main() {
    // credentials aren't needed for public endpoints
    let secrets_provider: Arc<Mutex<Box<dyn SecretsProvider>>> = Box::new(Arc::new(Mutex::new(StaticSecretsProvider::new("", ""))));
    let nonce_provider: Arc<Mutex<Box<dyn NonceProvider>>> =
        Arc::new(Mutex::new(Box::new(IncreasingNonceProvider::new())));
    let mut client = CoreKrakenClient::new(secrets_provider, nonce_provider);

    let request = TradableAssetPairsRequest::builder()
        .pair(Pairs::new(vec!["BTCUSD".to_string()]))
        .build();

    let open_orders_response = client.get_tradable_asset_pairs(&request).await;

    // Note that Kraken will return assets in their own naming scheme, e.g. a request for
    // "BTCUSD" will return as "XXBTZUSD"
    // For a reasonable understanding of their mappings, see: https://gist.github.com/brendano257/975a395d73a6d7bb53e53d292534d6af
    if let Ok(ResultErrorResponse {
                  result: Some(tradable_assets),
                  ..
              }) = open_orders_response
    {
        for (asset, details) in tradable_assets {
            println!("{asset}: {details:?}")
        }
    }
}
```

### Example: Calling a Private Endpoint

Private endpoint calls require valid credentials, which can be provided statically by any means, or via an
`EnvSecretsProvider` that will automatically load an `.env` file in the project directory, and retrieve the specified
keys from the local env. See the [full example](examples/live_open_orders_request.rs) for imports.

```rust
#[tokio::main]
async fn main() {
    // note that this will fail if you don't have your key and secret set to these env vars
    // eg `export KRAKEN_KEY="YOUR-API-KEY"`, ...
    let secrets_provider: Arc<Mutex<Box<dyn SecretsProvider>>> = Box::new(Arc::new(Mutex::new(StaticSecretsProvider::new("", ""))));
    let nonce_provider: Arc<Mutex<Box<dyn NonceProvider>>> =
        Arc::new(Mutex::new(Box::new(IncreasingNonceProvider::new())));
    let mut client = CoreKrakenClient::new(secrets_provider, nonce_provider);

    let request = OpenOrdersRequest::builder().build();

    let open_orders_response = client.get_open_orders(&request).await;

    if let Ok(ResultErrorResponse {
                  result: Some(open_orders),
                  ..
              }) = open_orders_response
    {
        for (order_id, order) in open_orders.open {
            println!("{order_id}: {order:?}")
        }
    }
}
```

### Example: Listening to Websockets (V2)

Public websockets require no authentication, so it's as easy as creating a `v2::KrakenWSSClient`, connecting, and
sending any subscription methods and then awaiting the `.next()` method of the returned `KrakenMessageStream`.

You can also visit the [full example](examples/live_wss_ohlc.rs) with logging and imports.

```rust
#[tokio::main]
async fn main() {
    let mut client = KrakenWSSClient::new();
    let mut kraken_stream = client.connect::<WssMessage>().await.unwrap();

    let ohlc_params = OhlcSubscription::new(vec!["ETH/USD".into()], 60);
    let subscription = Message::new_subscription(ohlc_params, 0);

    let result = kraken_stream.send(&subscription).await;
    assert!(result.is_ok());

    while let Ok(Some(message)) = timeout(Duration::from_secs(10), kraken_stream.next()).await {
        if let Ok(response) = message {
            println!("{:?}", response);
        } else {
            println!("Message failed: {:?}", message);
        }
    }
}
```

### Request Details

Requests that have more than 1 or 2 parameters are generally given a struct, rather than having methods with many
parameters. The `builder` implementation enforces required parameter by using
the [simple-builder](https://crates.io/crates/simple-builder) package that marks
fields as required, ensuring they must be provided in the `.builder()` call. Any optional parameters can be added using
a fluent API.

For example, the Depth (orderbook) endpoint requires a pair, but can optionally take a `count` parameter for the number
of bids/asks on each side to return. The builder then behaves like below:

```
let request = OrderbookRequest::builder("ETHUSD".to_string())
        .count(500)
        .build();
```

### Response Details

A best-effort was made to adhere to the format of Kraken's responses, except for cases where it poses some pretty
severe usability limitations. Deserialization uses `serde`, and leaves most datatypes as-is, except Strings are
parsed to `rust_decimal::Decimal`, and many enums are used where the values are clearly documented. The majority
of `i64`
`String`/RFC3339, and `f64` timestamps remain as such. The goal was to provide a great base library for others to
build from, without limiting downstream uses by parsing everything and reducing overall performance. If you're
developing
general-purpose trading algorithms, you should be writing them over a common abstraction that can do this parsing
anyway.

If you disagree or have parsing, formatting, or any other issues or blocked use cases, please reach out with a clear
example of your issue!

### Security

- The `secrecy` crate is used to prevent accidental logging of websocket tokens in request and response objects
- Tracing of requests and responses are off by default, and **will log tokens when enabled**, as they log incoming and
  outgoing messages as strings, which cannot be redacted easily.

### Stability

- This crate aims to be a stable base for others to build from, following semantic versioning.
- Breaking changes will be noted ahead of time in the changelog as much as possible, and will be noted along with an
  upgrade plan in each version's changelog notes.

### Misc Details

- Parameters and response values are often renamed from the Kraken API fields to adhere to Rust's naming conventions or
  improve readability[^2]

### Contributions

This is a large project developed in isolation, and I undoubtedly missed things despite my best efforts. Please reach
out with a clear example of any bugs, usability problems, or suggestions for improvement!

[^1]: An example being the AddOrder endpoint that requires a "Good-'til-Date" order to also have a specified `endtm`
value. Cases like these are numerous and *not* enforced by this library.

[^2]: Examples include `refid` -> `ref_id`, `endtm` -> `end_time`, `ofs` -> `order_flags` (
or `offset`...), `vol_exec` -> `executed_volume`, `qty` -> `quantity`, and many more.

[^3]: This crate never supported NFT trading, however it has since been removed. If you find a feature missing, feel
free to contribute or reach out for support!