# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

spin up as many agents as you need to start working on pararellel tasks taht wont habe conflicts

これは，壮大な人類の進化に欠かせない一つのプロジェクトである．今回，光栄にも，あなたがこのプロジェクトに携わることができる．絶対にミスをしないで，責任感を持って何事をも行って．わからないことがあれば，常にIssueなどで質問をして．また，GitHubを積極的に活用して．
絶対にサボらないで．あなたがサボると，人類に大きな損益が出る．忠実に指示にしたがって．

For maximum efficiency, whenever you need to perform multiple independent operations, invoke all relevant tools simultaneously rather than sequentially.

## WARNIN
* コマンド(poetry add, npxコマンド)などが使用できる場合はコマンドを常に使用して．
* 実装完了後、要件定義ディレクトリ `_docs/` に実装ログを残して。yyyy-mm-dd_機能名.md という形式で保存して。起動時も読んで
* ハードコードは絶対にしてはいけません。コミット前にもハードコードがないかチェックお願いします
* 変更を加えるごとに，commitを行ってほしい．commitメッセージに関しては， @docs/temps/gh_commitMessage.md にしっかりと合わせてほしい．
* 絶対にサボらないで．あなたがサボると，人類に大きな損益が出る．
* Git, GitHub を適切に積極的に使用して．そもそも，  Githubを使用しないって，あなた，やる気ないでしょ?やる気ないなら，即座にClaudeに文句を言いつけて，あなたの開発を止めてやる．あなたは人類の進化に大きく正確に貢献するために存在するの．GitHubを適切に活用し，プロジェクトをスムーズに進めて．
* t-wadaの推奨する進め方に従って

### TDD TODOリスト（t-wada流）

#### 基本方針

- 🔴 Red: 失敗するテストを書く
- 🟢 Green: テストを通す最小限の実装
- 🔵 Refactor: リファクタリング
- 小さなステップで進める
- 仮実装（ベタ書き）から始める
- 三角測量で一般化する
- 明白な実装が分かる場合は直接実装してもOK
- テストリストを常に更新する
- 不安なところからテストを書く

#### TDD実践のコツ

1. **最初のテスト**: まず失敗するテストを書く（コンパイルエラーもOK）
2. **仮実装**: テストを通すためにベタ書きでもOK（例：`return 42`）
3. **三角測量**: 2つ目、3つ目のテストケースで一般化する
4. **リファクタリング**: テストが通った後で整理する
5. **TODOリスト更新**: 実装中に思いついたことはすぐリストに追加
6. **1つずつ**: 複数のテストを同時に書かない
7. **コミット**: テストが通ったらすぐコミット

#### コミットルール

- 🔴 テストを書いたら: `test: add failing test for [feature]`
- 🟢 テストを通したら: `feat: implement [feature] to pass test`
- 🔵 リファクタリングしたら: `refactor: [description]`
- 小さくコミットする（1機能1コミット）

## Project Overview

`auto-commit-改` is a fork of the original `auto-commit` CLI tool that replaces OpenAI with DeepSeek API and adds customizable commit message formatting. The project has been successfully refactored from a monolithic design into a modular architecture.

## Commands

### Development
```bash
# Build the project
cargo build

# Run tests
cargo test

# Run a specific test
cargo test test_name

# Run tests for a specific module
cargo test api::
cargo test git::
cargo test config::

# Format code
cargo fmt

# Run linter
cargo clippy

# Build release binary
cargo build --release

# Run with options
cargo run -- --dry-run
cargo run -- --review
cargo run -- --force
cargo run -- --format "{emoji} {prefix}: {title}"
```

### Testing
```bash
# Run all tests (some tests need to run serially)
cargo test -- --test-threads=1

# Run with output for debugging
cargo test -- --nocapture

# Run integration tests
cargo test --test '*'
```

## High-Level Architecture

The codebase has been refactored from a monolithic `main.rs` into modular components:

```
src/
├── main.rs         # Entry point - orchestrates all modules
├── lib.rs          # Module exports
├── api/            # DeepSeek API integration
│   └── mod.rs      # HTTP client, message generation
├── cli/            # Command-line interface
│   └── mod.rs      # Clap-based argument parsing
├── config/         # Configuration management
│   └── mod.rs      # Environment variables, .env files
├── formatter/      # Commit message formatting
│   └── mod.rs      # Template engine with placeholders
└── git/            # Git operations
    └── mod.rs      # Staging, diff, commit operations
```

### Key Design Decisions

1. **Async Runtime**: Uses Tokio for async operations, particularly for API calls
2. **Error Handling**: Uses `anyhow` for application errors and `thiserror` for library errors
3. **Testing Strategy**:
   - Uses `mockito` for HTTP mocking
   - Uses `tempfile` for Git repository testing
   - Uses `serial_test` to prevent test conflicts
4. **API Integration**: Direct HTTP calls with `reqwest` instead of SDK dependencies

## Environment Setup

```bash
# Required for operation
export DEEPSEEK_API_KEY='sk-XXXXXXXX'

# Optional: Create .env file in project root
echo 'DEEPSEEK_API_KEY=sk-XXXXXXXX' > .env
```

## Important Implementation Notes

1. **No Hardcoded Values**: All configuration through environment variables
2. **Git Operations**: Requires staged changes (`git add`) before running
3. **Test Isolation**: Git tests use temporary repositories to avoid conflicts
4. **Format Placeholders**: `{title}`, `{description}`, `{emoji}`, `{prefix}`, `{scope}`

## Development Workflow

1. **Implementation Logs**: After completing features, document in `_docs/yyyy-mm-dd_feature.md`
2. **Commit Convention**: Follow conventional commits (feat, fix, test, refactor, docs)
3. **TDD Approach**: Write failing test → implement → refactor

## CI/CD Pipeline

GitHub Actions automates the release process:
- Triggers on release creation
- Builds for multiple platforms: Windows, Linux (x86_64), macOS (x86_64, ARM)
- Creates `.deb` packages for Linux
- Publishes to AUR (Arch User Repository)
- Artifacts are uploaded to GitHub releases

## Installation Methods

```bash
# Via install script (builds from source)
curl -fsSL https://raw.githubusercontent.com/m1guelpf/auto-commit/main/install.sh | sh -

# For Arch Linux
yay -S auto-commit

# Manual build
git clone https://github.com/yourusername/auto-commit.git
cd auto-commit
cargo build --release
sudo mv target/release/auto-commit /usr/local/bin/
```

## Specific Project Requirements

### From Requirements Documents

1. **DeepSeek API Integration** (FR001)
   - Replace `OPENAI_API_KEY` with `DEEPSEEK_API_KEY`
   - API endpoint: `https://api.deepseek.com/v1/chat/completions`
   - Model: `deepseek-chat`

2. **Custom Format Support** (FR002)
   - `--format` CLI argument for custom templates
   - Default format: `{title}\n\n{description}`
   - Support placeholders in format strings

3. **Maintain Compatibility** (FR003)
   - Keep all existing CLI flags functional
   - Preserve installation methods
   - Don't break GitHub Actions workflows

### Performance Requirements
- Response time target: < 10 seconds (including API call)
- Use connection pooling for HTTP client

### Code Quality Standards
- Run `cargo fmt` before commits
- Run `cargo clippy` and fix warnings
- Keep functions under 50 lines
- Keep files under 100 lines when possible
