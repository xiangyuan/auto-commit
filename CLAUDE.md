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

`auto-commit-改` is a forked and enhanced version of the original `auto-commit` CLI tool. This project replaces OpenAI with DeepSeek API for commit message generation and adds customizable message formatting capabilities.

### Project Goals (from 企画書)
- Replace OpenAI dependency with DeepSeek API for better coding-focused LLM performance
- Add flexible commit message format customization
- Refactor `main.rs` to improve maintainability and code quality
- Maintain compatibility with existing installation methods and build processes

## Key Architecture

- **Single-file application**: Currently all logic in `src/main.rs` (planned refactoring into modules)
- **Async runtime**: Uses Tokio for async operations
- **LLM Integration**: Migrating from `async-openai` to DeepSeek API using `reqwest`
- **CLI interface**: Built with Clap for command-line argument parsing

## Common Commands

### Development
```bash
# Build the project
cargo build

# Run in development
cargo run

# Build release binary
cargo build --release

# Run with options
cargo run -- --dry-run
cargo run -- --review
cargo run -- --force
```

### Installation
```bash
# Install via curl script
curl -fsSL https://raw.githubusercontent.com/m1guelpf/auto-commit/main/install.sh | sh -

# Or for Arch Linux users
yay -S auto-commit
```

## Environment Setup

The tool requires a DeepSeek API key:
```bash
export DEEPSEEK_API_KEY='sk-XXXXXXXX'
```

## Important Notes

- No test suite exists - TDD approach should be followed for new features
- The project follows library conventions (no Cargo.lock committed)
- Multi-platform support: Linux (x86_64, ARM), macOS (Intel, ARM), Windows
- GitHub Actions handles automated releases via `.github/workflows/release.yml`
- The tool requires staged Git changes to generate commit messages

## Functional Requirements (from 要件定義書)

### FR001: DeepSeek API Integration
- Replace OpenAI API with DeepSeek API for commit message generation
- Read `DEEPSEEK_API_KEY` environment variable instead of `OPENAI_API_KEY`
- Send staged diff (`git diff --staged`) to DeepSeek API endpoint
- Parse API response and format as commit message

### FR002: Custom Commit Message Format
- Add `--format` CLI argument to specify custom message templates
- Support placeholders like `{title}` and `{description}` in format strings
- Apply default format (`タイトル\n\n説明`) when not specified
- Allow project-specific commit conventions

### FR003: Maintain Existing Features
- Keep `--dry-run` flag for output without committing
- Keep `--review` flag for editing messages before commit
- Keep `--force` flag to skip confirmation prompts

## Non-Functional Requirements

### Performance
- Target response time: < 10 seconds (including API call)

### Code Quality
- Follow Rust formatting standards (`cargo fmt`)
- Modularize `main.rs` into separate concerns (API, Git operations, CLI parsing)
- Use `cargo clippy` for linting in CI

### Development Process
- Follow TDD methodology (Red-Green-Refactor cycle)
- Commit frequently with conventional commit messages
- Update implementation logs in `_docs/` directory

## Project Phases

### Phase 1: Foundation & API Integration (1 week)
- Refactor `main.rs` to separate Git operations and CLI parsing
- Implement DeepSeek API client
- Create working prototype

### Phase 2: Feature Implementation (1 week)
- Add custom format functionality
- Integrate with existing CLI options

### Phase 3: Testing & Release (3 days)
- Cross-platform testing
- Update documentation
- Release v1.0.0