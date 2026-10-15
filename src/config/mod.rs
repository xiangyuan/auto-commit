use anyhow::{Context, Result};
use std::collections::HashMap;
use std::{fs, path::Path};

use crate::api::Provider;

/// Default .gitmessage template embedded at compile time
const DEFAULT_GITMESSAGE_TEMPLATE: &str = include_str!("../../docs/.gitmessage");

#[derive(Debug, Clone)]
pub struct Config {
    pub provider: Provider,
    pub api_key: String,
    /// Resolved model id. CLI `--model` wins over the provider model env var and the built-in default.
    pub model: String,
    pub gitmessage_template: String,
}

/// Load .gitmessage template from various locations
/// Search order:
/// 1. ~/.gitmessage (user custom) - highest priority
/// 2. ./.gitmessage (project root)
/// 3. Embedded default template (from docs/.gitmessage at build time)
pub fn load_gitmessage_template() -> String {
    let search_paths = [
        dirs::home_dir().map(|p| p.join(".gitmessage")),
        Some(Path::new(".gitmessage").to_path_buf()),
    ];

    for path_opt in search_paths.iter().flatten() {
        if path_opt.exists() {
            if let Ok(content) = fs::read_to_string(path_opt) {
                return content;
            }
        }
    }

    // Fall back to embedded default template
    DEFAULT_GITMESSAGE_TEMPLATE.to_string()
}

/// Parse emoji mappings from .gitmessage template content
/// Returns a formatted string with prefix -> emoji mappings
pub fn parse_emoji_mappings(template: &str) -> String {
    let mut mappings = Vec::new();

    for line in template.lines() {
        let line = line.trim();
        // Look for emoji mapping lines like: "#  🐛  :bug:           fix"
        if line.starts_with('#') && line.contains(':') {
            // Extract emoji and prefix
            if let Some(emoji) = extract_emoji_from_line(line) {
                if let Some(prefix) = extract_prefix_from_line(line) {
                    mappings.push((prefix, emoji));
                }
            }
        }
    }

    if mappings.is_empty() {
        return String::new();
    }

    let mut result = String::from("根据以下 emoji 映射选择合适的类型：\n");
    for (prefix, emoji) in mappings {
        result.push_str(&format!("- {}: {}\n", prefix, emoji));
    }

    result
}

fn extract_emoji_from_line(line: &str) -> Option<String> {
    // Find the first emoji character (after the initial #)
    let chars: Vec<char> = line.chars().collect();
    for c in chars.iter().skip(1) {
        if is_emoji_char(*c) {
            return Some(c.to_string());
        }
    }
    None
}

fn extract_prefix_from_line(line: &str) -> Option<String> {
    // Extract the last word in the line (the prefix)
    let parts: Vec<&str> = line.split_whitespace().collect();
    parts.last().map(|s| s.to_string())
}

fn is_emoji_char(c: char) -> bool {
    // Common emoji ranges
    matches!(c as u32,
        0x1F300..=0x1F9FF | // Misc Symbols and Pictographs, Emoticons, etc.
        0x2600..=0x26FF |   // Misc symbols
        0x2700..=0x27BF |   // Dingbats
        0x231A..=0x231B |   // Watch, Hourglass
        0x23E9..=0x23F3 |   // Media controls
        0x25AA..=0x25AB |   // Squares
        0x25B6 | 0x25C0 |   // Play buttons
        0x2B50 | 0x2B55     // Star, Circle
    )
}

impl Config {
    /// Load config from environment variables.
    ///
    /// Detects the provider from which API key is set.
    /// Priority: `OPENAI_API_KEY` > `DEEPSEEK_API_KEY` > `GEMINI_API_KEY` > `XAI_API_KEY`.
    pub fn from_env() -> Result<Self> {
        Self::from_env_for(None)
    }

    /// Load config from the environment for one provider, or auto-detect when `explicit` is `None`.
    pub fn from_env_for(explicit: Option<Provider>) -> Result<Self> {
        let (provider, api_key) =
            select_provider(explicit, |provider| provider.api_key_from_env())?;
        Ok(Self::assemble(provider, api_key, None))
    }

    /// Load config from a `.env` file.
    ///
    /// Supports multiple API keys with the same priority as [`Self::from_env`].
    pub fn from_env_file(path: &Path) -> Result<Self> {
        Self::from_env_file_for(path, None)
    }

    /// Load config from a `.env` file, optionally forcing one provider.
    pub fn from_env_file_for(path: &Path, explicit: Option<Provider>) -> Result<Self> {
        let content = fs::read_to_string(path).context("Failed to read .env file")?;
        let values = parse_env_assignments(&content);
        let (provider, api_key) = select_provider(explicit, |provider| {
            values
                .get(provider.api_key_var())
                .cloned()
                .filter(|key| !key.is_empty())
        })?;
        let file_model = values
            .get(provider.model_var())
            .cloned()
            .filter(|model| !model.is_empty());
        Ok(Self::assemble(provider, api_key, file_model))
    }

