use std::time::Duration;

/// Configuration for an exponential backoff, allowing control over the entire strategy.
///
/// This will retry a failing operation up to `max_tries`, waiting for a duration of `t_wait` on the
/// first failure and `t_wait * backoff^attempt` for all remaining attempts. The maximum single wait
/// can be capped by `backoff_max`, and the total waiting time before failing can be set with `t_wait_max`.
///
/// TODO: example w/ series of waits
pub struct ExponentialBackoffConfig {
    pub max_retries: i32,
    pub t_wait: Duration,
    pub backoff: f64,
    pub t_wait_max: Option<Duration>,
    pub backoff_max: Option<Duration>,
}
