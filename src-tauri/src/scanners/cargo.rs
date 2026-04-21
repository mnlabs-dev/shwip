use crate::error::ShwipError;
use crate::models::{Confidence, ScanConfig, ScanResult};
use crate::scanners::EcosystemScanner;
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct CargoScanner;

impl EcosystemScanner for CargoScanner {
    fn name(&self) -> &'static str {
        "cargo"
    }

    fn scan(&self, home: &Path, config: &ScanConfig) -> Result<Vec<ScanResult>, ShwipError> {
        let mut results = Vec::new();

        for (dir, label) in [
            (home.join(".cargo/registry/cache"), "cargo registry cache"),
            (home.join(".cargo/registry/src"), "cargo registry sources"),
        ] {
            if dir.exists() {
                let size = crate::scanner::dir_size(&dir);
                if size >= config.min_size_bytes {
                    results.push(ScanResult {
                        category: "Cargo".into(),
                        path: dir.to_string_lossy().into(),
                        size_bytes: size,
                        confidence: Confidence::Safe,
                        reason: format!("{label}, regenerable via cargo build"),
                    });
                }
            }
        }

        results.extend(scan_rustup_toolchains(home, config)?);

        Ok(results)
    }
}

fn scan_rustup_toolchains(
    home: &Path,
    config: &ScanConfig,
) -> Result<Vec<ScanResult>, ShwipError> {
    let toolchains_dir = home.join(".rustup/toolchains");
    if !toolchains_dir.exists() {
        return Ok(Vec::new());
    }

    let active_toolchain = get_active_toolchain();
    let mut results = Vec::new();

    let entries = match fs::read_dir(&toolchains_dir) {
        Ok(e) => e,
        Err(_) => return Ok(results),
    };

    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let size = crate::scanner::dir_size(&path);
        if size < config.min_size_bytes {
            continue;
        }

        let is_active = active_toolchain
            .as_ref()
            .map(|a| name.contains(a))
            .unwrap_or(false);

        if !is_active {
            results.push(ScanResult {
                category: "Rustup".into(),
                path: path.to_string_lossy().into(),
                size_bytes: size,
                confidence: Confidence::Safe,
                reason: format!("toolchain '{name}' not active"),
            });
        }
    }

    Ok(results)
}

fn get_active_toolchain() -> Option<String> {
    Command::new("rustup")
        .args(["show", "active-toolchain"])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                let s = String::from_utf8_lossy(&o.stdout);
                s.split_whitespace().next().map(|t| t.to_string())
            } else {
                None
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cargo_absent() {
        let scanner = CargoScanner;
        let config = ScanConfig::default();
        let result = scanner.scan(Path::new("/nonexistent"), &config).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_rustup_toolchains_absent() {
        let result =
            scan_rustup_toolchains(Path::new("/nonexistent"), &ScanConfig::default()).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_active_toolchain_returns_some_or_none() {
        let result = get_active_toolchain();
        // Either Some("stable-...") or None if rustup not installed
        if let Some(ref t) = result {
            assert!(!t.is_empty());
        }
    }
}
