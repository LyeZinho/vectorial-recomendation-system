//! Skip-gram Word2Vec training on random walks

use anyhow::Result;
use std::collections::HashMap;

pub struct SkipGramTrainer {
    pub embedding_dim: usize,
    pub learning_rate: f32,
    pub window_size: usize,
    pub num_epochs: usize,
}

impl SkipGramTrainer {
    pub fn new(
        embedding_dim: usize,
        learning_rate: f32,
        window_size: usize,
        num_epochs: usize,
    ) -> Self {
        Self {
            embedding_dim,
            learning_rate,
            window_size,
            num_epochs,
        }
    }

    /// Train Skip-gram model on walks corpus
    pub fn train(&self, _walks: Vec<Vec<String>>) -> Result<HashMap<String, Vec<f32>>> {
        // Implementation in Task 5
        Ok(HashMap::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_gram_creation() {
        let trainer = SkipGramTrainer::new(256, 0.025, 5, 5);
        assert_eq!(trainer.embedding_dim, 256);
    }
}
