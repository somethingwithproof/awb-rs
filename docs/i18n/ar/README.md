# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

إعادة كتابة حديثة وعالية الأداء لبرنامج AutoWikiBrowser (AWB) بلغة Rust، توفّر إمكانات التحرير الآلي لموسوعة ويكيبيديا مع التركيز على الأمان والأداء والقابلية للتوسّع.

## البنية المعمارية

```
awb-rs/
├── awb-core/           # منطق الأعمال الأساسي والتجريدات
├── awb-api/            # تنفيذ عميل واجهة برمجة تطبيقات MediaWiki
├── awb-cli/            # واجهة سطر الأوامر
├── awb-rules/          # محرّك القواعد لأتمتة التحرير
├── awb-lua-plugin/     # بيئة تشغيل إضافات Lua
├── awb-wasm-plugin/    # بيئة تشغيل إضافات WebAssembly
├── awb-security/       # الأمان والعزل في بيئة معزولة
├── awb-db/             # إدارة الحالة الدائمة
├── awb-ui/             # مكوّنات واجهة الطرفية
├── awb-fixtures/       # بيانات الاختبار والنماذج الوهمية
└── awb-integration/    # مجموعة اختبارات التكامل
```

**تدفّق البيانات:**
```
CLI → النواة → API → MediaWiki
       ↓
    محرّك القواعد → الإضافات (Lua/WASM)
       ↓
    طبقة الأمان → التنفيذ المعزول
       ↓
    قاعدة البيانات → حفظ الحالة الدائمة
```

## الميزات

- **بنية معمارية حديثة**: تنفيذ غير متزامن عبر async/await باستخدام بيئة التشغيل Tokio لتحقيق أداء عالٍ
- **دعم مواقع ويكي المتعددة**: يعمل مع أي موقع ويكي يعتمد على MediaWiki
- **تحرير مبني على القواعد**: تعريفات تصريحية للقواعد بصيغة TOML
- **نظام الإضافات**: قابل للتوسّع عبر نصوص Lua البرمجية ووحدات WebAssembly
- **أمان معزّز**: تنفيذ الإضافات داخل بيئة معزولة مع حدود صارمة للموارد
- **العمليات الدفعية**: معالجة جماعية فعّالة لعدد كبير من الصفحات
- **واجهة طرفية تفاعلية**: واجهة طرفية غنية للمراقبة والتحكّم
- **وضع التشغيل التجريبي**: معاينة التغييرات قبل تطبيقها الفعلي
- **إدارة الحالة**: استئناف الجلسات التي انقطعت
- **تحديد المعدّل**: احترام حدود واجهة برمجة التطبيقات الخاصة بالويكي وأفضل الممارسات
- **التدويل**: دعم 45 لغة عرض عبر `--lang` أو `AWB_LANG` أو إعدادات اللغة POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`)، بما في ذلك دعم التخطيط من اليمين إلى اليسار للغات العربية والفارسية والعبرية والأردية

## اللغات المدعومة

تدعم واجهة سطر الأوامر وتقارير البوت وواجهة GTK4 الرسومية 45 لغة (بما في ذلك 6 لغات من اليمين إلى اليسار: `ar` و`ar-EG` و`ar-MA` و`fa` و`he` و`ur`). مرّر الخيار `--lang <LANG>` في واجهة سطر الأوامر، أو اضبط `AWB_LANG` أو متغيرات اللغة POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`) لكلٍّ من واجهة سطر الأوامر وواجهة GTK4 الرسومية. للاطلاع على الجدول الكامل للغات، يُرجى الرجوع إلى [README.md](../../../README.md).

## المتطلبات الأساسية

- **Rust**: الإصدار 1.94 أو أحدث (الحد الأدنى المدعوم لإصدار Rust)
- **Cargo**: مُضمّن تلقائيًا مع تثبيت Rust

## التثبيت

### البناء من الشيفرة المصدرية

```bash
# استنساخ المستودع
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# بناء جميع الحزم في مساحة العمل
cargo build --workspace --release

# تثبيت الملف التنفيذي
cargo install --path crates/awb_cli
```

## التطوير

### البناء

```bash
# بناء وضع التصحيح
cargo build --workspace

# بناء وضع الإصدار
cargo build --workspace --release
```

### الاختبار

