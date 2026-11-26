# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Rust で再実装された、モダンかつ高性能な AutoWikiBrowser (AWB) です。安全性、パフォーマンス、拡張性を重視し、Wikipedia の自動編集機能を提供します。

## アーキテクチャ

```
awb-rs/
├── awb-core/           # コアビジネスロジックおよび抽象化
├── awb-api/            # MediaWiki API クライアント実装
├── awb-cli/            # コマンドラインインターフェース
├── awb-rules/          # 自動編集用ルールエンジン
├── awb-lua-plugin/     # Lua プラグインランタイム
├── awb-wasm-plugin/    # WebAssembly プラグインランタイム
├── awb-security/       # セキュリティおよびサンドボックス機能
├── awb-db/             # 永続的な状態管理
├── awb-ui/             # ターミナル UI コンポーネント
├── awb-fixtures/       # テストフィクスチャおよびモックデータ
└── awb-integration/    # 結合テストスイート
```

**データフロー:**
```
CLI → コア → API → MediaWiki
       ↓
   ルールエンジン → プラグイン (Lua/WASM)
       ↓
   セキュリティ層 → サンドボックス実行
       ↓
   データベース → 状態の永続化
```

## 主な機能

- **モダンなアーキテクチャ**: Tokio ランタイムによる async/await を採用した高いパフォーマンス
- **マルチ Wiki 対応**: MediaWiki を採用したあらゆる Wiki で利用可能
- **ルールベースの編集**: TOML 形式による宣言的なルール定義
- **プラグインシステム**: Lua スクリプトおよび WebAssembly モジュールによる拡張性
- **堅牢なセキュリティ**: リソース制限を備えたサンドボックス環境でのプラグイン実行
- **バッチ処理**: 効率的な一括ページ処理
- **対話型 TUI**: 監視および制御のための高機能なターミナルインターフェース
- **ドライランモード**: 変更を適用する前に差分をプレビュー
- **状態管理**: 中断されたセッションの再開に対応
- **レート制限**: Wiki の API 制限およびベストプラクティスを遵守
- **国際化 (i18n)**: `--lang`、`AWB_LANG`、または POSIX ロケール（`LC_ALL`/`LC_MESSAGES`/`LANG`）による 45 の表示言語に対応し、アラビア語、ペルシア語、ヘブライ語、ウルドゥー語の RTL レイアウトをサポート

## 対応言語

CLI、ボットレポート、および GTK4 UI は 45 言語（6 つの RTL ロケール `ar`、`ar-EG`、`ar-MA`、`fa`、`he`、`ur` を含む）に対応しています。システムのロケール検出を上書きするには、`--lang <LANG>` を指定するか環境変数 `AWB_LANG` を設定してください。対応言語の一覧表については [README.md](../../../README.md) を参照してください。

## 前提条件

- **Rust**: バージョン 1.85 以上（MSRV）
- **Cargo**: Rust のインストールに同梱

## インストール

### ソースからのビルド

```bash
# リポジトリをクローン
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# ワークスペース内の全クレートをビルド
cargo build --workspace --release

# バイナリをインストール
cargo install --path awb-cli
```

## 開発

### ビルド

```bash
# デバッグビルド
cargo build --workspace

# リリースビルド
cargo build --workspace --release
```

### テスト

```bash
# すべてのテストを実行
cargo test --workspace

# 標準出力を表示してテストを実行
cargo test --workspace -- --nocapture
```

### リント

```bash
# リントエラーを確認
cargo clippy --workspace --all-targets --all-features

# 自動修正を適用
cargo clippy --fix --allow-dirty --workspace
```

### フォーマット

```bash
# フォーマットを確認
cargo fmt --all -- --check

# フォーマットを適用
cargo fmt --all
```

## 使用例

### 認証

```bash
# Wiki にログイン（認証情報を安全に保存）
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# OAuth でログイン
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### ページ一覧の取得

```bash
# カテゴリからページ一覧を取得
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# 検索結果からページ一覧を取得
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# ファイルからページ一覧を取得
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### 編集ルールの実行

```bash
# プロファイルを使用してルールを実行（ドライランモード）
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# 実際のページにルールを適用
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### 自動ボットモード

```bash
# 制限付きで無人ボットモードを実行
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# 前回のセッションから再開
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### ルールプロファイルの例

`my-rules.toml` ファイルを作成します。

```toml
[profile]
name = "クリーンアップボット"
description = "一般的な書式の問題を修正"
edit_summary = "自動クリーンアップ（ボット）"

[[rules]]
name = "連続するスペースの修正"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "外部リンクの修正"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "不足しているカテゴリの追加"
plugin = "add_categories.lua"
```

## プラグインシステム

awb-rs は 2 種類のプラグインをサポートしています。

### Lua プラグイン

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### WebAssembly プラグイン

```rust
// WASM へのコンパイル: cargo build --target wasm32-wasi
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**プラグインの機能:**
- リソース制限（メモリ、CPU 時間）付きのサンドボックス実行
- 安全な API サブセットへのアクセス（ファイルシステムやネットワークへのアクセス不可）
- バージョン互換性チェック
- 開発モードでのホットリロード対応

## セキュリティモデル

- **認証情報の保存**: OS のキーリングを使用してパスワードを安全に保存
- **プラグインのサンドボックス化**: Lua および WASM を隔離された環境で実行
- **リソース制限**: プラグインごとに CPU 時間、メモリ、命令数を制限
- **API レート制限**: `Retry-After` ヘッダーの遵守および設定可能な遅延時間
- **編集の検証**: 適用前の変更プレビューおよび監査ログの記録
- **OAuth サポート**: ボット向けの推奨認証方式

## ライセンス

以下のいずれかのライセンスの下で提供されています。

- Apache License, Version 2.0（[LICENSE-APACHE](../../../LICENSE-APACHE) または http://www.apache.org/licenses/LICENSE-2.0）
- MIT ライセンス（[LICENSE-MIT](../../../LICENSE-MIT) または http://opensource.org/licenses/MIT）

いずれかを選択してご利用いただけます。

## コントリビューション

コントリビューションを歓迎します。ガイドラインについては [CONTRIBUTING.md](../../../CONTRIBUTING.md) をご覧ください。

## プロジェクトの現状

本プロジェクトは Rust による AutoWikiBrowser の積極的な再実装です。現在の状況は以下のとおりです。

- [x] コア API クライアント
- [x] ルールエンジン
- [x] CLI インターフェース
- [x] プラグインシステム (Lua/WASM)
- [x] セキュリティサンドボックス
- [ ] ターミナル UI（開発中）
- [ ] GUI（計画中）
- [ ] 高度な差分表示（計画中）

## リンク

- [ドキュメント](https://docs.rs/awb-rs)
- [イシュートラッカー](https://github.com/somethingwithproof/awb-rs/issues)
- [オリジナルの AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
