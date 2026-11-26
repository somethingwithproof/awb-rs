# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Implementasi ulang AutoWikiBrowser (AWB) yang modern dan berperforma tinggi dalam bahasa Rust, menyediakan kemampuan penyuntingan Wikipedia otomatis dengan fokus pada keamanan, performa, dan ekstensibilitas.

## Arsitektur

```
awb-rs/
├── awb-core/           # Logika bisnis inti dan abstraksi
├── awb-api/            # Implementasi klien MediaWiki API
├── awb-cli/            # Antarmuka baris perintah
├── awb-rules/          # Mesin aturan untuk otomatisasi penyuntingan
├── awb-lua-plugin/     # Runtime plugin Lua
├── awb-wasm-plugin/    # Runtime plugin WebAssembly
├── awb-security/       # Keamanan dan sandboxing
├── awb-db/             # Manajemen status persisten
├── awb-ui/             # Komponen antarmuka terminal (TUI)
├── awb-fixtures/       # Data uji dan data tiruan
└── awb-integration/    # Rangkaian pengujian integrasi
```

**Alur Data:**
```
CLI → Inti → API → MediaWiki
       ↓
    Mesin Aturan → Plugin (Lua/WASM)
       ↓
    Lapisan Keamanan → Eksekusi Terisolasi (Sandbox)
       ↓
    Basis Data → Persistensi Status
```

## Fitur

- **Arsitektur Modern**: Pemrograman asinkron async/await dengan runtime Tokio untuk performa tinggi
- **Dukungan Multi-Wiki**: Dapat digunakan pada berbagai situs wiki berbasis MediaWiki
- **Penyuntingan Berbasis Aturan**: Definisi aturan deklaratif menggunakan format TOML
- **Sistem Plugin**: Dapat diperluas melalui skrip Lua dan modul WebAssembly
- **Keamanan Terperkuat**: Eksekusi plugin di dalam sandbox dengan pembatasan sumber daya
- **Operasi Massal**: Pemrosesan halaman dalam jumlah besar secara efisien
- **TUI Interaktif**: Antarmuka terminal yang lengkap untuk pemantauan dan pengendalian
- **Mode Uji Coba (Dry-Run)**: Pratinjau perubahan sebelum diterapkan secara permanen
- **Manajemen Status**: Melanjutkan sesi yang terhenti
- **Pembatasan Laju (Rate Limiting)**: Mematuhi batasan API wiki serta praktik terbaik
- **Internasionalisasi**: Mendukung 45 bahasa tampilan melalui `--lang`, `AWB_LANG`, atau lokal POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`), termasuk dukungan tata letak RTL untuk bahasa Arab, Persia, Ibrani, dan Urdu

## Bahasa yang Didukung

CLI, laporan bot, dan antarmuka GTK4 mendukung 45 bahasa (termasuk 6 lokal RTL: `ar`, `ar-EG`, `ar-MA`, `fa`, `he`, dan `ur`). Berikan argumen `--lang <LANG>` atau atur variabel lingkungan `AWB_LANG` untuk menimpa deteksi lokal sistem. Lihat tabel bahasa selengkapnya di [README.md](../../../README.md).

## Prasyarat

- **Rust**: Versi 1.85 atau lebih baru (MSRV)
- **Cargo**: Sudah disertakan bersama instalasi Rust

## Instalasi

### Dari Kode Sumber

```bash
# Kloning repositori
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Bangun semua crate dalam workspace
cargo build --workspace --release

# Instal berkas biner
cargo install --path awb-cli
```

## Pengembangan

### Bangun

```bash
# Build debug
cargo build --workspace

# Build rilis
cargo build --workspace --release
```

### Pengujian

```bash
# Jalankan semua pengujian
cargo test --workspace

# Jalankan pengujian dengan menampilkan keluaran
cargo test --workspace -- --nocapture
```

### Lint

```bash
# Periksa masalah linting
cargo clippy --workspace --all-targets --all-features

