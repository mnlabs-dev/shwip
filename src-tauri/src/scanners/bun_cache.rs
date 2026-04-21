use crate::error::ShwipError;
use crate::models::{ScanConfig, ScanResult};
use crate::scanners::EcosystemScanner;
use std::path::Path;

pub struct BunCacheScanner;

impl EcosystemScanner for BunCacheScanner {
    fn name(&self) -> &'static str {
        "bun"
    }

    fn scan(&self, home: &Path, config: &ScanConfig) -> Result<Vec<ScanResult>, ShwipError> {
        let cache_dir = home.join(".bun/install/cache");
        super::npm_cache::scan_cache_dir(&cache_dir, "bun cache", config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bun_cache_absent() {
        let scanner = BunCacheScanner;
        let config = ScanConfig::default();
        let result = scanner.scan(Path::new("/nonexistent"), &config).unwrap();
        assert!(result.is_empty());
    }
}
