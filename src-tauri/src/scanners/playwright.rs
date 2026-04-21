use crate::error::ShwipError;
use crate::models::{Confidence, ScanConfig, ScanResult};
use crate::scanners::EcosystemScanner;
use std::fs;
use std::path::Path;

pub struct PlaywrightScanner;

impl EcosystemScanner for PlaywrightScanner {
    fn name(&self) -> &'static str {
        "playwright"
    }

    fn scan(&self, home: &Path, config: &ScanConfig) -> Result<Vec<ScanResult>, ShwipError> {
        let cache_dir = home.join("Library/Caches/ms-playwright");
        if !cache_dir.exists() {
            return Ok(Vec::new());
        }

        let mut browsers: Vec<(String, String, u64)> = Vec::new();

        if let Ok(entries) = fs::read_dir(&cache_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                let path = entry.path();

                if !path.is_dir() {
                    continue;
                }

                let size = crate::scanner::dir_size(&path);
                let (browser, version) = parse_browser_version(&name);
                browsers.push((browser, version, size));
            }
        }

        let latest = find_latest_versions(&browsers);
        let mut results = Vec::new();

        for (browser, version, size) in &browsers {
            if size < &config.min_size_bytes {
                continue;
            }

            let key = format!("{}-{}", browser, version);
            let is_latest = latest.get(browser.as_str()).map(|v| v == version).unwrap_or(false);

            if !is_latest {
                results.push(ScanResult {
                    category: "Playwright".into(),
                    path: cache_dir.join(&key).to_string_lossy().into(),
                    size_bytes: *size,
                    confidence: Confidence::Safe,
                    reason: format!("{} {} (newer version available)", browser, version),
                });
            }
        }

        Ok(results)
    }
}

fn parse_browser_version(name: &str) -> (String, String) {
    let parts: Vec<&str> = name.rsplitn(2, '-').collect();
    if parts.len() == 2 {
        (parts[1].to_string(), parts[0].to_string())
    } else {
        (name.to_string(), String::new())
    }
}

fn find_latest_versions(browsers: &[(String, String, u64)]) -> std::collections::HashMap<&str, &str> {
    let mut latest: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
    for (browser, version, _) in browsers {
        let entry = latest.entry(browser.as_str()).or_insert(version.as_str());
        if version.as_str() > *entry {
            *entry = version.as_str();
        }
    }
    latest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playwright_absent() {
        let scanner = PlaywrightScanner;
        let config = ScanConfig::default();
        let result = scanner.scan(Path::new("/nonexistent"), &config).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_browser_version() {
        let (browser, version) = parse_browser_version("chromium-1148");
        assert_eq!(browser, "chromium");
        assert_eq!(version, "1148");
    }

    #[test]
    fn test_find_latest() {
        let browsers = vec![
            ("chromium".into(), "1100".into(), 100u64),
            ("chromium".into(), "1200".into(), 100u64),
            ("firefox".into(), "1300".into(), 100u64),
        ];
        let latest = find_latest_versions(&browsers);
        assert_eq!(latest["chromium"], "1200");
        assert_eq!(latest["firefox"], "1300");
    }
}
