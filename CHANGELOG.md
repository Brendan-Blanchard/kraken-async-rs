# Changelog

### In Progress

- Considering package re-organization for better imports etc.
- Convenient type defs and functions for verbose type signatures like `Box<Arc<Mutex<dyn SecretsProvider>>>`

### v0.4.x

- Fix `StatusUpdate.connection_id` after observing it's not always populated

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