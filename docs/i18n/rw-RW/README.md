# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Ivugurura rigezweho kandi ryihuta rya AutoWikiBrowser (AWB) ryanditswe muri Rust, ritanga ubushobozi bwo guhindura Wikipedia mu buryo bwikora rishingiye ku mutekano, imikorere myiza, n'ubushobozi bwo kwaguka.

## Imyubakire y'Uburyo

```
awb-rs/
├── awb-core/           # Amategeko shingiro y'imikorere n'ibisobanuro by'ibanze
├── awb-api/            # Ishyirwa mu bikorwa ry'umukiriya wa MediaWiki API
├── awb-cli/            # Imigaragarire y'umurongo w'amabwiriza (CLI)
├── awb-rules/          # Moteri y'amategeko yo guhindura mu buryo bwikora
├── awb-lua-plugin/     # Uburyo bwo gukoresha imigereka ya Lua
├── awb-wasm-plugin/    # Uburyo bwo gukoresha imigereka ya WebAssembly
├── awb-security/       # Umutekano n'uburyo bwo gutandukanya imikorere (sandboxing)
├── awb-db/             # Imicungire y'ibikwa rihoraho ry'imiterere
├── awb-ui/             # Ibice bigize imigaragarire ya terminal (TUI)
├── awb-fixtures/       # Amakuru y'isuzuma n'ibyigano by'amakuru
└── awb-integration/    # Urutonde rw'amasuzuma yo guhuza ibice
```

**Uko Amakuru Agenda:**
```
CLI → Umutima → API → MediaWiki
         ↓
     Moteri y'Amategeko → Imigereka (Lua/WASM)
         ↓
     Urwego rw'Umutekano → Imikorere Yigenga Irinzwe
         ↓
     Ububiko bw'Amakuru → Ibikwa ry'Imiterere
```

## Ubushobozi

- **Imyubakire Igezweho**: Imikorere idategerezanya (`async/await`) ikoresha Tokio kugira ngo itange umuvuduko wo hejuru
- **Gushyigikira Wiki Nyinshi**: Ikorana na wiki iyo ari yo yose ikoresha MediaWiki
- **Guhindura Gushingiye ku Mategeko**: Ibisobanuro by'amategeko bigaragara neza mu miterere ya TOML
- **Uburyo bw'Imigereka**: Ishobora kwagurwa hifashishijwe porogaramu nto za Lua n'ibice bya WebAssembly
- **Umutekano Ukomeye**: Imikorere y'imigereka mu mwanya urinzwe ifite imipaka y'ubushobozi bukoreshwa
- **Ibikorwa bya Rimwe mu Mwinshi**: Gutunganya impapuro nyinshi mu buryo bwihuse kandi bunoze
- **Imigaragarire ya TUI Ikoreshwa**: Imigaragarire ikungahaye ya terminal yo gukurikirana no kugenzura
- **Uburyo bw'Igerageza (`dry-run`)**: Kureba impinduka mbere yo kuzishyira mu bikorwa
- **Imicungire y'Imiterere**: Gukomereza aho igikorwa cyahagaze
- **Kugabanya Umuvuduko w'Ibisabwa**: Yubahiriza imipaka ya API ya wiki n'amahame meza y'imikoreshereze
- **Gukoresha Indimi Mpuzamahanga**: Indimi 45 z'imigaragarire ukoresheje `--lang`, `AWB_LANG`, cyangwa igenamiterere rya POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`), harimo n'uburyo bw'imyandikire iva iburyo ijya ibumoso (RTL) ku Cyarabu, Igiperesi, Igiheburayo, n'Ikiurdu

## Indimi Zishyigikiwe

Umurongo w'amabwiriza (CLI), raporo za robo, n'imigaragarire ya GTK4 bishyigikira indimi 45 (harimo indimi 6 za RTL: `ar`, `ar-EG`, `ar-MA`, `fa`, `he`, na `ur`). Koresha `--lang <LANG>` muri CLI, cyangwa ushyireho `AWB_LANG` cyangwa impinduka z'ururimi za POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`) kuri CLI no ku migaragarire ya GTK4. Reba [README.md](../../../README.md) kugira ngo ubone imbonerahamwe yuzuye y'indimi.

## Ibisabwa Mbere

- **Rust**: Verisiyo 1.94 cyangwa hejuru yayo (MSRV)
- **Cargo**: Ijyana n'iyinjiza rya Rust

## Iyinjiza

### Uhereye ku Nkomoko ya Kode

```bash
# Gukurura ububiko bwa kode
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Kubaka ibice byose byo mu mwanya w'akazi
cargo build --workspace --release

# Gushyiramo porogaramu ikoreshwa
cargo install --path crates/awb_cli
```

## Iterambere rya Porogaramu

### Kubaka

```bash
# Kubaka mu buryo bwo gukosora (debug)
cargo build --workspace

# Kubaka mu buryo bwo gusohora (release)
cargo build --workspace --release
```

### Isuzuma

```bash
# Gukora amasuzuma yose
cargo test --workspace

# Gukora amasuzuma werekana ibisohoka byose
cargo test --workspace -- --nocapture
```

