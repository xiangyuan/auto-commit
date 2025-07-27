# auto-commit

> AI-Powered Git Commit Message Generator - DeepSeek API Edition

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)

## Overview

`auto-commit` is a CLI tool that analyzes staged changes and automatically generates appropriate commit messages. This project is a fork of [m1guelpf/auto-commit](https://github.com/m1guelpf/auto-commit) that replaces the backend from OpenAI to DeepSeek API and adds customizable commit message formatting features.

## Features

- 🤖 **DeepSeek API Integration**: High-quality commit message generation with coding-specialized LLM
- 🎨 **Custom Formatting**: Freely customizable message format with `--format` option
- 🚀 **High Performance**: Lightweight and fast execution built with Rust
- 🔧 **Flexible Configuration**: Configuration management via environment variables or `.env` files
- 🌍 **Multi-platform**: Supports Windows, macOS, and Linux

## Installation

### 1. Build from Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/clearclown/auto-commit.git
cd auto-commit

# Build
cargo build --release

# Move binary to PATH
sudo mv target/release/auto-commit /usr/local/bin/
```

### 2. Using Cargo

```bash
# Install directly from repository
cargo install --git https://github.com/clearclown/auto-commit.git
```

### 3. Manual Installation

```bash
# Clone the repository
git clone https://github.com/clearclown/auto-commit.git
cd auto-commit

# Release build
cargo build --release

# Copy executable to desired location
cp target/release/auto-commit ~/.local/bin/
# or
cp target/release/auto-commit ~/bin/

# Ensure PATH is set
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## Setup

### 1. Configure DeepSeek API Key

Set your DeepSeek API key as an environment variable:

```bash
export DEEPSEEK_API_KEY='sk-XXXXXXXXXXXXXXXX'
```

Or create a `.env` file in the project root:

```bash
echo 'DEEPSEEK_API_KEY=sk-XXXXXXXXXXXXXXXX' > ~/.env
```

### 2. Verify Installation

```bash
# Check version
auto-commit --version

# Show help
auto-commit --help
```

## Usage

### Basic Usage

```bash
# Stage changes
git add .

# Generate and execute commit message
auto-commit
```

### Options

```bash
# Dry run (don't actually commit)
auto-commit --dry-run

# Review generated message before committing
auto-commit --review

# Force execution (no confirmation)
auto-commit --force

# Specify custom format
auto-commit --format "{emoji} {prefix}: {title}"

# Verbose logging
auto-commit -v
```

### Format Placeholders

Available placeholders for custom formatting:

- `{title}` - Commit summary (first line)
- `{description}` - Detailed description
- `{emoji}` - GitMoji style emoji
- `{prefix}` - Conventional Commits prefix (feat, fix, etc.)
- `{scope}` - Change scope (optional)

### Usage Examples

```bash
# Conventional Commits format
auto-commit --format "{prefix}({scope}): {title}\n\n{description}"

# GitMoji format
auto-commit --format "{emoji} {title}\n\n{description}"

# Simple format
auto-commit --format "{title}"
```

## Developer Information

### Requirements

- Rust 1.70.0 or higher
- Git 2.0 or higher

### Build Instructions

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

### Project Structure

```
src/
├── main.rs         # Entry point
├── lib.rs          # Module exports
├── api/            # DeepSeek API integration
├── cli/            # CLI interface
├── config/         # Configuration management
├── formatter/      # Message formatter
└── git/            # Git operations
```

## Troubleshooting

### API Key Not Recognized

```bash
# Check environment variable
echo $DEEPSEEK_API_KEY

# Check .env file
cat ~/.env
```

### Commit Failures

```bash
# Check for staged changes
git status

# Verify Git configuration
git config user.name
git config user.email
```

### Build Errors

```bash
# Check Rust version
rustc --version

# Update dependencies
cargo update
```

## Contributing

1. Fork this repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (use `auto-commit`!)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Create a Pull Request

## License

MIT License - See [LICENSE](LICENSE) file for details.

## Acknowledgments

- Original [auto-commit](https://github.com/m1guelpf/auto-commit) project
- [DeepSeek](https://www.deepseek.com/) team
- All contributors

## Links

- [GitHub Repository](https://github.com/clearclown/auto-commit)
- [Issue Tracker](https://github.com/clearclown/auto-commit/issues)
- [Original Project](https://github.com/m1guelpf/auto-commit)
- [DeepSeek API Documentation](https://api.deepseek.com/docs)