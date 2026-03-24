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
        if vector.len() != self.embedding_dim {
            anyhow::bail!(
                "Vector dimension mismatch: expected {}, got {}",
                self.embedding_dim,
                vector.len()
            );
        }

        self.data.insert(id, vector);
        Ok(())
    }

    /// Query k nearest neighbors using brute-force search
    pub fn search(&self, query: &[f32], k: usize) -> Result<Vec<(String, f32)>> {
        if query.len() != self.embedding_dim {
            anyhow::bail!(
                "Query dimension mismatch: expected {}, got {}",
                self.embedding_dim,
                query.len()
            );
        }

        let mut results: Vec<(String, f32)> = self
            .data
            .iter()
            .map(|(id, vector)| {
                let similarity = Self::cosine_similarity(query, vector);
                (id.clone(), similarity)
            })
            .collect();

        // Sort by similarity (descending)
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Return top k
        results.truncate(k);
        Ok(results)
    }

    /// Cosine similarity between two vectors
    pub fn cosine_similarity(vec_a: &[f32], vec_b: &[f32]) -> f32 {
        let dot_product: f32 = vec_a.iter().zip(vec_b.iter()).map(|(a, b)| a * b).sum();

        let magnitude_a: f32 = vec_a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let magnitude_b: f32 = vec_b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if magnitude_a == 0.0 || magnitude_b == 0.0 {
            return 0.0;
        }

        dot_product / (magnitude_a * magnitude_b)
    }

    /// Get vector for entity
    pub fn get(&self, id: &str) -> Option<&Vec<f32>> {
        self.data.get(id)
    }

    /// Size of index
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
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

    #[test]
    fn test_insert_and_get() {
        let mut index = HNSWIndex::new(3, 32);
        let vec = vec![1.0, 0.0, 0.0];

        index.insert("anime1".to_string(), vec.clone()).unwrap();

        assert_eq!(index.len(), 1);
        assert_eq!(index.get("anime1"), Some(&vec));
    }

    #[test]
    fn test_cosine_similarity_identical() {
        let vec = vec![1.0, 0.0, 0.0];
        let sim = HNSWIndex::cosine_similarity(&vec, &vec);
        assert!((sim - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let vec_a = vec![1.0, 0.0, 0.0];
        let vec_b = vec![0.0, 1.0, 0.0];
        let sim = HNSWIndex::cosine_similarity(&vec_a, &vec_b);
        assert!(sim.abs() < 0.001);
    }

    #[test]
    fn test_search_returns_k_results() {
        let mut index = HNSWIndex::new(3, 32);

        index
            .insert("anime1".to_string(), vec![1.0, 0.0, 0.0])
            .unwrap();
        index
            .insert("anime2".to_string(), vec![0.9, 0.1, 0.0])
            .unwrap();
        index
            .insert("anime3".to_string(), vec![0.0, 1.0, 0.0])
            .unwrap();

        let results = index.search(&vec![1.0, 0.0, 0.0], 2).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, "anime1");
    }
}
