# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

பாதுகாப்பு, செயல்திறன் மற்றும் விரிவாக்கத்திறன் ஆகியவற்றில் கவனம் செலுத்தி, தானியங்கி விக்கிப்பீடியா தொகுப்புத் திறன்களை வழங்கும் வகையில் Rust மொழியில் மறுஉருவாக்கம் செய்யப்பட்ட நவீன, உயர் செயல்திறன் கொண்ட AutoWikiBrowser (AWB) மென்பொருள்.

## கட்டமைப்பு

```
awb-rs/
├── awb-core/           # மைய வணிக தர்க்கம் மற்றும் சுருக்க அடுக்குகள்
├── awb-api/            # MediaWiki API கிளையன்ட் செயலாக்கம்
├── awb-cli/            # கட்டளை வரி இடைமுகம்
├── awb-rules/          # தொகுப்புத் தானியக்கத்திற்கான விதிப் பொறி
├── awb-lua-plugin/     # Lua செருகுநிரல் இயக்கநேரம்
├── awb-wasm-plugin/    # WebAssembly செருகுநிரல் இயக்கநேரம்
├── awb-security/       # பாதுகாப்பு மற்றும் தனிமைப்படுத்தப்பட்ட இயக்கம்
├── awb-db/             # நிலையான நிலை மேலாண்மை
├── awb-ui/             # முனைய பயனர் இடைமுகக் கூறுகள்
├── awb-fixtures/       # சோதனை மாதிரிகள் மற்றும் போலித் தரவுகள்
└── awb-integration/    # ஒருங்கிணைப்புச் சோதனைத் தொகுப்பு
```

**தரவுப் பாய்வு:**
```
CLI → மைய அடுக்கு → API → MediaWiki
         ↓
      விதிப் பொறி → செருகுநிரல்கள் (Lua/WASM)
         ↓
      பாதுகாப்பு அடுக்கு → தனிமைப்படுத்தப்பட்ட இயக்கம்
         ↓
      தரவுத்தளம் → நிலை சேமிப்பு
```

## அம்சங்கள்

- **நவீன கட்டமைப்பு**: உயர் செயல்திறனுக்காக Tokio இயக்கநேரத்துடன் ஒத்திசையா (`async/await`) செயலாக்கம்
- **பல்-விக்கி ஆதரவு**: எந்தவொரு MediaWiki அடிப்படையிலான விக்கியுடனும் செயல்படும் திறன்
- **விதி அடிப்படையிலான தொகுப்பு**: அறிவிப்பு முறையிலான TOML விதி வரையறைகள்
- **செருகுநிரல் அமைப்பு**: Lua நிரல்கள் மற்றும் WebAssembly தொகுதிகள் மூலம் விரிவாக்கக்கூடியது
- **வலுப்படுத்தப்பட்ட பாதுகாப்பு**: வள வரம்புகளுடன் தனிமைப்படுத்தப்பட்ட செருகுநிரல் இயக்கம்
- **தொகுதிச் செயல்பாடுகள்**: அதிக எண்ணிக்கையிலான பக்கங்களைத் திறம்படச் செயலாக்குதல்
- **ஊடாடும் TUI**: கண்காணிப்பு மற்றும் கட்டுப்பாட்டிற்கான வளமான முனைய இடைமுகம்
- **சோதனை ஓட்டப் பயன்முறை (டிரை-ரன்)**: மாற்றங்களைப் பயன்படுத்துவதற்கு முன்பு முன்னோட்டம் காணுதல்
- **நிலை மேலாண்மை**: தடைபட்ட அமர்வுகளை மீண்டும் தொடருதல்
- **வீத வரம்பு கட்டுப்பாடு**: விக்கி API வரம்புகளையும் சிறந்த நடைமுறைகளையும் மதித்தல்
- **பன்னாட்டுமயமாக்கல்**: `--lang`, `AWB_LANG`, அல்லது POSIX மொழியமைப்பு (`LC_ALL`/`LC_MESSAGES`/`LANG`) வழியாக 45 காட்சி மொழிகள்; அரபி, பாரசீகம், எபிரேயம் மற்றும் உருது ஆகியவற்றிற்கான RTL தளவமைப்பு ஆதரவு உட்பட

