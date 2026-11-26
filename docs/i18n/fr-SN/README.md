# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Une réécriture moderne et performante d'AutoWikiBrowser (AWB) en Rust, conçue pour l'édition automatisée sur Wikipédia en mettant l'accent sur la fiabilité, la performance et l'extensibilité.

## Architecture

```
awb-rs/
├── awb-core/           # Logique métier centrale et abstractions
├── awb-api/            # Implémentation du client de l'API MediaWiki
├── awb-cli/            # Interface en ligne de commande
├── awb-rules/          # Moteur de règles pour l'automatisation des éditions
├── awb-lua-plugin/     # Moteur d'exécution pour les plugins Lua
├── awb-wasm-plugin/    # Moteur d'exécution pour les plugins WebAssembly
├── awb-security/       # Sécurité et cloisonnement (sandboxing)
├── awb-db/             # Gestion de la persistance des états
├── awb-ui/             # Composants d'interface terminal (TUI)
├── awb-fixtures/       # Données de test et simulations (mocks)
└── awb-integration/    # Suite de tests d'intégration
```

**Flux de données :**
```
CLI → Cœur → API → MediaWiki
        ↓
     Moteur de règles → Plugins (Lua/WASM)
        ↓
     Couche sécurité → Exécution cloisonnée
        ↓
     Base de données → Persistance d'état
```

## Fonctionnalités

- **Architecture moderne** : Modèle asynchrone (`async/await`) reposant sur le runtime Tokio pour des performances optimales
- **Compatibilité multi-wiki** : Fonctionne avec tout wiki basé sur MediaWiki
- **Édition pilotée par règles** : Règles déclaratives définies dans des fichiers TOML
- **Système d'extensions** : Extensible au moyen de scripts Lua et de modules WebAssembly
- **Sécurité renforcée** : Exécution cloisonnée des plugins avec plafonnement des ressources
- **Traitement par lots** : Exécution rapide d'opérations sur un grand volume de pages
- **Interface TUI interactive** : Interface en mode terminal riche pour la supervision et le pilotage
- **Mode essai (`dry-run`)** : Prévisualisation des modifications avant leur publication effective
- **Gestion des sessions** : Reprise aisée des sessions interrompues
- **Régulation des requêtes** : Respect des limites de l'API wiki et des bonnes pratiques d'exploitation
- **Internationalisation** : 45 langues d'interface sélectionnables via `--lang`, `AWB_LANG` ou les variables locales POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`), avec prise en charge de l'affichage de droite à gauche (RTL) pour l'arabe, le persan, l'hébreu et l'ourdou

## Langues prises en charge

La ligne de commande, les rapports de bot et l'interface GTK4 prennent en charge 45 langues (incluant 6 locales RTL : `ar`, `ar-EG`, `ar-MA`, `fa`, `he` et `ur`). Utilisez `--lang <LANG>` en ligne de commande, ou définissez `AWB_LANG` ou les variables POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`) pour la CLI et l'interface GTK4. Reportez-vous à [README.md](../../../README.md) pour consulter le tableau complet des langues.

## Prérequis

- **Rust** : Version 1.94 ou ultérieure (MSRV)
- **Cargo** : Fourni avec l'installation de Rust

## Installation

### Compilation depuis les sources

```bash
# Cloner le dépôt
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Compiler toutes les crates de l'espace de travail
cargo build --workspace --release

# Installer le binaire
cargo install --path crates/awb_cli
```

## Développement

### Compilation

```bash
# Compilation de débogage
cargo build --workspace

# Compilation optimisée (release)
cargo build --workspace --release
```

### Tests

```bash
# Lancer tous les tests
cargo test --workspace

# Lancer les tests en affichant la sortie standard
cargo test --workspace -- --nocapture
```

### Vérification statique (Lint)

```bash
# Détecter les problèmes de qualité de code
cargo clippy --workspace --all-targets --all-features

# Appliquer les corrections automatiques
cargo clippy --fix --allow-dirty --workspace
```

### Mise en forme

```bash
# Contrôler le formatage du code
cargo fmt --all -- --check

# Appliquer le formatage
cargo fmt --all
```

## Exemples d'utilisation

### Authentification

```bash
# Se connecter à un wiki (enregistrement sécurisé des identifiants)
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

### Énumération de pages

```bash
# Lister les pages appartenant à une catégorie
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Lister les pages issues d'une recherche
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

### Exécution des règles d'édition

```bash
# Exécuter un profil de règles en mode essai (dry-run)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Appliquer les règles sur les pages cibles
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Mode bot automatisé

```bash
# Lancer en mode bot autonome avec seuils de contrôle
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Reprendre à partir d'une session enregistrée
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
description = "Corriger les erreurs courantes de mise en forme"
edit_summary = "Nettoyage automatique (Bot)"

[[rules]]
name = "Supprimer les espaces doubles"
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

awb-rs prend en charge deux catégories de plugins :

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
// Compiler en WASM avec : cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Caractéristiques des plugins :**
- Exécution cloisonnée avec plafonds de ressources (mémoire, temps CPU)
- Accès restreint à un sous-ensemble sûr d'API (sans accès au système de fichiers ni au réseau)
- Contrôle de compatibilité entre versions
- Rechargement à chaud pris en charge en mode développement

## Modèle de sécurité

- **Stockage des identifiants** : Recours au gestionnaire de mots de passe du système d'exploitation pour une conservation sécurisée
- **Cloisonnement des plugins** : Exécution des plugins Lua et WASM dans des environnements isolés
- **Limitation des ressources** : Plafonds de temps CPU, de mémoire et de nombre d'instructions pour chaque plugin
- **Régulation des appels API** : Prise en compte des en-têtes `Retry-After` et des temporisations paramétrables
- **Validation des éditions** : Prévisualisation des changements avant enregistrement et journaux d'audit
- **Prise en charge d'OAuth** : Méthode d'authentification recommandée pour les comptes de bots

## Licence

Ce projet est distribué, selon votre choix, sous l'une des licences suivantes :

- Licence Apache, version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) ou http://www.apache.org/licenses/LICENSE-2.0)
- Licence MIT ([LICENSE-MIT](../../../LICENSE-MIT) ou http://opensource.org/licenses/MIT)

## Contribuer

Les contributions sont chaleureusement accueillies ! Consultez [CONTRIBUTING.md](../../../CONTRIBUTING.md) pour prendre connaissance des règles de contribution.

## État d'avancement du projet

Il s'agit d'une réécriture active d'AutoWikiBrowser en Rust. État actuel :

- [x] Client API principal
- [x] Moteur de règles
- [x] Interface en ligne de commande (CLI)
- [x] Système de plugins (Lua/WASM)
- [x] Cloisonnement de sécurité
- [ ] Interface terminal (en cours)
- [ ] Interface graphique (prévu)
- [ ] Visualisation avancée des diffs (prévu)

## Liens

- [Documentation](https://docs.rs/awb-rs)
- [Gestionnaire de tickets](https://github.com/somethingwithproof/awb-rs/issues)
- [Projet AWB original](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
