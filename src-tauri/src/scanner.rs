use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct ScanResult {
    pub category: String,
    pub path: String,
    pub size_bytes: u64,
    pub confidence: Confidence,
    pub reason: String,
}

#[derive(Serialize)]
pub enum Confidence {
    Safe,
    Review,
    Keep,
}

pub fn scan_all() -> Vec<ScanResult> {
    let home = dirs_home();
    let mut results = Vec::new();

    results.extend(scan_app_residuals(&home));

    results
}

fn scan_app_residuals(home: &PathBuf) -> Vec<ScanResult> {
    let app_support = home.join("Library/Application Support");
    let installed = installed_apps();
    let mut results = Vec::new();

    if let Ok(entries) = fs::read_dir(&app_support) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let path = entry.path();

            if !path.is_dir() {
                continue;
            }

            let size = dir_size(&path);
            if size < 10_000_000 {
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
    }

    results
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
    installed.iter().any(|app| {
        app.contains(&lower) || lower.contains(app)
    })
}

fn dir_size(path: &PathBuf) -> u64 {
    let mut size = 0u64;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let meta = entry.metadata();
            if let Ok(m) = meta {
                if m.is_file() {
                    size += m.len();
                } else if m.is_dir() {
                    size += dir_size(&entry.path());
                }
            }
        }
    }
    size
}

fn dirs_home() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"))
}
