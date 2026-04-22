use crate::error::ShwipError;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrashLogEntry {
    pub original_path: PathBuf,
    pub size_bytes: u64,
    pub category: String,
    pub timestamp: u64,
}

pub fn move_to_trash(path: &Path) -> Result<(), ShwipError> {
    if !path.exists() {
        return Err(ShwipError::Trash(format!(
            "path does not exist: {}",
            path.display()
        )));
    }
    trash::delete(path).map_err(|e| ShwipError::Trash(e.to_string()))
}

pub fn create_log_entry(path: &Path, size_bytes: u64, category: &str) -> TrashLogEntry {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    TrashLogEntry {
        original_path: path.to_path_buf(),
        size_bytes,
        category: category.to_string(),
        timestamp,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_move_to_trash_nonexistent() {
        let result = move_to_trash(Path::new("/nonexistent/path/to/file"));
        assert!(result.is_err());
    }

    #[test]
    fn test_move_to_trash_real_file() {
        let dir = std::env::temp_dir().join("shwip_test_trash");
        let _ = fs::create_dir_all(&dir);
        let file = dir.join("test_trash_file.txt");
        fs::write(&file, "test content").unwrap();

        let result = move_to_trash(&file);
        assert!(result.is_ok());
        assert!(!file.exists());

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_create_log_entry() {
        let entry = create_log_entry(
            Path::new("/tmp/test"),
            1024,
            "App residual",
        );
        assert_eq!(entry.original_path, Path::new("/tmp/test"));
        assert_eq!(entry.size_bytes, 1024);
        assert_eq!(entry.category, "App residual");
        assert!(entry.timestamp > 0);
    }
}
