use anyhow::Result;
use async_trait::async_trait;

/// Trait for LLM clients that can generate commit messages
#[async_trait]
pub trait LlmClient: Send + Sync {
    /// Generate a commit message from a git diff
    ///
    /// # Arguments
    /// * `diff` - The git diff to analyze
    /// * `template` - Optional template for the commit message format
    ///
    /// # Returns
    /// A tuple of (title, description) for the commit message
    async fn generate_commit_message(
        &self,
        diff: &str,
        template: Option<&str>,
    ) -> Result<(String, String)>;

    /// Get the provider name for display
    fn provider_name(&self) -> &str;
}

/// Build prompt for commit message generation
pub fn build_prompt(diff: &str, template: Option<&str>) -> String {
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

/// Parse commit message into title and description
pub fn parse_commit_message(message: &str) -> (String, String) {
    let parts: Vec<&str> = message.splitn(2, "\n\n").collect();
    let title = parts[0].trim().to_string();
    let description = parts.get(1).map(|s| s.trim()).unwrap_or("").to_string();
    (title, description)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_commit_message_with_description() {
        let message = "feat: Add new feature\n\nThis is the description";
        let (title, desc) = parse_commit_message(message);
        assert_eq!(title, "feat: Add new feature");
        assert_eq!(desc, "This is the description");
    }

    #[test]
    fn test_parse_commit_message_without_description() {
        let message = "fix: Quick fix";
        let (title, desc) = parse_commit_message(message);
        assert_eq!(title, "fix: Quick fix");
        assert_eq!(desc, "");
    }

    #[test]
    fn test_build_prompt_with_template() {
        let diff = "+added line";
        let template = "Custom template";
        let prompt = build_prompt(diff, Some(template));
        assert!(prompt.contains("Custom template"));
        assert!(prompt.contains("+added line"));
    }

    #[test]
    fn test_build_prompt_without_template() {
        let diff = "+added line";
        let prompt = build_prompt(diff, None);
        assert!(prompt.contains("使用可能なprefix"));
        assert!(prompt.contains("+added line"));
    }
}
