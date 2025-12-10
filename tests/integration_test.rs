//! Integration tests for multi-provider support
//!
//! These tests verify that:
//! 1. Provider detection works correctly
//! 2. API response parsing matches real API formats
//! 3. Error handling is consistent across providers

use auto_commit::api::{create_client, create_client_for_provider, LlmClient, Provider};
use serial_test::serial;

/// Clear all API key environment variables
fn clear_env_keys() {
    std::env::remove_var("OPENAI_API_KEY");
    std::env::remove_var("DEEPSEEK_API_KEY");
    std::env::remove_var("GEMINI_API_KEY");
}

mod provider_detection {
    use super::*;

    #[test]
    #[serial]
    fn test_auto_detect_openai() {
        clear_env_keys();
        std::env::set_var("OPENAI_API_KEY", "sk-openai-test");

        let client = create_client().expect("Should create client");
        assert_eq!(client.provider_name(), "OpenAI");

        clear_env_keys();
    }

    #[test]
    #[serial]
    fn test_auto_detect_deepseek() {
        clear_env_keys();
        std::env::set_var("DEEPSEEK_API_KEY", "sk-deepseek-test");

        let client = create_client().expect("Should create client");
        assert_eq!(client.provider_name(), "DeepSeek");

        clear_env_keys();
    }

    #[test]
    #[serial]
    fn test_auto_detect_gemini() {
        clear_env_keys();
        std::env::set_var("GEMINI_API_KEY", "AIza-test");

        let client = create_client().expect("Should create client");
        assert_eq!(client.provider_name(), "Gemini");

        clear_env_keys();
    }

    #[test]
    #[serial]
    fn test_priority_order() {
        clear_env_keys();
        // Set all keys - OpenAI should have highest priority
        std::env::set_var("OPENAI_API_KEY", "openai");
        std::env::set_var("DEEPSEEK_API_KEY", "deepseek");
        std::env::set_var("GEMINI_API_KEY", "gemini");

        let client = create_client().expect("Should create client");
        assert_eq!(client.provider_name(), "OpenAI");

        clear_env_keys();
    }

    #[test]
    #[serial]
    fn test_fallback_to_deepseek() {
        clear_env_keys();
        // Only DeepSeek and Gemini set
        std::env::set_var("DEEPSEEK_API_KEY", "deepseek");
        std::env::set_var("GEMINI_API_KEY", "gemini");

        let client = create_client().expect("Should create client");
        assert_eq!(client.provider_name(), "DeepSeek");

        clear_env_keys();
    }

    #[test]
    #[serial]
    fn test_no_key_error() {
        clear_env_keys();

        let result = create_client();
        assert!(result.is_err());
        // Just verify it errors, don't need to check the message
        clear_env_keys();
    }
}

mod client_factory {
    use super::*;

    #[test]
    fn test_create_openai_client() {
        let client = create_client_for_provider(Provider::OpenAi, "test-key".into());
        assert_eq!(client.provider_name(), "OpenAI");
    }

    #[test]
    fn test_create_deepseek_client() {
        let client = create_client_for_provider(Provider::DeepSeek, "test-key".into());
        assert_eq!(client.provider_name(), "DeepSeek");
    }

    #[test]
    fn test_create_gemini_client() {
        let client = create_client_for_provider(Provider::Gemini, "test-key".into());
        assert_eq!(client.provider_name(), "Gemini");
    }
}

mod provider_properties {
    use super::*;

    #[test]
    fn test_openai_base_url() {
        assert_eq!(Provider::OpenAi.base_url(), "https://api.openai.com");
    }

    #[test]
    fn test_deepseek_base_url() {
        assert_eq!(Provider::DeepSeek.base_url(), "https://api.deepseek.com");
    }

    #[test]
    fn test_gemini_base_url() {
        assert!(Provider::Gemini.base_url().contains("googleapis.com"));
    }

