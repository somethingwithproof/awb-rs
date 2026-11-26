# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

’n Moderne, hoëwerkverrigting-herskrywing van AutoWikiBrowser (AWB) in Rust, wat outomatiese Wikipedia-redigeringsvermoëns bied met ’n fokus op veiligheid, werkverrigting en uitbreibaarheid.

## Argitektuur

```
awb-rs/
├── awb-core/           # Kernbesigheidslogika en abstraksies
├── awb-api/            # MediaWiki API-kliëntimplementering
├── awb-cli/            # Opdraglynkoppelvlak
├── awb-rules/          # Reëlenjin vir redigeringsoutomatisering
├── awb-lua-plugin/     # Lua-inproplooptyd
├── awb-wasm-plugin/    # WebAssembly-inproplooptyd
├── awb-security/       # Sekuriteit en sandkasisolasie
├── awb-db/             # Volgehoue statusbestuur
├── awb-ui/             # Terminaal-gebruikerskoppelvlakkomponente
├── awb-fixtures/       # Toetsvoorwerpe en skyndata
└── awb-integration/    # Integrasietoetsreeks
```

**Datavloei:**
```
CLI → Kern → API → MediaWiki
       ↓
    Reëlenjin → Inproppe (Lua/WASM)
       ↓
    Sekuriteitslaag → Gesandkaste Uitvoering
       ↓
    Databasis → Statusbewaring
```

## Kenmerke

- **Moderne argitektuur**: Asinkrone uitvoering (`async/await`) met die Tokio-looptyd vir hoë werkverrigting
- **Multi-wiki-ondersteuning**: Werk met enige MediaWiki-gedrewe wiki
- **Reëlgebaseerde redigering**: Deklaratiewe TOML-gebaseerde reëldefinisies
- **Inpropstelsel**: Uitbreibaar deur Lua-skripte en WebAssembly-modules
- **Versterkte sekuriteit**: Gesandkaste inpropuitvoering met hulpbronbeperkings
- **Groepbewerkings**: Doeltreffende massaverwerking van bladsye
- **Interaktiewe TUI**: Ryk terminaal-koppelvlak vir monitering en beheer
- **Proeflopiemodus (Dry-Run)**: Besigtig veranderings vooraf voordat dit toegepas word
- **Statusbestuur**: Hervat onderbroke sessies
- **Tempobeperking**: Respekteer wiki-API-perke en beste praktyke
- **Internasionalisering**: 45 vertoontale via `--lang`, `AWB_LANG`, of POSIX-lokaliteit (`LC_ALL`/`LC_MESSAGES`/`LANG`), insluitend RTL-uitlegondersteuning vir Arabies, Persies, Hebreeus en Oerdoe

## Ondersteunde tale

Die CLI, botverslae en GTK4-gebruikerskoppelvlak ondersteun 45 tale (insluitend 6 RTL-lokaliteite: `ar`, `ar-EG`, `ar-MA`, `fa`, `he`, en `ur`). Gebruik `--lang <LANG>` in die CLI, of stel `AWB_LANG` of POSIX-lokaliteitsveranderlikes (`LC_ALL`/`LC_MESSAGES`/`LANG`) vir beide die CLI en die GTK4-gebruikerskoppelvlak. Sien [README.md](../../../README.md) vir die volledige taaltabel.

## Voorvereistes

- **Rust**: Weergawe 1.94 of hoër (MSRV)
- **Cargo**: Ingesluit by die Rust-installasie

## Installasie

### Vanuit bronkode

```bash
# Kloon die bewaarplek
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Bou alle werkspasie-kratte
cargo build --workspace --release

# Installeer die uitvoerbare lêer
cargo install --path crates/awb_cli
```

## Ontwikkeling

### Bou

```bash
# Ontfoutbou
cargo build --workspace

# Vrystellingsbou
cargo build --workspace --release
```

### Toetsing

