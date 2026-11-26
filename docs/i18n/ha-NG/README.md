# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Sake rubuta AutoWikiBrowser (AWB) na zamani mai ƙarfin aiki a cikin Rust, wanda ke ba da damar gyaran Wikipedia ta atomatik tare da mai da hankali kan tsaro, sauri, da damar faɗaɗawa.

## Tsarin Gine-gine

```
awb-rs/
├── awb-core/           # Babbar dabara ta aiki da ƙa'idojin tsari
├── awb-api/            # Aiwatar da abokin ciniki na MediaWiki API
├── awb-cli/            # Tsarin layin umarni (CLI)
├── awb-rules/          # Injin ƙa'idoji don gyaran aiki ta atomatik
├── awb-lua-plugin/     # Yanayin gudanar da ƙarin manhajar Lua
├── awb-wasm-plugin/    # Yanayin gudanar da ƙarin manhajar WebAssembly
├── awb-security/       # Tsaro da keɓewar aiki (sandboxing)
├── awb-db/             # Gudanar da adana yanayin aiki na dindindin
├── awb-ui/             # Abubuwan haɗin fuskar tasha (TUI)
├── awb-fixtures/       # Bayanan gwaji da samfuran kwaikwayo
└── awb-integration/    # Jerin gwaje-gwajen haɗaka
```

**Gudanar da Bayanai:**
```
CLI → Cibiya → API → MediaWiki
        ↓
     Injin Ƙa'idoji → Ƙarin Manhajoji (Lua/WASM)
        ↓
     Ɓangaren Tsaro → Gudanarwa a Keɓaɓɓen Wuri
        ↓
     Rumbun Bayanai → Adana Yanayin Aiki
```

## Fasaloli

- **Tsarin Zamani**: Aiki marar jira (`async/await`) tare da injin Tokio don ƙwararren sauri
- **Tallafin Wiki da Dama**: Yana aiki da kowace wiki da ke amfani da MediaWiki
- **Gyara Bisa Ƙa'idoji**: Bayyana ƙa'idojin aiki ta hanyar fayilolin TOML
- **Tsarin Ƙarin Manhajoji**: Ana iya faɗaɗa shi ta hanyar rubutun Lua da sassa na WebAssembly
- **Tsaro Mai Ƙarfi**: Gudanar da ƙarin manhajoji a keɓaɓɓen wuri tare da iyakokin albarkatu
- **Ayyukan Rukunoni**: Sarrafa shafuka masu yawa cikin inganci
- **Fuskar TUI Mai Ma'amala**: Ingantacciyar fuskar tasha don sa ido da sarrafawa
- **Yanayin Gwaji (`dry-run`)**: Duba sauye-sauye kafin aiwatar da su
- **Gudanar da Yanayin Aiki**: Ci gaba da zaman aikin da aka katse
- **Ƙayyade Adadin Buƙatu**: Yana mutunta iyakokin API na wiki da ƙa'idojin aiki mafi kyau
- **Tallafin Harsuna na Duniya**: Harsunan nuni 45 ta hanyar `--lang`, `AWB_LANG`, ko saitin yankin POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`), ciki har da tallafin rubutu daga dama zuwa hagu (RTL) don Larabci, Farisanci, Ibrananci, da Urdu

## Harsunan da Ake Tallafawa

Layin umarni (CLI), rahotannin bot, da fuskar GTK4 suna tallafawa harsuna 45 (ciki har da harsunan RTL guda 6: `ar`, `ar-EG`, `ar-MA`, `fa`, `he`, da `ur`). Yi amfani da `--lang <LANG>` a cikin CLI, ko saita `AWB_LANG` ko masu canjin yanki na POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`) don CLI da fuskar GTK4 duka biyu. Duba [README.md](../../../README.md) don cikakken teburin harsuna.

## Abubuwan da Ake Buƙata

- **Rust**: Sigar 1.94 ko sama da haka (MSRV)
- **Cargo**: Yana zuwa tare da shigar da Rust

## Shigarwa

### Daga Tushen Lamba

```bash
# Ɗauko ma'ajiyar lamba
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Gina dukkan fakitin wurin aiki
cargo build --workspace --release

# Shigar da fayil ɗin gudanarwa
cargo install --path crates/awb_cli
```

## Haɓakawa

### Ginawa

```bash
# Ginin gyaran kurakurai (debug)
cargo build --workspace

# Ginin saki (release)
cargo build --workspace --release
```

### Gwaji

```bash
# Gudanar da dukkan gwaje-gwaje
cargo test --workspace

# Gudanar da gwaje-gwaje tare da nuna cikakken sakamako
cargo test --workspace -- --nocapture
```

