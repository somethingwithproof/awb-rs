# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Rust-এ পুনর্লিখিত একটি আধুনিক ও উচ্চ-ক্ষমতাসম্পন্ন AutoWikiBrowser (AWB), যা নিরাপত্তা, কর্মক্ষমতা এবং সম্প্রসারণযোগ্যতার ওপর গুরুত্ব দিয়ে স্বয়ংক্রিয় উইকিপিডিয়া সম্পাদনার সুবিধা প্রদান করে।

## আর্কিটেকচার

```
awb-rs/
├── awb-core/           # মূল ব্যবসায়িক যুক্তি এবং বিমূর্ততা
├── awb-api/            # MediaWiki API ক্লায়েন্ট বাস্তবায়ন
├── awb-cli/            # কমান্ড-লাইন ইন্টারফেস
├── awb-rules/          # স্বয়ংক্রিয় সম্পাদনার জন্য নিয়ম ইঞ্জিন
├── awb-lua-plugin/     # Lua প্লাগইন রানটাইম
├── awb-wasm-plugin/    # WebAssembly প্লাগইন রানটাইম
├── awb-security/       # নিরাপত্তা এবং স্যান্ডবক্সিং
├── awb-db/             # স্থায়ী অবস্থা ব্যবস্থাপনা
├── awb-ui/             # টার্মিনাল ইন্টারফেস উপাদান
├── awb-fixtures/       # পরীক্ষার ফিক্সচার এবং মক ডেটা
└── awb-integration/    # ইন্টিগ্রেশন টেস্ট স্যুট
```

**ডেটা প্রবাহ:**
```
CLI → কোর → API → MediaWiki
       ↓
    নিয়ম ইঞ্জিন → প্লাগইন (Lua/WASM)
       ↓
    নিরাপত্তা স্তর → স্যান্ডবক্স নির্বাহ
       ↓
    ডেটাবেস → স্থায়ী অবস্থা সংরক্ষণ
```

## বৈশিষ্ট্যসমূহ

- **আধুনিক আর্কিটেকচার**: উচ্চ কর্মক্ষমতার জন্য Tokio রানটাইমের সাথে অ্যাসিঙ্ক্রোনাস প্রোগ্রামিং
- **বহু-উইকি সমর্থন**: যেকোনো MediaWiki-চালিত উইকির সাথে কাজ করার সক্ষমতা
- **নিয়ম-ভিত্তিক সম্পাদনা**: ঘোষণামূলক TOML-ভিত্তিক নিয়ম সংজ্ঞা
- **প্লাগইন সিস্টেম**: Lua স্ক্রিপ্ট এবং WebAssembly মডিউলের মাধ্যমে সম্প্রসারণযোগ্য
- **সুদৃঢ় নিরাপত্তা**: সম্পদের সীমাবদ্ধতাসহ স্যান্ডবক্স পরিবেশে প্লাগইন নির্বাহ
- **ব্যাচ প্রক্রিয়াকরণ**: দক্ষতার সাথে বিপুল সংখ্যক পৃষ্ঠা প্রক্রিয়াকরণ
- **ইন্টারঅ্যাক্টিভ টার্মিনাল ইন্টারফেস**: পর্যবেক্ষণ ও নিয়ন্ত্রণের জন্য সমৃদ্ধ টার্মিনাল ইন্টারফেস
- **ড্রাই-রান মোড**: পরিবর্তন প্রয়োগের আগে প্রিভিউ দেখার সুবিধা
- **অবস্থা ব্যবস্থাপনা**: বিঘ্নিত সেশন পুনরায় শুরু করার সুবিধা
- **হার সীমাবদ্ধকরণ**: উইকি এপিআই সীমা এবং সর্বোত্তম অনুশীলন মেনে চলা
- **আন্তর্জাতিকীকরণ**: `--lang`, `AWB_LANG`, বা POSIX লোকেল (`LC_ALL`/`LC_MESSAGES`/`LANG`) এর মাধ্যমে ৪৫টি প্রদর্শন ভাষা, যার মধ্যে আরবি, ফার্সি, হিব্রু এবং উর্দু ভাষার জন্য ডান-থেকে-বাম বিন্যাস সমর্থন অন্তর্ভুক্ত

