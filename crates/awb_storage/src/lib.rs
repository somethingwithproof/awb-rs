// SPDX-FileCopyrightText: 2026 Thomas Vincent <thomasvincent@gmail.com>
// SPDX-License-Identifier: MIT

pub mod config_store;
pub mod error;
pub mod session_store;

pub use config_store::{Preferences, TomlConfigStore};
pub use error::StorageError;
pub use session_store::{JsonSessionStore, SessionStore};
