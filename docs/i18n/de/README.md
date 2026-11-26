# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Eine moderne, leistungsstarke Neuimplementierung von AutoWikiBrowser (AWB) in Rust, die automatisierte Wikipedia-Bearbeitungsfunktionen mit Fokus auf Sicherheit, Leistung und Erweiterbarkeit bietet.

## Architektur

```
awb-rs/
├── awb-core/           # Zentrale Geschäftslogik und Abstraktionen
├── awb-api/            # Implementierung des MediaWiki-API-Clients
├── awb-cli/            # Kommandozeilenschnittstelle
├── awb-rules/          # Regel-Engine für die Bearbeitungsautomatisierung
├── awb-lua-plugin/     # Lua-Plugin-Laufzeitumgebung
├── awb-wasm-plugin/    # WebAssembly-Plugin-Laufzeitumgebung
├── awb-security/       # Sicherheit und Sandboxing
├── awb-db/             # Persistente Zustandsverwaltung
├── awb-ui/             # Terminal-UI-Komponenten
├── awb-fixtures/       # Test-Fixtures und Mock-Daten
└── awb-integration/    # Integrationstest-Suite
```

**Datenfluss:**
```
CLI → Core → API → MediaWiki
       ↓
    Regel-Engine → Plugins (Lua/WASM)
       ↓
    Sicherheitsschicht → Ausführung in der Sandbox
       ↓
    Datenbank → Zustandspersistenz
```

## Funktionen

- **Moderne Architektur**: Async/await mit Tokio-Laufzeitumgebung für hohe Leistung
- **Multi-Wiki-Unterstützung**: Kompatibel mit jedem MediaWiki-basierten Wiki
- **Regelbasierte Bearbeitung**: Deklarative, TOML-basierte Regeldefinitionen
- **Plugin-System**: Erweiterbar durch Lua-Skripte und WebAssembly-Module
- **Erhöhte Sicherheit**: Sandbox-Ausführung für Plugins mit Ressourcenlimits
- **Stapelverarbeitung**: Effiziente Massenbearbeitung von Seiten
- **Interaktive TUI**: Umfangreiche Terminaloberfläche zur Überwachung und Steuerung
- **Testlauf-Modus (Dry-Run)**: Vorschau von Änderungen vor deren Anwendung
- **Zustandsverwaltung**: Fortsetzen unterbrochener Sitzungen
- **Ratenbegrenzung**: Einhaltung von Wiki-API-Limits und bewährten Verfahren
- **Internationalisierung**: 45 Anzeigesprachen über `--lang`, `AWB_LANG` oder POSIX-Locale (`LC_ALL`/`LC_MESSAGES`/`LANG`), einschließlich RTL-Layout-Unterstützung für Arabisch, Persisch, Hebräisch und Urdu

## Unterstützte Sprachen

Die CLI, Bot-Berichte und die GTK4-Benutzeroberfläche unterstützen 45 Sprachen (darunter 6 RTL-Locales: `ar`, `ar-EG`, `ar-MA`, `fa`, `he` und `ur`). Übergeben Sie `--lang <LANG>` oder setzen Sie `AWB_LANG`, um die automatische Erkennung der System-Locale zu überschreiben.

Die vollständige Übersicht aller 45 unterstützten Sprachen sowie Links zu den jeweiligen lokalisierten Dokumentationen finden Sie in der englischen [README.md](../../../README.md).

## Voraussetzungen

- **Rust**: Version 1.85 oder höher (MSRV)
- **Cargo**: In der Rust-Installation enthalten

## Installation

### Aus dem Quellcode

```bash
# Repository klonen
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Alle Workspace-Crates kompilieren
cargo build --workspace --release

# Binärdatei installieren
cargo install --path awb-cli
```

## Entwicklung

### Kompilieren

```bash
# Debug-Build
cargo build --workspace

# Release-Build
cargo build --workspace --release
```

### Tests ausführen

```bash
# Alle Tests ausführen
cargo test --workspace

# Tests mit Standardausgabe ausführen
cargo test --workspace -- --nocapture
```

### Linting

```bash
# Auf Linting-Probleme prüfen
cargo clippy --workspace --all-targets --all-features

# Automatische Korrekturen anwenden
cargo clippy --fix --allow-dirty --workspace
```

### Formatierung

```bash
# Formatierung prüfen
cargo fmt --all -- --check

# Formatierung anwenden
cargo fmt --all
```

## Anwendungsbeispiele

### Authentifizierung

```bash
# Bei einem Wiki anmelden (speichert Zugangsdaten sicher)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Mit OAuth anmelden
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Seiten auflisten

```bash
# Seiten aus einer Kategorie auflisten
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Seiten aus einer Suche auflisten
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Seiten aus einer Datei auflisten
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Bearbeitungsregeln ausführen

```bash
# Regeln mit einem Profil ausführen (Dry-Run-Modus)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Regeln auf echte Seiten anwenden
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Automatisierter Bot-Modus

```bash
# Im unbeaufsichtigten Bot-Modus mit Limits ausführen
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Vorherige Sitzung fortsetzen
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Beispiel-Regelprofil

Erstellen Sie eine Datei `my-rules.toml`:

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

## Plugin-System

awb-rs unterstützt zwei Plugin-Typen:

### Lua-Plugins

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### WebAssembly-Plugins

```rust
// Zu WASM kompilieren mit: cargo build --target wasm32-wasi
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Plugin-Funktionen:**
- Ausführung in einer Sandbox mit Ressourcenlimits (Speicher, CPU-Zeit)
- Zugriff auf eine sichere Teilmenge von APIs (kein Dateisystem-, kein Netzwerkzugriff)
- Überprüfung der Versionskompatibilität
- Hot-Reload-Unterstützung im Entwicklungsmodus

## Sicherheitsmodell

- **Zugangsdatenspeicherung**: Verwendet den Schlüsselbund des Betriebssystems zur sicheren Passwortspeicherung
- **Plugin-Sandboxing**: Lua und WASM werden in isolierten Umgebungen ausgeführt
- **Ressourcenlimits**: Begrenzung von CPU-Zeit, Arbeitsspeicher und Befehlsanzahl pro Plugin
- **API-Ratenbegrenzung**: Berücksichtigt `Retry-After`-Header und konfigurierbare Verzögerungen
- **Bearbeitungsvalidierung**: Änderungsvorschau vor dem Anwenden sowie Audit-Protokolle
- **OAuth-Unterstützung**: Bevorzugte Authentifizierungsmethode für Bots

## Lizenz

Lizenziert unter einer der folgenden Lizenzen nach Ihrer Wahl:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) oder http://www.apache.org/licenses/LICENSE-2.0)
- MIT-Lizenz ([LICENSE-MIT](../../../LICENSE-MIT) oder http://opensource.org/licenses/MIT)

## Mitwirken

Beiträge sind willkommen! Richtlinien finden Sie in [CONTRIBUTING.md](../../../CONTRIBUTING.md).

## Projektstatus

Dies ist eine aktive Neuimplementierung von AutoWikiBrowser in Rust. Aktueller Stand:

- [x] Kern-API-Client
- [x] Regel-Engine
- [x] CLI-Schnittstelle
- [x] Plugin-System (Lua/WASM)
- [x] Sicherheits-Sandboxing
- [ ] Terminal-UI (in Arbeit)
- [ ] GUI (geplant)
- [ ] Erweiterte Diff-Visualisierung (geplant)

## Links

- [Dokumentation](https://docs.rs/awb-rs)
- [Issue-Tracker](https://github.com/somethingwithproof/awb-rs/issues)
- [Original-AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