## সমর্থিত ভাষাসমূহ

কমান্ড-লাইন ইন্টারফেস, বট প্রতিবেদন এবং GTK4 ইন্টারফেস ৪৫টি ভাষা সমর্থন করে (যার মধ্যে ৬টি ডান-থেকে-বাম লোকেল রয়েছে: `ar`, `ar-EG`, `ar-MA`, `fa`, `he`, এবং `ur`)। CLI-তে `--lang <LANG>` ব্যবহার করুন, অথবা CLI ও GTK4 ইন্টারফেস উভয়ের জন্য `AWB_LANG` কিংবা POSIX লোকেল ভ্যারিয়েবল (`LC_ALL`/`LC_MESSAGES`/`LANG`) সেট করুন। সম্পূর্ণ ভাষার তালিকার জন্য [README.md](../../../README.md) দেখুন।

## পূর্বশর্তসমূহ

- **Rust**: সংস্করণ ১.৯৪ বা তদূর্ধ্ব
- **Cargo**: Rust ইনস্টলেশনের সাথে অন্তর্ভুক্ত

## ইনস্টলেশন

### উৎস কোড থেকে নির্মাণ

```bash
# রিপোজিটরি ক্লোন করুন
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# সকল ওয়ার্কস্পেস ক্রেট বিল্ড করুন
cargo build --workspace --release

# বাইনারি ইনস্টল করুন
cargo install --path crates/awb_cli
```

## উন্নয়ন

### বিল্ড

```bash
# ডিবাগ বিল্ড
cargo build --workspace

# রিলিজ বিল্ড
cargo build --workspace --release
```

### পরীক্ষা

```bash
# সকল পরীক্ষা চালান
cargo test --workspace

# আউটপুটসহ পরীক্ষা চালান
cargo test --workspace -- --nocapture
```

### লিন্ট

```bash
# লিন্টিং সমস্যা যাচাই করুন
cargo clippy --workspace --all-targets --all-features

# স্বয়ংক্রিয় সংশোধন প্রয়োগ করুন
cargo clippy --fix --allow-dirty --workspace
```

### ফরম্যাটিং

```bash
# কোড ফরম্যাটিং যাচাই করুন
cargo fmt --all -- --check

# কোড ফরম্যাটিং প্রয়োগ করুন
cargo fmt --all
```

## ব্যবহারের উদাহরণ

### প্রমাণীকরণ

```bash
# উইকিতে লগইন করুন (নিরাপদভাবে শংসাপত্র সংরক্ষণ করে)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# OAuth ব্যবহার করে লগইন করুন
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### পৃষ্ঠার তালিকা তৈরি

```bash
# কোনো বিষয়শ্রেণী থেকে পৃষ্ঠার তালিকা তৈরি করুন
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# অনুসন্ধান ফলাফল থেকে পৃষ্ঠার তালিকা তৈরি করুন
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# কোনো ফাইল থেকে পৃষ্ঠার তালিকা তৈরি করুন
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### সম্পাদনা নিয়ম চালানো

```bash
# প্রোফাইল ব্যবহার করে নিয়ম চালান (ড্রাই-রান মোড)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# প্রকৃত পৃষ্ঠায় নিয়ম প্রয়োগ করুন
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### স্বয়ংক্রিয় বট মোড

```bash
# সীমাবদ্ধতাসহ স্বয়ংক্রিয় বট মোডে চালান
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# পূর্ববর্তী সেশন থেকে পুনরায় শুরু করুন
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### নিয়ম প্রোফাইলের উদাহরণ

একটি `my-rules.toml` ফাইল তৈরি করুন:

