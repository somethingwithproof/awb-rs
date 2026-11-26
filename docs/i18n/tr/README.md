# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Güvenlik, performans ve genişletilebilirlik odaklı, otomatik Vikipedi düzenleme yetenekleri sunan, AutoWikiBrowser (AWB) aracının Rust dilinde modern ve yüksek performanslı yeniden yazımı.

## Mimari

```
awb-rs/
├── awb-core/           # Temel iş mantığı ve soyutlamalar
├── awb-api/            # MediaWiki API istemci uygulaması
├── awb-cli/            # Komut satırı arayüzü
├── awb-rules/          # Düzenleme otomasyonu için kural motoru
├── awb-lua-plugin/     # Lua eklenti çalışma zamanı
├── awb-wasm-plugin/    # WebAssembly eklenti çalışma zamanı
├── awb-security/       # Güvenlik ve yalıtım (sandboxing)
├── awb-db/             # Kalıcı durum yönetimi
├── awb-ui/             # Terminal kullanıcı arayüzü bileşenleri
├── awb-fixtures/       # Test düzenekleri ve sahte (mock) veriler
└── awb-integration/    # Entegrasyon test paketi
```

**Veri Akışı:**
```
CLI → Core → API → MediaWiki
       ↓
    Kural Motoru → Eklentiler (Lua/WASM)
       ↓
    Güvenlik Katmanı → Yalıtılmış (Sandbox) Yürütme
       ↓
    Veritabanı → Durum Kalıcılığı
```

## Özellikler

- **Modern Mimari**: Yüksek performans için Tokio çalışma zamanı ile async/await
- **Çoklu Viki Desteği**: MediaWiki tabanlı herhangi bir viki ile çalışır
- **Kural Tabanlı Düzenleme**: Bildirimsel TOML tabanlı kural tanımları
- **Eklenti Sistemi**: Lua betikleri ve WebAssembly modülleri ile genişletilebilir
- **Güçlendirilmiş Güvenlik**: Kaynak sınırları ile yalıtılmış eklenti yürütme
- **Toplu İşlemler**: Verimli toplu sayfa işleme
- **Etkileşimli TUI**: İzleme ve kontrol için zengin terminal arayüzü
- **Deneme Modu (Dry-Run)**: Değişiklikleri uygulamadan önce önizleme
- **Durum Yönetimi**: Kesintiye uğrayan oturumları kaldığı yerden sürdürme
- **Hız Sınırlama**: Viki API sınırlarına ve en iyi uygulamalara uyum
- **Uluslararasılaştırma**: Arapça, Farsça, İbranice ve Urduca için RTL düzen desteği dâhil olmak üzere `--lang`, `AWB_LANG` veya POSIX yerel ayarı (`LC_ALL`/`LC_MESSAGES`/`LANG`) üzerinden 45 görüntüleme dili

## Desteklenen Diller

CLI, bot raporları ve GTK4 kullanıcı arayüzü 45 dili destekler (6 RTL yerel ayarı dâhil: `ar`, `ar-EG`, `ar-MA`, `fa`, `he` ve `ur`). Sistem yerel ayarı algılamasını geçersiz kılmak için `--lang <LANG>` parametresini geçin veya `AWB_LANG` ortam değişkenini ayarlayın.

Desteklenen 45 dilin tam tablosu ve her bir yerelleştirilmiş belgenin bağlantısı için [README.md](../../../README.md) dosyasına bakın.

## Ön Koşullar

- **Rust**: Sürüm 1.85 veya üzeri (MSRV)
- **Cargo**: Rust kurulumuna dâhildir

## Kurulum

### Kaynak Koddan

```bash
# Depoyu klonlayın
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Tüm çalışma alanı crate'lerini derleyin
cargo build --workspace --release

# İkili dosyayı kurun
cargo install --path awb-cli
```

## Geliştirme

### Derleme

```bash
# Hata ayıklama (debug) derlemesi
cargo build --workspace

# Sürüm (release) derlemesi
cargo build --workspace --release
```

### Test

```bash
# Tüm testleri çalıştırın
cargo test --workspace

# Testleri çıktı ile birlikte çalıştırın
cargo test --workspace -- --nocapture
```

