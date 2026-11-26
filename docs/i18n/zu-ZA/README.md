# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Ukubhalwa kabusha kwesimanje okusebenza ngezinga eliphezulu kwe-AutoWikiBrowser (AWB) nge-Rust, okuhlinzeka ngamandla okuhlela i-Wikipedia ngokuzenzakalelayo kugxilwe kwezokuphepha, ukusebenza kahle, nokwandiseka.

## Ukwakheka koHlelo

```
awb-rs/
├── awb-core/           # Inhliziyo yemisebenzi nezakhiwo eziyisisekelo
├── awb-api/            # Ukufakwa kokusebenza kweklayenti le-MediaWiki API
├── awb-cli/            # Isixhumanisi somugqa womyalo (CLI)
├── awb-rules/          # Injini yemithetho yokuhlela ngokuzenzakalelayo
├── awb-lua-plugin/     # Indawo yokusebenza yama-plugin e-Lua
├── awb-wasm-plugin/    # Indawo yokusebenza yama-plugin e-WebAssembly
├── awb-security/       # Ezokuphepha nokuhlukaniswa kwendawo yokusebenza (sandboxing)
├── awb-db/             # Ukuphathwa kokugcinwa kwesimo okuhlala njalo
├── awb-ui/             # Izingxenye zesixhumanisi setheminali (TUI)
├── awb-fixtures/       # Idatha yokuhlola nedatha yokulingisa
└── awb-integration/    # Iqoqo lezivivinyo zokuhlanganiswa
```

**Ukugeleza kweDatha:**
```
CLI → Ingqikithi → API → MediaWiki
         ↓
     Injini Yemithetho → Ama-Plugin (Lua/WASM)
         ↓
     Ungqimba Lwezokuphepha → Ukusebenza Okuhlukanisiwe
         ↓
     Isizindalwazi → Ukugcinwa Kwesimo
```

## Izici

- **Ukwakheka Kwesimanje**: Ukusebenza okungavumelani ngesikhathi (`async/await`) ngohlelo lwe-Tokio ukuze kutholakale ukusebenza okuphezulu
- **Ukusekela Ama-Wiki Amaningi**: Isebenza nanoma iyiphi i-wiki esebenzisa i-MediaWiki
- **Ukuhlela Okusekelwe Kemithetho**: Izincazelo zemithetho ezicacile ngefomethi ye-TOML
- **Uhlelo Lwama-Plugin**: Ingandiswa ngemibhalo ye-Lua namamojuli e-WebAssembly
- **Ezokuphepha Eziqinisiwe**: Ukusebenza kwama-plugin endaweni ehlukanisiwe enemikhawulo yezinsiza
- **Ukusebenza Kweqoqo**: Ukucubungula amakhasi amaningi ngempumelelo ngesikhathi esisodwa
- **I-TUI Esebenzisanayo**: Isixhumanisi setheminali esicebile sokuqapha nokulawula
- **Imodi Yokuhlola (`dry-run`)**: Buka izinguquko ngaphambi kokuzisebenzisa
- **Ukuphathwa Kwesimo**: Qhubeka namaseshini aphazamisekile
- **Ukulinganiselwa Kwezinga Lezicelo**: Ihlonipha imikhawulo ye-API ye-wiki nezindlela ezinhle kakhulu zokusebenza
- **Ukusekela Izilimi Zomhlaba**: Izilimi ezingama-45 zokubonisa nge-`--lang`, `AWB_LANG`, noma izilungiselelo zendawo ze-POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`), kufaka phakathi ukusekelwa kwesakhiwo sokubhala kusuka kwesokudla kuya kwesokunxele (RTL) sesi-Arabhu, isiPheresiya, isiHebheru, nesi-Urdu

## Izilimi Ezisekelwayo

I-CLI, imibiko ye-bot, nesixhumanisi se-GTK4 kusekela izilimi ezingama-45 (kufaka phakathi izilimi ezi-6 ze-RTL: `ar`, `ar-EG`, `ar-MA`, `fa`, `he`, kanye ne-`ur`). Sebenzisa `--lang <LANG>` ku-CLI, noma usethe i-`AWB_LANG` noma okuguquguqukayo kwendawo ye-POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`) kukho kokubili i-CLI nesixhumanisi se-GTK4. Bheka ku-[README.md](../../../README.md) ukuze ubone ithebula eligcwele lezilimi.

## Izidingo Zokuqala

- **Rust**: Inguqulo 1.94 noma ngaphezulu (MSRV)
- **Cargo**: Ifakiwe ekufakweni kwe-Rust

## Ukufakwa

### Kusuka Kukhodi Yomthombo

```bash
# Kopisha inqolobane yekhodi
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Yakha wonke ama-crates endawo yokusebenza
cargo build --workspace --release

# Faka ifayela elisebenzayo
cargo install --path crates/awb_cli
```

## Ukuthuthukiswa

### Ukwakhiwa

```bash
# Ukwakhiwa kokulungisa amaphutha (debug)
cargo build --workspace

# Ukwakhiwa kokukhishwa (release)
cargo build --workspace --release
```

### Ukuhlolwa

```bash
# Sebenzisa zonke izivivinyo
cargo test --workspace

# Sebenzisa izivivinyo ubonise imiphumela ephelele
cargo test --workspace -- --nocapture
```

