//! This example tests a backoff configuration using a function that returns false for if it should
//! retry, thus resulting in no retries at all.
use std::num::ParseIntError;
use std::str::FromStr;
use retry_if::{retry, ExponentialBackoffConfig};
use std::time::Duration;
use tokio::time::Instant;

const BACKOFF_CONFIG: ExponentialBackoffConfig = ExponentialBackoffConfig {
    max_retries: 5,
    t_wait: Duration::from_secs(1),
    backoff: 2.0,
    t_wait_max: None,
    backoff_max: None,
};

#[tokio::test]
async fn test_retry_with_try_operator_on_result() {
    fn retry_if(_: &Result<i32, ParseIntError>) -> bool {
        false
    }

    #[retry(BACKOFF_CONFIG, retry_if)]
    async fn method(int: &str) -> Result<i32, ParseIntError> {
        return Ok(i32::from_str(int)?);
    }

    let result = method("3").await;
    assert_eq!(Ok(3), result);
}

#[tokio::test]
async fn test_retry_with_try_operator_on_option() {
    fn retry_if(_: &Result<i32, ParseIntError>) -> bool {
        false
    }

    #[retry(BACKOFF_CONFIG, retry_if)]
    async fn method(int: &str) -> Option<i32> {
        return Some(i32::from_str(int).ok()?);
    }

    let result = method("3").await;
    assert_eq!(Some(3), result);
}
