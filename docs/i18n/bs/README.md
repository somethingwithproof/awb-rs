# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Moderna, brza reimplementacija alata AutoWikiBrowser (AWB) u Rustu koja pruža mogućnosti automatizovanog uređivanja Wikipedije s fokusom na sigurnost, performanse i proširivost.

## Arhitektura

```
awb-rs/
├── awb-core/           # Osnovna poslovna logika i apstrakcije
├── awb-api/            # Implementacija MediaWiki API klijenta
├── awb-cli/            # Interfejs komandne linije
├── awb-rules/          # Mehanizam pravila za automatizaciju uređivanja
├── awb-lua-plugin/     # Izvršno okruženje za Lua dodatke
├── awb-wasm-plugin/    # Izvršno okruženje za WebAssembly dodatke
├── awb-security/       # Sigurnost i izolacija (sandboxing)
├── awb-db/             # Trajno upravljanje stanjem
├── awb-ui/             # Komponente terminalnog korisničkog interfejsa
├── awb-fixtures/       # Testni podaci i probni objekti
└── awb-integration/    # Skup integracijskih testova
```

**Tok podataka:**
```
CLI → Core → API → MediaWiki
       ↓
    Mehanizam pravila → Dodaci (Lua/WASM)
       ↓
    Sigurnosni sloj → Izolovano izvršavanje
       ↓
    Baza podataka → Trajno čuvanje stanja
```

## Mogućnosti

- **Moderna arhitektura**: Async/await s Tokio izvršnim okruženjem za visoke performanse
- **Podrška za više wikija**: Rad s bilo kojim wikijem zasnovanim na MediaWiki platformi
- **Uređivanje zasnovano na pravilima**: Deklarativne definicije pravila u TOML formatu
- **Sistem dodataka**: Proširivost putem Lua skripti i WebAssembly modula
- **Ojačana sigurnost**: Izolovano izvršavanje dodataka uz ograničenja resursa
- **Grupne operacije**: Efikasna masovna obrada stranica
- **Interaktivni TUI**: Bogat terminalni interfejs za nadzor i kontrolu
- **Probni način rada (Dry-Run)**: Pregled izmjena prije njihove primjene
- **Upravljanje stanjem**: Nastavak prekinutih sesija
- **Ograničenje brzine**: Poštuje ograničenja wiki API-ja i dobre prakse
- **Internacionalizacija**: 48 jezika prikaza putem `--lang`, `AWB_LANG` ili POSIX lokaliteta (`LC_ALL`/`LC_MESSAGES`/`LANG`), uključujući RTL raspored za arapski, perzijski, hebrejski i urdu

## Podržani jezici

CLI, izvještaji bota i GTK4 korisnički interfejs podržavaju 48 jezika (uključujući 6 RTL lokaliteta: `ar`, `ar-EG`, `ar-MA`, `fa`, `he` i `ur`). Proslijedite `--lang <LANG>` u CLI-u ili postavite `AWB_LANG` odnosno POSIX varijable lokaliteta (`LC_ALL`/`LC_MESSAGES`/`LANG`) za CLI i GTK4 interfejs.

Potpuni pregled svih 48 podržanih jezika i linkove do lokalizovane dokumentacije pronađite u engleskom [README.md](../../../README.md).

## Preduslovi

- **Rust**: verzija 1.94 ili novija (MSRV)
- **Cargo**: uključen uz instalaciju Rusta

## Instalacija

### Iz izvornog koda

```bash
# Klonirajte repozitorij
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Izgradite sve workspace pakete
cargo build --workspace --release

# Instalirajte izvršnu datoteku
cargo install --path crates/awb_cli
```

## Razvoj

### Izgradnja

```bash
# Debug izgradnja
cargo build --workspace

# Release izgradnja
cargo build --workspace --release
```

### Testiranje

```bash
# Pokrenite sve testove
cargo test --workspace

# Pokrenite testove s prikazom izlaza
cargo test --workspace -- --nocapture
```

### Linting

```bash
# Provjerite probleme statičke analize
cargo clippy --workspace --all-targets --all-features

# Primijenite automatske ispravke
cargo clippy --fix --allow-dirty --workspace
```

### Formatiranje

```bash
# Provjerite formatiranje
cargo fmt --all -- --check

# Primijenite formatiranje
cargo fmt --all
```

## Primjeri korištenja

### Autentifikacija

```bash
# Prijava na wiki (sigurno pohranjuje pristupne podatke)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Prijava putem OAuth-a
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Izlistavanje stranica

```bash
# Izlistaj stranice iz kategorije
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Izlistaj stranice iz pretrage
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Izlistaj stranice iz datoteke
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Pokretanje pravila uređivanja

```bash
# Pokreni pravila s profilom (probni način rada)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Primijeni pravila na stvarne stranice
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Automatizovani bot način rada

```bash
# Pokreni u automatskom bot načinu s ograničenjima
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Nastavi iz prethodne sesije
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Primjer profila pravila

Kreirajte datoteku `my-rules.toml`:

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

## Sistem dodataka

awb-rs podržava dva tipa dodataka:

### Lua dodaci

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### WebAssembly dodaci

```rust
// Kompajlirajte u WASM pomoću: cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Mogućnosti dodataka:**
- Izolovano izvršavanje s ograničenjima resursa (memorija, CPU vrijeme)
- Pristup sigurnom podskupu API-ja (bez pristupa datotečnom sistemu ili mreži)
- Provjera kompatibilnosti verzija
- Podrška za automatsko ponovno učitavanje u razvojnom načinu rada

## Sigurnosni model

- **Pohrana pristupnih podataka**: Koristi sistemski privjesak ključeva za sigurnu pohranu lozinki
- **Izolacija dodataka**: Lua i WASM se izvršavaju u izolovanim okruženjima
- **Ograničenja resursa**: Ograničenje CPU vremena, memorije i broja instrukcija po dodatku
- **Ograničenje brzine API-ja**: Poštuje zaglavlja `Retry-After` i podesive pauze
- **Validacija izmjena**: Pregled promjena prije primjene i revizijski dnevnici
- **OAuth podrška**: Preporučena metoda autentifikacije za botove

## Licenca

Licencirano pod jednom od sljedećih licenci, po vašem izboru:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) ili http://www.apache.org/licenses/LICENSE-2.0)
- MIT licenca ([LICENSE-MIT](../../../LICENSE-MIT) ili http://opensource.org/licenses/MIT)

## Doprinos

Doprinosi su dobrodošli! Pogledajte [CONTRIBUTING.md](../../../CONTRIBUTING.md) za smjernice.

## Status projekta

Ovo je aktivna reimplementacija alata AutoWikiBrowser u Rustu. Trenutni status:

- [x] Osnovni API klijent
- [x] Mehanizam pravila
- [x] CLI interfejs
- [x] Sistem dodataka (Lua/WASM)
- [x] Sigurnosna izolacija
- [ ] Terminalni UI (u razvoju)
- [ ] GUI (planirano)
- [ ] Napredna vizualizacija razlika (planirano)

## Linkovi

- [Dokumentacija](https://docs.rs/awb-rs)
- [Praćenje problema](https://github.com/somethingwithproof/awb-rs/issues)
- [Originalni AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
