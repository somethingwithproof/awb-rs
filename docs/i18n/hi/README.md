# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Rust में पुनः निर्मित एक आधुनिक, उच्च-प्रदर्शन AutoWikiBrowser (AWB), जो सुरक्षा, प्रदर्शन और विस्तारशीलता पर केंद्रित स्वचालित विकिपीडिया संपादन क्षमताएँ प्रदान करता है।

## आर्किटेक्चर

```
awb-rs/
├── awb-core/           # मुख्य व्यावसायिक तर्क और अमूर्तन
├── awb-api/            # MediaWiki API क्लाइंट कार्यान्वयन
├── awb-cli/            # कमांड-लाइन इंटरफ़ेस
├── awb-rules/          # संपादन स्वचालन के लिए नियम इंजन
├── awb-lua-plugin/     # Lua प्लगइन रनटाइम
├── awb-wasm-plugin/    # WebAssembly प्लगइन रनटाइम
├── awb-security/       # सुरक्षा और सैंडबॉक्सिंग
├── awb-db/             # स्थायी अवस्था प्रबंधन
├── awb-ui/             # टर्मिनल इंटरफ़ेस घटक
├── awb-fixtures/       # परीक्षण फ़िक्स्चर और मॉक डेटा
└── awb-integration/    # एकीकरण परीक्षण सूट
```

**डेटा प्रवाह:**
```
CLI → कोर → API → MediaWiki
       ↓
    नियम इंजन → प्लगइन (Lua/WASM)
       ↓
    सुरक्षा परत → सैंडबॉक्स निष्पादन
       ↓
    डेटाबेस → अवस्था संचयन
```

## प्रमुख विशेषताएँ

- **आधुनिक आर्किटेक्चर**: उच्च प्रदर्शन के लिए Tokio रनटाइम के साथ अतुल्यकालिक निष्पादन
- **बहु-विकी समर्थन**: किसी भी MediaWiki-आधारित विकी के साथ कार्य करने में सक्षम
- **नियम-आधारित संपादन**: घोषणात्मक TOML-आधारित नियम परिभाषाएँ
- **प्लगइन प्रणाली**: Lua स्क्रिप्ट और WebAssembly मॉड्यूल के माध्यम से विस्तार योग्य
- **सुदृढ़ सुरक्षा**: संसाधन सीमाओं के साथ सैंडबॉक्स वातावरण में प्लगइन निष्पादन
- **बैच संचालन**: बड़े पैमाने पर पृष्ठों का कुशल प्रसंस्करण
- **संवादात्मक टर्मिनल इंटरफ़ेस**: निगरानी और नियंत्रण के लिए समृद्ध टर्मिनल इंटरफ़ेस
- **ड्राई-रन मोड**: परिवर्तन लागू करने से पहले उनका पूर्वावलोकन
- **अवस्था प्रबंधन**: बाधित सत्रों को पुनः प्रारंभ करने की सुविधा
- **दर सीमितीकरण**: विकी एपीआई सीमाओं और सर्वोत्तम प्रथाओं का पालन
- **अंतर्राष्ट्रीयकरण**: `--lang`, `AWB_LANG`, या POSIX लोकेल (`LC_ALL`/`LC_MESSAGES`/`LANG`) के माध्यम से 45 प्रदर्शन भाषाएँ, जिनमें अरबी, फ़ारसी, हिब्रू और उर्दू के लिए दाएँ-से-बाएँ लेआउट समर्थन शामिल है

## समर्थित भाषाएँ

कमांड-लाइन इंटरफ़ेस, बॉट रिपोर्ट और GTK4 इंटरफ़ेस 45 भाषाओं का समर्थन करते हैं (जिनमें 6 दाएँ-से-बाएँ लोकेल `ar`, `ar-EG`, `ar-MA`, `fa`, `he`, और `ur` शामिल हैं)। CLI में `--lang <LANG>` का उपयोग करें, या CLI और GTK4 इंटरफ़ेस दोनों के लिए `AWB_LANG` अथवा POSIX लोकेल चर (`LC_ALL`/`LC_MESSAGES`/`LANG`) सेट करें। संपूर्ण भाषा तालिका के लिए [README.md](../../../README.md) देखें।

## पूर्वापेक्षाएँ

- **Rust**: संस्करण 1.94 या उससे उच्चतर
- **Cargo**: Rust इंस्टॉलेशन के साथ सम्मिलित

## इंस्टॉलेशन

### स्रोत कोड से निर्माण

```bash
# रिपॉज़िटरी क्लोन करें
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# सभी वर्कस्पेस क्रेट बिल्ड करें
cargo build --workspace --release

# बाइनरी इंस्टॉल करें
cargo install --path crates/awb_cli
```

## विकास

### बिल्ड

```bash
# डीबग बिल्ड
cargo build --workspace

# रिलीज़ बिल्ड
cargo build --workspace --release
```

### परीक्षण

```bash
# सभी परीक्षण चलाएँ
cargo test --workspace

# आउटपुट के साथ परीक्षण चलाएँ
cargo test --workspace -- --nocapture
```

