# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

การพัฒนา AutoWikiBrowser (AWB) ขึ้นใหม่ในภาษา Rust ที่ทันสมัยและมีประสิทธิภาพสูง พร้อมความสามารถในการแก้ไขวิกิพีเดียแบบอัตโนมัติ โดยมุ่งเน้นด้านความปลอดภัย ประสิทธิภาพ และความสามารถในการต่อขยาย

## สถาปัตยกรรม

```
awb-rs/
├── awb-core/           # ตรรกะหลักและนามธรรมของระบบ
├── awb-api/            # ไคลเอนต์สำหรับเชื่อมต่อ MediaWiki API
├── awb-cli/            # ส่วนติดต่อผู้ใช้แบบบรรทัดคำสั่ง
├── awb-rules/          # เอ็นจินกฎสำหรับการแก้ไขอัตโนมัติ
├── awb-lua-plugin/     # รันไทม์สำหรับปลั๊กอิน Lua
├── awb-wasm-plugin/    # รันไทม์สำหรับปลั๊กอิน WebAssembly
├── awb-security/       # ระบบความปลอดภัยและการแยกสภาพแวดล้อม
├── awb-db/             # การจัดการสถานะแบบถาวร
├── awb-ui/             # ส่วนประกอบส่วนติดต่อผู้ใช้แบบเทอร์มินัล
├── awb-fixtures/       # ชุดข้อมูลจำลองและฟิกซ์เจอร์สำหรับการทดสอบ
└── awb-integration/    # ชุดทดสอบการบูรณาการระบบ
```

**การไหลของข้อมูล:**
```
CLI → แกนหลัก → API → MediaWiki
         ↓
      เอ็นจินกฎ → ปลั๊กอิน (Lua/WASM)
         ↓
      ชั้นความปลอดภัย → การประมวลผลในสภาพแวดล้อมแยก
         ↓
      ฐานข้อมูล → การบันทึกสถานะถาวร
```

## คุณสมบัติ

- **สถาปัตยกรรมที่ทันสมัย**: ใช้การทำงานแบบอะซิงโครนัส (`async/await`) ร่วมกับรันไทม์ Tokio เพื่อประสิทธิภาพสูงสุด
- **รองรับหลายวิกิ**: ใช้งานร่วมกับเว็บไซต์วิกิใดก็ได้ที่ขับเคลื่อนด้วย MediaWiki
- **การแก้ไขตามกฎ**: กำหนดกฎแบบประกาศในรูปแบบ TOML
- **ระบบปลั๊กอิน**: ต่อขยายการทำงานได้ด้วยสคริปต์ Lua และโมดูล WebAssembly
- **ความปลอดภัยระดับสูง**: ประมวลผลปลั๊กอินในสภาพแวดล้อมแยกพร้อมจำกัดการใช้ทรัพยากร
- **การประมวลผลแบบกลุ่ม**: จัดการหน้าจำนวนมากได้อย่างมีประสิทธิภาพ
- **TUI แบบโต้ตอบ**: ส่วนติดต่อแบบเทอร์มินัลที่ครบครันสำหรับการติดตามและควบคุม
- **โหมดทดลองทำงาน (ดรายรัน)**: แสดงตัวอย่างการเปลี่ยนแปลงก่อนบันทึกจริง
- **การจัดการสถานะ**: สามารถดำเนินงานต่อจากเซสชันที่หยุดชะงักได้
- **การจำกัดอัตราการเรียกใช้**: ปฏิบัติตามข้อจำกัดของวิกิ API และแนวทางปฏิบัติที่ดีที่สุด
- **รองรับหลายภาษา**: รองรับภาษาแสดงผล 45 ภาษา ผ่าน `--lang`, `AWB_LANG` หรือค่าโลแคล POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`) รวมถึงการจัดวางรูปแบบ RTL สำหรับภาษาอาหรับ เปอร์เซีย ฮีบรู และอูรดู

## ภาษาที่รองรับ

CLI รายงานของบอต และส่วนติดต่อผู้ใช้ GTK4 รองรับทั้งหมด 45 ภาษา (รวมถึงโลแคล RTL 6 ภาษา ได้แก่ `ar`, `ar-EG`, `ar-MA`, `fa`, `he` และ `ur`) ใช้ `--lang <LANG>` ใน CLI หรือตั้งค่า `AWB_LANG` หรือตัวแปรโลแคล POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`) สำหรับทั้ง CLI และ GTK4 UI ดูตารางภาษาทั้งหมดได้ที่ [README.md](../../../README.md)

## ข้อกำหนดเบื้องต้น

- **Rust**: เวอร์ชัน 1.94 ขึ้นไป (MSRV)
- **Cargo**: ติดตั้งมาพร้อมกับชุดเครื่องมือ Rust

## การติดตั้ง

### ติดตั้งจากซอร์สโค้ด

```bash
# โคลนคลังซอร์สโค้ด
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# คอมไพล์เครตทั้งหมดในเวิร์กสเปซ
cargo build --workspace --release

# ติดตั้งไฟล์ไบนารี
cargo install --path crates/awb_cli
```

## การพัฒนา

### การคอมไพล์

```bash
# คอมไพล์โหมดดีบัก
cargo build --workspace

# คอมไพล์โหมดรีลีส
cargo build --workspace --release
```

### การทดสอบ

```bash
# รันการทดสอบทั้งหมด
cargo test --workspace

# รันการทดสอบพร้อมแสดงผลลัพธ์ทั้งหมด
cargo test --workspace -- --nocapture
```

