use governor::{
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use std::collections::HashMap;
use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};

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
    per_user_limiters: Arc<Mutex<HashMap<String, Arc<DirectRateLimiter>>>>,
    config: RateLimitConfig,
}

impl Clone for RateLimitManager {
    fn clone(&self) -> Self {
        Self {
            limiter: self.limiter.clone(),
            per_user_limiters: self.per_user_limiters.clone(),
            config: RateLimitConfig {
                requests_per_second: self.config.requests_per_second,
            },
        }
    }
}

impl RateLimitManager {
    pub fn new(config: RateLimitConfig) -> Self {
        let quota = Quota::per_second(NonZeroU32::new(config.requests_per_second).unwrap());
        let limiter = RateLimiter::direct(quota);

        Self {
            limiter: Arc::new(limiter),
            per_user_limiters: Arc::new(Mutex::new(HashMap::new())),
            config,
        }
    }

    pub fn check_limit(&self) -> Result<(), ()> {
        self.limiter.check().map_err(|_| ())
    }

    pub fn check_limit_for_user(&self, user_id: &str) -> Result<(), ()> {
        let mut limiters = self.per_user_limiters.lock().unwrap();

        let limiter = limiters.entry(user_id.to_string()).or_insert_with(|| {
            let quota =
                Quota::per_second(NonZeroU32::new(self.config.requests_per_second).unwrap());
            Arc::new(RateLimiter::direct(quota))
        });

        limiter.check().map_err(|_| ())
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

    #[test]
    fn test_per_user_rate_limiting() {
        let manager = RateLimitManager::new(RateLimitConfig {
            requests_per_second: 5,
        });

        for _ in 0..5 {
            assert!(manager.check_limit_for_user("user1").is_ok());
        }

        assert!(manager.check_limit_for_user("user1").is_err());

        for _ in 0..5 {
            assert!(manager.check_limit_for_user("user2").is_ok());
        }

        assert!(manager.check_limit_for_user("user2").is_err());
    }
}
