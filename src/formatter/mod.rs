use anyhow::Result;

pub struct CommitFormatter {
    format: Option<String>,
}

impl CommitFormatter {
    pub fn new(format: Option<String>) -> Self {
        Self { format }
    }

    pub fn format_message(&self, data: CommitData) -> Result<String> {
        let template = self.format.as_deref().unwrap_or(Self::get_default_format());
        
        let mut result = template.to_string();
        
        // Replace placeholders
        result = result.replace("{title}", &data.title);
        result = result.replace("{description}", &data.description);
        result = result.replace("{prefix}", data.prefix.as_deref().unwrap_or(""));
        result = result.replace("{emoji}", data.emoji.as_deref().unwrap_or(""));
        result = result.replace("{scope}", data.scope.as_deref().unwrap_or(""));
        
        Ok(result)
    }

    pub fn get_default_format() -> &'static str {
        "{title}\n\n{description}"
    }
}

#[derive(Debug, Clone)]
pub struct CommitData {
    pub title: String,
    pub description: String,
    pub prefix: Option<String>,
    pub emoji: Option<String>,
    pub scope: Option<String>,
}

impl CommitData {
    pub fn from_message(message: &str) -> Self {
        let parts: Vec<&str> = message.splitn(2, "\n\n").collect();
        let title = parts[0].to_string();
        let description = parts.get(1).unwrap_or(&"").to_string();
        
        // Parse prefix and emoji from title
        let (prefix, emoji) = Self::parse_title(&title);
        
        Self {
            title,
            description,
            prefix,
            emoji,
            scope: None,
        }
    }

    fn parse_title(title: &str) -> (Option<String>, Option<String>) {
        // Simple parser for conventional commit format
        // Examples: "feat: ✨ Add feature", "fix: Bug fix", "Update README"
        
        if let Some(colon_pos) = title.find(':') {
            let prefix = title[..colon_pos].trim().to_string();
            let rest = title[colon_pos + 1..].trim();
            
            // Check if the rest starts with an emoji
            let mut emoji = None;
            let mut chars = rest.chars();
            
            if let Some(first_char) = chars.next() {
                if is_emoji(first_char) {
                    emoji = Some(first_char.to_string());
                }
            }
            
            (Some(prefix), emoji)
        } else {
            (None, None)
        }
    }
}

fn is_emoji(c: char) -> bool {
    matches!(c, '✨' | '🐛' | '📝' | '💄' | '♻' | '🚀' | '💚' | '🍱' | '👍' | '🔥' | '🎨' | '⚙' | '🚧' | '💢' | '🆙' | '👮' | '👕' | '⏪' | '📛' | '🚨' | '💡')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_format() {
        let formatter = CommitFormatter::new(None);
        let data = CommitData {
            title: "feat: Add user authentication".to_string(),
            description: "Implemented JWT-based authentication".to_string(),
            prefix: Some("feat".to_string()),
            emoji: None,
            scope: None,
        };

        let result = formatter.format_message(data).unwrap();
        assert_eq!(result, "feat: Add user authentication\n\nImplemented JWT-based authentication");
    }

    #[test]
    fn test_custom_format_with_prefix() {
        let formatter = CommitFormatter::new(Some("{prefix}: {title}".to_string()));
        let data = CommitData {
            title: "Add user authentication".to_string(),
            description: "Implemented JWT-based authentication".to_string(),
            prefix: Some("feat".to_string()),
            emoji: None,
            scope: None,
        };

        let result = formatter.format_message(data).unwrap();
        assert_eq!(result, "feat: Add user authentication");
    }

    #[test]
    fn test_custom_format_with_emoji() {
        let formatter = CommitFormatter::new(Some("{prefix}: {emoji} {title}".to_string()));
        let data = CommitData {
            title: "Add user authentication".to_string(),
            description: "Implemented JWT-based authentication".to_string(),
            prefix: Some("feat".to_string()),
            emoji: Some("✨".to_string()),
            scope: None,
        };

        let result = formatter.format_message(data).unwrap();
        assert_eq!(result, "feat: ✨ Add user authentication");
    }

    #[test]
    fn test_custom_format_with_scope() {
        let formatter = CommitFormatter::new(Some("{prefix}({scope}): {title}".to_string()));
        let data = CommitData {
            title: "Add login endpoint".to_string(),
            description: "Added POST /api/login endpoint".to_string(),
            prefix: Some("feat".to_string()),
            emoji: None,
            scope: Some("api".to_string()),
        };

        let result = formatter.format_message(data).unwrap();
        assert_eq!(result, "feat(api): Add login endpoint");
    }

    #[test]
    fn test_format_with_missing_placeholders() {
        let formatter = CommitFormatter::new(Some("{prefix}: {emoji} {title}".to_string()));
        let data = CommitData {
            title: "Add user authentication".to_string(),
            description: "Implemented JWT-based authentication".to_string(),
            prefix: Some("feat".to_string()),
            emoji: None, // Missing emoji
            scope: None,
        };

        let result = formatter.format_message(data).unwrap();
        assert_eq!(result, "feat:  Add user authentication"); // Empty emoji
    }

    #[test]
    fn test_parse_title_with_prefix() {
        let data = CommitData::from_message("fix: Resolve memory leak\n\nFixed memory issue");
        assert_eq!(data.title, "fix: Resolve memory leak");
        assert_eq!(data.description, "Fixed memory issue");
        assert_eq!(data.prefix, Some("fix".to_string()));
    }

    #[test]
    fn test_parse_title_with_prefix_and_emoji() {
        let data = CommitData::from_message("feat: ✨ Add dark mode\n\nAdded theme switching");
        assert_eq!(data.title, "feat: ✨ Add dark mode");
        assert_eq!(data.prefix, Some("feat".to_string()));
        assert_eq!(data.emoji, Some("✨".to_string()));
    }

    #[test]
    fn test_parse_title_without_prefix() {
        let data = CommitData::from_message("Update README.md\n\nAdded installation instructions");
        assert_eq!(data.title, "Update README.md");
        assert_eq!(data.prefix, None);
        assert_eq!(data.emoji, None);
    }
}