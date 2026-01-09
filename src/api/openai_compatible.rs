use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::client::{build_prompt, parse_commit_message, LlmClient};
use super::provider::Provider;

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ChatMessage,
}

/// Client for OpenAI-compatible APIs (OpenAI, DeepSeek)
#[derive(Debug, Clone)]
pub struct OpenAiCompatibleClient {
    api_key: String,
    base_url: String,
    model: String,
    provider: Provider,
    client: reqwest::Client,
}

impl OpenAiCompatibleClient {
    /// Create a new client for the specified provider
    pub fn new(provider: Provider, api_key: String) -> Self {
        Self {
            api_key,
            base_url: provider.base_url().to_string(),
            model: provider.default_model().to_string(),
            provider,
            client: reqwest::Client::new(),
        }
    }

    /// Create client with custom base URL (useful for testing or custom endpoints)
    pub fn with_base_url(provider: Provider, api_key: String, base_url: String) -> Self {
        Self {
            api_key,
            base_url,
            model: provider.default_model().to_string(),
            provider,
            client: reqwest::Client::new(),
        }
    }

    /// Set custom model
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }
}

#[async_trait]
impl LlmClient for OpenAiCompatibleClient {
    async fn generate_commit_message(
        &self,
        diff: &str,
        template: Option<&str>,
        emoji_guide: Option<&str>,
        selected_type: Option<(&str, &str, &str)>,
    ) -> Result<(String, String)> {
        let prompt = build_prompt(diff, template, emoji_guide, selected_type);

        let request = ChatRequest {
            model: self.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: "あなたは経験豊富なソフトウェアエンジニアです。Git diffから適切なコミットメッセージを生成してください。".to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: prompt,
                },
            ],
            temperature: 0.7,
        };

        let response = self
            .client
            .post(format!("{}/v1/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .with_context(|| format!("Failed to send request to {} API", self.provider))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "{} API request failed ({}): {}",
                self.provider,
                status,
                error_text
            ));
        }

        let api_response: ChatResponse = response
            .json()
            .await
            .with_context(|| format!("Failed to parse {} API response", self.provider))?;

        let message = api_response
            .choices
            .first()
            .context("No choices in API response")?
            .message
            .content
            .trim();

        Ok(parse_commit_message(message))
    }

    fn provider_name(&self) -> &str {
        match self.provider {
            Provider::OpenAi => "OpenAI",
            Provider::DeepSeek => "DeepSeek",
            _ => "Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn test_openai_generate_commit_message() {
        let mut server = Server::new_async().await;
        let _m = server
            .mock("POST", "/v1/chat/completions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "choices": [{
                    "message": {
                        "role": "assistant",
                        "content": "feat: Add user authentication\n\nImplemented JWT-based auth"
                    }
                }]
            }"#,
            )
            .create_async()
            .await;

        let client =
            OpenAiCompatibleClient::with_base_url(Provider::OpenAi, "test-key".into(), server.url());

        let (title, desc) = client
            .generate_commit_message("diff --git", None)
            .await
            .unwrap();

        assert_eq!(title, "feat: Add user authentication");
        assert_eq!(desc, "Implemented JWT-based auth");
    }

    #[tokio::test]
    async fn test_deepseek_generate_commit_message() {
        let mut server = Server::new_async().await;
        let _m = server
            .mock("POST", "/v1/chat/completions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "choices": [{
                    "message": {
                        "role": "assistant",
                        "content": "fix: Resolve memory leak"
                    }
                }]
            }"#,
            )
            .create_async()
            .await;

        let client = OpenAiCompatibleClient::with_base_url(
            Provider::DeepSeek,
            "test-key".into(),
            server.url(),
        );

        let (title, desc) = client
            .generate_commit_message("diff --git", None)
            .await
            .unwrap();

        assert_eq!(title, "fix: Resolve memory leak");
        assert_eq!(desc, "");
    }

    #[tokio::test]
    async fn test_api_error_handling() {
        let mut server = Server::new_async().await;
        let _m = server
            .mock("POST", "/v1/chat/completions")
            .with_status(401)
            .with_body(r#"{"error": "Invalid API key"}"#)
            .create_async()
            .await;

        let client =
            OpenAiCompatibleClient::with_base_url(Provider::OpenAi, "bad-key".into(), server.url());

        let result = client.generate_commit_message("diff", None).await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("401") || err.contains("API"));
    }

    #[test]
    fn test_provider_name() {
        let openai = OpenAiCompatibleClient::new(Provider::OpenAi, "key".into());
        assert_eq!(openai.provider_name(), "OpenAI");

        let deepseek = OpenAiCompatibleClient::new(Provider::DeepSeek, "key".into());
        assert_eq!(deepseek.provider_name(), "DeepSeek");
    }

    #[test]
    fn test_with_model() {
        let client = OpenAiCompatibleClient::new(Provider::OpenAi, "key".into())
            .with_model("gpt-4-turbo");
        assert_eq!(client.model, "gpt-4-turbo");
    }
}