### लिंट

```bash
# लिंटिंग समस्याओं की जाँच करें
cargo clippy --workspace --all-targets --all-features

# स्वचालित सुधार लागू करें
cargo clippy --fix --allow-dirty --workspace
```

### फ़ॉर्मैटिंग

```bash
# फ़ॉर्मैटिंग की जाँच करें
cargo fmt --all -- --check

# फ़ॉर्मैटिंग लागू करें
cargo fmt --all
```

## उपयोग के उदाहरण

### प्रमाणीकरण

```bash
# विकी में लॉगिन करें (क्रेडेंशियल सुरक्षित रूप से सहेजता है)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# OAuth के साथ लॉगिन करें
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### पृष्ठों की सूची बनाना

```bash
# किसी श्रेणी से पृष्ठों की सूची बनाएँ
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# खोज परिणाम से पृष्ठों की सूची बनाएँ
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# किसी फ़ाइल से पृष्ठों की सूची बनाएँ
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### संपादन नियम चलाना

```bash
# प्रोफ़ाइल के साथ नियम चलाएँ (ड्राई-रन मोड)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# वास्तविक पृष्ठों पर नियम लागू करें
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### स्वचालित बॉट मोड

```bash
# सीमाओं के साथ स्वचालित बॉट मोड में चलाएँ
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# पिछले सत्र से पुनः प्रारंभ करें
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### नियम प्रोफ़ाइल का उदाहरण

एक `my-rules.toml` फ़ाइल बनाएँ:

```toml
[profile]
name = "सफ़ाई बॉट"
description = "सामान्य फ़ॉर्मैटिंग समस्याएँ सुधारें"
edit_summary = "स्वचालित सफ़ाई (बॉट)"

[[rules]]
name = "दोहरे रिक्त स्थान सुधारें"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "बाहरी कड़ियाँ सुधारें"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "अनुपस्थित श्रेणियाँ जोड़ें"
plugin = "add_categories.lua"
```

## प्लगइन प्रणाली

awb-rs दो प्रकार के प्लगइन का समर्थन करता है:

### Lua प्लगइन

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### WebAssembly प्लगइन

```rust
// WASM में संकलित करने के लिए: cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**प्लगइन की विशेषताएँ:**
- संसाधन सीमाओं (मेमोरी, सीपीयू समय) के साथ सैंडबॉक्स निष्पादन
- एपीआई के सुरक्षित उपसमुच्चय तक पहुँच (फ़ाइल सिस्टम या नेटवर्क तक पहुँच नहीं)
- संस्करण संगतता जाँच
- विकास मोड में हॉट-रीलोड समर्थन

## सुरक्षा मॉडल

- **क्रेडेंशियल संग्रहण**: सुरक्षित पासवर्ड संग्रहण के लिए ऑपरेटिंग सिस्टम कीरिंग का उपयोग
- **प्लगइन सैंडबॉक्सिंग**: Lua और WASM का पृथक वातावरण में निष्पादन
- **संसाधन सीमाएँ**: प्रत्येक प्लगइन के लिए सीपीयू समय, मेमोरी और निर्देश गणना की सीमाएँ
- **एपीआई दर सीमितीकरण**: `Retry-After` हेडर और विन्यास योग्य विलंब का पालन
- **संपादन सत्यापन**: लागू करने से पहले परिवर्तनों का पूर्वावलोकन तथा ऑडिट लॉग
- **OAuth समर्थन**: बॉट के लिए अनुशंसित प्रमाणीकरण विधि

## लाइसेंस

आपकी पसंद के अनुसार निम्नलिखित में से किसी एक लाइसेंस के अंतर्गत उपलब्ध:

- अपाचे लाइसेंस, संस्करण 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) या http://www.apache.org/licenses/LICENSE-2.0)
- एमआईटी लाइसेंस ([LICENSE-MIT](../../../LICENSE-MIT) या http://opensource.org/licenses/MIT)

## योगदान

योगदान का स्वागत है! दिशानिर्देशों के लिए कृपया [CONTRIBUTING.md](../../../CONTRIBUTING.md) देखें।

## परियोजना की स्थिति

यह Rust में AutoWikiBrowser का सक्रिय पुनर्लेखन है। वर्तमान स्थिति:

- [x] मुख्य एपीआई क्लाइंट
- [x] नियम इंजन
- [x] कमांड-लाइन इंटरफ़ेस
- [x] प्लगइन प्रणाली (Lua/WASM)
- [x] सुरक्षा सैंडबॉक्सिंग
- [ ] टर्मिनल इंटरफ़ेस (प्रगति पर)
- [ ] ग्राफ़िकल इंटरफ़ेस (नियोजित)
- [ ] उन्नत अंतर विज़ुअलाइज़ेशन (नियोजित)

## लिंक

- [दस्तावेज़ीकरण](https://docs.rs/awb-rs)
- [समस्या ट्रैकर](https://github.com/somethingwithproof/awb-rs/issues)
- [मूल AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
