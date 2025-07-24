# auto-commit-改

AIを活用してGitコミットメッセージを自動生成する次世代CLIツール。DeepSeek APIを使用し、カスタマイズ可能なメッセージフォーマットをサポート。

## 特徴

- 🤖 **DeepSeek API統合**: コーディングに特化したLLMでより的確なコミットメッセージを生成
- 🎨 **カスタムフォーマット**: プロジェクトの規約に合わせてメッセージフォーマットを自由に設定
- 🚀 **高速レスポンス**: 10秒以内での生成を目標
- 🛡️ **既存機能の維持**: オリジナルの`auto-commit`の便利な機能をすべて継承

## インストール

### curlを使用（推奨）
```bash
curl -fsSL https://raw.githubusercontent.com/m1guelpf/auto-commit/main/install.sh | sh -
```

### Arch Linux (AUR)
```bash
yay -S auto-commit
```

### 手動インストール
[最新リリース](https://github.com/m1guelpf/auto-commit/releases/latest)から、お使いのOSに対応したバイナリをダウンロードしてください。

## セットアップ

DeepSeek APIキーを環境変数に設定してください：

```bash
export DEEPSEEK_API_KEY='sk-XXXXXXXX'
```

APIキーは[DeepSeekダッシュボード](https://platform.deepseek.com/)から取得できます。

## 使い方

### 基本的な使い方

1. 変更をステージング
```bash
git add .
```

2. コミットメッセージを自動生成
```bash
auto-commit
```

### オプション

```sh
$ auto-commit --help
AIによるコミットメッセージ自動生成

使用方法: auto-commit [OPTIONS]

オプション:
  -v, --verbose...  詳細な出力を表示
  -q, --quiet...    出力を抑制
      --dry-run     生成されたメッセージを表示するが、コミットは作成しない
  -r, --review      コミット前に生成されたメッセージを編集
  -f, --force       確認プロンプトをスキップ
      --format      カスタムメッセージフォーマット (例: "{type}: {title}\n\n{description}")
  -h, --help        ヘルプ情報を表示
  -V, --version     バージョン情報を表示
```

### カスタムフォーマット例

```bash
# Conventional Commits形式
auto-commit --format "feat: {title}\n\n{description}"

# 絵文字付き（プロジェクト標準）
auto-commit --format "{prefix}: {emoji} {title}\n\n{description}"

# スコープ付き
auto-commit --format "{prefix}({scope}): {emoji} {title}\n\n{description}"

# チケット番号付き
auto-commit --format "[TICKET-123] {prefix}: {title}\n\n{description}"
```

### コミットメッセージテンプレート

本プロジェクトでは、`docs/.gitmessage`に詳細なコミットメッセージテンプレートを用意しています。

#### 利用可能なPrefix一覧

| Prefix | 説明 | 絵文字 |
|--------|------|--------|
| feat | 新機能 | ✨ |
| fix | バグ修正 | 🐛 |
| hotfix | クリティカルな修正 | 🚨 |
| add | 新規ファイル／設定追加 | 👍 |
| update | 機能修正(バグ以外) | 👍 |
| change | 仕様変更 | 👍 |
| docs | ドキュメント | 📝 |
| style | フォーマット／空白など | 💄 |
| refactor | リファクタリング | ♻️ |
| perf | パフォーマンス改善 | 🚀 |
| test | テスト | 💚 |
| chore | ビルド／CI／依存追加 | 🍱 |
| disable | 一時的無効化 | 💡 |
| remove | 不要コード削除 | 🔥 |
| rename | リネーム | 📛 |
| upgrade | バージョンアップ | 🆙 |
| revert | 変更取り消し | ⏪ |

#### 設定方法

```bash
# グローバル設定
git config --global commit.template docs/.gitmessage

# プロジェクト単位での設定
git config commit.template docs/.gitmessage
```

## 開発

### 必要な環境
- Rust (最新版) - [rustup](https://rustup.rs/)でインストール
- Git
- DeepSeek APIキー

### ビルド方法

```bash
# 開発用ビルド
cargo build

# リリース用ビルド
cargo build --release

# 実行
cargo run -- --dry-run
```

### テスト駆動開発 (TDD)

本プロジェクトではTDDを実践しています：

```bash
# テスト実行
cargo test

# 特定のテストを実行
cargo test test_deepseek_api

# テストを監視モードで実行
cargo watch -x test

# リンター実行
cargo clippy

# フォーマット
cargo fmt
```

### コミット規約

プロジェクトのコミットメッセージは以下の形式に従います：

```
{prefix}({scope}): {emoji} 概要（50文字以内）

本文（オプション）：
* 何を・なぜを記述（72文字で折り返し）
* 方法は書かない

Co-authored-by: Name <email>
```

## プロジェクト構成

```
auto-commit/
├── src/
│   ├── main.rs         # エントリーポイント（リファクタリング予定）
│   ├── api/           # DeepSeek API連携（予定）
│   ├── git/           # Git操作（予定）
│   └── cli/           # CLI解析（予定）
├── docs/
│   ├── 企画書.md      # プロジェクト企画書
│   ├── 要件定義書.md   # 機能要件・非機能要件
│   └── .gitmessage    # コミットメッセージテンプレート
├── _docs/             # 実装ログディレクトリ
├── tests/             # テストコード（TDD実践）
├── CLAUDE.md          # Claude Code用ガイドライン
└── Cargo.toml         # Rust依存関係管理
```

## KPI

### ビジネスメトリクス
- GitHubリポジトリのスター数: +50
- フォーク数: +10

### 技術メトリクス
- `main.rs`のコード行数: 20%削減
- CIのビルド成功率: 99%以上を維持
- APIレスポンス時間: 10秒以内

## ライセンス

このプロジェクトはMITライセンスの下でオープンソース化されています。詳細は[LICENSEファイル](LICENSE)をご覧ください。

## 謝辞

このプロジェクトは[m1guelpf/auto-commit](https://github.com/m1guelpf/auto-commit)をフォークして作成されました。オリジナルの作者に感謝いたします。