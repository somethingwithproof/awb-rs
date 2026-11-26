# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

En modern och högpresterande nyimplementering av AutoWikiBrowser (AWB) i Rust, som erbjuder automatiserade funktioner för redigering på Wikipedia med fokus på säkerhet, prestanda och utbyggbarhet.

## Arkitektur

```
awb-rs/
├── awb-core/           # Central affärslogik och abstraktioner
├── awb-api/            # Implementation av MediaWiki API-klienten
├── awb-cli/            # Kommandoradsgränssnitt
├── awb-rules/          # Regelmotor för redigeringsautomatisering
├── awb-lua-plugin/     # Körtidsmiljö för Lua-insticksmoduler
├── awb-wasm-plugin/    # Körtidsmiljö för WebAssembly-insticksmoduler
├── awb-security/       # Säkerhet och sandlådeisolering
├── awb-db/             # Beständig tillståndshantering
├── awb-ui/             # Komponenter för terminalgränssnitt
├── awb-fixtures/       # Testfixturer och mockdata
└── awb-integration/    # Svit för integrationstester
```

**Dataflöde:**
```
CLI → Core → API → MediaWiki
       ↓
    Regelmotor → Insticksmoduler (Lua/WASM)
       ↓
    Säkerhetslager → Körning i sandlåda
       ↓
    Databas → Tillståndslagring
```

## Funktioner

- **Modern arkitektur**: Async/await med Tokio-körtidsmiljön för hög prestanda
- **Stöd för flera wikier**: Fungerar med valfri MediaWiki-driven wiki
- **Regelbaserad redigering**: Deklarativa TOML-baserade regeldefinitioner
- **Insticksmodulsystem**: Utbyggbart via Lua-skript och WebAssembly-moduler
- **Förstärkt säkerhet**: Isolerad körning av insticksmoduler i sandlåda med resursgränser
- **Batchoperationer**: Effektiv massbearbetning av sidor
- **Interaktiv TUI**: Rikt terminalgränssnitt för övervakning och styrning
- **Torrkörningsläge (dry-run)**: Förhandsgranska ändringar innan de tillämpas
- **Tillståndshantering**: Återuppta avbrutna sessioner
- **Hastighetsbegränsning**: Respekterar wikins API-gränser och bästa praxis
- **Internationalisering**: 45 visningsspråk via `--lang`, `AWB_LANG` eller POSIX-lokal (`LC_ALL`/`LC_MESSAGES`/`LANG`), inklusive stöd för RTL-layout för arabiska, persiska, hebreiska och urdu

## Språk som stöds

CLI, botrapporter och GTK4-gränssnittet stöder 45 språk (inklusive 6 RTL-lokaler: `ar`, `ar-EG`, `ar-MA`, `fa`, `he` och `ur`). Använd `--lang <LANG>` i CLI, eller sätt `AWB_LANG` eller POSIX-lokalvariabler (`LC_ALL`/`LC_MESSAGES`/`LANG`) för både CLI och GTK4-gränssnittet.

Se den fullständiga tabellen över alla 45 språk som stöds och länkar till respektive lokaliserad dokumentation i [README.md](../../../README.md).

## Förutsättningar

- **Rust**: Version 1.94 eller senare (MSRV)
- **Cargo**: Ingår i Rust-installationen

## Installation

### Från källkod

```bash
# Klona arkivet
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Bygg alla crates i arbetsytan
cargo build --workspace --release

# Installera binären
cargo install --path crates/awb_cli
```

## Utveckling

### Bygga

```bash
# Felsökningsbygge (debug)
cargo build --workspace

# Produktionsbygge (release)
cargo build --workspace --release
```

### Testa

```bash
# Kör alla tester
cargo test --workspace

# Kör tester med fullständig utdata
cargo test --workspace -- --nocapture
```

### Kodgranskning (Lint)

```bash
# Kontrollera lint-problem
cargo clippy --workspace --all-targets --all-features

# Tillämpa automatiska korrigeringar
cargo clippy --fix --allow-dirty --workspace
```

### Formatering

```bash
# Kontrollera kodformatering
cargo fmt --all -- --check

# Tillämpa kodformatering
cargo fmt --all
```

## Användningsexempel

### Autentisering

```bash
# Logga in på en wiki (lagrar inloggningsuppgifter säkert)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Logga in med OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Lista sidor

```bash
# Lista sidor från en kategori
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Lista sidor från en sökning
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Lista sidor från en fil
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Köra redigeringsregler

```bash
# Kör regler med en profil (dry-run-läge)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Tillämpa regler på riktiga sidor
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Automatiserat botläge

```bash
# Kör i obevakat botläge med gränser
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Återuppta från en tidigare session
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Exempel på regelprofil

Skapa en fil `my-rules.toml`:

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

## Insticksmodulsystem

awb-rs stöder två typer av insticksmoduler:

### Lua-insticksmoduler

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### WebAssembly-insticksmoduler

```rust
// Kompilera till WASM med: cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Funktioner för insticksmoduler:**
- Körning i sandlåda med resursgränser (minne, CPU-tid)
- Åtkomst till en säker delmängd av API:er (ingen filsystem- eller nätverksåtkomst)
- Kontroll av versionskompatibilitet
- Stöd för omladdning under körning (hot-reload) i utvecklingsläge

## Säkerhetsmodell

- **Lagring av autentiseringsuppgifter**: Använder operativsystemets nyckelring för säker lösenordslagring
- **Sandlådeisolering av insticksmoduler**: Lua och WASM körs i isolerade miljöer
- **Resursgränser**: Gränser för CPU-tid, minne och antal instruktioner per insticksmodul
- **API-hastighetsbegränsning**: Respekterar `Retry-After`-huvuden och konfigurerbara fördröjningar
- **Redigeringsvalidering**: Förhandsgranskning av ändringar före tillämpning samt granskningsloggar
- **OAuth-stöd**: Rekommenderad autentiseringsmetod för botar

## Licens

Licensierad enligt något av följande alternativ, efter eget val:

- Apache-licensen, version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) eller http://www.apache.org/licenses/LICENSE-2.0)
- MIT-licensen ([LICENSE-MIT](../../../LICENSE-MIT) eller http://opensource.org/licenses/MIT)

## Bidra

Bidrag är välkomna! Se [CONTRIBUTING.md](../../../CONTRIBUTING.md) för riktlinjer.

## Projektstatus

Detta är en aktiv nyimplementering av AutoWikiBrowser i Rust. Aktuell status:

- [x] Kärnklient för API
- [x] Regelmotor
- [x] CLI-gränssnitt
- [x] Insticksmodulsystem (Lua/WASM)
- [x] Säkerhetssandlåda
- [ ] Terminalgränssnitt (pågår)
- [ ] Grafiskt gränssnitt GUI (planerat)
- [ ] Avancerad diff-visualisering (planerad)

## Länkar

- [Dokumentation](https://docs.rs/awb-rs)
- [Ärendehantering](https://github.com/somethingwithproof/awb-rs/issues)
- [Ursprungliga AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
