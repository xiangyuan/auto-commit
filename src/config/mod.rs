use anyhow::{Context, Result};
use std::{env, fs, path::Path};

/// Default .gitmessage template embedded at compile time
const DEFAULT_GITMESSAGE_TEMPLATE: &str = include_str!("../../docs/.gitmessage");

#[derive(Debug, Clone)]
pub struct Config {
    pub deepseek_api_key: String,
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

impl Config {
    pub fn from_env() -> Result<Self> {
        let deepseek_api_key = env::var("DEEPSEEK_API_KEY")
            .context("DEEPSEEK_API_KEY environment variable not found")?;
        let gitmessage_template = load_gitmessage_template();

        Ok(Self { deepseek_api_key, gitmessage_template })
    }

    pub fn from_env_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .context("Failed to read .env file")?;

        let mut api_key = None;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some(key_value) = line.strip_prefix("DEEPSEEK_API_KEY=") {
                api_key = Some(key_value.trim_matches('\'').trim_matches('"').to_string());
            } else if let Some(key_value) = line.strip_prefix("export DEEPSEEK_API_KEY=") {
                api_key = Some(key_value.trim_matches('\'').trim_matches('"').to_string());
            }
        }

        let deepseek_api_key = api_key.context("DEEPSEEK_API_KEY not found in .env file")?;
        let gitmessage_template = load_gitmessage_template();

        Ok(Self { deepseek_api_key, gitmessage_template })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_config_from_env() {
        // Arrange
        env::set_var("DEEPSEEK_API_KEY", "test-key-123");

        // Act
        let config = Config::from_env().unwrap();

        // Assert
        assert_eq!(config.deepseek_api_key, "test-key-123");

        // Cleanup
        env::remove_var("DEEPSEEK_API_KEY");
    }

    #[test]
    fn test_config_from_env_missing_key() {
        // Arrange
        env::remove_var("DEEPSEEK_API_KEY");

        // Act
        let result = Config::from_env();

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_config_from_env_file() {
        // Arrange
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "DEEPSEEK_API_KEY='sk-test123'").unwrap();

        // Act
        let config = Config::from_env_file(temp_file.path()).unwrap();

        // Assert
        assert_eq!(config.deepseek_api_key, "sk-test123");
    }

    #[test]
    fn test_config_from_env_file_with_export() {
        // Arrange
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "export DEEPSEEK_API_KEY='sk-test456'").unwrap();

        // Act
        let config = Config::from_env_file(temp_file.path()).unwrap();

        // Assert
        assert_eq!(config.deepseek_api_key, "sk-test456");
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