### Kod Analizi (Lint)

```bash
# Lint sorunlarını denetleyin
cargo clippy --workspace --all-targets --all-features

# Otomatik düzeltmeleri uygulayın
cargo clippy --fix --allow-dirty --workspace
```

### Biçimlendirme

```bash
# Biçimlendirmeyi denetleyin
cargo fmt --all -- --check

# Biçimlendirmeyi uygulayın
cargo fmt --all
```

## Kullanım Örnekleri

### Kimlik Doğrulama

```bash
# Bir vikiye giriş yapın (kimlik bilgilerini güvenli biçimde saklar)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# OAuth ile giriş yapın
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Sayfaları Listeleme

```bash
# Bir kategorideki sayfaları listeleyin
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Bir aramadan gelen sayfaları listeleyin
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Bir dosyadan sayfaları listeleyin
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Düzenleme Kurallarını Çalıştırma

```bash
# Bir profil ile kuralları çalıştırın (dry-run modu)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Kuralları gerçek sayfalara uygulayın
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Otomatik Bot Modu

```bash
# Sınırlarla gözetimsiz bot modunda çalıştırın
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Önceki oturumdan devam edin
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Örnek Kural Profili

Bir `my-rules.toml` dosyası oluşturun:

```toml
[profile]
name = "Cleanup Bot"
description = "Fix common formatting issues"
edit_summary = "Automated cleanup (Bot)"

[[rules]]
name = "Fix double spaces"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "Fix external links"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "Add missing categories"
plugin = "add_categories.lua"
```

## Eklenti Sistemi

awb-rs iki eklenti türünü destekler:

### Lua Eklentileri

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### WebAssembly Eklentileri

```rust
// Şununla WASM olarak derleyin: cargo build --target wasm32-wasi
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Eklenti Özellikleri:**
- Kaynak sınırlarıyla (bellek, CPU süresi) yalıtılmış (sandbox) yürütme
- Güvenli API alt kümesine erişim (dosya sistemi ve ağ erişimi yok)
- Sürüm uyumluluğu denetimi
- Geliştirme modunda anında yeniden yükleme (hot-reload) desteği

## Güvenlik Modeli

- **Kimlik Bilgisi Saklama**: Güvenli parola depolama için işletim sistemi anahtarlığını kullanır
- **Eklenti Yalıtımı**: Lua ve WASM yalıtılmış ortamlarda çalışır
- **Kaynak Sınırları**: Eklenti başına CPU süresi, bellek ve komut sayısı sınırları
- **API Hız Sınırlama**: `Retry-After` başlıklarına ve yapılandırılabilir gecikmelere uyar
- **Düzenleme Doğrulaması**: Uygulamadan önce değişiklik önizlemesi ve denetim günlükleri
- **OAuth Desteği**: Botlar için tercih edilen kimlik doğrulama yöntemi

## Lisans

Seçiminize bağlı olarak aşağıdaki lisanslardan biri altında lisanslanmıştır:

- Apache Lisansı, Sürüm 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) veya http://www.apache.org/licenses/LICENSE-2.0)
- MIT Lisansı ([LICENSE-MIT](../../../LICENSE-MIT) veya http://opensource.org/licenses/MIT)

## Katkıda Bulunma

Katkılarınız memnuniyetle karşılanır! Yönergeler için lütfen [CONTRIBUTING.md](../../../CONTRIBUTING.md) dosyasına bakın.

## Proje Durumu

Bu, AutoWikiBrowser aracının Rust dilindeki aktif yeniden yazımıdır. Mevcut durum:

- [x] Temel API istemcisi
- [x] Kural motoru
- [x] CLI arayüzü
- [x] Eklenti sistemi (Lua/WASM)
- [x] Güvenlik yalıtımı (sandboxing)
- [ ] Terminal kullanıcı arayüzü (devam ediyor)
- [ ] Grafik kullanıcı arayüzü GUI (planlandı)
- [ ] Gelişmiş fark (diff) görselleştirme (planlandı)

## Bağlantılar

- [Belgeler](https://docs.rs/awb-rs)
- [Sorun Takipçisi](https://github.com/somethingwithproof/awb-rs/issues)
- [Orijinal AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
