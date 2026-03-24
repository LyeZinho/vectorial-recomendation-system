use governor::{
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use std::num::NonZeroU32;
use std::sync::Arc;

type DirectRateLimiter = RateLimiter<
    governor::state::NotKeyed,
    governor::state::InMemoryState,
    governor::clock::DefaultClock,
>;

pub struct RateLimitConfig {
    pub requests_per_second: u32,
}

pub struct RateLimitManager {
    limiter: Arc<DirectRateLimiter>,
}

impl Clone for RateLimitManager {
    fn clone(&self) -> Self {
        Self {
            limiter: self.limiter.clone(),
        }
    }
}

impl RateLimitManager {
    pub fn new(config: RateLimitConfig) -> Self {
        let quota = Quota::per_second(NonZeroU32::new(config.requests_per_second).unwrap());
        let limiter = RateLimiter::direct(quota);

        Self {
            limiter: Arc::new(limiter),
        }
    }

    pub fn check_limit(&self) -> Result<(), ()> {
        self.limiter.check().map_err(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_allows_requests_within_quota() {
        let manager = RateLimitManager::new(RateLimitConfig {
            requests_per_second: 10,
        });

        for _ in 0..10 {
            assert!(manager.check_limit().is_ok());
        }

        assert!(manager.check_limit().is_err());
    }
}