    /// Replace the resolved model id. An empty string leaves the current model in place.
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        let model = model.into();
        if !model.is_empty() {
            self.model = model;
        }
        self
    }

    fn assemble(provider: Provider, api_key: String, file_model: Option<String>) -> Self {
        let model = file_model.unwrap_or_else(|| provider.resolve_model());
        Self {
            provider,
            api_key,
            model,
            gitmessage_template: load_gitmessage_template(),
        }
    }
}

fn select_provider(
    explicit: Option<Provider>,
    mut lookup: impl FnMut(Provider) -> Option<String>,
) -> Result<(Provider, String)> {
    if let Some(provider) = explicit {
        let key = lookup(provider).with_context(|| {
            format!(
                "No API key found for {}. Set {}",
                provider.name(),
                provider.api_key_var()
            )
        })?;
        return Ok((provider, key));
    }

    for provider in Provider::priority() {
        if let Some(key) = lookup(provider) {
            return Ok((provider, key));
        }
    }

    anyhow::bail!(Provider::missing_key_message())
}

fn parse_env_assignments(content: &str) -> HashMap<String, String> {
    let mut values = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let line = line.strip_prefix("export ").unwrap_or(line);
        let Some((key, raw_value)) = line.split_once('=') else {
            continue;
        };
        let value = raw_value
            .trim()
            .trim_matches('\'')
            .trim_matches('"')
            .to_string();
        values.insert(key.trim().to_string(), value);
    }
    values
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn clear_env_keys() {
        env::remove_var("OPENAI_API_KEY");
        env::remove_var("DEEPSEEK_API_KEY");
        env::remove_var("GEMINI_API_KEY");
        env::remove_var("XAI_API_KEY");
        env::remove_var("OPENAI_MODEL");
        env::remove_var("DEEPSEEK_MODEL");
        env::remove_var("GEMINI_MODEL");
        env::remove_var("XAI_MODEL");
    }

    #[test]
    #[serial]
    fn test_config_from_env_openai() {
        clear_env_keys();
        env::set_var("OPENAI_API_KEY", "sk-openai-test");

        let config = Config::from_env().unwrap();

        assert_eq!(config.provider, Provider::OpenAi);
        assert_eq!(config.api_key, "sk-openai-test");

        clear_env_keys();
    }

    #[test]
    #[serial]
    fn test_config_from_env_deepseek() {
        clear_env_keys();
        env::set_var("DEEPSEEK_API_KEY", "sk-deepseek-test");

        let config = Config::from_env().unwrap();

        assert_eq!(config.provider, Provider::DeepSeek);
        assert_eq!(config.api_key, "sk-deepseek-test");

        clear_env_keys();
    }

    #[test]
    #[serial]
    fn test_config_from_env_gemini() {
        clear_env_keys();
        env::set_var("GEMINI_API_KEY", "AIza-gemini-test");

        let config = Config::from_env().unwrap();

        assert_eq!(config.provider, Provider::Gemini);
        assert_eq!(config.api_key, "AIza-gemini-test");

        clear_env_keys();
    }

    #[test]
    #[serial]
    fn test_config_from_env_grok() {
        clear_env_keys();
        env::set_var("XAI_API_KEY", "xai-test");

        let config = Config::from_env().unwrap();

        assert_eq!(config.provider, Provider::Grok);
        assert_eq!(config.api_key, "xai-test");
        assert_eq!(config.model, "grok-4.7");

        clear_env_keys();
    }

    #[test]
    #[serial]
    fn test_config_model_from_env() {
        clear_env_keys();
        env::set_var("XAI_API_KEY", "xai-test");
        env::set_var("XAI_MODEL", "grok-4.5");

        let config = Config::from_env().unwrap();

        assert_eq!(config.provider, Provider::Grok);
        assert_eq!(config.model, "grok-4.5");

        clear_env_keys();
    }

    #[test]
    #[serial]
    fn test_config_explicit_provider_ignores_higher_priority_key() {
        clear_env_keys();
        env::set_var("OPENAI_API_KEY", "openai");
        env::set_var("XAI_API_KEY", "xai-test");

        let config = Config::from_env_for(Some(Provider::Grok)).unwrap();

        assert_eq!(config.provider, Provider::Grok);
        assert_eq!(config.api_key, "xai-test");

        clear_env_keys();
    }

    #[test]
    #[serial]
    fn test_config_explicit_provider_missing_key() {
        clear_env_keys();
        env::set_var("OPENAI_API_KEY", "openai");

        let err = Config::from_env_for(Some(Provider::Grok))
            .unwrap_err()
            .to_string();
        assert!(err.contains("XAI_API_KEY"));

        clear_env_keys();
    }

    #[test]
    #[serial]
    fn test_config_with_model_overrides_env() {
        clear_env_keys();
        env::set_var("XAI_API_KEY", "xai-test");
        env::set_var("XAI_MODEL", "grok-4.5");

        let config = Config::from_env().unwrap().with_model("grok-4.6");

        assert_eq!(config.model, "grok-4.6");

        clear_env_keys();
    }

    #[test]
    #[serial]
    fn test_config_from_env_priority() {
        clear_env_keys();
        // Set all keys - OpenAI should win
        env::set_var("OPENAI_API_KEY", "openai");
        env::set_var("DEEPSEEK_API_KEY", "deepseek");
        env::set_var("GEMINI_API_KEY", "gemini");
        env::set_var("XAI_API_KEY", "xai");

        let config = Config::from_env().unwrap();

        assert_eq!(config.provider, Provider::OpenAi);
        assert_eq!(config.api_key, "openai");

        clear_env_keys();
    }

    #[test]
    #[serial]
    fn test_config_from_env_missing_key() {
        clear_env_keys();

        let result = Config::from_env();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("XAI_API_KEY"));
    }

    #[test]
    fn test_config_from_env_file_openai() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "OPENAI_API_KEY='sk-test123'").unwrap();

        let config = Config::from_env_file(temp_file.path()).unwrap();

        assert_eq!(config.provider, Provider::OpenAi);
        assert_eq!(config.api_key, "sk-test123");
    }

    #[test]
    fn test_config_from_env_file_deepseek() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "DEEPSEEK_API_KEY='sk-deepseek'").unwrap();

        let config = Config::from_env_file(temp_file.path()).unwrap();

        assert_eq!(config.provider, Provider::DeepSeek);
        assert_eq!(config.api_key, "sk-deepseek");
    }

    #[test]
    fn test_config_from_env_file_gemini() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "GEMINI_API_KEY='AIza-test'").unwrap();

        let config = Config::from_env_file(temp_file.path()).unwrap();

        assert_eq!(config.provider, Provider::Gemini);
        assert_eq!(config.api_key, "AIza-test");
    }

    #[test]
    fn test_config_from_env_file_grok() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "XAI_API_KEY='xai-test'").unwrap();
        writeln!(temp_file, "XAI_MODEL='grok-4.5'").unwrap();

        let config = Config::from_env_file(temp_file.path()).unwrap();

        assert_eq!(config.provider, Provider::Grok);
        assert_eq!(config.api_key, "xai-test");
        assert_eq!(config.model, "grok-4.5");
    }

    #[test]
    fn test_config_from_env_file_explicit_grok() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "OPENAI_API_KEY='openai'").unwrap();
        writeln!(temp_file, "XAI_API_KEY='xai-test'").unwrap();

        let config = Config::from_env_file_for(temp_file.path(), Some(Provider::Grok)).unwrap();

        assert_eq!(config.provider, Provider::Grok);
        assert_eq!(config.api_key, "xai-test");
    }

    #[test]
    fn test_config_from_env_file_with_export() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "export OPENAI_API_KEY='sk-test456'").unwrap();

        let config = Config::from_env_file(temp_file.path()).unwrap();

        assert_eq!(config.provider, Provider::OpenAi);
        assert_eq!(config.api_key, "sk-test456");
    }

    #[test]
    fn test_config_from_env_file_priority() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "GEMINI_API_KEY='gemini'").unwrap();
        writeln!(temp_file, "OPENAI_API_KEY='openai'").unwrap();
        writeln!(temp_file, "DEEPSEEK_API_KEY='deepseek'").unwrap();

        let config = Config::from_env_file(temp_file.path()).unwrap();

        // OpenAI should be selected (priority)
        assert_eq!(config.provider, Provider::OpenAi);
        assert_eq!(config.api_key, "openai");
    }

    #[test]
    fn test_load_gitmessage_template_default() {
        // Should always return a template (embedded default if no custom file)
        let template = load_gitmessage_template();

        // Verify embedded template contains expected content
        assert!(template.contains("Commit Message Template"));
        assert!(template.contains("feat:"));
        assert!(template.contains("fix:"));
    }

    #[test]
    fn test_default_template_is_embedded() {
        // Verify the default template is properly embedded at compile time
        assert!(!DEFAULT_GITMESSAGE_TEMPLATE.is_empty());
        assert!(DEFAULT_GITMESSAGE_TEMPLATE.contains("Prefix"));
        assert!(DEFAULT_GITMESSAGE_TEMPLATE.contains("Emojis"));
    }
}
