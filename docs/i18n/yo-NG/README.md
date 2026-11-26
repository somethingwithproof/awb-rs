# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Àtúnkọ ìgbàlódé tí ó ní iṣẹ́ gíga fún AutoWikiBrowser (AWB) ní èdè Rust, tí ó ń pèsè agbára àtúnṣe Wikipedia aládàáṣe pẹ̀lú ìtẹnumọ́ lórí ààbò, ìyára iṣẹ́, àti agbára ìfàgùn.

## Ìtòlẹ́sẹẹsẹ Ètò

```
awb-rs/
├── awb-core/           # Ìlànà iṣẹ́ pàtàkì àti àwọn ìpìlẹ̀ ètò
├── awb-api/            # Ìmúṣẹ olùbáraẹnisọ̀rọ̀ MediaWiki API
├── awb-cli/            # Ojú ìlà àṣẹ (CLI)
├── awb-rules/          # Ẹ̀rọ òfin fún àtúnṣe aládàáṣe
├── awb-lua-plugin/     # Àyíká ìṣiṣẹ́ àwọn àfikún Lua
├── awb-wasm-plugin/    # Àyíká ìṣiṣẹ́ àwọn àfikún WebAssembly
├── awb-security/       # Ààbò àti ìyàsọ́tọ̀ àyíká ìṣiṣẹ́ (sandboxing)
├── awb-db/             # Ìṣàkóso ìpamọ́ ipò títí láé
├── awb-ui/             # Àwọn ẹ̀yà ojú-ìṣiṣẹ́ terminal (TUI)
├── awb-fixtures/       # Àwọn dátà ìdánwò àti dátà àfarawé
└── awb-integration/    # Àkójọ àwọn ìdánwò ìṣọ̀kan
```

**Ìṣàn Dátà:**
```
CLI → Àárín Ètò → API → MediaWiki
         ↓
     Ẹ̀rọ Òfin → Àwọn Àfikún (Lua/WASM)
         ↓
     Ìpele Ààbò → Ìṣiṣẹ́ ní Àyíká Aláàbò
         ↓
     Ìpamọ́ Dátà → Ìtọ́jú Ipò Iṣẹ́
```

## Àwọn Ẹ̀yà

- **Ìtòlẹ́sẹẹsẹ Ìgbàlódé**: Ìṣiṣẹ́ `async/await` pẹ̀lú àyíká Tokio fún ìyára iṣẹ́ gíga
- **Àtìlẹ́yìn fún Ọ̀pọ̀ Wiki**: Ó ń ṣiṣẹ́ pẹ̀lú èyíkéyìí wiki tí ó ń lo MediaWiki
- **Àtúnṣe Tó Dá Lórí Òfin**: Àpèjúwe òfin tí ó ṣe kedere ní ọ̀nà TOML
- **Ètò Àwọn Àfikún**: A lè fà á gùn nípa lílo àwọn kóòdù Lua àti àwọn ẹ̀yà WebAssembly
- **Ààbò Tó Lágbára**: Ìṣiṣẹ́ àfikún ní àyíká tí a yà sọ́tọ̀ pẹ̀lú òdiwọ̀n ohun-èlò
- **Iṣẹ́ Ọ̀pọ̀ Ojú-ewé**: Ṣíṣe àtúnṣe ọ̀pọ̀ ojú-ewé lẹ́ẹ̀kan náà láìfàṣẹ̀yìn
- **Ojú TUI Alábàáṣepọ̀**: Ojú-ìṣiṣẹ́ terminal tí ó kún fún ohun èlò fún àbójútó àti ìṣàkóso
- **Ipò Àyẹ̀wò (`dry-run`)**: Ṣe àyẹ̀wò àwọn àyípadà kí o tó fi wọ́n sílẹ̀
- **Ìṣàkóso Ipò Iṣẹ́**: Tẹ̀síwájú láti ibi tí ìṣiṣẹ́ tí a dá dúró dé
- **Òdiwọ̀n Ìyára Ìbéèrè**: Ó ń bọ̀wọ̀ fún òdiwọ̀n API wiki àti àwọn ìlànà tó dára jùlọ
- **Ìṣètò Èdè Àgbáyé**: Èdè ìfihàn 45 nípasẹ̀ `--lang`, `AWB_LANG`, tàbí ìṣètò agbègbè POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`), pẹ̀lú àtìlẹ́yìn fún ìkọ̀wé láti ọ̀tún sí òsì (RTL) fún Lárúbáwá, Páṣíà, Hébérù, àti Ùrdù

## Àwọn Èdè Tí A Ṣe Àtìlẹ́yìn Fún

Ojú ìlà àṣẹ (CLI), àwọn ìròyìn bọ́ọ̀tì, àti ojú-ìṣiṣẹ́ GTK4 ń ṣe àtìlẹ́yìn fún èdè 45 (pẹ̀lú èdè RTL 6: `ar`, `ar-EG`, `ar-MA`, `fa`, `he`, àti `ur`). Lo `--lang <LANG>` tàbí ṣètò `AWB_LANG` láti yí ìdámọ̀ èdè ètò ẹ̀rọ padà. Wo [README.md](../../../README.md) fún tábìlì èdè tí ó pé.

## Àwọn Ohun Àkọ́kọ́ Tí A Nílò

- **Rust**: Ẹ̀yà 1.85 tàbí jù bẹ́ẹ̀ lọ (MSRV)
- **Cargo**: Ó wà nínú ìfisílé Rust

## Ìfisílé

### Láti Inú Kóòdù Orísun

```bash
# Ṣe àdàkọ ibi ìpamọ́ kóòdù
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Kọ gbogbo àwọn crates inú àyíká iṣẹ́
cargo build --workspace --release