```toml
[profile]
name = "পরিচ্ছন্নতা বট"
description = "সাধারণ বিন্যাসগত ত্রুটি সংশোধন"
edit_summary = "স্বয়ংক্রিয় পরিচ্ছন্নতা (বট)"

[[rules]]
name = "দ্বৈত ফাঁকা স্থান সংশোধন"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "বহিঃসংযোগ সংশোধন"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "অনুপস্থিত বিষয়শ্রেণী যোগ করুন"
plugin = "add_categories.lua"
```

## প্লাগইন সিস্টেম

awb-rs দুই ধরনের প্লাগইন সমর্থন করে:

### Lua প্লাগইন

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### WebAssembly প্লাগইন

```rust
// WASM-এ কম্পাইল করুন: cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**প্লাগইনের বৈশিষ্ট্যসমূহ:**
- সম্পদের সীমাবদ্ধতাসহ (মেমরি, সিপিইউ সময়) স্যান্ডবক্স নির্বাহ
- নিরাপদ এপিআই উপসেটে প্রবেশাধিকার (ফাইল সিস্টেম বা নেটওয়ার্ক প্রবেশাধিকার নেই)
- সংস্করণ সামঞ্জস্যতা যাচাইকরণ
- উন্নয়ন মোডে হট-রিলোড সমর্থন

## নিরাপত্তা মডেল

- **শংসাপত্র সংরক্ষণ**: নিরাপদ পাসওয়ার্ড সংরক্ষণের জন্য অপারেটিং সিস্টেম কী-রিং ব্যবহার করে
- **প্লাগইন স্যান্ডবক্সিং**: Lua এবং WASM বিচ্ছিন্ন পরিবেশে পরিচালিত হয়
- **সম্পদের সীমাবদ্ধতা**: প্রতিটি প্লাগইনের জন্য সিপিইউ সময়, মেমরি এবং নির্দেশ সংখ্যার সীমা
- **এপিআই হার সীমাবদ্ধকরণ**: `Retry-After` হেডার এবং কনফিগারযোগ্য বিলম্ব মেনে চলে
- **সম্পাদনা যাচাইকরণ**: প্রয়োগের পূর্বে পরিবর্তন প্রিভিউ এবং নিরীক্ষা লগ
- **OAuth সমর্থন**: বটের জন্য পছন্দের প্রমাণীকরণ পদ্ধতি

## লাইসেন্স

আপনার পছন্দ অনুযায়ী নিম্নলিখিত যেকোনো একটি লাইসেন্সের অধীনে উপলব্ধ:

- অ্যাপাচি লাইসেন্স, সংস্করণ ২.০ ([LICENSE-APACHE](../../../LICENSE-APACHE) বা http://www.apache.org/licenses/LICENSE-2.0)
- এমআইটি লাইসেন্স ([LICENSE-MIT](../../../LICENSE-MIT) বা http://opensource.org/licenses/MIT)

## অবদান

অবদান স্বাগত! নির্দেশিকার জন্য অনুগ্রহ করে [CONTRIBUTING.md](../../../CONTRIBUTING.md) দেখুন।

## প্রকল্পের অবস্থা

এটি Rust-এ AutoWikiBrowser-এর একটি সক্রিয় পুনর্লিখন প্রকল্প। বর্তমান অবস্থা:

- [x] মূল এপিআই ক্লায়েন্ট
- [x] নিয়ম ইঞ্জিন
- [x] কমান্ড-লাইন ইন্টারফেস
- [x] প্লাগইন সিস্টেম (Lua/WASM)
- [x] নিরাপত্তা স্যান্ডবক্সিং
- [ ] টার্মিনাল ইন্টারফেস (চলমান)
- [ ] গ্রাফিক্যাল ইন্টারফেস (পরিকল্পিত)
- [ ] উন্নত পার্থক্য প্রদর্শন (পরিকল্পিত)

## লিংকসমূহ

- [নথিপত্র](https://docs.rs/awb-rs)
- [সমস্যা ট্র্যাকার](https://github.com/somethingwithproof/awb-rs/issues)
- [মূল AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
