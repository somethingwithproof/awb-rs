# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Nowoczesna, wysokowydajna reimplementacja AutoWikiBrowser (AWB) w języku Rust, zapewniająca zautomatyzowane możliwości edycji Wikipedii z naciskiem na bezpieczeństwo, wydajność i rozszerzalność.

## Architektura

```
awb-rs/
├── awb-core/           # Główna logika biznesowa i abstrakcje
├── awb-api/            # Implementacja klienta API MediaWiki
├── awb-cli/            # Interfejs wiersza poleceń
├── awb-rules/          # Silnik reguł do automatyzacji edycji
├── awb-lua-plugin/     # Środowisko uruchomieniowe wtyczek Lua
├── awb-wasm-plugin/    # Środowisko uruchomieniowe wtyczek WebAssembly
├── awb-security/       # Bezpieczeństwo i izolacja (sandboxing)
├── awb-db/             # Trwałe zarządzanie stanem
├── awb-ui/             # Komponenty terminalowego interfejsu użytkownika
├── awb-fixtures/       # Dane testowe (fixtures) i atrapy (mock)
└── awb-integration/    # Zestaw testów integracyjnych
```

**Przepływ danych:**
```
CLI → Core → API → MediaWiki
       ↓
    Silnik reguł → Wtyczki (Lua/WASM)
       ↓
    Warstwa bezpieczeństwa → Wykonywanie w piaskownicy
       ↓
    Baza danych → Trwałość stanu
```

## Funkcje

- **Nowoczesna architektura**: Async/await ze środowiskiem uruchomieniowym Tokio dla wysokiej wydajności
- **Obsługa wielu wiki**: Współpraca z dowolną wiki opartą na MediaWiki
- **Edycja oparta na regułach**: Deklaratywne definicje reguł w formacie TOML
- **System wtyczek**: Możliwość rozszerzania za pomocą skryptów Lua i modułów WebAssembly
- **Wzmocnione bezpieczeństwo**: Izolowane wykonywanie wtyczek z limitami zasobów
- **Operacje wsadowe**: Wydajne masowe przetwarzanie stron
- **Interaktywny TUI**: Rozbudowany interfejs terminalowy do monitorowania i sterowania
- **Tryb próbny (dry-run)**: Podgląd zmian przed ich zastosowaniem
- **Zarządzanie stanem**: Wznawianie przerwanych sesji
- **Ograniczanie szybkości zapytań**: Przestrzeganie limitów API wiki i dobrych praktyk
- **Internacjonalizacja**: 45 języków wyświetlania poprzez `--lang`, `AWB_LANG` lub ustawienia lokalne POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`), w tym obsługa układu RTL dla języka arabskiego, perskiego, hebrajskiego i urdu

## Obsługiwane języki

CLI, raporty botów oraz interfejs graficzny GTK4 obsługują 45 języków (w tym 6 lokalizacji RTL: `ar`, `ar-EG`, `ar-MA`, `fa`, `he` oraz `ur`). W CLI użyj opcji `--lang <LANG>`, a dla CLI i interfejsu GTK4 ustaw zmienną `AWB_LANG` lub zmienne lokalizacji POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`).

Pełną tabelę wszystkich 45 obsługiwanych języków wraz z odnośnikami do zlokalizowanej dokumentacji znajdziesz w pliku [README.md](../../../README.md).

## Wymagania wstępne

- **Rust**: Wersja 1.94 lub nowsza (MSRV)
- **Cargo**: Dołączone do instalacji Rust

## Instalacja

### Ze źródeł

```bash
# Sklonuj repozytorium
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Zbuduj wszystkie pakiety (crates) w przestrzeni roboczej
cargo build --workspace --release

# Zainstaluj plik wykonywalny
cargo install --path crates/awb_cli
```

## Rozwój projektu

### Budowanie

```bash
# Kompilacja debug
cargo build --workspace

# Kompilacja release
cargo build --workspace --release
```

