use std::time::Duration;

/// Configuration for an exponential backoff, allowing control over the entire strategy.
///
/// This will retry a failing operation up to `max_tries`, waiting for a duration of `t_wait` on the
/// first failure and `t_wait * backoff**attempt` for all remaining attempts. The maximum single wait
/// can be capped by `backoff_max`, and the total waiting time before failing can be set with `t_wait_max`.
///
/// The behavior of `t_wait_max` is such that the function guarantees it will not begin sleeping if
/// sleeping would cause the function to exceed a total execution time of `t_wait_max`. It is
/// however possible for the execution to exceed `t_wait_max` if the decorated code
/// (e.g. calling an API) causes it to exceed this time.
///
///
/// # Example: Classic Exponential Backoff
/// This backoff configuration will retry up to 5 times, waiting 1 second at first, then 2 seconds,
/// 4 seconds, etc.
/// ```
/// # use crate::retry_if::ExponentialBackoffConfig;
/// # use tokio::time::Duration;
///
/// const BACKOFF_CONFIG: ExponentialBackoffConfig = ExponentialBackoffConfig {
///     max_retries: 5,
///     t_wait: Duration::from_secs(1),
///     backoff: 2.0,
///     t_wait_max: None,
///     backoff_max: None,
/// };
/// ```
///
/// # Example: Constant Linear Retries
/// This configuration will retry up to five times, with no exponential behavior, since the backoff
/// exponent is `1.0`. It will wait 1 second for all retry attempts.
/// ```
/// # use crate::retry_if::ExponentialBackoffConfig;
/// # use tokio::time::Duration;
///
/// const BACKOFF_CONFIG: ExponentialBackoffConfig = ExponentialBackoffConfig {
///     max_retries: 5,
///     t_wait: Duration::from_secs(1),
///     backoff: 1.0,
///     t_wait_max: None,
///     backoff_max: None,
/// };
/// ```
///
/// # Example: Backoff With a Maximum Wait Time
/// This backoff configuration will retry up to 25 times, doubling the wait with each retry.
///
/// Setting `t_wait_max` to 10 minutes means that regardless of the number of retries or backoff
/// exponent, it will return in 10 minutes or less.
///
/// ```
/// # use crate::retry_if::ExponentialBackoffConfig;
/// # use tokio::time::Duration;
///
/// const BACKOFF_CONFIG: ExponentialBackoffConfig = ExponentialBackoffConfig {
///     max_retries: 25,
///     t_wait: Duration::from_secs(1),
///     backoff: 2.0,
///     t_wait_max: Some(Duration::from_secs(600)),
///     backoff_max: None,
/// };
/// ```
///
/// # Example: Limited Backoff
/// This backoff configuration will retry up to 15 times, waiting 1 second at first, then 2 seconds,
/// 4 seconds, etc.
///
/// Setting `backoff_max` to 30 seconds ensures it will exponentially back off until reaching a 30s
/// wait time, then will continue to retry every 30s until it succeeds, exhausts retries, or hits
/// `t_wait_max` (not configured in this case).
/// ```
/// # use crate::retry_if::ExponentialBackoffConfig;
/// # use tokio::time::Duration;
///
/// const BACKOFF_CONFIG: ExponentialBackoffConfig = ExponentialBackoffConfig {
///     max_retries: 15,
///     t_wait: Duration::from_secs(1),
///     backoff: 2.0,
///     t_wait_max: None,
///     backoff_max: Some(Duration::from_secs(30)),
/// };
/// ```

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExponentialBackoffConfig {
    /// maximum number of retry attempts to make
    pub max_retries: i32,
    /// initial duration to wait
    pub t_wait: Duration,
    /// backoff exponent, e.g. `2.0` for a classic exponential backoff
    pub backoff: f64,
    /// maximum time to attempt retries before returning the last result
    pub t_wait_max: Option<Duration>,
    /// maximum time to wait for any single retry, i.e. backoff exponentially up to this duration,
    /// then wait in constant time of `backoff_max`
    pub backoff_max: Option<Duration>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "serde")]
    #[test]
    fn test_simple_deserialization() {
        let raw = r#"{"max_retries": 3, "t_wait": {"secs": 5,"nanos": 0}, "backoff": 2}"#;
        let expected_config = ExponentialBackoffConfig {
            max_retries: 3,
            t_wait: Duration::from_secs(5),
            backoff: 2.0,
            t_wait_max: None,
            backoff_max: None,
        };

        let config: ExponentialBackoffConfig = serde_json::from_str(raw).unwrap();

        assert_eq!(expected_config, config);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialization_with_optionals() {
        let raw = r#"{
            "max_retries": 3,
            "t_wait": {"secs": 5,"nanos": 0},
            "backoff": 2,
            "t_wait_max": {"secs": 120,"nanos": 0},
            "backoff_max": {"secs": 15,"nanos": 0}
        }"#;
        let expected_config = ExponentialBackoffConfig {
            max_retries: 3,
            t_wait: Duration::from_secs(5),
            backoff: 2.0,
            t_wait_max: Some(Duration::from_secs(120)),
            backoff_max: Some(Duration::from_secs(15)),
        };

        let config: ExponentialBackoffConfig = serde_json::from_str(raw).unwrap();

        assert_eq!(expected_config, config);
    }
}
