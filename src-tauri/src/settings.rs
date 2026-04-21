use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub profiles: Vec<String>,
    pub exclusions: Vec<PathBuf>,
    pub schedule_enabled: bool,
    pub schedule_interval_hours: u64,
    pub autostart: bool,
    pub show_notifications: bool,
}

impl Default for Settings {
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
            schedule_enabled: false,
            schedule_interval_hours: 24,
            autostart: false,
            show_notifications: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_default_profiles() {
        let s = Settings::default();
        assert_eq!(s.profiles.len(), 12);
        assert!(s.profiles.contains(&"nvm".to_string()));
    }

    #[test]
    fn test_settings_default_schedule_off() {
        let s = Settings::default();
        assert!(!s.schedule_enabled);
        assert_eq!(s.schedule_interval_hours, 24);
    }

    #[test]
    fn test_settings_serialization_roundtrip() {
        let s = Settings::default();
        let json = serde_json::to_string(&s).unwrap();
        let deserialized: Settings = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.profiles.len(), s.profiles.len());
    }
}
