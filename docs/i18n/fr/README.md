# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Une réécriture moderne et haute performance d'AutoWikiBrowser (AWB) en Rust, offrant des capacités d'édition automatisée sur Wikipédia axées sur la sécurité, la performance et l'extensibilité.

## Architecture

```
awb-rs/
├── awb-core/           # Logique métier centrale et abstractions
├── awb-api/            # Implémentation du client API MediaWiki
├── awb-cli/            # Interface en ligne de commande
├── awb-rules/          # Moteur de règles pour l'automatisation des éditions
├── awb-lua-plugin/     # Environnement d'exécution de greffons Lua
├── awb-wasm-plugin/    # Environnement d'exécution de greffons WebAssembly
├── awb-security/       # Sécurité et bac à sable (sandboxing)
├── awb-db/             # Gestion persistante de l'état
├── awb-ui/             # Composants d'interface utilisateur en terminal
├── awb-fixtures/       # Jeux de données de test et bouchons (mocks)
└── awb-integration/    # Suite de tests d'intégration
```

**Flux de données :**
```
CLI → Core → API → MediaWiki
       ↓
    Moteur de règles → Greffons (Lua/WASM)
       ↓
    Couche de sécurité → Exécution en bac à sable
       ↓
    Base de données → Persistance de l'état
```

## Fonctionnalités

- **Architecture moderne** : Async/await avec le runtime Tokio pour des performances élevées
- **Prise en charge multi-wiki** : Fonctionne avec tout wiki propulsé par MediaWiki
- **Édition par règles** : Définitions de règles déclaratives au format TOML
- **Système de greffons** : Extensible via des scripts Lua et des modules WebAssembly
- **Sécurité renforcée** : Exécution isolée des greffons avec limites de ressources
- **Opérations par lots** : Traitement de masse efficace des pages
- **Interface TUI interactive** : Interface terminal riche pour la surveillance et le contrôle
- **Mode simulation (dry-run)** : Prévisualisation des modifications avant leur application
- **Gestion d'état** : Reprise des sessions interrompues
- **Limitation de débit** : Respecte les limites de l'API du wiki et les bonnes pratiques
- **Internationalisation** : 45 langues d'affichage via `--lang`, `AWB_LANG` ou la locale POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`), incluant la prise en charge de la mise en page RTL pour l'arabe, le persan, l'hébreu et l'ourdou

## Langues prises en charge

La CLI, les rapports de bots et l'interface graphique GTK4 prennent en charge 45 langues (dont 6 locales RTL : `ar`, `ar-EG`, `ar-MA`, `fa`, `he` et `ur`). Utilisez `--lang <LANG>` dans la CLI, ou définissez `AWB_LANG` ou les variables de locale POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`) pour la CLI et l'interface GTK4.

Consultez le tableau complet des 45 langues prises en charge ainsi que les liens vers chaque version localisée dans [README.md](../../../README.md).

## Prérequis

- **Rust** : Version 1.94 ou supérieure (MSRV)
- **Cargo** : Inclus dans l'installation de Rust

## Installation

### Depuis les sources

```bash
# Cloner le dépôt
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Compiler tous les crates de l'espace de travail
cargo build --workspace --release

# Installer le binaire
cargo install --path crates/awb_cli
```

## Développement

### Compilation

```bash
# Compilation en mode débogage
cargo build --workspace

# Compilation en mode release
cargo build --workspace --release
```

### Tests

```bash
# Exécuter tous les tests
cargo test --workspace

# Exécuter les tests en affichant la sortie standard
cargo test --workspace -- --nocapture
```

### Analyse statique (Lint)

```bash
# Vérifier les avertissements et problèmes de qualité
cargo clippy --workspace --all-targets --all-features

# Appliquer les corrections automatiques
cargo clippy --fix --allow-dirty --workspace
```

### Formatage

```bash
# Vérifier le formatage
cargo fmt --all -- --check

# Appliquer le formatage
cargo fmt --all
```

## Exemples d'utilisation

### Authentification

```bash
# Se connecter à un wiki (stocke les identifiants de manière sécurisée)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Se connecter avec OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Listage de pages

```bash
# Lister les pages d'une catégorie
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Lister les pages à partir d'une recherche
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Lister les pages depuis un fichier
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Exécution de règles d'édition

```bash
# Exécuter des règles avec un profil (mode simulation)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Appliquer les règles à de vraies pages
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Mode bot automatisé

```bash
# Exécuter en mode bot autonome avec limites
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Reprendre une session précédente
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Exemple de profil de règles

Créez un fichier `my-rules.toml` :

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

## Système de greffons

awb-rs prend en charge deux types de greffons :

### Greffons Lua

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### Greffons WebAssembly

```rust
// Compiler vers WASM avec : cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Fonctionnalités des greffons :**
- Exécution en bac à sable avec limites de ressources (mémoire, temps CPU)
- Accès à un sous-ensemble sécurisé d'API (aucun accès au système de fichiers ni au réseau)
- Vérification de compatibilité des versions
- Rechargement à chaud (hot-reload) en mode développement

## Modèle de sécurité

- **Stockage des identifiants** : Utilise le trousseau du système d'exploitation pour stocker les mots de passe en toute sécurité
- **Isolation des greffons** : Lua et WASM s'exécutent dans des environnements isolés
- **Limites de ressources** : Limites de temps CPU, de mémoire et de nombre d'instructions par greffon
- **Limitation de débit de l'API** : Respecte les en-têtes `Retry-After` et les délais configurables
- **Validation des modifications** : Prévisualisation des changements avant application et journaux d'audit
- **Prise en charge OAuth** : Méthode d'authentification privilégiée pour les bots

## Licence

Distribué selon votre choix sous l'une des licences suivantes :

- Licence Apache, version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) ou http://www.apache.org/licenses/LICENSE-2.0)
- Licence MIT ([LICENSE-MIT](../../../LICENSE-MIT) ou http://opensource.org/licenses/MIT)

## Contribuer

Les contributions sont les bienvenues ! Veuillez consulter [CONTRIBUTING.md](../../../CONTRIBUTING.md) pour connaître les directives.

## État du projet

Ce projet est une réécriture active d'AutoWikiBrowser en Rust. État actuel :

- [x] Client API principal
- [x] Moteur de règles
- [x] Interface CLI
- [x] Système de greffons (Lua/WASM)
- [x] Bac à sable de sécurité
- [ ] Interface utilisateur en terminal (en cours)
- [ ] Interface graphique GUI (planifiée)
- [ ] Visualisation avancée des différences (planifiée)

## Liens

- [Documentation](https://docs.rs/awb-rs)
- [Suivi des tickets](https://github.com/somethingwithproof/awb-rs/issues)
- [AWB original](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
