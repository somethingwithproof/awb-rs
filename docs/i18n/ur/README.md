# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Rust میں AutoWikiBrowser (AWB) کی ایک جدید اور اعلیٰ کارکردگی کی حامل ازسرِ نو تشکیل، جو تحفظ، رفتار اور وسعت پذیری پر خصوصی توجہ کے ساتھ ویکیپیڈیا کی خودکار تدوین کی سہولیات فراہم کرتی ہے۔

## فنِ تعمیر

```
awb-rs/
├── awb-core/           # بنیادی کاروباری منطق اور تجریدات
├── awb-api/            # MediaWiki API کلائنٹ کا نفاذ
├── awb-cli/            # کمانڈ لائن انٹرفیس
├── awb-rules/          # خودکار تدوین کے لیے قواعد کا انجن
├── awb-lua-plugin/     # Lua پلگ اِن رن ٹائم
├── awb-wasm-plugin/    # WebAssembly پلگ اِن رن ٹائم
├── awb-security/       # تحفظ اور سینڈ باکسنگ
├── awb-db/             # مستقل حالت کا انتظام
├── awb-ui/             # ٹرمینل صارف انٹرفیس کے اجزاء
├── awb-fixtures/       # آزمائشی فکسچرز اور نقلی ڈیٹا
└── awb-integration/    # مربوط جانچ کا مجموعہ
```

**ڈیٹا کا بہاؤ:**
```
CLI → بنیادی تہہ → API → MediaWiki
        ↓
     قواعد کا انجن → پلگ اِنز (Lua/WASM)
        ↓
     تحفظ کی تہہ → سینڈ باکس میں عمل درآمد
        ↓
     ڈیٹا بیس → حالت کا محفوظ اندراج
```

## خصوصیات

- **جدید فنِ تعمیر**: اعلیٰ کارکردگی کے لیے Tokio رن ٹائم کے ساتھ غیر ہم وقت (`async/await`) نفاذ
- **متعدد ویکی کی معاونت**: MediaWiki پر مبنی کسی بھی ویکی کے ساتھ کام کرنے کی صلاحیت
- **قواعد پر مبنی تدوین**: اعلانیہ TOML پر مبنی قواعد کی تعریفیں
- **پلگ اِن نظام**: Lua اسکرپٹس اور WebAssembly ماڈیولز کے ذریعے وسعت پذیر
- **مضبوط تحفظ**: وسائل کی حدود کے ساتھ سینڈ باکس میں پلگ اِن کا عمل درآمد
- **اجتماعی کارروائیاں**: بڑی تعداد میں صفحات کی مؤثر پردازش
- **تفاعلی TUI**: نگرانی اور کنٹرول کے لیے جامع ٹرمینل انٹرفیس
- **آزمائشی موڈ (ڈرائی رن)**: تبدیلیاں لاگو کرنے سے پہلے ان کا پیش منظر دیکھنے کی سہولت
- **حالت کا انتظام**: منقطع ہونے والے سیشنز کو دوبارہ جاری رکھنے کی صلاحیت
- **شرح کی تحدید**: ویکی API کی حدود اور بہترین طریقوں کی پابندی
- **بین الاقوامیت**: `--lang`، `AWB_LANG`، یا POSIX لوکیل (`LC_ALL`/`LC_MESSAGES`/`LANG`) کے ذریعے 45 نمائشی زبانیں، بشمول عربی، فارسی، عبرانی اور اردو کے لیے RTL ترتیب کی معاونت

## معاونت یافتہ زبانیں

CLI، بوٹ رپورٹس اور GTK4 صارف انٹرفیس 45 زبانوں کی معاونت کرتے ہیں (جن میں 6 RTL لوکیلز شامل ہیں: `ar`، `ar-EG`، `ar-MA`، `fa`، `he`، اور `ur`)۔ CLI میں `--lang <LANG>` استعمال کریں، یا CLI اور GTK4 صارف انٹرفیس دونوں کے لیے `AWB_LANG` یا POSIX لوکیل متغیرات (`LC_ALL`/`LC_MESSAGES`/`LANG`) متعین کریں۔ زبانوں کی مکمل فہرست کے لیے [README.md](../../../README.md) ملاحظہ کریں۔

## پیشگی تقاضے

- **Rust**: ورژن 1.94 یا اس سے جدید (MSRV)
- **Cargo**: Rust کی تنصیب کے ساتھ شامل ہے

## تنصیب

### ماخذ کوڈ سے

```bash
# ریپوزٹری کلون کریں
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# ورک اسپیس کے تمام کریٹس تیار کریں
cargo build --workspace --release

# بائنری نصب کریں
cargo install --path crates/awb_cli
```

## ترقی

### بلڈ

```bash
# ڈی بگ بلڈ
cargo build --workspace

# ریلیز بلڈ
cargo build --workspace --release
```

### جانچ