## ஆதரிக்கப்படும் மொழிகள்

CLI, தானியங்கி அறிக்கைகள் மற்றும் GTK4 பயனர் இடைமுகம் ஆகியவை 45 மொழிகளை ஆதரிக்கின்றன (இதில் 6 RTL மொழிகள் அடங்கும்: `ar`, `ar-EG`, `ar-MA`, `fa`, `he`, மற்றும் `ur`). கணினி மொழியமைப்பு கண்டறிதலை மேலெழுத `--lang <LANG>` அளவுருவை வழங்கவும் அல்லது `AWB_LANG` மாறியை அமைக்கவும். முழு மொழி அட்டவணைக்கு [README.md](../../../README.md) கோப்பைப் பார்க்கவும்.

## முன்நிபந்தனைகள்

- **Rust**: பதிப்பு 1.85 அல்லது அதற்கு மேற்பட்டது (MSRV)
- **Cargo**: Rust நிறுவலுடன் இணைந்து வருகிறது

## நிறுவல்

### மூலக் குறியீட்டிலிருந்து

```bash
# களஞ்சியத்தை நகலெடுக்கவும்
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# அனைத்து பணியிடத் தொகுப்புகளையும் உருவாக்கவும்
cargo build --workspace --release

# இயக்கக் கோப்பை நிறுவவும்
cargo install --path awb-cli
```

## உருவாக்கம்

### உருவாக்குதல்

```bash
# பிழைதிருத்த உருவாக்கம்
cargo build --workspace

# வெளியீட்டு உருவாக்கம்
cargo build --workspace --release
```

### சோதனை

```bash
# அனைத்து சோதனைகளையும் இயக்கவும்
cargo test --workspace

# வெளியீட்டுடன் சோதனைகளை இயக்கவும்
cargo test --workspace -- --nocapture
```

### குறியீட்டுத் தரச் சோதனை

```bash
# குறியீட்டு எச்சரிக்கைகளைச் சரிபார்க்கவும்
cargo clippy --workspace --all-targets --all-features

# தானியங்கித் திருத்தங்களைப் பயன்படுத்தவும்
cargo clippy --fix --allow-dirty --workspace
```

### வடிவமைப்பு

```bash
# குறியீட்டு வடிவமைப்பைச் சரிபார்க்கவும்
cargo fmt --all -- --check

# குறியீட்டு வடிவமைப்பைப் பயன்படுத்தவும்
cargo fmt --all
```

## பயன்பாட்டு எடுத்துக்காட்டுகள்

### அங்கீகாரம்

```bash
# விக்கியில் உள்நுழையவும் (அடையாளச் சான்றுகள் பாதுகாப்பாகச் சேமிக்கப்படும்)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# OAuth மூலம் உள்நுழையவும்
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### பக்கங்களைப் பட்டியலிடுதல்

```bash
# ஒரு பகுப்பிலிருந்து பக்கங்களைப் பட்டியலிடவும்
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# தேடல் முடிவுகளிலிருந்து பக்கங்களைப் பட்டியலிடவும்
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# ஒரு கோப்பிலிருந்து பக்கங்களைப் பட்டியலிடவும்
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### தொகுப்பு விதிகளை இயக்குதல்

```bash
# சுயவிவரத்துடன் விதிகளை இயக்கவும் (சோதனை ஓட்டப் பயன்முறை)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# உண்மையான பக்கங்களில் விதிகளைப் பயன்படுத்தவும்
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### தானியங்கி பாட் பயன்முறை

```bash
# வரம்புகளுடன் மேற்பார்வையற்ற பாட் பயன்முறையில் இயக்கவும்
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# முந்தைய அமர்விலிருந்து மீண்டும் தொடரவும்
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### மாதிரி விதி சுயவிவரம்

`my-rules.toml` என்ற கோப்பை உருவாக்கவும்:

```toml
[profile]
name = "தூய்மைப்படுத்தும் தானியங்கி"
description = "பொதுவான வடிவமைப்புப் பிழைகளைச் சரிசெய்தல்"
edit_summary = "தானியங்கித் தூய்மைப்படுத்தல் (Bot)"

[[rules]]
name = "இரட்டை இடைவெளிகளைச் சரிசெய்தல்"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "வெளி இணைப்புகளைச் சரிசெய்தல்"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "விடுபட்ட பகுப்புகளைச் சேர்த்தல்"
plugin = "add_categories.lua"
```

