# Changelog

### In Progress

- Made `KrakenClient.set_user_agent` async (not required but allows Tokio mutex proxying downstream)
- Changed `vol` and `vol_exec` to `volume` and `volume_executed` in all places
- Added String data to several variants of `ClientError` where additional information is helpful downstream
- Added From<...> impls for StringCSV, OrderFlags, and IntOrString
    - These reduce things like `StringCSV::new(vec!["someTxId".to_string()])` to `"someTxId".into()`
- Added example of querying an order to /examples

### v0.0.1

- Initial version