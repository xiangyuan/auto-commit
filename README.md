# auto-commit

> AI搭載Git自動コミットメッセージ生成ツール - DeepSeek API対応版

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)

🌐 **Languages**: [English](docs/README.en.md) | [简体中文](docs/README.zh-CN.md) | [Русский](docs/README.ru.md) | [العربية](docs/README.ar.md) | [فارسی](docs/README.fa.md)

## 概要

`auto-commit`は、ステージされた変更を分析し、適切なコミットメッセージを自動生成するCLIツールです。本プロジェクトは[m1guelpf/auto-commit](https://github.com/m1guelpf/auto-commit)のフォークで、バックエンドをOpenAIからDeepSeek APIへ切り替え、カスタマイズ可能なコミットメッセージフォーマット機能を追加しています。

## 特徴

- 🤖 **DeepSeek API統合**: コーディングに特化したLLMによる高品質なコミットメッセージ生成
- 🎨 **カスタムフォーマット**: `--format`オプションで自由にメッセージ形式を設定可能
- 🚀 **高速実行**: Rust製で軽量・高速動作
- 🔧 **柔軟な設定**: 環境変数や`.env`ファイルでの設定管理
- 🌍 **マルチプラットフォーム**: Windows、macOS、Linux対応

## インストール方法

### 1. ソースからビルド（推奨）

```bash
# リポジトリをクローン
git clone https://github.com/clearclown/auto-commit.git
cd auto-commit

# ビルド
cargo build --release

# バイナリをパスに移動
sudo mv target/release/auto-commit /usr/local/bin/
```

### 2. Cargoを使用

```bash
# リポジトリから直接インストール
cargo install --git https://github.com/clearclown/auto-commit.git
```

### 3. 手動インストール

```bash
# リポジトリをクローン
git clone https://github.com/clearclown/auto-commit.git
cd auto-commit

# リリースビルド
cargo build --release

# 実行可能ファイルを任意の場所にコピー
cp target/release/auto-commit ~/.local/bin/
# または
cp target/release/auto-commit ~/bin/

# パスが通っていることを確認
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## セットアップ

### 1. DeepSeek APIキーの設定

DeepSeek APIキーを環境変数に設定します：

```bash
export DEEPSEEK_API_KEY='sk-XXXXXXXXXXXXXXXX'
```

または、プロジェクトルートに`.env`ファイルを作成：

```bash
echo 'DEEPSEEK_API_KEY=sk-XXXXXXXXXXXXXXXX' > ~/.env
```

### 2. 動作確認

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

- `{title}` - コミットの要約（1行目）
- `{description}` - 詳細な説明
- `{emoji}` - GitMojiスタイルの絵文字
- `{prefix}` - Conventional Commitsプレフィックス（feat、fix等）
- `{scope}` - 変更スコープ（optional）

### 使用例

```bash
# Conventional Commits形式
auto-commit --format "{prefix}({scope}): {title}\n\n{description}"

# GitMoji形式
auto-commit --format "{emoji} {title}\n\n{description}"

# シンプル形式
auto-commit --format "{title}"
```

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
cargo test

# フォーマット
cargo fmt

# Lint
cargo clippy
```

### プロジェクト構造

```
src/
├── main.rs         # エントリーポイント
├── lib.rs          # モジュールエクスポート
├── api/            # DeepSeek API統合
├── cli/            # CLIインターフェース
├── config/         # 設定管理
├── formatter/      # メッセージフォーマッター
└── git/            # Git操作
```

## トラブルシューティング

### APIキーが認識されない

```bash
# 環境変数の確認
echo $DEEPSEEK_API_KEY

# .envファイルの確認
cat ~/.env
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
- [DeepSeek](https://www.deepseek.com/)チーム
- すべてのコントリビューター

## リンク

- [GitHub Repository](https://github.com/clearclown/auto-commit)
- [Issue Tracker](https://github.com/clearclown/auto-commit/issues)
- [Original Project](https://github.com/m1guelpf/auto-commit)
- [DeepSeek API Documentation](https://api.deepseek.com/docs)