### Binciken Lamba (Lint)

```bash
# Bincika matsalolin tsarin lamba
cargo clippy --workspace --all-targets --all-features

# Aiwatar da gyare-gyare ta atomatik
cargo clippy --fix --allow-dirty --workspace
```

### Tsara Lamba (Format)

```bash
# Duba tsarin rubutun lamba
cargo fmt --all -- --check

# Aiwatar da tsarin rubutu
cargo fmt --all
```

## Misalan Amfani

### Tabbatar da Shaida

```bash
# Shiga cikin wiki (yana adana bayanan sirri cikin tsaro)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Shiga ta amfani da OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Jero Shafuka

```bash
# Jero shafuka daga wani rukuni
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Jero shafuka daga sakamakon bincike
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Jero shafuka daga fayil
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Gudanar da Ƙa'idojin Gyara

```bash
# Gudanar da ƙa'idoji tare da bayanin tsari (yanayin gwaji)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Aiwatar da ƙa'idoji a kan shafuka na gaske
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Yanayin Bot na Atomatik

```bash
# Gudanar a yanayin bot marar sa ido tare da iyakoki
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Ci gaba daga zaman aikin baya
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Misalin Bayanin Ƙa'idoji

Ƙirƙiri fayil `my-rules.toml`:

```toml
[profile]
name = "Cleanup Bot"
description = "Gyara matsalolin tsari na yau da kullum"
edit_summary = "Tsaftacewa ta atomatik (Bot)"

[[rules]]
name = "Gyara sarari biyu"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "Gyara hanyoyin waje"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "Ƙara rukunonin da suka ɓata"
plugin = "add_categories.lua"
```

## Tsarin Ƙarin Manhajoji

awb-rs yana tallafawa nau'ikan ƙarin manhajoji guda biyu:

### Ƙarin Manhajojin Lua

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### Ƙarin Manhajojin WebAssembly

```rust
// Haɗa zuwa WASM tare da: cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Fasalolin Ƙarin Manhajoji:**
- Gudanarwa a keɓaɓɓen wuri tare da iyakokin albarkatu (ƙwaƙwalwar ajiya, lokacin CPU)
- Samun dama ga amintaccen ɓangaren API kaɗai (babu damar tsarin fayiloli ko hanyar sadarwa)
- Binciken dacewar sigar manhaja
- Tallafin sake loda nan take a yanayin haɓakawa

## Tsarin Tsaro

- **Adana Bayanan Sirri**: Yana amfani da ma'ajiyar sirri ta tsarin aiki don adana kalmomin sirri cikin tsaro
- **Keɓe Ƙarin Manhajoji**: Lua da WASM suna aiki a cikin keɓaɓɓun muhallai
- **Iyakokin Albarkatu**: Ƙayyadadden lokacin CPU, ƙwaƙwalwar ajiya, da adadin umarnai ga kowane ƙarin manhaja
- **Ƙayyade Buƙatun API**: Yana mutunta kanun `Retry-After` da tazarar jira da za a iya saitawa
- **Tabbatar da Gyare-gyare**: Duba sauye-sauye kafin aiwatarwa, tare da rajistar bincike
- **Tallafin OAuth**: Hanyar tabbatar da shaida da aka fi so don bot

## Lasisi

An bayar da lasisi ƙarƙashin ɗaya daga cikin waɗannan:

- Lasisin Apache, Sifa 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) ko http://www.apache.org/licenses/LICENSE-2.0)
- Lasisin MIT ([LICENSE-MIT](../../../LICENSE-MIT) ko http://opensource.org/licenses/MIT)

bisa zaɓinka.

## Ba da Gudummawa

Ana maraba da masu ba da gudummawa! Da fatan a duba [CONTRIBUTING.md](../../../CONTRIBUTING.md) don ƙa'idoji da jagorori.

## Matsayin Aikin

Wannan sake rubuta AutoWikiBrowser ne da ke ci gaba a cikin Rust. Matsayin yanzu:

- [x] Babban abokin ciniki na API
- [x] Injin ƙa'idoji
- [x] Fuskar layin umarni (CLI)
- [x] Tsarin ƙarin manhajoji (Lua/WASM)
- [x] Keɓewar tsaro (sandboxing)
- [ ] Fuskar tasha ta TUI (ana kan aiki)
- [ ] Fuskar zane ta GUI (an tsara)
- [ ] Ingantaccen nuni na bambancin gyare-gyare (an tsara)

## Hanyoyin Haɗi

- [Takardun Bayani](https://docs.rs/awb-rs)
- [Sashen Bibiyar Matsaloli](https://github.com/somethingwithproof/awb-rs/issues)
- [AWB na Asali](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
