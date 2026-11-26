# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Rust मध्ये नव्याने लिहिलेले आधुनिक आणि उच्च-कार्यक्षमता असलेले AutoWikiBrowser (AWB) रूपांतरण, जे सुरक्षितता, वेग आणि विस्तारक्षमतेवर भर देऊन स्वयंचलित विकिपीडिया संपादन क्षमता प्रदान करते.

## संरचना

```
awb-rs/
├── awb-core/           # मुख्य व्यवसाय तर्क आणि अमूर्त स्तर
├── awb-api/            # MediaWiki API क्लायंट अंमलबजावणी
├── awb-cli/            # कमांड-लाइन इंटरफेस
├── awb-rules/          # संपादन स्वयंचलनासाठी नियम इंजिन
├── awb-lua-plugin/     # Lua प्लगइन रनटाइम
├── awb-wasm-plugin/    # WebAssembly प्लगइन रनटाइम
├── awb-security/       # सुरक्षितता आणि सँडबॉक्सिंग
├── awb-db/             # कायमस्वरूपी स्थिती व्यवस्थापन
├── awb-ui/             # टर्मिनल UI घटक
├── awb-fixtures/       # चाचणी फिक्स्चर आणि नमुना डेटा
└── awb-integration/    # एकात्मिक चाचणी संच
```

**डेटा प्रवाह:**
```
CLI → मुख्य स्तर → API → MediaWiki
        ↓
     नियम इंजिन → प्लगइन्स (Lua/WASM)
        ↓
     सुरक्षा स्तर → सँडबॉक्स अंमलबजावणी
        ↓
     डेटाबेस → स्थिती जतन
```

## वैशिष्ट्ये

- **आधुनिक संरचना**: उच्च कार्यक्षमतेसाठी Tokio रनटाइमसह असिंक्रोनस (`async/await`) अंमलबजावणी
- **बहु-विकी समर्थन**: कोणत्याही MediaWiki-आधारित विकीवर कार्य करण्याची क्षमता
- **नियम-आधारित संपादन**: TOML स्वरूपातील घोषणात्मक नियम व्याख्या
- **प्लगइन प्रणाली**: Lua स्क्रिप्ट्स आणि WebAssembly मॉड्यूल्सद्वारे विस्तारयोग्य
- **कडक सुरक्षितता**: संसाधन मर्यादांसह सँडबॉक्स केलेले प्लगइन कार्यन्वयन
- **गट प्रक्रिया (बॅच ऑपरेशन्स)**: मोठ्या प्रमाणावर पृष्ठांची कार्यक्षम प्रक्रिया
- **संवादात्मक TUI**: निरीक्षण आणि नियंत्रणासाठी समृद्ध टर्मिनल इंटरफेस
- **चाचणी मोड (ड्राय-रन)**: बदल लागू करण्यापूर्वी त्यांची पूर्वझलक पाहण्याची सोय
- **स्थिती व्यवस्थापन**: खंडित झालेली सत्रे पुन्हा सुरू करण्याची सुविधा
- **दर मर्यादा नियंत्रण**: विकी API मर्यादा आणि सर्वोत्तम पद्धतींचे पालन
- **आंतरराष्ट्रीयीकरण**: `--lang`, `AWB_LANG` किंवा POSIX लोकेल (`LC_ALL`/`LC_MESSAGES`/`LANG`) द्वारे ४५ प्रदर्शन भाषा, ज्यामध्ये अरबी, फारसी, हिब्रू आणि उर्दूंसाठी RTL मांडणी समर्थनाचा समावेश आहे

## समर्थित भाषा

CLI, बॉट अहवाल आणि GTK4 UI ४५ भाषांना समर्थन देतात (यामध्ये ६ RTL लोकेल्स समाविष्ट आहेत: `ar`, `ar-EG`, `ar-MA`, `fa`, `he` आणि `ur`). सिस्टम लोकेल शोध ओव्हरराइड करण्यासाठी `--lang <LANG>` द्या किंवा `AWB_LANG` सेट करा. संपूर्ण भाषा सारणीसाठी [README.md](../../../README.md) पहा.

## पूर्वअटी

- **Rust**: आवृत्ती 1.85 किंवा त्यानंतरची (MSRV)
- **Cargo**: Rust स्थापनेसह उपलब्ध

## स्थापना

### स्रोत कोडमधून

```bash
# रिपॉझिटरी क्लोन करा
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# सर्व वर्कस्पेस क्रेट्स तयार करा
cargo build --workspace --release

# बायनरी स्थापित करा
cargo install --path awb-cli
```

## विकास

### बिल्ड

```bash
# डीबग बिल्ड
cargo build --workspace

# रिलीझ बिल्ड
cargo build --workspace --release
```

### चाचणी

```bash
# सर्व चाचण्या चालवा
cargo test --workspace

# आउटपुटसह चाचण्या चालवा
cargo test --workspace -- --nocapture
```

