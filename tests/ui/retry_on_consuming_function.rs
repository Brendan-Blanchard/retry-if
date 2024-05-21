use retry_if::{retry, ExponentialBackoffConfig};
use std::time::Duration;

const BACKOFF_CONFIG: ExponentialBackoffConfig = ExponentialBackoffConfig {
    max_retries: 5,
    t_wait: Duration::from_secs(1),
    backoff: 2.0,
    t_wait_max: None,
    backoff_max: None,
};

fn retry_if(_i: &NewCounter) -> bool {
    true
}

pub struct Counter {
    pub count: i64,
}

pub struct NewCounter {
    pub count: i64,
}

impl From<Counter> for NewCounter {
    fn from(value: Counter) -> Self {
        NewCounter {
            count: value.count,
        }
    }
}

impl Counter {
    #[retry(BACKOFF_CONFIG, retry_if)]
    async fn consume_self(self) -> NewCounter {
        self.into()
    }
}

#[tokio::main]
async fn main() {
    let counter = Counter { count: 0 };
    let _new_counter = counter.consume_self().await;
}