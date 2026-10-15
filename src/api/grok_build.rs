use std::time::Duration;

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use tokio::process::Command;

use super::client::{build_prompt, parse_commit_message, LlmClient};
use super::provider::Provider;

const DISALLOWED_TOOLS: &str = "run_terminal_cmd,run_terminal_command,write,search_replace,task,Agent,web_search,web_fetch,image_gen,image_edit";

/// Commit messages through the local `grok` CLI, which uses the signed-in Grok Build subscription.
#[derive(Debug, Clone)]
pub struct GrokBuildClient {
    bin: String,
    model: Option<String>,
}

impl GrokBuildClient {
    pub fn new(model: Option<String>) -> Self {
        let bin = std::env::var("GROK_BIN")
            .ok()
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "grok".to_string());
        Self { bin, model }
    }

    fn timeout() -> Duration {
        let seconds = std::env::var("GROK_BUILD_TIMEOUT_SECS")
            .ok()
            .and_then(|value| value.parse().ok())
            .filter(|seconds: &u64| *seconds > 0)
            .unwrap_or(180);
        Duration::from_secs(seconds)
    }
}

#[async_trait]
impl LlmClient for GrokBuildClient {
    async fn generate_commit_message(
        &self,
        diff: &str,
        template: Option<&str>,
        emoji_guide: Option<&str>,
        selected_type: Option<(&str, &str, &str)>,
    ) -> Result<(String, String)> {
        if !Provider::has_grok_session() {
            anyhow::bail!("Grok Build is not signed in. Run `grok login`.");
        }

        let prompt = commit_prompt(diff, template, emoji_guide, selected_type);
        let prompt_file =
            tempfile::NamedTempFile::new().context("Failed to create Grok prompt file")?;
        std::fs::write(prompt_file.path(), prompt).context("Failed to write Grok prompt file")?;

        let output = self.run_grok(prompt_file.path()).await?;
        let text = message_text(&output.stdout, &output.stderr, output.status.success())?;
        Ok(parse_commit_message(&text))
    }

    fn provider_name(&self) -> &str {
        Provider::Grok.name()
    }
}

fn commit_prompt(
    diff: &str,
    template: Option<&str>,
    emoji_guide: Option<&str>,
    selected_type: Option<(&str, &str, &str)>,
) -> String {
    format!(
        "Do not use tools. Do not edit files. Reply with only the commit message.\n\n{}",
        build_prompt(diff, template, emoji_guide, selected_type)
    )
}

struct GrokOutput {
    status: std::process::ExitStatus,
    stdout: String,
    stderr: String,
}

impl GrokBuildClient {
    async fn run_grok(&self, prompt_file: &std::path::Path) -> Result<GrokOutput> {
        let mut command = Command::new(&self.bin);
        command
            .arg("--prompt-file")
            .arg(prompt_file)
            .arg("--output-format")
            .arg("json")
            .arg("--verbatim")
            .arg("--max-turns")
            .arg("1")
            .arg("--no-subagents")
            .arg("--disable-web-search")
            .arg("--no-auto-update")
            .arg("--disallowed-tools")
            .arg(DISALLOWED_TOOLS)
            .arg("--system-prompt-override")
            .arg("You write git commit messages. Do not use tools. Reply with only the commit message.")
            .env_remove("XAI_API_KEY")
            .env("GROK_MEMORY", "0")
            .env("GROK_DISABLE_AUTOUPDATER", "1")
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true);
        if let Some(model) = &self.model {
            command.arg("-m").arg(model);
        }

        let child = command
            .spawn()
            .with_context(|| format!("Failed to start Grok Build CLI at {}", self.bin))?;
        let output = tokio::time::timeout(Self::timeout(), child.wait_with_output())
            .await
            .context("Grok Build timed out")?
            .context("Failed to read Grok Build output")?;

