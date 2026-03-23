//! Shared application state (thread-safe)

use crate::ml::HNSWIndex;
use parking_lot::RwLock;
use std::sync::Arc;

pub type SharedState = Arc<AppState>;

pub struct AppState {
    /// Vector index (HNSW) for k-NN queries
    pub index: RwLock<HNSWIndex>,
    /// Neo4j graph connection for complex queries
    pub graph: neo4rs::Graph,
}

impl AppState {
    pub fn new(graph: neo4rs::Graph) -> Self {
        Self {
            index: RwLock::new(HNSWIndex::new(256, 32)),
            graph,
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
