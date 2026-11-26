# awb-rs

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

A modern, high-performance AutoWikiBrowser (AWB) rewrite in Rust, providing automated Wikipedia editing capabilities with a focus on safety, performance, and extensibility.

## Architecture

```
awb-rs/
├── awb-core/           # Core business logic and abstractions
├── awb-api/            # MediaWiki API client implementation
├── awb-cli/            # Command-line interface
├── awb-rules/          # Rule engine for edit automation
├── awb-lua-plugin/     # Lua plugin runtime
├── awb-wasm-plugin/    # WebAssembly plugin runtime
├── awb-security/       # Security and sandboxing
├── awb-db/             # Persistent state management
├── awb-ui/             # Terminal UI components
├── awb-fixtures/       # Test fixtures and mock data
└── awb-integration/    # Integration test suite
```

**Data Flow:**
```
CLI → Core → API → MediaWiki
       ↓
    Rules Engine → Plugins (Lua/WASM)
       ↓
    Security Layer → Sandboxed Execution
       ↓
    Database → State Persistence
```

## Features

- **Modern Architecture**: Async/await with Tokio runtime for high performance
- **Multi-Wiki Support**: Work with any MediaWiki-powered wiki
- **Rule-Based Editing**: Declarative TOML-based rule definitions
- **Plugin System**: Extensible via Lua scripts and WebAssembly modules
- **Security Hardened**: Sandboxed plugin execution with resource limits
- **Batch Operations**: Efficient bulk page processing
- **Interactive TUI**: Rich terminal interface for monitoring and control
- **Dry-Run Mode**: Preview changes before applying them
- **State Management**: Resume interrupted sessions
- **Rate Limiting**: Respects wiki API limits and best practices
- **Internationalization**: 48 display languages via `--lang`, `AWB_LANG`, or POSIX locale (`LC_ALL`/`LC_MESSAGES`/`LANG`), including RTL layout support for Arabic, Persian, Hebrew, and Urdu

## Supported Languages

The CLI, bot reports, and GTK4 UI support 48 languages (including 6 RTL locales: `ar`, `ar-EG`, `ar-MA`, `fa`, `he`, and `ur`). Pass `--lang <LANG>` in the CLI, or set `AWB_LANG` / POSIX locale variables (`LC_ALL`/`LC_MESSAGES`/`LANG`) for both the CLI and GTK4 UI.

| Code | Language | README | Code | Language | README | Code | Language | README |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `en` | English | [README](README.md) | `pl` | Polski | [README](docs/i18n/pl/README.md) | `so-SO` | Soomaali | [README](docs/i18n/so-SO/README.md) |
| `de` | Deutsch | [README](docs/i18n/de/README.md) | `tr` | Türkçe | [README](docs/i18n/tr/README.md) | `sw-KE` | Kiswahili | [README](docs/i18n/sw-KE/README.md) |
| `es` | Español | [README](docs/i18n/es/README.md) | `sv` | Svenska | [README](docs/i18n/sv/README.md) | `xh-ZA` | isiXhosa | [README](docs/i18n/xh-ZA/README.md) |
| `fr` | Français | [README](docs/i18n/fr/README.md) | `vi` | Tiếng Việt | [README](docs/i18n/vi/README.md) | `yo-NG` | Yorùbá | [README](docs/i18n/yo-NG/README.md) |
| `it` | Italiano | [README](docs/i18n/it/README.md) | `id` | Bahasa Indonesia | [README](docs/i18n/id/README.md) | `zu-ZA` | isiZulu | [README](docs/i18n/zu-ZA/README.md) |
| `pt-BR` | Português (Brasil) | [README](docs/i18n/pt-BR/README.md) | `af-ZA` | Afrikaans | [README](docs/i18n/af-ZA/README.md) | `bn` | বাংলা | [README](docs/i18n/bn/README.md) |
| `nl` | Nederlands | [README](docs/i18n/nl/README.md) | `am-ET` | አማርኛ | [README](docs/i18n/am-ET/README.md) | `fa` | فارسی (RTL) | [README](docs/i18n/fa/README.md) |
| `ru` | Русский | [README](docs/i18n/ru/README.md) | `ar-EG` | العربية (مصر) (RTL) | [README](docs/i18n/ar-EG/README.md) | `he` | עברית (RTL) | [README](docs/i18n/he/README.md) |
| `uk` | Українська | [README](docs/i18n/uk/README.md) | `ar-MA` | العربية (المغرب) (RTL) | [README](docs/i18n/ar-MA/README.md) | `mr` | मराठी | [README](docs/i18n/mr/README.md) |
| `ja` | 日本語 | [README](docs/i18n/ja/README.md) | `fr-MA` | Français (Maroc) | [README](docs/i18n/fr-MA/README.md) | `ta` | தமிழ் | [README](docs/i18n/ta/README.md) |
| `ko` | 한국어 | [README](docs/i18n/ko/README.md) | `fr-SN` | Français (Sénégal) | [README](docs/i18n/fr-SN/README.md) | `te` | తెలుగు | [README](docs/i18n/te/README.md) |
| `zh-CN` | 简体中文 | [README](docs/i18n/zh-CN/README.md) | `ha-NG` | Hausa | [README](docs/i18n/ha-NG/README.md) | `th` | ไทย | [README](docs/i18n/th/README.md) |
| `zh-TW` | 繁體中文 | [README](docs/i18n/zh-TW/README.md) | `ig-NG` | Igbo | [README](docs/i18n/ig-NG/README.md) | `ur` | اردو (RTL) | [README](docs/i18n/ur/README.md) |
| `ar` | العربية (RTL) | [README](docs/i18n/ar/README.md) | `mg-MG` | Malagasy | [README](docs/i18n/mg-MG/README.md) | `ms` | Bahasa Melayu | [README](docs/i18n/ms/README.md) |
| `hi` | हिन्दी | [README](docs/i18n/hi/README.md) | `rw-RW` | Kinyarwanda | [README](docs/i18n/rw-RW/README.md) | `fil` | Filipino | [README](docs/i18n/fil/README.md) |
| `bs` | Bosanski | [README](docs/i18n/bs/README.md) | `hr` | Hrvatski | [README](docs/i18n/hr/README.md) | `sr` | Српски | [README](docs/i18n/sr/README.md) |

