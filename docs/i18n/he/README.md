# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

שכתוב מודרני ועתיר-ביצועים של AutoWikiBrowser (AWB) בשפת Rust, המספק יכולות עריכה אוטומטית בוויקיפדיה תוך דגש על בטיחות, ביצועים ויכולת הרחבה.

## ארכיטקטורה

```
awb-rs/
├── awb-core/           # לוגיקה עסקית מרכזית והפשטות
├── awb-api/            # מימוש לקוח לממשק MediaWiki API
├── awb-cli/            # ממשק שורת פקודה
├── awb-rules/          # מנוע כללים לאוטומציה של עריכות
├── awb-lua-plugin/     # סביבת ריצה לתוספי Lua
├── awb-wasm-plugin/    # סביבת ריצה לתוספי WebAssembly
├── awb-security/       # אבטחה ובידוד בארגז חול
├── awb-db/             # ניהול מצב מתמיד
├── awb-ui/             # רכיבי ממשק מסוף
├── awb-fixtures/       # נתוני בדיקה ונתוני דמה
└── awb-integration/    # חבילת בדיקות אינטגרציה
```

**זרימת נתונים:**
```
CLI → ליבה → API → MediaWiki
       ↓
    מנוע כללים → תוספים (Lua/WASM)
       ↓
    שכבת אבטחה → הרצה מבודדת
       ↓
    מסד נתונים → שמירת מצב מתמיד
```

## תכונות

- **ארכיטקטורה מודרנית**: תכנות אסינכרוני עם סביבת הריצה Tokio לביצועים גבוהים
- **תמיכה בריבוי אתרי ויקי**: פועל מול כל אתר ויקי המבוסס על MediaWiki
- **עריכה מבוססת כללים**: הגדרות כללים הצהרתיות בתבנית TOML
- **מערכת תוספים**: הרחבה באמצעות תסריטי Lua ומודולי WebAssembly
- **אבטחה מוקשחת**: הרצת תוספים בארגז חול מבודד עם הגבלות משאבים
- **פעולות אצווה**: עיבוד יעיל של דפים רבים בבת אחת
- **ממשק מסוף אינטראקטיבי**: ממשק מסוף עשיר לניטור ולשליטה
- **מצב הרצת ניסיון**: תצוגה מקדימה של שינויים לפני החלתם בפועל
- **ניהול מצב**: המשכת הפעלות שנקטעו
- **הגבלת קצב**: כיבוד מגבלות ממשק האתר ונהלי העבודה המומלצים
- **בינאום**: תמיכה ב-45 שפות תצוגה באמצעות `--lang`, `AWB_LANG` או הגדרות אזור POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`), כולל תמיכה בפריסה מימין לשמאל עבור ערבית, פרסית, עברית ואורדו

## שפות נתמכות

ממשק שורת הפקודה, דוחות הבוט וממשק GTK4 תומכים ב-45 שפות (כולל 6 שפות מימין לשמאל: `ar`, `ar-EG`, `ar-MA`, `fa`, `he` ו-`ur`). השתמשו ב-`--lang <LANG>` ב-CLI, או הגדירו את `AWB_LANG` או משתני POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`) גם עבור ה-CLI וגם עבור ממשק GTK4. לטבלת השפות המלאה ראו [README.md](../../../README.md).

## דרישות מוקדמות

- **Rust**: גרסה 1.94 ומעלה (גרסה נתמכת מזערית)
- **Cargo**: כלול בהתקנת Rust

## התקנה

### בנייה מקוד המקור

```bash
# שכפול המאגר
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# בניית כל החבילות בסביבת העבודה
cargo build --workspace --release

# התקנת קובץ ההרצה
cargo install --path crates/awb_cli
```

## פיתוח

### בנייה

```bash
# בניית ניפוי שגיאות
cargo build --workspace

# בניית הפצה
cargo build --workspace --release
```

### בדיקות

```bash
# הרצת כל הבדיקות
cargo test --workspace

# הרצת בדיקות עם הצגת פלט
cargo test --workspace -- --nocapture
```

### ניתוח קוד סטטי

