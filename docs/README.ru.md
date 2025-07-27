# auto-commit

> Генератор сообщений коммитов Git на основе ИИ - версия DeepSeek API

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)

## Обзор

`auto-commit` - это инструмент командной строки, который анализирует подготовленные изменения и автоматически генерирует соответствующие сообщения коммитов. Этот проект является форком [m1guelpf/auto-commit](https://github.com/m1guelpf/auto-commit), который заменяет бэкенд с OpenAI на DeepSeek API и добавляет возможность настройки формата сообщений коммитов.

## Возможности

- 🤖 **Интеграция DeepSeek API**: Высококачественная генерация сообщений коммитов с помощью специализированной для кодирования LLM
- 🎨 **Пользовательское форматирование**: Свободная настройка формата сообщений с помощью опции `--format`
- 🚀 **Высокая производительность**: Легковесное и быстрое выполнение, построенное на Rust
- 🔧 **Гибкая конфигурация**: Управление конфигурацией через переменные окружения или `.env` файлы
- 🌍 **Мультиплатформенность**: Поддержка Windows, macOS и Linux

## Установка

### 1. Сборка из исходного кода (рекомендуется)

```bash
# Клонировать репозиторий
git clone https://github.com/clearclown/auto-commit.git
cd auto-commit

# Сборка
cargo build --release

# Переместить бинарный файл в PATH
sudo mv target/release/auto-commit /usr/local/bin/
```

### 2. Использование Cargo

```bash
# Установка напрямую из репозитория
cargo install --git https://github.com/clearclown/auto-commit.git
```

### 3. Ручная установка

```bash
# Клонировать репозиторий
git clone https://github.com/clearclown/auto-commit.git
cd auto-commit

# Релизная сборка
cargo build --release

# Скопировать исполняемый файл в нужное место
cp target/release/auto-commit ~/.local/bin/
# или
cp target/release/auto-commit ~/bin/

# Убедиться, что PATH настроен
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## Настройка

### 1. Настройка ключа API DeepSeek

Установите ваш ключ API DeepSeek как переменную окружения:

```bash
export DEEPSEEK_API_KEY='sk-XXXXXXXXXXXXXXXX'
```

Или создайте файл `.env` в корне проекта:

```bash
echo 'DEEPSEEK_API_KEY=sk-XXXXXXXXXXXXXXXX' > ~/.env
```

### 2. Проверка установки

```bash
# Проверить версию
auto-commit --version

# Показать справку
auto-commit --help
```

## Использование

### Базовое использование

```bash
# Подготовить изменения
git add .

# Сгенерировать и выполнить сообщение коммита
auto-commit
```

### Опции

```bash
# Пробный запуск (без фактического коммита)
auto-commit --dry-run

# Просмотр сгенерированного сообщения перед коммитом
auto-commit --review

# Принудительное выполнение (без подтверждения)
auto-commit --force

# Указать пользовательский формат
auto-commit --format "{emoji} {prefix}: {title}"

# Подробное логирование
auto-commit -v
```

### Заполнители формата

Доступные заполнители для пользовательского форматирования:

- `{title}` - Краткое описание коммита (первая строка)
- `{description}` - Подробное описание
- `{emoji}` - Эмодзи в стиле GitMoji
- `{prefix}` - Префикс Conventional Commits (feat, fix и т.д.)
- `{scope}` - Область изменений (опционально)

### Примеры использования

```bash
# Формат Conventional Commits
auto-commit --format "{prefix}({scope}): {title}\n\n{description}"

# Формат GitMoji
auto-commit --format "{emoji} {title}\n\n{description}"

# Простой формат
auto-commit --format "{title}"
```

## Информация для разработчиков

### Требования

- Rust 1.70.0 или выше
- Git 2.0 или выше

### Инструкции по сборке

```bash
# Сборка для разработки
cargo build

# Релизная сборка
cargo build --release

# Запуск тестов
cargo test

# Форматирование кода
cargo fmt

# Проверка кода
cargo clippy
```

### Структура проекта

```
src/
├── main.rs         # Точка входа
├── lib.rs          # Экспорт модулей
├── api/            # Интеграция DeepSeek API
├── cli/            # CLI интерфейс
├── config/         # Управление конфигурацией
├── formatter/      # Форматтер сообщений
└── git/            # Git операции
```

## Устранение неполадок

### API ключ не распознается

```bash
# Проверить переменную окружения
echo $DEEPSEEK_API_KEY

# Проверить файл .env
cat ~/.env
```

### Ошибки коммита

```bash
# Проверить наличие подготовленных изменений
git status

# Проверить конфигурацию Git
git config user.name
git config user.email
```

### Ошибки сборки

```bash
# Проверить версию Rust
rustc --version

# Обновить зависимости
cargo update
```

## Вклад в проект

1. Сделайте форк репозитория
2. Создайте ветку функции (`git checkout -b feature/amazing-feature`)
3. Закоммитьте изменения (используйте `auto-commit`!)
4. Отправьте в ветку (`git push origin feature/amazing-feature`)
5. Создайте Pull Request

## Лицензия

Лицензия MIT - См. файл [LICENSE](LICENSE) для подробностей.

## Благодарности

- Оригинальный проект [auto-commit](https://github.com/m1guelpf/auto-commit)
- Команда [DeepSeek](https://www.deepseek.com/)
- Все контрибьюторы

## Ссылки

- [GitHub репозиторий](https://github.com/clearclown/auto-commit)
- [Трекер проблем](https://github.com/clearclown/auto-commit/issues)
- [Оригинальный проект](https://github.com/m1guelpf/auto-commit)
- [Документация DeepSeek API](https://api.deepseek.com/docs)