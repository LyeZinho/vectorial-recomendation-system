//! ML pipeline: random walks → embeddings → HNSW index

pub mod random_walk;
pub mod skip_gram;
pub mod vector_index;

pub use random_walk::RandomWalkGenerator;
pub use skip_gram::SkipGramTrainer;
pub use vector_index::HNSWIndex;

use anyhow::Result;
use std::collections::HashMap;

/// ML orchestration: coordinate walk generation → training → indexing
pub struct MLPipeline {
    pub embeddings: HashMap<String, Vec<f32>>,
    pub index: Option<HNSWIndex>,
}

impl MLPipeline {
    pub fn new() -> Self {
        Self {
            embeddings: HashMap::new(),
            index: None,
        }
    }

    /// Placeholder for full pipeline orchestration
    pub fn train(&mut self) -> Result<()> {
        // Will be populated in later tasks
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_creation() {
        let pipeline = MLPipeline::new();
        assert!(pipeline.embeddings.is_empty());
    }
}
