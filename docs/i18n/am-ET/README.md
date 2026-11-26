# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

በደህንነት፣ በአፈጻጸም እና በመስፋፋት ችሎታ ላይ በማተኮር ራስ-ሰር የዊኪፒዲያ አርትዖት አቅምን የሚሰጥ፣ በRust ቋንቋ የተጻፈ ዘመናዊና ከፍተኛ አፈጻጸም ያለው የAutoWikiBrowser (AWB) ዳግም ትግበራ።

## መዋቅር

```
awb-rs/
├── awb-core/           # ዋና የሥራ አመክንዮ እና ረቂቅ አወቃቀሮች
├── awb-api/            # የMediaWiki API ደንበኛ ትግበራ
├── awb-cli/            # የትዕዛዝ መስመር በይነገጽ
├── awb-rules/          # ለአርትዖት ራስ-ሰር አሠራር የደንብ ሞተር
├── awb-lua-plugin/     # የLua ተሰኪ ማስፈጸሚያ አካባቢ
├── awb-wasm-plugin/    # የWebAssembly ተሰኪ ማስፈጸሚያ አካባቢ
├── awb-security/       # ደህንነት እና የገለልተኛ አካባቢ ቁጥጥር
├── awb-db/             # ዘላቂ የሁኔታ አስተዳደር
├── awb-ui/             # የተርሚናል የተጠቃሚ በይነገጽ አካላት
├── awb-fixtures/       # የሙከራ ማዘጋጃዎች እና ምሳሌያዊ ውሂብ
└── awb-integration/    # የተቀናጀ የሙከራ ስብስብ
```

**የውሂብ ፍሰት፦**
```
CLI → ዋና ንብርብር → API → MediaWiki
         ↓
      የደንብ ሞተር → ተሰኪዎች (Lua/WASM)
         ↓
      የደህንነት ንብርብር → ገለልተኛ አፈጻጸም
         ↓
      የውሂብ ጎታ → የሁኔታ ማስቀመጫ
```

## ዋና ዋና ባህሪያት

- **ዘመናዊ መዋቅር**፦ ለከፍተኛ አፈጻጸም ከTokio ማስፈጸሚያ ጋር የተገነባ ያልተመሳሰለ (`async/await`) አሠራር
- **ባለብዙ ዊኪ ድጋፍ**፦ በMediaWiki ከሚሠራ ማንኛውም ዊኪ ጋር ይሠራል
- **በደንብ ላይ የተመሠረተ አርትዖት**፦ በTOML የተገለጹ ግልጽ የደንብ መግለጫዎች
- **የተሰኪ ሥርዓት**፦ በLua ስክሪፕቶች እና በWebAssembly ሞጁሎች ሊሰፋ የሚችል
- **ጠንካራ ደህንነት**፦ የሀብት ገደብ ያለው ገለልተኛ የተሰኪ ማስፈጸሚያ
- **የጅምላ ክንውኖች**፦ በርካታ ገጾችን በብቃት ማቀናበር
- **መስተጋብራዊ TUI**፦ ለክትትልና ቁጥጥር የሚሆን የተሟላ የተርሚናል በይነገጽ
- **የሙከራ ሂደት (Dry-Run)**፦ ለውጦችን ከመተግበር በፊት አስቀድሞ መመልከት
- **የሁኔታ አስተዳደር**፦ የተቋረጡ ክፍለ-ጊዜዎችን ካቆሙበት መቀጠል
- **የፍጥነት ገደብ**፦ የዊኪ API ገደቦችን እና ምርጥ ልምዶችን ያከብራል
- **ዓለም አቀፋዊ ድጋፍ**፦ በ`--lang`፣ `AWB_LANG` ወይም POSIX አካባቢ (`LC_ALL`/`LC_MESSAGES`/`LANG`) አማካኝነት 45 የማሳያ ቋንቋዎች፣ ለአረብኛ፣ ፋርስኛ፣ ዕብራይስጥ እና ኡርዱ የRTL አቀማመጥ ድጋፍን ጨምሮ

## የሚደገፉ ቋንቋዎች

CLI፣ የቦት ሪፖርቶች እና የGTK4 ተጠቃሚ በይነገጽ 45 ቋንቋዎችን ይደግፋሉ (6 የRTL አካባቢዎችን ጨምሮ፦ `ar`፣ `ar-EG`፣ `ar-MA`፣ `fa`፣ `he` እና `ur`)። በCLI ውስጥ `--lang <LANG>` ይጠቀሙ፣ ወይም ለCLI እና ለGTK4 ተጠቃሚ በይነገጽ ሁለቱም `AWB_LANG` ወይም የPOSIX አካባቢ ተለዋዋጮችን (`LC_ALL`/`LC_MESSAGES`/`LANG`) ያቀናብሩ። ሙሉውን የቋንቋዎች ሠንጠረዥ ለማየት [README.md](../../../README.md) ይመልከቱ።

## ቅድመ ሁኔታዎች

- **Rust**፦ ስሪት 1.94 ወይም ከዚያ በላይ (MSRV)
- **Cargo**፦ ከRust ጭነት ጋር አብሮ ይመጣል

## መጫን

### ከምንጭ ኮድ

```bash
# ማከማቻውን ይቅዱ
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# ሁሉንም የሥራ ቦታ ክሬቶች ይገንቡ
cargo build --workspace --release

# ተፈጻሚውን ፋይል ይጫኑ
cargo install --path crates/awb_cli
```

## ልማት

### ግንባታ

```bash
# የማረሚያ ግንባታ
cargo build --workspace

# የልቀት ግንባታ
cargo build --workspace --release
```

### ሙከራ

```bash
# ሁሉንም ሙከራዎች ያሂዱ
cargo test --workspace

# ሙከራዎችን ከሙሉ ውጤት ጋር ያሂዱ
cargo test --workspace -- --nocapture
```

