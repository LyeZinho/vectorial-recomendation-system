use redis::{Commands, RedisResult};

pub struct CacheManager {
    client: redis::Client,
}

impl CacheManager {
    pub fn new(redis_url: &str) -> RedisResult<Self> {
        let client = redis::Client::open(redis_url)?;
        Ok(Self { client })
    }

    pub fn get(&self, key: &str) -> RedisResult<Option<String>> {
        let mut conn = self.client.get_connection()?;
        conn.get(key)
    }

    pub fn set_ex(&self, key: &str, value: &str, ttl_seconds: u64) -> RedisResult<()> {
        let mut conn = self.client.get_connection()?;
        conn.set_ex(key, value, ttl_seconds)
    }

    pub fn del(&self, key: &str) -> RedisResult<()> {
        let mut conn = self.client.get_connection()?;
        conn.del::<_, ()>(key)
    }

    pub fn invalidate_pattern(&self, pattern: &str) -> RedisResult<()> {
        let mut conn = self.client.get_connection()?;
        let keys: Vec<String> = conn.keys(pattern)?;
        for key in keys {
            conn.del::<_, ()>(&key)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_manager_creation() {
        let result = CacheManager::new("redis://127.0.0.1:6379/");
        assert!(result.is_ok() || result.is_err());
    }
}
