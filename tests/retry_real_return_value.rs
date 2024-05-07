use retry_if::{retry, ExponentialBackoffConfig};
use std::num::ParseIntError;
use std::str::FromStr;
use std::time::Duration;

const BACKOFF_CONFIG: ExponentialBackoffConfig = ExponentialBackoffConfig {
    max_retries: 5,
    t_wait: Duration::from_secs(1),
    backoff: 2.0,
    t_wait_max: None,
    backoff_max: None,
};

#[tokio::test]
async fn test_simple_retryable() {
    let val = retryable().await;
    assert_eq!(Ok(3), val);
}

fn retry_if(_: &Result<i32, ParseIntError>) -> bool {
    false
}

#[retry(BACKOFF_CONFIG, retry_if)]
async fn retryable() -> Result<i32, ParseIntError> {
    // need async call to detect improper usage in macro
    tokio::time::sleep(Duration::from_secs(0)).await;
    i32::from_str("3")
}
