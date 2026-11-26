# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Современная высокопроизводительная переработка AutoWikiBrowser (AWB) на Rust, предоставляющая средства автоматизированного редактирования Википедии с акцентом на безопасность, производительность и расширяемость.

## Архитектура

```
awb-rs/
├── awb-core/           # Основная бизнес-логика и абстракции
├── awb-api/            # Реализация клиента MediaWiki API
├── awb-cli/            # Интерфейс командной строки
├── awb-rules/          # Механизм правил для автоматизации правок
├── awb-lua-plugin/     # Среда выполнения плагинов Lua
├── awb-wasm-plugin/    # Среда выполнения плагинов WebAssembly
├── awb-security/       # Безопасность и изоляция (песочница)
├── awb-db/             # Постоянное хранение состояния
├── awb-ui/             # Компоненты терминального пользовательского интерфейса
├── awb-fixtures/       # Тестовые фикстуры и имитационные данные (mock)
└── awb-integration/    # Набор интеграционных тестов
```

**Поток данных:**
```
CLI → Core → API → MediaWiki
       ↓
    Механизм правил → Плагины (Lua/WASM)
       ↓
    Уровень безопасности → Выполнение в песочнице
       ↓
    База данных → Сохранение состояния
```

## Возможности

- **Современная архитектура**: Async/await на базе среды выполнения Tokio для высокой производительности
- **Поддержка нескольких вики**: Работа с любыми вики на движке MediaWiki
- **Редактирование на основе правил**: Декларативные описания правил в формате TOML
- **Система плагинов**: Расширение с помощью скриптов Lua и модулей WebAssembly
- **Усиленная безопасность**: Изолированное выполнение плагинов в песочнице с ограничениями ресурсов
- **Пакетные операции**: Эффективная массовая обработка страниц
- **Интерактивный TUI**: Функциональный терминальный интерфейс для мониторинга и управления
- **Пробный режим (dry-run)**: Предварительный просмотр изменений перед их применением
- **Управление состоянием**: Возобновление прерванных сессий
- **Ограничение частоты запросов**: Соблюдение лимитов API вики и рекомендуемых практик
- **Интернационализация**: 45 языков интерфейса через `--lang`, `AWB_LANG` или локаль POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`), включая поддержку разметки RTL для арабского, персидского, иврита и урду

## Поддерживаемые языки

CLI, отчёты ботов и графический интерфейс GTK4 поддерживают 45 языков (включая 6 локалей RTL: `ar`, `ar-EG`, `ar-MA`, `fa`, `he` и `ur`). Передайте флаг `--lang <LANG>` или задайте переменную `AWB_LANG`, чтобы переопределить автоматическое определение системной локали.

Полную таблицу всех 45 поддерживаемых языков и ссылки на локализованную документацию см. в [README.md](../../../README.md).

## Предварительные требования

- **Rust**: Версия 1.85 или выше (MSRV)
- **Cargo**: Входит в стандартную установку Rust

## Установка

### Из исходного кода

```bash
# Клонировать репозиторий
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Собрать все крейты рабочего пространства
cargo build --workspace --release

# Установить исполняемый файл
cargo install --path awb-cli
```

## Разработка

### Сборка

```bash
# Отладочная сборка
cargo build --workspace

# Релизная сборка
cargo build --workspace --release
```

### Тестирование

```bash
# Запустить все тесты
cargo test --workspace

# Запустить тесты с выводом в stdout
cargo test --workspace -- --nocapture
```

### Статический анализ (Lint)

```bash
# Проверить наличие замечаний линтера
cargo clippy --workspace --all-targets --all-features

# Применить автоматические исправления
cargo clippy --fix --allow-dirty --workspace
```

### Форматирование

```bash
# Проверить форматирование кода
cargo fmt --all -- --check

# Применить форматирование
cargo fmt --all
```

## Примеры использования

### Аутентификация

```bash
# Выполнить вход на вики (учётные данные сохраняются безопасно)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Выполнить вход с использованием OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Получение списка страниц

```bash
# Получить список страниц из категории
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Получить список страниц по поисковому запросу
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Получить список страниц из файла
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Применение правил редактирования

```bash
# Запустить правила с профилем (в пробном режиме dry-run)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Применить правила к реальным страницам
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Автоматический режим бота

```bash
# Запустить в автономном режиме бота с ограничениями
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Возобновить предыдущую сессию
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Пример профиля правил

Создайте файл `my-rules.toml`:

```toml
[profile]
name = "Cleanup Bot"
description = "Fix common formatting issues"
edit_summary = "Automated cleanup (Bot)"

[[rules]]
name = "Fix double spaces"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "Fix external links"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "Add missing categories"
plugin = "add_categories.lua"
```

## Система плагинов

awb-rs поддерживает два типа плагинов:

### Плагины Lua

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### Плагины WebAssembly

```rust
// Скомпилируйте в WASM командой: cargo build --target wasm32-wasi
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Возможности плагинов:**
- Выполнение в песочнице с ограничениями ресурсов (память, процессорное время)
- Доступ к безопасному подмножеству API (без доступа к файловой системе и сети)
- Проверка совместимости версий
- Поддержка горячей перезагрузки (hot-reload) в режиме разработки

## Модель безопасности

- **Хранение учётных данных**: Использование системного хранилища ключей ОС для безопасного хранения паролей
- **Изоляция плагинов**: Lua и WASM выполняются в изолированных средах
- **Ограничения ресурсов**: Ограничения процессорного времени, объёма памяти и числа инструкций на каждый плагин
- **Ограничение частоты запросов к API**: Учёт заголовков `Retry-After` и настраиваемых задержек
- **Проверка правок**: Предварительный просмотр изменений перед сохранением и журналы аудита
- **Поддержка OAuth**: Предпочтительный метод аутентификации для ботов

## Лицензия

Проект распространяется на ваш выбор под одной из следующих лицензий:

- Лицензия Apache, версия 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) или http://www.apache.org/licenses/LICENSE-2.0)
- Лицензия MIT ([LICENSE-MIT](../../../LICENSE-MIT) или http://opensource.org/licenses/MIT)

## Участие в разработке

Мы приветствуем ваш вклад! Ознакомьтесь с рекомендациями в файле [CONTRIBUTING.md](../../../CONTRIBUTING.md).

## Статус проекта

Это активно развивающаяся переработка AutoWikiBrowser на Rust. Текущее состояние:

- [x] Основной клиент API
- [x] Механизм правил
- [x] Интерфейс командной строки (CLI)
- [x] Система плагинов (Lua/WASM)
- [x] Изоляция безопасности (песочница)
- [ ] Терминальный интерфейс (в разработке)
- [ ] Графический интерфейс GUI (запланирован)
- [ ] Расширенная визуализация различий (запланирована)

## Ссылки

- [Документация](https://docs.rs/awb-rs)
- [Трекер задач](https://github.com/somethingwithproof/awb-rs/issues)
- [Оригинальный AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
