# 2026-10-15 Grok provider

> 此后 Grok 改为走 Grok Build 订阅，不再使用 `XAI_API_KEY`。见 [2026-10-15_grok-build-subscription.md](2026-10-15_grok-build-subscription.md)。下面是当时的 API key 实现记录。

## 做了什么

自动提交接入 Grok。Grok 走现有的 OpenAI 兼容客户端，请求 `https://api.x.ai/v1/chat/completions`。

- 环境变量：`XAI_API_KEY`
- 默认模型：`grok-4.7`
- 模型覆盖：`XAI_MODEL`，或 `--model`
- 显式选择：`--provider grok`（别名 `xai`）

自动探测顺序保持原有优先，Grok 在最后：

`OPENAI_API_KEY` > `DEEPSEEK_API_KEY` > `GEMINI_API_KEY` > `XAI_API_KEY`

只设置 `XAI_API_KEY` 时使用 Grok。同时存在其他 key 时，用 `--provider grok` 固定供应商，不会回落到优先级更高的 key。

模型解析顺序：`--model` > 对应的 `*_MODEL` 环境变量（`.env` 文件里的值优先于进程环境）> 内置默认值。

## 用法

```bash
export XAI_API_KEY='xai-...'
auto-commit --provider grok --dry-run

# 可选
export XAI_MODEL='grok-4.5'
auto-commit --provider grok --model grok-4.5
```

`.env` 同样认 `XAI_API_KEY` 和 `XAI_MODEL`。程序仍先读 `src/.env`，读不到再读进程环境变量。

## 验证

- `cargo test --lib`：新增的 Grok 探测、工厂、`.env`、mockito 请求测试通过。
- `cargo test --test integration_test -- --test-threads=1`：21 项通过。
- `cargo test --lib cli::`：`--provider` / `--model` 解析通过。
- `cargo build --bins` 通过。
- `cargo clippy --all-targets` 在本次改动的文件上没有新警告。仓库里原有的 `src/git/mod.rs`、`src/api/client.rs` 警告还在。

下面两项是改动前就失败的测试，断言仍要求英文模板文案 `Commit Message Template` / `Prefix`，而 `docs/.gitmessage` 已是中文模板；本机 `~/.gitmessage` 也会让加载测试读到用户模板：

- `config::tests::test_default_template_is_embedded`
- `config::tests::test_load_gitmessage_template_default`

## 未改的协议

继续使用 Chat Completions，没有切换到 `/v1/responses`，也没有新增 SDK。
