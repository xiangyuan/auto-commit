# auto-commit

> AI 驱动的 Git 提交信息生成工具 - DeepSeek API 版

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)

## 概述

`auto-commit` 是一个分析暂存更改并自动生成适当提交信息的 CLI 工具。本项目是 [m1guelpf/auto-commit](https://github.com/m1guelpf/auto-commit) 的分支版本，将后端从 OpenAI 替换为 DeepSeek API，并添加了可自定义的提交信息格式功能。

## 特性

- 🤖 **DeepSeek API 集成**：使用专注于编码的 LLM 生成高质量的提交信息
- 🎨 **自定义格式**：通过 `--format` 选项自由定制消息格式
- 🚀 **高性能**：基于 Rust 构建，轻量且快速
- 🔧 **灵活配置**：通过环境变量或 `.env` 文件管理配置
- 🌍 **多平台支持**：支持 Windows、macOS 和 Linux

## 安装方法

### 1. 从源码构建（推荐）

```bash
# 克隆仓库
git clone https://github.com/clearclown/auto-commit.git
cd auto-commit

# 构建
cargo build --release

# 将二进制文件移动到 PATH
sudo mv target/release/auto-commit /usr/local/bin/
```

### 2. 使用 Cargo

```bash
# 直接从仓库安装
cargo install --git https://github.com/clearclown/auto-commit.git
```

### 3. 手动安装

```bash
# 克隆仓库
git clone https://github.com/clearclown/auto-commit.git
cd auto-commit

# 发布构建
cargo build --release

# 复制可执行文件到指定位置
cp target/release/auto-commit ~/.local/bin/
# 或
cp target/release/auto-commit ~/bin/

# 确保 PATH 已设置
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## 设置

### 1. 配置 DeepSeek API 密钥

将 DeepSeek API 密钥设置为环境变量：

```bash
export DEEPSEEK_API_KEY='sk-XXXXXXXXXXXXXXXX'
```

或在项目根目录创建 `.env` 文件：

```bash
echo 'DEEPSEEK_API_KEY=sk-XXXXXXXXXXXXXXXX' > ~/.env
```

### 2. 验证安装

```bash
# 检查版本
auto-commit --version

# 显示帮助
auto-commit --help
```

## 使用方法

### 基本用法

```bash
# 暂存更改
git add .

# 生成并执行提交信息
auto-commit
```

### 选项

```bash
# 演练运行（不实际提交）
auto-commit --dry-run

# 提交前查看生成的信息
auto-commit --review

# 强制执行（无需确认）
auto-commit --force

# 指定自定义格式
auto-commit --format "{emoji} {prefix}: {title}"

# 详细日志输出
auto-commit -v
```

### 格式占位符

自定义格式可用的占位符：

- `{title}` - 提交摘要（第一行）
- `{description}` - 详细描述
- `{emoji}` - GitMoji 风格表情符号
- `{prefix}` - 约定式提交前缀（feat、fix 等）
- `{scope}` - 更改范围（可选）

### 使用示例

```bash
# 约定式提交格式
auto-commit --format "{prefix}({scope}): {title}\n\n{description}"

# GitMoji 格式
auto-commit --format "{emoji} {title}\n\n{description}"

# 简单格式
auto-commit --format "{title}"
```

## 开发者信息

### 环境要求

- Rust 1.70.0 或更高版本
- Git 2.0 或更高版本

### 构建说明

```bash
# 开发构建
cargo build

# 发布构建
cargo build --release

# 运行测试
cargo test

# 格式化代码
cargo fmt

# 代码检查
cargo clippy
```

### 项目结构

```
src/
├── main.rs         # 入口点
├── lib.rs          # 模块导出
├── api/            # DeepSeek API 集成
├── cli/            # CLI 界面
├── config/         # 配置管理
├── formatter/      # 消息格式化器
└── git/            # Git 操作
```

## 故障排除

### API 密钥无法识别

```bash
# 检查环境变量
echo $DEEPSEEK_API_KEY

# 检查 .env 文件
cat ~/.env
```

### 提交失败

```bash
# 检查是否有暂存的更改
git status

# 验证 Git 配置
git config user.name
git config user.email
```

### 构建错误

```bash
# 检查 Rust 版本
rustc --version

# 更新依赖
cargo update
```

## 贡献

1. Fork 本仓库
2. 创建特性分支（`git checkout -b feature/amazing-feature`）
3. 提交更改（使用 `auto-commit`！）
4. 推送到分支（`git push origin feature/amazing-feature`）
5. 创建 Pull Request

## 许可证

MIT 许可证 - 详情请参阅 [LICENSE](LICENSE) 文件。

## 致谢

- 原始 [auto-commit](https://github.com/m1guelpf/auto-commit) 项目
- [DeepSeek](https://www.deepseek.com/) 团队
- 所有贡献者

## 链接

- [GitHub 仓库](https://github.com/clearclown/auto-commit)
- [问题追踪器](https://github.com/clearclown/auto-commit/issues)
- [原始项目](https://github.com/m1guelpf/auto-commit)
- [DeepSeek API 文档](https://api.deepseek.com/docs)