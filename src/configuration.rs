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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    #[tokio::test]
    async fn test() {
        let duration = Duration::from_secs(2);

        let new = duration.mul_f64(2.5);

        println!("{:?}", new);

        tokio::time::sleep(new).await;
    }

    #[tokio::test]
    async fn test_pow() {
        let wait = Duration::from_secs(2);
        let wait_max = Some(Duration::from_secs(3));
        let backoff: f64 = 2.0;

        for attempt in 0..3 {
            let retry_wait = wait.mul_f64(backoff.powi(attempt));
            println!("{:?}", retry_wait.min(wait_max.unwrap_or(Duration::MAX)));
        }
    }
}
