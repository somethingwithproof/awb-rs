# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Uma reescrita moderna e de alto desempenho do AutoWikiBrowser (AWB) em Rust, oferecendo recursos automatizados de edição para a Wikipédia com foco em segurança, desempenho e extensibilidade.

## Arquitetura

```
awb-rs/
├── awb-core/           # Lógica de negócios principal e abstrações
├── awb-api/            # Implementação do cliente da API do MediaWiki
├── awb-cli/            # Interface de linha de comando
├── awb-rules/          # Motor de regras para automação de edições
├── awb-lua-plugin/     # Runtime de plugins Lua
├── awb-wasm-plugin/    # Runtime de plugins WebAssembly
├── awb-security/       # Segurança e isolamento (sandboxing)
├── awb-db/             # Gerenciamento persistente de estado
├── awb-ui/             # Componentes de interface de usuário no terminal
├── awb-fixtures/       # Fixtures de teste e dados simulados (mocks)
└── awb-integration/    # Suíte de testes de integração
```

**Fluxo de dados:**
```
CLI → Core → API → MediaWiki
       ↓
    Motor de regras → Plugins (Lua/WASM)
       ↓
    Camada de segurança → Execução em sandbox
       ↓
    Banco de dados → Persistência de estado
```

## Recursos

- **Arquitetura moderna**: Async/await com runtime Tokio para alto desempenho
- **Suporte multi-wiki**: Compatível com qualquer wiki baseado em MediaWiki
- **Edição baseada em regras**: Definições declarativas de regras em TOML
- **Sistema de plugins**: Extensível via scripts Lua e módulos WebAssembly
- **Segurança reforçada**: Execução isolada de plugins com limites de recursos
- **Operações em lote**: Processamento eficiente de páginas em massa
- **TUI interativa**: Interface de terminal completa para monitoramento e controle
- **Modo de simulação (dry-run)**: Pré-visualize alterações antes de aplicá-las
- **Gerenciamento de estado**: Retome sessões interrompidas
- **Limitação de taxa**: Respeita os limites da API do wiki e as boas práticas
- **Internacionalização**: 45 idiomas de exibição via `--lang`, `AWB_LANG` ou locale POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`), incluindo suporte a layout RTL para árabe, persa, hebraico e urdu

## Idiomas suportados

A CLI, os relatórios de bots e a interface gráfica GTK4 suportam 45 idiomas (incluindo 6 locales RTL: `ar`, `ar-EG`, `ar-MA`, `fa`, `he` e `ur`). Use `--lang <LANG>` na CLI ou defina `AWB_LANG` ou variáveis de locale POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`) tanto para a CLI quanto para a interface GTK4.

Consulte a tabela completa dos 45 idiomas suportados e os links para cada documentação localizada em [README.md](../../../README.md).

## Pré-requisitos

- **Rust**: Versão 1.94 ou superior (MSRV)
- **Cargo**: Incluído na instalação do Rust

## Instalação

### A partir do código-fonte

```bash
# Clonar o repositório
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Compilar todos os crates do workspace
cargo build --workspace --release

# Instalar o binário
cargo install --path crates/awb_cli
```

## Desenvolvimento

### Compilação

```bash
# Compilação de depuração (debug)
cargo build --workspace

# Compilação de produção (release)
cargo build --workspace --release
```

### Testes

```bash
# Executar todos os testes
cargo test --workspace

# Executar testes exibindo a saída padrão
cargo test --workspace -- --nocapture
```

### Análise estática (Lint)

```bash
# Verificar problemas de lint
cargo clippy --workspace --all-targets --all-features

# Aplicar correções automáticas
cargo clippy --fix --allow-dirty --workspace
```

### Formatação

```bash
# Verificar a formatação
cargo fmt --all -- --check

# Aplicar a formatação
cargo fmt --all
```

## Exemplos de uso

### Autenticação

```bash
# Fazer login em um wiki (armazena credenciais com segurança)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Fazer login com OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Listagem de páginas

```bash
# Listar páginas de uma categoria
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Listar páginas a partir de uma busca
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Listar páginas a partir de um arquivo
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Execução de regras de edição

```bash
# Executar regras com um perfil (modo dry-run)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Aplicar regras a páginas reais
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Modo bot automatizado

```bash
# Executar em modo bot autônomo com limites
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Retomar de uma sessão anterior
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Exemplo de perfil de regras

Crie um arquivo `my-rules.toml`:

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

## Sistema de plugins

O awb-rs suporta dois tipos de plugins:

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
// Compilar para WASM com: cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Recursos de plugins:**
- Execução em sandbox com limites de recursos (memória, tempo de CPU)
- Acesso a um subconjunto seguro de APIs (sem acesso ao sistema de arquivos ou à rede)
- Verificação de compatibilidade de versões
- Suporte a recarga dinâmica (hot-reload) em modo de desenvolvimento

## Modelo de segurança

- **Armazenamento de credenciais**: Utiliza o chaveiro do sistema operacional para armazenamento seguro de senhas
- **Isolamento de plugins**: Lua e WASM são executados em ambientes isolados
- **Limites de recursos**: Limites de tempo de CPU, memória e contagem de instruções por plugin
- **Limitação de taxa da API**: Respeita cabeçalhos `Retry-After` e intervalos configuráveis
- **Validação de edições**: Pré-visualização de alterações antes da aplicação e registros de auditoria
- **Suporte a OAuth**: Método de autenticação preferencial para bots

## Licença

Licenciado sob qualquer uma das seguintes licenças, à sua escolha:

- Licença Apache, versão 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) ou http://www.apache.org/licenses/LICENSE-2.0)
- Licença MIT ([LICENSE-MIT](../../../LICENSE-MIT) ou http://opensource.org/licenses/MIT)

## Contribuições

Contribuições são bem-vindas! Consulte [CONTRIBUTING.md](../../../CONTRIBUTING.md) para ver as diretrizes.

## Status do projeto

Esta é uma reescrita ativa do AutoWikiBrowser em Rust. Status atual:

- [x] Cliente principal da API
- [x] Motor de regras
- [x] Interface CLI
- [x] Sistema de plugins (Lua/WASM)
- [x] Isolamento de segurança (sandboxing)
- [ ] Interface de usuário no terminal (em andamento)
- [ ] Interface gráfica GUI (planejada)
- [ ] Visualização avançada de diffs (planejada)

## Links

- [Documentação](https://docs.rs/awb-rs)
- [Rastreador de issues](https://github.com/somethingwithproof/awb-rs/issues)
- [AWB original](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