## செருகுநிரல் அமைப்பு

awb-rs இரண்டு வகையான செருகுநிரல்களை ஆதரிக்கிறது:

### Lua செருகுநிரல்கள்

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### WebAssembly செருகுநிரல்கள்

```rust
// WASM ஆகத் தொகுக்க: cargo build --target wasm32-wasi
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**செருகுநிரல் அம்சங்கள்:**
- வள வரம்புகளுடன் தனிமைப்படுத்தப்பட்ட இயக்கம் (நினைவகம், CPU நேரம்)
- பாதுகாப்பான API துணைத்தொகுப்பிற்கான அணுகல் (கோப்பு அமைப்பு மற்றும் பிணைய அணுகல் இல்லை)
- பதிப்புப் பொருந்தக்கூடிய தன்மைச் சரிபார்ப்பு
- உருவாக்கப் பயன்முறையில் உடனடி மறுஏற்ற ஆதரவு

## பாதுகாப்பு மாதிரி

- **அடையாளச் சான்று சேமிப்பு**: கடவுச்சொற்களைப் பாதுகாப்பாகச் சேமிக்க இயக்க முறைமையின் திறவுகோல் வளையத்தைப் பயன்படுத்துகிறது
- **செருகுநிரல் தனிமைப்படுத்தல்**: Lua மற்றும் WASM ஆகியவை தனிமைப்படுத்தப்பட்ட சூழல்களில் இயங்குகின்றன
- **வள வரம்புகள்**: ஒவ்வொரு செருகுநிரலுக்கும் CPU நேரம், நினைவகம் மற்றும் கட்டளை எண்ணிக்கை வரம்புகள்
- **API வீத வரம்பு**: `Retry-After` தலைப்புகளையும் தனிப்பயனாக்கக்கூடிய தாமதங்களையும் மதிக்கிறது
- **தொகுப்பு சரிபார்ப்பு**: பயன்படுத்துவதற்கு முன்பு மாற்றங்களின் முன்னோட்டம் மற்றும் தணிக்கைப் பதிவுகள்
- **OAuth ஆதரவு**: தானியங்கிகளுக்குப் பரிந்துரைக்கப்படும் அங்கீகார முறை

## உரிமம்

பின்வரும் உரிமங்களில் ஏதேனும் ஒன்றின் கீழ் உரிமம் வழங்கப்படுகிறது:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) அல்லது http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../../../LICENSE-MIT) அல்லது http://opensource.org/licenses/MIT)

உங்கள் விருப்பத்திற்கு ஏற்ப.

## பங்களிப்பு

பங்களிப்புகள் வரவேற்கப்படுகின்றன! வழிகாட்டுதல்களுக்கு [CONTRIBUTING.md](../../../CONTRIBUTING.md) கோப்பைப் பார்க்கவும்.

## திட்ட நிலை

இது Rust மொழியில் AutoWikiBrowser-ஐ மறுஉருவாக்கம் செய்யும் செயல்பாட்டிலுள்ள திட்டமாகும். தற்போதைய நிலை:

- [x] மைய API கிளையன்ட்
- [x] விதிப் பொறி
- [x] CLI இடைமுகம்
- [x] செருகுநிரல் அமைப்பு (Lua/WASM)
- [x] பாதுகாப்பு தனிமைப்படுத்தல்
- [ ] முனைய பயனர் இடைமுகம் (நடைபெற்று வருகிறது)
- [ ] வரைகலை பயனர் இடைமுகம் (திட்டமிடப்பட்டுள்ளது)
- [ ] மேம்பட்ட வேறுபாட்டுக் காட்சிப்படுத்தல் (திட்டமிடப்பட்டுள்ளது)

## இணைப்புகள்

- [ஆவணங்கள்](https://docs.rs/awb-rs)
- [சிக்கல் கண்காணிப்பாளர்](https://github.com/somethingwithproof/awb-rs/issues)
- [மூல AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
