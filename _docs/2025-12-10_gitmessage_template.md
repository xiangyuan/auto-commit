# 実装ログ: .gitmessageテンプレート対応

日付: 2025-12-10
実装者: Claude Code

## 実装概要

`docs/.gitmessage` テンプレートを読み込み、DeepSeek APIのプロンプトに渡すことで、
プロジェクト固有のコミットメッセージ規約に従ったメッセージを生成できるようにした。

## 実装方針

- TDD（テスト駆動開発）アプローチ
- 論理的なコミット分割（機能ごとに分離）
- 既存機能への影響を最小化

## 変更内容

### 1. Config モジュール (`src/config/mod.rs`)

**新機能:**
- `load_gitmessage_template()` 関数を追加
- 検索順序:
  1. `~/.gitmessage` (ユーザーカスタム)
  2. `./.gitmessage` (プロジェクトルート)
  3. `docs/.gitmessage` (デフォルト)
- `Config` 構造体に `gitmessage_template: Option<String>` フィールドを追加

**テスト:**
- `test_load_gitmessage_template_from_docs`: docs/.gitmessageの読み込み検証

### 2. API モジュール (`src/api/mod.rs`)

**新機能:**
- `build_prompt()` メソッド: テンプレートからプロンプトを構築
- `generate_commit_message()`: 第2引数に `template: Option<&str>` を追加
- `DEFAULT_PROMPT_TEMPLATE`: テンプレートがない場合のフォールバック

**プロンプト構造:**
```
## コミットメッセージのルール
{テンプレート内容}

## 重要な指示
- コミットメッセージのみを出力
- 1行目はタイトル、空行を挟んで本文
- prefixとemojiを適切に選択

## Git diff
```diff
{変更内容}
```
```

### 3. main.rs

- `config.gitmessage_template` を API 呼び出しに渡すように変更

### 4. 依存関係 (`Cargo.toml`)

- `dirs` v6.0.0 追加: クロスプラットフォームでホームディレクトリを取得

## コミット履歴

1. `chore(deps): 👍 Add dirs crate for home directory detection`
2. `feat(config): ✨ Add .gitmessage template loading support`
3. `feat(api): ✨ Support custom .gitmessage template in prompts`
4. `feat: ✨ Wire gitmessage template to commit generation`

## 使用方法

```bash
# 変更をステージ
git add .

# auto-commitを実行（docs/.gitmessageのルールに従う）
auto-commit -f

# カスタムテンプレートを使用する場合
# ~/.gitmessage または ./.gitmessage にテンプレートを配置
```

## テスト結果

```
test result: ok. 28 passed; 0 failed; 0 ignored
```

## 学習ポイント

- 論理的なコミット分割の実践
- Option型を活用した後方互換性の維持
- クロスプラットフォーム対応（dirs クレート）
