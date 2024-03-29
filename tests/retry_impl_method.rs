use retrys::{retry, ExponentialBackoffConfig};
use std::time::Duration;
use tokio::time::{pause, Instant};

const BACKOFF_CONFIG: ExponentialBackoffConfig = ExponentialBackoffConfig {
    max_retries: 5,
    t_wait: Duration::from_secs(1),
    t_wait_max: None,
    backoff: 2.0,
    backoff_max: None,
};

fn should_retry(_i: i64) -> bool {
    true
}

pub struct Counter {
    pub count: i64,
}

impl Counter {
    #[retry(BACKOFF_CONFIG, should_retry)]
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
    assert!(duration > Duration::from_secs(31));
    assert!(duration < Duration::from_millis(31100));
    // initial attempt + 5 retries
    assert_eq!(6, counter.count);
}
