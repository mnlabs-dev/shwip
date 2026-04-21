use std::fs;
use std::path::PathBuf;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

pub fn log_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join(".shwip")
        .join("logs")
}

pub fn init_tracing() -> WorkerGuard {
    let dir = log_dir();
    fs::create_dir_all(&dir).expect("cannot create log directory");

    let file_appender = tracing_appender::rolling::daily(&dir, "shwip.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_target(true),
        )
        .init();

    guard
}

pub fn latest_log_path() -> Option<PathBuf> {
    let dir = log_dir();
    let mut entries: Vec<_> = fs::read_dir(&dir)
        .ok()?
        .flatten()
        .filter(|e| {
            e.path()
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with("shwip.log"))
        })
        .collect();
    entries.sort_by_key(|e| std::cmp::Reverse(e.metadata().ok().and_then(|m| m.modified().ok())));
    entries.first().map(|e| e.path())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_dir_is_under_home() {
        let dir = log_dir();
        assert!(dir.to_str().unwrap().contains(".shwip/logs"));
    }

    #[test]
    fn test_latest_log_path_no_crash() {
        let _ = latest_log_path();
    }
}
