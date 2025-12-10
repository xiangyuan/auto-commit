# auto-commit

> AI 驱动的 Git 提交信息自动生成工具 - 支持 OpenAI / DeepSeek / Gemini

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/auto-commit.svg)](https://crates.io/crates/auto-commit)

🌐 **语言**: [日本語](../../README.md) | [English](README.en.md) | [繁體中文](README.zh-TW.md) | [Русский](README.ru.md) | [العربية](README.ar.md) | [فارسی](README.fa.md)

## 概述

`auto-commit` 是一个分析暂存更改并自动生成适当提交信息的 CLI 工具。本项目是 [m1guelpf/auto-commit](https://github.com/m1guelpf/auto-commit) 的分支版本，支持多个 LLM 提供商（OpenAI、DeepSeek、Gemini），并添加了可自定义的提交信息格式功能。

## 特性

- 🤖 **多提供商支持**：可选择 OpenAI、DeepSeek 或 Google Gemini
- 🎨 **自定义格式**：通过 `--format` 选项自由定制消息格式
- 📝 **.gitmessage 模板**：可设置项目特定的提交规则
- 🚀 **高性能**：基于 Rust 构建，轻量且快速
- 🔧 **灵活配置**：通过环境变量或 `.env` 文件管理配置
- 🌍 **多平台支持**：支持 Windows、macOS 和 Linux

## 安装方法

### Homebrew (macOS / Linux)

```bash
brew tap clearclown/tap
brew install auto-commit
```

### Cargo (Rust)

```bash
# 从 crates.io 安装
cargo install auto-commit

# 直接从 GitHub 仓库安装
cargo install --git https://github.com/clearclown/auto-commit.git
```

### 二进制下载

从 [GitHub Releases](https://github.com/clearclown/auto-commit/releases) 下载适合您平台的二进制文件：

| 平台 | 文件 |
|------|------|
| macOS (Apple Silicon) | `auto-commit-darwin-aarch64` |
| macOS (Intel) | `auto-commit-darwin-x86_64` |
| Linux (x86_64) | `auto-commit-linux-x86_64` |
| Linux (deb) | `auto-commit-linux-x86_64.deb` |
| Windows (x86_64) | `auto-commit-win-x86_64.exe` |

```bash
# 示例：macOS (Apple Silicon)
curl -LO https://github.com/clearclown/auto-commit/releases/latest/download/auto-commit-darwin-aarch64
chmod +x auto-commit-darwin-aarch64
sudo mv auto-commit-darwin-aarch64 /usr/local/bin/auto-commit
```

### 从源码构建

```bash
git clone https://github.com/clearclown/auto-commit.git
cd auto-commit
cargo build --release
sudo mv target/release/auto-commit /usr/local/bin/
```

## 设置

### API 密钥配置

将 LLM 提供商的 API 密钥设置为环境变量。如果设置了多个密钥，将按优先级自动选择。

**优先级**：`OPENAI_API_KEY` > `DEEPSEEK_API_KEY` > `GEMINI_API_KEY`

```bash
# 使用 OpenAI
export OPENAI_API_KEY='sk-...'

# 使用 DeepSeek
export DEEPSEEK_API_KEY='sk-...'

# 使用 Google Gemini
export GEMINI_API_KEY='AIza...'
```

或在项目根目录创建 `.env` 文件：

```bash
OPENAI_API_KEY='sk-...'
# DEEPSEEK_API_KEY='sk-...'
# GEMINI_API_KEY='AIza...'
```

## 使用方法

### 基本用法

```bash
# 暂存更改
git add .

# 自动生成并提交
auto-commit
```

执行时会显示正在使用的提供商：

```
⠋ Generating commit message using OpenAI...
✓ Commit message generated (OpenAI)
```

### 选项

```bash
# 演练运行（不实际提交）
auto-commit --dry-run

# 提交前查看生成的信息
auto-commit --review

# 强制执行（无需确认）
auto-commit --force

# 自定义格式
auto-commit --format "{emoji} {prefix}: {title}"

# 详细日志输出
auto-commit -v
```

### 格式占位符

| 占位符 | 说明 | 示例 |
|--------|------|------|
| `{title}` | 提交摘要（第一行） | `Add user authentication` |
| `{description}` | 详细描述 | `Implemented JWT-based auth...` |
| `{emoji}` | GitMoji 风格表情符号 | `✨`, `🐛`, `📝` |
| `{prefix}` | 约定式提交前缀 | `feat`, `fix`, `docs` |
| `{scope}` | 更改范围（可选） | `api`, `cli`, `config` |

## 支持的提供商

| 提供商 | 默认模型 | 环境变量 |
|--------|----------|----------|
| OpenAI | `gpt-4o-mini` | `OPENAI_API_KEY` |
| DeepSeek | `deepseek-chat` | `DEEPSEEK_API_KEY` |
| Google Gemini | `gemini-2.0-flash` | `GEMINI_API_KEY` |

## 许可证

MIT 许可证 - 详情请参阅 [LICENSE](../../LICENSE) 文件。

## 链接

- [GitHub 仓库](https://github.com/clearclown/auto-commit)
- [问题追踪器](https://github.com/clearclown/auto-commit/issues)
- [原始项目](https://github.com/m1guelpf/auto-commit)
