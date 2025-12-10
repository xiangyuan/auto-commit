# auto-commit

> مولد رسائل Git commit مدعوم بالذكاء الاصطناعي - يدعم OpenAI / DeepSeek / Gemini

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/auto-commit.svg)](https://crates.io/crates/auto-commit)

🌐 **اللغات**: [日本語](../../README.md) | [English](README.en.md) | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md) | [Русский](README.ru.md) | [فارسی](README.fa.md)

## نظرة عامة

`auto-commit` هو أداة CLI تحلل التغييرات المرحلية وتولد رسائل commit مناسبة تلقائياً. هذا المشروع هو fork من [m1guelpf/auto-commit](https://github.com/m1guelpf/auto-commit)، مع إضافة دعم لعدة مزودي LLM (OpenAI، DeepSeek، Gemini) وتنسيق رسائل commit قابل للتخصيص.

## الميزات

- 🤖 **دعم مزودين متعددين**: اختر من OpenAI أو DeepSeek أو Google Gemini
- 🎨 **تنسيق مخصص**: تخصيص تنسيق الرسالة بحرية باستخدام خيار `--format`
- 📝 **قوالب .gitmessage**: تعيين قواعد commit خاصة بالمشروع
- 🚀 **تنفيذ سريع**: مبني بـ Rust للأداء الخفيف والسريع
- 🔧 **تكوين مرن**: إدارة الإعدادات عبر متغيرات البيئة أو ملفات `.env`
- 🌍 **متعدد المنصات**: يدعم Windows و macOS و Linux

## التثبيت

### Homebrew (macOS / Linux)

```bash
brew tap clearclown/tap
brew install auto-commit
```

### Cargo (Rust)

```bash
# من crates.io
cargo install auto-commit

# مباشرة من مستودع GitHub
cargo install --git https://github.com/clearclown/auto-commit.git
```

### تحميل الملفات الثنائية

قم بتحميل الملفات الثنائية لمنصتك من [GitHub Releases](https://github.com/clearclown/auto-commit/releases):

| المنصة | الملف |
|--------|-------|
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

### البناء من المصدر

```bash
git clone https://github.com/clearclown/auto-commit.git
cd auto-commit
cargo build --release
sudo mv target/release/auto-commit /usr/local/bin/
```

## الإعداد

### تكوين مفتاح API

قم بتعيين مفتاح API لمزود LLM الخاص بك كمتغير بيئة. إذا تم تعيين مفاتيح متعددة، يتم اختيارها تلقائياً حسب الأولوية.

**الأولوية**: `OPENAI_API_KEY` > `DEEPSEEK_API_KEY` > `GEMINI_API_KEY`

```bash
# استخدام OpenAI
export OPENAI_API_KEY='sk-...'

# استخدام DeepSeek
export DEEPSEEK_API_KEY='sk-...'

# استخدام Google Gemini
export GEMINI_API_KEY='AIza...'
```

أو أنشئ ملف `.env` في جذر مشروعك:

```bash
OPENAI_API_KEY='sk-...'
# DEEPSEEK_API_KEY='sk-...'
# GEMINI_API_KEY='AIza...'
```

## الاستخدام

### الاستخدام الأساسي

```bash
# ترحيل التغييرات
git add .

# توليد تلقائي والـ commit
auto-commit
```

يتم عرض المزود المستخدم أثناء التنفيذ:

```
⠋ Generating commit message using OpenAI...
✓ Commit message generated (OpenAI)
```

### الخيارات

```bash
# تشغيل تجريبي (بدون commit فعلي)
auto-commit --dry-run

# مراجعة الرسالة المولدة قبل الـ commit
auto-commit --review

# تنفيذ إجباري (بدون تأكيد)
auto-commit --force

# تنسيق مخصص
auto-commit --format "{emoji} {prefix}: {title}"

# تسجيل مفصل
auto-commit -v
```

### العناصر النائبة للتنسيق

| العنصر النائب | الوصف | مثال |
|---------------|-------|------|
| `{title}` | ملخص الـ commit (السطر الأول) | `Add user authentication` |
| `{description}` | وصف مفصل | `Implemented JWT-based auth...` |
| `{emoji}` | رمز تعبيري بنمط GitMoji | `✨`, `🐛`, `📝` |
| `{prefix}` | بادئة Conventional Commits | `feat`, `fix`, `docs` |
| `{scope}` | نطاق التغيير (اختياري) | `api`, `cli`, `config` |

## المزودون المدعومون

| المزود | النموذج الافتراضي | متغير البيئة |
|--------|------------------|--------------|
| OpenAI | `gpt-4o-mini` | `OPENAI_API_KEY` |
| DeepSeek | `deepseek-chat` | `DEEPSEEK_API_KEY` |
| Google Gemini | `gemini-2.0-flash` | `GEMINI_API_KEY` |

## الترخيص

رخصة MIT - راجع ملف [LICENSE](../../LICENSE) للتفاصيل.

## الروابط

- [مستودع GitHub](https://github.com/clearclown/auto-commit)
- [متتبع المشكلات](https://github.com/clearclown/auto-commit/issues)
- [المشروع الأصلي](https://github.com/m1guelpf/auto-commit)
