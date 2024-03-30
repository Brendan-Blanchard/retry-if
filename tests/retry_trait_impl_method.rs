//! This example tests a backoff configuration with a maximum increase in the backoff of 2.5s, and
//! and overall execution time maximum of 10s.
//!
//! The expectation is that retries will take 1s, 2s, 2.5s, 2.5s for a total of 8s of execution time.
//! This is because at 8s, it's clear that another wait of 2.5s would exceed the maximum time of 10s,
//! so it exits early.
use retry_if::{retry, ExponentialBackoffConfig};
use std::time::Duration;
use tokio::time::{pause, Instant};

const BACKOFF_CONFIG: ExponentialBackoffConfig = ExponentialBackoffConfig {
    max_retries: 25,
    t_wait: Duration::from_secs(1),
    backoff: 2.0,
    t_wait_max: Some(Duration::from_secs(10)),
    backoff_max: Some(Duration::from_millis(2500)),
};

fn should_retry(_i: i64) -> bool {
    true
}

trait Counter {
    async fn increase_count(&mut self) -> i64;
}

pub struct SimpleCounter {
    pub count: i64,
}

impl Counter for SimpleCounter {
    #[retry(BACKOFF_CONFIG, should_retry)]
    async fn increase_count(&mut self) -> i64 {
        self.count += 1;
        self.count
    }
}

#[tokio::test]
async fn main() {
    let mut counter = SimpleCounter { count: 0 };

    pause();
    let start = Instant::now();
    counter.increase_count().await;
    let end = Instant::now();
    let duration = end - start;

    // waits of 1s, 2s, 2.5s, 2.5s, at 8s waiting another 2.5s would exceed time, so it exits early
    assert!(duration > Duration::from_secs(8));
    assert!(duration < Duration::from_millis(8100));
    // initial attempt + 4 retries
    assert_eq!(5, counter.count);
}
