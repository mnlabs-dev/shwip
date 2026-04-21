use crate::error::ShwipError;
use crate::models::{ScanConfig, ScanResult};
use crate::scanners::EcosystemScanner;
use std::path::Path;

pub struct PnpmCacheScanner;

impl EcosystemScanner for PnpmCacheScanner {
    fn name(&self) -> &'static str {
        "pnpm"
    }

    fn scan(&self, home: &Path, config: &ScanConfig) -> Result<Vec<ScanResult>, ShwipError> {
        let cache_dir = home.join("Library/pnpm");
        super::npm_cache::scan_cache_dir(&cache_dir, "pnpm cache", config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pnpm_cache_absent() {
        let scanner = PnpmCacheScanner;
        let config = ScanConfig::default();
        let result = scanner.scan(Path::new("/nonexistent"), &config).unwrap();
        assert!(result.is_empty());
    }
}
