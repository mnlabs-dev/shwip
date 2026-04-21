use crate::error::ShwipError;
use crate::models::{Confidence, ScanConfig, ScanResult};
use std::fs;
use std::path::{Path, PathBuf};

pub async fn scan_all() -> Result<Vec<ScanResult>, ShwipError> {
    let config = ScanConfig::default();

    tokio::task::spawn_blocking(move || scan_all_sync(&config))
        .await
        .map_err(|e| ShwipError::Io(e.to_string()))?
}

fn scan_all_sync(config: &ScanConfig) -> Result<Vec<ScanResult>, ShwipError> {
    let home = dirs_home();
    let mut results = Vec::new();

    results.extend(scan_app_residuals(&home, config)?);

    Ok(results)
}

fn scan_app_residuals(
    home: &Path,
    config: &ScanConfig,
) -> Result<Vec<ScanResult>, ShwipError> {
    let app_support = home.join("Library/Application Support");
    let installed = installed_apps();
    let mut results = Vec::new();

    let entries = match fs::read_dir(&app_support) {
        Ok(e) => e,
        Err(_) => return Ok(results),
    };

    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        if config.exclusions.iter().any(|ex| path.starts_with(ex)) {
            continue;
        }

        let size = dir_size(&path);
        if size < config.min_size_bytes {
            continue;
        }

        if !is_app_installed(&name, &installed) {
            results.push(ScanResult {
                category: "App residual".into(),
                path: path.to_string_lossy().into(),
                size_bytes: size,
                confidence: Confidence::Safe,
                reason: format!("'{}' not found in /Applications", name),
            });
        }
    }

    Ok(results)
}

fn installed_apps() -> Vec<String> {
    let mut apps = Vec::new();
    if let Ok(entries) = fs::read_dir("/Applications") {
        for entry in entries.flatten() {
            apps.push(
                entry
                    .file_name()
                    .to_string_lossy()
                    .trim_end_matches(".app")
                    .to_lowercase(),
            );
        }
    }
    apps
}

fn is_app_installed(name: &str, installed: &[String]) -> bool {
    let lower = name.to_lowercase();
    installed
        .iter()
        .any(|app| app.contains(&lower) || lower.contains(app))
}

pub fn dir_size(path: &Path) -> u64 {
    let mut size = 0u64;
    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => return 0,
    };
    for entry in entries.flatten() {
        if let Ok(m) = entry.metadata() {
            if m.is_file() {
                size += m.len();
            } else if m.is_dir() {
                size += dir_size(&entry.path());
            }
        }
    }
    size
}

fn dirs_home() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir_size_nonexistent() {
        assert_eq!(dir_size(Path::new("/nonexistent/path")), 0);
    }

    #[test]
    fn test_installed_apps_returns_vec() {
        let apps = installed_apps();
        assert!(apps.iter().all(|a| a == &a.to_lowercase()));
    }

    #[test]
    fn test_is_app_installed_match() {
        let installed = vec!["visual studio code".to_string(), "firefox".to_string()];
        assert!(is_app_installed("Code", &installed));
        assert!(is_app_installed("Firefox", &installed));
        assert!(!is_app_installed("Nonexistent", &installed));
    }
}
