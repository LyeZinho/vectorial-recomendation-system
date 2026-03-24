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
    pub fn train(&self, walks: Vec<Vec<String>>) -> Result<HashMap<String, Vec<f32>>> {
        // Step 1: Build vocabulary
        let mut vocab = HashMap::new();
        let mut vocab_idx = 0;

        for walk in &walks {
            for token in walk {
                vocab.entry(token.clone()).or_insert_with(|| {
                    let idx = vocab_idx;
                    vocab_idx += 1;
                    idx
                });
            }
        }

        let vocab_size = vocab.len();

        // Step 2: Initialize embeddings (random normal)
        let mut embeddings: Vec<Vec<f32>> = (0..vocab_size)
            .map(|_| {
                (0..self.embedding_dim)
                    .map(|_| rand::random::<f32>() - 0.5)
                    .collect()
            })
            .collect();

        // Step 3: Training loop (gradient descent)
        let mut learning_rate = self.learning_rate;

        for _epoch in 0..self.num_epochs {
            for walk in &walks {
                for center_pos in 0..walk.len() {
                    let center_word = &walk[center_pos];
                    let center_idx = vocab[center_word];

                    // Context window
                    let start = center_pos.saturating_sub(self.window_size);
                    let end = (center_pos + self.window_size + 1).min(walk.len());

                    for context_pos in start..end {
                        if context_pos == center_pos {
                            continue;
                        }

                        let context_word = &walk[context_pos];
                        let context_idx = vocab[context_word];

                        // Gradient descent: update context embedding
                        let center_vec = embeddings[center_idx].clone();
                        let context_vec = embeddings[context_idx].clone();

                        let dot_product: f32 = center_vec
                            .iter()
                            .zip(context_vec.iter())
                            .map(|(a, b)| a * b)
                            .sum();

                        // Simplified gradient (full softmax would be too expensive)
                        let sigmoid_val = 1.0 / (1.0 + (-dot_product).exp());
                        let gradient_scale = (sigmoid_val - 1.0) * learning_rate;

                        for i in 0..self.embedding_dim {
                            embeddings[context_idx][i] -= gradient_scale * center_vec[i];
                        }
                    }
                }
            }

            learning_rate *= 0.9; // Decay learning rate
        }

        // Step 4: Return embeddings as HashMap
        let mut result = HashMap::new();

        for (token, idx) in vocab {
            result.insert(token, embeddings[idx].clone());
        }

        Ok(result)
    }

    /// Mock training for unit tests (no actual learning)
    pub fn train_mock(&self, walks: Vec<Vec<String>>) -> Result<HashMap<String, Vec<f32>>> {
        let mut result = HashMap::new();

        for walk in walks {
            for token in walk {
                result.entry(token).or_insert_with(|| {
                    (0..self.embedding_dim)
                        .map(|_| rand::random::<f32>() - 0.5)
                        .collect()
                });
            }
        }

        Ok(result)
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
