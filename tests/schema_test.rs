use anime_harvester::schema::{EntityType, SchemaInferrer};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_infer_schema_detects_anime_core() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();

    let csv_content = "Title,Score,Episodes,Studio,Genres\nFMA,9.1,64,Bones,Action|Adventure";
    fs::write(root.join("test.csv"), csv_content).unwrap();

    let inferrer = SchemaInferrer::new();
    let schema = inferrer
        .infer_schema(root.join("test.csv").to_str().unwrap())
        .unwrap();

    assert_eq!(schema.entity_type, EntityType::AnimeCore);
    assert!(schema.confidence > 0.5, "Confidence should be > 0.5");
}

#[test]
fn test_infer_schema_detects_user_anime_list() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();

    let csv_content =
        "user_id,anime_id,my_score,my_status,my_watched_episodes\n123,456,9,completed,12";
    fs::write(root.join("test.csv"), csv_content).unwrap();

    let inferrer = SchemaInferrer::new();
    let schema = inferrer
        .infer_schema(root.join("test.csv").to_str().unwrap())
        .unwrap();

    assert_eq!(schema.entity_type, EntityType::UserAnimeList);
}
