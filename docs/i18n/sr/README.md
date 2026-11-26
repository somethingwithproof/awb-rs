# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Модерна, брза реимплементација алата AutoWikiBrowser (AWB) у Rust-у која пружа могућности аутоматизованог уређивања Википедије са фокусом на безбедност, перформансе и проширивост.

## Архитектура

```
awb-rs/
├── awb-core/           # Основна пословна логика и апстракције
├── awb-api/            # Имплементација MediaWiki API клијента
├── awb-cli/            # Интерфејс командне линије
├── awb-rules/          # Механизам правила за аутоматизацију уређивања
├── awb-lua-plugin/     # Извршно окружење за Lua додатке
├── awb-wasm-plugin/    # Извршно окружење за WebAssembly додатке
├── awb-security/       # Безбедност и изолација (sandboxing)
├── awb-db/             # Трајно управљање стањем
├── awb-ui/             # Компоненте терминалног корисничког интерфејса
├── awb-fixtures/       # Тестни подаци и пробни објекти
└── awb-integration/    # Скуп интеграционих тестова
```

**Ток података:**
```
CLI → Core → API → MediaWiki
       ↓
    Механизам правила → Додаци (Lua/WASM)
       ↓
    Безбедносни слој → Изоловано извршавање
       ↓
    База података → Трајно чување стања
```

## Могућности

- **Модерна архитектура**: Async/await са Tokio извршним окружењем за високе перформансе
- **Подршка за више викија**: Рад са било којим викијем заснованим на MediaWiki платформи
- **Уређивање засновано на правилима**: Декларативне дефиниције правила у TOML формату
- **Систем додатака**: Проширивост путем Lua скрипти и WebAssembly модула
- **Ојачана безбедност**: Изоловано извршавање додатака уз ограничења ресурса
- **Групне операције**: Ефикасна масовна обрада страница
- **Интерактивни TUI**: Богат терминални интерфејс за надзор и контролу
- **Пробни режим (Dry-Run)**: Преглед измена пре њихове примене
- **Управљање стањем**: Наставак прекинутих сесија
- **Ограничење брзине**: Поштује ограничења вики API-ја и добре праксе
- **Интернационализација**: 48 језика приказа путем `--lang`, `AWB_LANG` или POSIX локалитета (`LC_ALL`/`LC_MESSAGES`/`LANG`), укључујући RTL распоред за арапски, персијски, хебрејски и урду

## Подржани језици

CLI, извештаји бота и GTK4 кориснички интерфејс подржавају 48 језика (укључујући 6 RTL локалитета: `ar`, `ar-EG`, `ar-MA`, `fa`, `he` и `ur`). Проследите `--lang <LANG>` у CLI-у или поставите `AWB_LANG` односно POSIX променљиве локалитета (`LC_ALL`/`LC_MESSAGES`/`LANG`) за CLI и GTK4 интерфејс.

Потпуни преглед свих 48 подржаних језика и везе ка локализованој документацији потражите у енглеској датотеци [README.md](../../../README.md).

## Предуслови

- **Rust**: верзија 1.94 или новија (MSRV)
- **Cargo**: укључен уз инсталацију Rust-а

## Инсталација

### Из изворног кода

```bash
# Клонирајте репозиторијум
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Изградите све workspace пакете
cargo build --workspace --release

# Инсталирајте извршну датотеку
cargo install --path crates/awb_cli
```

## Развој

### Изградња

```bash
# Debug изградња
cargo build --workspace

# Release изградња
cargo build --workspace --release
```

### Тестирање

```bash
# Покрените све тестове
cargo test --workspace

# Покрените тестове са приказом излаза
cargo test --workspace -- --nocapture
```

### Статичка анализа (Lint)

```bash
# Проверите проблеме статичке анализе
cargo clippy --workspace --all-targets --all-features

# Примените аутоматске исправке
cargo clippy --fix --allow-dirty --workspace
```

### Форматирање

```bash
# Проверите форматирање
cargo fmt --all -- --check

# Примените форматирање
cargo fmt --all
```

## Примери коришћења

### Аутентификација

```bash
# Пријава на вики (безбедно чува приступне податке)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Пријава путем OAuth-а
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Листање страница

```bash
# Излистај странице из категорије
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Излистај странице из претраге
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Излистај странице из датотеке
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Покретање правила уређивања

```bash
# Покрени правила са профилом (пробни режим)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Примени правила на стварне странице
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Аутоматизовани бот режим

```bash
# Покрени у аутоматском бот режиму са ограничењима
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Настави из претходне сесије
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Пример профила правила

Креирајте датотеку `my-rules.toml`:

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

## Систем додатака

awb-rs подржава два типа додатака:

### Lua додаци

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### WebAssembly додаци

```rust
// Преведите у WASM помоћу: cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Могућности додатака:**
- Изоловано извршавање са ограничењима ресурса (меморија, CPU време)
- Приступ безбедном подскупу API-ја (без приступа систему датотека или мрежи)
- Провера компатибилности верзија
- Подршка за аутоматско поновно учитавање у развојном режиму

## Безбедносни модел

- **Чување приступних података**: Користи системски привезак кључева за безбедно чување лозинки
- **Изолација додатака**: Lua и WASM се извршавају у изолованим окружењима
- **Ограничења ресурса**: Ограничење CPU времена, меморије и броја инструкција по додатку
- **Ограничење брзине API-ја**: Поштује заглавља `Retry-After` и подесива кашњења
- **Валидација измена**: Преглед промена пре примене и ревизијски дневници
- **OAuth подршка**: Препоручена метода аутентификације за ботове

## Лиценца

Лиценцирано под једном од следећих лиценци, по вашем избору:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) или http://www.apache.org/licenses/LICENSE-2.0)
- MIT лиценца ([LICENSE-MIT](../../../LICENSE-MIT) или http://opensource.org/licenses/MIT)

## Допринос

Доприноси су добродошли! Погледајте [CONTRIBUTING.md](../../../CONTRIBUTING.md) за смернице.

## Статус пројекта

Ово је активна реимплементација алата AutoWikiBrowser у Rust-у. Тренутни статус:

- [x] Основни API клијент
- [x] Механизам правила
- [x] CLI интерфејс
- [x] Систем додатака (Lua/WASM)
- [x] Безбедносна изолација
- [ ] Терминални UI (у развоју)
- [ ] GUI (планирано)
- [ ] Напредна визуализација разлика (планирано)

## Везе

- [Документација](https://docs.rs/awb-rs)
- [Праћење проблема](https://github.com/somethingwithproof/awb-rs/issues)
- [Оригинални AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
