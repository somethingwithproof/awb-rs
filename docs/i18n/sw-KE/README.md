# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Uandishi upya wa kisasa wenye utendakazi wa hali ya juu wa AutoWikiBrowser (AWB) katika Rust, ukitoa uwezo wa uhariri wa kiotomatiki wa Wikipedia kwa kuzingatia usalama, utendakazi, na uwezo wa kupanuka.

## Usanifu wa Mfumo

```
awb-rs/
├── awb-core/           # Mantiki kuu ya biashara na misingi ya mfumo
├── awb-api/            # Utekelezaji wa mteja wa MediaWiki API
├── awb-cli/            # Kiolesura cha mstari wa amri (CLI)
├── awb-rules/          # Injini ya sheria kwa uhariri wa kiotomatiki
├── awb-lua-plugin/     # Mazingira ya utekelezaji wa programu-jalizi za Lua
├── awb-wasm-plugin/    # Mazingira ya utekelezaji wa programu-jalizi za WebAssembly
├── awb-security/       # Usalama na kutengwa kwa utekelezaji (sandboxing)
├── awb-db/             # Usimamizi wa uhifadhi wa hali ya kudumu
├── awb-ui/             # Vipengele vya kiolesura cha terminal (TUI)
├── awb-fixtures/       # Data ya majaribio na data ya kuiga
└── awb-integration/    # Mkusanyiko wa majaribio ya muunganisho
```

**Mtiririko wa Data:**
```
CLI → Msingi → API → MediaWiki
         ↓
     Injini ya Sheria → Programu-jalizi (Lua/WASM)
         ↓
     Safu ya Usalama → Utekelezaji Uliotengwa
         ↓
     Hifadhidata → Uhifadhi wa Hali
```

## Vipengele

- **Usanifu wa Kisasa**: Utekelezaji usiolingana (`async/await`) pamoja na mazingira ya Tokio kwa utendakazi wa juu
- **Usaidizi wa Wiki Nyingi**: Inafanya kazi na wiki yoyote inayoendeshwa na MediaWiki
- **Uhariri Unaotegemea Sheria**: Ufafanuzi wazi wa sheria katika muundo wa TOML
- **Mfumo wa Programu-jalizi**: Inaweza kupanuliwa kupitia hati za Lua na moduli za WebAssembly
- **Usalama Ulioimarishwa**: Utekelezaji uliotengwa wa programu-jalizi wenye mipaka ya rasilimali
- **Uendeshaji wa Vikundi**: Uchakataji bora wa kurasa nyingi kwa pamoja
- **TUI Shirikishi**: Kiolesura tajiri cha terminal kwa ufuatiliaji na udhibiti
- **Hali ya Majaribio (`dry-run`)**: Kagua mabadiliko kabla ya kuyatekeleza
- **Usimamizi wa Hali**: Endelea na vipindi vilivyokatizwa
- **Udhibiti wa Kasi ya Maombi**: Huheshimu mipaka ya API ya wiki na mbinu bora za matumizi
- **Lugha za Kimataifa**: Lugha 45 za maonyesho kupitia `--lang`, `AWB_LANG`, au mipangilio ya eneo ya POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`), ikijumuisha usaidizi wa mpangilio wa kulia-kwenda-kushoto (RTL) kwa Kiarabu, Kiajemi, Kiebrania, na Kiurdu

## Lugha Zinazotumika

CLI, ripoti za bot, na kiolesura cha GTK4 vinasaidia lugha 45 (ikijumuisha lugha 6 za RTL: `ar`, `ar-EG`, `ar-MA`, `fa`, `he`, na `ur`). Tumia `--lang <LANG>` katika CLI, au weka `AWB_LANG` au vigezo vya lugha vya POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`) kwa CLI na kiolesura cha GTK4 vyote viwili. Tazama [README.md](../../../README.md) kwa jedwali kamili la lugha.

## Mahitaji ya Awali

- **Rust**: Toleo la 1.94 au zaidi (MSRV)
- **Cargo**: Imejumuishwa na usakinishaji wa Rust

## Usakinishaji

### Kutoka kwa Msimbo Chanzo

```bash
# Nakili hazina ya msimbo
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Jenga crates zote za eneo la kazi
cargo build --workspace --release

# Sakinisha faili tekelezi
cargo install --path crates/awb_cli
```

## Uendelezaji

### Ujenzi

```bash
# Ujenzi wa utatuzi (debug)
cargo build --workspace

# Ujenzi wa toleo (release)
cargo build --workspace --release
```

### Majaribio

```bash
# Endesha majaribio yote
cargo test --workspace

# Endesha majaribio ukionyesha matokeo yote
cargo test --workspace -- --nocapture
```

