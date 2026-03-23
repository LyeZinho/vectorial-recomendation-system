use anime_harvester::scanner::FileScanner;
use anime_harvester::schema::SchemaInferrer;
use anime_harvester::storage::SqliteStorage;
use tempfile::TempDir;

#[test]
fn test_full_pipeline_on_real_datapool() {
    // Scan real /datapool
    let files = FileScanner::scan("/home/pedro/repo/vectorial-recomendation-system/datapool")
        .expect("Should scan datapool");

    println!("Found {} CSV files", files.len());
    assert!(files.len() > 0, "Should find CSV files in datapool");

    // Infer schema on first few files
    let inferrer = SchemaInferrer::new();
    let mut schema_count = 0;

    for file in files.iter().take(5) {
        match inferrer.infer_schema(file) {
            Ok(schema) => {
                println!(
                    "{}: {} (confidence: {:.2})",
                    std::path::Path::new(file)
                        .file_name()
                        .unwrap()
                        .to_string_lossy(),
                    schema.entity_type,
                    schema.confidence
                );
                schema_count += 1;
            }
            Err(e) => {
                eprintln!("Failed to infer schema for {}: {}", file, e);
            }
        }
    }

    assert!(schema_count > 0, "Should infer schema on at least one file");
}

#[test]
fn test_sqlite_storage_persistence() {
    let temp = TempDir::new().unwrap();
    let db_path = temp.path().join("harvest.db");

    // Create storage and insert data
    {
        let storage = SqliteStorage::new(db_path.to_str().unwrap()).unwrap();
        let id = uuid::Uuid::new_v4();
        storage.insert_entity(&id, "Anime", "Test Anime").unwrap();
        storage.insert_external_id(&id, "mal_id", "5114").unwrap();
        storage
            .insert_harvest_log("test.csv", "AnimeCore", 0.95, 100)
            .unwrap();
    }

    // Verify data persists
    let storage = SqliteStorage::new(db_path.to_str().unwrap()).unwrap();
    assert!(storage.table_exists("entities").unwrap());
    assert!(storage.table_exists("harvest_log").unwrap());
}

#[test]
fn test_cli_help_runs() {
    let output = std::process::Command::new(
        "/home/pedro/repo/vectorial-recomendation-system/.worktrees/harvester-phase1/target/debug/anime-harvester",
    )
    .arg("--help")
    .output()
    .expect("Failed to run CLI");

    assert!(output.status.success(), "CLI help should succeed");
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    let help_text = format!("{}{}", stdout, stderr);
    assert!(help_text.contains("harvest") || help_text.contains("Harvest"));
}
