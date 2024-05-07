use retry_if::{retry, ExponentialBackoffConfig};
use std::num::ParseIntError;
use std::str::FromStr;
use std::time::Duration;

const INVALID_BACKOFF_CONFIG: ExponentialBackoffConfig = ExponentialBackoffConfig {
    max_retries: 0,
    t_wait: Duration::from_secs(1),
    backoff: 2.0,
    t_wait_max: None,
    backoff_max: None,
};

#[should_panic(expected = "retries failed to produce a value; max_retries must be >= 1")]
#[tokio::test]
async fn test_invalid_configuration() {
    let _ = invalid_retryable().await;
}

fn retry_if(_: &Result<i32, ParseIntError>) -> bool {
    false
}

#[retry(INVALID_BACKOFF_CONFIG, retry_if)]
async fn invalid_retryable() -> Result<i32, ParseIntError> {
    // need async call to detect improper usage in macro
    tokio::time::sleep(Duration::from_secs(0)).await;
    i32::from_str("3")
}
