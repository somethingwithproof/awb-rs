# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

إعادة بناء حديثة وبأداء عالي لأداة AutoWikiBrowser (AWB) بلغة Rust، بتقدم إمكانيات تعديل تلقائي لويكيبيديا مع تركيز كبير على الأمان، السرعة، وسهولة التوسع.

## البنية المعمارية

```
awb-rs/
├── awb-core/           # المنطق الأساسي والتجريدات البرمجية
├── awb-api/            # تطبيق عميل MediaWiki API
├── awb-cli/            # واجهة سطر الأوامر
├── awb-rules/          # محرك القواعد لأتمتة التعديلات
├── awb-lua-plugin/     # بيئة تشغيل إضافات Lua
├── awb-wasm-plugin/    # بيئة تشغيل إضافات WebAssembly
├── awb-security/       # الأمان والعزل في بيئة محمية
├── awb-db/             # إدارة وحفظ الحالة الدائمة
├── awb-ui/             # مكونات واجهة الطرفية
├── awb-fixtures/       # بيانات الاختبار والنماذج التجريبية
└── awb-integration/    # مجموعة اختبارات التكامل
```

**مسار البيانات:**
```
CLI → النواة الأساسية → API → MediaWiki
           ↓
      محرك القواعد → الإضافات (Lua/WASM)
           ↓
      طبقة الأمان → تشغيل معزول وآمن
           ↓
      قاعدة البيانات → حفظ الحالة
```

## المميزات

- **بنية حديثة**: شغالة بنمط غير متزامن (`async/await`) مع بيئة Tokio علشان تدّي أعلى أداء
- **دعم أكتر من ويكي**: تقدر تشتغل مع أي ويكي شغال بنظام MediaWiki
- **تعديل مبني على القواعد**: تعريف القواعد بشكل واضح ومباشر بملفات TOML
- **نظام إضافات مرن**: قابل للتوسيع عن طريق سكربتات Lua ووحدات WebAssembly
- **أمان عالي**: تشغيل الإضافات جوه بيئة معزولة بحدود واضحة للموارد
- **عمليات جماعية**: معالجة عدد كبير من الصفحات بكفاءة وسرعة
- **واجهة طرفية تفاعلية (TUI)**: واجهة غنية للمتابعة والتحكم لحظة بلحظة
- **وضع التجربة (Dry-Run)**: تقدر تشوف معاينة التعديلات قبل ما تتطبق فعليًا
- **إدارة الحالة**: إمكانية استكمال الجلسات اللي اتقطعت من مكان ما وقفت
- **التحكم في معدل الطلبات**: بيلتزم بحدود API الخاصة بالويكي وأفضل الممارسات
- **دعم تعدد اللغات**: 45 لغة عرض عن طريق `--lang` أو `AWB_LANG` أو إعدادات POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`)، بما فيها دعم اتجاه اليمين لليسار (RTL) للعربي والفارسي والعبري والأردو

## اللغات المدعومة

واجهة سطر الأوامر (CLI) وتقارير البوت وواجهة GTK4 بتدعم 45 لغة (منهم 6 لغات RTL: `ar`، `ar-EG`، `ar-MA`، `fa`، `he`، و`ur`). استخدم `--lang <LANG>` في واجهة سطر الأوامر (CLI)، أو اضبط `AWB_LANG` أو متغيرات POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`) لواجهة سطر الأوامر وواجهة GTK4 مع بعض. شوف الجدول الكامل للغات في [README.md](../../../README.md).

## المتطلبات الأساسية

- **Rust**: إصدار 1.94 أو أحدث (MSRV)
- **Cargo**: بيجي متثبت تلقائيًا مع Rust

## التثبيت

### التثبيت من الكود المصدري

```bash
# انسخ المستودع على جهازك
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# ابني كل الحزم الموجودة في مساحة العمل
cargo build --workspace --release

# ثبّت البرنامج التنفيذي
cargo install --path crates/awb_cli
```

## التطوير

### البناء

```bash
# بناء لوضع التصحيح
cargo build --workspace

# بناء لوضع الإصدار النهائي
cargo build --workspace --release
```

### الاختبارات

```bash
# شغّل كل الاختبارات
cargo test --workspace

# شغّل الاختبارات مع إظهار كل المخرجات
cargo test --workspace -- --nocapture
```