### Ukuhlolwa kweKhodi (Lint)

```bash
# Hlola izinkinga zekhwalithi yekhodi
cargo clippy --workspace --all-targets --all-features

# Sebenzisa ukulungiswa okuzenzakalelayo
cargo clippy --fix --allow-dirty --workspace
```

### Ukuhlela Ubunjalo beKhodi

```bash
# Hlola ukufomethwa kwekhodi
cargo fmt --all -- --check

# Sebenzisa ukufomethwa kwekhodi
cargo fmt --all
```

## Izibonelo Zokusetshenziswa

### Ukuqinisekiswa Kokungena

```bash
# Ngena ku-wiki (igcina imininingwane yokungena ngokuphepha)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Ngena usebenzisa i-OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Ukufaka Ohlwini Amakhasi

```bash
# Faka ohlwini amakhasi avela esigabeni
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Faka ohlwini amakhasi avela ekusesheni
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Faka ohlwini amakhasi avela efayeleni
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Ukusebenzisa Imithetho Yokuhlela

```bash
# Sebenzisa imithetho ngephrofayili (imodi yokuhlola)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Sebenzisa imithetho emakhasini angempela
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Imodi yeBot Ezenzakalelayo

```bash
# Sebenza kwimodi ye-bot engaqashiwe enemikhawulo
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Qhubeka ukusuka kwiseshini yangaphambilini
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Isibonelo Sephrofayili Yemithetho

Dala ifayela elithi `my-rules.toml`:

```toml
[profile]
name = "Cleanup Bot"
description = "Lungisa izinkinga ezivamile zokufometha"
edit_summary = "Ukuhlanza okuzenzakalelayo (Bot)"

[[rules]]
name = "Lungisa izikhala eziphindwe kabili"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "Lungisa izixhumanisi zangaphandle"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "Engeza izigaba ezishodayo"
plugin = "add_categories.lua"
```

## Uhlelo Lwama-Plugin

i-awb-rs isekela izinhlobo ezimbili zama-plugin:

### Ama-Plugin e-Lua

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### Ama-Plugin e-WebAssembly

```rust
// Hlanganisa uye ku-WASM nge: cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Izici Zama-Plugin:**
- Ukusebenza okuhlukanisiwe okunemikhawulo yezinsiza (imemori, isikhathi se-CPU)
- Ukufinyelela engxenyeni ephephile ye-API kuphela (akukho ukufinyelela kuhlelo lwamafayela noma kunethiwekhi)
- Ukuhlola ukuhambisana kwezinguqulo
- Ukusekelwa kokulayisha kabusha ngokushesha kwimodi yokuthuthukisa

## Imodeli Yezokuphepha

- **Ukugcinwa Kwemininingwane Yokungena**: Isebenzisa isigcini samaphasiwedi sohlelo lokusebenza ukuze igcinwe ngokuphepha
- **Ukuhlukaniswa Kwama-Plugin**: I-Lua ne-WASM zisebenza ezindaweni ezihlukanisiwe eziphephile
- **Imikhawulo Yezinsiza**: Imikhawulo yesikhathi se-CPU, imemori, nesibalo semiyalelo ku-plugin ngayinye
- **Ukulinganiselwa Kwezinga Le-API**: Ihlonipha izihloko ze-`Retry-After` nokubambezeleka okungalungiselelwa
- **Ukuqinisekiswa Kokuhlela**: Buka izinguquko ngaphambi kokuzisebenzisa, kanye namarekhodi okucwaninga
- **Ukusekelwa Kwe-OAuth**: Indlela yokuqinisekisa ekhethwayo yama-bot

## Ilayisensi

Inikwe ilayisensi ngaphansi kwenye yalezi ezilandelayo:

- Ilayisensi ye-Apache, Inguqulo 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) noma http://www.apache.org/licenses/LICENSE-2.0)
- Ilayisensi ye-MIT ([LICENSE-MIT](../../../LICENSE-MIT) noma http://opensource.org/licenses/MIT)

ngokukhetha kwakho.

## Ukufaka Isandla

Ukufaka isandla kwamukelekile! Sicela ubheke ku-[CONTRIBUTING.md](../../../CONTRIBUTING.md) ukuze uthole imihlahlandlela.

## Isimo Sephrojekthi

Lokhu ukubhalwa kabusha okusebenzayo kwe-AutoWikiBrowser nge-Rust. Isimo samanje:

- [x] Iklayenti eliyinhloko le-API
- [x] Injini yemithetho
- [x] Isixhumanisi somugqa womyalo (CLI)
- [x] Uhlelo lwama-plugin (Lua/WASM)
- [x] Ukuhlukaniswa kwezokuphepha (sandboxing)
- [ ] Isixhumanisi setheminali TUI (kuyaqhubeka)
- [ ] Isixhumanisi sesithombe GUI (kuhleliwe)
- [ ] Ukuboniswa okuthuthukisiwe komehluko wezinguquko (kuhleliwe)

## Izixhumanisi

- [Imibhalo Yezobuchwepheshe](https://docs.rs/awb-rs)
- [Umkhondo Wezinkinga](https://github.com/somethingwithproof/awb-rs/issues)
- [I-AWB Yoqobo](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