# Fi fáìlì ìṣiṣẹ́ sí orí ẹ̀rọ
cargo install --path awb-cli
```

## Ìdàgbàsókè Ètò

### Ìkọ́lé

```bash
# Ìkọ́lé àyẹ̀wò àṣìṣe (debug)
cargo build --workspace

# Ìkọ́lé ìtújáde (release)
cargo build --workspace --release
```

### Ìdánwò

```bash
# Ṣe gbogbo àwọn ìdánwò
cargo test --workspace

# Ṣe àwọn ìdánwò pẹ̀lú ìfihàn gbogbo èsì
cargo test --workspace -- --nocapture
```

### Àyẹ̀wò Kóòdù (Lint)

```bash
# Ṣe àyẹ̀wò àwọn ìṣòro dídára kóòdù
cargo clippy --workspace --all-targets --all-features

# Fi àwọn àtúnṣe aládàáṣe sílẹ̀
cargo clippy --fix --allow-dirty --workspace
```

### Ìṣètò Ìrísí Kóòdù

```bash
# Ṣe àyẹ̀wò ìrísí kóòdù
cargo fmt --all -- --check

# Fi ìṣètò ìrísí kóòdù sílẹ̀
cargo fmt --all
```

## Àwọn Àpẹẹrẹ Ìlò

### Ìjẹ́rìísí Ìdánimọ̀

```bash
# Wọlé sí wiki (ó ń tọ́jú àwọn ọ̀rọ̀ aṣínà ní ọ̀nà aláàbò)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Wọlé pẹ̀lú OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Ṣíṣe Àkójọ Àwọn Ojú-ewé

```bash
# Ṣe àkójọ àwọn ojú-ewé láti inú ẹ̀ka kan
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Ṣe àkójọ àwọn ojú-ewé láti inú ìwádìí
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Ṣe àkójọ àwọn ojú-ewé láti inú fáìlì kan
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Ṣíṣe Àwọn Òfin Àtúnṣe

```bash
# Ṣiṣẹ́ pẹ̀lú àwọn òfin láti inú profáìlì (ipò àyẹ̀wò)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Fi àwọn òfin sílẹ̀ lórí àwọn ojú-ewé gidi
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Ipò Bọ́ọ̀tì Aládàáṣe

```bash
# Ṣiṣẹ́ ní ipò bọ́ọ̀tì aládàáṣe pẹ̀lú àwọn òdiwọ̀n
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Tẹ̀síwájú láti inú ìṣiṣẹ́ tẹ́lẹ̀
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Àpẹẹrẹ Ìṣètò Òfin

Ṣẹ̀dá fáìlì `my-rules.toml`:

```toml
[profile]
name = "Cleanup Bot"
description = "Ṣàtúnṣe àwọn ìṣòro ìrísí ọ̀rọ̀ tí ó wọ́pọ̀"
edit_summary = "Ìmọ́tótó aládàáṣe (Bot)"

