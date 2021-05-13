## 0.1.0
Initial release

## 0.2.0
- Fixed HMAC authorization
- Added a serialized for GET requests. The crate `serde_php` did not work for this usecase.

## 0.2.1
- Removed println! statements that were left in accidentally
- Renamed the `espocrm` module to `espo_api_client` (Has no effect on users of the crate)
- Removed unneeded dependency on `serde_php`, this should have been removed in 0.2.0