# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Een moderne, krachtige herimplementatie van AutoWikiBrowser (AWB) in Rust, die geautomatiseerde Wikipedia-bewerkingsmogelijkheden biedt met de nadruk op veiligheid, prestaties en uitbreidbaarheid.

## Architectuur

```
awb-rs/
├── awb-core/           # Kernbedrijfslogica en abstracties
├── awb-api/            # Implementatie van de MediaWiki-API-client
├── awb-cli/            # Commandoregelinterface
├── awb-rules/          # Regelengine voor bewerkingsautomatisering
├── awb-lua-plugin/     # Runtime voor Lua-plugins
├── awb-wasm-plugin/    # Runtime voor WebAssembly-plugins
├── awb-security/       # Beveiliging en sandboxing
├── awb-db/             # Persistente statusbeheerlaag
├── awb-ui/             # Terminal-UI-componenten
├── awb-fixtures/       # Testfixtures en mockgegevens
└── awb-integration/    # Integratietestsuite
```

**Gegevensstroom:**
```
CLI → Core → API → MediaWiki
       ↓
    Regelengine → Plugins (Lua/WASM)
       ↓
    Beveiligingslaag → Uitvoering in sandbox
       ↓
    Database → Statuspersistentie
```

## Functionaliteiten

- **Moderne architectuur**: Async/await met de Tokio-runtime voor hoge prestaties
- **Multi-wiki-ondersteuning**: Werkt met elke MediaWiki-gebaseerde wiki
- **Regelgebaseerd bewerken**: Declaratieve, TOML-gebaseerde regeldefinities
- **Pluginsysteem**: Uitbreidbaar via Lua-scripts en WebAssembly-modules
- **Versterkte beveiliging**: Geïsoleerde uitvoering van plugins in een sandbox met resourcelimieten
- **Batchbewerkingen**: Efficiënte bulkverwerking van pagina's
- **Interactieve TUI**: Uitgebreide terminalinterface voor monitoring en besturing
- **Proefdraaimodus (dry-run)**: Bekijk wijzigingen vooraf voordat ze worden toegepast
- **Statusbeheer**: Hervat onderbroken sessies
- **Snelheidsbeperking**: Respecteert wiki-API-limieten en best practices
- **Internationalisatie**: 45 weergavetalen via `--lang`, `AWB_LANG` of POSIX-locale (`LC_ALL`/`LC_MESSAGES`/`LANG`), inclusief ondersteuning voor RTL-weergave voor Arabisch, Perzisch, Hebreeuws en Urdu

## Ondersteunde talen

De CLI, botrapporten en GTK4-gebruikersinterface ondersteunen 45 talen (waaronder 6 RTL-locales: `ar`, `ar-EG`, `ar-MA`, `fa`, `he` en `ur`). Geef `--lang <LANG>` mee of stel `AWB_LANG` in om de automatische detectie van de systeemlocale te overschrijven.

Zie [README.md](../../../README.md) voor het volledige overzicht van alle 45 ondersteunde talen en koppelingen naar de gelokaliseerde documentatie.

## Vereisten

- **Rust**: Versie 1.85 of hoger (MSRV)
- **Cargo**: Meegeleverd bij de Rust-installatie

## Installatie

### Vanuit de broncode

```bash
# Repository klonen
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Alle workspace-crates compileren
cargo build --workspace --release

# Het uitvoerbare bestand installeren
cargo install --path awb-cli
```

## Ontwikkeling

### Compileren

```bash
# Debug-build
cargo build --workspace

# Release-build
cargo build --workspace --release
```

### Testen

```bash
# Alle tests uitvoeren
cargo test --workspace

# Tests uitvoeren met volledige uitvoer
cargo test --workspace -- --nocapture
```

### Linting

```bash
# Controleren op linting-problemen
cargo clippy --workspace --all-targets --all-features

# Automatische oplossingen toepassen
cargo clippy --fix --allow-dirty --workspace
```

### Opmaak

```bash
# Code-opmaak controleren
cargo fmt --all -- --check

# Code-opmaak toepassen
cargo fmt --all
```

## Gebruiksvoorbeelden

### Authenticatie

```bash
# Aanmelden bij een wiki (slaat inloggegevens veilig op)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Aanmelden met OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Pagina's oplijsten

```bash
# Pagina's uit een categorie oplijsten
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Pagina's uit een zoekopdracht oplijsten
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Pagina's uit een bestand oplijsten
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Bewerkingsregels uitvoeren

```bash
# Regels uitvoeren met een profiel (dry-run-modus)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Regels toepassen op echte pagina's
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Geautomatiseerde botmodus

```bash
# Uitvoeren in onbeheerde botmodus met limieten
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Vorige sessie hervatten
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Voorbeeld van een regelprofiel

Maak een bestand `my-rules.toml` aan:

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

## Pluginsysteem

awb-rs ondersteunt twee typen plugins:

### Lua-plugins

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### WebAssembly-plugins

```rust
// Compileren naar WASM met: cargo build --target wasm32-wasi
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Pluginfunctionaliteiten:**
- Uitvoering in een sandbox met resourcelimieten (geheugen, CPU-tijd)
- Toegang tot een veilige deelverzameling van API's (geen bestandssysteem- of netwerktoegang)
- Controle op versiecompatibiliteit
- Ondersteuning voor hot-reload in ontwikkelingsmodus

## Beveiligingsmodel

- **Opslag van inloggegevens**: Gebruikt de sleutelhanger van het besturingssysteem voor veilige wachtwoordopslag
- **Plugin-sandboxing**: Lua en WASM draaien in geïsoleerde omgevingen
- **Resourcelimieten**: Limieten op CPU-tijd, geheugen en instructieaantal per plugin
- **API-snelheidsbeperking**: Respecteert `Retry-After`-headers en configureerbare vertragingen
- **Bewerkingsvalidatie**: Voorbeeldweergave van wijzigingen vóór toepassing en auditlogboeken
- **OAuth-ondersteuning**: Voorkeursmethode voor authenticatie van bots

## Licentie

Gelicenseerd onder een van de volgende licenties naar keuze:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) of http://www.apache.org/licenses/LICENSE-2.0)
- MIT-licentie ([LICENSE-MIT](../../../LICENSE-MIT) of http://opensource.org/licenses/MIT)

## Bijdragen

Bijdragen zijn welkom! Raadpleeg [CONTRIBUTING.md](../../../CONTRIBUTING.md) voor de richtlijnen.

## Projectstatus

Dit is een actieve herimplementatie van AutoWikiBrowser in Rust. Huidige status:

- [x] Kern-API-client
- [x] Regelengine
- [x] CLI-interface
- [x] Pluginsysteem (Lua/WASM)
- [x] Beveiligingssandboxing
- [ ] Terminal-UI (in ontwikkeling)
- [ ] GUI (gepland)
- [ ] Geavanceerde diff-visualisatie (gepland)

## Koppelingen

- [Documentatie](https://docs.rs/awb-rs)
- [Issue-tracker](https://github.com/somethingwithproof/awb-rs/issues)
- [Originele AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
