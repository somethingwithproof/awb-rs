# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

بازنویسی مدرن و پرسرعت AutoWikiBrowser (AWB) به زبان Rust که قابلیت‌های ویرایش خودکار ویکی‌پدیا را با تمرکز بر امنیت، کارایی و توسعه‌پذیری فراهم می‌کند.

## معماری

```
awb-rs/
├── awb-core/           # منطق اصلی برنامه و انتزاع‌ها
├── awb-api/            # پیاده‌سازی کلاینت رابط برنامه‌نویسی MediaWiki
├── awb-cli/            # رابط خط فرمان
├── awb-rules/          # موتور قواعد برای خودکارسازی ویرایش
├── awb-lua-plugin/     # محیط اجرای افزونه‌های Lua
├── awb-wasm-plugin/    # محیط اجرای افزونه‌های WebAssembly
├── awb-security/       # امنیت و ایزوله‌سازی در جعبه شنی
├── awb-db/             # مدیریت وضعیت ماندگار
├── awb-ui/             # اجزای رابط کاربری ترمینال
├── awb-fixtures/       # داده‌های آزمون و نمونه‌های شبیه‌سازی‌شده
└── awb-integration/    # مجموعه آزمون‌های یکپارچه‌سازی
```

**جریان داده:**
```
CLI → هسته → API → MediaWiki
       ↓
    موتور قواعد → افزونه‌ها (Lua/WASM)
       ↓
    لایه امنیت → اجرای ایزوله
       ↓
    پایگاه داده → ماندگاری وضعیت
```

## ویژگی‌ها

- **معماری مدرن**: برنامه‌نویسی ناهمگام با زمان اجرا Tokio برای دستیابی به کارایی بالا
- **پشتیبانی از چند ویکی**: سازگار با هر ویکی مبتنی بر MediaWiki
- **ویرایش مبتنی بر قواعد**: تعریف اعلانی قواعد با قالب TOML
- **سامانه افزونه‌ها**: قابل گسترش از طریق اسکریپت‌های Lua و ماژول‌های WebAssembly
- **امنیت تقویت‌شده**: اجرای افزونه‌ها در محیط ایزوله با محدودیت‌های منابع
- **عملیات دسته‌ای**: پردازش کارآمد و انبوه صفحه‌ها
- **رابط ترمینال تعاملی**: رابط ترمینال غنی برای پایش و کنترل
- **حالت آزمایشی**: پیش‌نمایش تغییرات پیش از اعمال واقعی آن‌ها
- **مدیریت وضعیت**: امکان ادامه نشست‌های ناتمام
- **محدودسازی نرخ درخواست**: رعایت محدودیت‌های رابط برنامه‌نویسی ویکی و بهترین رویه‌ها
- **بین‌المللی‌سازی**: پشتیبانی از ۴۵ زبان نمایشی از طریق `--lang`، `AWB_LANG` یا تنظیمات محلی POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`)، از جمله پشتیبانی از چیدمان راست‌به‌چپ برای زبان‌های عربی، فارسی، عبری و اردو

## زبان‌های پشتیبانی‌شده

رابط خط فرمان، گزارش‌های ربات و رابط کاربری GTK4 از ۴۵ زبان پشتیبانی می‌کنند (از جمله ۶ زبان راست‌به‌چپ: `ar`، `ar-EG`، `ar-MA`، `fa`، `he` و `ur`). برای بازنویسی تشخیص خودکار زبان سیستم، گزینه `--lang <LANG>` را ارسال کنید یا متغیر `AWB_LANG` را تنظیم نمایید. برای مشاهده جدول کامل زبان‌ها به [README.md](../../../README.md) مراجعه کنید.

## پیش‌نیازها

- **Rust**: نسخه ۱.۸۵ یا بالاتر (حداقل نسخه پشتیبانی‌شده)
- **Cargo**: همراه با نصب Rust ارائه می‌شود

## نصب

### ساخت از کد منبع

```bash
# کلون کردن مخزن
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# ساخت تمام بسته‌های فضای کاری
cargo build --workspace --release

# نصب فایل اجرایی
cargo install --path awb-cli
```

## توسعه

### ساخت

```bash
# ساخت اشکال‌زدایی
cargo build --workspace

# ساخت انتشار
cargo build --workspace --release
```

### آزمون

```bash
# اجرای همه آزمون‌ها
cargo test --workspace

# اجرای آزمون‌ها همراه با نمایش خروجی
cargo test --workspace -- --nocapture
```

### تحلیل ایستا

```bash
# بررسی خطاها و هشدارهای سبک کدنویسی
cargo clippy --workspace --all-targets --all-features

