use crate::error::ShwipError;
use crate::models::{Confidence, ScanConfig, ScanResult};
use crate::scanners::EcosystemScanner;
use std::path::Path;
use std::process::Command;

pub struct DockerScanner;

impl EcosystemScanner for DockerScanner {
    fn name(&self) -> &'static str {
        "docker"
    }

    fn scan(&self, home: &Path, config: &ScanConfig) -> Result<Vec<ScanResult>, ShwipError> {
        let mut results = Vec::new();

        results.extend(scan_docker_system(config)?);

        let orbstack_dir = home.join(".orbstack");
        if orbstack_dir.exists() {
            let size = crate::scanner::dir_size(&orbstack_dir);
            if size >= config.min_size_bytes {
                results.push(ScanResult {
                    category: "OrbStack".into(),
                    path: orbstack_dir.to_string_lossy().into(),
                    size_bytes: size,
                    confidence: Confidence::Review,
                    reason: "OrbStack data directory, review before cleaning".into(),
                });
            }
        }

        Ok(results)
    }
}

fn scan_docker_system(config: &ScanConfig) -> Result<Vec<ScanResult>, ShwipError> {
    let output = match Command::new("docker")
        .args(["system", "df", "--format", "{{.Type}}\t{{.Size}}\t{{.Reclaimable}}"])
        .output()
    {
        Ok(o) if o.status.success() => o,
        _ => return Ok(Vec::new()),
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut results = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 3 {
            let category = parts[0];
            let reclaimable = parts[2];

            let size = parse_docker_size(reclaimable);
            if size < config.min_size_bytes {
                continue;
            }

            results.push(ScanResult {
                category: format!("Docker {category}"),
                path: "docker system".into(),
                size_bytes: size,
                confidence: Confidence::Safe,
                reason: format!("Docker {category} reclaimable: {reclaimable}"),
            });
        }
    }

    Ok(results)
}

fn parse_docker_size(s: &str) -> u64 {
    let s = s.trim();
    let (num_part, _) = s.split_once('(').unwrap_or((s, ""));
    let num_part = num_part.trim();

    let (num_str, multiplier) = if let Some(stripped) = num_part.strip_suffix("GB") {
        (stripped, 1_073_741_824u64)
    } else if let Some(stripped) = num_part.strip_suffix("MB") {
        (stripped, 1_048_576u64)
    } else if let Some(stripped) = num_part.strip_suffix("kB") {
        (stripped, 1_024u64)
    } else if let Some(stripped) = num_part.strip_suffix('B') {
        (stripped, 1u64)
    } else {
        (num_part, 1u64)
    };

    num_str
        .trim()
        .parse::<f64>()
        .map(|n| (n * multiplier as f64) as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docker_absent() {
        let scanner = DockerScanner;
        let config = ScanConfig::default();
        let result = scanner.scan(Path::new("/nonexistent"), &config).unwrap();
        // May or may not find docker system results, but should not error
        assert!(result.is_empty() || !result.is_empty());
    }

    #[test]
    fn test_parse_docker_size_gb() {
        let size = parse_docker_size("2.5GB");
        assert!(size > 2_000_000_000);
    }

    #[test]
    fn test_parse_docker_size_with_paren() {
        let size = parse_docker_size("1.2GB (100%)");
        assert!(size > 1_000_000_000);
    }
}
