use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct DeepSeekRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct DeepSeekResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: Message,
}

#[derive(Debug, Clone)]
pub struct DeepSeekClient {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

const DEFAULT_PROMPT_TEMPLATE: &str = r#"以下のGit diffを分析して、適切なコミットメッセージを生成してください。

コミットメッセージは以下のフォーマットで生成してください：
1行目: コミットタイトル（50文字以内、prefix: を含む）
2行目: 空行
3行目以降: 詳細説明（必要に応じて）

使用可能なprefix:
- feat: 新機能
- fix: バグ修正
- docs: ドキュメント
- style: フォーマット
- refactor: リファクタリング
- test: テスト
- chore: ビルド/CI"#;

impl DeepSeekClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.deepseek.com".to_string(),
            client: reqwest::Client::new(),
        }
    }

    fn build_prompt(&self, diff: &str, template: Option<&str>) -> String {
        let rules = template.unwrap_or(DEFAULT_PROMPT_TEMPLATE);

        format!(
            "以下のGit diffを分析して、適切なコミットメッセージを生成してください。\n\n\
            ## コミットメッセージのルール\n\
            {}\n\n\
            ## 重要な指示\n\
            - コミットメッセージのみを出力してください（説明や補足は不要）\n\
            - 1行目はタイトル、空行を挟んで本文を記述\n\
            - prefixとemojiを適切に選択してください\n\n\
            ## Git diff\n\
            ```\n{}\n```",
            rules, diff
        )
    }

    pub async fn generate_commit_message(
        &self,
        diff: &str,
        template: Option<&str>,
    ) -> Result<(String, String)> {
        let prompt = self.build_prompt(diff, template);

        let request = DeepSeekRequest {
            model: "deepseek-chat".to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "あなたは経験豊富なソフトウェアエンジニアです。Git diffから適切なコミットメッセージを生成してください。".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt,
                },
            ],
            temperature: 0.7,
        };

        let response = self.client
            .post(format!("{}/v1/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to DeepSeek API")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("API request failed: {}", error_text));
        }

        let api_response: DeepSeekResponse = response
            .json()
            .await
            .context("Failed to parse DeepSeek API response")?;

        let message = api_response.choices
            .first()
            .context("No choices in API response")?
            .message
            .content
            .trim();

        // Parse the message into title and description
        let parts: Vec<&str> = message.splitn(2, "\n\n").collect();
        let title = parts[0].to_string();
        let description = parts.get(1).unwrap_or(&"").to_string();

        Ok((title, description))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn test_generate_commit_message_success() {
        // Arrange
        let mut server = Server::new_async().await;
        let _m = server
            .mock("POST", "/v1/chat/completions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "choices": [{
                    "message": {
                        "role": "assistant",
                        "content": "feat: Add user authentication\n\nImplemented JWT-based authentication system with login and logout functionality"
                    }
                }]
            }"#)
            .create_async()
            .await;

        let client = DeepSeekClient {
            api_key: "test-key".to_string(),
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let diff = "diff --git a/src/auth.rs b/src/auth.rs\n+pub fn login() {}";

        // Act
        let (title, description) = client.generate_commit_message(diff, None).await.unwrap();

        // Assert
        assert_eq!(title, "feat: Add user authentication");
        assert_eq!(description, "Implemented JWT-based authentication system with login and logout functionality");
    }

    #[tokio::test]
    async fn test_generate_commit_message_api_error() {
        // Arrange
        let mut server = Server::new_async().await;
        let _m = server
            .mock("POST", "/v1/chat/completions")
            .with_status(401)
            .with_body(r#"{"error": "Invalid API key"}"#)
            .create_async()
            .await;

        let client = DeepSeekClient {
            api_key: "invalid-key".to_string(),
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let diff = "some diff";

        // Act
        let result = client.generate_commit_message(diff, None).await;

        // Assert
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_parse_commit_message() {
        // Test parsing of commit message into title and description
        let message = "fix: Resolve memory leak in cache manager\n\nFixed the issue where cache entries were not properly cleaned up after expiration";
        
        let (title, description) = parse_commit_message(message);
        
        assert_eq!(title, "fix: Resolve memory leak in cache manager");
        assert_eq!(description, "Fixed the issue where cache entries were not properly cleaned up after expiration");
    }

    fn parse_commit_message(message: &str) -> (String, String) {
        let parts: Vec<&str> = message.splitn(2, "\n\n").collect();
        let title = parts[0].to_string();
        let description = parts.get(1).unwrap_or(&"").to_string();
        (title, description)
    }
}