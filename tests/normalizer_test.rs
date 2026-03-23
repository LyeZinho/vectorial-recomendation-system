use anime_harvester::normalizer::CanonicalEntity;
use uuid::Uuid;

#[test]
fn test_canonical_entity_anime_creation() {
    let anime = CanonicalEntity::Anime {
        internal_id: Uuid::new_v4(),
        title: "FMA: Brotherhood".to_string(),
        score: Some(9.09),
        episodes: Some(64),
        genres: vec!["Action".to_string(), "Adventure".to_string()],
        studios: vec!["Bones".to_string()],
        staff: vec![],
        external_ids: vec![("mal_id".to_string(), "5114".to_string())],
    };

    match anime {
        CanonicalEntity::Anime { title, score, .. } => {
            assert_eq!(title, "FMA: Brotherhood");
            assert_eq!(score, Some(9.09));
        }
        _ => panic!("Expected Anime variant"),
    }
}

#[test]
fn test_canonical_entity_user_anime_list() {
    let entry = CanonicalEntity::UserAnimeEntry {
        internal_id: Uuid::new_v4(),
        user_id: Uuid::new_v4(),
        anime_id: Uuid::new_v4(),
        score: Some(9),
        status: "completed".to_string(),
    };

    match entry {
        CanonicalEntity::UserAnimeEntry { score, status, .. } => {
            assert_eq!(score, Some(9));
            assert_eq!(status, "completed");
        }
        _ => panic!("Expected UserAnimeEntry variant"),
    }
}
