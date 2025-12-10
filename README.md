# auto-commit

> AI搭載Git自動コミットメッセージ生成ツール - OpenAI / DeepSeek / Gemini 対応

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/auto-commit.svg)](https://crates.io/crates/auto-commit)

🌐 **Languages**: [English](docs/README.en.md) | [简体中文](docs/README.zh-CN.md) | [Русский](docs/README.ru.md) | [العربية](docs/README.ar.md) | [فارسی](docs/README.fa.md)

## 概要

`auto-commit`は、ステージされた変更を分析し、適切なコミットメッセージを自動生成するCLIツールです。本プロジェクトは[m1guelpf/auto-commit](https://github.com/m1guelpf/auto-commit)のフォークで、複数のLLMプロバイダー（OpenAI、DeepSeek、Gemini）に対応し、カスタマイズ可能なコミットメッセージフォーマット機能を追加しています。

## 特徴

- 🤖 **マルチプロバイダー対応**: OpenAI、DeepSeek、Google Gemini から選択可能
- 🎨 **カスタムフォーマット**: `--format`オプションで自由にメッセージ形式を設定可能
- 📝 **.gitmessage テンプレート**: プロジェクト固有のコミットルールを設定可能
- 🚀 **高速実行**: Rust製で軽量・高速動作
- 🔧 **柔軟な設定**: 環境変数や`.env`ファイルでの設定管理
- 🌍 **マルチプラットフォーム**: Windows、macOS、Linux対応

## インストール方法

### Homebrew (macOS / Linux)

```bash
brew tap clearclown/tap
brew install auto-commit
```

### Cargo (Rust)

```bash
# crates.io から
cargo install auto-commit

# GitHubリポジトリから直接
cargo install --git https://github.com/clearclown/auto-commit.git
```

### バイナリダウンロード

[GitHub Releases](https://github.com/clearclown/auto-commit/releases)から、お使いのプラットフォーム向けのバイナリをダウンロード：

| プラットフォーム | ファイル |
|-----------------|---------|
| macOS (Apple Silicon) | `auto-commit-darwin-aarch64` |
| macOS (Intel) | `auto-commit-darwin-x86_64` |
| Linux (x86_64) | `auto-commit-linux-x86_64` |
| Linux (deb) | `auto-commit-linux-x86_64.deb` |
| Windows (x86_64) | `auto-commit-win-x86_64.exe` |

```bash
# 例: macOS (Apple Silicon)
curl -LO https://github.com/clearclown/auto-commit/releases/latest/download/auto-commit-darwin-aarch64
chmod +x auto-commit-darwin-aarch64
sudo mv auto-commit-darwin-aarch64 /usr/local/bin/auto-commit
```

### Arch Linux (AUR)

> **注意**: 現在AURの `auto-commit` パッケージはオリジナルの [m1guelpf/auto-commit](https://github.com/m1guelpf/auto-commit) 用です。このフォーク版をインストールするには、Cargo (GitHub) またはソースビルドをご利用ください。

```bash
# オリジナル版をインストールする場合
yay -S auto-commit
# または
paru -S auto-commit
```

### ソースからビルド

```bash
# リポジトリをクローン
git clone https://github.com/clearclown/auto-commit.git
cd auto-commit

# ビルド
cargo build --release

# バイナリをパスに移動
sudo mv target/release/auto-commit /usr/local/bin/
```

## セットアップ

### APIキーの設定

お使いのLLMプロバイダーのAPIキーを環境変数に設定します。複数設定されている場合は、優先順位に従って自動選択されます。

**優先順位**: `OPENAI_API_KEY` > `DEEPSEEK_API_KEY` > `GEMINI_API_KEY`

```bash
# OpenAI を使用
export OPENAI_API_KEY='sk-...'

# DeepSeek を使用
export DEEPSEEK_API_KEY='sk-...'

# Google Gemini を使用
export GEMINI_API_KEY='AIza...'
```

または、プロジェクトルートに`.env`ファイルを作成：

```bash
# .env ファイル（いずれか1つを設定）
OPENAI_API_KEY='sk-...'
# DEEPSEEK_API_KEY='sk-...'
# GEMINI_API_KEY='AIza...'
```

### 動作確認

```bash
# バージョン確認
auto-commit --version

# ヘルプ表示
auto-commit --help
```

## 使い方

### 基本的な使用方法

```bash
# 変更をステージ
git add .

# コミットメッセージを自動生成して実行
auto-commit
```

実行時に使用されるプロバイダーが表示されます：

```
⠋ Generating commit message using OpenAI...
✓ Commit message generated (OpenAI)
```

### オプション

```bash
# ドライラン（実際にはコミットしない）
auto-commit --dry-run

# 生成されたメッセージを確認してから決定
auto-commit --review

# 強制実行（確認なし）
auto-commit --force

# カスタムフォーマット指定
auto-commit --format "{emoji} {prefix}: {title}"

# 詳細なログ出力
auto-commit -v
```

### フォーマットプレースホルダー

カスタムフォーマットで使用可能なプレースホルダー：

| プレースホルダー | 説明 | 例 |
|-----------------|------|-----|
| `{title}` | コミットの要約（1行目） | `Add user authentication` |
| `{description}` | 詳細な説明 | `Implemented JWT-based auth...` |
| `{emoji}` | GitMojiスタイルの絵文字 | `✨`, `🐛`, `📝` |
| `{prefix}` | Conventional Commitsプレフィックス | `feat`, `fix`, `docs` |
| `{scope}` | 変更スコープ（optional） | `api`, `cli`, `config` |

### 使用例

```bash
# Conventional Commits形式
auto-commit --format "{prefix}({scope}): {title}\n\n{description}"

# GitMoji形式
auto-commit --format "{emoji} {title}\n\n{description}"

# シンプル形式
auto-commit --format "{title}"
```

## .gitmessage テンプレート

プロジェクト固有のコミットルールを設定できます。以下の優先順位で読み込まれます：

1. `~/.gitmessage` - ユーザーカスタム（最優先）
2. `./.gitmessage` - プロジェクトルート
3. 組み込みデフォルト - バイナリに埋め込み済み

テンプレートファイルには、コミットメッセージのフォーマットルール、使用可能なプレフィックス、絵文字ガイドラインなどを記述できます。

## 対応プロバイダー

| プロバイダー | デフォルトモデル | 環境変数 |
|-------------|------------------|----------|
| OpenAI | `gpt-4o-mini` | `OPENAI_API_KEY` |
| DeepSeek | `deepseek-chat` | `DEEPSEEK_API_KEY` |
| Google Gemini | `gemini-2.0-flash` | `GEMINI_API_KEY` |

## 開発者向け情報

### 必要環境

- Rust 1.70.0以上
- Git 2.0以上

### ビルド方法

```bash
# 開発ビルド
cargo build

# リリースビルド
cargo build --release

# テスト実行
cargo test -- --test-threads=1

# フォーマット
cargo fmt

# Lint
cargo clippy
```

### プロジェクト構造

```
src/
├── main.rs              # エントリーポイント
├── lib.rs               # モジュールエクスポート
├── api/
│   ├── mod.rs           # クライアントファクトリ
│   ├── client.rs        # LlmClient trait
│   ├── provider.rs      # Provider enum（自動検出）
│   ├── openai_compatible.rs  # OpenAI/DeepSeek クライアント
│   └── gemini.rs        # Gemini クライアント
├── cli/                 # CLIインターフェース
├── config/              # 設定管理
├── formatter/           # メッセージフォーマッター
└── git/                 # Git操作
```

## トラブルシューティング

### APIキーが認識されない

```bash
# 環境変数の確認
echo $OPENAI_API_KEY
echo $DEEPSEEK_API_KEY
echo $GEMINI_API_KEY

# .envファイルの確認
cat .env
```

### 使用されるプロバイダーを確認したい

`-v` オプションで詳細ログを出力すると、どのプロバイダーが選択されたか確認できます：

```bash
auto-commit -v --dry-run
```

### コミットが失敗する

```bash
# ステージされた変更があるか確認
git status

# Gitの設定確認
git config user.name
git config user.email
```

### ビルドエラー

```bash
# Rustのバージョン確認
rustc --version

# 依存関係の更新
cargo update
```

## 貢献方法

1. このリポジトリをフォーク
2. フィーチャーブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット（`auto-commit`を使用！）
4. ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. プルリクエストを作成

## ライセンス

MITライセンス - 詳細は[LICENSE](LICENSE)ファイルを参照してください。

## 謝辞

- オリジナルの[auto-commit](https://github.com/m1guelpf/auto-commit)プロジェクト
- [OpenAI](https://openai.com/)
- [DeepSeek](https://www.deepseek.com/)
- [Google Gemini](https://ai.google.dev/)
- すべてのコントリビューター

## リンク

- [GitHub Repository](https://github.com/clearclown/auto-commit)
- [Issue Tracker](https://github.com/clearclown/auto-commit/issues)
- [Original Project](https://github.com/m1guelpf/auto-commit)
- [OpenAI API Documentation](https://platform.openai.com/docs)
- [DeepSeek API Documentation](https://platform.deepseek.com/api-docs)
- [Gemini API Documentation](https://ai.google.dev/api)
