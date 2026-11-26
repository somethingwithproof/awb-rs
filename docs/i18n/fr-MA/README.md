# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Réécriture moderne et haute performance d'AutoWikiBrowser (AWB) en Rust, offrant des capacités d'édition automatisée pour Wikipédia avec une attention particulière portée à la sûreté, aux performances et à l'extensibilité.

## Architecture

```
awb-rs/
├── awb-core/           # Logique métier principale et abstractions
├── awb-api/            # Implémentation du client API MediaWiki
├── awb-cli/            # Interface en ligne de commande
├── awb-rules/          # Moteur de règles pour l'automatisation des modifications
├── awb-lua-plugin/     # Environnement d'exécution des plugins Lua
├── awb-wasm-plugin/    # Environnement d'exécution des plugins WebAssembly
├── awb-security/       # Sécurité et isolation en bac à sable
├── awb-db/             # Gestion de la persistance d'état
├── awb-ui/             # Composants d'interface utilisateur terminal (TUI)
├── awb-fixtures/       # Jeux de données de test et données simulées
└── awb-integration/    # Suite de tests d'intégration
```

**Flux de données :**
```
CLI → Noyau → API → MediaWiki
        ↓
     Moteur de règles → Plugins (Lua/WASM)
        ↓
     Couche de sécurité → Exécution en bac à sable
        ↓
     Base de données → Persistance d'état
```

## Fonctionnalités

- **Architecture moderne** : Programmation asynchrone (`async/await`) avec l'environnement d'exécution Tokio pour des performances élevées
- **Prise en charge multi-wiki** : Compatible avec tout wiki propulsé par MediaWiki
- **Édition basée sur des règles** : Définitions déclaratives des règles au format TOML
- **Système de plugins** : Extensible via des scripts Lua et des modules WebAssembly
- **Sécurité renforcée** : Exécution des plugins en bac à sable avec limites de ressources
- **Opérations par lots** : Traitement efficace de pages en masse
- **Interface TUI interactive** : Interface terminal complète pour le suivi et le contrôle
- **Mode simulation (`dry-run`)** : Prévisualisation des modifications avant leur application
- **Gestion de l'état** : Reprise des sessions interrompues
- **Limitation de débit** : Respect des quotas de l'API wiki et des bonnes pratiques
- **Internationalisation** : 45 langues d'affichage via `--lang`, `AWB_LANG` ou les paramètres régionaux POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`), incluant la prise en charge de la mise en page de droite à gauche (RTL) pour l'arabe, le persan, l'hébreu et l'ourdou

## Langues prises en charge

L'interface en ligne de commande, les rapports de bot et l'interface graphique GTK4 prennent en charge 45 langues (dont 6 locales RTL : `ar`, `ar-EG`, `ar-MA`, `fa`, `he` et `ur`). Utilisez l'option `--lang <LANG>` en ligne de commande, ou définissez `AWB_LANG` ou les variables de locale POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`) pour la CLI et l'interface GTK4. Consultez [README.md](../../../README.md) pour le tableau complet des langues prises en charge.

## Prérequis

- **Rust** : Version 1.94 ou supérieure (MSRV)
- **Cargo** : Inclus avec l'installation de Rust

## Installation

### À partir des sources

```bash
# Cloner le dépôt
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Compiler l'ensemble des crates de l'espace de travail
cargo build --workspace --release

# Installer l'exécutable
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
# Exécuter l'ensemble des tests
cargo test --workspace

# Exécuter les tests avec affichage complet de la sortie
cargo test --workspace -- --nocapture
```

### Analyse statique (Lint)

```bash
# Vérifier les avertissements et erreurs d'analyse statique
cargo clippy --workspace --all-targets --all-features

# Appliquer les corrections automatiques
cargo clippy --fix --allow-dirty --workspace
```

### Formatage

```bash
# Vérifier le formatage du code
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

# Se connecter via OAuth
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

# Lister les pages à partir d'un fichier
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Exécution des règles d'édition

```bash
# Exécuter des règles avec un profil (mode simulation)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Appliquer les règles aux pages réelles
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Mode bot automatisé

```bash
# Exécuter en mode bot autonome avec des limites
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
description = "Correction des problèmes courants de mise en forme"
edit_summary = "Nettoyage automatisé (Bot)"

[[rules]]
name = "Corriger les doubles espaces"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "Corriger les liens externes"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "Ajouter les catégories manquantes"
plugin = "add_categories.lua"
```

## Système de plugins

awb-rs prend en charge deux types de plugins :

### Plugins Lua

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### Plugins WebAssembly

```rust
// Compiler vers WASM avec : cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Fonctionnalités des plugins :**
- Exécution en bac à sable avec limites de ressources (mémoire, temps CPU)
- Accès à un sous-ensemble sécurisé d'API (aucun accès au système de fichiers ni au réseau)
- Vérification de la compatibilité des versions
- Prise en charge du rechargement à chaud en mode développement

## Modèle de sécurité

- **Stockage des identifiants** : Utilisation du trousseau du système d'exploitation pour le stockage sécurisé des mots de passe
- **Isolation des plugins** : Exécution des scripts Lua et modules WASM dans des environnements isolés
- **Limites de ressources** : Quotas de temps CPU, de mémoire et de nombre d'instructions par plugin
- **Limitation de débit de l'API** : Respect des en-têtes `Retry-After` et des délais configurables
- **Validation des modifications** : Prévisualisation des changements avant application et journaux d'audit
- **Prise en charge d'OAuth** : Méthode d'authentification privilégiée pour les bots

## Licence

Distribué au choix sous l'une des deux licences suivantes :

- Licence Apache, version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) ou http://www.apache.org/licenses/LICENSE-2.0)
- Licence MIT ([LICENSE-MIT](../../../LICENSE-MIT) ou http://opensource.org/licenses/MIT)

selon votre convenance.

## Contribuer

Les contributions sont les bienvenues ! Veuillez consulter [CONTRIBUTING.md](../../../CONTRIBUTING.md) pour les directives de contribution.

## État du projet

Ce projet est une réécriture active d'AutoWikiBrowser en Rust. État actuel :

- [x] Client API principal
- [x] Moteur de règles
- [x] Interface en ligne de commande (CLI)
- [x] Système de plugins (Lua/WASM)
- [x] Isolation de sécurité en bac à sable
- [ ] Interface utilisateur terminal (en cours)
- [ ] Interface graphique (planifié)
- [ ] Visualisation avancée des différences (planifié)

## Liens

- [Documentation](https://docs.rs/awb-rs)
- [Suivi des anomalies](https://github.com/somethingwithproof/awb-rs/issues)
- [AWB d'origine](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
