//! This example tests a backoff configuration with a dynamic `retry_if` function that will
//! return false after 2 attempts.
//!
//! The expectation is that two retries will take 1s, 3s, for a total of 4s of execution time. No
//! other conditions will be triggered.
use retry_if::{retry, ExponentialBackoffConfig};
use std::time::Duration;
use tokio::time::{pause, Instant};

const BACKOFF_CONFIG: ExponentialBackoffConfig = ExponentialBackoffConfig {
    max_retries: 5,
    t_wait: Duration::from_secs(1),
    backoff: 3.0,
    t_wait_max: None,
    backoff_max: None,
};

fn retry_if(i: i64) -> bool {
    i < 3
}

pub struct Counter {
    pub count: i64,
}

impl Counter {
    #[retry(BACKOFF_CONFIG, retry_if)]
    async fn increase_count(&mut self) -> i64 {
        self.count += 1;
        self.count
    }
}

#[tokio::test]
async fn main() {
    let mut counter = Counter { count: 0 };

    pause();
    let start = Instant::now();
    counter.increase_count().await;
    let end = Instant::now();
    let duration = end - start;

    // waits of 1s, 3s = 4s
    assert!(duration > Duration::from_secs(4));
    assert!(duration < Duration::from_millis(4100));
    // initial attempt + 2 retries
    assert_eq!(3, counter.count);
}