### Testy

```bash
# Uruchom wszystkie testy
cargo test --workspace

# Uruchom testy z wyświetlaniem wyjścia standardowego
cargo test --workspace -- --nocapture
```

### Analiza statyczna (Lint)

```bash
# Sprawdź problemy z analizą statyczną
cargo clippy --workspace --all-targets --all-features

# Zastosuj automatyczne poprawki
cargo clippy --fix --allow-dirty --workspace
```

### Formatowanie

```bash
# Sprawdź formatowanie kodu
cargo fmt --all -- --check

# Zastosuj formatowanie kodu
cargo fmt --all
```

## Przykłady użycia

### Uwierzytelnianie

```bash
# Zaloguj się do wiki (bezpiecznie przechowuje dane uwierzytelniające)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Zaloguj się za pomocą OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Pobieranie listy stron

```bash
# Pobierz listę stron z kategorii
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Pobierz listę stron z wyszukiwania
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Pobierz listę stron z pliku
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Uruchamianie reguł edycji

```bash
# Uruchom reguły z profilem (tryb dry-run)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Zastosuj reguły na rzeczywistych stronach
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Zautomatyzowany tryb bota

```bash
# Uruchom w bezobsługowym trybie bota z limitami
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Wznów poprzednią sesję
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Przykładowy profil reguł

Utwórz plik `my-rules.toml`:

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

## System wtyczek

awb-rs obsługuje dwa rodzaje wtyczek:

### Wtyczki Lua

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### Wtyczki WebAssembly

```rust
// Skompiluj do WASM poleceniem: cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Możliwości wtyczek:**
- Wykonywanie w piaskownicy z limitami zasobów (pamięć, czas procesora)
- Dostęp do bezpiecznego podzbioru API (brak dostępu do systemu plików i sieci)
- Sprawdzanie zgodności wersji
- Obsługa przeładowania na gorąco (hot-reload) w trybie deweloperskim

## Model bezpieczeństwa

- **Przechowywanie poświadczeń**: Wykorzystuje systemowy pęk kluczy do bezpiecznego przechowywania haseł
- **Izolacja wtyczek**: Lua i WASM działają w odizolowanych środowiskach
- **Limity zasobów**: Limity czasu procesora, pamięci oraz liczby instrukcji dla każdej wtyczki
- **Ograniczanie szybkości API**: Respektuje nagłówki `Retry-After` i konfigurowalne opóźnienia
- **Walidacja edycji**: Podgląd zmian przed zastosowaniem oraz dzienniki audytu
- **Obsługa OAuth**: Preferowana metoda uwierzytelniania dla botów

## Licencja

Projekt jest udostępniany do wyboru na jednej z poniższych licencji:

- Licencja Apache, wersja 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) lub http://www.apache.org/licenses/LICENSE-2.0)
- Licencja MIT ([LICENSE-MIT](../../../LICENSE-MIT) lub http://opensource.org/licenses/MIT)

## Współtworzenie

Wkład społeczności jest mile widziany! Zapoznaj się z wytycznymi w pliku [CONTRIBUTING.md](../../../CONTRIBUTING.md).

## Status projektu

To aktywnie rozwijana reimplementacja AutoWikiBrowser w języku Rust. Obecny stan:

- [x] Główny klient API
- [x] Silnik reguł
- [x] Interfejs CLI
- [x] System wtyczek (Lua/WASM)
- [x] Izolacja bezpieczeństwa (sandboxing)
- [ ] Interfejs terminalowy (w trakcie)
- [ ] Interfejs graficzny GUI (planowany)
- [ ] Zaawansowana wizualizacja różnic (planowana)

## Odnośniki

- [Dokumentacja](https://docs.rs/awb-rs)
- [System śledzenia zgłoszeń](https://github.com/somethingwithproof/awb-rs/issues)
- [Oryginalny AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
