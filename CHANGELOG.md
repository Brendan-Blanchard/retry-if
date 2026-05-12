# Changelog

### In Progress

...

### v0.3.1

- Fix release by bumping `retry-if-macro` version and pointing to it

### v0.3.0

- Change `ExponentialBackoffConfig.max_retries` to `u32` to better reflect use
    - **Breaking change that will require clients moving this field to u32**
- Add `ExponentialBackoffConfig::get_backoff_duration` helper for custom uses

### v0.2.3

- Add optional feature support for serde
- Add PartialEq for `ExponentialBackoffConfig`
- Bump dependencies

### v0.2.1

- Derive Debug, Clone, Copy for `ExponentialBackoffConfig`