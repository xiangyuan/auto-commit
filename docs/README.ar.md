# auto-commit

> مولد رسائل Git commit مدعوم بالذكاء الاصطناعي - إصدار DeepSeek API

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)

## نظرة عامة

`auto-commit` هو أداة سطر أوامر تقوم بتحليل التغييرات المُحضرة وتوليد رسائل commit مناسبة تلقائياً. هذا المشروع هو نسخة متفرعة من [m1guelpf/auto-commit](https://github.com/m1guelpf/auto-commit) تستبدل الواجهة الخلفية من OpenAI إلى DeepSeek API وتضيف ميزة تنسيق رسائل commit القابلة للتخصيص.

## المميزات

- 🤖 **تكامل DeepSeek API**: توليد رسائل commit عالية الجودة باستخدام LLM متخصص في البرمجة
- 🎨 **تنسيق مخصص**: تخصيص حر لصيغة الرسائل باستخدام خيار `--format`
- 🚀 **أداء عالي**: تنفيذ خفيف وسريع مبني بـ Rust
- 🔧 **تكوين مرن**: إدارة التكوين عبر متغيرات البيئة أو ملفات `.env`
- 🌍 **متعدد المنصات**: يدعم Windows و macOS و Linux

## التثبيت

### 1. البناء من المصدر (موصى به)

```bash
# استنساخ المستودع
git clone https://github.com/clearclown/auto-commit.git
cd auto-commit

# البناء
cargo build --release

# نقل الملف التنفيذي إلى PATH
sudo mv target/release/auto-commit /usr/local/bin/
```

### 2. استخدام Cargo

```bash
# التثبيت مباشرة من المستودع
cargo install --git https://github.com/clearclown/auto-commit.git
```

### 3. التثبيت اليدوي

```bash
# استنساخ المستودع
git clone https://github.com/clearclown/auto-commit.git
cd auto-commit

# بناء الإصدار
cargo build --release

# نسخ الملف التنفيذي إلى المكان المطلوب
cp target/release/auto-commit ~/.local/bin/
# أو
cp target/release/auto-commit ~/bin/

# التأكد من تعيين PATH
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## الإعداد

### 1. تكوين مفتاح DeepSeek API

عيّن مفتاح DeepSeek API كمتغير بيئة:

```bash
export DEEPSEEK_API_KEY='sk-XXXXXXXXXXXXXXXX'
```

أو أنشئ ملف `.env` في جذر المشروع:

```bash
echo 'DEEPSEEK_API_KEY=sk-XXXXXXXXXXXXXXXX' > ~/.env
```

### 2. التحقق من التثبيت

```bash
# فحص الإصدار
auto-commit --version

# عرض المساعدة
auto-commit --help
```

## الاستخدام

### الاستخدام الأساسي

```bash
# تحضير التغييرات
git add .

# توليد وتنفيذ رسالة commit
auto-commit
```

### الخيارات

```bash
# تشغيل تجريبي (بدون commit فعلي)
auto-commit --dry-run

# مراجعة الرسالة المولدة قبل الـ commit
auto-commit --review

# التنفيذ القسري (بدون تأكيد)
auto-commit --force

# تحديد تنسيق مخصص
auto-commit --format "{emoji} {prefix}: {title}"

# سجل مفصل
auto-commit -v
```

### متغيرات التنسيق

المتغيرات المتاحة للتنسيق المخصص:

- `{title}` - ملخص الـ commit (السطر الأول)
- `{description}` - الوصف التفصيلي
- `{emoji}` - رمز تعبيري بأسلوب GitMoji
- `{prefix}` - بادئة Conventional Commits (feat، fix، إلخ)
- `{scope}` - نطاق التغيير (اختياري)

### أمثلة الاستخدام

```bash
# تنسيق Conventional Commits
auto-commit --format "{prefix}({scope}): {title}\n\n{description}"

# تنسيق GitMoji
auto-commit --format "{emoji} {title}\n\n{description}"

# تنسيق بسيط
auto-commit --format "{title}"
```

## معلومات المطورين

### المتطلبات

- Rust 1.70.0 أو أعلى
- Git 2.0 أو أعلى

### تعليمات البناء

```bash
# بناء التطوير
cargo build

# بناء الإصدار
cargo build --release

# تشغيل الاختبارات
cargo test

# تنسيق الكود
cargo fmt

# فحص الكود
cargo clippy
```

### هيكل المشروع

```
src/
├── main.rs         # نقطة الدخول
├── lib.rs          # تصدير الوحدات
├── api/            # تكامل DeepSeek API
├── cli/            # واجهة CLI
├── config/         # إدارة التكوين
├── formatter/      # منسق الرسائل
└── git/            # عمليات Git
```

## استكشاف الأخطاء وإصلاحها

### مفتاح API غير معترف به

```bash
# فحص متغير البيئة
echo $DEEPSEEK_API_KEY

# فحص ملف .env
cat ~/.env
```

### فشل الـ commit

```bash
# التحقق من وجود تغييرات محضرة
git status

# التحقق من تكوين Git
git config user.name
git config user.email
```

### أخطاء البناء

```bash
# فحص إصدار Rust
rustc --version

# تحديث التبعيات
cargo update
```

## المساهمة

1. قم بعمل Fork للمستودع
2. أنشئ فرع للميزة (`git checkout -b feature/amazing-feature`)
3. احفظ تغييراتك (استخدم `auto-commit`!)
4. ادفع إلى الفرع (`git push origin feature/amazing-feature`)
5. أنشئ Pull Request

## الترخيص

ترخيص MIT - راجع ملف [LICENSE](LICENSE) للتفاصيل.

## شكر وتقدير

- مشروع [auto-commit](https://github.com/m1guelpf/auto-commit) الأصلي
- فريق [DeepSeek](https://www.deepseek.com/)
- جميع المساهمين

## الروابط

- [مستودع GitHub](https://github.com/clearclown/auto-commit)
- [تتبع المشاكل](https://github.com/clearclown/auto-commit/issues)
- [المشروع الأصلي](https://github.com/m1guelpf/auto-commit)
- [وثائق DeepSeek API](https://api.deepseek.com/docs)