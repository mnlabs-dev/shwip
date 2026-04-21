use crate::error::ShwipError;
use crate::models::{Confidence, ScanConfig, ScanResult};
use crate::scanners::EcosystemScanner;
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct OllamaScanner;

impl EcosystemScanner for OllamaScanner {
    fn name(&self) -> &'static str {
        "ollama"
    }

    fn scan(&self, home: &Path, config: &ScanConfig) -> Result<Vec<ScanResult>, ShwipError> {
        let models_dir = home.join(".ollama/models");
        if !models_dir.exists() {
            return Ok(Vec::new());
        }

        let blobs_dir = models_dir.join("blobs");
        if !blobs_dir.exists() {
            return Ok(Vec::new());
        }

        let model_info = get_ollama_models();
        let total_size = crate::scanner::dir_size(&models_dir);

        if total_size < config.min_size_bytes {
            return Ok(Vec::new());
        }

        let mut results = Vec::new();

        match model_info {
            Some(models) => {
                for model in models {
                    results.push(ScanResult {
                        category: "Ollama".into(),
                        path: models_dir.to_string_lossy().into(),
                        size_bytes: model.size_bytes,
                        confidence: Confidence::Review,
                        reason: format!("model '{}', {}", model.name, model.detail),
                    });
                }
            }
            None => {
                results.push(ScanResult {
                    category: "Ollama".into(),
                    path: models_dir.to_string_lossy().into(),
                    size_bytes: total_size,
                    confidence: Confidence::Review,
                    reason: "Ollama models directory (ollama CLI not available for details)"
                        .into(),
                });
            }
        }

        Ok(results)
    }
}

struct OllamaModel {
    name: String,
    size_bytes: u64,
    detail: String,
}

fn get_ollama_models() -> Option<Vec<OllamaModel>> {
    let output = Command::new("ollama").arg("list").output().ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut models = Vec::new();

    for line in stdout.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let name = parts[0].to_string();
            let size_str = parts.get(2).unwrap_or(&"0");
            let size_bytes = parse_size(size_str);
            let detail = parts[3..].join(" ");

            models.push(OllamaModel {
                name,
                size_bytes,
                detail,
            });
        }
    }

    Some(models)
}

fn parse_size(s: &str) -> u64 {
    let s = s.trim().to_uppercase();
    let (num_str, multiplier) = if s.ends_with("GB") {
        (&s[..s.len() - 2], 1_073_741_824u64)
    } else if s.ends_with("MB") {
        (&s[..s.len() - 2], 1_048_576u64)
    } else if s.ends_with("KB") {
        (&s[..s.len() - 2], 1_024u64)
    } else {
        (s.as_str(), 1u64)
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
    fn test_ollama_absent() {
        let scanner = OllamaScanner;
        let config = ScanConfig::default();
        let result = scanner.scan(Path::new("/nonexistent"), &config).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_size_gb() {
        assert_eq!(parse_size("4.7GB"), 4_7 * 1_073_741_824 / 10);
    }

    #[test]
    fn test_parse_size_mb() {
        let result = parse_size("512MB");
        assert!(result > 500_000_000 && result < 600_000_000);
    }

    #[test]
    fn test_parse_size_invalid() {
        assert_eq!(parse_size("invalid"), 0);
    }
}
