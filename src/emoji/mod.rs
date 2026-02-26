use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
struct EmojiEntry {
    emoji: String,
    prefixes: Vec<String>,
}

/// Parse .gitmessage file and extract emoji mappings
pub fn parse_gitmessage(path: &Path) -> Result<String> {
    let content = fs::read_to_string(path)?;
    extract_emoji_section(&content)
}

/// Extract emoji section from .gitmessage content
fn extract_emoji_section(content: &str) -> Result<String> {
    let mut emoji_guide = String::from("## Emoji 映射（基于 .gitmessage）\n");
    emoji_guide.push_str("请根据以下映射为 commit message 选择合适的 emoji：\n\n");

    let emoji_entries = extract_emoji_entries(content, None);
    if emoji_entries.is_empty() {
        // 如果没有找到映射，使用默认的
        emoji_guide.push_str(get_default_emoji_mappings());
    } else {
        for entry in emoji_entries {
            let joined_prefixes = entry.prefixes.join(" / ");
            emoji_guide.push_str(&format!(
                "- {} {} ({})\n",
                joined_prefixes, entry.emoji, entry.emoji
            ));
        }
    }

    Ok(emoji_guide)
}

/// Parse a single emoji mapping line
#[cfg(test)]
fn parse_emoji_line(line: &str) -> Option<String> {
    let cleaned = clean_template_line(line);
    let entry = parse_emoji_mapping_line(cleaned, None)?;
    let joined_prefixes = entry.prefixes.join(" / ");
    Some(format!(
        "{} {} ({})",
        joined_prefixes, entry.emoji, entry.emoji
    ))
}

/// Get default emoji mappings if .gitmessage parsing fails
fn get_default_emoji_mappings() -> &'static str {
    "- fix/hotfix: 🐛 Bug修复\n\
     - feat: ✨ 新功能\n\
     - docs: 📝 文档\n\
     - style: 💄 格式/样式\n\
     - refactor: ♻️ 重构\n\
     - perf: 🚀 性能优化\n\
     - test: 💚 测试\n\
     - chore: 🍱 构建/CI/依赖\n\
     - add/update/change: 👍 添加/更新\n\
     - remove: 🔥 删除\n\
     - upgrade: 🆙 版本升级\n\
     - revert: ⏪ 回滚\n\
     - rename: 📛 重命名\n\
     - disable: 🚧 临时禁用\n"
}

/// Extract commit type list from .gitmessage template for interactive selection
pub fn extract_commit_types(content: &str) -> Vec<(String, String, String)> {
    let prefix_entries = extract_prefix_entries(content);

    if !prefix_entries.is_empty() {
        let prefix_set: HashSet<String> = prefix_entries
            .iter()
            .map(|(prefix, _)| prefix.clone())
            .collect();
        let emoji_entries = extract_emoji_entries(content, Some(&prefix_set));
        let emoji_map = build_emoji_map(&emoji_entries);

        let mut types = Vec::with_capacity(prefix_entries.len());
        for (prefix, desc) in prefix_entries {
            let emoji = emoji_map
                .get(&prefix)
                .cloned()
                .or_else(|| default_emoji_for_prefix(&prefix).map(ToString::to_string))
                .unwrap_or_else(|| "🔧".to_string());
            types.push((prefix, emoji, desc));
        }

        if !types.is_empty() {
            return types;
        }
    }

    let emoji_entries = extract_emoji_entries(content, None);
    let mut types = build_types_from_emoji_entries(&emoji_entries);

    // 如果没有找到，返回默认类型
    if types.is_empty() {
        types = get_default_commit_types();
    }

    types
}

fn extract_prefix_entries(content: &str) -> Vec<(String, String)> {
    let mut entries = Vec::new();
    let mut seen = HashSet::new();
    let mut in_prefix_section = false;

    for raw_line in content.lines() {
        let line = clean_template_line(raw_line);
        if line.is_empty() || is_section_separator(line) {
            continue;
        }

        if is_prefix_section_header(line) {
            in_prefix_section = true;
            continue;
        }

        if in_prefix_section && is_section_header(line) && !is_prefix_section_header(line) {
            break;
        }

        if !in_prefix_section {
            continue;
        }

        if let Some((prefix, desc)) = parse_prefix_line(line) {
            if seen.insert(prefix.clone()) {
                entries.push((prefix, desc));
            }
        }
    }

    entries
}

fn extract_emoji_entries(
    content: &str,
    allowed_prefixes: Option<&HashSet<String>>,
) -> Vec<EmojiEntry> {
    let mut entries = Vec::new();
    let mut in_emoji_section = false;

    for raw_line in content.lines() {
        let line = clean_template_line(raw_line);
        if line.is_empty() || is_section_separator(line) {
            continue;
        }

        if is_emoji_section_header(line) {
            in_emoji_section = true;
            continue;
        }

        if in_emoji_section && is_section_header(line) && !is_emoji_section_header(line) {
            break;
        }

        if !in_emoji_section {
            continue;
        }

        if let Some(entry) = parse_emoji_mapping_line(line, allowed_prefixes) {
            entries.push(entry);
        }
    }

    entries
}

