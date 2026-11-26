# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

భద్రత, పనితీరు మరియు విస్తరణ సామర్థ్యంపై దృష్టి సారిస్తూ స్వయంచాలక వికీపీడియా సవరణ సామర్థ్యాలను అందించే, Rust లో ఆధునికంగా మరియు అధిక పనితీరుతో పునర్నిర్మించబడిన AutoWikiBrowser (AWB).

## నిర్మాణం

```
awb-rs/
├── awb-core/           # ప్రధాన వ్యాపార తర్కం మరియు నైరూప్యతలు
├── awb-api/            # MediaWiki API క్లయింట్ అమలు
├── awb-cli/            # కమాండ్-లైన్ ఇంటర్‌ఫేస్
├── awb-rules/          # సవరణ స్వయంచాలీకరణ కోసం నియమాల ఇంజిన్
├── awb-lua-plugin/     # Lua ప్లగిన్ రన్‌టైమ్
├── awb-wasm-plugin/    # WebAssembly ప్లగిన్ రన్‌టైమ్
├── awb-security/       # భద్రత మరియు శాండ్‌బాక్సింగ్
├── awb-db/             # శాశ్వత స్థితి నిర్వహణ
├── awb-ui/             # టెర్మినల్ UI భాగాలు
├── awb-fixtures/       # పరీక్షా ఫిక్స్‌చర్లు మరియు నమూనా డేటా
└── awb-integration/    # సమీకృత పరీక్షా సముదాయం
```

**డేటా ప్రవాహం:**
```
CLI → ప్రధాన పొర → API → MediaWiki
         ↓
      నియమాల ఇంజిన్ → ప్లగిన్లు (Lua/WASM)
         ↓
      భద్రతా పొర → శాండ్‌బాక్స్ అమలు
         ↓
      డేటాబేస్ → స్థితి నిల్వ
```

## లక్షణాలు

- **ఆధునిక నిర్మాణం**: అధిక పనితీరు కోసం Tokio రన్‌టైమ్‌తో అసింక్రోనస్ (`async/await`) విధానం
- **బహుళ-వికీ మద్దతు**: ఏదైనా MediaWiki ఆధారిత వికీతో పనిచేసే సామర్థ్యం
- **నియమ ఆధారిత సవరణ**: ప్రకటనాత్మక TOML ఆధారిత నియమ నిర్వచనాలు
- **ప్లగిన్ వ్యవస్థ**: Lua స్క్రిప్ట్‌లు మరియు WebAssembly మాడ్యూళ్ల ద్వారా విస్తరించదగినది
- **పటిష్టమైన భద్రత**: వనరుల పరిమితులతో శాండ్‌బాక్స్ చేయబడిన ప్లగిన్ అమలు
- **సమూహ చర్యలు (బ్యాచ్ ఆపరేషన్లు)**: పెద్ద సంఖ్యలో పేజీలను సమర్థవంతంగా ప్రాసెస్ చేయడం
- **ఇంటరాక్టివ్ TUI**: పర్యవేక్షణ మరియు నియంత్రణ కోసం సమగ్ర టెర్మినల్ ఇంటర్‌ఫేస్
- **డ్రై-రన్ మోడ్**: మార్పులను వర్తింపజేసే ముందు వాటి ముందస్తు వీక్షణ
- **స్థితి నిర్వహణ**: మధ్యలో ఆగిపోయిన సెషన్‌లను తిరిగి ప్రారంభించడం
- **రేటు పరిమితి నియంత్రణ**: వికీ API పరిమితులు మరియు ఉత్తమ పద్ధతులను పాటించడం
- **అంతర్జాతీయీకరణ**: `--lang`, `AWB_LANG`, లేదా POSIX లోకేల్ (`LC_ALL`/`LC_MESSAGES`/`LANG`) ద్వారా 45 ప్రదర్శన భాషలు; అరబిక్, పర్షియన్, హిబ్రూ మరియు ఉర్దూ భాషలకు RTL లేఅవుట్ మద్దతుతో సహా

## మద్దతు ఉన్న భాషలు

CLI, బాట్ నివేదికలు మరియు GTK4 UI 45 భాషలకు మద్దతు ఇస్తాయి (ఇందులో 6 RTL లోకేల్స్ ఉన్నాయి: `ar`, `ar-EG`, `ar-MA`, `fa`, `he`, మరియు `ur`). సిస్టమ్ లోకేల్ గుర్తింపును అధిగమించడానికి `--lang <LANG>` ఇవ్వండి లేదా `AWB_LANG` సెట్ చేయండి. పూర్తి భాషల పట్టిక కోసం [README.md](../../../README.md) చూడండి.

## ముందస్తు అవసరాలు

- **Rust**: వెర్షన్ 1.85 లేదా ఆపై వెర్షన్ (MSRV)
- **Cargo**: Rust స్థాపనతో పాటు అందుబాటులో ఉంటుంది

## స్థాపన

### సోర్స్ కోడ్ నుండి

```bash
# రిపోజిటరీని క్లోన్ చేయండి
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# అన్ని వర్క్‌స్పేస్ క్రేట్‌లను నిర్మించండి
cargo build --workspace --release

# బైనరీని స్థాపించండి
cargo install --path awb-cli
```

## అభివృద్ధి

### బిల్డ్

```bash
# డీబగ్ బిల్డ్
cargo build --workspace

# రిలీజ్ బిల్డ్
cargo build --workspace --release
```

### పరీక్షలు

```bash
# అన్ని పరీక్షలను అమలు చేయండి
cargo test --workspace

# అవుట్‌పుట్‌తో పరీక్షలను అమలు చేయండి
cargo test --workspace -- --nocapture
```

### లింట్

