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
async fn test_retry_with_try_operator_on_result() {
    // show that the ? (Try) operator can be used and is properly expanded
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
async fn test_retry_with_try_operator_on_result_with_into() {
    // show that the ? operator semantics of auto-converting using From/Into are intact after expansion
    #[derive(Debug, PartialEq)]
    pub enum SomeError {
        ParseIntError,
        Other,
    }

    impl From<ParseIntError> for SomeError {
        fn from(_: ParseIntError) -> Self {
            SomeError::ParseIntError
        }
    }

    fn retry_if(_: &Result<i32, SomeError>) -> bool {
        false
    }

    #[retry(BACKOFF_CONFIG, retry_if)]
    async fn method(int: &str) -> Result<i32, SomeError> {
        return Ok(i32::from_str(int)?);
    }

    let result = method("3").await;
    assert_eq!(Ok(3), result);

    let result = method("notNum").await;
    assert_eq!(Err(SomeError::ParseIntError), result);
}
