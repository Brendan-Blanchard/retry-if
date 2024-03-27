pub use retrys_macro::retry;

#[cfg(test)]
mod tests {
    use super::*;

    trait AsyncTrait {
        async fn async_method(&self);

        async fn async_fn();
    }

    struct AsyncTraitImpl {}

    impl AsyncTrait for AsyncTraitImpl {
        #[retry(Ok)]
        async fn async_method(&self) {}

        #[retry(Ok)]
        async fn async_fn() {}
    }

    #[tokio::test]
    async fn test_basic_async_function() {
        #[retry(Error)]
        async fn test_fn() {}

        test_fn().await;
    }

    #[tokio::test]
    async fn test_trait_async_fn() {
        let trait_impl = AsyncTraitImpl {};

        trait_impl.async_method().await;

        AsyncTraitImpl::async_fn().await;
    }
}