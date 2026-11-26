# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Ndegharị ọgbara ọhụrụ nwere oke ọsọ nke AutoWikiBrowser (AWB) n'asụsụ Rust, nke na-enye ike idezi Wikipedia n'akpaaka site n'ilekwasị anya na nchekwa, arụmọrụ dị elu, na ikike mgbakwunye.

## Nhazi Usoro

```
awb-rs/
├── awb-core/           # Isi usoro ọrụ na ntọala nkọwa
├── awb-api/            # Mmejuputa onye ahịa MediaWiki API
├── awb-cli/            # Ihu ahịrị iwu (CLI)
├── awb-rules/          # Igwe ụkpụrụ maka ndezi akpaaka
├── awb-lua-plugin/     # Gburugburu ọrụ maka mgbakwunye Lua
├── awb-wasm-plugin/    # Gburugburu ọrụ maka mgbakwunye WebAssembly
├── awb-security/       # Nchekwa na nkewapụ gburugburu ọrụ (sandboxing)
├── awb-db/             # Njikwa nchekwa ọnọdụ na-adịgide adịgide
├── awb-ui/             # Akụkụ ihu njikwa terminal (TUI)
├── awb-fixtures/       # Ngwa ule na data nṅomi
└── awb-integration/    # Usoro ule njikọta
```

**Usoro Nrugharị Data:**
```
CLI → Isi Usoro → API → MediaWiki
         ↓
     Igwe Ụkpụrụ → Mgbakwunye (Lua/WASM)
         ↓
     Ngalaba Nchekwa → Mmejuputa n'Ọnọdụ Kewapụrụ
         ↓
     Nchekwa Data → Nchekwa Ọnọdụ
```

## Atụmatụ

- **Nhazi Ọgbara Ọhụrụ**: Usoro `async/await` ya na igwe Tokio maka oke arụmọrụ
- **Nkwado Ọtụtụ Wiki**: Na-arụ ọrụ na wiki ọ bụla na-eji MediaWiki
- **Ndezi Dabere n'Ụkpụrụ**: Nkọwa ụkpụrụ n'ụdị TOML doro anya
- **Usoro Mgbakwunye**: Enwere ike ịgbakwunye ọrụ site na edemede Lua na ngwa WebAssembly
- **Nchekwa Sike**: Mmejuputa mgbakwunye n'ọnọdụ kewapụrụ nwere oke akụrụngwa
- **Ọrụ Ìgwè**: Nhazi ọtụtụ peeji n'otu oge n'ụzọ dị irè
- **Ihu TUI Na-emekọrịta Ihe**: Ihu terminal zuru oke maka nleba anya na njikwa
- **Ọnọdụ Nnwale (`dry-run`)**: Lelee mgbanwe tupu itinye ha n'ọrụ
- **Njikwa Ọnọdụ Ọrụ**: Maliteghachi oge ọrụ a kwụsịrị n'etiti
- **Oke Ọsọ Arịrịọ**: Na-asọpụrụ oke API nke wiki na ụkpụrụ kacha mma
- **Nkọwa Mba Ụwa**: Asụsụ ngosi 45 site na `--lang`, `AWB_LANG`, ma ọ bụ nhazi mpaghara POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`), gụnyere nkwado nhazi aka nri gaa aka ekpe (RTL) maka Arabic, Persian, Hebrew, na Urdu

## Asụsụ A Na-akwado

Ahịrị iwu (CLI), mkpesa bot, na ihu GTK4 na-akwado asụsụ 45 (gụnyere asụsụ RTL 6: `ar`, `ar-EG`, `ar-MA`, `fa`, `he`, na `ur`). Tinye `--lang <LANG>` ma ọ bụ tọọ `AWB_LANG` iji dochie nchọpụta asụsụ sistemụ. Lelee [README.md](../../../README.md) maka tebụl asụsụ zuru ezu.

## Ihe Achọrọ Tupu Mgbe

- **Rust**: Ụdị 1.85 ma ọ bụ karịa (MSRV)
- **Cargo**: A gụnyere ya na nrụnye Rust

## Nrụnye

### Site na Koodu Isi

```bash
# Detuo ebe nchekwa koodu
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Wuo crates niile dị n'ebe ọrụ
cargo build --workspace --release

# Wụnye ngwa a na-agba ọsọ
cargo install --path awb-cli
```

## Mmepe

### Owuwu

```bash
# Owuwu nchọpụta mmejọ (debug)
cargo build --workspace

# Owuwu mbipụta (release)
cargo build --workspace --release
```

### Ule

```bash
# Gbaa ule niile
cargo test --workspace

# Gbaa ule ma gosi nsonaazụ zuru ezu
cargo test --workspace -- --nocapture
```

### Nnyocha Koodu (Lint)

```bash
# Lelee nsogbu nnyocha koodu
cargo clippy --workspace --all-targets --all-features

