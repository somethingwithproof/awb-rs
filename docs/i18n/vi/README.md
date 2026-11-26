# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Phiên bản viết lại hiện đại, hiệu năng cao của AutoWikiBrowser (AWB) bằng ngôn ngữ Rust, cung cấp khả năng biên tập Wikipedia tự động với trọng tâm về độ an toàn, hiệu năng và khả năng mở rộng.

## Kiến trúc

```
awb-rs/
├── awb-core/           # Logic nghiệp vụ cốt lõi và các lớp trừu tượng
├── awb-api/            # Triển khai client cho MediaWiki API
├── awb-cli/            # Giao diện dòng lệnh
├── awb-rules/          # Công cụ quy tắc cho tự động hóa biên tập
├── awb-lua-plugin/     # Môi trường thực thi plugin Lua
├── awb-wasm-plugin/    # Môi trường thực thi plugin WebAssembly
├── awb-security/       # Bảo mật và cô lập sandbox
├── awb-db/             # Quản lý trạng thái bền vững
├── awb-ui/             # Các thành phần giao diện dòng lệnh tương tác (TUI)
├── awb-fixtures/       # Dữ liệu mẫu và giả lập cho kiểm thử
└── awb-integration/    # Bộ kiểm thử tích hợp
```

**Luồng dữ liệu:**
```
CLI → Lõi xử lý → API → MediaWiki
       ↓
    Công cụ quy tắc → Plugin (Lua/WASM)
       ↓
    Lớp bảo mật → Thực thi trong sandbox
       ↓
    Cơ sở dữ liệu → Lưu trữ trạng thái bền vững
```

## Tính năng

- **Kiến trúc hiện đại**: Bất đồng bộ async/await với runtime Tokio cho hiệu năng cao
- **Hỗ trợ đa Wiki**: Hoạt động với bất kỳ trang wiki nào sử dụng MediaWiki
- **Biên tập theo quy tắc**: Định nghĩa quy tắc khai báo dựa trên định dạng TOML
- **Hệ thống plugin**: Có thể mở rộng thông qua kịch bản Lua và mô-đun WebAssembly
- **Bảo mật tăng cường**: Thực thi plugin trong môi trường sandbox có giới hạn tài nguyên
- **Xử lý hàng loạt**: Xử lý hiệu quả số lượng lớn trang
- **TUI tương tác**: Giao diện dòng lệnh phong phú để giám sát và điều khiển
- **Chế độ chạy thử (Dry-Run)**: Xem trước thay đổi trước khi áp dụng thực tế
- **Quản lý trạng thái**: Tiếp tục các phiên làm việc bị gián đoạn
- **Giới hạn tần suất**: Tuân thủ giới hạn API của wiki và các thực hành tốt nhất
- **Quốc tế hóa**: Hỗ trợ 45 ngôn ngữ hiển thị thông qua `--lang`, `AWB_LANG` hoặc cấu hình locale POSIX (`LC_ALL`/`LC_MESSAGES`/`LANG`), bao gồm hỗ trợ bố cục RTL cho tiếng Ả Rập, tiếng Ba Tư, tiếng Do Thái và tiếng Urdu

## Ngôn ngữ được hỗ trợ

Giao diện dòng lệnh, báo cáo bot và giao diện GTK4 hỗ trợ 45 ngôn ngữ (bao gồm 6 ngôn ngữ RTL: `ar`, `ar-EG`, `ar-MA`, `fa`, `he` và `ur`). Truyền tham số `--lang <LANG>` hoặc thiết lập biến môi trường `AWB_LANG` để ghi đè cơ chế tự nhận diện ngôn ngữ hệ thống. Xem bảng ngôn ngữ đầy đủ tại [README.md](../../../README.md).

## Yêu cầu hệ thống

- **Rust**: Phiên bản 1.85 trở lên (MSRV)
- **Cargo**: Đi kèm sẵn khi cài đặt Rust

## Cài đặt

### Biên dịch từ mã nguồn

```bash
# Sao chép kho mã nguồn
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# Biên dịch tất cả các crate trong workspace
cargo build --workspace --release

# Cài đặt tệp thực thi
cargo install --path awb-cli
```

## Phát triển

### Biên dịch

```bash
# Bản dựng gỡ lỗi (debug)
cargo build --workspace

# Bản dựng phát hành (release)
cargo build --workspace --release
```

### Kiểm thử

```bash
# Chạy tất cả bài kiểm thử
cargo test --workspace

# Chạy kiểm thử và hiển thị đầu ra
cargo test --workspace -- --nocapture
```

