use anime_harvester::recommendation::apply_anti_hype_multiplier;
use anime_harvester::recommendation::epsilon_greedy_select;
use anime_harvester::recommendation::ScoringEngine;

#[test]
fn test_scoring_engine_deduplication() {
    let layer1 = vec![
        ("anime1".to_string(), 0.9, "technical_dna".to_string()),
        ("anime2".to_string(), 0.8, "technical_dna".to_string()),
    ];
    let layer2 = vec![
        ("anime1".to_string(), 0.7, "niche_clustering".to_string()),
        ("anime3".to_string(), 0.75, "niche_clustering".to_string()),
    ];

    let result = ScoringEngine::merge_and_deduplicate(vec![layer1, layer2], 10);

    assert_eq!(result.len(), 3);
    assert_eq!(result[0].0, "anime1");
    assert_eq!(result[0].1, 0.9);
}

#[test]
fn test_recommendation_filtering_with_epsilon_greedy() {
    let scored = vec![
        ("anime1".to_string(), 0.95),
        ("anime2".to_string(), 0.88),
        ("anime3".to_string(), 0.82),
        ("anime4".to_string(), 0.75),
        ("anime5".to_string(), 0.68),
    ];

    let result = epsilon_greedy_select(scored, 3, 0.2);
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].0, "anime1");
}

#[test]
fn test_anti_hype_multiplier_effect() {
    let obscure = apply_anti_hype_multiplier("anime1", 0.7, 0.1);
    let popular = apply_anti_hype_multiplier("anime2", 0.7, 0.9);

    assert!(obscure > popular);
}

#[test]
fn test_full_recommendation_flow_integration() {
    let tech_dna_layer = vec![
        ("anime_tech1".to_string(), 0.92, "technical_dna".to_string()),
        ("anime_tech2".to_string(), 0.85, "technical_dna".to_string()),
    ];

    let niche_layer = vec![
        (
            "anime_niche1".to_string(),
            0.88,
            "niche_clustering".to_string(),
        ),
        (
            "anime_niche2".to_string(),
            0.81,
            "niche_clustering".to_string(),
        ),
    ];

    let influence_layer = vec![(
        "anime_inf1".to_string(),
        0.79,
        "influence_chain".to_string(),
    )];

    let underground_layer = vec![(
        "anime_under1".to_string(),
        0.75,
        "underground_discovery".to_string(),
    )];

    let all_layers = vec![
        tech_dna_layer,
        niche_layer,
        influence_layer,
        underground_layer,
    ];

    let merged = ScoringEngine::merge_and_deduplicate(all_layers, 5);

    assert!(merged.len() <= 5);
    assert!(merged.len() > 0);
    assert_eq!(merged[0].1, 0.92);
}
