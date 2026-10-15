use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

use crate::api::Provider;

#[derive(Parser, Debug)]
#[command(version)]
#[command(name = "Auto Commit")]
#[command(author = "Miguel Piedrafita <soy@miguelpiedrafita.com>")]
#[command(about = "Automagically generate commit messages.", long_about = None)]
pub struct Cli {
    #[clap(flatten)]
    pub verbose: Verbosity<InfoLevel>,

    #[arg(
        long = "dry-run",
        help = "Output the generated message, but don't create a commit."
    )]
    pub dry_run: bool,

    #[arg(
        short,
        long,
        help = "Edit the generated commit message before committing."
    )]
    pub review: bool,

    #[arg(short, long, help = "Don't ask for confirmation before committing.")]
    pub force: bool,

    #[arg(
        long,
        help = "Custom message format with placeholders like {title}, {description}, {prefix}, {emoji}, {scope}"
    )]
    pub format: Option<String>,

    #[arg(
        long,
        value_parser = parse_provider,
        help = "LLM provider to use. Overrides automatic detection. One of: openai, deepseek, gemini, grok"
    )]
    pub provider: Option<Provider>,

    #[arg(
        long,
        help = "Model id override for the selected provider. Also reads OPENAI_MODEL, DEEPSEEK_MODEL, GEMINI_MODEL, or XAI_MODEL"
    )]
    pub model: Option<String>,
}

fn parse_provider(raw: &str) -> Result<Provider, String> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "openai" | "open-ai" => Ok(Provider::OpenAi),
        "deepseek" | "deep-seek" => Ok(Provider::DeepSeek),
        "gemini" => Ok(Provider::Gemini),
        "grok" | "xai" => Ok(Provider::Grok),
        other => Err(format!(
            "unknown provider '{other}'. expected openai, deepseek, gemini, or grok"
        )),
    }
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::Provider;

    #[test]
    fn test_parse_dry_run() {
        let args = vec!["auto-commit", "--dry-run"];
        let cli = Cli::parse_from(args);
        assert!(cli.dry_run);
        assert!(!cli.review);
        assert!(!cli.force);
        assert!(cli.format.is_none());
        assert!(cli.provider.is_none());
        assert!(cli.model.is_none());
    }

    #[test]
    fn test_parse_provider_and_model() {
        let cases = [
            ("openai", Provider::OpenAi),
            ("open-ai", Provider::OpenAi),
            ("deepseek", Provider::DeepSeek),
            ("deep-seek", Provider::DeepSeek),
            ("gemini", Provider::Gemini),
            ("grok", Provider::Grok),
            ("xai", Provider::Grok),
        ];

        for (name, expected) in cases {
            let cli =
                Cli::parse_from(["auto-commit", "--provider", name, "--model", "custom-model"]);
            assert_eq!(cli.provider, Some(expected), "{name}");
            assert_eq!(cli.model.as_deref(), Some("custom-model"));
        }
    }

    #[test]
    fn test_parse_unknown_provider() {
        let result = Cli::try_parse_from(["auto-commit", "--provider", "claude"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_review() {
        let args = vec!["auto-commit", "--review"];
        let cli = Cli::parse_from(args);
        assert!(!cli.dry_run);
        assert!(cli.review);
        assert!(!cli.force);
    }

    #[test]
    fn test_parse_force() {
        let args = vec!["auto-commit", "--force"];
        let cli = Cli::parse_from(args);
        assert!(!cli.dry_run);
        assert!(!cli.review);
        assert!(cli.force);
    }

    #[test]
    fn test_parse_format() {
        let args = vec!["auto-commit", "--format", "{prefix}: {title}"];
        let cli = Cli::parse_from(args);
        assert_eq!(cli.format, Some("{prefix}: {title}".to_string()));
    }

    #[test]
    fn test_parse_multiple_options() {
        let args = vec![
            "auto-commit",
            "--dry-run",
            "--force",
            "--format",
            "custom: {title}",
        ];
        let cli = Cli::parse_from(args);
        assert!(cli.dry_run);
        assert!(cli.force);
        assert_eq!(cli.format, Some("custom: {title}".to_string()));
    }

    #[test]
    fn test_parse_short_options() {
        let args = vec!["auto-commit", "-r", "-f"];
        let cli = Cli::parse_from(args);
        assert!(cli.review);
        assert!(cli.force);
    }

    #[test]
    fn test_verbosity_levels() {
        // Default verbosity
        let args = vec!["auto-commit"];
        let cli = Cli::parse_from(args);
        assert_eq!(cli.verbose.log_level(), Some(log::Level::Info));

        // Quiet mode
        let args = vec!["auto-commit", "-q"];
        let cli = Cli::parse_from(args);
        assert_eq!(cli.verbose.log_level(), Some(log::Level::Warn));

        // Very quiet mode
        let args = vec!["auto-commit", "-qq"];
        let cli = Cli::parse_from(args);
        assert_eq!(cli.verbose.log_level(), Some(log::Level::Error));

        // Verbose mode
        let args = vec!["auto-commit", "-v"];
        let cli = Cli::parse_from(args);
        assert_eq!(cli.verbose.log_level(), Some(log::Level::Debug));

        // Very verbose mode
        let args = vec!["auto-commit", "-vv"];
        let cli = Cli::parse_from(args);
        assert_eq!(cli.verbose.log_level(), Some(log::Level::Trace));
    }
}
