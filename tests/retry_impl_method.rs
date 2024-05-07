//! This example tests a backoff configuration with no maximum on individual waits, no overall wait
//! maximum, and a max of 5 retries that causes it to exit.
//!
//! The expectation is that 5 retries will take 1s, 2s, 4s, 8s, and 16s for a total of 31s of
//! execution time, and 6 increments of the counter.
use retry_if::{retry, ExponentialBackoffConfig};
use std::time::Duration;
use tokio::time::{pause, Instant};

const BACKOFF_CONFIG: ExponentialBackoffConfig = ExponentialBackoffConfig {
    max_retries: 5,
    t_wait: Duration::from_secs(1),
    backoff: 2.0,
    t_wait_max: None,
    backoff_max: None,
};

fn retry_if(_i: &i64) -> bool {
    true
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

    // max of 5 retries, waits of 1s, 2s, 4s, 8s, 16s = 31s
    println!("{:?}", duration);
    assert!(duration > Duration::from_secs(31));
    assert!(duration < Duration::from_millis(31100));
    // initial attempt + 5 retries
    assert_eq!(5, counter.count);
}
