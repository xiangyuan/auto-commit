use std::fmt;

/// Supported LLM providers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider {
    OpenAi,
    DeepSeek,
    Gemini,
}

impl Provider {
    /// Detect provider from environment variables (priority order)
    pub fn detect() -> Option<(Self, String)> {
        // Priority: OPENAI_API_KEY > DEEPSEEK_API_KEY > GEMINI_API_KEY
        if let Ok(key) = std::env::var("OPENAI_API_KEY") {
            if !key.is_empty() {
                return Some((Self::OpenAi, key));
            }
        }
        if let Ok(key) = std::env::var("DEEPSEEK_API_KEY") {
            if !key.is_empty() {
                return Some((Self::DeepSeek, key));
            }
        }
        if let Ok(key) = std::env::var("GEMINI_API_KEY") {
            if !key.is_empty() {
                return Some((Self::Gemini, key));
            }
        }
        None
    }

    /// Get the base URL for API requests
    pub fn base_url(&self) -> &'static str {
        match self {
            Self::OpenAi => "https://api.openai.com",
            Self::DeepSeek => "https://api.deepseek.com",
            Self::Gemini => "https://generativelanguage.googleapis.com",
        }
    }

    /// Get the default model for this provider
    pub fn default_model(&self) -> &'static str {
        match self {
            Self::OpenAi => "gpt-4o-mini",
            Self::DeepSeek => "deepseek-chat",
            Self::Gemini => "gemini-2.0-flash",
        }
    }

    /// Check if this provider uses OpenAI-compatible API
    pub fn is_openai_compatible(&self) -> bool {
        matches!(self, Self::OpenAi | Self::DeepSeek)
    }
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpenAi => write!(f, "OpenAI"),
            Self::DeepSeek => write!(f, "DeepSeek"),
            Self::Gemini => write!(f, "Gemini"),
        }
    }
}

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
    fn test_detect_openai() {
        clear_env_keys();
        std::env::set_var("OPENAI_API_KEY", "sk-test");

        let result = Provider::detect();
        assert!(result.is_some());
        let (provider, key) = result.unwrap();
        assert_eq!(provider, Provider::OpenAi);
        assert_eq!(key, "sk-test");

        clear_env_keys();
    }

    #[test]
    #[serial]
    fn test_detect_deepseek() {
        clear_env_keys();
        std::env::set_var("DEEPSEEK_API_KEY", "sk-deepseek");

        let result = Provider::detect();
        assert!(result.is_some());
        let (provider, key) = result.unwrap();
        assert_eq!(provider, Provider::DeepSeek);
        assert_eq!(key, "sk-deepseek");

        clear_env_keys();
    }

    #[test]
    #[serial]
    fn test_detect_gemini() {
        clear_env_keys();
        std::env::set_var("GEMINI_API_KEY", "AIza-test");

        let result = Provider::detect();
        assert!(result.is_some());
        let (provider, key) = result.unwrap();
        assert_eq!(provider, Provider::Gemini);
        assert_eq!(key, "AIza-test");

        clear_env_keys();
    }

    #[test]
    #[serial]
    fn test_detect_priority() {
        clear_env_keys();
        // Set all keys - OpenAI should win
        std::env::set_var("OPENAI_API_KEY", "openai-key");
        std::env::set_var("DEEPSEEK_API_KEY", "deepseek-key");
        std::env::set_var("GEMINI_API_KEY", "gemini-key");

        let result = Provider::detect();
        assert!(result.is_some());
        let (provider, _) = result.unwrap();
        assert_eq!(provider, Provider::OpenAi);

        clear_env_keys();
    }

    #[test]
    #[serial]
    fn test_detect_none() {
        clear_env_keys();

        let result = Provider::detect();
        assert!(result.is_none());
    }

    #[test]
    fn test_base_url() {
        assert_eq!(Provider::OpenAi.base_url(), "https://api.openai.com");
        assert_eq!(Provider::DeepSeek.base_url(), "https://api.deepseek.com");
        assert!(Provider::Gemini.base_url().contains("googleapis.com"));
    }

    #[test]
    fn test_openai_compatible() {
        assert!(Provider::OpenAi.is_openai_compatible());
        assert!(Provider::DeepSeek.is_openai_compatible());
        assert!(!Provider::Gemini.is_openai_compatible());
    }
}
