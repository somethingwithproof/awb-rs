# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Suvremena, brza ponovna izvedba alata AutoWikiBrowser (AWB) u Rustu koja omogućuje automatizirano uređivanje Wikipedije s naglaskom na sigurnost, performanse i proširivost.

## Arhitektura

```
awb-rs/
├── awb-core/           # Središnja poslovna logika i apstrakcije
├── awb-api/            # Implementacija MediaWiki API klijenta
├── awb-cli/            # Sučelje naredbenog retka
├── awb-rules/          # Mehanizam pravila za automatizaciju uređivanja
├── awb-lua-plugin/     # Izvršna okolina za Lua dodatke
├── awb-wasm-plugin/    # Izvršna okolina za WebAssembly dodatke
├── awb-security/       # Sigurnost i izolacija (sandboxing)
├── awb-db/             # Trajno upravljanje stanjem
├── awb-ui/             # Komponente terminalnog korisničkog sučelja
├── awb-fixtures/       # Testni podaci i lažni objekti
└── awb-integration/    # Skup integracijskih testova
```

**Tijek podataka:**
```
CLI → Core → API → MediaWiki
       ↓
    Mehanizam pravila → Dodaci (Lua/WASM)
       ↓
    Sigurnosni sloj → Izolirano izvršavanje
       ↓
    Baza podataka → Trajna pohrana stanja
```

## Značajke

- **Suvremena arhitektura**: Async/await s izvršnom okolinom Tokio za visoke performanse
- **Podrška za više wikija**: Rad s bilo kojim wikijem pokretanim sustavom MediaWiki
- **Uređivanje temeljeno na pravilima**: Deklarativne definicije pravila u TOML formatu
- **Sustav dodataka**: Proširivost putem Lua skripti i WebAssembly modula
- **Ojačana sigurnost**: Izolirano izvršavanje dodataka uz ograničenja resursa
- **Skupne operacije**: Učinkovita masovna obrada stranica
- **Interaktivni TUI**: Bogato terminalno sučelje za nadzor i upravljanje
- **Probni način rada (Dry-Run)**: Pretpregled izmjena prije primjene
- **Upravljanje stanjem**: Nastavak prekinutih sesija
- **Ograničenje učestalosti**: Poštuje ograničenja wiki API-ja i dobre prakse
- **Internacionalizacija**: 48 jezika prikaza putem `--lang`, `AWB_LANG` ili POSIX lokaliteta (`LC_ALL`/`LC_MESSAGES`/`LANG`), uključujući RTL raspored za arapski, perzijski, hebrejski i urdu

## Podržani jezici

CLI, izvješća bota i GTK4 korisničko sučelje podržavaju 48 jezika (uključujući 6 RTL lokaliteta: `ar`, `ar-EG`, `ar-MA`, `fa`, `he` i `ur`). Proslijedite `--lang <LANG>` u CLI-ju ili postavite `AWB_LANG` odnosno POSIX varijable lokaliteta (`LC_ALL`/`LC_MESSAGES`/`LANG`) za CLI i GTK4 sučelje.

Potpuni pregled svih 48 podržanih jezika i poveznice na lokaliziranu dokumentaciju pronađite u engleskoj datoteci [README.md](../../../README.md).

## Preduvjeti

- **Rust**: inačica 1.94 ili novija (MSRV)
- **Cargo**: uključen uz instalaciju Rusta

## Instalacija

### Iz izvornog koda

```bash
# Klonirajte repozitorij
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Izgradite sve pakete radnog prostora
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

# Pokrenite testove s ispisom izlaza
cargo test --workspace -- --nocapture
```

### Linting

```bash
# Provjerite pogreške statičke analize
cargo clippy --workspace --all-targets --all-features

# Primijenite automatske popravke
cargo clippy --fix --allow-dirty --workspace
```

### Oblikovanje koda

```bash
# Provjerite oblikovanje
cargo fmt --all -- --check

# Primijenite oblikovanje
cargo fmt --all
```

## Primjeri uporabe

### Provjera autentičnosti

```bash
# Prijava na wiki (sigurno pohranjuje vjerodajnice)
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

### Dohvaćanje popisa stranica

```bash
# Dohvati stranice iz kategorije
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Dohvati stranice iz pretraživanja
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Dohvati stranice iz datoteke
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

### Automatizirani bot način rada

```bash
# Pokreni u nenadziranom bot načinu s ograničenjima
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

Stvorite datoteku `my-rules.toml`:

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

## Sustav dodataka

awb-rs podržava dvije vrste dodataka:

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
// Prevedite u WASM pomoću: cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Značajke dodataka:**
- Izolirano izvršavanje s ograničenjima resursa (memorija, CPU vrijeme)
- Pristup sigurnom podskupu API-ja (bez pristupa datotečnom sustavu ili mreži)
- Provjera kompatibilnosti inačica
- Podrška za automatsko ponovno učitavanje u razvojnom načinu rada

## Sigurnosni model

- **Pohrana vjerodajnica**: Koristi privjesak ključeva operacijskog sustava za sigurnu pohranu zaporki
- **Izolacija dodataka**: Lua i WASM izvršavaju se u izoliranim okolinama
- **Ograničenja resursa**: Ograničenja CPU vremena, memorije i broja instrukcija po dodatku
- **Ograničenje učestalosti API-ja**: Poštuje zaglavlja `Retry-After` i podesiva kašnjenja
- **Provjera izmjena**: Pretpregled promjena prije primjene i revizijski zapisnici
- **Podrška za OAuth**: Preporučena metoda provjere autentičnosti za botove

## Licenca

Licencirano pod jednom od sljedećih licenci, po vašem izboru:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) ili http://www.apache.org/licenses/LICENSE-2.0)
- MIT licenca ([LICENSE-MIT](../../../LICENSE-MIT) ili http://opensource.org/licenses/MIT)

## Doprinosi

Doprinosi su dobrodošli! Smjernice potražite u datoteci [CONTRIBUTING.md](../../../CONTRIBUTING.md).

## Status projekta

Ovo je aktivna ponovna izvedba alata AutoWikiBrowser u Rustu. Trenutni status:

- [x] Osnovni API klijent
- [x] Mehanizam pravila
- [x] CLI sučelje
- [x] Sustav dodataka (Lua/WASM)
- [x] Sigurnosna izolacija
- [ ] Terminalno sučelje (u izradi)
- [ ] GUI (planirano)
- [ ] Napredna vizualizacija razlika (planirano)

## Poveznice

- [Dokumentacija](https://docs.rs/awb-rs)
- [Praćenje problema](https://github.com/somethingwithproof/awb-rs/issues)
- [Izvorni AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