### Kiểm tra mã tĩnh (Lint)

```bash
# Kiểm tra các vấn đề chất lượng mã
cargo clippy --workspace --all-targets --all-features

# Tự động áp dụng các bản sửa lỗi
cargo clippy --fix --allow-dirty --workspace
```

### Định dạng mã

```bash
# Kiểm tra định dạng mã nguồn
cargo fmt --all -- --check

# Áp dụng định dạng mã nguồn
cargo fmt --all
```

## Ví dụ sử dụng

### Xác thực

```bash
# Đăng nhập vào wiki (lưu trữ thông tin đăng nhập an toàn)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# Đăng nhập bằng OAuth
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### Liệt kê trang

```bash
# Liệt kê các trang từ một thể loại
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# Liệt kê các trang từ kết quả tìm kiếm
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# Liệt kê các trang từ tệp văn bản
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### Chạy quy tắc biên tập

```bash
# Chạy quy tắc với tệp hồ sơ (chế độ chạy thử)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# Áp dụng quy tắc lên các trang thực tế
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### Chế độ Bot tự động

```bash
# Chạy ở chế độ bot tự động với các giới hạn an toàn
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# Tiếp tục từ phiên làm việc trước đó
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### Ví dụ hồ sơ quy tắc

Tạo tệp `my-rules.toml`:

```toml
[profile]
name = "Bot dọn dẹp"
description = "Sửa các lỗi định dạng phổ biến"
edit_summary = "Tự động dọn dẹp (Bot)"

[[rules]]
name = "Sửa khoảng trắng thừa"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "Sửa liên kết ngoài"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "Thêm thể loại còn thiếu"
plugin = "add_categories.lua"
```

## Hệ thống plugin

awb-rs hỗ trợ hai loại plugin:

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
// Biên dịch sang WASM bằng lệnh: cargo build --target wasm32-wasi
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**Tính năng của plugin:**
- Thực thi trong sandbox với giới hạn tài nguyên (bộ nhớ, thời gian CPU)
- Chỉ truy cập vào tập con API an toàn (không có quyền truy cập hệ thống tệp hay mạng)
- Kiểm tra tính tương thích phiên bản
- Hỗ trợ nạp lại nóng (hot-reload) trong chế độ phát triển

## Mô hình bảo mật

- **Lưu trữ thông tin xác thực**: Sử dụng keyring của hệ điều hành để lưu mật khẩu an toàn
- **Cô lập plugin**: Lua và WASM chạy trong các môi trường được cô lập riêng biệt
- **Giới hạn tài nguyên**: Giới hạn thời gian CPU, bộ nhớ và số lượng lệnh cho mỗi plugin
- **Giới hạn tần suất API**: Tuân thủ header `Retry-After` và khoảng trễ có thể cấu hình
- **Xác thực chỉnh sửa**: Xem trước thay đổi trước khi áp dụng kèm nhật ký kiểm toán
- **Hỗ trợ OAuth**: Phương thức xác thực được khuyến nghị cho bot

## Giấy phép

Được cấp phép theo một trong hai giấy phép sau, tùy bạn lựa chọn:

- Giấy phép Apache, phiên bản 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) hoặc http://www.apache.org/licenses/LICENSE-2.0)
- Giấy phép MIT ([LICENSE-MIT](../../../LICENSE-MIT) hoặc http://opensource.org/licenses/MIT)

## Đóng góp

Mọi đóng góp đều được hoan nghênh! Vui lòng xem hướng dẫn chi tiết tại [CONTRIBUTING.md](../../../CONTRIBUTING.md).

## Trạng thái dự án

Đây là dự án đang được tích cực phát triển nhằm viết lại AutoWikiBrowser bằng Rust. Trạng thái hiện tại:

- [x] Client API cốt lõi
- [x] Công cụ quy tắc
- [x] Giao diện dòng lệnh (CLI)
- [x] Hệ thống plugin (Lua/WASM)
- [x] Cơ chế cô lập bảo mật
- [ ] Giao diện dòng lệnh tương tác (đang thực hiện)
- [ ] Giao diện đồ họa (đã lên kế hoạch)
- [ ] Hiển thị so sánh khác biệt nâng cao (đã lên kế hoạch)

## Liên kết

- [Tài liệu](https://docs.rs/awb-rs)
- [Theo dõi sự cố](https://github.com/somethingwithproof/awb-rs/issues)
- [AWB gốc](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
