# auto-commit

> AI 驅動的 Git 提交訊息自動產生工具 - 支援 OpenAI / DeepSeek / Gemini

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/auto-commit.svg)](https://crates.io/crates/auto-commit)

🌐 **語言**: [日本語](../../README.md) | [English](README.en.md) | [简体中文](README.zh-CN.md) | [Русский](README.ru.md) | [العربية](README.ar.md) | [فارسی](README.fa.md)

## 概述

`auto-commit` 是一個分析暫存變更並自動產生適當提交訊息的 CLI 工具。本專案是 [m1guelpf/auto-commit](https://github.com/m1guelpf/auto-commit) 的分支版本，支援多個 LLM 提供者（OpenAI、DeepSeek、Gemini），並新增了可自訂的提交訊息格式功能。

## 特色

- 🤖 **多提供者支援**：可選擇 OpenAI、DeepSeek 或 Google Gemini
- 🎨 **自訂格式**：透過 `--format` 選項自由定制訊息格式
- 📝 **.gitmessage 範本**：可設定專案特定的提交規則
- 🚀 **高效能**：基於 Rust 建構，輕量且快速
- 🔧 **彈性配置**：透過環境變數或 `.env` 檔案管理設定
- 🌍 **多平台支援**：支援 Windows、macOS 和 Linux

## 安裝方法

### Homebrew (macOS / Linux)

```bash
brew tap clearclown/tap
brew install auto-commit
```

### Cargo (Rust)

```bash
# 從 crates.io 安裝
cargo install auto-commit

# 直接從 GitHub 儲存庫安裝
cargo install --git https://github.com/clearclown/auto-commit.git
```

### 二進位下載

從 [GitHub Releases](https://github.com/clearclown/auto-commit/releases) 下載適合您平台的二進位檔案：

| 平台 | 檔案 |
|------|------|
| macOS (Apple Silicon) | `auto-commit-darwin-aarch64` |
| macOS (Intel) | `auto-commit-darwin-x86_64` |
| Linux (x86_64) | `auto-commit-linux-x86_64` |
| Linux (deb) | `auto-commit-linux-x86_64.deb` |
| Windows (x86_64) | `auto-commit-win-x86_64.exe` |

```bash
# 範例：macOS (Apple Silicon)
curl -LO https://github.com/clearclown/auto-commit/releases/latest/download/auto-commit-darwin-aarch64
chmod +x auto-commit-darwin-aarch64
sudo mv auto-commit-darwin-aarch64 /usr/local/bin/auto-commit
```

### 從原始碼建構

```bash
git clone https://github.com/clearclown/auto-commit.git
cd auto-commit
cargo build --release
sudo mv target/release/auto-commit /usr/local/bin/
```

## 設定

### API 金鑰配置

將 LLM 提供者的 API 金鑰設定為環境變數。如果設定了多個金鑰，將按優先順序自動選擇。

**優先順序**：`OPENAI_API_KEY` > `DEEPSEEK_API_KEY` > `GEMINI_API_KEY`

```bash
# 使用 OpenAI
export OPENAI_API_KEY='sk-...'

# 使用 DeepSeek
export DEEPSEEK_API_KEY='sk-...'

# 使用 Google Gemini
export GEMINI_API_KEY='AIza...'
```

或在專案根目錄建立 `.env` 檔案：

```bash
OPENAI_API_KEY='sk-...'
# DEEPSEEK_API_KEY='sk-...'
# GEMINI_API_KEY='AIza...'
```

## 使用方法

### 基本用法

```bash
# 暫存變更
git add .

# 自動產生並提交
auto-commit
```

執行時會顯示正在使用的提供者：

```
⠋ Generating commit message using OpenAI...
✓ Commit message generated (OpenAI)
```

### 選項

```bash
# 演練運行（不實際提交）
auto-commit --dry-run

# 提交前查看產生的訊息
auto-commit --review

# 強制執行（無需確認）
auto-commit --force

# 自訂格式
auto-commit --format "{emoji} {prefix}: {title}"

# 詳細日誌輸出
auto-commit -v
```

### 格式佔位符

| 佔位符 | 說明 | 範例 |
|--------|------|------|
| `{title}` | 提交摘要（第一行） | `Add user authentication` |
| `{description}` | 詳細描述 | `Implemented JWT-based auth...` |
| `{emoji}` | GitMoji 風格表情符號 | `✨`, `🐛`, `📝` |
| `{prefix}` | 約定式提交前綴 | `feat`, `fix`, `docs` |
| `{scope}` | 變更範圍（可選） | `api`, `cli`, `config` |

## 支援的提供者

| 提供者 | 預設模型 | 環境變數 |
|--------|----------|----------|
| OpenAI | `gpt-4o-mini` | `OPENAI_API_KEY` |
| DeepSeek | `deepseek-chat` | `DEEPSEEK_API_KEY` |
| Google Gemini | `gemini-2.0-flash` | `GEMINI_API_KEY` |

## 授權條款

MIT 授權條款 - 詳情請參閱 [LICENSE](../../LICENSE) 檔案。

## 連結

- [GitHub 儲存庫](https://github.com/clearclown/auto-commit)
- [問題追蹤器](https://github.com/clearclown/auto-commit/issues)
- [原始專案](https://github.com/m1guelpf/auto-commit)
