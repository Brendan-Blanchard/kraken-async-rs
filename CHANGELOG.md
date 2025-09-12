# Changelog

### In Progress

- Considering package re-organization for better imports etc.
- Convenient type defs and functions for verbose type signatures like `Box<Arc<Mutex<dyn SecretsProvider>>>`

### v0.13.0

**All changes are breaking unless otherwise noted and given upgrade instructions.**

- Made `ErrorResponse.status` optional and added `ErrorResponse.symbol` to match real returned data
    - Update any raw struct definitions in client code
- Updated to tracing-subscriber 0.3.19 for security patch
- Update tokio to 1.47.1

### v0.12.0

**All changes are breaking unless otherwise noted and given upgrade instructions.**

- Enabled rust-decimal serde-with-arbitrary-precision feature to preserve trailing zeros (helps with book checksums)
- Changed `RecentTrade.time` to a `Decimal`, which side-steps a serde-json issue with `arbitrary-precision`
  feature: https://github.com/Brendan-Blanchard/kraken-async-rs/issues/13
    - Either consume as a `Decimal` or convert back down to `f64` (technically fallible, but very unlikely in this case)

### v0.11.0

**All changes are breaking unless otherwise noted and given upgrade instructions.**

- Add "stop market", "touched market" and "liquidation market" to `TradeType`
    - If you've matched on this enum exhaustively, these cases will need to be added. Otherwise, this is non-breaking.
- Add `timestamp` field to `L3Orderbook` and `L3OrderbookUpdate`
    - If you've instantiated these directly, struct definitions will require the new fields

### v0.10.0

**All changes are breaking unless otherwise noted and given upgrade instructions.**

- Remove `ClosedOrder` model - differences in documentation were incorrect
    - Upgrade by changing any `ClosedOrder` types to `Order` - the documentation had the "descr" field not present in
      responses (but it is now(?))
- Add missing `without_count` and `consolidate_taker` fields to `ClosedOrderRequest`
    - Upgrade by specifying these fields if you've used the struct syntax, and considering using a builder in the future
      to avoid additional parameters breaking your usage
- Update tokio for security vulnerability (non-breaking)
- Update rust_decimal, ws-mock (non-breaking)

### v0.9.0

- Add Trade.trade_id field, which was missing

### v0.8.0

**All changes are breaking unless otherwise noted and given upgrade instructions.**

- Change `RecentTrades.last` to `String` (since unix timestamp nanoseconds can be i128), and change
  `RecentTradesRequest.since` to `String` to match docs and allow for any precision input
    - Upgrade by changing anywhere you've consumed `RecentTrades.last`, or supplied `RecentTradesRequest.since` to use
      `String` instead of `i64`
        - The fast but unsafe (on read) approach would be `trades.last.parse::<i64>.unwrap()` or
          `request.since = Some(123.to_string())` if you relied on the value itself, but the `last` field should really
          only
          be used to pipe back in as the `since` field in a subsequent request, which should make changes minor
- Update to ws-mock v0.3.0 and tokio-tungstenite v0.26.2
    - This was breaking internally (core tungstenite types changed), but should not break clients unless they also
      depended on tokio-tungstenite < 0.26.0
- Update to secrecy v0.10.3
    - This is breaking if you consumed the `Secrets` class from this library directly, otherwise there should be no
      changes required

### v0.7.1

- Update minor versions of dependencies
- Fix flagged openssl vulnerability

### v0.7.0

**All changes are breaking unless otherwise noted and given upgrade instructions.**

- Minor version bumps for hyper, tokio, serde, url, tracing, tokio-stream, time, and tracing-subscriber (non-breaking)
- Relax type bounds for `KrakenWSSClient`'s `new_with_urls` and `new_with_tracing` (non-breaking)
- Remove V1 websockets, `KrakenWSSClient` and all associated tests and data
    - Upgrade path for existing V2 users:
        - Remove all imports with `::v2::` in them
        - Follow build failures and re-import removed imports
- Remove deprecated features `debug-inbound` and `debug-outbound`
    - Upgrade path:
        - Use the `new_with_tracing` methods on `KrakenWSSClient` and `KrakenClient` to enable tracing (disabled by
          default)
- Relax type bounds for `KrakenClient`'s `new_with_url` and `set_user_agent` methods to `impl ToString`
    - Upgrade path:
        - Update any implementations of the trait `KrakenClient` you've defined - otherwise, relaxing this type bound is
          non-breaking

