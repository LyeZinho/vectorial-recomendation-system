use anime_harvester::scanner::FileScanner;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_scan_finds_csv_files() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();

    fs::create_dir_all(root.join("archive")).unwrap();
    fs::write(root.join("anime.csv"), "header1,header2\n").unwrap();
    fs::write(root.join("archive/users.csv"), "u1,u2\n").unwrap();
    fs::write(root.join("readme.txt"), "not csv\n").unwrap();

    let files = FileScanner::scan(root.to_str().unwrap()).unwrap();

    assert_eq!(files.len(), 2, "Should find exactly 2 CSV files");
    assert!(files.iter().any(|f| f.ends_with("anime.csv")));
    assert!(files.iter().any(|f| f.ends_with("users.csv")));
}

#[test]
fn test_scan_skips_non_csv_files() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();

    fs::write(root.join("archive.zip"), "binary\n").unwrap();
    fs::write(root.join("readme.md"), "# readme\n").unwrap();
    fs::write(root.join("data.json"), "{}").unwrap();

    let files = FileScanner::scan(root.to_str().unwrap()).unwrap();

    assert_eq!(files.len(), 0, "Should skip non-CSV files");
}
