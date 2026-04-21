use crate::error::ShwipError;
use crate::models::{Confidence, ScanConfig, ScanResult};
use crate::scanners::EcosystemScanner;
use std::fs;
use std::path::Path;
use sysinfo::System;

pub struct AppResidualScanner;

impl EcosystemScanner for AppResidualScanner {
    fn name(&self) -> &'static str {
        "app_residuals"
    }

    fn scan(&self, home: &Path, config: &ScanConfig) -> Result<Vec<ScanResult>, ShwipError> {
        let installed = installed_apps();
        let running = running_processes();
        let launch_agents = active_launch_agents(home);
        let mut results = Vec::new();

        let dirs = [
            home.join("Library/Application Support"),
            home.join("Library/Caches"),
            home.join("Library/Preferences"),
            home.join("Library/Saved Application State"),
        ];

        for dir in &dirs {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    let path = entry.path();

                    if !path.is_dir() {
                        continue;
                    }

                    if config.exclusions.iter().any(|ex| path.starts_with(ex)) {
                        continue;
                    }

                    let size = crate::scanner::dir_size(&path);
                    if size < config.min_size_bytes {
                        continue;
                    }

                    let app_installed = is_app_installed(&name, &installed);
                    let process_running = is_process_running(&name, &running);
                    let has_launch_agent = has_launch_agent(&name, &launch_agents);

                    if app_installed || process_running || has_launch_agent {
                        continue;
                    }

                    results.push(ScanResult {
                        category: "App residual".into(),
                        path: path.to_string_lossy().into(),
                        size_bytes: size,
                        confidence: Confidence::Safe,
                        reason: format!(
                            "'{}' not installed, no active process, no LaunchAgent",
                            name
                        ),
                    });
                }
            }
        }

        Ok(results)
    }
}

fn installed_apps() -> Vec<String> {
    let mut apps = Vec::new();
    for dir in ["/Applications", &format!("{}/Applications", dirs::home_dir().map(|h| h.display().to_string()).unwrap_or_default())] {
        if let Ok(entries) = fs::read_dir(dir) {
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
    }
    apps
}

fn running_processes() -> Vec<String> {
    let mut sys = System::new();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
    sys.processes()
        .values()
        .map(|p| p.name().to_string_lossy().to_lowercase())
        .collect()
}

fn active_launch_agents(home: &Path) -> Vec<String> {
    let mut agents = Vec::new();
    let dirs = [
        home.join("Library/LaunchAgents"),
        Path::new("/Library/LaunchAgents").to_path_buf(),
    ];
    for dir in &dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                agents.push(entry.file_name().to_string_lossy().to_lowercase());
            }
        }
    }
    agents
}

fn is_app_installed(name: &str, installed: &[String]) -> bool {
    let lower = name.to_lowercase();
    installed
        .iter()
        .any(|app| app.contains(&lower) || lower.contains(app))
}

fn is_process_running(name: &str, processes: &[String]) -> bool {
    let lower = name.to_lowercase();
    processes.iter().any(|p| p.contains(&lower))
}

fn has_launch_agent(name: &str, agents: &[String]) -> bool {
    let lower = name.to_lowercase();
    agents.iter().any(|a| a.contains(&lower))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_installed_apps_lowercase() {
        let apps = installed_apps();
        assert!(apps.iter().all(|a| a == &a.to_lowercase()));
    }

    #[test]
    fn test_is_app_installed_match() {
        let installed = vec!["visual studio code".to_string(), "firefox".to_string()];
        assert!(is_app_installed("Code", &installed));
        assert!(!is_app_installed("Nonexistent", &installed));
    }

    #[test]
    fn test_is_process_running() {
        let processes = vec!["finder".to_string(), "dock".to_string()];
        assert!(is_process_running("Finder", &processes));
        assert!(!is_process_running("Nonexistent", &processes));
    }

    #[test]
    fn test_has_launch_agent() {
        let agents = vec!["com.apple.something.plist".to_string()];
        assert!(has_launch_agent("apple", &agents));
        assert!(!has_launch_agent("nonexistent", &agents));
    }
}
