use crate::error::ShwipError;
use crate::models::{Confidence, ScanConfig, ScanResult};
use crate::scanners::EcosystemScanner;
use std::fs;
use std::path::Path;

pub struct NvmScanner;

impl EcosystemScanner for NvmScanner {
    fn name(&self) -> &'static str {
        "nvm"
    }

    fn scan(&self, home: &Path, config: &ScanConfig) -> Result<Vec<ScanResult>, ShwipError> {
        let nvm_dir = home.join(".nvm/versions/node");
        if !nvm_dir.exists() {
            return Ok(Vec::new());
        }

        let current_version = current_nvm_version(home);
        let nvmrc_versions = collect_nvmrc_versions(home);
        let mut results = Vec::new();

        let entries = match fs::read_dir(&nvm_dir) {
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

            let version = name.trim_start_matches('v').to_string();
            let is_current = current_version.as_deref() == Some(version.as_str());
            let in_nvmrc = nvmrc_versions.iter().any(|v| v.contains(&version));

            let (confidence, reason) = if is_current {
                (Confidence::Keep, format!("v{version} is the active Node version"))
            } else if in_nvmrc {
                (Confidence::Keep, format!("v{version} referenced in a .nvmrc file"))
            } else {
                (Confidence::Safe, format!("v{version} not in any .nvmrc and not active"))
            };

            if confidence != Confidence::Keep {
                results.push(ScanResult {
                    category: "NVM".into(),
                    path: path.to_string_lossy().into(),
                    size_bytes: size,
                    confidence,
                    reason,
                });
            }
        }

        Ok(results)
    }
}

fn current_nvm_version(home: &Path) -> Option<String> {
    let alias_default = home.join(".nvm/alias/default");
    if let Ok(content) = fs::read_to_string(alias_default) {
        let version = content.trim().trim_start_matches('v').to_string();
        if !version.is_empty() {
            return Some(version);
        }
    }

    let current_link = home.join(".nvm/current");
    if let Ok(target) = fs::read_link(current_link) {
        if let Some(name) = target.file_name() {
            return Some(name.to_string_lossy().trim_start_matches('v').to_string());
        }
    }

    None
}

fn collect_nvmrc_versions(home: &Path) -> Vec<String> {
    let mut versions = Vec::new();
    let dev_dir = home.join("Developer");

    if !dev_dir.exists() {
        return versions;
    }

    if let Ok(entries) = fs::read_dir(&dev_dir) {
        for entry in entries.flatten() {
            let nvmrc = entry.path().join(".nvmrc");
            if let Ok(content) = fs::read_to_string(nvmrc) {
                let v = content.trim().trim_start_matches('v').to_string();
                if !v.is_empty() {
                    versions.push(v);
                }
            }
        }
    }

    versions
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_nvm_absent_dir() {
        let scanner = NvmScanner;
        let config = ScanConfig::default();
        let result = scanner.scan(Path::new("/nonexistent"), &config).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_current_nvm_version_missing() {
        let version = current_nvm_version(Path::new("/nonexistent"));
        assert!(version.is_none());
    }

    #[test]
    fn test_collect_nvmrc_no_dev_dir() {
        let versions = collect_nvmrc_versions(Path::new("/nonexistent"));
        assert!(versions.is_empty());
    }

    #[test]
    fn test_nvmrc_parsing() {
        let dir = std::env::temp_dir().join("shwip_test_nvm");
        let project = dir.join("Developer/test-project");
        let _ = fs::create_dir_all(&project);
        fs::write(project.join(".nvmrc"), "v20.11.0\n").unwrap();

        let versions = collect_nvmrc_versions(&dir);
        assert!(versions.contains(&"20.11.0".to_string()));

        let _ = fs::remove_dir_all(&dir);
    }
}