```bash
# تشغيل جميع الاختبارات
cargo test --workspace

# تشغيل الاختبارات مع إظهار المخرجات
cargo test --workspace -- --nocapture
```

### التحليل الثابت

```bash
# فحص مشكلات جودة الشيفرة
cargo clippy --workspace --all-targets --all-features

# تطبيق الإصلاحات التلقائية
cargo clippy --fix --allow-dirty --workspace
```

### التنسيق

```bash
# التحقق من تنسيق الشيفرة
cargo fmt --all -- --check

# تطبيق تنسيق الشيفرة
cargo fmt --all
```

## أمثلة الاستخدام

### المصادقة

```bash
# تسجيل الدخول إلى ويكي (تخزين بيانات الاعتماد بشكل آمن)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# تسجيل الدخول باستخدام OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### سرد الصفحات

```bash
# سرد الصفحات من تصنيف
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# سرد الصفحات من نتائج البحث
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# سرد الصفحات من ملف
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### تشغيل قواعد التحرير

```bash
# تشغيل القواعد باستخدام ملف تعريف (وضع التشغيل التجريبي)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# تطبيق القواعد على الصفحات الفعلية
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### وضع البوت الآلي

```bash
# التشغيل في وضع البوت غير المراقب مع قيود
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# الاستئناف من جلسة سابقة
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### مثال على ملف تعريف القواعد

أنشئ ملفًا باسم `my-rules.toml`:

```toml
[profile]
name = "بوت التنظيف"
description = "إصلاح مشكلات التنسيق الشائعة"
edit_summary = "تنظيف آلي (بوت)"

[[rules]]
name = "إصلاح المسافات المزدوجة"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "إصلاح الروابط الخارجية"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "إضافة التصنيفات المفقودة"
plugin = "add_categories.lua"
```

## نظام الإضافات

يدعم awb-rs نوعين من الإضافات:

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
// التحويل البرمجي إلى WASM عبر: cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**ميزات الإضافات:**
- تنفيذ معزول مع حدود للموارد (الذاكرة ووقت المعالج)
- وصول إلى مجموعة فرعية آمنة من واجهات البرمجة (دون وصول إلى نظام الملفات أو الشبكة)
- التحقق من توافق الإصدارات
- دعم إعادة التحميل الفوري في وضع التطوير

## نموذج الأمان

- **تخزين بيانات الاعتماد**: يستخدم سلسلة مفاتيح نظام التشغيل لتخزين كلمات المرور بأمان
- **عزل الإضافات**: تعمل إضافات Lua وWASM في بيئات معزولة تمامًا
- **حدود الموارد**: قيود على وقت المعالج والذاكرة وعدد التعليمات لكل إضافة
- **تحديد معدّل واجهة البرمجة**: احترام ترويسات `Retry-After` والفترات الزمنية القابلة للتهيئة
- **التحقق من التعديلات**: معاينة التغييرات قبل تطبيقها مع سجلات تدقيق
- **دعم OAuth**: طريقة المصادقة المفضّلة للبوتات

## الترخيص

مرخّص وفقًا لأي من الرخصتين التاليتين حسب اختيارك:

- رخصة أباتشي، الإصدار 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) أو http://www.apache.org/licenses/LICENSE-2.0)
- رخصة MIT ([LICENSE-MIT](../../../LICENSE-MIT) أو http://opensource.org/licenses/MIT)

## المساهمة

المساهمات مرحّب بها دائمًا! يُرجى الاطلاع على [CONTRIBUTING.md](../../../CONTRIBUTING.md) للتعرّف على الإرشادات.

## حالة المشروع

هذا المشروع عبارة عن إعادة كتابة نشطة لبرنامج AutoWikiBrowser بلغة Rust. الحالة الحالية:

- [x] عميل واجهة برمجة التطبيقات الأساسي
- [x] محرّك القواعد
- [x] واجهة سطر الأوامر
- [x] نظام الإضافات (Lua/WASM)
- [x] العزل الأمني
- [ ] واجهة الطرفية (قيد التنفيذ)
- [ ] الواجهة الرسومية (مخطّط لها)
- [ ] عرض متقدّم للفروقات (مخطّط له)

## الروابط

- [التوثيق](https://docs.rs/awb-rs)
- [تتبّع المشكلات](https://github.com/somethingwithproof/awb-rs/issues)
- [برنامج AWB الأصلي](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
