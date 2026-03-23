//! Recursive file discovery module
pub struct FileScanner;

impl FileScanner {
    pub fn scan(_root: &str) -> anyhow::Result<Vec<String>> {
        Ok(vec![])
    }
}
