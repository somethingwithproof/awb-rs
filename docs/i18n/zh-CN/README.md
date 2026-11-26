# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

基于 Rust 重写的现代化、高性能 AutoWikiBrowser (AWB)，专注于安全性、性能与可扩展性，提供自动化的 Wikipedia 编辑能力。

## 架构

```
awb-rs/
├── awb-core/           # 核心业务逻辑与抽象层
├── awb-api/            # MediaWiki API 客户端实现
├── awb-cli/            # 命令行界面
├── awb-rules/          # 自动化编辑规则引擎
├── awb-lua-plugin/     # Lua 插件运行时
├── awb-wasm-plugin/    # WebAssembly 插件运行时
├── awb-security/       # 安全与沙箱机制
├── awb-db/             # 持久化状态管理
├── awb-ui/             # 终端界面组件
├── awb-fixtures/       # 测试夹具与模拟数据
└── awb-integration/    # 集成测试套件
```

**数据流：**
```
CLI → 核心层 → API → MediaWiki
       ↓
    规则引擎 → 插件 (Lua/WASM)
       ↓
    安全层 → 沙箱执行
       ↓
    数据库 → 状态持久化
```

## 功能特性

- **现代化架构**：基于 Tokio 运行时的 async/await 异步设计，性能卓越
- **多 Wiki 支持**：适用于所有基于 MediaWiki 的 Wiki 站点
- **基于规则的编辑**：采用声明式 TOML 格式定义编辑规则
- **插件系统**：支持通过 Lua 脚本与 WebAssembly 模块进行扩展
- **安全加固**：在受资源限制的沙箱环境中执行插件
- **批量操作**：高效的页面批处理能力
- **交互式 TUI**：提供丰富的终端界面用于监控与控制
- **试运行模式**：在正式应用更改前预览编辑效果
- **状态管理**：支持恢复中断的会话
- **速率限制**：遵守 Wiki API 限制与最佳实践
- **国际化支持**：通过 `--lang`、`AWB_LANG` 或 POSIX 区域设置（`LC_ALL`/`LC_MESSAGES`/`LANG`）支持 45 种显示语言，包括阿拉伯语、波斯语、希伯来语和乌尔都语的 RTL 布局支持

## 支持的语言

CLI、机器人报告以及 GTK4 界面共支持 45 种语言（包含 6 个 RTL 区域设置：`ar`、`ar-EG`、`ar-MA`、`fa`、`he` 和 `ur`）。可通过传入 `--lang <LANG>` 或设置 `AWB_LANG` 覆盖系统语言检测。完整语言列表请参阅 [README.md](../../../README.md)。

## 环境要求

- **Rust**：1.85 或更高版本（MSRV）
- **Cargo**：随 Rust 安装自动附带

## 安装

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# 构建工作区中的所有 crate
cargo build --workspace --release

# 安装二进制程序
cargo install --path awb-cli
```

## 开发

### 构建

```bash
# 调试构建
cargo build --workspace

# 发布构建
cargo build --workspace --release
```

### 测试

```bash
# 运行全部测试
cargo test --workspace

# 运行测试并显示标准输出
cargo test --workspace -- --nocapture
```

### 代码检查

```bash
# 检查代码规范问题
cargo clippy --workspace --all-targets --all-features

# 自动修复可修复的问题
cargo clippy --fix --allow-dirty --workspace
```

### 代码格式化

```bash
# 检查代码格式
cargo fmt --all -- --check

# 应用代码格式
cargo fmt --all
```

## 使用示例

### 身份认证

```bash
# 登录 Wiki（安全存储凭据）
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# 使用 OAuth 登录
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### 获取页面列表

```bash
# 从分类获取页面列表
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# 从搜索结果获取页面列表
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# 从文件读取页面列表
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### 运行编辑规则

```bash
# 使用配置文件运行规则（试运行模式）
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# 将规则应用到实际页面
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### 自动机器人模式

```bash
# 以无人值守的机器人模式运行并设置限制
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# 从先前的会话恢复
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### 规则配置文件示例

创建 `my-rules.toml` 文件：

```toml
[profile]
name = "清理机器人"
description = "修复常见格式问题"
edit_summary = "自动清理（机器人）"

[[rules]]
name = "修复多余空格"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "修复外部链接"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "补充缺失分类"
plugin = "add_categories.lua"
```

## 插件系统

awb-rs 支持两种类型的插件：

### Lua 插件

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### WebAssembly 插件

```rust
// 使用以下命令编译为 WASM：cargo build --target wasm32-wasi
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**插件特性：**
- 带资源限制（内存、CPU 时间）的沙箱执行
- 仅可访问安全的 API 子集（禁止访问文件系统与网络）
- 版本兼容性检查
- 开发模式下支持热重载

## 安全模型

- **凭据存储**：使用操作系统密钥环安全存储密码
- **插件沙箱**：Lua 与 WASM 在隔离环境中运行
- **资源限制**：为每个插件限制 CPU 时间、内存和指令数
- **API 速率限制**：遵守 `Retry-After` 响应头并支持可配置延迟
- **编辑校验**：应用更改前支持预览，并提供审计日志
- **OAuth 支持**：面向机器人的首选认证方式

## 许可证

本项目采用以下任一许可证授权，您可任选其一：

- Apache License, Version 2.0（[LICENSE-APACHE](../../../LICENSE-APACHE) 或 http://www.apache.org/licenses/LICENSE-2.0）
- MIT 许可证（[LICENSE-MIT](../../../LICENSE-MIT) 或 http://opensource.org/licenses/MIT）

## 参与贡献

欢迎参与贡献！请参阅 [CONTRIBUTING.md](../../../CONTRIBUTING.md) 了解贡献指南。

## 项目状态

本项目正在使用 Rust 积极重写 AutoWikiBrowser。当前进展如下：

- [x] 核心 API 客户端
- [x] 规则引擎
- [x] CLI 命令行界面
- [x] 插件系统 (Lua/WASM)
- [x] 安全沙箱
- [ ] 终端界面（进行中）
- [ ] 图形用户界面（规划中）
- [ ] 高级差异可视化（规划中）

## 相关链接

- [文档](https://docs.rs/awb-rs)
- [问题追踪](https://github.com/somethingwithproof/awb-rs/issues)
- [原版 AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
