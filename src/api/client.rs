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
    /// * `emoji_guide` - Optional emoji mapping guide from .gitmessage
    /// * `selected_type` - Optional tuple of (prefix, emoji, description) for commit type
    ///
    /// # Returns
    /// A tuple of (title, description) for the commit message
    async fn generate_commit_message(
        &self,
        diff: &str,
        template: Option<&str>,
        emoji_guide: Option<&str>,
        selected_type: Option<(&str, &str, &str)>,
    ) -> Result<(String, String)>;

    /// Get the provider name for display
    fn provider_name(&self) -> &str;
}

/// Truncate diff if it exceeds maximum length
/// Keeps important parts at the beginning and end
fn truncate_diff(diff: &str, max_chars: usize) -> String {
    if diff.len() <= max_chars {
        return diff.to_string();
    }
    
    // Keep first 60% and last 20% of the allowed space, with a truncation notice
    let truncation_notice = "\n\n... [diff truncated due to length] ...\n\n";
    let available_chars = max_chars.saturating_sub(truncation_notice.len());
    let first_part_size = (available_chars as f64 * 0.6) as usize;
    let last_part_size = (available_chars as f64 * 0.2) as usize;
    
    let first_part = &diff[..first_part_size.min(diff.len())];
    let last_part = if diff.len() > first_part_size + last_part_size {
        &diff[diff.len().saturating_sub(last_part_size)..]
    } else {
        ""
    };
    
    format!("{}{}{}", first_part, truncation_notice, last_part)
}

