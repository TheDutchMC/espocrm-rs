## 0.4.0 (2022-9-13)
- Added `tracing` support as an optional feature
- Updated dependencies

## 0.3.1
- More flexible dependency requirments
- Removed rustls and openssl features

## 0.3.0
- Added Method enum so end users don't have to include reqwest in their `Cargo.toml`
- Added missing `#[derive(Debug, Eq, PartialEq, Clone)]` to the Enums and Structs that were missing them
- Switched to using `AsRef<str>` instead of String for `EspoApiClient::request()`
- Type `NoGeneric` is now `()` instead of `u8`, that just makes way more sense

## 0.2.1
- Removed println! statements that were left in accidentally
- Renamed the `espocrm` module to `espo_api_client` (Has no effect on users of the crate)
- Removed unneeded dependency on `serde_php`, this should have been removed in 0.2.0

## 0.2.0
- Fixed HMAC authorization
- Added a serialized for GET requests. The crate `serde_php` did not work for this usecase.

## 0.1.0
Initial release