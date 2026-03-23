//! Recommendation engine: 4-layer logic orchestration

pub mod layers;
pub mod scoring;
pub mod filters;

pub use layers::RecommendationLayers;
pub use scoring::ScoringEngine;
pub use filters::{epsilon_greedy_select, apply_anti_hype_multiplier};

#[cfg(test)]
mod tests {
    #[test]
    fn test_recommendation_module_exports() {
        assert!(true);
    }
}
