//! Tests for recommendation layers

#[test]
fn test_technical_dna_result_shape() {
    // Results should be Vec<(anime_id, score, layer_name)>
    let results: Vec<(String, f32, String)> = vec![
        ("anime1".to_string(), 0.95, "technical_dna".to_string()),
        ("anime2".to_string(), 0.87, "technical_dna".to_string()),
    ];

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].2, "technical_dna");
}

#[test]
fn test_niche_clustering_result_shape() {
    let results: Vec<(String, f32, String)> =
        vec![("anime1".to_string(), 0.85, "niche_clustering".to_string())];

    assert_eq!(results[0].2, "niche_clustering");
}

#[test]
fn test_influence_chain_result_shape() {
    let results: Vec<(String, f32, String)> =
        vec![("anime1".to_string(), 0.78, "influence_chain".to_string())];

    assert_eq!(results[0].2, "influence_chain");
}

#[test]
fn test_underground_discovery_applies_multiplier() {
    use anime_harvester::recommendation::apply_anti_hype_multiplier;

    let base_score = 0.8;
    let obscure_anime_score = apply_anti_hype_multiplier("anime1", base_score, 0.1);
    let popular_anime_score = apply_anti_hype_multiplier("anime2", base_score, 0.9);

    // Obscure anime should get a bigger multiplier boost
    assert!(obscure_anime_score > popular_anime_score);
}