# Tinye ndozi akpaaka
cargo clippy --fix --allow-dirty --workspace
```

### Nhazi Ọdịdị Koodu

```bash
# Lelee nhazi ọdịdị koodu
cargo fmt --all -- --check

# Tinye nhazi ọdịdị koodu
cargo fmt --all
```

## Ọmụmaatụ Ojiji

### Nkwenye Njirimara

```bash
# Banye na wiki (na-echekwa paswọọdụ n'ụzọ dị nchebe)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Banye site na OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Idepụta Peeji

```bash
# Depụta peeji sitere n'otu ngalaba
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Depụta peeji sitere na nchọta
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Depụta peeji sitere na faịlụ
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Ịgba Ụkpụrụ Ndezi

```bash
# Gbaa ụkpụrụ site na profaịlụ (ọnọdụ nnwale)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Tinye ụkpụrụ na peeji ndị dị adị
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Ọnọdụ Bot Akpaaka

```bash
# Gbaa n'ọnọdụ bot na-enweghị nleba anya nwere oke
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Maliteghachi site n'oge ọrụ gara aga
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Ọmụmaatụ Profaịlụ Ụkpụrụ

Mepụta faịlụ `my-rules.toml`:

```toml
[profile]
name = "Cleanup Bot"
description = "Dozie nsogbu nhazi ederede nkịtị"
edit_summary = "Nchacha akpaaka (Bot)"

[[rules]]
name = "Dozie oghere abụọ"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "Dozie njikọ mpụga"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "Tinye ngalaba ndị na-efu"
plugin = "add_categories.lua"
```

## Usoro Mgbakwunye

awb-rs na-akwado ụdị mgbakwunye abụọ:

### Mgbakwunye Lua

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### Mgbakwunye WebAssembly

```rust
// Chịkọta gaa WASM site na: cargo build --target wasm32-wasi
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Atụmatụ Mgbakwunye:**
- Mmejuputa kewapụrụ nwere oke akụrụngwa (ebe nchekwa, oge CPU)
- Nweta naanị akụkụ API dị nchebe (enweghị ohere sistemụ faịlụ ma ọ bụ netwọkụ)
- Nyocha ndakọrịta ụdị ngwa
- Nkwado mbugharị ọkụ n'ọnọdụ mmepe

## Ụdị Nchekwa

- **Nchekwa Njirimara**: Na-eji ebe nchekwa paswọọdụ nke sistemụ arụmọrụ maka nchekwa siri ike
- **Nkewapụ Mgbakwunye**: Lua na WASM na-arụ ọrụ na gburugburu kewapụrụ iche
- **Oke Akụrụngwa**: Oke oge CPU, ebe nchekwa, na ọnụọgụ ntụziaka maka mgbakwunye ọ bụla
- **Oke Ọsọ API**: Na-asọpụrụ isiokwu `Retry-After` na nkwụsịtụ a pụrụ ịhazi
- **Nkwado Ndezi**: Nlele mgbanwe tupu itinye ha, yana ndekọ nnyocha
- **Nkwado OAuth**: Ụzọ nkwenye njirimara kacha mma maka bot

## Ikikere

E nyere ikikere n'okpuru nke ọ bụla n'ime ndị a:

- Ikikere Apache, Ụdị 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) ma ọ bụ http://www.apache.org/licenses/LICENSE-2.0)
- Ikikere MIT ([LICENSE-MIT](../../../LICENSE-MIT) ma ọ bụ http://opensource.org/licenses/MIT)

dịka nhọrọ gị siri dị.

## Ntinye Aka

A na-anabata ntinye aka! Biko lelee [CONTRIBUTING.md](../../../CONTRIBUTING.md) maka ntụziaka.

## Ọnọdụ Ọrụ

Nke a bụ ndegharị na-aga n'ihu nke AutoWikiBrowser n'asụsụ Rust. Ọnọdụ dị ugbu a:

- [x] Isi onye ahịa API
- [x] Igwe ụkpụrụ
- [x] Ihu ahịrị iwu (CLI)
- [x] Usoro mgbakwunye (Lua/WASM)
- [x] Nkewapụ nchekwa (sandboxing)
- [ ] Ihu terminal TUI (na-aga n'ihu)
- [ ] Ihu eserese GUI (akwadoro n'atụmatụ)
- [ ] Ngosi ọdịiche dị elu (akwadoro n'atụmatụ)

## Njikọ

- [Akwụkwọ Nkọwa](https://docs.rs/awb-rs)
- [Ngalaba Nsochi Nsogbu](https://github.com/somethingwithproof/awb-rs/issues)
- [AWB Mbụ](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
