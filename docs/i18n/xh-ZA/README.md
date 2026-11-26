# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Ubhalo olutsha lwale mihla nolunamandla aphezulu lwe-AutoWikiBrowser (AWB) nge-Rust, olubonelela ngezakhono zokuhlela iWikipedia ngokuzenzekelayo kugxilwe kukhuseleko, ukusebenza kakuhle, kunye nokwandiseka.

## Uyilo lweNkqubo

```
awb-rs/
├── awb-core/           # Ingqiqo engundoqo yomsebenzi kunye nezakhelo ezingundoqo
├── awb-api/            # Ukuphunyezwa komxhasi we-MediaWiki API
├── awb-cli/            # Ujongano lomgca womyalelo (CLI)
├── awb-rules/          # Injini yemithetho yokuhlela ngokuzenzekelayo
├── awb-lua-plugin/     # Indawo yokusebenza yeeplagini ze-Lua
├── awb-wasm-plugin/    # Indawo yokusebenza yeeplagini ze-WebAssembly
├── awb-security/       # Ukhuseleko nokwahlulwa kwendawo yokusebenza (sandboxing)
├── awb-db/             # Ulawulo lokugcinwa kwemeko engapheliyo
├── awb-ui/             # Amalungu ojongano lwetheminali (TUI)
├── awb-fixtures/       # Idatha yovavanyo nedatha yokulingisa
└── awb-integration/    # Uluhlu lweemvavanyo zokudibanisa
```

**Ukuhamba kweDatha:**
```
CLI → Undoqo → API → MediaWiki
         ↓
     Injini yeMithetho → Iiplagini (Lua/WASM)
         ↓
     Umaleko woKhuseleko → Ukuqhuba kwiNdawo eKhethiweyo
         ↓
     Isiseko seDatha → Ukugcinwa kweMeko
```

## Iimpawu

- **Uyilo lwale Mihla**: Ukusebenza okungahambelaniyo (`async/await`) kunye nenkqubo ye-Tokio ukufumana ukusebenza okuphezulu
- **Inkxaso yeeWiki eziNinzi**: Isebenza nayiphi na iwiki eqhutywa yi-MediaWiki
- **Ukuhlela okuSekelwe kwiMithetho**: Iinkcazo zemithetho ezicacileyo kwifomathi ye-TOML
- **INkqubo yeePlagini**: Inokwandiswa ngezikripthi ze-Lua kunye neemodyuli ze-WebAssembly
- **Ukhuseleko oluQinisekisiweyo**: Ukuqhuba kweeplagini kwindawo ekhuselekileyo eyahluliweyo enemida yezibonelelo
- **Imisebenzi yamaQela**: Ukucubungula amaphepha amaninzi ngokufanelekileyo ngexesha elinye
- **I-TUI eSebenzisanayo**: Ujongano lwetheminali olutyebileyo lokubeka iliso nolawulo
- **Imo yoVavanyo (`dry-run`)**: Jonga utshintsho phambi kokuba ulusebenzise
- **Ulawulo lweMeko**: Qhubeka neeseshoni eziphazamisekileyo
- **Ukuthintela uMlinganiselo weZicelo**: Ihlonipha imida ye-API yewiki kunye neendlela ezilungileyo zokusebenza
- **Ukusekwa kweelwimi zaMazwe ngamazwe**: Iilwimi ezingama-45 zokubonisa nge-`--lang`, `AWB_LANG`, okanye iisetingi zendawo ze-POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`), kuquka inkxaso yendlela yokubhala ukusuka ekunene ukuya ekhohlo (RTL) yesiArabhu, isiPersi, isiHebhere, nesiUrdu

## Iilwimi eziXhaswayo

I-CLI, iingxelo zeebot, kunye nojongano lwe-GTK4 zixhasa iilwimi ezingama-45 (kuquka iilwimi ezi-6 ze-RTL: `ar`, `ar-EG`, `ar-MA`, `fa`, `he`, kunye ne-`ur`). Sebenzisa `--lang <LANG>` okanye usete i-`AWB_LANG` ukutshintsha ukubonwa kolwimi lwenkqubo. Jonga ku-[README.md](../../../README.md) ngoluhlu olupheleleyo lweelwimi.

## Iimfuno zokuQala

- **Rust**: Inguqulelo 1.85 okanye ngaphezulu (MSRV)
- **Cargo**: Iqukiwe kufakelo lwe-Rust

## Ufakelo

### Ukusuka kwiKhowudi yoMthombo

```bash
# Khuphela uvimba wekhowudi
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Yakha zonke ii-crates zendawo yokusebenza
cargo build --workspace --release

# Faka inkqubo eqhutywayo
cargo install --path awb-cli
```

## Uphuhliso

### Ulwakhiwo

```bash
# Ulwakhiwo lokulungisa iziphene (debug)
cargo build --workspace

# Ulwakhiwo lokukhutshwa (release)
cargo build --workspace --release
```

### Uvavanyo

```bash
# Qhuba zonke iimvavanyo
cargo test --workspace

