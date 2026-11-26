# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Dib-u-qoris casri ah oo waxqabad sare leh oo lagu sameeyay AutoWikiBrowser (AWB) iyadoo la adeegsanayo Rust, taas oo bixisa awoodaha tafatirka otomaatiga ah ee Wikipedia iyadoo xoogga la saarayo amniga, waxqabadka, iyo ballaarinta.

## Qaab-dhismeedka

```
awb-rs/
├── awb-core/           # Xeerarka aasaasiga ah ee shaqada iyo qaab-dhismeedka guud
├── awb-api/            # Hirgelinta macmiilka MediaWiki API
├── awb-cli/            # Isdhexgalka khadka taliska (CLI)
├── awb-rules/          # Matoorka xeerarka ee tafatirka otomaatiga ah
├── awb-lua-plugin/     # Deegaanka fulinta kordhinta Lua
├── awb-wasm-plugin/    # Deegaanka fulinta kordhinta WebAssembly
├── awb-security/       # Amniga iyo gooni-u-soocidda (sandboxing)
├── awb-db/             # Maareynta kaydinta xaaladda joogtada ah
├── awb-ui/             # Qaybaha isdhexgalka terminal-ka (TUI)
├── awb-fixtures/       # Xogta tijaabada iyo xogta mala-awaalka ah
└── awb-integration/    # Xirmada tijaabooyinka isku-dhafka
```

**Qulqulka Xogta:**
```
CLI → Xudunta → API → MediaWiki
         ↓
     Matoorka Xeerarka → Kordhinta (Lua/WASM)
         ↓
     Lakabka Amniga → Fulin Gooni-u-soocan
         ↓
     Keydka Xogta → Kaydinta Xaaladda
```

## Astaamaha

- **Qaab-dhismeed Casri ah**: Fulinta aan is-sugayn (`async/await`) oo leh deegaanka Tokio si loo helo waxqabad sare
- **Taageerada Wiki-yo Badan**: Waxay la shaqeysaa wiki kasta oo ku dhisan MediaWiki
- **Tafatir ku Saleysan Xeerar**: Qeexidda xeerarka oo cad iyadoo la adeegsanayo qaabka TOML
- **Nidaamka Kordhinta (Plugins)**: Waxaa lagu ballaarin karaa qoraallada Lua iyo qaybaha WebAssembly
- **Amni Adag**: Fulinta kordhinta oo gooni-u-soocan lehna xaddidaad kheyraad
- **Hawlaha Wadajirka ah**: Habaynta boggag badan hal mar si hufan
- **TUI Isdhexgal leh**: Muuqaalka terminal-ka oo hodan ah oo loogu talagalay la-socodka iyo maamulka
- **Habka Tijaabada (`dry-run`)**: Eegista isbeddellada ka hor inta aan la hirgelin
- **Maareynta Xaaladda**: Dib-u-bilaabidda fadhiyada kala go'ay
- **Xaddididda Xawaaraha Codsiyada**: Waxay ixtiraamtaa xadka API ee wiki iyo hababka ugu wanaagsan
- **Caalami-gareynta**: 45 luqadood oo bandhig ah iyadoo la adeegsanayo `--lang`, `AWB_LANG`, ama dejinta deegaanka POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`), oo ay ku jirto taageerada qaabka midig-ilaa-bidix (RTL) ee Carabiga, Faarisiga, Cibraaniga, iyo Urduuga

## Luqadaha la Taageero

Khadka taliska (CLI), warbixinnada bot-ka, iyo muuqaalka GTK4 waxay taageeraan 45 luqadood (oo ay ku jiraan 6 luqadood oo RTL ah: `ar`, `ar-EG`, `ar-MA`, `fa`, `he`, iyo `ur`). Ku isticmaal `--lang <LANG>` khadka taliska (CLI), ama deji `AWB_LANG` ama doorsoomayaasha POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`) labadaba CLI iyo muuqaalka GTK4. Eeg [README.md](../../../README.md) si aad u aragto shaxda buuxda ee luqadaha.

## Shuruudaha Hore

- **Rust**: Nooca 1.94 ama ka sareeya (MSRV)
- **Cargo**: Waxay la timaaddaa ku-rakibidda Rust

## Ku-rakibidda

### Laga bilaabo Koodhka Isha

```bash
# Nuqul ka qaado kaydka koodhka
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Dhis dhammaan crates-ka goobta shaqada
cargo build --workspace --release

# Ku rakib faylka la fulin karo
cargo install --path crates/awb_cli
```

## Horumarinta

### Dhisidda

```bash
# Dhismaha cilad-saarka (debug)
cargo build --workspace

# Dhismaha sii-deynta (release)
cargo build --workspace --release
```

### Tijaabinta

```bash
# Fuli dhammaan tijaabooyinka
cargo test --workspace

# Fuli tijaabooyinka adigoo muujinaya natiijada oo dhan
cargo test --workspace -- --nocapture
```

### Hubinta Koodhka (Lint)

