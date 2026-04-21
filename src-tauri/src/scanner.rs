use crate::error::ShwipError;
use crate::models::{ScanConfig, ScanResult};
use crate::scanners;
use std::fs;
use std::path::Path;
use tokio::task::JoinSet;

pub async fn scan_all() -> Result<Vec<ScanResult>, ShwipError> {
    let config = ScanConfig::default();
    scan_parallel(&config).await
}

pub async fn scan_all_with_progress<F>(progress: F) -> Result<Vec<ScanResult>, ShwipError>
where
    F: Fn(&str, bool) + Send + 'static,
{
    let config = ScanConfig::default();
    let home = dirs_home();
    let active_scanners = scanners::scanners_for_profiles(&config.profiles);
    let total = active_scanners.len();
    let mut set = JoinSet::new();

    for scanner in active_scanners {
        let h = home.clone();
        let c = config.clone();
        set.spawn_blocking(move || {
            let name = scanner.name().to_string();
            let result = scanner.scan(&h, &c);
            (name, result)
        });
    }

    let mut results = Vec::new();
    let mut done = 0usize;

    while let Some(join_result) = set.join_next().await {
        let (name, scan_result) = join_result.map_err(|e| ShwipError::Io(e.to_string()))?;
        done += 1;
        match scan_result {
            Ok(r) => {
                progress(&name, true);
                results.extend(r);
            }
            Err(e) => {
                eprintln!("scanner '{name}' failed: {e}");
                progress(&name, false);
            }
        }
        let _ = (done, total);
    }

    Ok(results)
}

async fn scan_parallel(config: &ScanConfig) -> Result<Vec<ScanResult>, ShwipError> {
    let home = dirs_home();
    let active_scanners = scanners::scanners_for_profiles(&config.profiles);
    let mut set = JoinSet::new();

    for scanner in active_scanners {
        let h = home.clone();
        let c = config.clone();
        set.spawn_blocking(move || {
            let name = scanner.name().to_string();
            let result = scanner.scan(&h, &c);
            (name, result)
        });
    }

    let mut results = Vec::new();

    while let Some(join_result) = set.join_next().await {
        let (name, scan_result) = join_result.map_err(|e| ShwipError::Io(e.to_string()))?;
        match scan_result {
            Ok(r) => results.extend(r),
            Err(e) => {
                eprintln!("scanner '{name}' failed: {e}");
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
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_dir_size_nonexistent() {
        assert_eq!(dir_size(Path::new("/nonexistent/path")), 0);
    }

    #[test]
    fn test_dirs_home_returns_path() {
        let home = dirs_home();
        assert!(home.exists());
    }

    #[tokio::test]
    async fn test_scan_all_returns_results() {
        let results = scan_all().await;
        assert!(results.is_ok());
    }

    #[tokio::test]
    async fn test_scan_all_with_progress_calls_callback() {
        let count = Arc::new(AtomicUsize::new(0));
        let count_clone = count.clone();
        let results = scan_all_with_progress(move |_name, _ok| {
            count_clone.fetch_add(1, Ordering::Relaxed);
        })
        .await;
        assert!(results.is_ok());
        assert!(count.load(Ordering::Relaxed) > 0);
    }
}