[[rules]]
name = "Ṣàtúnṣe àlàfo méjì"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "Ṣàtúnṣe àwọn ìjápọ̀ ìta"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "Fi àwọn ẹ̀ka tí ó ṣọ̀nù kún un"
plugin = "add_categories.lua"
```

## Ètò Àwọn Àfikún

awb-rs ń ṣe àtìlẹ́yìn fún irú àfikún méjì:

### Àwọn Àfikún Lua

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### Àwọn Àfikún WebAssembly

```rust
// Kọ ọ́ sí WASM pẹ̀lú: cargo build --target wasm32-wasi
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Àwọn Ẹ̀yà Àfikún:**
- Ìṣiṣẹ́ ní àyíká tí a yà sọ́tọ̀ pẹ̀lú òdiwọ̀n ohun-èlò (ìrántí, àkókò CPU)
- Àǹfààní sí apá ààbò ti API nìkan (kò sí àǹfààní sí ètò fáìlì tàbí nẹ́tíwọ́ọ̀kì)
- Àyẹ̀wò ìbáramu ẹ̀yà ètò
- Àtìlẹ́yìn fún ìtúnkó lẹ́sẹ̀kẹsẹ̀ ní ipò ìdàgbàsókè

## Àpẹẹrẹ Ààbò

- **Ìpamọ́ Ìdánimọ̀**: Ó ń lo ibi ìpamọ́ kọ́kọ́rọ́ ti ètò ìṣiṣẹ́ ẹ̀rọ fún ìtọ́jú ọ̀rọ̀ aṣínà aláàbò
- **Ìyàsọ́tọ̀ Àwọn Àfikún**: Lua àti WASM ń ṣiṣẹ́ ní àyíká tí a yà sọ́tọ̀
- **Òdiwọ̀n Ohun-èlò**: Òdiwọ̀n àkókò CPU, ìrántí, àti iye àṣẹ fún àfikún kọ̀ọ̀kan
- **Òdiwọ̀n Ìyára API**: Ó ń bọ̀wọ̀ fún àwọn àkọlé `Retry-After` àti àkókò ìdúró tí a lè ṣètò
- **Ìjẹ́rìísí Àtúnṣe**: Ṣe àyẹ̀wò àwọn àyípadà kí o tó fi wọ́n sílẹ̀, pẹ̀lú àwọn àkọsílẹ̀ àyẹ̀wò
- **Àtìlẹ́yìn OAuth**: Ọ̀nà ìjẹ́rìísí ìdánimọ̀ tí a yàn jùlọ fún àwọn bọ́ọ̀tì

## Ìwé Àṣẹ

A fún ní ìwé àṣẹ lábẹ́ èyíkéyìí nínú àwọn wọ̀nyí:

- Ìwé Àṣẹ Apache, Ẹ̀yà 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) tàbí http://www.apache.org/licenses/LICENSE-2.0)
- Ìwé Àṣẹ MIT ([LICENSE-MIT](../../../LICENSE-MIT) tàbí http://opensource.org/licenses/MIT)

gẹ́gẹ́ bí àṣàyàn rẹ.

## Ìkópa

A gbà gbogbo ìkópa tọwọ́tẹsẹ̀! Jọ̀wọ́ wo [CONTRIBUTING.md](../../../CONTRIBUTING.md) fún àwọn ìlànà.

## Ipò Iṣẹ́ Àkànṣe

Èyí jẹ́ àtúnkọ AutoWikiBrowser tí ó ń lọ lọ́wọ́ ní èdè Rust. Ipò lọ́wọ́lọ́wọ́:

- [x] Olùbáraẹnisọ̀rọ̀ API pàtàkì
- [x] Ẹ̀rọ òfin
- [x] Ojú ìlà àṣẹ (CLI)
- [x] Ètò àwọn àfikún (Lua/WASM)
- [x] Ìyàsọ́tọ̀ ààbò (sandboxing)
- [ ] Ojú-ìṣiṣẹ́ terminal TUI (ń lọ lọ́wọ́)
- [ ] Ojú-ìṣiṣẹ́ àwòrán GUI (wà lẹ́rò)
- [ ] Ìfihàn ìyàtọ̀ àtúnṣe tí ó ga (wà lẹ́rò)

## Àwọn Ìjápọ̀

- [Àwọn Ìwé Ìtọ́sọ́nà](https://docs.rs/awb-rs)
- [Olùtọ́pa Ìṣòro](https://github.com/somethingwithproof/awb-rs/issues)
- [AWB Àtilẹ̀wá](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