```bash
# Voer alle toetse uit
cargo test --workspace

# Voer toetse uit met volledige afvoer
cargo test --workspace -- --nocapture
```

### Kode-analise (Lint)

```bash
# Kontroleer vir kodekwaliteitprobleme
cargo clippy --workspace --all-targets --all-features

# Pas outomatiese regstellings toe
cargo clippy --fix --allow-dirty --workspace
```

### Formatering

```bash
# Kontroleer kodeformatering
cargo fmt --all -- --check

# Pas kodeformatering toe
cargo fmt --all
```

## Gebruiksvoorbeelde

### Verifikasie

```bash
# Meld aan by ’n wiki (berg geloofsbriewe veilig)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Meld aan met OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Lys van bladsye

```bash
# Lys bladsye vanuit ’n kategorie
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Lys bladsye vanuit ’n soektog
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Lys bladsye vanuit ’n lêer
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Uitvoer van redigeringsreëls

```bash
# Voer reëls uit met ’n profiel (proeflopiemodus)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Pas reëls toe op werklike bladsye
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Outomatiese botmodus

```bash
# Voer in onbewaakte botmodus uit met perke
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Hervat vanaf ’n vorige sessie
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Voorbeeld-reëlprofiel

Skep ’n lêer genaamd `my-rules.toml`:

```toml
[profile]
name = "Skoonmaakbot"
description = "Herstel algemene formateringsprobleme"
edit_summary = "Outomatiese skoonmaak (Bot)"

[[rules]]
name = "Herstel dubbele spasies"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "Herstel eksterne skakels"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "Voeg ontbrekende kategorieë by"
plugin = "add_categories.lua"
```

## Inpropstelsel

awb-rs ondersteun twee soorte inproppe:

### Lua-inproppe

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### WebAssembly-inproppe

```rust
// Kompileer na WASM met: cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Inpropkenmerke:**
- Gesandkaste uitvoering met hulpbronbeperkings (geheue, CPU-tyd)
- Toegang tot ’n veilige deelversameling van API's (geen lêerstelsel- of netwerktoegang nie)
- Kontrole vir weergaweversoenbaarheid
- Ondersteuning vir vinnige herlaai in ontwikkelingsmodus

## Sekuriteitsmodel

- **Berging van geloofsbriewe**: Gebruik die bedryfstelsel se sleutelring vir veilige wagwoordberging
- **Inprop-sandkas**: Lua en WASM loop in geïsoleerde omgewings
- **Hulpbronbeperkings**: Beperkings op CPU-tyd, geheue en instruksietelling per inprop
- **API-tempobeperking**: Respekteer `Retry-After`-opskrifte en konfigureerbare vertragings
- **Redigeringsvalidering**: Voorskou van veranderings voor toepassing en ouditlogboeke
- **OAuth-ondersteuning**: Voorkeur-verifikasiemetode vir bots

## Lisensie

Gelisensieer onder enige een van:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) of http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../../../LICENSE-MIT) of http://opensource.org/licenses/MIT)

na u keuse.

## Bydraes

Bydraes is welkom! Raadpleeg asseblief [CONTRIBUTING.md](../../../CONTRIBUTING.md) vir riglyne.

## Projekstatus

Hierdie is ’n aktiewe herskrywing van AutoWikiBrowser in Rust. Huidige status:

- [x] Kern-API-kliënt
- [x] Reëlenjin
- [x] CLI-koppelvlak
- [x] Inpropstelsel (Lua/WASM)
- [x] Sekuriteitsandkas
- [ ] Terminaal-gebruikerskoppelvlak (aan die gang)
- [ ] GUI (beplan)
- [ ] Gevorderde verskilvisualisering (beplan)

## Skakels

- [Dokumentasie](https://docs.rs/awb-rs)
- [Probleemopsporing](https://github.com/somethingwithproof/awb-rs/issues)
- [Oorspronklike AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
