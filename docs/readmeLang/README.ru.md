# auto-commit

> Генератор сообщений Git-коммитов на основе ИИ - поддержка OpenAI / DeepSeek / Gemini / Grok

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/auto-commit.svg)](https://crates.io/crates/auto-commit)

🌐 **Языки**: [日本語](../../README.md) | [English](README.en.md) | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md) | [العربية](README.ar.md) | [فارسی](README.fa.md)

## Обзор

`auto-commit` — это CLI-инструмент, который анализирует подготовленные изменения и автоматически генерирует подходящие сообщения коммитов. Этот проект является форком [m1guelpf/auto-commit](https://github.com/m1guelpf/auto-commit) с поддержкой нескольких LLM-провайдеров (OpenAI, DeepSeek, Gemini, Grok) и настраиваемым форматированием сообщений коммитов.

## Возможности

- 🤖 **Поддержка нескольких провайдеров**: Выбор между OpenAI, DeepSeek, Google Gemini или Grok
- 🎨 **Пользовательское форматирование**: Свободная настройка формата сообщений с опцией `--format`
- 📝 **Шаблоны .gitmessage**: Установка правил коммитов для конкретного проекта
- 🚀 **Быстрое выполнение**: Написан на Rust для легкой и быстрой работы
- 🔧 **Гибкая настройка**: Управление настройками через переменные окружения или файлы `.env`
- 🌍 **Мультиплатформенность**: Поддержка Windows, macOS и Linux

## Установка

### Homebrew (macOS / Linux)

```bash
brew tap clearclown/tap
brew install auto-commit
```

### Cargo (Rust)

```bash
# Из crates.io
cargo install auto-commit

# Напрямую из репозитория GitHub
cargo install --git https://github.com/clearclown/auto-commit.git
```

### Загрузка бинарных файлов

Загрузите бинарные файлы для вашей платформы с [GitHub Releases](https://github.com/clearclown/auto-commit/releases):

| Платформа | Файл |
|-----------|------|
| macOS (Apple Silicon) | `auto-commit-darwin-aarch64` |
| macOS (Intel) | `auto-commit-darwin-x86_64` |
| Linux (x86_64) | `auto-commit-linux-x86_64` |
| Linux (deb) | `auto-commit-linux-x86_64.deb` |
| Windows (x86_64) | `auto-commit-win-x86_64.exe` |

```bash
# Пример: macOS (Apple Silicon)
curl -LO https://github.com/clearclown/auto-commit/releases/latest/download/auto-commit-darwin-aarch64
chmod +x auto-commit-darwin-aarch64
sudo mv auto-commit-darwin-aarch64 /usr/local/bin/auto-commit
```

### Сборка из исходников

```bash
git clone https://github.com/clearclown/auto-commit.git
cd auto-commit
cargo build --release
sudo mv target/release/auto-commit /usr/local/bin/
```

## Настройка

### Конфигурация API-ключа

Установите API-ключ вашего LLM-провайдера как переменную окружения. Если установлено несколько ключей, они автоматически выбираются по приоритету.

**Приоритет**: `OPENAI_API_KEY` > `DEEPSEEK_API_KEY` > `GEMINI_API_KEY`. Если ни одного ключа нет, используется вошедший сеанс Grok Build. Если другой ключ задан, нужен `--provider grok`.

```bash
# Использовать OpenAI
export OPENAI_API_KEY='sk-...'

# Использовать DeepSeek
export DEEPSEEK_API_KEY='sk-...'

# Использовать Google Gemini
export GEMINI_API_KEY='AIza...'

# Использовать Grok Build (подписка)
# Ключ xAI API не нужен. После `grok login` запросы идут по подписке.
grok login
auto-commit --provider grok
```

Или создайте файл `.env` в корне проекта:

```bash
OPENAI_API_KEY='sk-...'
# DEEPSEEK_API_KEY='sk-...'
# GEMINI_API_KEY='AIza...'
# XAI_MODEL='grok-4.5'  # необязательно; передаётся только с --provider grok
```

## Использование

### Базовое использование

```bash
# Подготовить изменения
git add .

# Автогенерация и коммит
auto-commit
```

Используемый провайдер отображается во время выполнения:

```
⠋ Generating commit message using OpenAI...
✓ Commit message generated (OpenAI)
```

### Опции

```bash
# Пробный запуск (без реального коммита)
auto-commit --dry-run

# Просмотр сгенерированного сообщения перед коммитом
auto-commit --review

# Принудительное выполнение (без подтверждения)
auto-commit --force

# Пользовательский формат
auto-commit --format "{emoji} {prefix}: {title}"

# Подписка Grok Build (нужен `grok login`)
auto-commit --provider grok

# Переопределить модель. Без флага остаётся модель по умолчанию CLI Grok
auto-commit --provider grok --model grok-4.5

# Подробное логирование
auto-commit -v
```

### Заполнители формата

| Заполнитель | Описание | Пример |
|-------------|----------|--------|
| `{title}` | Краткое описание коммита (первая строка) | `Add user authentication` |
| `{description}` | Подробное описание | `Implemented JWT-based auth...` |
| `{emoji}` | Эмодзи в стиле GitMoji | `✨`, `🐛`, `📝` |
| `{prefix}` | Префикс Conventional Commits | `feat`, `fix`, `docs` |
| `{scope}` | Область изменений (опционально) | `api`, `cli`, `config` |

## Поддерживаемые провайдеры

| Провайдер | Модель по умолчанию | Переменная окружения |
|-----------|---------------------|---------------------|
| OpenAI | `gpt-4o-mini` | `OPENAI_API_KEY` |
| DeepSeek | `deepseek-chat` | `DEEPSEEK_API_KEY` |
| Google Gemini | `gemini-2.0-flash` | `GEMINI_API_KEY` |
| Grok Build | модель CLI (`--model` или `XAI_MODEL`) | `grok login` |

Модель по умолчанию можно переопределить флагом `--model` или переменными `OPENAI_MODEL`, `DEEPSEEK_MODEL`, `GEMINI_MODEL`, `XAI_MODEL`. `--model` имеет приоритет. Grok Build передаёт модель в CLI только если заданы `--model` или `XAI_MODEL`. `XAI_API_KEY` не читается.

## Лицензия

Лицензия MIT - подробности см. в файле [LICENSE](../../LICENSE).

## Ссылки

- [Репозиторий GitHub](https://github.com/clearclown/auto-commit)
- [Трекер проблем](https://github.com/clearclown/auto-commit/issues)
- [Оригинальный проект](https://github.com/m1guelpf/auto-commit)