### Ukaguzi wa Msimbo (Lint)

```bash
# Kagua matatizo ya ubora wa msimbo
cargo clippy --workspace --all-targets --all-features

# Tekeleza marekebisho ya kiotomatiki
cargo clippy --fix --allow-dirty --workspace
```

### Uumbizaji

```bash
# Kagua uumbizaji wa msimbo
cargo fmt --all -- --check

# Tekeleza uumbizaji
cargo fmt --all
```

## Mifano ya Matumizi

### Uthibitishaji

```bash
# Ingia kwenye wiki (huhifadhi vitambulisho kwa usalama)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Ingia kwa kutumia OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Kuorodhesha Kurasa

```bash
# Orodhesha kurasa kutoka kwa kategoria
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Orodhesha kurasa kutoka kwa utafutaji
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Orodhesha kurasa kutoka kwa faili
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Kuendesha Sheria za Uhariri

```bash
# Endesha sheria kwa kutumia wasifu (hali ya majaribio)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Tekeleza sheria kwenye kurasa halisi
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Hali ya Bot Otomatiki

```bash
# Endesha katika hali ya bot bila uangalizi pamoja na mipaka
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Endelea kutoka kwa kipindi kilichopita
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Mfano wa Wasifu wa Sheria

Unda faili `my-rules.toml`:

```toml
[profile]
name = "Cleanup Bot"
description = "Rekebisha matatizo ya kawaida ya uumbizaji"
edit_summary = "Usafishaji wa kiotomatiki (Bot)"

[[rules]]
name = "Rekebisha nafasi mbili"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "Rekebisha viungo vya nje"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "Ongeza kategoria zinazokosekana"
plugin = "add_categories.lua"
```

## Mfumo wa Programu-jalizi

awb-rs inasaidia aina mbili za programu-jalizi:

### Programu-jalizi za Lua

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### Programu-jalizi za WebAssembly

```rust
// Kusanya kuwa WASM kwa: cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Vipengele vya Programu-jalizi:**
- Utekelezaji uliotengwa wenye mipaka ya rasilimali (kumbukumbu, muda wa CPU)
- Ufikiaji wa sehemu salama ya API pekee (hakuna ufikiaji wa mfumo wa faili au mtandao)
- Ukaguzi wa utangamano wa matoleo
- Usaidizi wa kupakia upya papo hapo katika hali ya uendelezaji

## Muundo wa Usalama

- **Uhifadhi wa Vitambulisho**: Hutumia hifadhi ya funguo za mfumo wa uendeshaji kwa uhifadhi salama wa nywila
- **Kutengwa kwa Programu-jalizi**: Lua na WASM huendeshwa katika mazingira yaliyotengwa
- **Mipaka ya Rasilimali**: Mipaka ya muda wa CPU, kumbukumbu, na idadi ya maagizo kwa kila programu-jalizi
- **Udhibiti wa Kasi ya API**: Huheshimu vichwa vya `Retry-After` na ucheleweshaji unaoweza kusanidiwa
- **Uthibitishaji wa Uhariri**: Kagua mabadiliko kabla ya kutekeleza, pamoja na kumbukumbu za ukaguzi
- **Usaidizi wa OAuth**: Mbinu ya uthibitishaji inayopendekezwa kwa bot

## Leseni

Imepewa leseni chini ya mojawapo ya zifuatazo:

- Leseni ya Apache, Toleo la 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) au http://www.apache.org/licenses/LICENSE-2.0)
- Leseni ya MIT ([LICENSE-MIT](../../../LICENSE-MIT) au http://opensource.org/licenses/MIT)

kwa chaguo lako.

## Kuchangia

Michango inakaribishwa! Tafadhali tazama [CONTRIBUTING.md](../../../CONTRIBUTING.md) kwa miongozo.

## Hali ya Mradi

Huu ni uandishi upya unaoendelea wa AutoWikiBrowser katika Rust. Hali ya sasa:

- [x] Mteja mkuu wa API
- [x] Injini ya sheria
- [x] Kiolesura cha mstari wa amri (CLI)
- [x] Mfumo wa programu-jalizi (Lua/WASM)
- [x] Kutengwa kwa usalama (sandboxing)
- [ ] Kiolesura cha terminal TUI (inaendelea)
- [ ] Kiolesura cha picha GUI (imepangwa)
- [ ] Mwonekano wa hali ya juu wa tofauti za mabadiliko (imepangwa)

## Viungo

- [Nyaraka](https://docs.rs/awb-rs)
- [Ufuatiliaji wa Masuala](https://github.com/somethingwithproof/awb-rs/issues)
- [AWB ya Asili](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
