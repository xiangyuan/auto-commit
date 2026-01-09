use anyhow::Result;
use std::fs;
use std::path::Path;

/// Parse .gitmessage file and extract emoji mappings
pub fn parse_gitmessage(path: &Path) -> Result<String> {
    let content = fs::read_to_string(path)?;
    extract_emoji_section(&content)
}

/// Extract emoji section from .gitmessage content
fn extract_emoji_section(content: &str) -> Result<String> {
    let mut emoji_guide = String::from("## Emoji 映射（基于 .gitmessage）\n");
    emoji_guide.push_str("请根据以下映射为 commit message 选择合适的 emoji：\n\n");
    
    let lines: Vec<&str> = content.lines().collect();
    let mut in_emoji_section = false;
    let mut found_mappings = false;
    
    for line in lines {
        // 检测 emoji 速查区域的开始
        if line.contains("Emoji 速查") || line.contains("Emoji") && line.contains("----") {
            in_emoji_section = true;
            continue;
        }
        
        // 检测区域结束
        if in_emoji_section && line.trim().starts_with("# ----") && found_mappings {
            break;
        }
        
        // 在 emoji 区域内提取映射
        if in_emoji_section && !line.trim().is_empty() && !line.trim().starts_with("#") {
            if let Some(mapping) = parse_emoji_line(line) {
                emoji_guide.push_str(&format!("- {}\n", mapping));
                found_mappings = true;
            }
        }
    }
    
    if !found_mappings {
        // 如果没有找到映射，使用默认的
        emoji_guide.push_str(get_default_emoji_mappings());
    }
    
    Ok(emoji_guide)
}

/// Parse a single emoji mapping line
fn parse_emoji_line(line: &str) -> Option<String> {
    let line = line.trim().trim_start_matches('#').trim();
    
    // 格式: 🐛  :bug:           fix
    // 或: 🐛  :bug:           fix (描述)
    if line.is_empty() {
        return None;
    }
    
    // 提取 emoji、code 和 prefix
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 3 {
        let emoji = parts[0];
        let joined = parts[2..].join(" ");
        let prefix = joined.split('(').next()?.trim();
        
        // 验证是否是 emoji 字符
        if emoji.chars().next()?.is_emoji() {
            return Some(format!("{} {} ({})", prefix, emoji, emoji));
        }
    }
    
    None
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
    let mut types = Vec::new();
    
    let lines: Vec<&str> = content.lines().collect();
    let mut in_emoji_section = false;
    
    for line in lines {
        // 检测 emoji 速查区域
        if line.contains("Emoji 速查") || (line.contains("Emoji") && line.contains("----")) {
            in_emoji_section = true;
            continue;
        }
        
        // 检测区域结束
        if in_emoji_section && line.trim().starts_with("# ----") && !types.is_empty() {
            break;
        }
        
        // 解析 emoji 映射行
        if in_emoji_section {
            if let Some((prefix, emoji, desc)) = parse_type_line(line) {
                types.push((prefix, emoji, desc));
            }
        }
    }
    
    // 如果没有找到，返回默认类型
    if types.is_empty() {
        types = get_default_commit_types();
    }
    
    types
}

/// Parse a line to extract (prefix, emoji, description)
fn parse_type_line(line: &str) -> Option<(String, String, String)> {
    let line = line.trim().trim_start_matches('#').trim();
    
    if line.is_empty() {
        return None;
    }
    
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 3 {
        let emoji = parts[0];
        
        // 验证是否是 emoji 字符
        if emoji.chars().next()?.is_emoji() {
            let rest = parts[2..].join(" ");
            let prefix = rest.split_whitespace().last()?.to_string();
            let description = rest.replace(&prefix, "").trim().to_string();
            
            return Some((prefix, emoji.to_string(), description));
        }
    }
    
    None
}

/// Get default commit types
fn get_default_commit_types() -> Vec<(String, String, String)> {
    vec![
        ("feat".to_string(), "✨".to_string(), "新功能".to_string()),
        ("fix".to_string(), "🐛".to_string(), "Bug修复".to_string()),
        ("docs".to_string(), "📝".to_string(), "文档".to_string()),
        ("style".to_string(), "💄".to_string(), "格式".to_string()),
        ("refactor".to_string(), "♻️".to_string(), "重构".to_string()),
        ("perf".to_string(), "🚀".to_string(), "性能优化".to_string()),
        ("test".to_string(), "💚".to_string(), "测试".to_string()),
        ("chore".to_string(), "🍱".to_string(), "构建/CI".to_string()),
        ("add".to_string(), "👍".to_string(), "添加".to_string()),
        ("update".to_string(), "👍".to_string(), "更新".to_string()),
        ("remove".to_string(), "🔥".to_string(), "删除".to_string()),
    ]
}

trait EmojiChar {
    fn is_emoji(&self) -> bool;
}

impl EmojiChar for char {
    fn is_emoji(&self) -> bool {
        matches!(*self as u32,
            0x1F300..=0x1F9FF | // Miscellaneous Symbols and Pictographs, Emoticons, Transport and Map, etc.
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
        assert!(result.unwrap().contains("fix"));
        assert!(result.unwrap().contains("🐛"));
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
}
