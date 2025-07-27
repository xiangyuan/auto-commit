# auto-commit

> تولیدکننده پیام‌های Git commit مبتنی بر هوش مصنوعی - نسخه DeepSeek API

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)

## نمای کلی

`auto-commit` یک ابزار خط فرمان است که تغییرات مرحله‌بندی شده را تحلیل کرده و به طور خودکار پیام‌های commit مناسب تولید می‌کند. این پروژه یک انشعاب از [m1guelpf/auto-commit](https://github.com/m1guelpf/auto-commit) است که backend را از OpenAI به DeepSeek API تغییر داده و قابلیت‌های قالب‌بندی پیام commit قابل تنظیم را اضافه کرده است.

## ویژگی‌ها

- 🤖 **یکپارچه‌سازی DeepSeek API**: تولید پیام‌های commit با کیفیت بالا با LLM تخصصی برنامه‌نویسی
- 🎨 **قالب‌بندی سفارشی**: تنظیم آزادانه قالب پیام با گزینه `--format`
- 🚀 **عملکرد بالا**: اجرای سبک و سریع ساخته شده با Rust
- 🔧 **پیکربندی انعطاف‌پذیر**: مدیریت پیکربندی از طریق متغیرهای محیطی یا فایل‌های `.env`
- 🌍 **چند پلتفرمی**: پشتیبانی از Windows، macOS و Linux

## نصب

### 1. ساخت از منبع (توصیه شده)

```bash
# کلون کردن مخزن
git clone https://github.com/clearclown/auto-commit.git
cd auto-commit

# ساخت
cargo build --release

# انتقال فایل باینری به PATH
sudo mv target/release/auto-commit /usr/local/bin/
```

### 2. استفاده از Cargo

```bash
# نصب مستقیم از مخزن
cargo install --git https://github.com/clearclown/auto-commit.git
```

### 3. نصب دستی

```bash
# کلون کردن مخزن
git clone https://github.com/clearclown/auto-commit.git
cd auto-commit

# ساخت نسخه انتشار
cargo build --release

# کپی فایل اجرایی به مکان دلخواه
cp target/release/auto-commit ~/.local/bin/
# یا
cp target/release/auto-commit ~/bin/

# اطمینان از تنظیم PATH
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## راه‌اندازی

### 1. پیکربندی کلید API DeepSeek

کلید API DeepSeek خود را به عنوان متغیر محیطی تنظیم کنید:

```bash
export DEEPSEEK_API_KEY='sk-XXXXXXXXXXXXXXXX'
```

یا یک فایل `.env` در ریشه پروژه ایجاد کنید:

```bash
echo 'DEEPSEEK_API_KEY=sk-XXXXXXXXXXXXXXXX' > ~/.env
```

### 2. تأیید نصب

```bash
# بررسی نسخه
auto-commit --version

# نمایش راهنما
auto-commit --help
```

## استفاده

### استفاده پایه

```bash
# مرحله‌بندی تغییرات
git add .

# تولید و اجرای پیام commit
auto-commit
```

### گزینه‌ها

```bash
# اجرای آزمایشی (بدون commit واقعی)
auto-commit --dry-run

# بررسی پیام تولید شده قبل از commit
auto-commit --review

# اجرای اجباری (بدون تأیید)
auto-commit --force

# تعیین قالب سفارشی
auto-commit --format "{emoji} {prefix}: {title}"

# خروجی لاگ مفصل
auto-commit -v
```

### جایگزین‌های قالب

جایگزین‌های موجود برای قالب‌بندی سفارشی:

- `{title}` - خلاصه commit (خط اول)
- `{description}` - توضیحات تفصیلی
- `{emoji}` - ایموجی سبک GitMoji
- `{prefix}` - پیشوند Conventional Commits (feat، fix و غیره)
- `{scope}` - محدوده تغییر (اختیاری)

### مثال‌های استفاده

```bash
# قالب Conventional Commits
auto-commit --format "{prefix}({scope}): {title}\n\n{description}"

# قالب GitMoji
auto-commit --format "{emoji} {title}\n\n{description}"

# قالب ساده
auto-commit --format "{title}"
```

## اطلاعات توسعه‌دهندگان

### الزامات

- Rust 1.70.0 یا بالاتر
- Git 2.0 یا بالاتر

### دستورالعمل‌های ساخت

```bash
# ساخت توسعه
cargo build

# ساخت انتشار
cargo build --release

# اجرای تست‌ها
cargo test

# قالب‌بندی کد
cargo fmt

# بررسی کد
cargo clippy
```

### ساختار پروژه

```
src/
├── main.rs         # نقطه ورود
├── lib.rs          # صادرات ماژول‌ها
├── api/            # یکپارچه‌سازی DeepSeek API
├── cli/            # رابط CLI
├── config/         # مدیریت پیکربندی
├── formatter/      # قالب‌بند پیام
└── git/            # عملیات Git
```

## عیب‌یابی

### کلید API شناسایی نمی‌شود

```bash
# بررسی متغیر محیطی
echo $DEEPSEEK_API_KEY

# بررسی فایل .env
cat ~/.env
```

### خطاهای commit

```bash
# بررسی وجود تغییرات مرحله‌بندی شده
git status

# تأیید پیکربندی Git
git config user.name
git config user.email
```

### خطاهای ساخت

```bash
# بررسی نسخه Rust
rustc --version

# به‌روزرسانی وابستگی‌ها
cargo update
```

## مشارکت

1. این مخزن را Fork کنید
2. یک شاخه ویژگی ایجاد کنید (`git checkout -b feature/amazing-feature`)
3. تغییرات خود را commit کنید (از `auto-commit` استفاده کنید!)
4. به شاخه Push کنید (`git push origin feature/amazing-feature`)
5. یک Pull Request ایجاد کنید

## مجوز

مجوز MIT - برای جزئیات فایل [LICENSE](LICENSE) را ببینید.

## قدردانی‌ها

- پروژه اصلی [auto-commit](https://github.com/m1guelpf/auto-commit)
- تیم [DeepSeek](https://www.deepseek.com/)
- همه مشارکت‌کنندگان

## پیوندها

- [مخزن GitHub](https://github.com/clearclown/auto-commit)
- [ردیاب مسائل](https://github.com/clearclown/auto-commit/issues)
- [پروژه اصلی](https://github.com/m1guelpf/auto-commit)
- [مستندات DeepSeek API](https://api.deepseek.com/docs)