    #[test]
    fn test_default_models() {
        // These should be reasonable defaults
        assert!(Provider::OpenAi.default_model().contains("gpt"));
        assert!(Provider::DeepSeek.default_model().contains("deepseek"));
        assert!(Provider::Gemini.default_model().contains("gemini"));
    }

    #[test]
    fn test_openai_compatible_flag() {
        assert!(Provider::OpenAi.is_openai_compatible());
        assert!(Provider::DeepSeek.is_openai_compatible());
        assert!(!Provider::Gemini.is_openai_compatible());
    }
}

/// Tests that verify mock responses match expected API formats
///
/// These tests use mockito to ensure our parsing code handles
/// real API response structures correctly.
mod api_response_format {
    use super::*;

    /// Test that we correctly parse OpenAI's response format
    #[tokio::test]
    async fn test_openai_response_format() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("POST", "/v1/chat/completions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "id": "chatcmpl-abc123",
                    "object": "chat.completion",
                    "created": 1234567890,
                    "model": "gpt-4o-mini",
                    "choices": [{
                        "index": 0,
                        "message": {
                            "role": "assistant",
                            "content": "feat: Add user authentication\n\nImplemented JWT-based authentication"
                        },
                        "finish_reason": "stop"
                    }],
                    "usage": {
                        "prompt_tokens": 100,
                        "completion_tokens": 50,
                        "total_tokens": 150
                    }
                }"#,
            )
            .create_async()
            .await;

        let client = auto_commit::api::OpenAiCompatibleClient::with_base_url(
            Provider::OpenAi,
            "test-key".into(),
            server.url(),
        );

        let (title, desc) = client
            .generate_commit_message("diff content", None)
            .await
            .expect("Should parse response");

        assert_eq!(title, "feat: Add user authentication");
        assert_eq!(desc, "Implemented JWT-based authentication");
    }

    /// Test that we correctly parse Gemini's response format
    #[tokio::test]
    async fn test_gemini_response_format() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock(
                "POST",
                mockito::Matcher::Regex(r"/v1beta/models/.*:generateContent.*".to_string()),
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "candidates": [{
                        "content": {
                            "parts": [{
                                "text": "fix: Resolve null pointer exception\n\nFixed crash when user data is empty"
                            }],
                            "role": "model"
                        },
                        "finishReason": "STOP",
                        "index": 0
                    }],
                    "usageMetadata": {
                        "promptTokenCount": 100,
                        "candidatesTokenCount": 50,
                        "totalTokenCount": 150
                    }
                }"#,
            )
            .create_async()
            .await;

        let client =
            auto_commit::api::GeminiClient::with_base_url("test-key".into(), server.url());

        let (title, desc) = client
            .generate_commit_message("diff content", None)
            .await
            .expect("Should parse response");

        assert_eq!(title, "fix: Resolve null pointer exception");
        assert_eq!(desc, "Fixed crash when user data is empty");
    }
}

/// Tests for error handling across providers
mod error_handling {
    use super::*;

    #[tokio::test]
    async fn test_openai_unauthorized_error() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("POST", "/v1/chat/completions")
            .with_status(401)
            .with_body(r#"{"error": {"message": "Invalid API key", "type": "invalid_request_error"}}"#)
            .create_async()
            .await;

        let client = auto_commit::api::OpenAiCompatibleClient::with_base_url(
            Provider::OpenAi,
            "bad-key".into(),
            server.url(),
        );

        let result = client.generate_commit_message("diff", None).await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("401") || err.contains("API"));
    }

    #[tokio::test]
    async fn test_gemini_error() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock(
                "POST",
                mockito::Matcher::Regex(r"/v1beta/models/.*:generateContent.*".to_string()),
            )
            .with_status(400)
            .with_body(r#"{"error": {"code": 400, "message": "Invalid request"}}"#)
            .create_async()
            .await;

        let client =
            auto_commit::api::GeminiClient::with_base_url("test-key".into(), server.url());

        let result = client.generate_commit_message("diff", None).await;
        assert!(result.is_err());
    }
}
