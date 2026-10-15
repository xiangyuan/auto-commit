use std::fmt;

/// Supported LLM providers.
///
/// Automatic detection follows [`Provider::priority`]: OpenAI, DeepSeek, Gemini, then Grok.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider {
    OpenAi,
    DeepSeek,
    Gemini,
    Grok,
}

impl Provider {
    /// Detection order. The first provider with a non-empty API key wins.
    pub fn priority() -> [Self; 4] {
        [Self::OpenAi, Self::DeepSeek, Self::Gemini, Self::Grok]
    }

    /// Detect provider from environment variables (priority order).
    pub fn detect() -> Option<(Self, String)> {
        Self::priority()
            .into_iter()
            .find_map(|provider| provider.api_key_from_env().map(|key| (provider, key)))
    }

    /// Environment variable that holds this provider's API key.
    pub fn api_key_var(&self) -> &'static str {
        match self {
            Self::OpenAi => "OPENAI_API_KEY",
            Self::DeepSeek => "DEEPSEEK_API_KEY",
            Self::Gemini => "GEMINI_API_KEY",
            Self::Grok => "XAI_API_KEY",
        }
    }

    /// Environment variable that overrides this provider's model id.
    pub fn model_var(&self) -> &'static str {
        match self {
            Self::OpenAi => "OPENAI_MODEL",
            Self::DeepSeek => "DEEPSEEK_MODEL",
            Self::Gemini => "GEMINI_MODEL",
            Self::Grok => "XAI_MODEL",
        }
    }

    /// Display name used in logs and the spinner.
    pub fn name(&self) -> &'static str {
        match self {
            Self::OpenAi => "OpenAI",
            Self::DeepSeek => "DeepSeek",
            Self::Gemini => "Gemini",
            Self::Grok => "Grok",
        }
    }

    /// Read this provider's API key from the environment.
    pub fn api_key_from_env(&self) -> Option<String> {
        non_empty_var(self.api_key_var())
    }

    /// Model id from the provider model env var, otherwise [`Self::default_model`].
    pub fn resolve_model(&self) -> String {
        non_empty_var(self.model_var()).unwrap_or_else(|| self.default_model().to_string())
    }

    /// Message shown when no supported API key is set.
    pub fn missing_key_message() -> String {
        let keys: Vec<&str> = Self::priority().iter().map(Self::api_key_var).collect();
        format!("No API key found. Set {}", keys.join(", "))
    }

    /// Get the base URL for API requests.
    ///
    /// OpenAI-compatible clients append `/v1/chat/completions`.
    pub fn base_url(&self) -> &'static str {
        match self {
            Self::OpenAi => "https://api.openai.com",
            Self::DeepSeek => "https://api.deepseek.com",
            Self::Gemini => "https://generativelanguage.googleapis.com",
            Self::Grok => "https://api.x.ai",
        }
    }

    /// Built-in model id used when no model override is set.
    pub fn default_model(&self) -> &'static str {
        match self {
            Self::OpenAi => "gpt-4o-mini",
            Self::DeepSeek => "deepseek-chat",
            Self::Gemini => "gemini-2.0-flash",
            Self::Grok => "grok-4.7",
        }
    }

    /// Check if this provider uses an OpenAI-compatible chat completions API.
    pub fn is_openai_compatible(&self) -> bool {
        matches!(self, Self::OpenAi | Self::DeepSeek | Self::Grok)
    }
}

fn non_empty_var(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|value| !value.is_empty())
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
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
        std::env::remove_var("XAI_API_KEY");
        std::env::remove_var("OPENAI_MODEL");
        std::env::remove_var("DEEPSEEK_MODEL");
        std::env::remove_var("GEMINI_MODEL");
        std::env::remove_var("XAI_MODEL");
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
    fn test_detect_grok() {
        clear_env_keys();
        std::env::set_var("XAI_API_KEY", "xai-test");

        let result = Provider::detect();
        assert!(result.is_some());
        let (provider, key) = result.unwrap();
        assert_eq!(provider, Provider::Grok);
        assert_eq!(key, "xai-test");

        clear_env_keys();
    }

    #[test]
    #[serial]
    fn test_detect_grok_ignores_empty_key() {
        clear_env_keys();
        std::env::set_var("XAI_API_KEY", "");

        assert!(Provider::detect().is_none());

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
        std::env::set_var("XAI_API_KEY", "xai-key");

        let result = Provider::detect();
        assert!(result.is_some());
        let (provider, _) = result.unwrap();
        assert_eq!(provider, Provider::OpenAi);

        clear_env_keys();
    }

    #[test]
    #[serial]
    fn test_detect_grok_is_lowest_priority() {
        clear_env_keys();
        std::env::set_var("GEMINI_API_KEY", "gemini-key");
        std::env::set_var("XAI_API_KEY", "xai-key");

        let (provider, key) = Provider::detect().unwrap();
        assert_eq!(provider, Provider::Gemini);
        assert_eq!(key, "gemini-key");

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
        assert_eq!(Provider::Grok.base_url(), "https://api.x.ai");
    }

    #[test]
    fn test_default_model() {
        assert_eq!(Provider::Grok.default_model(), "grok-4.7");
    }

    #[test]
    #[serial]
    fn test_resolve_model_uses_xai_model_env() {
        clear_env_keys();
        std::env::set_var("XAI_MODEL", "grok-4.5");

        assert_eq!(Provider::Grok.resolve_model(), "grok-4.5");

        clear_env_keys();
    }

    #[test]
    #[serial]
    fn test_resolve_model_ignores_empty_override() {
        clear_env_keys();
        std::env::set_var("XAI_MODEL", "");

        assert_eq!(
            Provider::Grok.resolve_model(),
            Provider::Grok.default_model()
        );

        clear_env_keys();
    }

    #[test]
    fn test_openai_compatible() {
        assert!(Provider::OpenAi.is_openai_compatible());
        assert!(Provider::DeepSeek.is_openai_compatible());
        assert!(Provider::Grok.is_openai_compatible());
        assert!(!Provider::Gemini.is_openai_compatible());
    }

    #[test]
    fn test_grok_name() {
        assert_eq!(Provider::Grok.name(), "Grok");
        assert_eq!(Provider::Grok.api_key_var(), "XAI_API_KEY");
        assert_eq!(Provider::Grok.model_var(), "XAI_MODEL");
    }
}
