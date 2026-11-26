# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Isang moderno at mataas ang pagganap na muling pagsulat ng AutoWikiBrowser (AWB) sa Rust, na nagbibigay ng kakayahan sa awtomatikong pag-edit sa Wikipedia na nakatuon sa seguridad, bilis, at pagpapalawak.

## Arkitektura

```
awb-rs/
├── awb-core/           # Pangunahing lohika ng sistema at mga abstraksyon
├── awb-api/            # Implementasyon ng kliyente ng MediaWiki API
├── awb-cli/            # Command-line interface
├── awb-rules/          # Makina ng mga panuntunan para sa awtomatikong pag-edit
├── awb-lua-plugin/     # Runtime para sa mga Lua plugin
├── awb-wasm-plugin/    # Runtime para sa mga WebAssembly plugin
├── awb-security/       # Seguridad at pagbubukod sa sandbox
├── awb-db/             # Pamamahala ng permanenteng estado
├── awb-ui/             # Mga bahagi ng terminal UI
├── awb-fixtures/       # Mga fixture sa pagsusuri at halimbawang datos
└── awb-integration/    # Hanay ng mga integration test
```

**Daloy ng Datos:**
```
CLI → Pangunahing Layer → API → MediaWiki
            ↓
      Makina ng Panuntunan → Mga Plugin (Lua/WASM)
            ↓
      Layer ng Seguridad → Pagpapatakbo sa Sandbox
            ↓
      Database → Permanenteng Pag-imbak ng Estado
```

## Mga Tampok

- **Modernong Arkitektura**: Async/await gamit ang Tokio runtime para sa mataas na pagganap
- **Suporta sa Maraming Wiki**: Gumagana sa anumang wiki na pinapagana ng MediaWiki
- **Pag-edit Batay sa Panuntunan**: Deklaratibong pagtukoy ng mga panuntunan gamit ang TOML
- **Sistema ng Plugin**: Maaaring palawakin sa pamamagitan ng mga Lua script at WebAssembly module
- **Pinatibay na Seguridad**: Pagpapatakbo ng mga plugin sa sandbox na may mga limitasyon sa yaman
- **Mga Operasyong Maramihan**: Mahusay na pagproseso ng maraming pahina nang sabay-sabay
- **Interaktibong TUI**: Mayamang terminal interface para sa pagsubaybay at pagkontrol
- **Dry-Run Mode**: Suriin muna ang mga pagbabago bago ilapat ang mga ito
- **Pamamahala ng Estado**: Ipagpatuloy ang mga naantalang sesyon
- **Paglilimita ng Rate**: Sumusunod sa mga limitasyon ng wiki API at mga pinakamahusay na kasanayan
- **Internasyonalisasyon**: 45 wika ng pagpapakita sa pamamagitan ng `--lang`, `AWB_LANG`, o POSIX locale (`LC_ALL`/`LC_MESSAGES`/`LANG`), kabilang ang suporta sa RTL na layout para sa Arabic, Persian, Hebrew, at Urdu

## Mga Sinusuportahang Wika

Sinusuportahan ng CLI, mga ulat ng bot, at GTK4 UI ang 45 wika (kabilang ang 6 na RTL locale: `ar`, `ar-EG`, `ar-MA`, `fa`, `he`, at `ur`). Gamitin ang `--lang <LANG>` sa CLI, o itakda ang `AWB_LANG` o mga POSIX locale variable (`LC_ALL`/`LC_MESSAGES`/`LANG`) para sa parehong CLI at GTK4 UI. Tingnan ang [README.md](../../../README.md) para sa kumpletong talahanayan ng mga wika.

## Mga Kinakailangan

- **Rust**: Bersyon 1.94 o mas bago (MSRV)
- **Cargo**: Kasama sa pag-install ng Rust

## Pag-install

### Mula sa Source Code

```bash
# I-clone ang repositoryo
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# I-build ang lahat ng mga crate sa workspace
cargo build --workspace --release

# I-install ang binary
cargo install --path crates/awb_cli
```

## Pag-develop

### Pag-build

```bash
# Debug build
cargo build --workspace

# Release build
cargo build --workspace --release
```

### Pagsusuri

```bash
# Patakbuhin ang lahat ng mga pagsusuri
cargo test --workspace

# Patakbuhin ang mga pagsusuri na may detalyadong output
cargo test --workspace -- --nocapture
```

