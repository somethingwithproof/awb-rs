# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Famerenana manoratra maoderina sy matanjaka an'ny AutoWikiBrowser (AWB) amin'ny Rust, manome fahaizana manova Wikipedia mandeha ho azy ary mifantoka amin'ny fiarovana, ny fahombiazana, ary ny fahafaha-manitatra.

## Firafitra

```
awb-rs/
├── awb-core/           # Loika fototra sy rafitra ankapobeny
├── awb-api/            # Fampiharana mpanjifa MediaWiki API
├── awb-cli/            # Fifandraisana amin'ny baikon'ny terminal (CLI)
├── awb-rules/          # Motera fitsipika ho an'ny fanovana mandeha ho azy
├── awb-lua-plugin/     # Tontolo fampandehanana plugin Lua
├── awb-wasm-plugin/    # Tontolo fampandehanana plugin WebAssembly
├── awb-security/       # Fiarovana sy fanokanan-toerana (sandboxing)
├── awb-db/             # Fitantanana fitahirizana toetra maharitra
├── awb-ui/             # Singa ho an'ny fifandraisana terminal (TUI)
├── awb-fixtures/       # Angona fitsapana sy angona fakana tahaka
└── awb-integration/    # Fitambarana fitsapana fampidirana
```

**Fikorianan'ny angona:**
```
CLI → Fototra → API → MediaWiki
         ↓
     Motera fitsipika → Plugin (Lua/WASM)
         ↓
     Sosona fiarovana → Fampandehanana mitoka-monina
         ↓
     Tahirin-kevitra → Fitahirizana toetra
```

## Endri-javatra

- **Firafitra maoderina**: Fampandehanana tsy mifampiandry (`async/await`) miaraka amin'ny tontolo Tokio ho an'ny fahombiazana ambony
- **Fanohanana wiki maro**: Miasa amin'ny wiki rehetra mampiasa MediaWiki
- **Fanovana mifototra amin'ny fitsipika**: Famaritana fitsipika mazava amin'ny alalan'ny TOML
- **Rafitra plugin**: Azo itarina amin'ny alalan'ny skripta Lua sy maody WebAssembly
- **Fiarovana nohamafisina**: Fampandehanana plugin mitoka-monina misy fetran'ny loharanon-karena
- **Fikarakarana marobe miaraka**: Fikarakarana pejy maro amin'ny fomba mahomby
- **TUI ifandraisana**: Fifandraisana terminal manankarena ho an'ny fanaraha-maso sy ny fitantanana
- **Fomba fanandramana (`dry-run`)**: Fijerena mialoha ny fanovana alohan'ny hampiharana azy
- **Fitantanana ny toetra**: Famerenana manohy fivoriana tapaka
- **Famerana ny hafainganam-pandeha**: Manaja ny fetran'ny API wiki sy ny fanao tsara indrindra
- **Fampiasana fiteny maro**: Fiteny fampisehoana 45 amin'ny alalan'ny `--lang`, `AWB_LANG`, na ny mari-paritra POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`), ao anatin'izany ny fanohanana ny fisehoana avy ankavanana miankavia (RTL) ho an'ny Arabo, Persiana, Hebreo, ary Urdu

## Fiteny tohanana

Ny CLI, ny tatitry ny baoty, ary ny fifandraisana GTK4 dia manohana fiteny 45 (anisan'izany ny fiteny RTL 6: `ar`, `ar-EG`, `ar-MA`, `fa`, `he`, ary `ur`). Ampiasao ny `--lang <LANG>` na ovay ny `AWB_LANG` mba hanoloana ny fiteny hita ho azy avy amin'ny rafitra. Jereo ny [README.md](../../../README.md) ho an'ny tabilao feno momba ny fiteny.

## Fepetra ilaina

- **Rust**: Kinova 1.85 na ambony kokoa (MSRV)
- **Cargo**: Miaraka amin'ny fametrahana Rust

## Fametrahana

### Avy amin'ny kaody loharano

```bash
# Maka dika mitovy amin'ny tahiry
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Manangana ny crates rehetra ao amin'ny toeram-piasana
cargo build --workspace --release

# Mametraka ny programa azo ampiasaina
cargo install --path awb-cli
```

## Fampandrosoana

### Fananganana

```bash
# Fananganana fitsapana (debug)
cargo build --workspace

# Fananganana famoahana (release)
cargo build --workspace --release
```

### Fitsapana

```bash
# Mandefa ny fitsapana rehetra
cargo test --workspace

# Mandefa ny fitsapana miaraka amin'ny fampisehoana ny vokatra rehetra
cargo test --workspace -- --nocapture
```

### Fanamarinana kaody (Lint)

```bash
# Manamarina ny olana ara-kalitao amin'ny kaody
cargo clippy --workspace --all-targets --all-features

