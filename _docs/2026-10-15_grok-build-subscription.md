# 2026-10-15 Grok Build 订阅

## 做了什么

Grok 的提交信息不再请求 `https://api.x.ai`，也不再读取 `XAI_API_KEY`。生成时调用本机已登录的 `grok` CLI，用量记在 Grok Build 订阅上。

- 登录：`grok login`。只检查 `$GROK_HOME/auth.json`（默认 `~/.grok/auth.json`）是否为非空文件，不读取内容。
- 显式选择：`--provider grok`（别名 `xai`）。
- 自动探测：`OPENAI_API_KEY` > `DEEPSEEK_API_KEY` > `GEMINI_API_KEY`，都没有时才用已登录的 Grok Build。设了其他密钥时必须加 `--provider grok`。
- 模型：默认不向 CLI 传 `-m`。只有 `--model` 或 `XAI_MODEL` 会覆盖。
- 子进程会去掉 `XAI_API_KEY`，避免退回到按量计费的 API。
- 可选环境变量：`GROK_BIN`（默认 `grok`）、`GROK_HOME`、`GROK_BUILD_TIMEOUT_SECS`（默认 180）。

调用参数包括 `--prompt-file`、`--output-format json`、`--verbatim`、`--max-turns 1`、`--no-subagents`、`--disable-web-search`、`--no-auto-update`，并禁用写文件和联网工具。不使用 `--yolo`。

## 用法

```bash
grok login
auto-commit --provider grok --dry-run

# 可选
export XAI_MODEL='grok-4.5'
auto-commit --provider grok --model grok-4.5
```

未登录时提示：`Grok Build is not signed in. Run grok login.`

程序仍先读 `src/.env`。文件里若有其他供应商的 API key，自动选择会用那个 key；要用订阅就加 `--provider grok`。文件里的 `XAI_API_KEY` 不会选中 Grok。

## 验证

- `cargo test --offline --lib -- --test-threads=1`：89 通过，2 失败。失败的是改动前就有的模板测试 `test_default_template_is_embedded`、`test_load_gitmessage_template_default`。
- `cargo test --offline --test integration_test -- --test-threads=1`：21 通过。
- `cargo test --offline --lib grok_build`：4 通过。假 CLI 确认了参数、未转发 `XAI_API_KEY`、未默认传 `-m`、显式模型、错误 JSON、未登录。
- `cargo clippy --all-targets -D warnings`：本次改动的文件没有新警告。原有警告在 `src/api/client.rs`、`src/emoji/mod.rs`、`src/git/mod.rs`。
