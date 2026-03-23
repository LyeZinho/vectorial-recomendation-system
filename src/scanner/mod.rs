//! Recursive file discovery module
use walkdir::WalkDir;

pub struct FileScanner;

impl FileScanner {
    pub fn scan(root: &str) -> anyhow::Result<Vec<String>> {
        let mut csv_files = Vec::new();

        for entry in WalkDir::new(root)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.path().is_file())
        {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "csv" {
                    csv_files.push(path.to_string_lossy().to_string());
                }
            }
        }

        csv_files.sort();
        Ok(csv_files)
    }
}