### v0.6.0

- Deprecate `debug-inbound` and `debug-outbound` features
- Add `new_with_tracing` method to `KrakenClient` to allow setting a flag for tracing inbound REST messages
- Add `new_with_tracing` method to `KrakenWSSClient` to allow setting a flag for tracing inbound websocket messages

### v0.5.0

- Update to Spot REST V1, Spot Websockets V2.0.9 as of 26 September 2024 changelog notes
    - Adds `cl_ord_id` (client_order_id) to many types
    - Adds AmendOrder endpoint, WS request/responses for amending orders, and updates to many request and response types
    - Adds get_order_amends endpoint to retrieve amendments made to a particular order
    - Updated rate limiting to support amending orders and correcting their lifetimes for correct penalties
- Bump various dependencies
- Fix `StatusUpdate.connection_id` after observing it's not always populated
- Add test-support default feature for common example and testing code

### v0.4.2

- Fix `ExecutionResult` for trigger orders, where `actual_price`, `peak_price`, `last_price`, and `timestamp` are not
  necessarily provided

### v0.4.1

- Fix parsing of `ExecutionResult` after addition of new fields
- Fix `PositionStatusV2::Open` -> `Opened`
- Add cl_ord_id (client_order_id) to `AddOrderResult`, `CancelOrderResult`, `BatchOrder`, `BatchOrderParams`,
  `BatchCancelResponse`, and `ExecutionResult`

### v0.4.0

- Update tokio-tungstenite to 0.23.1, several breaking changes
- Fix serde and Display for SelfTradePrevention to be snake_case

### v0.3.0

- Add custom `Token(Secret<String>)` type for websocket tokens to avoid accidental exposure via logging etc
    - Feature-gated logging of incoming and outgoing messages to prevent disclosure of secrets by default
      (see `debug-inbound` and `debug-outbound` features)
- Add `Clone` to remaining response types, add `Copy` where sensible
- Modify `OpenPosition` to have `net` and `value` be optional (return depends on query params)

### v0.2.0

- Change `KrakenClient` signature to require `secrets_provider: Box<Arc<Mutex<dyn SecretsProvider>>>,` to enable `Clone`
  on `CoreKrakenClient`
- Fix `BalancesSubscription` sending null for snapshot by default (skip_serializing_none)
- Fix `WSSMessage` by adding `WssMessage(ErrorResponse { .. })` variant
- Change `checksum` fields for L2 and L3 books to `u32` to match crc32fast crate impl
- Derive Clone for `Message<T>` and all `*Subscription` types
- Derive `Clone` for `RateLimitedKrakenClient` so clones can share the rate limiters
- Derive `Debug` for `CoreKrakenClient` for consistency
- Make incoming WSS messages trace!(...) only; reduces logs with correct settings
- Make `ExecutionResult.order_user_ref` optional
- Bump dependencies

### v0.1.0

- Websockets V2 support - all V2 websocket endpoints are now supported
    - Deprecated v1 `KrakenWSSClient` since Kraken will support but no longer update v1
- Update dependencies (ws-mock -> 0.1.1)

### v0.0.2

Using rust_decimal::Decimal instead of Strings where applicable. Bug fixes, cleanup, more enum usage, and other changes
from using downstream and finding what drove poor patterns, etc.

- Decimal conversions instead of String (settled on this due to abundance/annoyance of Decimal::try_from(&someString)?
- Made `KrakenClient.set_user_agent` async (not required but allows Tokio mutex proxying downstream)
- Changed `vol` and `vol_exec` to `volume` and `volume_executed` in all places
- Added String data to several variants of `ClientError` where additional information is helpful downstream
- Added From<...> impls for StringCSV, OrderFlags, and IntOrString
    - These reduce things like `StringCSV::new(vec!["someTxId".to_string()])` to `"someTxId".into()`
- Added example of querying an order to /examples
- Updated dependencies, including `h2` for a security advisory, though it doesn't affect how it's used here
- Add `err(Debug)` tracing to all `CoreKrakenClient` methods
- Add `ret` tracing to any small responses and all trading endpoints in `CoreKrakenClient`
- Add `Clone` to WSS un/subscribe message types
- Add `Debug`, `Clone`, `Copy` to `KrakenWSSClient`
- Add `pub` to additional fields on structs in private and public WSS messages
- Change `OpenOrder` WSS type to use `OrderStatus` enum
- Used `TriggerType` enum in additional places

### v0.0.1

- Initial version