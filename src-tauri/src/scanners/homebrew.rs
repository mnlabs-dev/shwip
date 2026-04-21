use crate::error::ShwipError;
use crate::models::{Confidence, ScanConfig, ScanResult};
use crate::scanners::EcosystemScanner;
use std::path::Path;
use std::process::Command;

pub struct HomebrewScanner;

impl EcosystemScanner for HomebrewScanner {
    fn name(&self) -> &'static str {
        "homebrew"
    }

    fn scan(&self, _home: &Path, config: &ScanConfig) -> Result<Vec<ScanResult>, ShwipError> {
        let cache_dir = get_brew_cache();
        let cache_dir = match cache_dir {
            Some(d) => d,
            None => return Ok(Vec::new()),
        };

        let cache_path = Path::new(&cache_dir);
        if !cache_path.exists() {
            return Ok(Vec::new());
        }

        let size = crate::scanner::dir_size(cache_path);
        if size < config.min_size_bytes {
            return Ok(Vec::new());
        }

        let cleanup_items = get_cleanup_count();

        Ok(vec![ScanResult {
            category: "Homebrew".into(),
            path: cache_dir,
            size_bytes: size,
            confidence: Confidence::Safe,
            reason: match cleanup_items {
                Some(n) => format!("Homebrew cache, {} items cleanable", n),
                None => "Homebrew cache directory".into(),
            },
        }])
    }
}

fn get_brew_cache() -> Option<String> {
    Command::new("brew")
        .arg("--cache")
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        })
}

fn get_cleanup_count() -> Option<usize> {
    Command::new("brew")
        .args(["cleanup", "--dry-run"])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                let stdout = String::from_utf8_lossy(&o.stdout);
                Some(stdout.lines().count())
            } else {
                None
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_homebrew_scanner() {
        let scanner = HomebrewScanner;
        let config = ScanConfig::default();
        // Should not error even if brew is not installed
        let result = scanner.scan(Path::new("/nonexistent"), &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_brew_cache_returns_option() {
        let cache = get_brew_cache();
        // Either Some(path) or None
        if let Some(ref p) = cache {
            assert!(!p.is_empty());
        }
    }
}
