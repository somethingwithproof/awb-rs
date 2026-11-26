# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Una riscrittura moderna e ad alte prestazioni di AutoWikiBrowser (AWB) in Rust, che offre funzionalità automatizzate di modifica per Wikipedia con particolare attenzione a sicurezza, prestazioni ed estensibilità.

## Architettura

```
awb-rs/
├── awb-core/           # Logica applicativa principale e astrazioni
├── awb-api/            # Implementazione del client API MediaWiki
├── awb-cli/            # Interfaccia a riga di comando
├── awb-rules/          # Motore di regole per l'automazione delle modifiche
├── awb-lua-plugin/     # Runtime per plugin Lua
├── awb-wasm-plugin/    # Runtime per plugin WebAssembly
├── awb-security/       # Sicurezza e sandboxing
├── awb-db/             # Gestione persistente dello stato
├── awb-ui/             # Componenti dell'interfaccia utente da terminale
├── awb-fixtures/       # Fixture di test e dati fittizi (mock)
└── awb-integration/    # Suite di test di integrazione
```

**Flusso dei dati:**
```
CLI → Core → API → MediaWiki
       ↓
    Motore di regole → Plugin (Lua/WASM)
       ↓
    Livello di sicurezza → Esecuzione in sandbox
       ↓
    Database → Persistenza dello stato
```

## Funzionalità

- **Architettura moderna**: Async/await con runtime Tokio per prestazioni elevate
- **Supporto multi-wiki**: Compatibile con qualsiasi wiki basato su MediaWiki
- **Modifica basata su regole**: Definizioni dichiarative delle regole in formato TOML
- **Sistema di plugin**: Estensibile tramite script Lua e moduli WebAssembly
- **Sicurezza rafforzata**: Esecuzione isolata dei plugin con limiti di risorse
- **Operazioni in batch**: Elaborazione massiva ed efficiente delle pagine
- **TUI interattiva**: Interfaccia da terminale ricca per il monitoraggio e il controllo
- **Modalità di simulazione (dry-run)**: Anteprima delle modifiche prima dell'applicazione
- **Gestione dello stato**: Ripresa delle sessioni interrotte
- **Limitazione della frequenza**: Rispetta i limiti delle API del wiki e le migliori pratiche
- **Internazionalizzazione**: 45 lingue di visualizzazione tramite `--lang`, `AWB_LANG` o locale POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`), incluso il supporto per il layout RTL per arabo, persiano, ebraico e urdu

## Lingue supportate

La CLI, i report dei bot e l'interfaccia grafica GTK4 supportano 45 lingue (tra cui 6 locale RTL: `ar`, `ar-EG`, `ar-MA`, `fa`, `he` e `ur`). Passare `--lang <LANG>` oppure impostare `AWB_LANG` per sostituire il rilevamento automatico della locale di sistema.

Per la tabella completa di tutte le 45 lingue supportate e i collegamenti alle relative documentazioni localizzate, consultare [README.md](../../../README.md).

## Prerequisiti

- **Rust**: Versione 1.85 o superiore (MSRV)
- **Cargo**: Incluso nell'installazione di Rust

## Installazione

### Dai sorgenti

```bash
# Clonare il repository
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Compilare tutti i crate del workspace
cargo build --workspace --release

# Installare il binario
cargo install --path awb-cli
```

## Sviluppo

### Compilazione

```bash
# Compilazione di debug
cargo build --workspace

# Compilazione di rilascio
cargo build --workspace --release
```

### Test

```bash
# Eseguire tutti i test
cargo test --workspace

# Eseguire i test mostrando l'output standard
cargo test --workspace -- --nocapture
```

### Linting

```bash
# Verificare eventuali problemi di linting
cargo clippy --workspace --all-targets --all-features

# Applicare le correzioni automatiche
cargo clippy --fix --allow-dirty --workspace
```

### Formattazione

```bash
# Verificare la formattazione
cargo fmt --all -- --check

# Applicare la formattazione
cargo fmt --all
```

## Esempi di utilizzo

### Autenticazione

```bash
# Accedere a un wiki (salva le credenziali in modo sicuro)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Accedere tramite OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Elenco delle pagine

```bash
# Elencare le pagine di una categoria
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Elencare le pagine da una ricerca
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Elencare le pagine da un file
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Esecuzione delle regole di modifica

```bash
# Eseguire le regole con un profilo (modalità dry-run)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Applicare le regole a pagine reali
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Modalità bot automatizzata

```bash
# Eseguire in modalità bot non presidiata con limiti
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Riprendere da una sessione precedente
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Esempio di profilo di regole

Creare un file `my-rules.toml`:

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

## Sistema di plugin

awb-rs supporta due tipi di plugin:

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
// Compilare in WASM con: cargo build --target wasm32-wasi
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Funzionalità dei plugin:**
- Esecuzione in sandbox con limiti di risorse (memoria, tempo CPU)
- Accesso a un sottoinsieme sicuro di API (nessun accesso al filesystem o alla rete)
- Verifica della compatibilità di versione
- Supporto hot-reload in modalità di sviluppo

## Modello di sicurezza

- **Archiviazione delle credenziali**: Utilizza il portachiavi del sistema operativo per salvare le password in sicurezza
- **Sandboxing dei plugin**: Lua e WASM vengono eseguiti in ambienti isolati
- **Limiti di risorse**: Limiti di tempo CPU, memoria e numero di istruzioni per singolo plugin
- **Limitazione della frequenza API**: Rispetta le intestazioni `Retry-After` e ritardi configurabili
- **Convalida delle modifiche**: Anteprima delle modifiche prima dell'applicazione e log di audit
- **Supporto OAuth**: Metodo di autenticazione raccomandato per i bot

## Licenza

Rilasciato a vostra scelta sotto una delle seguenti licenze:

- Licenza Apache, versione 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) o http://www.apache.org/licenses/LICENSE-2.0)
- Licenza MIT ([LICENSE-MIT](../../../LICENSE-MIT) o http://opensource.org/licenses/MIT)

## Contribuire

I contributi sono benvenuti! Consultare [CONTRIBUTING.md](../../../CONTRIBUTING.md) per le linee guida.

## Stato del progetto

Questa è una riscrittura attiva di AutoWikiBrowser in Rust. Stato attuale:

- [x] Client API principale
- [x] Motore di regole
- [x] Interfaccia CLI
- [x] Sistema di plugin (Lua/WASM)
- [x] Sandboxing di sicurezza
- [ ] Interfaccia utente da terminale (in corso)
- [ ] Interfaccia grafica GUI (pianificata)
- [ ] Visualizzazione avanzata delle differenze (pianificata)

## Collegamenti

- [Documentazione](https://docs.rs/awb-rs)
- [Tracciamento segnalazioni](https://github.com/somethingwithproof/awb-rs/issues)
- [AWB originale](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
