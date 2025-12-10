# マルチプロバイダー対応 (OpenAI / DeepSeek / Gemini)

## 実装日時
2025-12-10

## 概要
auto-commitがOpenAI、DeepSeek、Geminiの3つのLLMプロバイダーをサポートするようリファクタリング。
環境変数に設定されたAPIキーを自動検出し、適切なプロバイダーに接続する。

## API互換性

| プロバイダー | エンドポイント | 互換性 |
|-------------|---------------|--------|
| **OpenAI** | `/v1/chat/completions` | OpenAI形式 |
| **DeepSeek** | `/v1/chat/completions` | OpenAI互換 |
| **Gemini** | `/v1beta/models/{model}:generateContent` | 独自形式 |

## 優先順位
環境変数の検出優先順位：
1. `OPENAI_API_KEY` - 最優先
2. `DEEPSEEK_API_KEY` - 次点
3. `GEMINI_API_KEY` - 最後

## アーキテクチャ

### ファイル構成
```
src/api/
├── mod.rs              # モジュールエクスポート、ファクトリ関数
├── client.rs           # LlmClient trait定義、共通ユーティリティ
├── provider.rs         # Provider enum、自動検出ロジック
├── openai_compatible.rs # OpenAI/DeepSeek共通クライアント
└── gemini.rs           # Gemini専用クライアント
```

### 主要コンポーネント

#### LlmClient Trait
```rust
#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn generate_commit_message(
        &self,
        diff: &str,
        template: Option<&str>,
    ) -> Result<(String, String)>;

    fn provider_name(&self) -> &str;
}
```

#### Provider Enum
```rust
pub enum Provider {
    OpenAi,
    DeepSeek,
    Gemini,
}

impl Provider {
    pub fn detect() -> Option<(Self, String)>;
    pub fn base_url(&self) -> &'static str;
    pub fn default_model(&self) -> &'static str;
    pub fn is_openai_compatible(&self) -> bool;
}
```

#### ファクトリ関数
```rust
// 自動検出してクライアント作成
pub fn create_client() -> Result<Box<dyn LlmClient>>;

// 指定プロバイダーでクライアント作成
pub fn create_client_for_provider(provider: Provider, api_key: String) -> Box<dyn LlmClient>;
```

## 使用方法

### 環境変数設定
```bash
# OpenAIを使用
export OPENAI_API_KEY='sk-...'

# DeepSeekを使用
export DEEPSEEK_API_KEY='sk-...'

# Geminiを使用
export GEMINI_API_KEY='AIza...'
```

### .envファイル
```env
# どれか1つを設定（優先順位: OPENAI > DEEPSEEK > GEMINI）
OPENAI_API_KEY='sk-...'
# DEEPSEEK_API_KEY='sk-...'
# GEMINI_API_KEY='AIza...'
```

### コマンド実行
```bash
git add .
auto-commit -f  # 設定されたAPIキーに基づいて自動的にプロバイダーを選択
```

## テスト戦略

### ユニットテスト (mockito使用)
- 各プロバイダーのAPIレスポンス形式を正確にモック
- エラーハンドリングの検証
- 実際のAPI呼び出しなしでロジックをテスト

### 統合テスト
- `tests/integration_test.rs` に配置
- プロバイダー検出ロジックの検証
- クライアントファクトリの動作確認
- APIレスポンスパース処理の検証

### テスト数
- ユニットテスト: 57件
- 統合テスト: 18件
- **合計: 75件 (all passed)**

## デフォルトモデル

| プロバイダー | デフォルトモデル |
|-------------|------------------|
| OpenAI | `gpt-4o-mini` |
| DeepSeek | `deepseek-chat` |
| Gemini | `gemini-2.0-flash` |

## 破壊的変更

### Config構造体
```rust
// 旧
pub struct Config {
    pub deepseek_api_key: String,
    pub gitmessage_template: String,
}

// 新
pub struct Config {
    pub provider: Provider,
    pub api_key: String,
    pub gitmessage_template: String,
}
```

### 非推奨 (deprecated)
- `DeepSeekClient` → `OpenAiCompatibleClient` を使用
- `DeepSeekRequest` → 削除
- `DeepSeekResponse` → 削除

## 追加依存関係
```toml
[dependencies]
async-trait = "0.1.89"
```

## 参考資料
- [OpenAI API Docs](https://platform.openai.com/docs/api-reference)
- [DeepSeek API Docs](https://platform.deepseek.com/api-docs)
- [Google Gemini API Docs](https://ai.google.dev/api/rest)
- [LLM API Pricing Comparison 2025](https://intuitionlabs.ai/articles/llm-api-pricing-comparison-2025)
