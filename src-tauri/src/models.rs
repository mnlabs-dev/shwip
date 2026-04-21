use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub category: String,
    pub path: String,
    pub size_bytes: u64,
    pub confidence: Confidence,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Confidence {
    Safe,
    Review,
    Keep,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    pub profiles: Vec<String>,
    pub exclusions: Vec<PathBuf>,
    pub min_size_bytes: u64,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            profiles: vec![
                "app_residuals".into(),
                "nvm".into(),
                "npm".into(),
                "bun".into(),
                "pnpm".into(),
                "uv".into(),
                "cargo".into(),
                "ollama".into(),
                "playwright".into(),
                "docker".into(),
                "xcode".into(),
                "homebrew".into(),
            ],
            exclusions: Vec::new(),
            min_size_bytes: 10_000_000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confidence_serialization() {
        let json = serde_json::to_string(&Confidence::Safe).unwrap();
        assert_eq!(json, "\"Safe\"");
        let parsed: Confidence = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, Confidence::Safe);
    }

    #[test]
    fn test_scan_result_serialization() {
        let result = ScanResult {
            category: "test".into(),
            path: "/tmp/test".into(),
            size_bytes: 1024,
            confidence: Confidence::Review,
            reason: "test reason".into(),
        };
        let json = serde_json::to_string(&result).unwrap();
        let parsed: ScanResult = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.confidence, Confidence::Review);
        assert_eq!(parsed.size_bytes, 1024);
    }

    #[test]
    fn test_scan_config_default() {
        let config = ScanConfig::default();
        assert_eq!(config.profiles.len(), 12);
        assert_eq!(config.min_size_bytes, 10_000_000);
        assert!(config.exclusions.is_empty());
    }
}
