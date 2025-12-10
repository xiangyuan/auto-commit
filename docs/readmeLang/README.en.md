# auto-commit

> AI-powered Git commit message generator - supports OpenAI / DeepSeek / Gemini

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/auto-commit.svg)](https://crates.io/crates/auto-commit)

🌐 **Languages**: [日本語](../../README.md) | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md) | [Русский](README.ru.md) | [العربية](README.ar.md) | [فارسی](README.fa.md)

## Overview

`auto-commit` is a CLI tool that analyzes staged changes and automatically generates appropriate commit messages. This project is a fork of [m1guelpf/auto-commit](https://github.com/m1guelpf/auto-commit), adding support for multiple LLM providers (OpenAI, DeepSeek, Gemini) and customizable commit message formatting.

## Features

- 🤖 **Multi-provider support**: Choose from OpenAI, DeepSeek, or Google Gemini
- 🎨 **Custom formatting**: Freely customize message format with `--format` option
- 📝 **.gitmessage templates**: Set project-specific commit rules
- 🚀 **Fast execution**: Built with Rust for lightweight and fast performance
- 🔧 **Flexible configuration**: Manage settings via environment variables or `.env` files
- 🌍 **Multi-platform**: Supports Windows, macOS, and Linux

## Installation

### Homebrew (macOS / Linux)

```bash
brew tap clearclown/tap
brew install auto-commit
```

### Cargo (Rust)

```bash
# From crates.io
cargo install auto-commit

# Directly from GitHub repository
cargo install --git https://github.com/clearclown/auto-commit.git
```

### Binary Download

Download binaries for your platform from [GitHub Releases](https://github.com/clearclown/auto-commit/releases):

| Platform | File |
|----------|------|
| macOS (Apple Silicon) | `auto-commit-darwin-aarch64` |
| macOS (Intel) | `auto-commit-darwin-x86_64` |
| Linux (x86_64) | `auto-commit-linux-x86_64` |
| Linux (deb) | `auto-commit-linux-x86_64.deb` |
| Windows (x86_64) | `auto-commit-win-x86_64.exe` |

```bash
# Example: macOS (Apple Silicon)
curl -LO https://github.com/clearclown/auto-commit/releases/latest/download/auto-commit-darwin-aarch64
chmod +x auto-commit-darwin-aarch64
sudo mv auto-commit-darwin-aarch64 /usr/local/bin/auto-commit
```

### Build from Source

```bash
git clone https://github.com/clearclown/auto-commit.git
cd auto-commit
cargo build --release
sudo mv target/release/auto-commit /usr/local/bin/
```

## Setup

### API Key Configuration

Set the API key for your LLM provider as an environment variable. If multiple keys are set, they are automatically selected by priority.

**Priority**: `OPENAI_API_KEY` > `DEEPSEEK_API_KEY` > `GEMINI_API_KEY`

```bash
# Use OpenAI
export OPENAI_API_KEY='sk-...'

# Use DeepSeek
export DEEPSEEK_API_KEY='sk-...'

# Use Google Gemini
export GEMINI_API_KEY='AIza...'
```

Or create a `.env` file in your project root:

```bash
OPENAI_API_KEY='sk-...'
# DEEPSEEK_API_KEY='sk-...'
# GEMINI_API_KEY='AIza...'
```

## Usage

### Basic Usage

```bash
# Stage changes
git add .

# Auto-generate and commit
auto-commit
```

The provider being used is displayed during execution:

```
⠋ Generating commit message using OpenAI...
✓ Commit message generated (OpenAI)
```

### Options

```bash
# Dry run (don't actually commit)
auto-commit --dry-run

# Review generated message before committing
auto-commit --review

# Force execution (no confirmation)
auto-commit --force

# Custom format
auto-commit --format "{emoji} {prefix}: {title}"

# Verbose logging
auto-commit -v
```

### Format Placeholders

| Placeholder | Description | Example |
|-------------|-------------|---------|
| `{title}` | Commit summary (first line) | `Add user authentication` |
| `{description}` | Detailed description | `Implemented JWT-based auth...` |
| `{emoji}` | GitMoji-style emoji | `✨`, `🐛`, `📝` |
| `{prefix}` | Conventional Commits prefix | `feat`, `fix`, `docs` |
| `{scope}` | Change scope (optional) | `api`, `cli`, `config` |

## Supported Providers

| Provider | Default Model | Environment Variable |
|----------|---------------|---------------------|
| OpenAI | `gpt-4o-mini` | `OPENAI_API_KEY` |
| DeepSeek | `deepseek-chat` | `DEEPSEEK_API_KEY` |
| Google Gemini | `gemini-2.0-flash` | `GEMINI_API_KEY` |

## License

MIT License - See [LICENSE](../../LICENSE) file for details.

## Links

- [GitHub Repository](https://github.com/clearclown/auto-commit)
- [Issue Tracker](https://github.com/clearclown/auto-commit/issues)
- [Original Project](https://github.com/m1guelpf/auto-commit)