# Qhuba iimvavanyo ubonisa iziphumo ezipheleleyo
cargo test --workspace -- --nocapture
```

### Uhlolo lweKhowudi (Lint)

```bash
# Khangela iingxaki zomgangatho wekhowudi
cargo clippy --workspace --all-targets --all-features

# Sebenzisa izilungiso ezizenzekelayo
cargo clippy --fix --allow-dirty --workspace
```

### Ukufomatha

```bash
# Khangela ifomathi yekhowudi
cargo fmt --all -- --check

# Sebenzisa ifomathi yekhowudi
cargo fmt --all
```

## Imizekelo yokuSetyenziswa

### Uqinisekiso loChongeko

```bash
# Ngena kwiwiki (igcina iinkcukacha zokungena ngokukhuselekileyo)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Ngena nge-OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Ukudwelisa amaPhepha

```bash
# Dwelisa amaphepha asuka kuhlelo oluthile
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Dwelisa amaphepha asuka kukhangelo
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Dwelisa amaphepha asuka kwifayile
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Ukuqhuba iMithetho yokuHlela

```bash
# Qhuba imithetho ngeprofayile (imo yovavanyo)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Sebenzisa imithetho kumaphepha okwenene
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Imo yeBot eZenzekelayo

```bash
# Qhuba kwimo yebot engajongwanga enemida
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Qhubeka ukusuka kwiseshoni yangaphambili
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Umzekelo weProfayile yeMithetho

Yenza ifayile ethi `my-rules.toml`:

```toml
[profile]
name = "Cleanup Bot"
description = "Lungisa iingxaki eziqhelekileyo zokufomatha"
edit_summary = "Ucoco oluzenzekelayo (Bot)"

[[rules]]
name = "Lungisa izikhewu eziphindwe kabini"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "Lungisa amakhonkco angaphandle"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "Yongeza iindidi ezingekhoyo"
plugin = "add_categories.lua"
```

## INkqubo yeePlagini

i-awb-rs ixhasa iindidi ezimbini zeeplagini:

### Iiplagini ze-Lua

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### Iiplagini ze-WebAssembly

```rust
// Hlanganisa uye kwi-WASM nge: cargo build --target wasm32-wasi
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Iimpawu zeePlagini:**
- Ukuqhuba kwindawo eyahluliweyo enemida yezibonelelo (imemori, ixesha le-CPU)
- Ukufikelela kwinxalenye ekhuselekileyo ye-API kuphela (akukho fikelelo kwinkqubo yeefayile okanye kwinethiwekhi)
- Uhlolo lokuhambelana kweenguqulelo
- Inkxaso yokulayisha kwakhona ngoko nangoko kwimo yophuhliso

## Umzekelo woKhuseleko

- **Ukugcinwa kweeNkcukacha zokuNgena**: Isebenzisa isigcini samagama okugqitha senkqubo yokusebenza ukugcina ngokukhuselekileyo
- **Ukwahlulwa kweePlagini**: I-Lua ne-WASM zisebenza kwiindawo ezahluliweyo ezikhuselekileyo
- **Imida yezibonelelo**: Imida yexesha le-CPU, imemori, kunye nenani lemiyalelo kwiplagini nganye
- **Ukuthintela uMlinganiselo we-API**: Ihlonipha izihloko ze-`Retry-After` kunye nokulibazisa okunokulungiselelwa
- **Uqinisekiso loHlelo**: Jonga utshintsho phambi kokuba ulusebenzise, kunye neerekhodi zophicotho
- **Inkxaso ye-OAuth**: Indlela yokuqinisekisa ekhethwayo kwiibots

## ILayisensi

Inikwe ilayisensi phantsi kwenye yezi zilandelayo:

- ILayisensi ye-Apache, Inguqulelo 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) okanye http://www.apache.org/licenses/LICENSE-2.0)
- ILayisensi ye-MIT ([LICENSE-MIT](../../../LICENSE-MIT) okanye http://opensource.org/licenses/MIT)

ngokokhetho lwakho.

## Ukufaka iGalelo

Igalelo lamkelekile! Nceda ujonge ku-[CONTRIBUTING.md](../../../CONTRIBUTING.md) ngezikhokelo.

## Imeko yeProjekthi

Olu lubhalo olutsha olusebenzayo lwe-AutoWikiBrowser nge-Rust. Imeko yangoku:

- [x] Umxhasi ongundoqo we-API
- [x] Injini yemithetho
- [x] Ujongano lomgca womyalelo (CLI)
- [x] Inkqubo yeeplagini (Lua/WASM)
- [x] Ukwahlulwa kokhuseleko (sandboxing)
- [ ] Ujongano lwetheminali TUI (luyaqhubeka)
- [ ] Ujongano lwezithombe GUI (lucwangcisiwe)
- [ ] Umboniso ophucukileyo womahluko wotshintsho (ucwangcisiwe)

## Amakhonkco

- [Amaxwebhu oLwazi](https://docs.rs/awb-rs)
- [Umkhondo weeNgxaki](https://github.com/somethingwithproof/awb-rs/issues)
- [I-AWB yoQobo](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