## Prerequisites

- **Rust**: Version 1.94 or higher (MSRV)
- **Cargo**: Included with Rust installation

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Build all workspace crates
cargo build --workspace --release

# Install the binary
cargo install --path crates/awb_cli
```

## Development

### Build

```bash
# Debug build
cargo build --workspace

# Release build
cargo build --workspace --release
```

### Test

```bash
# Run all tests
cargo test --workspace

# Run tests with output
cargo test --workspace -- --nocapture
```

### Lint

```bash
# Check for linting issues
cargo clippy --workspace --all-targets --all-features

# Apply automatic fixes
cargo clippy --fix --allow-dirty --workspace
```

### Format

```bash
# Check formatting
cargo fmt --all -- --check

# Apply formatting
cargo fmt --all
```

## Usage Examples

### Authentication

```bash
# Login to a wiki (stores credentials securely)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Login with OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Listing Pages

```bash
# List pages from a category
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# List pages from a search
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# List pages from a file
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Running Edit Rules

```bash
# Run rules with a profile (dry-run mode)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Apply rules to real pages
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Automated Bot Mode

```bash
# Run in unattended bot mode with limits
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Resume from previous session
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Example Rule Profile

Create a `my-rules.toml` file:

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

## Plugin System

awb-rs supports two plugin types:

### Lua Plugins

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### WebAssembly Plugins

```rust
// Compile to WASM with: cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Plugin Features:**
- Sandboxed execution with resource limits (memory, CPU time)
- Access to safe subset of APIs (no filesystem, network)
- Version compatibility checking
- Hot-reload support in development mode

## Security Model

- **Credential Storage**: Uses OS keyring for secure password storage
- **Plugin Sandboxing**: Lua and WASM run in isolated environments
- **Resource Limits**: CPU time, memory, and instruction count limits per plugin
- **API Rate Limiting**: Respects Retry-After headers and configurable delays
- **Edit Validation**: Preview changes before applying, audit logs
- **OAuth Support**: Preferred authentication method for bots

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Project Status

This is an active rewrite of AutoWikiBrowser in Rust. Current status:

- [x] Core API client
- [x] Rule engine
- [x] CLI interface
- [x] Plugin system (Lua/WASM)
- [x] Security sandboxing
- [ ] Terminal UI (in progress)
- [ ] GUI (planned)
- [ ] Advanced diff visualization (planned)

## Links

- [Documentation](https://docs.rs/awb-rs)
- [Issue Tracker](https://github.com/somethingwithproof/awb-rs/issues)
- [Original AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
