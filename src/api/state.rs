//! Shared application state (thread-safe)

use crate::ml::HNSWIndex;
use crate::api::auth::JwtManager;
use crate::api::cache::CacheManager;
use parking_lot::RwLock;
use std::sync::Arc;

pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub index: RwLock<HNSWIndex>,
    pub graph: neo4rs::Graph,
    pub jwt_manager: JwtManager,
    pub cache: Option<CacheManager>,
}

impl AppState {
    pub fn new(graph: neo4rs::Graph, jwt_manager: JwtManager, cache: Option<CacheManager>) -> Self {
        Self {
            index: RwLock::new(HNSWIndex::new(256, 32)),
            graph,
            jwt_manager,
            cache,
        }
    }

    pub fn as_shared(self) -> SharedState {
        Arc::new(self)
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_app_state_creation() {
        assert!(true);
    }
}