fn parse_prefix_line(line: &str) -> Option<(String, String)> {
    let (raw_prefix, raw_desc) = line.split_once(':')?;
    let prefix = normalize_prefix_token(raw_prefix)?;
    let desc = raw_desc.trim();

    if desc.is_empty() {
        return None;
    }

    Some((prefix, desc.to_string()))
}

fn parse_emoji_mapping_line(
    line: &str,
    allowed_prefixes: Option<&HashSet<String>>,
) -> Option<EmojiEntry> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }

    let emoji = parts[0];
    if !emoji.chars().next()?.is_emoji() {
        return None;
    }

    let rest_start = if parts.len() >= 3 && parts[1].starts_with(':') && parts[1].ends_with(':') {
        2
    } else {
        1
    };

    if rest_start >= parts.len() {
        return None;
    }

    let mapping_text = parts[rest_start..].join(" ");
    let prefixes = extract_prefix_candidates(&mapping_text, allowed_prefixes);
    if prefixes.is_empty() {
        return None;
    }

    Some(EmojiEntry {
        emoji: emoji.to_string(),
        prefixes,
    })
}

fn extract_prefix_candidates(
    text: &str,
    allowed_prefixes: Option<&HashSet<String>>,
) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    if let Some(allowed) = allowed_prefixes {
        for token in text.split(|c: char| c == '/' || c == ',' || c == '，' || c.is_whitespace()) {
            if let Some(prefix) = normalize_prefix_token(token) {
                if allowed.contains(&prefix) && seen.insert(prefix.clone()) {
                    result.push(prefix);
                }
            }
        }
        return result;
    }

    if text.contains('/') {
        for chunk in text.split('/') {
            if let Some(prefix) = chunk.split_whitespace().find_map(normalize_prefix_token) {
                if seen.insert(prefix.clone()) {
                    result.push(prefix);
                }
            }
        }
        return result;
    }

    if let Some(prefix) = text
        .split_whitespace()
        .filter_map(normalize_prefix_token)
        .last()
    {
        result.push(prefix);
    }

    result
}

fn build_emoji_map(entries: &[EmojiEntry]) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for entry in entries {
        for prefix in &entry.prefixes {
            map.entry(prefix.clone())
                .or_insert_with(|| entry.emoji.clone());
        }
    }

    map
}

fn build_types_from_emoji_entries(entries: &[EmojiEntry]) -> Vec<(String, String, String)> {
    let mut types = Vec::new();
    let mut seen = HashSet::new();

    for entry in entries {
        for prefix in &entry.prefixes {
            if seen.insert(prefix.clone()) {
                let desc = default_description_for_prefix(prefix)
                    .unwrap_or(prefix)
                    .to_string();
                types.push((prefix.clone(), entry.emoji.clone(), desc));
            }
        }
    }

    types
}

fn default_emoji_for_prefix(prefix: &str) -> Option<&'static str> {
    match prefix {
        "feat" => Some("✨"),
        "fix" | "hotfix" => Some("🐛"),
        "docs" => Some("📝"),
        "style" => Some("💄"),
        "refactor" => Some("♻️"),
        "perf" => Some("🚀"),
        "test" => Some("💚"),
        "chore" => Some("🍱"),
        "add" | "update" | "change" => Some("👍"),
        "remove" => Some("🔥"),
        "upgrade" => Some("🆙"),
        "revert" => Some("⏪"),
        "rename" => Some("📛"),
        "disable" => Some("🚧"),
        _ => None,
    }
}

fn default_description_for_prefix(prefix: &str) -> Option<&'static str> {
    match prefix {
        "feat" => Some("新功能"),
        "fix" => Some("Bug修复"),
        "hotfix" => Some("紧急修复"),
        "add" => Some("新增"),
        "update" => Some("更新"),
        "change" => Some("变更"),
        "docs" => Some("文档"),
        "style" => Some("样式"),
        "refactor" => Some("重构"),
        "perf" => Some("性能优化"),
        "test" => Some("测试"),
        "chore" => Some("构建/CI"),
        "disable" => Some("临时禁用"),
        "remove" => Some("删除"),
        "rename" => Some("重命名"),
        "upgrade" => Some("版本升级"),
        "revert" => Some("回滚"),
        _ => None,
    }
}

fn clean_template_line(line: &str) -> &str {
    line.trim().trim_start_matches('#').trim()
}

fn is_section_separator(line: &str) -> bool {
    !line.is_empty() && line.chars().all(|c| c == '-' || c == '=')
}

fn is_prefix_section_header(line: &str) -> bool {
    line.contains("前缀列表") || line.to_ascii_lowercase().contains("prefix list")
}

fn is_emoji_section_header(line: &str) -> bool {
    if line.contains("Emoji 速查") {
        return true;
    }

    let lower = line.to_ascii_lowercase();
    lower.contains("emoji list") || lower.contains("emojis list")
}

