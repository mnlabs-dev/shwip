use crate::error::ShwipError;
use crate::models::{ScanConfig, ScanResult};
use crate::scanners;
use std::fs;
use std::path::Path;

pub async fn scan_all() -> Result<Vec<ScanResult>, ShwipError> {
    let config = ScanConfig::default();

    tokio::task::spawn_blocking(move || scan_all_sync(&config))
        .await
        .map_err(|e| ShwipError::Io(e.to_string()))?
}

fn scan_all_sync(config: &ScanConfig) -> Result<Vec<ScanResult>, ShwipError> {
    let home = dirs_home();
    let active_scanners = scanners::scanners_for_profiles(&config.profiles);
    let mut results = Vec::new();

    for scanner in active_scanners {
        match scanner.scan(&home, config) {
            Ok(r) => results.extend(r),
            Err(e) => {
                eprintln!("scanner '{}' failed: {}", scanner.name(), e);
            }
        }
    }

    Ok(results)
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

fn dirs_home() -> std::path::PathBuf {
    dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir_size_nonexistent() {
        assert_eq!(dir_size(Path::new("/nonexistent/path")), 0);
    }

    #[test]
    fn test_dirs_home_returns_path() {
        let home = dirs_home();
        assert!(home.exists());
    }
}
