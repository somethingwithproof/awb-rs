# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

إعادة كتابة حديثة وبأداء عالي لـ AutoWikiBrowser (AWB) بلغة Rust، كتوفر إمكانيات التعديل الأوتوماتيكي فـ ويكيبيديا مع التركيز على الأمان، السرعة، وسهولة التوسيع.

## الهندسة المعمارية

```
awb-rs/
├── awb-core/           # المنطق الأساسي والتجريدات ديال النظام
├── awb-api/            # تطبيق عميل MediaWiki API
├── awb-cli/            # واجهة سطر الأوامر
├── awb-rules/          # محرك القواعد لأتمتة التعديلات
├── awb-lua-plugin/     # بيئة تشغيل إضافات Lua
├── awb-wasm-plugin/    # بيئة تشغيل إضافات WebAssembly
├── awb-security/       # الأمان والعزل فـ بيئة محمية
├── awb-db/             # تدبير الحالة المستمرة
├── awb-ui/             # مكونات واجهة الطرفية
├── awb-fixtures/       # معطيات الاختبار والبيانات التجريبية
└── awb-integration/    # مجموعة اختبارات التكامل
```

**تدفق المعطيات:**
```
CLI → النواة → API → MediaWiki
        ↓
     محرك القواعد → الإضافات (Lua/WASM)
        ↓
     طبقة الأمان → تشغيل معزول
        ↓
     قاعدة البيانات → حفظ الحالة
```

## المميزات

- **هندسة حديثة**: برمجة غير متزامنة (`async/await`) مع بيئة Tokio باش تعطي أداء عالي
- **دعم متعدد للويكيات**: خدام مع أي ويكي مبني على MediaWiki
- **تعديل مبني على القواعد**: تعريف القواعد بطريقة واضحة باستعمال ملفات TOML
- **نظام الإضافات**: قابل للتوسيع عبر سكربتات Lua ووحدات WebAssembly
- **أمان معزّز**: تشغيل الإضافات فـ بيئة معزولة مع حدود مضبوطة للموارد
- **معالجة جماعية**: معالجة عدد كبير ديال الصفحات بكفاءة
- **واجهة طرفية تفاعلية (TUI)**: واجهة طرفية غنية للمراقبة والتحكم
- **وضع التجربة (Dry-Run)**: معاينة التغييرات قبل ما تطبقها فعلياً
- **تدبير الحالة**: استئناف الجلسات اللي تقطعات من البلاصة فين وقفات
- **تحديد وتيرة الطلبات**: كيحترم حدود API ديال الويكي وأفضل الممارسات
- **الدعم الدولي للغات**: 45 لغة عرض عبر `--lang`، `AWB_LANG`، أو إعدادات POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`)، بما فيها دعم العرض من اليمين لليسار (RTL) للعربية، الفارسية، العبرية، والأردو

## اللغات المدعومة

واجهة CLI، تقارير البوت، وواجهة GTK4 كيدعمو 45 لغة (من بينها 6 لغات RTL: `ar`، `ar-EG`، `ar-MA`، `fa`، `he`، و`ur`). دوز `--lang <LANG>` ولا عيّن المتغير `AWB_LANG` باش تبدّل اللغة التلقائية ديال النظام. شوف [README.md](../../../README.md) باش تطالع الجدول الكامل ديال اللغات.

## المتطلبات القبلية

- **Rust**: النسخة 1.85 ولا أحدث (MSRV)
- **Cargo**: كيجي مدمج مع التثبيت ديال Rust

## التثبيت

### انطلاقاً من الكود المصدري

```bash
# نسخ المستودع
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# بناء جميع الحزم ديال مساحة العمل
cargo build --workspace --release

# تثبيت الملف التنفيذي
cargo install --path awb-cli
```

## التطوير

### البناء

```bash
# بناء للتصحيح (Debug)
cargo build --workspace

# بناء للإصدار (Release)
cargo build --workspace --release
```

### الاختبارات

```bash
# تشغيل جميع الاختبارات
cargo test --workspace

# تشغيل الاختبارات مع إظهار المخرجات كاملة
cargo test --workspace -- --nocapture
```

### فحص جودة الكود

```bash
# التحقق من مشاكل جودة الكود
cargo clippy --workspace --all-targets --all-features