### Pagsusuri ng Estilo (Lint)

```bash
# Suriin kung may mga isyu sa linting
cargo clippy --workspace --all-targets --all-features

# Ilapat ang mga awtomatikong pag-aayos
cargo clippy --fix --allow-dirty --workspace
```

### Pag-format

```bash
# Suriin ang pag-format ng code
cargo fmt --all -- --check

# Ilapat ang pag-format ng code
cargo fmt --all
```

## Mga Halimbawa ng Paggamit

### Pagpapatunay

```bash
# Mag-login sa isang wiki (ligtas na iniimbak ang mga kredensyal)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Mag-login gamit ang OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Paglilista ng mga Pahina

```bash
# Maglista ng mga pahina mula sa isang kategorya
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Maglista ng mga pahina mula sa isang paghahanap
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Maglista ng mga pahina mula sa isang file
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Pagpapatakbo ng mga Panuntunan sa Pag-edit

```bash
# Patakbuhin ang mga panuntunan gamit ang isang profile (dry-run mode)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Ilapat ang mga panuntunan sa mga totoong pahina
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Awtomatikong Bot Mode

```bash
# Patakbuhin sa awtomatikong bot mode na may mga limitasyon
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Ipagpatuloy mula sa nakaraang sesyon
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Halimbawa ng Profile ng mga Panuntunan

Gumawa ng file na `my-rules.toml`:

```toml
[profile]
name = "Bot sa Paglilinis"
description = "Ayusin ang mga karaniwang isyu sa pag-format"
edit_summary = "Awtomatikong paglilinis (Bot)"

[[rules]]
name = "Ayusin ang dobleng espasyo"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "Ayusin ang mga panlabas na link"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "Magdagdag ng mga nawawalang kategorya"
plugin = "add_categories.lua"
```

## Sistema ng Plugin

Sinusuportahan ng awb-rs ang dalawang uri ng plugin:

### Mga Lua Plugin

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### Mga WebAssembly Plugin

```rust
// I-compile sa WASM gamit ang: cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Mga Tampok ng Plugin:**
- Pagpapatakbo sa sandbox na may mga limitasyon sa yaman (memorya, oras ng CPU)
- Access sa ligtas na bahagi ng mga API (walang access sa filesystem o network)
- Pagsusuri ng pagiging tugma ng bersyon
- Suporta sa hot-reload sa development mode

## Modelo ng Seguridad

- **Pag-imbak ng Kredensyal**: Gumagamit ng keyring ng OS para sa ligtas na pag-imbak ng password
- **Pagbubukod ng Plugin sa Sandbox**: Pinapatakbo ang Lua at WASM sa nakahiwalay na kapaligiran
- **Mga Limitasyon sa Yaman**: Mga limitasyon sa oras ng CPU, memorya, at bilang ng instruksyon sa bawat plugin
- **Paglilimita ng Rate ng API**: Sumusunod sa mga header na `Retry-After` at mga naisasaayos na pagkaantala
- **Pagpapatunay ng Pag-edit**: Preview ng mga pagbabago bago ilapat at mga audit log
- **Suporta sa OAuth**: Pangunahing inirerekomendang paraan ng pagpapatunay para sa mga bot

## Lisensya

Lisensyado sa ilalim ng alinman sa:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) o http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../../../LICENSE-MIT) o http://opensource.org/licenses/MIT)

ayon sa iyong pagpipilian.

## Pag-aambag

Malugod na tinatanggap ang mga kontribusyon! Pakitingnan ang [CONTRIBUTING.md](../../../CONTRIBUTING.md) para sa mga alituntunin.

## Katayuan ng Proyekto

Ito ay isang aktibong muling pagsulat ng AutoWikiBrowser sa Rust. Kasalukuyang katayuan:

- [x] Pangunahing API client
- [x] Makina ng panuntunan
- [x] CLI interface
- [x] Sistema ng plugin (Lua/WASM)
- [x] Seguridad na sandbox
- [ ] Terminal UI (kasalukuyang ginagawa)
- [ ] GUI (pinaplano)
- [ ] Mas pinahusay na pagpapakita ng diff (pinaplano)

## Mga Link

- [Dokumentasyon](https://docs.rs/awb-rs)
- [Tagasubaybay ng Isyu](https://github.com/somethingwithproof/awb-rs/issues)
- [Orihinal na AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