```bash
# లింటింగ్ సమస్యలను తనిఖీ చేయండి
cargo clippy --workspace --all-targets --all-features

# స్వయంచాలక పరిష్కారాలను వర్తింపజేయండి
cargo clippy --fix --allow-dirty --workspace
```

### ఫార్మాటింగ్

```bash
# కోడ్ ఫార్మాటింగ్‌ను తనిఖీ చేయండి
cargo fmt --all -- --check

# కోడ్ ఫార్మాటింగ్‌ను వర్తింపజేయండి
cargo fmt --all
```

## వినియోగ ఉదాహరణలు

### ప్రామాణీకరణ

```bash
# వికీలో లాగిన్ అవ్వండి (వివరాలు సురక్షితంగా నిల్వ చేయబడతాయి)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# OAuth ద్వారా లాగిన్ అవ్వండి
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### పేజీల జాబితాను పొందడం

```bash
# వర్గం నుండి పేజీల జాబితాను పొందండి
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# శోధన నుండి పేజీల జాబితాను పొందండి
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# ఫైల్ నుండి పేజీల జాబితాను పొందండి
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### సవరణ నియమాలను అమలు చేయడం

```bash
# ప్రొఫైల్‌తో నియమాలను అమలు చేయండి (డ్రై-రన్ మోడ్)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# అసలు పేజీలపై నియమాలను వర్తింపజేయండి
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### స్వయంచాలక బాట్ మోడ్

```bash
# పరిమితులతో స్వయంచాలక బాట్ మోడ్‌లో అమలు చేయండి
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# మునుపటి సెషన్ నుండి తిరిగి ప్రారంభించండి
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### ఉదాహరణ నియమాల ప్రొఫైల్

`my-rules.toml` ఫైల్‌ను రూపొందించండి:

```toml
[profile]
name = "శుభ్రత బాట్"
description = "సాధారణ ఫార్మాటింగ్ సమస్యలను సరిచేయండి"
edit_summary = "స్వయంచాలక శుభ్రత (బాట్)"

[[rules]]
name = "రెట్టింపు ఖాళీలను సరిచేయండి"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "బాహ్య లింకులను సరిచేయండి"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "లోపించిన వర్గాలను జోడించండి"
plugin = "add_categories.lua"
```

## ప్లగిన్ వ్యవస్థ

awb-rs రెండు రకాల ప్లగిన్లకు మద్దతు ఇస్తుంది:

### Lua ప్లగిన్లు

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### WebAssembly ప్లగిన్లు

```rust
// WASM కు కంపైల్ చేయడానికి: cargo build --target wasm32-wasi
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**ప్లగిన్ లక్షణాలు:**
- వనరుల పరిమితులతో శాండ్‌బాక్స్ అమలు (మెమరీ, CPU సమయం)
- సురక్షిత API ఉపసమితికి ప్రాప్యత (ఫైల్‌సిస్టమ్ లేదా నెట్‌వర్క్ ప్రాప్యత లేదు)
- వెర్షన్ అనుకూలత తనిఖీ
- అభివృద్ధి మోడ్‌లో హాట్-రీలోడ్ మద్దతు

## భద్రతా నమూనా

- **ఆధారాల నిల్వ**: పాస్‌వర్డ్‌ల సురక్షిత నిల్వ కోసం ఆపరేటింగ్ సిస్టమ్ కీరింగ్ వినియోగం
- **ప్లగిన్ శాండ్‌బాక్సింగ్**: Lua మరియు WASM వేరుచేయబడిన సురక్షిత పరిసరాలలో అమలవుతాయి
- **వనరుల పరిమితులు**: ప్రతి ప్లగిన్‌కు CPU సమయం, మెమరీ మరియు సూచనల సంఖ్యపై పరిమితులు
- **API రేటు పరిమితి**: `Retry-After` హెడర్‌లు మరియు అనుకూలీకరించదగిన ఆలస్యాలను పాటిస్తుంది
- **సవరణ ధృవీకరణ**: వర్తింపజేసే ముందు మార్పుల ముందస్తు వీక్షణ మరియు ఆడిట్ లాగ్‌లు
- **OAuth మద్దతు**: బాట్‌ల కోసం ప్రాధాన్య ప్రామాణీకరణ పద్ధతి

## లైసెన్స్

కింది వాటిలో ఏదో ఒకదాని ప్రకారం లైసెన్స్ పొందింది:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) లేదా http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../../../LICENSE-MIT) లేదా http://opensource.org/licenses/MIT)

మీ ఎంపిక మేరకు.

## సహకారం

సహకారాలకు స్వాగతం! మార్గదర్శకాల కోసం దయచేసి [CONTRIBUTING.md](../../../CONTRIBUTING.md) చూడండి.

## ప్రాజెక్ట్ స్థితి

ఇది Rust లో AutoWikiBrowser యొక్క క్రియాశీల పునర్నిర్మాణం. ప్రస్తుత స్థితి:

- [x] ప్రధాన API క్లయింట్
- [x] నియమాల ఇంజిన్
- [x] CLI ఇంటర్‌ఫేస్
- [x] ప్లగిన్ వ్యవస్థ (Lua/WASM)
- [x] భద్రతా శాండ్‌బాక్సింగ్
- [ ] టెర్మినల్ UI (పురోగతిలో ఉంది)
- [ ] GUI (ప్రణాళికలో ఉంది)
- [ ] అధునాతన తేడాల దృశ్యీకరణ (ప్రణాళికలో ఉంది)

## లింకులు

- [డాక్యుమెంటేషన్](https://docs.rs/awb-rs)
- [సమస్యల ట్రాకర్](https://github.com/somethingwithproof/awb-rs/issues)
- [మూల AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