### Isuzuma ry'Ubuziranenge bwa Kode (Lint)

```bash
# Kugenzura ibibazo by'imikoreshereze ya kode
cargo clippy --workspace --all-targets --all-features

# Gushyiraho ibikosorwa mu buryo bwikora
cargo clippy --fix --allow-dirty --workspace
```

### Imiterere ya Kode

```bash
# Kugenzura imiterere y'imyandikire ya kode
cargo fmt --all -- --check

# Gushyiraho imiterere y'imyandikire ya kode
cargo fmt --all
```

## Ingero z'Imikoreshereze

### Kwemeza Umwirondoro

```bash
# Kwinjira kuri wiki (ibika amakuru y'ibanga mu mutekano)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Kwinjira ukoresheje OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Gukora Urutonde rw'Impapuro

```bash
# Gukora urutonde rw'impapuro ziri mu cyiciro
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Gukora urutonde rw'impapuro zivuye mu ishakisha
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Gukora urutonde rw'impapuro zivuye muri dosiye
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Gukoresha Amategeko yo Guhindura

```bash
# Gukoresha amategeko n'umwirondoro (uburyo bw'igerageza)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Gushyira amategeko mu bikorwa ku mpapuro za nyazo
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Uburyo bwa Robo Yikoresha

```bash
# Gukora mu buryo bwa robo idakurikiranwa ufite imipaka
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Gukomereza ku gikorwa cyabanje
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Urugero rw'Umwirondoro w'Amategeko

Kora dosiye yitwa `my-rules.toml`:

```toml
[profile]
name = "Cleanup Bot"
description = "Gukosora ibibazo bisanzwe by'imiterere y'inyandiko"
edit_summary = "Isukura rikozwe mu buryo bwikora (Bot)"

[[rules]]
name = "Gukosora imyanya ibiri ikurikiranye"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "Gukosora imiyoboro yo hanze"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "Kongeramo ibyiciro bibura"
plugin = "add_categories.lua"
```

## Uburyo bw'Imigereka

awb-rs ishyigikira ubwoko bubiri bw'imigereka:

### Imigereka ya Lua

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### Imigereka ya WebAssembly

```rust
// Hindura muri WASM ukoresheje: cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Ibiranga Imigereka:**
- Imikorere itandukanyijwe ifite imipaka y'ubushobozi (ububiko bw'igihe gito, igihe cya CPU)
- Uburenganzira ku gice gitekanye cya API gusa (nta burenganzira kuri dosiye za sisitemu cyangwa urusobe rw'itumanaho)
- Igenzura ry'imikoranire hagati ya verisiyo
- Ubushobozi bwo kongera gufungura ako kanya mu gihe cyo kubaka porogaramu

## Uburyo bw'Umutekano

- **Ububiko bw'Amakuru y'Ibanga**: Ikoresha ububiko bw'amagambo y'ibanga bwa sisitemu y'imikorere kugira ngo bibikwe neza
- **Gutandukanya Imigereka**: Lua na WASM bikorera mu bidukikije byigenga birinzwe
- **Imipaka y'Ubushobozi**: Imipaka y'igihe cya CPU, ububiko bw'igihe gito, n'umubare w'amabwiriza kuri buri mugereka
- **Kugabanya Umuvuduko wa API**: Yubahiriza imitwe ya `Retry-After` n'igihe cyo gutegereza gishobora guhindurwa
- **Kwemeza Impinduka**: Kureba impinduka mbere yo kuzishyira mu bikorwa, hamwe n'inyandiko z'igenzura
- **Gushyigikira OAuth**: Uburyo bwifuzwa bwo kwemeza umwirondoro wa robo

## Uruhushya

Itangwa munsi y'uruhushya rumwe muri izi zikurikira:

- Uruhushya rwa Apache, Verisiyo 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) cyangwa http://www.apache.org/licenses/LICENSE-2.0)
- Uruhushya rwa MIT ([LICENSE-MIT](../../../LICENSE-MIT) cyangwa http://opensource.org/licenses/MIT)

ukurikije amahitamo yawe.

## Gutanga Umusanzu

Imisanzu yose irakiriwe! Nyamuneka reba [CONTRIBUTING.md](../../../CONTRIBUTING.md) kugira ngo ubone amabwiriza.

## Aho Umushinga Ugeze

Iri ni ivugurura rya AutoWikiBrowser ririmo gukorwa muri Rust. Aho bigeze ubu:

- [x] Umukiriya shingiro wa API
- [x] Moteri y'amategeko
- [x] Imigaragarire y'umurongo w'amabwiriza (CLI)
- [x] Uburyo bw'imigereka (Lua/WASM)
- [x] Gutandukanya umutekano (sandboxing)
- [ ] Imigaragarire ya terminal TUI (iracyakorwa)
- [ ] Imigaragarire y'amashusho GUI (iteganyijwe)
- [ ] Imigaragarire ihanitse y'itandukaniro ry'impinduka (iteganyijwe)

## Imiyoboro

- [Inyandiko z'Amabwiriza](https://docs.rs/awb-rs)
- [Urutonde rw'Ibibazo](https://github.com/somethingwithproof/awb-rs/issues)
- [AWB y'Umwimerere](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
