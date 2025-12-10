use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::client::{build_prompt, parse_commit_message, LlmClient};
use super::provider::Provider;

/// Gemini API request format
#[derive(Debug, Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
    #[serde(rename = "generationConfig")]
    generation_config: GenerationConfig,
}

#[derive(Debug, Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Serialize)]
struct Part {
    text: String,
}

#[derive(Debug, Serialize)]
struct GenerationConfig {
    temperature: f32,
}

/// Gemini API response format
#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: ResponseContent,
}

#[derive(Debug, Deserialize)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

#[derive(Debug, Deserialize)]
struct ResponsePart {
    text: String,
}

/// Client for Google Gemini API
#[derive(Debug, Clone)]
pub struct GeminiClient {
    api_key: String,
    base_url: String,
    model: String,
    client: reqwest::Client,
}

impl GeminiClient {
    /// Create a new Gemini client
    pub fn new(api_key: String) -> Self {
        let provider = Provider::Gemini;
        Self {
            api_key,
            base_url: provider.base_url().to_string(),
            model: provider.default_model().to_string(),
            client: reqwest::Client::new(),
        }
    }

    /// Create client with custom base URL (useful for testing or custom endpoints)
    pub fn with_base_url(api_key: String, base_url: String) -> Self {
        Self {
            api_key,
            base_url,
            model: Provider::Gemini.default_model().to_string(),
            client: reqwest::Client::new(),
        }
    }

    /// Set custom model
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    fn build_system_prompt(&self, template: Option<&str>) -> String {
        format!(
            "あなたは経験豊富なソフトウェアエンジニアです。Git diffから適切なコミットメッセージを生成してください。\n\n{}",
            template.unwrap_or("")
        )
    }
}

#[async_trait]
impl LlmClient for GeminiClient {
    async fn generate_commit_message(
        &self,
        diff: &str,
        template: Option<&str>,
    ) -> Result<(String, String)> {
        let prompt = build_prompt(diff, template);
        let system_prompt = self.build_system_prompt(template);

        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![
                    Part {
                        text: system_prompt,
                    },
                    Part { text: prompt },
                ],
            }],
            generation_config: GenerationConfig { temperature: 0.7 },
        };

        // Gemini uses query parameter for API key
        let url = format!(
            "{}/v1beta/models/{}:generateContent?key={}",
            self.base_url, self.model, self.api_key
        );

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Gemini API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "Gemini API request failed ({}): {}",
                status,
                error_text
            ));
        }

        let api_response: GeminiResponse = response
            .json()
            .await
            .context("Failed to parse Gemini API response")?;

        let message = api_response
            .candidates
            .first()
            .context("No candidates in API response")?
            .content
            .parts
            .first()
            .context("No parts in response content")?
            .text
            .trim();

        Ok(parse_commit_message(message))
    }

    fn provider_name(&self) -> &str {
        "Gemini"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn test_gemini_generate_commit_message() {
        let mut server = Server::new_async().await;
        let _m = server
            .mock("POST", mockito::Matcher::Regex(r"/v1beta/models/.*:generateContent.*".to_string()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "candidates": [{
                    "content": {
                        "parts": [{
                            "text": "feat: Add new feature\n\nImplemented the feature"
                        }]
                    }
                }]
            }"#,
            )
            .create_async()
            .await;

        let client = GeminiClient::with_base_url("test-key".into(), server.url());

        let (title, desc) = client
            .generate_commit_message("diff --git", None)
            .await
            .unwrap();

        assert_eq!(title, "feat: Add new feature");
        assert_eq!(desc, "Implemented the feature");
    }

    #[tokio::test]
    async fn test_gemini_api_error() {
        let mut server = Server::new_async().await;
        let _m = server
            .mock("POST", mockito::Matcher::Regex(r"/v1beta/models/.*:generateContent.*".to_string()))
            .with_status(400)
            .with_body(r#"{"error": {"message": "Invalid API key"}}"#)
            .create_async()
            .await;

        let client = GeminiClient::with_base_url("bad-key".into(), server.url());

        let result = client.generate_commit_message("diff", None).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_provider_name() {
        let client = GeminiClient::new("key".into());
        assert_eq!(client.provider_name(), "Gemini");
    }

    #[test]
    fn test_with_model() {
        let client = GeminiClient::new("key".into()).with_model("gemini-1.5-pro");
        assert_eq!(client.model, "gemini-1.5-pro");
    }
}