```bash
# تمام ٹیسٹ چلائیں
cargo test --workspace

# آؤٹ پٹ کے ساتھ ٹیسٹ چلائیں
cargo test --workspace -- --nocapture
```

### لنٹنگ

```bash
# لنٹنگ مسائل کی جانچ کریں
cargo clippy --workspace --all-targets --all-features

# خودکار اصلاحات لاگو کریں
cargo clippy --fix --allow-dirty --workspace
```

### فارمیٹنگ

```bash
# کوڈ فارمیٹنگ کی جانچ کریں
cargo fmt --all -- --check

# کوڈ فارمیٹنگ لاگو کریں
cargo fmt --all
```

## استعمال کی مثالیں

### توثیق

```bash
# ویکی میں لاگ اِن کریں (اسناد محفوظ طریقے سے ذخیرہ کی جاتی ہیں)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# OAuth کے ذریعے لاگ اِن کریں
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### صفحات کی فہرست سازی

```bash
# کسی زمرے سے صفحات کی فہرست حاصل کریں
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# تلاش کے نتائج سے صفحات کی فہرست حاصل کریں
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# کسی فائل سے صفحات کی فہرست حاصل کریں
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### تدوینی قواعد کا نفاذ

```bash
# پروفائل کے ساتھ قواعد چلائیں (آزمائشی موڈ)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# اصل صفحات پر قواعد لاگو کریں
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### خودکار بوٹ موڈ

```bash
# حدود کے ساتھ خودکار بوٹ موڈ میں چلائیں
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# پچھلے سیشن سے دوبارہ جاری رکھیں
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### مثالی قواعد پروفائل

ایک `my-rules.toml` فائل بنائیں:

```toml
[profile]
name = "صفائی بوٹ"
description = "عمومی فارمیٹنگ مسائل کی درستی"
edit_summary = "خودکار صفائی (بوٹ)"

[[rules]]
name = "دوہری خالی جگہوں کی درستی"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "بیرونی روابط کی درستی"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "گمشدہ زمروں کا اضافہ"
plugin = "add_categories.lua"
```

## پلگ اِن نظام

awb-rs دو اقسام کے پلگ اِنز کی معاونت کرتا ہے:

### Lua پلگ اِنز

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### WebAssembly پلگ اِنز

```rust
// WASM میں کمپائل کرنے کے لیے: cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**پلگ اِن خصوصیات:**
- وسائل کی حدود کے ساتھ سینڈ باکس میں عمل درآمد (میموری، CPU وقت)
- API کے محفوظ ذیلی مجموعے تک رسائی (فائل سسٹم اور نیٹ ورک تک رسائی نہیں)
- ورژن کی مطابقت کی جانچ
- ترقیاتی موڈ میں فوری ری لوڈ کی معاونت

## سیکیورٹی ماڈل

- **اسناد کا ذخیرہ**: پاس ورڈز کے محفوظ اندراج کے لیے آپریٹنگ سسٹم کے کی رِنگ کا استعمال
- **پلگ اِن سینڈ باکسنگ**: Lua اور WASM الگ تھلگ محفوظ ماحول میں چلتے ہیں
- **وسائل کی حدود**: فی پلگ اِن CPU وقت، میموری اور ہدایات کی تعداد کی حدود
- **API شرح کی تحدید**: `Retry-After` ہیڈرز اور قابلِ ترتیب تاخیر کی پابندی
- **تدوین کی توثیق**: لاگو کرنے سے پہلے تبدیلیوں کا پیش منظر اور آڈٹ لاگز
- **OAuth معاونت**: بوٹس کے لیے ترجیحی توثیقی طریقہ کار

## لائسنس

مندرجہ ذیل میں سے کسی ایک کے تحت لائسنس یافتہ:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) یا http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../../../LICENSE-MIT) یا http://opensource.org/licenses/MIT)

آپ کے انتخاب کے مطابق۔

## تعاون

تعاون کا خیرمقدم کیا جاتا ہے! رہنما اصولوں کے لیے براہِ کرم [CONTRIBUTING.md](../../../CONTRIBUTING.md) ملاحظہ کریں۔

## منصوبے کی صورتِ حال

یہ Rust میں AutoWikiBrowser کی ایک فعال ازسرِ نو تشکیل ہے۔ موجودہ صورتِ حال:

- [x] بنیادی API کلائنٹ
- [x] قواعد کا انجن
- [x] CLI انٹرفیس
- [x] پلگ اِن نظام (Lua/WASM)
- [x] تحفظی سینڈ باکسنگ
- [ ] ٹرمینل صارف انٹرفیس (جاری ہے)
- [ ] گرافیکل انٹرفیس (منصوبہ بند)
- [ ] جدید فرق کی نمائش (منصوبہ بند)

## روابط

- [دستاویزات](https://docs.rs/awb-rs)
- [مسائل کا ٹریکر](https://github.com/somethingwithproof/awb-rs/issues)
- [اصل AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
