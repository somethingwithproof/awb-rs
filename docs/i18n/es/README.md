# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Una reimplementación moderna y de alto rendimiento de AutoWikiBrowser (AWB) en Rust, diseñada para ofrecer edición automatizada en Wikipedia con énfasis en la seguridad, el rendimiento y la extensibilidad.

## Arquitectura

```
awb-rs/
├── awb-core/           # Lógica de negocio principal y abstracciones
├── awb-api/            # Implementación del cliente de la API de MediaWiki
├── awb-cli/            # Interfaz de línea de comandos
├── awb-rules/          # Motor de reglas para la automatización de ediciones
├── awb-lua-plugin/     # Entorno de ejecución de complementos Lua
├── awb-wasm-plugin/    # Entorno de ejecución de complementos WebAssembly
├── awb-security/       # Seguridad y aislamiento (sandboxing)
├── awb-db/             # Gestión persistente del estado
├── awb-ui/             # Componentes de interfaz de usuario en terminal
├── awb-fixtures/       # Datos de prueba y simulaciones (mocks)
└── awb-integration/    # Conjunto de pruebas de integración
```

**Flujo de datos:**
```
CLI → Core → API → MediaWiki
       ↓
    Motor de reglas → Complementos (Lua/WASM)
       ↓
    Capa de seguridad → Ejecución aislada (sandbox)
       ↓
    Base de datos → Persistencia de estado
```

## Características

- **Arquitectura moderna**: Async/await con el entorno de ejecución Tokio para un alto rendimiento
- **Soporte multi-wiki**: Compatible con cualquier wiki basado en MediaWiki
- **Edición basada en reglas**: Definiciones de reglas declarativas en formato TOML
- **Sistema de complementos**: Extensible mediante scripts Lua y módulos WebAssembly
- **Seguridad reforzada**: Ejecución aislada de complementos con límites de recursos
- **Operaciones por lotes**: Procesamiento masivo y eficiente de páginas
- **TUI interactiva**: Interfaz de terminal avanzada para supervisión y control
- **Modo de prueba (dry-run)**: Previsualización de los cambios antes de aplicarlos
- **Gestión de estado**: Reanudación de sesiones interrumpidas
- **Limitación de tasa**: Respeta los límites de la API del wiki y las buenas prácticas
- **Internacionalización**: 45 idiomas de interfaz mediante `--lang`, `AWB_LANG` o la configuración regional POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`), incluido el soporte de diseño RTL para árabe, persa, hebreo y urdu

## Idiomas compatibles

La CLI, los informes de bots y la interfaz gráfica GTK4 admiten 45 idiomas (incluidas 6 configuraciones regionales RTL: `ar`, `ar-EG`, `ar-MA`, `fa`, `he` y `ur`). Use `--lang <LANG>` en la CLI, o establezca `AWB_LANG` o las variables de configuración regional POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`) tanto para la CLI como para la interfaz GTK4.

Consulte la tabla completa de los 45 idiomas compatibles y los enlaces a cada documentación localizada en [README.md](../../../README.md).

## Requisitos previos

- **Rust**: Versión 1.94 o superior (MSRV)
- **Cargo**: Incluido con la instalación de Rust

## Instalación

### Desde el código fuente

```bash
# Clonar el repositorio
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Compilar todos los crates del espacio de trabajo
cargo build --workspace --release

# Instalar el binario
cargo install --path crates/awb_cli
```

## Desarrollo

### Compilación

```bash
# Compilación de depuración
cargo build --workspace

# Compilación de lanzamiento
cargo build --workspace --release
```

### Pruebas

```bash
# Ejecutar todas las pruebas
cargo test --workspace

# Ejecutar las pruebas mostrando la salida estándar
cargo test --workspace -- --nocapture
```

### Análisis estático (Lint)

```bash
# Comprobar problemas de estilo y advertencias
cargo clippy --workspace --all-targets --all-features

# Aplicar correcciones automáticas
cargo clippy --fix --allow-dirty --workspace
```

### Formato

```bash
# Comprobar el formato del código
cargo fmt --all -- --check

# Aplicar el formato al código
cargo fmt --all
```

## Ejemplos de uso

### Autenticación

```bash
# Iniciar sesión en un wiki (almacena las credenciales de forma segura)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Iniciar sesión con OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Listado de páginas

```bash
# Listar páginas de una categoría
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Listar páginas a partir de una búsqueda
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Listar páginas desde un archivo
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Ejecución de reglas de edición

```bash
# Ejecutar reglas con un perfil (modo de prueba)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Aplicar reglas a páginas reales
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Modo bot automatizado

```bash
# Ejecutar en modo bot desatendido con límites
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Reanudar desde una sesión anterior
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Perfil de reglas de ejemplo

Cree un archivo `my-rules.toml`:

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

## Sistema de complementos

awb-rs admite dos tipos de complementos:

### Complementos Lua

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### Complementos WebAssembly

```rust
// Compilar a WASM con: cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Características de los complementos:**
- Ejecución aislada en sandbox con límites de recursos (memoria, tiempo de CPU)
- Acceso a un subconjunto seguro de APIs (sin acceso al sistema de archivos ni a la red)
- Verificación de compatibilidad de versiones
- Recarga en caliente (hot-reload) en modo de desarrollo

## Modelo de seguridad

- **Almacenamiento de credenciales**: Utiliza el depósito de claves del sistema operativo para almacenar contraseñas de forma segura
- **Aislamiento de complementos**: Lua y WASM se ejecutan en entornos aislados
- **Límites de recursos**: Límites de tiempo de CPU, memoria y número de instrucciones por complemento
- **Limitación de tasa de la API**: Respeta las cabeceras `Retry-After` y los retrasos configurables
- **Validación de ediciones**: Previsualización de cambios antes de aplicarlos y registros de auditoría
- **Soporte de OAuth**: Método de autenticación recomendado para bots

## Licencia

Distribuido bajo cualquiera de las siguientes licencias, a su elección:

- Licencia Apache, versión 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) o http://www.apache.org/licenses/LICENSE-2.0)
- Licencia MIT ([LICENSE-MIT](../../../LICENSE-MIT) o http://opensource.org/licenses/MIT)

## Contribuciones

Las contribuciones son bienvenidas. Consulte [CONTRIBUTING.md](../../../CONTRIBUTING.md) para conocer las directrices.

## Estado del proyecto

Esta es una reimplementación activa de AutoWikiBrowser en Rust. Estado actual:

- [x] Cliente principal de la API
- [x] Motor de reglas
- [x] Interfaz CLI
- [x] Sistema de complementos (Lua/WASM)
- [x] Aislamiento de seguridad (sandboxing)
- [ ] Interfaz de usuario en terminal (en curso)
- [ ] Interfaz gráfica de usuario (planificada)
- [ ] Visualización avanzada de diferencias (planificada)

## Enlaces

- [Documentación](https://docs.rs/awb-rs)
- [Seguimiento de incidencias](https://github.com/somethingwithproof/awb-rs/issues)
- [AWB original](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
