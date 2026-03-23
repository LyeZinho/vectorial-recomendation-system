//! HNSW (Hierarchical Navigable Small World) vector index

use anyhow::Result;
use std::collections::HashMap;

pub struct HNSWIndex {
    pub max_neighbors: usize,
    pub embedding_dim: usize,
    pub data: HashMap<String, Vec<f32>>,
}

impl HNSWIndex {
    pub fn new(embedding_dim: usize, max_neighbors: usize) -> Self {
        Self {
            embedding_dim,
            max_neighbors,
            data: HashMap::new(),
        }
    }

    /// Insert vector into index
    pub fn insert(&mut self, id: String, vector: Vec<f32>) -> Result<()> {
        // Implementation in Task 6
        self.data.insert(id, vector);
        Ok(())
    }

    /// Query k nearest neighbors
    pub fn search(&self, _query: Vec<f32>, _k: usize) -> Result<Vec<(String, f32)>> {
        // Implementation in Task 6
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hnsw_creation() {
        let index = HNSWIndex::new(256, 32);
        assert_eq!(index.embedding_dim, 256);
        assert_eq!(index.max_neighbors, 32);
    }
}
