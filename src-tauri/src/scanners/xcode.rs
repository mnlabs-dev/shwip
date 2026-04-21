use crate::error::ShwipError;
use crate::models::{Confidence, ScanConfig, ScanResult};
use crate::scanners::EcosystemScanner;
use std::fs;
use std::path::Path;

pub struct XcodeScanner;

impl EcosystemScanner for XcodeScanner {
    fn name(&self) -> &'static str {
        "xcode"
    }

    fn scan(&self, home: &Path, config: &ScanConfig) -> Result<Vec<ScanResult>, ShwipError> {
        let mut results = Vec::new();

        let derived_data = home.join("Library/Developer/Xcode/DerivedData");
        if derived_data.exists() {
            let size = crate::scanner::dir_size(&derived_data);
            if size >= config.min_size_bytes {
                results.push(ScanResult {
                    category: "Xcode".into(),
                    path: derived_data.to_string_lossy().into(),
                    size_bytes: size,
                    confidence: Confidence::Safe,
                    reason: "DerivedData, regenerable via Xcode build".into(),
                });
            }
        }

        let archives = home.join("Library/Developer/Xcode/Archives");
        if archives.exists() {
            let size = crate::scanner::dir_size(&archives);
            if size >= config.min_size_bytes {
                results.push(ScanResult {
                    category: "Xcode".into(),
                    path: archives.to_string_lossy().into(),
                    size_bytes: size,
                    confidence: Confidence::Review,
                    reason: "Xcode Archives, review before removing".into(),
                });
            }
        }

        results.extend(scan_simulators(home, config)?);

        Ok(results)
    }
}

fn scan_simulators(home: &Path, config: &ScanConfig) -> Result<Vec<ScanResult>, ShwipError> {
    let sim_dir = home.join("Library/Developer/CoreSimulator/Devices");
    if !sim_dir.exists() {
        return Ok(Vec::new());
    }

    let mut results = Vec::new();

    if let Ok(entries) = fs::read_dir(&sim_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            let plist = path.join("device.plist");
            if !plist.exists() {
                continue;
            }

            let size = crate::scanner::dir_size(&path);
            if size < config.min_size_bytes {
                continue;
            }

            let name = entry.file_name().to_string_lossy().to_string();
            results.push(ScanResult {
                category: "Xcode Simulator".into(),
                path: path.to_string_lossy().into(),
                size_bytes: size,
                confidence: Confidence::Review,
                reason: format!("simulator device '{}'", name),
            });
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xcode_absent() {
        let scanner = XcodeScanner;
        let config = ScanConfig::default();
        let result = scanner.scan(Path::new("/nonexistent"), &config).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_simulators_absent() {
        let result =
            scan_simulators(Path::new("/nonexistent"), &ScanConfig::default()).unwrap();
        assert!(result.is_empty());
    }
}
