//! Recommendation filters: epsilon-greedy exploration, anti-hype scoring

use rand::Rng;

/// Epsilon-greedy exploration: 90% exploitation, 10% exploration
pub fn epsilon_greedy_select(
    scored_recommendations: Vec<(String, f32)>,
    k: usize,
    epsilon: f32,
) -> Vec<(String, f32)> {
    let mut rng = rand::thread_rng();

    // Sort by score descending
    let mut sorted = scored_recommendations;
    sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Take top k * (1-epsilon) from exploitation
    let exploitation_count = ((k as f32) * (1.0 - epsilon)).ceil() as usize;
    let mut result: Vec<(String, f32)> = sorted.iter().take(exploitation_count).cloned().collect();

    // Add random k * epsilon from exploration
    let exploration_count = k - result.len();
    let exploration_pool: Vec<(String, f32)> =
        sorted.iter().skip(exploitation_count).cloned().collect();

    for _ in 0..exploration_count {
        if exploration_pool.is_empty() {
            break;
        }
        let idx = rng.gen_range(0..exploration_pool.len());
        result.push(exploration_pool[idx].clone());
    }

    result.truncate(k);
    result
}

/// Anti-hype multiplier: boost low-popularity high-quality anime
pub fn apply_anti_hype_multiplier(
    _anime_id: &str,
    base_score: f32,
    popularity: f32, // 1.0 = very popular, 0.0 = obscure
) -> f32 {
    // final_score = similarity * (1 + 1/log10(popularity+2))
    let popularity_penalty = 1.0 + (1.0 / ((popularity + 2.0).log10() + 0.001));
    base_score * popularity_penalty
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epsilon_greedy_selection() {
        let scored = vec![
            ("anime1".to_string(), 0.9),
            ("anime2".to_string(), 0.8),
            ("anime3".to_string(), 0.7),
            ("anime4".to_string(), 0.6),
        ];

        let result = epsilon_greedy_select(scored, 2, 0.1);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_anti_hype_multiplier() {
        let base_score = 1.0;
        let unpopular_score = apply_anti_hype_multiplier("anime1", base_score, 0.1);
        let popular_score = apply_anti_hype_multiplier("anime2", base_score, 0.9);

        // Unpopular anime should get a larger multiplier
        assert!(unpopular_score > popular_score);
    }
}