### การตรวจสอบโค้ด

```bash
# ตรวจสอบปัญหาด้านคุณภาพโค้ด
cargo clippy --workspace --all-targets --all-features

# ใช้การแก้ไขอัตโนมัติ
cargo clippy --fix --allow-dirty --workspace
```

### การจัดรูปแบบโค้ด

```bash
# ตรวจสอบการจัดรูปแบบโค้ด
cargo fmt --all -- --check

# จัดรูปแบบโค้ดอัตโนมัติ
cargo fmt --all
```

## ตัวอย่างการใช้งาน

### การยืนยันตัวตน

```bash
# เข้าสู่ระบบวิกิ (จัดเก็บข้อมูลรับรองอย่างปลอดภัย)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# เข้าสู่ระบบด้วย OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### การแสดงรายการหน้า

```bash
# ดึงรายการหน้าจากหมวดหมู่
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# ดึงรายการหน้าจากผลการค้นหา
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# ดึงรายการหน้าจากไฟล์
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### การเรียกใช้กฎการแก้ไข

```bash
# รันกฎด้วยโปรไฟล์ (โหมดทดลองทำงาน)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# นำกฎไปใช้กับหน้าจริง
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### โหมดบอตอัตโนมัติ

```bash
# รันในโหมดบอตอัตโนมัติพร้อมกำหนดขีดจำกัด
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# ดำเนินงานต่อจากเซสชันก่อนหน้า
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### ตัวอย่างโปรไฟล์กฎ

สร้างไฟล์ `my-rules.toml`:

```toml
[profile]
name = "บอตจัดระเบียบเนื้อหา"
description = "แก้ไขปัญหาการจัดรูปแบบทั่วไป"
edit_summary = "จัดระเบียบเนื้อหาอัตโนมัติ (บอต)"

[[rules]]
name = "แก้ไขช่องว่างซ้ำซ้อน"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "แก้ไขลิงก์ภายนอก"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "เพิ่มหมวดหมู่ที่ขาดหาย"
plugin = "add_categories.lua"
```

## ระบบปลั๊กอิน

awb-rs รองรับปลั๊กอิน 2 ประเภท:

### ปลั๊กอิน Lua

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### ปลั๊กอิน WebAssembly

```rust
// คอมไพล์เป็น WASM ด้วยคำสั่ง: cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**คุณสมบัติของปลั๊กอิน:**
- ประมวลผลในสภาพแวดล้อมแยกพร้อมจำกัดทรัพยากร (หน่วยความจำและเวลา CPU)
- เข้าถึงได้เฉพาะชุดย่อยของ API ที่ปลอดภัย (ไม่เข้าถึงระบบไฟล์และเครือข่าย)
- ตรวจสอบความเข้ากันได้ของเวอร์ชัน
- รองรับการโหลดซ้ำอัตโนมัติในโหมดพัฒนา

## รูปแบบความปลอดภัย

- **การจัดเก็บข้อมูลรับรอง**: ใช้ระบบพวงกุญแจของระบบปฏิบัติการเพื่อจัดเก็บรหัสผ่านอย่างปลอดภัย
- **การแยกสภาพแวดล้อมปลั๊กอิน**: Lua และ WASM ทำงานในสภาพแวดล้อมที่แยกส่วนอย่างสมบูรณ์
- **การจำกัดทรัพยากร**: จำกัดเวลา CPU หน่วยความจำ และจำนวนคำสั่งต่อปลั๊กอิน
- **การจำกัดอัตราการเรียก API**: ปฏิบัติตามส่วนหัว `Retry-After` และกำหนดระยะเวลาหน่วงได้
- **การตรวจสอบการแก้ไข**: แสดงตัวอย่างการเปลี่ยนแปลงก่อนบันทึกจริงพร้อมบันทึกการตรวจสอบ
- **รองรับ OAuth**: วิธีการยืนยันตัวตนที่แนะนำสำหรับบอต

## สัญญาอนุญาต

เผยแพร่ภายใต้สัญญาอนุญาตอย่างใดอย่างหนึ่งต่อไปนี้:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) หรือ http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../../../LICENSE-MIT) หรือ http://opensource.org/licenses/MIT)

ตามที่คุณเลือก

## การมีส่วนร่วม

ยินดีต้อนรับทุกการมีส่วนร่วม โปรดศึกษาแนวทางได้ที่ [CONTRIBUTING.md](../../../CONTRIBUTING.md)

## สถานะโครงการ

โครงการนี้อยู่ระหว่างการพัฒนา AutoWikiBrowser ขึ้นใหม่ในภาษา Rust อย่างต่อเนื่อง สถานะปัจจุบัน:

- [x] ไคลเอนต์ API หลัก
- [x] เอ็นจินกฎ
- [x] ส่วนติดต่อผู้ใช้แบบ CLI
- [x] ระบบปลั๊กอิน (Lua/WASM)
- [x] การแยกสภาพแวดล้อมด้านความปลอดภัย
- [ ] ส่วนติดต่อผู้ใช้แบบเทอร์มินัล (กำลังดำเนินการ)
- [ ] ส่วนติดต่อผู้ใช้แบบกราฟิก (อยู่ในแผนงาน)
- [ ] การแสดงผลความแตกต่างขั้นสูง (อยู่ในแผนงาน)

## ลิงก์

- [เอกสารประกอบ](https://docs.rs/awb-rs)
- [ระบบติดตามปัญหา](https://github.com/somethingwithproof/awb-rs/issues)
- [AWB ต้นฉบับ](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
