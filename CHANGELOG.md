# Changelog

### In Progress

- Fix `BalancesSubscription` sending null for snapshot by default (skip_serializing_none)
- Fix `WSSMessage` by adding WssMessage(ErrorResponse { .. }) variant
- Change `checksum` fields for L2 and L3 books to `u32` to match crc32fast crate impl
- Derive Clone for `Message<T>` and all `*Subscription` types
- Bump dependencies
- Derive `Clone` for `RateLimitedKrakenClient` so clones can share the rate limiters
- Derive `Debug` for `CoreKrakenClient` for consistency

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