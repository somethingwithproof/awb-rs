# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Сучасна високопродуктивна переробка AutoWikiBrowser (AWB) мовою Rust, що надає можливості автоматизованого редагування Вікіпедії з акцентом на безпеку, продуктивність і розширюваність.

## Архітектура

```
awb-rs/
├── awb-core/           # Основна бізнес-логіка та абстракції
├── awb-api/            # Реалізація клієнта MediaWiki API
├── awb-cli/            # Інтерфейс командного рядка
├── awb-rules/          # Рушій правил для автоматизації редагувань
├── awb-lua-plugin/     # Середовище виконання плагінів Lua
├── awb-wasm-plugin/    # Середовище виконання плагінів WebAssembly
├── awb-security/       # Безпека та ізоляція (пісочниця)
├── awb-db/             # Постійне керування станом
├── awb-ui/             # Компоненти термінального інтерфейсу користувача
├── awb-fixtures/       # Тестові фікстури та імітаційні дані (mock)
└── awb-integration/    # Набір інтеграційних тестів
```

**Потік даних:**
```
CLI → Core → API → MediaWiki
       ↓
    Рушій правил → Плагіни (Lua/WASM)
       ↓
    Рівень безпеки → Виконання у пісочниці
       ↓
    База даних → Збереження стану
```

## Можливості

- **Сучасна архітектура**: Async/await із середовищем виконання Tokio для високої продуктивності
- **Підтримка кількох вікі**: Робота з будь-якою вікі на основі MediaWiki
- **Редагування на основі правил**: Декларативні визначення правил у форматі TOML
- **Система плагінів**: Розширення за допомогою скриптів Lua та модулів WebAssembly
- **Посилена безпека**: Ізольоване виконання плагінів із обмеженнями ресурсів
- **Пакетні операції**: Ефективна масова обробка сторінок
- **Інтерактивний TUI**: Насичений термінальний інтерфейс для моніторингу та керування
- **Пробний режим (dry-run)**: Попередній перегляд змін перед їх застосуванням
- **Керування станом**: Відновлення перерваних сесій
- **Обмеження частоти запитів**: Дотримання лімітів API вікі та найкращих практик
- **Інтернаціоналізація**: 45 мов інтерфейсу через `--lang`, `AWB_LANG` або локаль POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`), включно з підтримкою RTL-розмітки для арабської, перської, івриту та урду

## Підтримувані мови

CLI, звіти ботів та графічний інтерфейс GTK4 підтримують 45 мов (зокрема 6 RTL-локалей: `ar`, `ar-EG`, `ar-MA`, `fa`, `he` та `ur`). Передайте `--lang <LANG>` або встановіть `AWB_LANG`, щоб перевизначити автоматичне визначення системної локалі.

Повну таблицю всіх 45 підтримуваних мов і посилання на локалізовану документацію див. у [README.md](../../../README.md).

## Попередні вимоги

- **Rust**: Версія 1.85 або новіша (MSRV)
- **Cargo**: Входить до встановлення Rust

## Встановлення

### Із вихідного коду

```bash
# Клонувати репозиторій
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Зібрати всі крейти робочого простору
cargo build --workspace --release

# Встановити виконуваний файл
cargo install --path awb-cli
```

## Розробка

### Збирання

```bash
# Налагоджувальна збірка
cargo build --workspace

# Релізна збірка
cargo build --workspace --release
```

### Тестування

```bash
# Запустити всі тести
cargo test --workspace

# Запустити тести з виведенням стандартного потоку
cargo test --workspace -- --nocapture
```

### Статичний аналіз (Lint)

```bash
# Перевірити код на попередження лінтера
cargo clippy --workspace --all-targets --all-features

# Застосувати автоматичні виправлення
cargo clippy --fix --allow-dirty --workspace
```

### Форматування

```bash
# Перевірити форматування
cargo fmt --all -- --check

# Застосувати форматування
cargo fmt --all
```

## Приклади використання

### Автентифікація

```bash
# Увійти до вікі (безпечно зберігає облікові дані)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Увійти за допомогою OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Отримання списку сторінок

```bash
# Отримати список сторінок із категорії
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Отримати список сторінок за пошуковим запитом
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Отримати список сторінок із файлу
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Виконання правил редагування

```bash
# Запустити правила з профілем (пробний режим dry-run)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Застосувати правила до реальних сторінок
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Автоматизований режим бота

```bash
# Запустити в автономному режимі бота з обмеженнями
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Відновити попередню сесію
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Приклад профілю правил

Створіть файл `my-rules.toml`:

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

## Система плагінів

awb-rs підтримує два типи плагінів:

### Плагіни Lua

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### Плагіни WebAssembly

```rust
// Скомпілюйте у WASM командою: cargo build --target wasm32-wasi
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Можливості плагінів:**
- Виконання у пісочниці з обмеженнями ресурсів (пам'ять, час ЦП)
- Доступ до безпечної підмножини API (без доступу до файлової системи та мережі)
- Перевірка сумісності версій
- Підтримка гарячого перезавантаження (hot-reload) у режимі розробки

## Модель безпеки

- **Зберігання облікових даних**: Використовує системне сховище ключів ОС для безпечного зберігання паролів
- **Ізоляція плагінів**: Lua та WASM працюють в ізольованих середовищах
- **Обмеження ресурсів**: Ліміти часу ЦП, обсягу пам'яті та кількості інструкцій для кожного плагіна
- **Обмеження частоти запитів до API**: Враховує заголовки `Retry-After` та налаштовувані затримки
- **Перевірка редагувань**: Попередній перегляд змін перед застосуванням і журнали аудиту
- **Підтримка OAuth**: Рекомендований спосіб автентифікації для ботів

## Ліцензія

Поширюється за вашим вибором на умовах однієї з наведених ліцензій:

- Ліцензія Apache, версія 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) або http://www.apache.org/licenses/LICENSE-2.0)
- Ліцензія MIT ([LICENSE-MIT](../../../LICENSE-MIT) або http://opensource.org/licenses/MIT)

## Внесок у проєкт

Внески вітаються! Перегляньте настанови у файлі [CONTRIBUTING.md](../../../CONTRIBUTING.md).

## Стан проєкту

Це активна переробка AutoWikiBrowser мовою Rust. Поточний стан:

- [x] Основний клієнт API
- [x] Рушій правил
- [x] Інтерфейс командного рядка (CLI)
- [x] Система плагінів (Lua/WASM)
- [x] Ізоляція безпеки (пісочниця)
- [ ] Термінальний інтерфейс (у розробці)
- [ ] Графічний інтерфейс GUI (заплановано)
- [ ] Розширена візуалізація відмінностей (заплановано)

## Посилання

- [Документація](https://docs.rs/awb-rs)
- [Трекер завдань](https://github.com/somethingwithproof/awb-rs/issues)
- [Оригінальний AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
