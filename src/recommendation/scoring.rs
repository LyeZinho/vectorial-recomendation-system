//! Final score calculation and deduplication

use std::collections::HashMap;

pub struct ScoringEngine;

impl ScoringEngine {
    /// Merge multiple recommendation layers and deduplicate
    /// Takes Vec of (anime_id, score, layer_name)
    /// Returns sorted Vec of top results with highest score per anime
    pub fn merge_and_deduplicate(
        layer_results: Vec<Vec<(String, f32, String)>>,
        k: usize,
    ) -> Vec<(String, f32, String)> {
        let mut best_scores: HashMap<String, (f32, String)> = HashMap::new();

        for layer in layer_results {
            for (anime_id, score, layer_name) in layer {
                let entry = best_scores
                    .entry(anime_id)
                    .or_insert((score, layer_name.clone()));
                if score > entry.0 {
                    entry.0 = score;
                    entry.1 = layer_name;
                }
            }
        }

        let mut result: Vec<(String, f32, String)> = best_scores
            .into_iter()
            .map(|(id, (score, layer))| (id, score, layer))
            .collect();

        result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        result.truncate(k);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_and_deduplicate() {
        let layer1 = vec![
            ("anime1".to_string(), 0.9, "technical_dna".to_string()),
            ("anime2".to_string(), 0.8, "technical_dna".to_string()),
        ];
        let layer2 = vec![
            ("anime1".to_string(), 0.7, "niche_clustering".to_string()),
            ("anime3".to_string(), 0.85, "niche_clustering".to_string()),
        ];

        let result = ScoringEngine::merge_and_deduplicate(vec![layer1, layer2], 3);

        // anime1 should keep score 0.9 from layer1 (higher)
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].0, "anime1"); // Highest score
        assert_eq!(result[0].1, 0.9);
    }
}