```bash
# בדיקת התראות איכות קוד
cargo clippy --workspace --all-targets --all-features

# החלת תיקונים אוטומטיים
cargo clippy --fix --allow-dirty --workspace
```

### עיצוב קוד

```bash
# בדיקת עיצוב הקוד
cargo fmt --all -- --check

# החלת עיצוב הקוד
cargo fmt --all
```

## דוגמאות שימוש

### אימות

```bash
# התחברות לוויקי (שמירת פרטי הגישה באופן מאובטח)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# התחברות באמצעות OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### הצגת רשימת דפים

```bash
# קבלת רשימת דפים מתוך קטגוריה
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# קבלת רשימת דפים מתוך חיפוש
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# קבלת רשימת דפים מתוך קובץ
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### הרצת כללי עריכה

```bash
# הרצת כללים באמצעות פרופיל (מצב הרצת ניסיון)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# החלת כללים על דפים אמיתיים
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### מצב בוט אוטומטי

```bash
# הרצה במצב בוט אוטומטי עם מגבלות
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# המשך מהפעלה קודמת
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### דוגמה לפרופיל כללים

צרו קובץ בשם `my-rules.toml`:

```toml
[profile]
name = "בוט ניקיון"
description = "תיקון בעיות עיצוב נפוצות"
edit_summary = "ניקיון אוטומטי (בוט)"

[[rules]]
name = "תיקון רווחים כפולים"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "תיקון קישורים חיצוניים"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "הוספת קטגוריות חסרות"
plugin = "add_categories.lua"
```

## מערכת תוספים

awb-rs תומך בשני סוגי תוספים:

### תוספי Lua

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### תוספי WebAssembly

```rust
// הידור ל-WASM באמצעות: cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**תכונות התוספים:**
- הרצה מבודדת בארגז חול עם הגבלת משאבים (זיכרון, זמן מעבד)
- גישה לתת-קבוצה בטוחה של ממשקים בלבד (ללא גישה למערכת הקבצים או לרשת)
- בדיקת תאימות גרסאות
- תמיכה בטעינה מחדש חמה במצב פיתוח

## מודל האבטחה

- **אחסון פרטי גישה**: שימוש בצרור המפתחות של מערכת ההפעלה לשמירה מאובטחת של סיסמאות
- **בידוד תוספים**: Lua ו-WASM פועלים בסביבות מבודדות
- **הגבלות משאבים**: מגבלות זמן מעבד, זיכרון ומספר הוראות לכל תוסף
- **הגבלת קצב ממשק**: כיבוד כותרות `Retry-After` והשהיות הניתנות להגדרה
- **אימות עריכות**: תצוגה מקדימה של שינויים לפני החלה ויומני ביקורת
- **תמיכה ב-OAuth**: שיטת האימות המועדפת עבור בוטים

## רישיון

הפרויקט מורשה תחת אחד משני הרישיונות הבאים, לפי בחירתכם:

- רישיון Apache, גרסה 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) או http://www.apache.org/licenses/LICENSE-2.0)
- רישיון MIT ([LICENSE-MIT](../../../LICENSE-MIT) או http://opensource.org/licenses/MIT)

## תרומה לפרויקט

תרומות מתקבלות בברכה! אנא עיינו בקובץ [CONTRIBUTING.md](../../../CONTRIBUTING.md) לקבלת הנחיות.

## מצב הפרויקט

זהו שכתוב פעיל של AutoWikiBrowser בשפת Rust. המצב הנוכחי:

- [x] לקוח ממשק ליבה
- [x] מנוע כללים
- [x] ממשק שורת פקודה
- [x] מערכת תוספים (Lua/WASM)
- [x] בידוד אבטחה
- [ ] ממשק מסוף (בעבודה)
- [ ] ממשק גרפי (מתוכנן)
- [ ] תצוגת השוואת שינויים מתקדמת (מתוכנן)

## קישורים

- [תיעוד](https://docs.rs/awb-rs)
- [מעקב תקלות](https://github.com/somethingwithproof/awb-rs/issues)
- [AWB המקורי](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
