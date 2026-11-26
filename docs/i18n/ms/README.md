# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Penulisan semula AutoWikiBrowser (AWB) yang moden dan berprestasi tinggi dalam bahasa Rust, menyediakan keupayaan suntingan Wikipedia automatik dengan tumpuan kepada keselamatan, prestasi dan kebolehlanjutan.

## Seni Bina

```
awb-rs/
├── awb-core/           # Logik teras perniagaan dan abstraksi
├── awb-api/            # Pelaksanaan klien MediaWiki API
├── awb-cli/            # Antara muka baris perintah
├── awb-rules/          # Enjin peraturan bagi automasi suntingan
├── awb-lua-plugin/     # Masa jalan pemalam Lua
├── awb-wasm-plugin/    # Masa jalan pemalam WebAssembly
├── awb-security/       # Keselamatan dan kotak pasir
├── awb-db/             # Pengurusan keadaan berterusan
├── awb-ui/             # Komponen antara muka pengguna terminal
├── awb-fixtures/       # Lekapan ujian dan data olok-olok
└── awb-integration/    # Rangkaian ujian integrasi
```

**Aliran Data:**
```
CLI → Teras → API → MediaWiki
       ↓
    Enjin Peraturan → Pemalam (Lua/WASM)
       ↓
    Lapisan Keselamatan → Pelaksanaan Kotak Pasir
       ↓
    Pangkalan Data → Penyimpanan Keadaan
```

## Ciri-ciri

- **Seni Bina Moden**: Tak segerak (`async/await`) dengan masa jalan Tokio untuk prestasi tinggi
- **Sokongan Berbilang Wiki**: Berfungsi dengan mana-mana wiki yang dikuasakan oleh MediaWiki
- **Suntingan Berasaskan Peraturan**: Takrifan peraturan deklaratif berasaskan TOML
- **Sistem Pemalam**: Boleh dilanjutkan melalui skrip Lua dan modul WebAssembly
- **Keselamatan Diperkukuh**: Pelaksanaan pemalam dalam kotak pasir dengan had sumber
- **Operasi Berkelompok**: Pemprosesan halaman secara pukal yang cekap
- **TUI Interaktif**: Antara muka terminal yang lengkap untuk pemantauan dan kawalan
- **Mod Ujian Kering (Dry-Run)**: Pratonton perubahan sebelum melaksanakannya
- **Pengurusan Keadaan**: Sambung semula sesi yang tergendala
- **Pengehadan Kadar**: Mematuhi had API wiki dan amalan terbaik
- **Pengantarabangsaan**: 45 bahasa paparan melalui `--lang`, `AWB_LANG`, atau lokal POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`), termasuk sokongan susun atur RTL bagi bahasa Arab, Parsi, Ibrani dan Urdu

## Bahasa yang Disokong

CLI, laporan bot dan antara muka GTK4 menyokong 45 bahasa (termasuk 6 lokal RTL: `ar`, `ar-EG`, `ar-MA`, `fa`, `he` dan `ur`). Berikan `--lang <LANG>` atau tetapkan `AWB_LANG` untuk mengatasi pengesanan lokal sistem. Rujuk [README.md](../../../README.md) bagi jadual bahasa lengkap.

## Prasyarat

- **Rust**: Versi 1.85 atau lebih tinggi (MSRV)
- **Cargo**: Disertakan bersama pemasangan Rust

## Pemasangan

### Daripada Kod Sumber

```bash
# Klon repositori
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Bina semua krat ruang kerja
cargo build --workspace --release

# Pasang fail binari
cargo install --path awb-cli
```

## Pembangunan

### Binaan

```bash
# Binaan nyahpepijat
cargo build --workspace

# Binaan keluaran
cargo build --workspace --release
```

### Ujian

```bash
# Jalankan semua ujian
cargo test --workspace

# Jalankan ujian bersama output
cargo test --workspace -- --nocapture
```

### Semakan Kod (Lint)

```bash
# Semak isu kualiti kod
cargo clippy --workspace --all-targets --all-features

# Gunakan pembaikan automatik
cargo clippy --fix --allow-dirty --workspace
```

### Pemformatan

```bash
# Semak pemformatan kod
cargo fmt --all -- --check

# Gunakan pemformatan kod
cargo fmt --all
```

## Contoh Penggunaan

### Pengesahan

```bash
# Log masuk ke wiki (menyimpan kelayakan dengan selamat)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Log masuk menggunakan OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Penyenaraian Halaman

```bash
# Senaraikan halaman daripada kategori
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Senaraikan halaman daripada carian
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Senaraikan halaman daripada fail
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Menjalankan Peraturan Suntingan

```bash
# Jalankan peraturan menggunakan profil (mod ujian kering)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Gunakan peraturan pada halaman sebenar
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Mod Bot Automatik

```bash
# Jalankan dalam mod bot tanpa pengawasan dengan had
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Sambung semula daripada sesi terdahulu
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Contoh Profil Peraturan

Cipta fail `my-rules.toml`:

```toml
[profile]
name = "Bot Pembersihan"
description = "Membaiki isu pemformatan lazim"
edit_summary = "Pembersihan automatik (Bot)"

[[rules]]
name = "Baiki ruang berganda"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "Baiki pautan luar"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "Tambah kategori yang tiada"
plugin = "add_categories.lua"
```

## Sistem Pemalam

awb-rs menyokong dua jenis pemalam:

### Pemalam Lua

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### Pemalam WebAssembly

```rust
// Kompil kepada WASM dengan: cargo build --target wasm32-wasi
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Ciri-ciri Pemalam:**
- Pelaksanaan kotak pasir dengan had sumber (memori, masa CPU)
- Akses kepada subset API yang selamat (tiada akses sistem fail atau rangkaian)
- Semakan keserasian versi
- Sokongan muat semula pantas dalam mod pembangunan

## Model Keselamatan

- **Penyimpanan Kelayakan**: Menggunakan gelang kunci sistem pengendalian bagi penyimpanan kata laluan yang selamat
- **Kotak Pasir Pemalam**: Lua dan WASM dijalankan dalam persekitaran terasing
- **Had Sumber**: Had masa CPU, memori dan bilangan arahan bagi setiap pemalam
- **Pengehadan Kadar API**: Mematuhi pengepala `Retry-After` serta lengah masa yang boleh dikonfigurasi
- **Pengesahan Suntingan**: Pratonton perubahan sebelum dilaksanakan dan log audit
- **Sokongan OAuth**: Kaedah pengesahan yang diutamakan untuk bot

## Lesen

Dilesenkan di bawah salah satu daripada:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) atau http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../../../LICENSE-MIT) atau http://opensource.org/licenses/MIT)

mengikut pilihan anda.

## Sumbangan

Sumbangan amat dialu-alukan! Sila rujuk [CONTRIBUTING.md](../../../CONTRIBUTING.md) untuk garis panduan.

## Status Projek

Ini ialah projek penulisan semula aktif bagi AutoWikiBrowser dalam Rust. Status semasa:

- [x] Klien API teras
- [x] Enjin peraturan
- [x] Antara muka CLI
- [x] Sistem pemalam (Lua/WASM)
- [x] Kotak pasir keselamatan
- [ ] Antara muka pengguna terminal (sedang dijalankan)
- [ ] GUI (dirancang)
- [ ] Visualisasi perbezaan lanjutan (dirancang)

## Pautan

- [Dokumentasi](https://docs.rs/awb-rs)
- [Penjejak Isu](https://github.com/somethingwithproof/awb-rs/issues)
- [AWB Asal](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