# اعمال اصلاحات خودکار
cargo clippy --fix --allow-dirty --workspace
```

### قالب‌بندی

```bash
# بررسی قالب‌بندی کد
cargo fmt --all -- --check

# اعمال قالب‌بندی کد
cargo fmt --all
```

## نمونه‌های کاربردی

### احراز هویت

```bash
# ورود به ویکی (ذخیره ایمن اطلاعات ورود)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# ورود با استفاده از OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### فهرست‌کردن صفحه‌ها

```bash
# دریافت فهرست صفحه‌ها از یک رده
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# دریافت فهرست صفحه‌ها از نتایج جستجو
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# دریافت فهرست صفحه‌ها از یک فایل
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### اجرای قواعد ویرایش

```bash
# اجرای قواعد با یک نمایه (حالت آزمایشی)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# اعمال قواعد روی صفحه‌های واقعی
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### حالت ربات خودکار

```bash
# اجرا در حالت ربات خودکار با محدودیت‌ها
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# ادامه از نشست قبلی
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### نمونه نمایه قواعد

فایلی با نام `my-rules.toml` ایجاد کنید:

```toml
[profile]
name = "ربات پاک‌سازی"
description = "اصلاح خطاهای رایج قالب‌بندی"
edit_summary = "پاک‌سازی خودکار (ربات)"

[[rules]]
name = "اصلاح فاصله‌های تکراری"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "اصلاح پیوندهای بیرونی"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "افزودن رده‌های جاافتاده"
plugin = "add_categories.lua"
```

## سامانه افزونه‌ها

awb-rs از دو نوع افزونه پشتیبانی می‌کند:

### افزونه‌های Lua

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### افزونه‌های WebAssembly

```rust
// کامپایل به WASM با دستور: cargo build --target wasm32-wasi
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**ویژگی‌های افزونه‌ها:**
- اجرای ایزوله همراه با محدودیت منابع (حافظه و زمان پردازنده)
- دسترسی محدود به زیرمجموعه‌ای امن از رابط‌های برنامه‌نویسی (بدون دسترسی به سیستم فایل یا شبکه)
- بررسی سازگاری نسخه‌ها
- پشتیبانی از بارگذاری مجدد آنی در حالت توسعه

## مدل امنیتی

- **ذخیره‌سازی اطلاعات ورود**: استفاده از کلیددان سیستم‌عامل برای نگهداری امن گذرواژه‌ها
- **ایزوله‌سازی افزونه‌ها**: اجرای Lua و WASM در محیط‌های کاملاً مجزا
- **محدودیت منابع**: اعمال سقف برای زمان پردازنده، حافظه و تعداد دستورالعمل‌ها در هر افزونه
- **محدودسازی نرخ درخواست**: رعایت سرآیندهای `Retry-After` و تأخیرهای قابل پیکربندی
- **اعتبارسنجی ویرایش‌ها**: پیش‌نمایش تغییرات پیش از اعمال و ثبت گزارش‌های ممیزی
- **پشتیبانی از OAuth**: روش احراز هویت پیشنهادی برای ربات‌ها

## مجوز

به انتخاب شما تحت یکی از دو مجوز زیر منتشر شده است:

- مجوز آپاچی، نسخه ۲.۰ ([LICENSE-APACHE](../../../LICENSE-APACHE) یا http://www.apache.org/licenses/LICENSE-2.0)
- مجوز MIT ([LICENSE-MIT](../../../LICENSE-MIT) یا http://opensource.org/licenses/MIT)

## مشارکت

از مشارکت شما استقبال می‌کنیم! لطفاً برای مطالعه راهنما به [CONTRIBUTING.md](../../../CONTRIBUTING.md) مراجعه کنید.

## وضعیت پروژه

این پروژه بازنویسی فعال AutoWikiBrowser به زبان Rust است. وضعیت کنونی:

- [x] کلاینت هسته‌ای رابط برنامه‌نویسی
- [x] موتور قواعد
- [x] رابط خط فرمان
- [x] سامانه افزونه‌ها (Lua/WASM)
- [x] ایزوله‌سازی امنیتی
- [ ] رابط کاربری ترمینال (در حال انجام)
- [ ] رابط گرافیکی (در برنامه)
- [ ] نمایش پیشرفته تفاوت‌ها (در برنامه)

## پیوندها

- [مستندات](https://docs.rs/awb-rs)
- [ردیاب خطاها و درخواست‌ها](https://github.com/somethingwithproof/awb-rs/issues)
- [نسخه اصلی AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