```bash
# Hubi dhibaatooyinka tayada koodhka
cargo clippy --workspace --all-targets --all-features

# Hirgeli hagaajinta otomaatiga ah
cargo clippy --fix --allow-dirty --workspace
```

### Qaabeynta

```bash
# Hubi qaabeynta koodhka
cargo fmt --all -- --check

# Hirgeli qaabeynta koodhka
cargo fmt --all
```

## Tusaalooyinka Isticmaalka

### Xaqiijinta Aqoonsiga

```bash
# Gal wiki (waxay si ammaan ah u kaydisaa aqoonsiga sirta ah)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Ku gal iyadoo la adeegsanayo OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Liis-gareynta Boggaga

```bash
# Liis-garee boggaga ka yimid qayb (category)
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Liis-garee boggaga ka yimid raadin
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Liis-garee boggaga ka yimid fayl
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Ku-shaqeynta Xeerarka Tafatirka

```bash
# Fuli xeerarka adigoo adeegsanaya faylka qeexidda (habka tijaabada)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Ku hirgeli xeerarka boggaga dhabta ah
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Habka Bot-ka Tooska ah

```bash
# Ku shaqee habka bot-ka iskiis u shaqeeya oo leh xaddidaad
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Ka sii wad fadhigii hore
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Tusaalaha Xogta Xeerarka

Abuur faylka `my-rules.toml`:

```toml
[profile]
name = "Cleanup Bot"
description = "Hagaajinta dhibaatooyinka qaabeynta caadiga ah"
edit_summary = "Nadiifin otomaatig ah (Bot)"

[[rules]]
name = "Hagaaji meelaha bannaan ee labanlaabmay"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "Hagaaji xiriiriyeyaasha dibadda"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "Ku dar qaybaha maqan"
plugin = "add_categories.lua"
```

## Nidaamka Kordhinta (Plugins)

awb-rs wuxuu taageeraa laba nooc oo kordhin ah:

### Kordhinta Lua

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### Kordhinta WebAssembly

```rust
// U beddel WASM adigoo adeegsanaya: cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Astaamaha Kordhinta:**
- Fulin gooni-u-soocan oo leh xaddidaad kheyraad (xusuusta, waqtiga CPU)
- Helitaanka qayb ammaan ah oo API ah oo keliya (ma laha marin u helidda nidaamka faylasha ama shabakadda)
- Hubinta iswaafajinta noocyada
- Taageerada dib-u-raridda degdegga ah (hot-reload) ee habka horumarinta

## Qaabka Amniga

- **Kaydinta Aqoonsiga**: Waxay adeegsataa kaydka furayaasha ee nidaamka hawlgalka si ammaan ah loogu kaydiyo ereyada sirta ah
- **Gooni-u-soocidda Kordhinta**: Lua iyo WASM waxay ku shaqeeyaan deegaanno gooni u go'an
- **Xaddidaadda Kheyraadka**: Xadka waqtiga CPU, xusuusta, iyo tirada amarrada ee kordhin kasta
- **Xaddididda Xawaaraha API**: Waxay ixtiraamtaa madaxyada `Retry-After` iyo dib-u-dhigista la habeyn karo
- **Xaqiijinta Tafatirka**: Eegista isbeddellada ka hor hirgelinta, iyo diiwaannada hantidhawrka
- **Taageerada OAuth**: Habka xaqiijinta aqoonsiga ee la doorbido ee loogu talagalay bot-yada

## Shatiga

Waxaa shati loogu bixiyay midkood kuwan soo socda:

- Shatiga Apache, Nooca 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) ama http://www.apache.org/licenses/LICENSE-2.0)
- Shatiga MIT ([LICENSE-MIT](../../../LICENSE-MIT) ama http://opensource.org/licenses/MIT)

sida aad doorato.

## Ka-qaybgalka

Ka-qaybgalka waa la soo dhaweynayaa! Fadlan eeg [CONTRIBUTING.md](../../../CONTRIBUTING.md) si aad u hesho tilmaamaha.

## Xaaladda Mashruuca

Tani waa dib-u-qoris firfircoon oo lagu sameynayo AutoWikiBrowser iyadoo la adeegsanayo Rust. Xaaladda hadda:

- [x] Macmiilka aasaasiga ah ee API
- [x] Matoorka xeerarka
- [x] Isdhexgalka khadka taliska (CLI)
- [x] Nidaamka kordhinta (Lua/WASM)
- [x] Gooni-u-soocidda amniga (sandboxing)
- [ ] Muuqaalka terminal-ka TUI (socda)
- [ ] Muuqaalka garaafka GUI (qorsheysan)
- [ ] Muujinta horumarsan ee farqiga isbeddellada (qorsheysan)

## Xiriiriyeyaasha

- [Dukumiintiyada](https://docs.rs/awb-rs)
- [Diiwaanka Ciladaha](https://github.com/somethingwithproof/awb-rs/issues)
- [AWB-ga Asalka ah](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
