use crate::error::ShwipError;
use crate::models::{Confidence, ScanConfig, ScanResult};
use crate::scanners::EcosystemScanner;
use std::path::Path;

pub struct NpmCacheScanner;

impl EcosystemScanner for NpmCacheScanner {
    fn name(&self) -> &'static str {
        "npm"
    }

    fn scan(&self, home: &Path, config: &ScanConfig) -> Result<Vec<ScanResult>, ShwipError> {
        let cache_dir = home.join(".npm/_cacache");
        scan_cache_dir(&cache_dir, "npm cache", config)
    }
}

pub fn scan_cache_dir(
    path: &Path,
    category: &str,
    config: &ScanConfig,
) -> Result<Vec<ScanResult>, ShwipError> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let size = crate::scanner::dir_size(path);
    if size < config.min_size_bytes {
        return Ok(Vec::new());
    }

    Ok(vec![ScanResult {
        category: category.into(),
        path: path.to_string_lossy().into(),
        size_bytes: size,
        confidence: Confidence::Safe,
        reason: format!("{}, regenerable via install", category),
    }])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_npm_cache_absent() {
        let scanner = NpmCacheScanner;
        let config = ScanConfig::default();
        let result = scanner.scan(Path::new("/nonexistent"), &config).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_scan_cache_dir_absent() {
        let config = ScanConfig::default();
        let result = scan_cache_dir(Path::new("/nonexistent"), "test", &config).unwrap();
        assert!(result.is_empty());
    }
}
