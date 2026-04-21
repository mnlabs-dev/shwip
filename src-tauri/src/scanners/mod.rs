pub mod app_residuals;
pub mod bun_cache;
pub mod cargo;
pub mod docker;
pub mod homebrew;
pub mod npm_cache;
pub mod nvm;
pub mod ollama;
pub mod playwright;
pub mod pnpm_cache;
pub mod python;
pub mod xcode;

use crate::error::ShwipError;
use crate::models::{ScanConfig, ScanResult};
use std::path::Path;

pub trait EcosystemScanner: Send + Sync {
    fn name(&self) -> &'static str;
    fn scan(&self, home: &Path, config: &ScanConfig) -> Result<Vec<ScanResult>, ShwipError>;
}

pub fn all_scanners() -> Vec<Box<dyn EcosystemScanner>> {
    vec![
        Box::new(app_residuals::AppResidualScanner),
        Box::new(nvm::NvmScanner),
        Box::new(npm_cache::NpmCacheScanner),
        Box::new(bun_cache::BunCacheScanner),
        Box::new(pnpm_cache::PnpmCacheScanner),
        Box::new(python::PythonScanner),
        Box::new(cargo::CargoScanner),
        Box::new(ollama::OllamaScanner),
        Box::new(playwright::PlaywrightScanner),
        Box::new(docker::DockerScanner),
        Box::new(xcode::XcodeScanner),
        Box::new(homebrew::HomebrewScanner),
    ]
}

pub fn scanners_for_profiles(profiles: &[String]) -> Vec<Box<dyn EcosystemScanner>> {
    all_scanners()
        .into_iter()
        .filter(|s| profiles.contains(&s.name().to_string()))
        .collect()
}
