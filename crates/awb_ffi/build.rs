// SPDX-FileCopyrightText: 2026 Thomas Vincent <thomasvincent@gmail.com>
// SPDX-License-Identifier: MIT

fn main() {
    uniffi::generate_scaffolding("src/awb_ffi.udl").unwrap();
}