# Terapkan perbaikan otomatis
cargo clippy --fix --allow-dirty --workspace
```

### Pemformatan

```bash
# Periksa pemformatan kode
cargo fmt --all -- --check

# Terapkan pemformatan kode
cargo fmt --all
```

## Contoh Penggunaan

### Autentikasi

```bash
# Masuk ke wiki (menyimpan kredensial secara aman)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Masuk menggunakan OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Menampilkan Daftar Halaman

```bash
# Daftar halaman dari sebuah kategori
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Daftar halaman dari hasil pencarian
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Daftar halaman dari sebuah berkas
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Menjalankan Aturan Penyuntingan

```bash
# Jalankan aturan dengan profil (mode uji coba)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Terapkan aturan ke halaman sebenarnya
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Mode Bot Otomatis

```bash
# Jalankan dalam mode bot otomatis dengan batasan
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Lanjutkan dari sesi sebelumnya
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Contoh Profil Aturan

Buat berkas `my-rules.toml`:

```toml
[profile]
name = "Bot Pembersihan"
description = "Perbaiki masalah pemformatan umum"
edit_summary = "Pembersihan otomatis (Bot)"

[[rules]]
name = "Perbaiki spasi ganda"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "Perbaiki pranala luar"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "Tambahkan kategori yang hilang"
plugin = "add_categories.lua"
```

## Sistem Plugin

awb-rs mendukung dua jenis plugin:

### Plugin Lua

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### Plugin WebAssembly

```rust
// Kompilasi ke WASM dengan: cargo build --target wasm32-wasi
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Fitur Plugin:**
- Eksekusi terisolasi dalam sandbox dengan pembatasan sumber daya (memori, waktu CPU)
- Akses hanya ke subset API yang aman (tanpa akses sistem berkas maupun jaringan)
- Pemeriksaan kompatibilitas versi
- Dukungan pemuatan ulang cepat (hot-reload) dalam mode pengembangan

## Model Keamanan

- **Penyimpanan Kredensial**: Menggunakan keyring sistem operasi untuk menyimpan kata sandi secara aman
- **Sandboxing Plugin**: Lua dan WASM dijalankan di lingkungan terisolasi
- **Batasan Sumber Daya**: Pembatasan waktu CPU, memori, dan jumlah instruksi per plugin
- **Pembatasan Laju API**: Mematuhi header `Retry-After` dan jeda yang dapat dikonfigurasi
- **Validasi Suntingan**: Pratinjau perubahan sebelum diterapkan serta pencatatan log audit
- **Dukungan OAuth**: Metode autentikasi yang direkomendasikan untuk bot

## Lisensi

Dilisensikan di bawah salah satu dari lisensi berikut, sesuai pilihan Anda:

- Lisensi Apache, Versi 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) atau http://www.apache.org/licenses/LICENSE-2.0)
- Lisensi MIT ([LICENSE-MIT](../../../LICENSE-MIT) atau http://opensource.org/licenses/MIT)

## Kontribusi

Kontribusi sangat diterima! Silakan lihat [CONTRIBUTING.md](../../../CONTRIBUTING.md) untuk panduan lengkap.

## Status Proyek

Ini merupakan proyek penulisan ulang aktif AutoWikiBrowser dalam bahasa Rust. Status saat ini:

- [x] Klien API inti
- [x] Mesin aturan
- [x] Antarmuka baris perintah (CLI)
- [x] Sistem plugin (Lua/WASM)
- [x] Sandboxing keamanan
- [ ] Antarmuka terminal (sedang dikerjakan)
- [ ] GUI (direncanakan)
- [ ] Visualisasi perbandingan (diff) lanjutan (direncanakan)

## Tautan

- [Dokumentasi](https://docs.rs/awb-rs)
- [Pelacak Masalah](https://github.com/somethingwithproof/awb-rs/issues)
- [AWB Asli](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
