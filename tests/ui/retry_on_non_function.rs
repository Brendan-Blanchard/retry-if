use retry_if::{retry, ExponentialBackoffConfig};
use std::time::Duration;

const BACKOFF_CONFIG: ExponentialBackoffConfig = ExponentialBackoffConfig {
    max_retries: 5,
    t_wait: Duration::from_secs(1),
    backoff: 2.0,
    t_wait_max: None,
    backoff_max: None,
};

fn retry_if(_: ()) -> bool {
    true
}

#[retry(BACKOFF_CONFIG, retry_if)]
pub trait NotAFunction {
    fn some_function(&mut self);
}

fn main() {}
