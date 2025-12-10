mod client;
mod gemini;
mod openai_compatible;
mod provider;

pub use client::LlmClient;
pub use gemini::GeminiClient;
pub use openai_compatible::OpenAiCompatibleClient;
pub use provider::Provider;

use anyhow::{Context, Result};

/// Create an LLM client based on available API keys
///
/// Priority order: OPENAI_API_KEY > DEEPSEEK_API_KEY > GEMINI_API_KEY
pub fn create_client() -> Result<Box<dyn LlmClient>> {
    let (provider, api_key) =
        Provider::detect().context("No API key found. Set OPENAI_API_KEY, DEEPSEEK_API_KEY, or GEMINI_API_KEY")?;

    Ok(create_client_for_provider(provider, api_key))
}

/// Create a client for a specific provider
pub fn create_client_for_provider(provider: Provider, api_key: String) -> Box<dyn LlmClient> {
    match provider {
        Provider::OpenAi | Provider::DeepSeek => {
            Box::new(OpenAiCompatibleClient::new(provider, api_key))
        }
        Provider::Gemini => Box::new(GeminiClient::new(api_key)),
    }
}

// Re-export for backward compatibility
pub use client::{build_prompt, parse_commit_message};

// Legacy types for backward compatibility (deprecated)
#[deprecated(since = "2.0.0", note = "Use OpenAiCompatibleClient or GeminiClient instead")]
pub type DeepSeekClient = OpenAiCompatibleClient;

#[deprecated(since = "2.0.0", note = "Use OpenAiCompatibleClient or GeminiClient instead")]
pub type DeepSeekRequest = ();

#[deprecated(since = "2.0.0", note = "Use OpenAiCompatibleClient or GeminiClient instead")]
pub type DeepSeekResponse = ();

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    fn clear_env_keys() {
        std::env::remove_var("OPENAI_API_KEY");
        std::env::remove_var("DEEPSEEK_API_KEY");
        std::env::remove_var("GEMINI_API_KEY");
    }

    #[test]
    #[serial]
    fn test_create_client_openai() {
        clear_env_keys();
        std::env::set_var("OPENAI_API_KEY", "sk-test");

        let client = create_client().unwrap();
        assert_eq!(client.provider_name(), "OpenAI");

        clear_env_keys();
    }

    #[test]
    #[serial]
    fn test_create_client_deepseek() {
        clear_env_keys();
        std::env::set_var("DEEPSEEK_API_KEY", "sk-deepseek");

        let client = create_client().unwrap();
        assert_eq!(client.provider_name(), "DeepSeek");

        clear_env_keys();
    }

    #[test]
    #[serial]
    fn test_create_client_gemini() {
        clear_env_keys();
        std::env::set_var("GEMINI_API_KEY", "AIza-test");

        let client = create_client().unwrap();
        assert_eq!(client.provider_name(), "Gemini");

        clear_env_keys();
    }

    #[test]
    #[serial]
    fn test_create_client_no_key() {
        clear_env_keys();

        let result = create_client();
        assert!(result.is_err());

        clear_env_keys();
    }

    #[test]
    fn test_create_client_for_provider() {
        let openai = create_client_for_provider(Provider::OpenAi, "key".into());
        assert_eq!(openai.provider_name(), "OpenAI");

        let deepseek = create_client_for_provider(Provider::DeepSeek, "key".into());
        assert_eq!(deepseek.provider_name(), "DeepSeek");

        let gemini = create_client_for_provider(Provider::Gemini, "key".into());
        assert_eq!(gemini.provider_name(), "Gemini");
    }
}
