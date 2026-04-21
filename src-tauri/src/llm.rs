use crate::models::ScanResult;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const DEFAULT_BASE_URL: &str = "http://localhost:11434";
const DEFAULT_MODEL: &str = "llama3.2:3b";
const TIMEOUT_SECS: u64 = 10;

#[derive(Debug, Clone)]
pub struct OllamaClient {
    base_url: String,
    model: String,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct GenerateResponse {
    response: String,
}

impl Default for OllamaClient {
    fn default() -> Self {
        Self::new(DEFAULT_BASE_URL, DEFAULT_MODEL)
    }
}

impl OllamaClient {
    pub fn new(base_url: &str, model: &str) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(TIMEOUT_SECS))
            .build()
            .unwrap_or_default();
        Self {
            base_url: base_url.to_string(),
            model: model.to_string(),
            client,
        }
    }

    pub async fn explain_item(&self, item: &ScanResult) -> String {
        match self.generate(item).await {
            Ok(text) => text,
            Err(_) => fallback_explanation(item),
        }
    }

    async fn generate(&self, item: &ScanResult) -> Result<String, reqwest::Error> {
        let prompt = format!(
            "Explain in 1-2 sentences why this file/directory can be cleaned up from a Mac:\n\
             Category: {}\n\
             Path: {}\n\
             Size: {} bytes\n\
             Confidence: {:?}\n\
             Reason: {}\n\
             Be concise and helpful for a developer.",
            item.category, item.path, item.size_bytes, item.confidence, item.reason
        );

        let req = GenerateRequest {
            model: self.model.clone(),
            prompt,
            stream: false,
        };

        let resp = self
            .client
            .post(format!("{}/api/generate", self.base_url))
            .json(&req)
            .send()
            .await?
            .json::<GenerateResponse>()
            .await?;

        Ok(resp.response.trim().to_string())
    }

    pub async fn is_available(&self) -> bool {
        self.client
            .get(format!("{}/api/tags", self.base_url))
            .timeout(Duration::from_secs(2))
            .send()
            .await
            .is_ok()
    }
}

pub fn fallback_explanation(item: &ScanResult) -> String {
    format!(
        "{} ({:?}): {}",
        item.category, item.confidence, item.reason
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Confidence;

    fn sample_item() -> ScanResult {
        ScanResult {
            category: "npm".into(),
            path: "/Users/test/.npm/_cacache".into(),
            size_bytes: 500_000_000,
            confidence: Confidence::Safe,
            reason: "npm cache, safe to delete".into(),
        }
    }

    #[test]
    fn test_fallback_explanation() {
        let item = sample_item();
        let explanation = fallback_explanation(&item);
        assert!(explanation.contains("npm"));
        assert!(explanation.contains("Safe"));
    }

    #[test]
    fn test_client_default() {
        let client = OllamaClient::default();
        assert_eq!(client.base_url, DEFAULT_BASE_URL);
        assert_eq!(client.model, DEFAULT_MODEL);
    }

    #[tokio::test]
    async fn test_explain_item_fallback_when_offline() {
        let client = OllamaClient::new("http://localhost:1", "fake");
        let item = sample_item();
        let result = client.explain_item(&item).await;
        assert!(result.contains("npm"));
    }
}