### लिंट

```bash
# लिंटिंग त्रुटी तपासा
cargo clippy --workspace --all-targets --all-features

# स्वयंचलित दुरुस्त्या लागू करा
cargo clippy --fix --allow-dirty --workspace
```

### फॉरमॅटिंग

```bash
# कोड फॉरमॅटिंग तपासा
cargo fmt --all -- --check

# कोड फॉरमॅटिंग लागू करा
cargo fmt --all
```

## वापराची उदाहरणे

### प्रमाणीकरण

```bash
# विकीवर लॉगिन करा (प्रमाणीकरण माहिती सुरक्षितपणे जतन केली जाते)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# OAuth वापरून लॉगिन करा
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### पृष्ठांची यादी तयार करणे

```bash
# श्रेणीनुसार पृष्ठांची यादी मिळवा
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# शोध परिणामांमधून पृष्ठांची यादी मिळवा
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# फाइलमधून पृष्ठांची यादी मिळवा
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### संपादन नियम चालवणे

```bash
# प्रोफाइलसह नियम चालवा (ड्राय-रन मोड)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# प्रत्यक्ष पृष्ठांवर नियम लागू करा
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### स्वयंचलित बॉट मोड

```bash
# मर्यादांसह स्वयंचलित बॉट मोडमध्ये चालवा
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# मागील सत्रापासून पुन्हा सुरू करा
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### नमुना नियम प्रोफाइल

`my-rules.toml` नावाची फाइल तयार करा:

```toml
[profile]
name = "स्वच्छता बॉट"
description = "सामान्य फॉरमॅटिंग त्रुटी दुरुस्त करा"
edit_summary = "स्वयंचलित स्वच्छता (बॉट)"

[[rules]]
name = "दुहेरी रिक्त स्थाने दुरुस्त करा"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "बाह्य दुवे दुरुस्त करा"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "नसलेल्या श्रेणी जोडा"
plugin = "add_categories.lua"
```

## प्लगइन प्रणाली

awb-rs दोन प्रकारच्या प्लगइन्सना समर्थन देते:

### Lua प्लगइन्स

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### WebAssembly प्लगइन्स

```rust
// WASM मध्ये संकलित करण्यासाठी: cargo build --target wasm32-wasi
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**प्लगइन वैशिष्ट्ये:**
- संसाधन मर्यादांसह सँडबॉक्स केलेले कार्यन्वयन (मेमरी, CPU वेळ)
- सुरक्षित API उपसंचाला प्रवेश (फाइलसिस्टम किंवा नेटवर्क प्रवेश नाही)
- आवृत्ती सुसंगतता तपासणी
- विकास मोडमध्ये हॉट-रीलोड समर्थन

## सुरक्षा मॉडेल

- **प्रमाणीकरण माहिती जतन**: सुरक्षित पासवर्ड संचयनासाठी ऑपरेटिंग सिस्टमच्या की-रिंगचा वापर
- **प्लगइन सँडबॉक्सिंग**: Lua आणि WASM वेगळ्या सुरक्षित वातावरणात चालतात
- **संसाधन मर्यादा**: प्रत्येक प्लगइनसाठी CPU वेळ, मेमरी आणि सूचना संख्येच्या मर्यादा
- **API दर मर्यादा**: `Retry-After` हेडर्स आणि सानुकूल करण्यायोग्य विलंबाचे पालन
- **संपादन पडताळणी**: लागू करण्यापूर्वी बदलांची पूर्वझलक आणि ऑडिट नोंदी
- **OAuth समर्थन**: बॉट्ससाठी प्राधान्यकृत प्रमाणीकरण पद्धत

## परवाना

खालीलपैकी कोणत्याही एका परवान्याअंतर्गत वितरित:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) किंवा http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../../../LICENSE-MIT) किंवा http://opensource.org/licenses/MIT)

तुमच्या निवडीनुसार.

## योगदान

योगदानाचे स्वागत आहे! मार्गदर्शक तत्त्वांसाठी कृपया [CONTRIBUTING.md](../../../CONTRIBUTING.md) पहा.

## प्रकल्पाची सद्यस्थिती

हा AutoWikiBrowser चे Rust मध्ये सक्रिय पुनर्लेखन असलेला प्रकल्प आहे. सद्यस्थिती:

- [x] मुख्य API क्लायंट
- [x] नियम इंजिन
- [x] CLI इंटरफेस
- [x] प्लगइन प्रणाली (Lua/WASM)
- [x] सुरक्षा सँडबॉक्सिंग
- [ ] टर्मिनल UI (प्रगतीपथावर)
- [ ] GUI (नियोजित)
- [ ] प्रगत फरक दृश्यीकरण (नियोजित)

## दुवे

- [दस्तऐवजीकरण](https://docs.rs/awb-rs)
- [समस्या ट्रॅकर](https://github.com/somethingwithproof/awb-rs/issues)
- [मूळ AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