### فحص جودة الكود

```bash
# افحص الكود وتأكد لو فيه أي تحذيرات أو ملاحظات
cargo clippy --workspace --all-targets --all-features

# طبّق التصليحات التلقائية
cargo clippy --fix --allow-dirty --workspace
```

### التنسيق

```bash
# اتأكد من تنسيق الكود
cargo fmt --all -- --check

# طبّق تنسيق الكود تلقائيًا
cargo fmt --all
```

## أمثلة للاستخدام

### تسجيل الدخول والمصادقة

```bash
# سجّل دخول على ويكي (بيحفظ بيانات الدخول بشكل آمن)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# سجّل دخول باستخدام OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### عرض قوائم الصفحات

```bash
# هات قايمة الصفحات من تصنيف معين
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# هات قايمة الصفحات من نتايج بحث
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# هات قايمة الصفحات من ملف نصي
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### تشغيل قواعد التعديل

```bash
# شغّل القواعد بملف تعريف معين في وضع التجربة
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# طبّق القواعد على صفحات حقيقية
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### وضع البوت التلقائي

```bash
# شغّل في وضع البوت التلقائي مع تحديد حدود واضحة
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# كمّل شغل من جلسة قديمة
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### مثال لملف قواعد

اعمل ملف اسمه `my-rules.toml`:

```toml
[profile]
name = "بوت التنظيم والتنسيق"
description = "تصليح مشاكل التنسيق الشائعة"
edit_summary = "تنظيم وتنسيق تلقائي (بوت)"

[[rules]]
name = "تصليح المسافات المزدوجة"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "تحديث الروابط الخارجية"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "إضافة التصنيفات الناقصة"
plugin = "add_categories.lua"
```

## نظام الإضافات

أداة awb-rs بتدعم نوعين من الإضافات:

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
// حوّل الكود لـ WASM بالأمر ده: cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**مميزات الإضافات:**
- تشغيل معزول مع حدود واضحة للموارد (الذاكرة ووقت الـ CPU)
- صلاحية وصول لجزء آمن فقط من الـ API (مفيش وصول لنظام الملفات أو الشبكة)
- فحص توافق الإصدارات تلقائيًا
- دعم إعادة التحميل الفوري أثناء وضع التطوير

## نموذج الأمان

- **حفظ بيانات الدخول**: بيستخدم مخزن المفاتيح الخاص بنظام التشغيل لحفظ كلمات السر بأمان
- **عزل الإضافات**: إضافات Lua وWASM بتشتغل في بيئات معزولة تمامًا
- **حدود الموارد**: قيود محددة لوقت الـ CPU، الذاكرة، وعدد التعليمات لكل إضافة
- **تنظيم معدل طلبات API**: بيلتزم بترويسات `Retry-After` وفترات انتظار قابلة للتعديل
- **التحقق من التعديلات**: معاينة التغييرات قبل الحفظ مع سجلات مراجعة كاملة
- **دعم OAuth**: الطريقة المفضلة والأكثر أمانًا لتسجيل دخول البوتات

## الترخيص

المشروع مرخّص تحت أي واحد من الترخيصين دول:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) أو http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../../../LICENSE-MIT) أو http://opensource.org/licenses/MIT)

على حسب اختيارك.

## المساهمة

نرحب جدًا بأي مساهمة! ياريت تراجع [CONTRIBUTING.md](../../../CONTRIBUTING.md) علشان تعرف إرشادات المساهمة.

## حالة المشروع

المشروع ده إعادة كتابة نشطة لأداة AutoWikiBrowser بلغة Rust. الوضع الحالي:

- [x] عميل API الأساسي
- [x] محرك القواعد
- [x] واجهة سطر الأوامر (CLI)
- [x] نظام الإضافات (Lua/WASM)
- [x] بيئة العزل والأمان
- [ ] واجهة الطرفية (شغالين عليها حاليًا)
- [ ] الواجهة الرسومية (GUI) (في الخطة)
- [ ] عرض متقدم للفروقات بين التعديلات (في الخطة)

## روابط مفيدة

- [التوثيق](https://docs.rs/awb-rs)
- [متابعة المشاكل والأخطاء](https://github.com/somethingwithproof/awb-rs/issues)
- [أداة AWB الأصلية](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