### የኮድ ጥራት ፍተሻ

```bash
# የኮድ ጥራት ችግሮችን ይፈትሹ
cargo clippy --workspace --all-targets --all-features

# ራስ-ሰር ማስተካከያዎችን ይተግብሩ
cargo clippy --fix --allow-dirty --workspace
```

### አቀራረጽ

```bash
# የኮድ አቀራረጽን ይፈትሹ
cargo fmt --all -- --check

# የኮድ አቀራረጽን ይተግብሩ
cargo fmt --all
```

## የአጠቃቀም ምሳሌዎች

### ማረጋገጫ

```bash
# ወደ ዊኪ ይግቡ (የመግቢያ መረጃዎች በደህና ይቀመጣሉ)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# በOAuth ይግቡ
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### ገጾችን መዘርዘር

```bash
# ከመደብ ውስጥ ገጾችን ይዘርዝሩ
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# ከፍለጋ ውጤቶች ገጾችን ይዘርዝሩ
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# ከፋይል ውስጥ ገጾችን ይዘርዝሩ
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### የአርትዖት ደንቦችን ማስኬድ

```bash
# ደንቦችን በመገለጫ ያሂዱ (የሙከራ ሂደት)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# ደንቦችን በእውነተኛ ገጾች ላይ ይተግብሩ
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### ራስ-ሰር የቦት ሁነታ

```bash
# ያለ ክትትል በቦት ሁነታ ከገደቦች ጋር ያሂዱ
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# ከቀዳሚው ክፍለ-ጊዜ ካቆመበት ይቀጥሉ
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### የደንብ መገለጫ ምሳሌ

`my-rules.toml` የተባለ ፋይል ይፍጠሩ፦

```toml
[profile]
name = "የጽዳት ቦት"
description = "የተለመዱ የአቀራረጽ ችግሮችን ያስተካክላል"
edit_summary = "ራስ-ሰር ጽዳት (ቦት)"

[[rules]]
name = "ድርብ ክፍተቶችን አስተካክል"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "ውጫዊ አገናኞችን አስተካክል"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "የጎደሉ መደቦችን ጨምር"
plugin = "add_categories.lua"
```

## የተሰኪ ሥርዓት

awb-rs ሁለት ዓይነት ተሰኪዎችን ይደግፋል፦

### የLua ተሰኪዎች

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### የWebAssembly ተሰኪዎች

```rust
// ወደ WASM ለማጠናቀር፦ cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**የተሰኪ ባህሪያት፦**
- የሀብት ገደብ ያለው ገለልተኛ አፈጻጸም (ማህደረ ትውስታ፣ የCPU ጊዜ)
- ደህንነቱ የተጠበቀ የAPI ንዑስ ስብስብ መዳረሻ (የፋይል ሥርዓት ወይም የአውታረ መረብ መዳረሻ የለም)
- የስሪት ተኳሃኝነት ፍተሻ
- በልማት ሁነታ ፈጣን ዳግም መጫን ድጋፍ

## የደህንነት ሞዴል

- **የመግቢያ መረጃ ማስቀመጫ**፦ የይለፍ ቃላትን በደህና ለማስቀመጥ የሥርዓተ ክወናውን የቁልፍ ቀለበት ይጠቀማል
- **የተሰኪ ገለልተኛ አካባቢ**፦ Lua እና WASM በተለዩ ደህንነታቸው በተጠበቀ አካባቢዎች ውስጥ ይሠራሉ
- **የሀብት ገደቦች**፦ ለእያንዳንዱ ተሰኪ የCPU ጊዜ፣ የማህደረ ትውስታ እና የትዕዛዝ ብዛት ገደቦች
- **የAPI ፍጥነት ገደብ**፦ የ`Retry-After` ራስጌዎችን እና ሊዋቀሩ የሚችሉ መዘግየቶችን ያከብራል
- **የአርትዖት ማረጋገጫ**፦ ከመተግበር በፊት የለውጦች ቅድመ ዕይታ እና የኦዲት መዝገቦች
- **የOAuth ድጋፍ**፦ ለቦቶች ተመራጭ የማረጋገጫ ዘዴ

## ፈቃድ

ከሚከተሉት ውስጥ በአንዱ ፈቃድ የቀረበ፦

- Apache License, Version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) ወይም http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../../../LICENSE-MIT) ወይም http://opensource.org/licenses/MIT)

በእርስዎ ምርጫ መሠረት።

## አስተዋጽኦ ማድረግ

አስተዋጽኦዎች ሁልጊዜ ይበረታታሉ! መመሪያዎችን ለማየት እባክዎ [CONTRIBUTING.md](../../../CONTRIBUTING.md) ይመልከቱ።

## የፕሮጀክቱ ሁኔታ

ይህ በRust ቋንቋ በንቃት እየተሠራ ያለ የAutoWikiBrowser ዳግም ትግበራ ነው። የአሁኑ ሁኔታ፦

- [x] ዋና የAPI ደንበኛ
- [x] የደንብ ሞተር
- [x] የCLI በይነገጽ
- [x] የተሰኪ ሥርዓት (Lua/WASM)
- [x] የደህንነት ገለልተኛ አካባቢ
- [ ] የተርሚናል ተጠቃሚ በይነገጽ (በሂደት ላይ)
- [ ] የግራፊክ ተጠቃሚ በይነገጽ (በዕቅድ ላይ)
- [ ] የላቀ የልዩነት ማሳያ (በዕቅድ ላይ)

## አገናኞች

- [ሰነዶች](https://docs.rs/awb-rs)
- [የችግር መከታተያ](https://github.com/somethingwithproof/awb-rs/issues)
- [ዋናው AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
