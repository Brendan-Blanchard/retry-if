# Retry-If

![badge](https://github.com/Brendan-Blanchard/retry-if/actions/workflows/main.yml/badge.svg)[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A predicate-based retry decorator for retrying arbitrary functions using an exponential backoff strategy.

This library aims to make decorating your code with a retry strategy as easy as possible, all that's required is a
defined retry strategy and a function that determines if the output of your code needs to be retried.

# Example: Retrying a Result-producing Function on Err(...)

The below example sets up a basic retry configuration that will retry up to five times, waiting at first 1 second, then
2 seconds, 4 seconds, etc. There is no configured maximum time to retry across all attempts (t_wait_max), nor is there
any maximum waiting time on each backoff (backoff_max).

```rust
use retry_if::{retry, ExponentialBackoffConfig};
use std::num::TryFromIntError;
use std::time::Duration;
use tokio::time::{pause, Instant};

const BACKOFF_CONFIG: ExponentialBackoffConfig = ExponentialBackoffConfig {
    max_retries: 5,
    t_wait: Duration::from_secs(1),
    backoff: 2.0,
    t_wait_max: None,
    backoff_max: None,
};

// this takes an address of the same type as the output of the decorated function.
//  It returns true if the function should be retried based on the result
fn retry_if(result: &Result<i64, TryFromIntError>) -> bool {
    result.is_err()
}

#[retry(BACKOFF_CONFIG, retry_if)]
async fn fallible_call() -> Result<i64, TryFromIntError> {
    // this will always produce a TryFromIntError, triggering a retry
    i64::try_from(i128::MAX)
}

#[tokio::main]
async fn main() {
    let start = Instant::now();

    let _ = fallible_call().await;

    let end = Instant::now();

    let elapsed = end - start;

    // expected waits are 1s, 2s, 4s, 8s, 16s = 31s
    assert!(elapsed > Duration::from_secs(31));
    assert!(elapsed < Duration::from_millis(31100));

    println!("Total test time: {elapsed:?}");
}
```

### Tracing

The crate exposes `tracing` as a feature to enable logging using the tokio [`tracing`] library's `tracing::info!` for
each of the retry attempts. These currently take the form of: `Sleeping {Duration:?} on attempt {i32}`. The output
traces will take on whatever instrumented scope is given to the parent function.

[`tracing`]: https://crates.io/crates/tracing

### Limitations

`#[retry(...)]` can decorate almost all cases of async functions. This includes:

- free functions such as `async fn do_thing() -> i32`
- methods in impl blocks such as `async fn do_thing(&mut self) -> bool`
- trait implementations in `impl <Trait> for <Struct> {}` blocks, such as `async fn do_thing(&self) -> String`

Use cases that retry-if cannot be applied to include:

- Functions that take and consume `self`, since ownership is passed and `self` may no longer exist after the first call
- Functions that rely on the try (?) operator on `Option`

# Example: Non-Working Function That Consumes Self

A non-working example of this is shown below, where `to_thing()` consumes `self`, making a second call impossible.

```rust
struct Thing {}

struct Other {}

impl Thing {
    #[retry(...)]
    async fn to_thing(self) -> Other {
        self.into()
    }
}
```

# Example: Non-Working Function That Uses Try on Option

A non-working example of this is shown below, where the function uses the try operator to exit early with an `Option`.
This cannot work because the code is expanded with `Result<T, E>` as the primary use case, and it's not possible
to determine if the `?` applies to a `Result` or an `Option` when looking at TokenStreams in the macro, making expansion
to both impossible when parsing.

```rust
struct Thing {}

struct Other {}

impl Thing {
    #[retry(...)]
    async fn do_thing(self) -> Option<i32> {
        // compilation fails here because this is expanded to match on `get_data()` and the match arms are Ok & Err
        //  instead of Some & None
        let data = get_data()?;
        data * 2
    }
}
```

### Contributions

Please reach out if you encounter edge cases, have any suggestions for improving the API, or have any clarifications
that can be made.