//! This example tests a backoff configuration with an exponent of 1.0, thus a linear backoff.
//!
//! A max of 5 tries should take 15s in total.
use retry_if::{retry, ExponentialBackoffConfig};
use std::time::Duration;
use tokio::time::{pause, Instant};

const BACKOFF_CONFIG: ExponentialBackoffConfig = ExponentialBackoffConfig {
    max_retries: 5,
    t_wait: Duration::from_secs(3),
    backoff: 1.0,
    t_wait_max: None,
    backoff_max: None,
};

#[tokio::test]
async fn test_should_retry() {
    pause();
    fn should_retry(_: ()) -> bool {
        true
    }

    #[retry(BACKOFF_CONFIG, should_retry)]
    async fn method() {}

    let start = Instant::now();
    method().await;
    let end = Instant::now();
    let duration = end - start;

    assert!(duration > Duration::from_secs(15));
    assert!(duration < Duration::from_millis(15100));
}