# Mampihatra fanitsiana mandeha ho azy
cargo clippy --fix --allow-dirty --workspace
```

### Fandrafetana kaody

```bash
# Manamarina ny endriky ny kaody
cargo fmt --all -- --check

# Mampihatra ny fandrafetana
cargo fmt --all
```

## Ohatra fampiasana

### Fanamarinana maha-izy azy

```bash
# Miditra amin'ny wiki (mitahiry ny tenimiafina am-pilaminana)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Miditra amin'ny alalan'ny OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Fitanisana pejy

```bash
# Mitanisa pejy avy amin'ny sokajy iray
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Mitanisa pejy avy amin'ny fikarohana
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Mitanisa pejy avy amin'ny rakitra iray
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Fampandehanana fitsipika fanovana

```bash
# Mampandeha fitsipika miaraka amin'ny mombamomba (fomba fanandramana)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Mampihatra ny fitsipika amin'ny pejy tena izy
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Fomba baoty mandeha ho azy

```bash
# Mandefa amin'ny fomba baoty tsy arahin-maso miaraka amin'ny fetra
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Manohy avy amin'ny fivoriana teo aloha
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Ohatra momba ny mombamomba ny fitsipika

Mamorona rakitra `my-rules.toml`:

```toml
[profile]
name = "Cleanup Bot"
description = "Manitsy ny olana mahazatra amin'ny fandrafetana"
edit_summary = "Fanadiovana mandeha ho azy (Bot)"

[[rules]]
name = "Manitsy elanelana roa"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "Manitsy rohy ivelany"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "Manampy sokajy tsy ampy"
plugin = "add_categories.lua"
```

## Rafitra plugin

awb-rs dia manohana karazana plugin roa:

### Plugin Lua

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### Plugin WebAssembly

```rust
// Manangona ho WASM amin'ny: cargo build --target wasm32-wasi
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Endri-javatra amin'ny plugin:**
- Fampandehanana mitoka-monina misy fetran'ny loharanon-karena (fitadidiana, fotoana CPU)
- Fidirana amin'ny ampahany azo antoka amin'ny API ihany (tsy misy fidirana amin'ny rafi-drakitra na tambajotra)
- Fanamarinana fifanarahana kinova
- Fanohanana famerenana mamaky eo no ho eo amin'ny fomba fampandrosoana

## Modely fiarovana

- **Fitahirizana ny mombamomba ny fidirana**: Mampiasa ny fitehirizana tenimiafin'ny rafitra fandidiana ho an'ny fiarovana
- **Fanokanan-toerana plugin**: Miasa ao anaty tontolo mitoka-monina ny Lua sy ny WASM
- **Fetran'ny loharanon-karena**: Fetran'ny fotoana CPU, fitadidiana, ary isan'ny baiko isaky ny plugin
- **Famerana ny fangatahana API**: Manaja ny lohateny `Retry-After` sy ny fotoana fiandrasana azo ovaina
- **Fanamarinana fanovana**: Fijerena mialoha ny fanovana alohan'ny fampiharana ary firaketana fanaraha-maso
- **Fanohanana OAuth**: Fomba fanamarinana maha-izy azy tiana kokoa ho an'ny baoty

## Fahazoan-dalana

Misy fahazoan-dalana eo ambanin'ny iray amin'ireto:

- Fahazoan-dalana Apache, Kinova 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) na http://www.apache.org/licenses/LICENSE-2.0)
- Fahazoan-dalana MIT ([LICENSE-MIT](../../../LICENSE-MIT) na http://opensource.org/licenses/MIT)

araka ny safidinao.

## Fandraisana anjara

Tongasoa ny fandraisana anjara rehetra! Azafady jereo ny [CONTRIBUTING.md](../../../CONTRIBUTING.md) ho an'ny torolalana.

## Toetran'ny tetikasa

Famerenana manoratra mavitrika an'ny AutoWikiBrowser amin'ny Rust izy ity. Toetra ankehitriny:

- [x] Mpanjifa API fototra
- [x] Motera fitsipika
- [x] Fifandraisana amin'ny baikon'ny terminal (CLI)
- [x] Rafitra plugin (Lua/WASM)
- [x] Fanokanan-toerana fiarovana (sandboxing)
- [ ] Fifandraisana terminal TUI (eo am-panatanterahana)
- [ ] Fifandraisana an-tsary GUI (voaomana)
- [ ] Fampisehoana fahasamihafana mandroso (voaomana)

## Rohy

- [Torolalana ara-teknika](https://docs.rs/awb-rs)
- [Fanaraha-maso ny olana](https://github.com/somethingwithproof/awb-rs/issues)
- [AWB tany am-boalohany](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