fn is_tips_header(line: &str) -> bool {
    line.contains("Tips") || line.contains("提示") || line.to_ascii_lowercase() == "tips"
}

fn is_section_header(line: &str) -> bool {
    is_prefix_section_header(line) || is_emoji_section_header(line) || is_tips_header(line)
}

fn normalize_prefix_token(token: &str) -> Option<String> {
    let normalized = token
        .trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '-' && c != '_')
        .to_ascii_lowercase();

    if normalized.is_empty() {
        return None;
    }

    if normalized
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        && normalized.chars().any(|c| c.is_ascii_alphabetic())
    {
        Some(normalized)
    } else {
        None
    }
}

/// Get default commit types
fn get_default_commit_types() -> Vec<(String, String, String)> {
    vec![
        ("feat".to_string(), "✨".to_string(), "新功能".to_string()),
        ("fix".to_string(), "🐛".to_string(), "Bug修复".to_string()),
        (
            "hotfix".to_string(),
            "🐛".to_string(),
            "紧急修复".to_string(),
        ),
        ("add".to_string(), "👍".to_string(), "新增".to_string()),
        ("update".to_string(), "👍".to_string(), "更新".to_string()),
        (
            "change".to_string(),
            "👍".to_string(),
            "规格变更".to_string(),
        ),
        ("docs".to_string(), "📝".to_string(), "文档".to_string()),
        ("style".to_string(), "💄".to_string(), "格式".to_string()),
        ("refactor".to_string(), "♻️".to_string(), "重构".to_string()),
        ("perf".to_string(), "🚀".to_string(), "性能优化".to_string()),
        ("test".to_string(), "💚".to_string(), "测试".to_string()),
        ("chore".to_string(), "🍱".to_string(), "构建/CI".to_string()),
        (
            "disable".to_string(),
            "🚧".to_string(),
            "临时禁用".to_string(),
        ),
        ("remove".to_string(), "🔥".to_string(), "删除".to_string()),
        ("rename".to_string(), "📛".to_string(), "重命名".to_string()),
        (
            "upgrade".to_string(),
            "🆙".to_string(),
            "版本升级".to_string(),
        ),
        ("revert".to_string(), "⏪".to_string(), "回滚".to_string()),
    ]
}

trait EmojiChar {
    fn is_emoji(&self) -> bool;
}

impl EmojiChar for char {
    fn is_emoji(&self) -> bool {
        matches!(*self as u32,
            0x1F000..=0x1FAFF | // Supplementary Symbols and Pictographs blocks
            0x2600..=0x26FF |   // Miscellaneous Symbols
            0x2700..=0x27BF |   // Dingbats
            0xFE00..=0xFE0F |   // Variation Selectors
            0x2300..=0x23FF     // Miscellaneous Technical
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_emoji_line() {
        let line = "#  🐛  :bug:           fix";
        let result = parse_emoji_line(line);
        assert!(result.is_some());
        let mapped = result.unwrap();
        assert!(mapped.contains("fix"));
        assert!(mapped.contains("🐛"));
    }

    #[test]
    fn test_extract_emoji_section() {
        let content = r#"
# Emoji 速查
#  🐛  :bug:           fix
#  ✨  :sparkles:      feat
# ----
"#;
        let result = extract_emoji_section(content).unwrap();
        assert!(result.contains("fix"));
        assert!(result.contains("feat"));
    }

    #[test]
    fn test_default_mappings() {
        let mappings = get_default_emoji_mappings();
        assert!(mappings.contains("fix"));
        assert!(mappings.contains("🐛"));
    }

    #[test]
    fn test_extract_commit_types_from_default_template() {
        let content = include_str!("../../docs/.gitmessage");
        let types = extract_commit_types(content);
        assert_eq!(types.len(), 17);
        assert!(types
            .iter()
            .any(|(prefix, emoji, _)| prefix == "hotfix" && emoji == "🐛"));
        assert!(types
            .iter()
            .any(|(prefix, emoji, _)| prefix == "upgrade" && emoji == "🆙"));
    }

    #[test]
    fn test_extract_commit_types_from_english_sections() {
        let content = r#"
# --------------------------------------------------------------------
# Prefix List
# --------------------------------------------------------------------
# feat: New feature
# fix: Bug fix
# add: Add files
# update: Update behavior
# change: Specification change
#
# --------------------------------------------------------------------
# Emojis List (Major Ones)
# --------------------------------------------------------------------
# ✨ :sparkles: feat
# 🐛 :bug: fix
# 👍 :+1: add / update / change
# "#;

        let types = extract_commit_types(content);
        assert_eq!(types.len(), 5);
        assert_eq!(types[0].0, "feat");
        assert_eq!(types[0].1, "✨");
        assert_eq!(types[2].0, "add");
        assert_eq!(types[2].1, "👍");
        assert_eq!(types[4].0, "change");
        assert_eq!(types[4].1, "👍");
    }
}