        Ok(GrokOutput {
            status: output.status,
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}

#[derive(Debug, Deserialize)]
struct HeadlessResponse {
    text: Option<String>,
    message: Option<String>,
    #[serde(rename = "type")]
    kind: Option<String>,
}

fn message_text(stdout: &str, stderr: &str, success: bool) -> Result<String> {
    let parsed = parse_headless(stdout);
    if !success {
        let detail = parsed
            .as_ref()
            .and_then(|response| response.message.clone())
            .filter(|message| !message.is_empty())
            .unwrap_or_else(|| stderr.trim().to_string());
        anyhow::bail!("Grok Build failed: {detail}");
    }
    if parsed
        .as_ref()
        .and_then(|response| response.kind.as_deref())
        == Some("error")
    {
        let detail = parsed
            .and_then(|response| response.message)
            .unwrap_or_else(|| "unknown error".to_string());
        anyhow::bail!("Grok Build failed: {detail}");
    }
    parsed
        .and_then(|response| response.text)
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty())
        .context("Grok Build returned an empty commit message")
}

fn parse_headless(stdout: &str) -> Option<HeadlessResponse> {
    let trimmed = stdout.trim();
    if let Ok(response) = serde_json::from_str::<HeadlessResponse>(trimmed) {
        return Some(response);
    }
    trimmed
        .lines()
        .rev()
        .find_map(|line| serde_json::from_str(line).ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;

    fn hide_grok_session() {
        let dir = std::env::temp_dir().join("auto-commit-no-grok-session");
        let _ = std::fs::create_dir_all(&dir);
        let _ = std::fs::remove_file(dir.join("auth.json"));
        std::env::set_var("GROK_HOME", &dir);
    }

    fn write_fake_grok(script: &str) -> (tempfile::TempDir, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("grok");
        std::fs::write(&path, script).unwrap();
        let mut permissions = std::fs::metadata(&path).unwrap().permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&path, permissions).unwrap();
        (dir, path)
    }

    #[tokio::test]
    #[serial]
    async fn test_grok_build_uses_subscription_cli() {
        hide_grok_session();
        let home = tempfile::tempdir().unwrap();
        std::fs::write(home.path().join("auth.json"), b"{}\n").unwrap();
        std::env::set_var("GROK_HOME", home.path());
        std::env::set_var("XAI_API_KEY", "should-not-be-forwarded");

        let args_file = home.path().join("args");
        let prompt_copy = home.path().join("prompt");
        let script = format!(
            "#!/bin/sh\nprintf '%s\\n' \"$@\" > '{}'\nif [ -n \"$XAI_API_KEY\" ]; then echo XAI_API_KEY_SET >> '{}'; fi\nprev=\nfor arg in \"$@\"; do\n  if [ \"$prev\" = \"--prompt-file\" ]; then cp \"$arg\" '{}'; fi\n  prev=\"$arg\"\ndone\nprintf '%s\\n' '{{\"text\":\"feat: Use Grok Build\\n\\nSubscription session\"}}'\n",
            args_file.display(),
            args_file.display(),
            prompt_copy.display()
        );
        let (bin_dir, bin) = write_fake_grok(&script);
        std::env::set_var("GROK_BIN", &bin);

        let client = GrokBuildClient::new(None);
        let (title, description) = client
            .generate_commit_message("diff --git a/file", None, None, None)
            .await
            .unwrap();

        assert_eq!(title, "feat: Use Grok Build");
        assert_eq!(description, "Subscription session");
        assert_eq!(client.provider_name(), "Grok Build");
        let args = std::fs::read_to_string(&args_file).unwrap();
        assert!(args.contains("--output-format\njson"));
        assert!(args.contains("--verbatim"));
        assert!(args.contains("--max-turns\n1"));
        assert!(!args.contains("--yolo"));
        assert!(!args.contains("XAI_API_KEY_SET"));
        assert!(!args.lines().any(|line| line == "-m"));
        let prompt = std::fs::read_to_string(prompt_copy).unwrap();
        assert!(prompt.contains("diff --git a/file"));
        assert!(prompt.contains("Do not use tools"));

        std::env::remove_var("GROK_BIN");
        std::env::remove_var("XAI_API_KEY");
        hide_grok_session();
        drop(bin_dir);
    }

    #[tokio::test]
    #[serial]
    async fn test_grok_build_passes_explicit_model() {
        hide_grok_session();
        let home = tempfile::tempdir().unwrap();
        std::fs::write(home.path().join("auth.json"), b"{}\n").unwrap();
        std::env::set_var("GROK_HOME", home.path());
        let args_file = home.path().join("args");
        let script = format!(
            "#!/bin/sh\nprintf '%s\\n' \"$@\" > '{}'\nprintf '%s\\n' '{{\"text\":\"fix: Model override\"}}'\n",
            args_file.display()
        );
        let (bin_dir, bin) = write_fake_grok(&script);
        std::env::set_var("GROK_BIN", &bin);

        let client = GrokBuildClient::new(Some("grok-4.5".into()));
        let (title, _) = client
            .generate_commit_message("diff", None, None, None)
            .await
            .unwrap();
        assert_eq!(title, "fix: Model override");
        let args = std::fs::read_to_string(args_file).unwrap();
        assert!(args.contains("-m\ngrok-4.5"));

        std::env::remove_var("GROK_BIN");
        hide_grok_session();
        drop(bin_dir);
    }

    #[tokio::test]
    #[serial]
    async fn test_grok_build_reports_cli_failure() {
        hide_grok_session();
        let home = tempfile::tempdir().unwrap();
        std::fs::write(home.path().join("auth.json"), b"{}\n").unwrap();
        std::env::set_var("GROK_HOME", home.path());
        let script = "#!/bin/sh\nprintf '%s\\n' '{\"type\":\"error\",\"message\":\"Not signed in\"}'\nexit 1\n";
        let (bin_dir, bin) = write_fake_grok(script);
        std::env::set_var("GROK_BIN", &bin);

        let error = GrokBuildClient::new(None)
            .generate_commit_message("diff", None, None, None)
            .await
            .unwrap_err()
            .to_string();
        assert!(error.contains("Not signed in"));

        std::env::remove_var("GROK_BIN");
        hide_grok_session();
        drop(bin_dir);
    }

    #[tokio::test]
    #[serial]
    async fn test_grok_build_requires_login() {
        hide_grok_session();
        let error = GrokBuildClient::new(None)
            .generate_commit_message("diff", None, None, None)
            .await
            .unwrap_err()
            .to_string();
        assert!(error.contains("grok login"));
    }
}