# تطبيق الإصلاحات الأوتوماتيكية
cargo clippy --fix --allow-dirty --workspace
```

### التنسيق

```bash
# التحقق من تنسيق الكود
cargo fmt --all -- --check

# تطبيق تنسيق الكود
cargo fmt --all
```

## أمثلة ديال الاستعمال

### المصادقة وتسجيل الدخول

```bash
# تسجيل الدخول فـ ويكي (كيسجل المعطيات بشكل آمن)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# تسجيل الدخول باستعمال OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### عرض لائحة الصفحات

```bash
# استخراج لائحة الصفحات من تصنيف
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# استخراج لائحة الصفحات من نتائج البحث
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# استخراج لائحة الصفحات من ملف
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### تشغيل قواعد التعديل

```bash
# تشغيل القواعد بملف إعدادات (وضع التجربة)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# تطبيق القواعد على صفحات حقيقية
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### وضع البوت الأوتوماتيكي

```bash
# التشغيل فـ وضع البوت الأوتوماتيكي مع تحديد السقف
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# استكمال العمل من جلسة سابقة
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### مثال لملف القواعد

صاوب ملف سميتو `my-rules.toml`:

```toml
[profile]
name = "بوت التنقيح"
description = "إصلاح مشاكل التنسيق المتكررة"
edit_summary = "تنقيح أوتوماتيكي (بوت)"

[[rules]]
name = "إصلاح المسافات المكررة"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "إصلاح الروابط الخارجية"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "إضافة التصنيفات الناقصة"
plugin = "add_categories.lua"
```

## نظام الإضافات

awb-rs كيدعم جوج أنواع ديال الإضافات:

### إضافات Lua

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### إضافات WebAssembly

```rust
// حوّل الكود لـ WASM بهاد الأمر: cargo build --target wasm32-wasi
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**مميزات الإضافات:**
- تشغيل معزول بحدود مضبوطة للموارد (الذاكرة ووقت الـ CPU)
- ولوج لجزء آمن فقط من واجهات API (بلا ولوج لنظام الملفات ولا الشبكة)
- التحقق من توافق النسخ
- دعم إعادة التحميل الفوري فـ وضع التطوير

## نموذج الأمان

- **حفظ بيانات الدخول**: كيستعمل خزنة المفاتيح ديال نظام التشغيل لحفظ كلمات السر بأمان
- **عزل الإضافات**: إضافات Lua وWASM كيشتاغلو فـ بيئات معزولة
- **حدود الموارد**: سقوف محددة لوقت الـ CPU، الذاكرة، وعدد التعليمات لكل إضافة
- **تنظيم وتيرة API**: كيحترم ترويسات `Retry-After` ومهل الانتظار القابلة للتخصيص
- **التحقق من التعديلات**: معاينة التغييرات قبل التطبيق مع سجلات التدقيق
- **دعم OAuth**: الطريقة المفضلة للمصادقة بالنسبة للبوتات

## الترخيص

هاد المشروع مرخّص تحت واحد من هاد جوج تراخيص:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) ولا http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../../../LICENSE-MIT) ولا http://opensource.org/licenses/MIT)

على حسب الاختيار ديالك.

## المساهمة

المساهمات مرحب بها! عافاك طالع [CONTRIBUTING.md](../../../CONTRIBUTING.md) باش تشوف القواعد ديال المساهمة.

## وضعية المشروع

هاد المشروع هو إعادة كتابة نشيطة لـ AutoWikiBrowser بلغة Rust. الوضعية الحالية:

- [x] عميل API الأساسي
- [x] محرك القواعد
- [x] واجهة CLI
- [x] نظام الإضافات (Lua/WASM)
- [x] بيئة العزل والأمان
- [ ] واجهة الطرفية (قيد الإنجاز)
- [ ] الواجهة الرسومية GUI (مبرمجة)
- [ ] عرض متقدم للفروقات (مبرمج)

## روابط

- [التوثيق](https://docs.rs/awb-rs)
- [متتبع المشاكل](https://github.com/somethingwithproof/awb-rs/issues)
- [أداة AWB الأصلية](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