/// Build prompt for commit message generation
pub fn build_prompt(
    diff: &str,
    template: Option<&str>,
    emoji_guide: Option<&str>,
    selected_type: Option<(&str, &str, &str)>,
) -> String {
    let rules = template.unwrap_or(DEFAULT_PROMPT_TEMPLATE);
    
    // Detect language from template
    let lang = detect_template_language(rules);
    
    let emoji_section = if let Some(guide) = emoji_guide {
        format!("\n\n{}", guide)
    } else {
        String::new()
    };
    
    // Truncate diff to avoid token limit issues
    // DeepSeek has 131,072 token limit; using ~300,000 chars (~75k tokens) for diff
    // to leave room for system message, instructions, and completion
    let truncated_diff = truncate_diff(diff, 300_000);

    let type_instruction = if let Some((prefix, emoji, desc)) = selected_type {
        match lang {
            Language::Chinese => format!(
                "\n\n## ⚠️ 必须使用的 Commit 类型（用户已选择）\n\
                **prefix**: {}\n\
                **emoji**: {}\n\
                **说明**: {}\n\n\
                ⚠️ 重要：无论 diff 内容是什么，你必须使用上述用户选择的 prefix 和 emoji！\n\
                生成的标题必须严格遵循此格式：\n\
                {}(scope): {} 摘要说明\n\n\
                示例：如果是修改配置文件，也要用：{}(config): {} 修改配置项",
                prefix, emoji, desc, prefix, emoji, prefix, emoji
            ),
            Language::Japanese => format!(
                "\n\n## ⚠️ 必須の Commit タイプ（ユーザー選択済み）\n\
                **prefix**: {}\n\
                **emoji**: {}\n\
                **説明**: {}\n\n\
                ⚠️ 重要：diff の内容に関わらず、上記で選択された prefix と emoji を必ず使用してください！\n\
                タイトルは必ず以下の形式に従ってください：\n\
                {}(scope): {} 概要説明\n\n\
                例：設定ファイルを変更する場合でも：{}(config): {} 設定項目を修正",
                prefix, emoji, desc, prefix, emoji, prefix, emoji
            ),
            Language::English => format!(
                "\n\n## ⚠️ Required Commit Type (User Selected)\n\
                **prefix**: {}\n\
                **emoji**: {}\n\
                **description**: {}\n\n\
                ⚠️ Important: You MUST use the prefix and emoji selected by the user above, regardless of the diff content!\n\
                The title must strictly follow this format:\n\
                {}(scope): {} summary description\n\n\
                Example: Even if modifying config files, use: {}(config): {} Update configuration",
                prefix, emoji, desc, prefix, emoji, prefix, emoji
            ),
        }
    } else {
        String::new()
    };

    let (intro, rule_header, instruction_header, instruction_items, diff_header) = match lang {
        Language::Chinese => (
            "请分析以下 Git diff 并生成合适的提交信息。",
            "## 提交信息规则",
            if selected_type.is_some() { "## 最重要的指示（必须遵守）" } else { "## 重要指示" },
            if selected_type.is_some() {
                "- ⚠️ 必须使用上面指定的 prefix 和 emoji（与 diff 内容无关）\n\
                - 只输出提交信息（不要说明或补充）\n\
                - 第1行是标题，空1行后写正文\n\
                - 标题格式：指定的 prefix(scope): 指定的 emoji 摘要"
            } else {
                "- 只输出提交信息（不要说明或补充）\n\
                - 第1行是标题，空1行后写正文\n\
                - 适当选择 prefix 和 emoji\n\
                - 标题格式：prefix(scope): emoji 摘要（例：feat(api): ✨ 添加用户认证）"
            },
            "## Git diff"
        ),
        Language::Japanese => (
            "以下のGit diffを分析して、適切なコミットメッセージを生成してください。",
            "## コミットメッセージのルール",
            if selected_type.is_some() { "## 最重要な指示（必ず守ること）" } else { "## 重要な指示" },
            if selected_type.is_some() {
                "- ⚠️ 必ず上記で指定された prefix と emoji を使用してください（diff の内容は関係ありません）\n\
                - コミットメッセージのみを出力してください（説明や補足は不要）\n\
                - 1行目はタイトル、空行を挟んで本文を記述\n\
                - タイトル形式：指定された prefix(scope): 指定された emoji 摘要"
            } else {
                "- コミットメッセージのみを出力してください（説明や補足は不要）\n\
                - 1行目はタイトル、空行を挟んで本文を記述\n\
                - prefixとemojiを適切に選択してください\n\
                - タイトル形式：prefix(scope): emoji 摘要（例：feat(api): ✨ ユーザー認証を追加）"
            },
            "## Git diff"
        ),
        Language::English => (
            "Please analyze the following Git diff and generate an appropriate commit message.",
            "## Commit Message Rules",
            if selected_type.is_some() { "## Most Important Instructions (Must Follow)" } else { "## Important Instructions" },
            if selected_type.is_some() {
                "- ⚠️ You MUST use the prefix and emoji specified above (regardless of diff content)\n\
                - Output only the commit message (no explanations or supplements)\n\
                - Line 1 is the title, blank line, then body\n\
                - Title format: specified prefix(scope): specified emoji summary"
            } else {
                "- Output only the commit message (no explanations or supplements)\n\
                - Line 1 is the title, blank line, then body\n\
                - Choose appropriate prefix and emoji\n\
                - Title format: prefix(scope): emoji summary (e.g., feat(api): ✨ Add user authentication)"
            },
            "## Git diff"
        ),
    };

    format!(
        "{}\n\n\
        {}\n\
        {}{}{}\n\n\
        {}\n\
        {}\n\n\
        {}\n\
        ```\n{}\n```",
        intro, rule_header, rules, emoji_section, type_instruction, 
        instruction_header, instruction_items, diff_header, truncated_diff
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Language {
    Chinese,
    Japanese,
    English,
}

/// Detect the primary language of the template
fn detect_template_language(template: &str) -> Language {
    let chinese_chars = template.chars().filter(|c| is_chinese(*c)).count();
    let japanese_chars = template.chars().filter(|c| is_japanese(*c)).count();
    let english_words = template.split_whitespace()
        .filter(|w| w.chars().all(|c| c.is_ascii_alphabetic()))
        .count();
    
    // Prioritize based on character count
    if chinese_chars > japanese_chars && chinese_chars > english_words {
        Language::Chinese
    } else if japanese_chars > chinese_chars && japanese_chars > english_words {
        Language::Japanese
    } else {
        Language::English
    }
}

fn is_chinese(c: char) -> bool {
    matches!(c as u32, 0x4E00..=0x9FFF)
}

fn is_japanese(c: char) -> bool {
    matches!(c as u32, 
        0x3040..=0x309F | // Hiragana
        0x30A0..=0x30FF   // Katakana
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
        let prompt = build_prompt(diff, Some(template), None, None);
        assert!(prompt.contains("Custom template"));
        assert!(prompt.contains("+added line"));
    }

    #[test]
    fn test_build_prompt_without_template() {
        let diff = "+added line";
        let prompt = build_prompt(diff, None, None, None);
        assert!(prompt.contains("使用可能なprefix"));
        assert!(prompt.contains("+added line"));
    }
    
    #[test]
    fn test_truncate_diff_short() {
        let diff = "+short diff";
        let result = truncate_diff(diff, 1000);
        assert_eq!(result, "+short diff");
    }
    
    #[test]
    fn test_truncate_diff_long() {
        let diff = "a".repeat(100000);
        let result = truncate_diff(&diff, 1000);
        assert!(result.len() <= 1100); // Allow small margin for notice
        assert!(result.contains("[diff truncated due to length]"));
    }
    
    #[test]
    fn test_truncate_diff_preserves_beginning_and_end() {
        let diff = format!("{}middle{}", "start".repeat(10000), "end".repeat(10000));
        let result = truncate_diff(&diff, 10000);
        assert!(result.starts_with("start"));
        assert!(result.ends_with("end"));
        assert!(result.contains("[diff truncated due to length]"));
    }
}
