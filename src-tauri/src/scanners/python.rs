use crate::error::ShwipError;
use crate::models::{Confidence, ScanConfig, ScanResult};
use crate::scanners::EcosystemScanner;
use std::fs;
use std::path::Path;

pub struct PythonScanner;

impl EcosystemScanner for PythonScanner {
    fn name(&self) -> &'static str {
        "uv"
    }

    fn scan(&self, home: &Path, config: &ScanConfig) -> Result<Vec<ScanResult>, ShwipError> {
        let mut results = Vec::new();

        for (dir, label) in [
            (home.join(".cache/uv"), "uv cache"),
            (home.join(".cache/pip"), "pip cache"),
        ] {
            if dir.exists() {
                let size = crate::scanner::dir_size(&dir);
                if size >= config.min_size_bytes {
                    results.push(ScanResult {
                        category: label.into(),
                        path: dir.to_string_lossy().into(),
                        size_bytes: size,
                        confidence: Confidence::Safe,
                        reason: format!("{label}, regenerable"),
                    });
                }
            }
        }

        results.extend(find_orphan_venvs(home, config)?);

        Ok(results)
    }
}

fn find_orphan_venvs(home: &Path, config: &ScanConfig) -> Result<Vec<ScanResult>, ShwipError> {
    let mut results = Vec::new();
    let dev_dir = home.join("Developer");

    if !dev_dir.exists() {
        return Ok(results);
    }

    if let Ok(entries) = fs::read_dir(&dev_dir) {
        for entry in entries.flatten() {
            let project = entry.path();
            if !project.is_dir() {
                continue;
            }

            for venv_name in [".venv", "venv", ".env"] {
                let venv = project.join(venv_name);
                if !venv.exists() || !venv.is_dir() {
                    continue;
                }

                let has_python_project = project.join("pyproject.toml").exists()
                    || project.join("setup.py").exists()
                    || project.join("requirements.txt").exists();

                if has_python_project {
                    continue;
                }

                let size = crate::scanner::dir_size(&venv);
                if size < config.min_size_bytes {
                    continue;
                }

                results.push(ScanResult {
                    category: "Python venv".into(),
                    path: venv.to_string_lossy().into(),
                    size_bytes: size,
                    confidence: Confidence::Review,
                    reason: format!(
                        "virtualenv '{venv_name}' with no Python project files in parent"
                    ),
                });
            }
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_absent_cache() {
        let scanner = PythonScanner;
        let config = ScanConfig::default();
        let result = scanner.scan(Path::new("/nonexistent"), &config).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_orphan_venvs_no_dev() {
        let result = find_orphan_venvs(Path::new("/nonexistent"), &ScanConfig::default()).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_orphan_venv_detection() {
        let dir = std::env::temp_dir().join("shwip_test_python");
        let project = dir.join("Developer/orphan-project/.venv/lib");
        let _ = fs::create_dir_all(&project);
        fs::write(project.join("big_file"), vec![0u8; 11_000_000]).unwrap();

        let mut config = ScanConfig::default();
        config.min_size_bytes = 1;

        let results = find_orphan_venvs(&dir, &config).unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].confidence, Confidence::Review);

        let _ = fs::remove_dir_all(&dir);
    }
}
