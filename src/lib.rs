mod configuration;

pub use retrys_macro::retry;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::ExponentialBackoffConfig;
    use std::time::Duration;

    const BACKOFF_CONFIG: ExponentialBackoffConfig = ExponentialBackoffConfig {
        max_tries: 5,
        t_wait: Duration::from_secs(1),
        t_wait_max: Duration::from_secs(10),
        backoff: 2.0,
        backoff_max: Some(Duration::from_secs(3)),
    };

    fn should_retry(_result: i32) -> bool {
        true
    }

    trait AsyncTrait {
        async fn async_method(&self) -> i32;

        async fn async_fn() -> i32;
    }

    struct AsyncTraitImpl {}

    impl AsyncTrait for AsyncTraitImpl {
        #[retry(BACKOFF_CONFIG, should_retry)]
        async fn async_method(&self) -> i32 {
            println!("Running...");
            0
        }

        #[retry(BACKOFF_CONFIG, should_retry)]
        async fn async_fn() -> i32 {
            println!("Running...");
            1
        }
    }

    #[tokio::test]
    async fn test_basic_async_function() {
        #[retry(BACKOFF_CONFIG, should_retry)]
        async fn test_fn() -> i32 {
            println!("Running...");
            42
        }

        assert_eq!(42, test_fn().await);
    }

    #[tokio::test]
    async fn test_trait_async_fn() {
        let trait_impl = AsyncTraitImpl {};

        trait_impl.async_method().await;

        AsyncTraitImpl::async_fn().await;
    }
}
