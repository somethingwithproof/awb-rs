# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

以 Rust 重寫的現代化、高效能 AutoWikiBrowser (AWB)，著重於安全性、效能與擴充性，提供自動化的 Wikipedia 編輯能力。

## 架構

```
awb-rs/
├── awb-core/           # 核心商業邏輯與抽象層
├── awb-api/            # MediaWiki API 用戶端實作
├── awb-cli/            # 命令列介面
├── awb-rules/          # 自動編輯規則引擎
├── awb-lua-plugin/     # Lua 外掛執行階段
├── awb-wasm-plugin/    # WebAssembly 外掛執行階段
├── awb-security/       # 安全性與沙盒機制
├── awb-db/             # 持久狀態管理
├── awb-ui/             # 終端機介面元件
├── awb-fixtures/       # 測試治具與模擬資料
└── awb-integration/    # 整合測試套件
```

**資料流：**
```
CLI → 核心層 → API → MediaWiki
       ↓
    規則引擎 → 外掛 (Lua/WASM)
       ↓
    安全層 → 沙盒執行
       ↓
    資料庫 → 狀態持久化
```

## 功能特色

- **現代化架構**：採用基於 Tokio 執行階段的非同步 async/await 設計，具備高效能表現
- **多 Wiki 支援**：可與任何基於 MediaWiki 的 Wiki 站台搭配運作
- **規則式編輯**：採用宣告式 TOML 設定檔定義編輯規則
- **外掛系統**：支援透過 Lua 指令碼與 WebAssembly 模組進行擴充
- **強化安全性**：在具備資源限制的沙盒環境中執行外掛
- **批次作業**：高效率處理大量頁面
- **互動式 TUI**：提供功能豐富的終端機介面以進行監控與操作
- **預演模式**：在實際套用變更前先預覽編輯差異
- **狀態管理**：支援續傳並恢復中斷的工作階段
- **速率限制**：遵循 Wiki API 的存取限制與最佳實務
- **國際化支援**：透過 `--lang`、`AWB_LANG` 或 POSIX 地區設定（`LC_ALL`/`LC_MESSAGES`/`LANG`）支援 45 種顯示語言，包含阿拉伯文、波斯文、希伯來文與烏爾都文的 RTL 版面配置

## 支援語言

CLI、機器人報告與 GTK4 介面共支援 45 種語言（包含 6 個 RTL 地區設定：`ar`、`ar-EG`、`ar-MA`、`fa`、`he` 與 `ur`）。在 CLI 中可傳入 `--lang <LANG>`，亦可設定 `AWB_LANG` 或 POSIX 地區設定變數（`LC_ALL`/`LC_MESSAGES`/`LANG`），同時適用於 CLI 與 GTK4 介面。完整語言列表請參閱 [README.md](../../../README.md)。

## 系統需求

- **Rust**：1.94 或以上版本（MSRV）
- **Cargo**：隨 Rust 安裝一併提供

## 安裝

### 從原始碼建置

```bash
# 複製儲存庫
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# 建置工作區中的所有套件
cargo build --workspace --release

# 安裝執行檔
cargo install --path crates/awb_cli
```

## 開發

### 建置

```bash
# 偵錯建置
cargo build --workspace

# 發行建置
cargo build --workspace --release
```

### 測試

```bash
# 執行所有測試
cargo test --workspace

# 執行測試並顯示輸出內容
cargo test --workspace -- --nocapture
```

### 靜態檢查

```bash
# 檢查程式碼品質問題
cargo clippy --workspace --all-targets --all-features

# 自動套用修正
cargo clippy --fix --allow-dirty --workspace
```

### 程式碼格式化

```bash
# 檢查格式是否符合規範
cargo fmt --all -- --check

# 套用程式碼格式化
cargo fmt --all
```

## 使用範例

### 身分驗證

```bash
# 登入 Wiki（安全儲存憑證）
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# 使用 OAuth 登入
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### 列出頁面

```bash
# 從分類取得頁面清單
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# 從搜尋結果取得頁面清單
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# 從檔案讀取頁面清單
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### 執行編輯規則

```bash
# 透過設定檔執行規則（預演模式）
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# 將規則套用至實際頁面
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### 自動機器人模式

```bash
# 以設定上限的無人值守機器人模式執行
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# 從先前的工作階段繼續執行
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### 規則設定檔範例

建立 `my-rules.toml` 檔案：

```toml
[profile]
name = "清理機器人"
description = "修正縮排與常見格式問題"
edit_summary = "自動清理（機器人）"

[[rules]]
name = "修正重複空白"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "修正外部連結"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "補上缺少的分類"
plugin = "add_categories.lua"
```

## 外掛系統

awb-rs 支援兩種外掛類型：

### Lua 外掛

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### WebAssembly 外掛

```rust
// 編譯為 WASM：cargo build --target wasm32-wasip1
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**外掛特色：**
- 具資源限制（記憶體、CPU 時間）的沙盒執行環境
- 僅開放安全的 API 子集（無法存取檔案系統與網路）
- 版本相容性檢查
- 開發模式下支援熱重載

## 安全模型

- **憑證儲存**：透過作業系統金鑰圈安全儲存密碼
- **外掛沙盒**：Lua 與 WASM 皆在隔離環境中執行
- **資源限制**：針對個別外掛限制 CPU 時間、記憶體與指令數量
- **API 速率限制**：遵循 `Retry-After` 標頭並支援可自訂延遲
- **編輯驗證**：套用變更前可先預覽，並記錄稽核日誌
- **OAuth 支援**：建議用於機器人的身分驗證方式

## 授權條款

本專案採用以下任一授權條款，您可依需求擇一使用：

- Apache License, Version 2.0（[LICENSE-APACHE](../../../LICENSE-APACHE) 或 http://www.apache.org/licenses/LICENSE-2.0）
- MIT 授權條款（[LICENSE-MIT](../../../LICENSE-MIT) 或 http://opensource.org/licenses/MIT）

## 參與貢獻

歡迎各種形式的貢獻！請參閱 [CONTRIBUTING.md](../../../CONTRIBUTING.md) 以了解貢獻指南。

## 專案狀態

本專案為使用 Rust 積極重寫中的 AutoWikiBrowser。目前進度如下：

- [x] 核心 API 用戶端
- [x] 規則引擎
- [x] CLI 命令列介面
- [x] 外掛系統 (Lua/WASM)
- [x] 安全性沙盒
- [ ] 終端機介面（開發中）
- [ ] 圖形使用者介面（規劃中）
- [ ] 進階差異視覺化（規劃中）

## 相關連結

- [說明文件](https://docs.rs/awb-rs)
- [問題追蹤](https://github.com/somethingwithproof/awb-rs/issues)
- [原版 AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
