# auto-commit

> تولیدکننده پیام‌های Git commit مبتنی بر هوش مصنوعی - پشتیبانی از OpenAI / DeepSeek / Gemini

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/auto-commit.svg)](https://crates.io/crates/auto-commit)

🌐 **زبان‌ها**: [日本語](../../README.md) | [English](README.en.md) | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md) | [Русский](README.ru.md) | [العربية](README.ar.md)

## مرور کلی

`auto-commit` یک ابزار CLI است که تغییرات مرحله‌بندی شده را تحلیل کرده و پیام‌های commit مناسب را به صورت خودکار تولید می‌کند. این پروژه یک fork از [m1guelpf/auto-commit](https://github.com/m1guelpf/auto-commit) است که پشتیبانی از چندین ارائه‌دهنده LLM (OpenAI، DeepSeek، Gemini) و قالب‌بندی سفارشی پیام commit را اضافه کرده است.

## ویژگی‌ها

- 🤖 **پشتیبانی از چند ارائه‌دهنده**: انتخاب از بین OpenAI، DeepSeek یا Google Gemini
- 🎨 **قالب‌بندی سفارشی**: سفارشی‌سازی آزاد فرمت پیام با گزینه `--format`
- 📝 **قالب‌های .gitmessage**: تنظیم قوانین commit خاص پروژه
- 🚀 **اجرای سریع**: ساخته شده با Rust برای عملکرد سبک و سریع
- 🔧 **پیکربندی انعطاف‌پذیر**: مدیریت تنظیمات از طریق متغیرهای محیطی یا فایل‌های `.env`
- 🌍 **چند پلتفرمی**: پشتیبانی از Windows، macOS و Linux

## نصب

### Homebrew (macOS / Linux)

```bash
brew tap clearclown/tap
brew install auto-commit
```

### Cargo (Rust)

```bash
# از crates.io
cargo install auto-commit

# مستقیم از مخزن GitHub
cargo install --git https://github.com/clearclown/auto-commit.git
```

### دانلود باینری

باینری‌های پلتفرم خود را از [GitHub Releases](https://github.com/clearclown/auto-commit/releases) دانلود کنید:

| پلتفرم | فایل |
|--------|------|
| macOS (Apple Silicon) | `auto-commit-darwin-aarch64` |
| macOS (Intel) | `auto-commit-darwin-x86_64` |
| Linux (x86_64) | `auto-commit-linux-x86_64` |
| Linux (deb) | `auto-commit-linux-x86_64.deb` |
| Windows (x86_64) | `auto-commit-win-x86_64.exe` |

```bash
# مثال: macOS (Apple Silicon)
curl -LO https://github.com/clearclown/auto-commit/releases/latest/download/auto-commit-darwin-aarch64
chmod +x auto-commit-darwin-aarch64
sudo mv auto-commit-darwin-aarch64 /usr/local/bin/auto-commit
```

### ساخت از سورس

```bash
git clone https://github.com/clearclown/auto-commit.git
cd auto-commit
cargo build --release
sudo mv target/release/auto-commit /usr/local/bin/
```

## راه‌اندازی

### پیکربندی کلید API

کلید API ارائه‌دهنده LLM خود را به عنوان متغیر محیطی تنظیم کنید. اگر چندین کلید تنظیم شده باشد، به صورت خودکار بر اساس اولویت انتخاب می‌شوند.

**اولویت**: `OPENAI_API_KEY` > `DEEPSEEK_API_KEY` > `GEMINI_API_KEY`

```bash
# استفاده از OpenAI
export OPENAI_API_KEY='sk-...'

# استفاده از DeepSeek
export DEEPSEEK_API_KEY='sk-...'

# استفاده از Google Gemini
export GEMINI_API_KEY='AIza...'
```

یا یک فایل `.env` در ریشه پروژه خود ایجاد کنید:

```bash
OPENAI_API_KEY='sk-...'
# DEEPSEEK_API_KEY='sk-...'
# GEMINI_API_KEY='AIza...'
```

## استفاده

### استفاده پایه

```bash
# مرحله‌بندی تغییرات
git add .

# تولید خودکار و commit
auto-commit
```

ارائه‌دهنده مورد استفاده در حین اجرا نمایش داده می‌شود:

```
⠋ Generating commit message using OpenAI...
✓ Commit message generated (OpenAI)
```

### گزینه‌ها

```bash
# اجرای آزمایشی (بدون commit واقعی)
auto-commit --dry-run

# بررسی پیام تولید شده قبل از commit
auto-commit --review

# اجرای اجباری (بدون تأیید)
auto-commit --force

# فرمت سفارشی
auto-commit --format "{emoji} {prefix}: {title}"

# ثبت جزئیات
auto-commit -v
```

### نگهدارنده‌های مکان فرمت

| نگهدارنده مکان | توضیحات | مثال |
|----------------|---------|------|
| `{title}` | خلاصه commit (خط اول) | `Add user authentication` |
| `{description}` | توضیحات مفصل | `Implemented JWT-based auth...` |
| `{emoji}` | ایموجی سبک GitMoji | `✨`, `🐛`, `📝` |
| `{prefix}` | پیشوند Conventional Commits | `feat`, `fix`, `docs` |
| `{scope}` | محدوده تغییر (اختیاری) | `api`, `cli`, `config` |

## ارائه‌دهندگان پشتیبانی شده

| ارائه‌دهنده | مدل پیش‌فرض | متغیر محیطی |
|-------------|-------------|-------------|
| OpenAI | `gpt-4o-mini` | `OPENAI_API_KEY` |
| DeepSeek | `deepseek-chat` | `DEEPSEEK_API_KEY` |
| Google Gemini | `gemini-2.0-flash` | `GEMINI_API_KEY` |

## مجوز

مجوز MIT - برای جزئیات به فایل [LICENSE](../../LICENSE) مراجعه کنید.

## پیوندها

- [مخزن GitHub](https://github.com/clearclown/auto-commit)
- [ردیاب مشکلات](https://github.com/clearclown/auto-commit/issues)
- [پروژه اصلی](https://github.com/m1guelpf/auto-commit)
