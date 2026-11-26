// SPDX-FileCopyrightText: 2026 Thomas Vincent <thomasvincent@gmail.com>
// SPDX-License-Identifier: MIT

//! Internationalization (i18n) support for AWB-RS.
//!
//! Provides compile-time checked translations for 45 locales (including 6 RTL
//! locales) with POSIX environment detection (`LC_ALL`, `LC_MESSAGES`, `LANG`)
//! and optional explicit override via `--lang` or `AWB_LANG`.

pub mod catalog;

pub use catalog::Catalog;

/// Supported display locales.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Locale {
    #[default]
    En,
    De,
    Es,
    Fr,
    It,
    PtBr,
    Nl,
    Ru,
    Uk,
    Ja,
    Ko,
    ZhCn,
    ZhTw,
    Ar,
    Hi,
    Pl,
    Tr,
    Sv,
    Vi,
    Id,
    AfZa,
    AmEt,
    ArEg,
    ArMa,
    FrMa,
    FrSn,
    HaNg,
    IgNg,
    MgMg,
    RwRw,
    SoSo,
    SwKe,
    XhZa,
    YoNg,
    ZuZa,
    Bn,
    Fa,
    He,
    Mr,
    Ta,
    Te,
    Th,
    Ur,
    Ms,
    Fil,
}

impl Locale {
    /// All 45 supported locales.
    pub const ALL: [Locale; 45] = [
        Locale::En,
        Locale::De,
        Locale::Es,
        Locale::Fr,
        Locale::It,
        Locale::PtBr,
        Locale::Nl,
        Locale::Ru,
        Locale::Uk,
        Locale::Ja,
        Locale::Ko,
        Locale::ZhCn,
        Locale::ZhTw,
        Locale::Ar,
        Locale::Hi,
        Locale::Pl,
        Locale::Tr,
        Locale::Sv,
        Locale::Vi,
        Locale::Id,
        Locale::AfZa,
        Locale::AmEt,
        Locale::ArEg,
        Locale::ArMa,
        Locale::FrMa,
        Locale::FrSn,
        Locale::HaNg,
        Locale::IgNg,
        Locale::MgMg,
        Locale::RwRw,
        Locale::SoSo,
        Locale::SwKe,
        Locale::XhZa,
        Locale::YoNg,
        Locale::ZuZa,
        Locale::Bn,
        Locale::Fa,
        Locale::He,
        Locale::Mr,
        Locale::Ta,
        Locale::Te,
        Locale::Th,
        Locale::Ur,
        Locale::Ms,
        Locale::Fil,
    ];

    /// Canonical BCP 47 language tag for this locale.
    pub const fn code(self) -> &'static str {
        match self {
            Locale::En => "en",
            Locale::De => "de",
            Locale::Es => "es",
            Locale::Fr => "fr",
            Locale::It => "it",
            Locale::PtBr => "pt-BR",
            Locale::Nl => "nl",
            Locale::Ru => "ru",
            Locale::Uk => "uk",
            Locale::Ja => "ja",
            Locale::Ko => "ko",
            Locale::ZhCn => "zh-CN",
            Locale::ZhTw => "zh-TW",
            Locale::Ar => "ar",
            Locale::Hi => "hi",
            Locale::Pl => "pl",
            Locale::Tr => "tr",
            Locale::Sv => "sv",
            Locale::Vi => "vi",
            Locale::Id => "id",
            Locale::AfZa => "af-ZA",
            Locale::AmEt => "am-ET",
            Locale::ArEg => "ar-EG",
            Locale::ArMa => "ar-MA",
            Locale::FrMa => "fr-MA",
            Locale::FrSn => "fr-SN",
            Locale::HaNg => "ha-NG",
            Locale::IgNg => "ig-NG",
            Locale::MgMg => "mg-MG",
            Locale::RwRw => "rw-RW",
            Locale::SoSo => "so-SO",
            Locale::SwKe => "sw-KE",
            Locale::XhZa => "xh-ZA",
            Locale::YoNg => "yo-NG",
            Locale::ZuZa => "zu-ZA",
            Locale::Bn => "bn",
            Locale::Fa => "fa",
            Locale::He => "he",
            Locale::Mr => "mr",
            Locale::Ta => "ta",
            Locale::Te => "te",
            Locale::Th => "th",
            Locale::Ur => "ur",
            Locale::Ms => "ms",
            Locale::Fil => "fil",
        }
    }

    /// Return `true` if this locale uses right-to-left (RTL) text direction.
    pub const fn is_rtl(self) -> bool {
        matches!(
            self,
            Locale::Ar | Locale::ArEg | Locale::ArMa | Locale::Fa | Locale::He | Locale::Ur
        )
    }

    /// Parse a BCP 47 or POSIX locale tag (for example `de_DE.UTF-8`, `ar_EG.UTF-8`,
    /// `fr-Latn-MA`, `zh_Hans_CN`, `iw`, `tl`, `C`).
    pub fn from_tag(tag: &str) -> Option<Locale> {
        let trimmed = tag.trim();
        if trimmed.is_empty() {
            return None;
        }

        let base = trimmed
            .split('@')
            .next()
            .unwrap_or(trimmed)
            .split('.')
            .next()
            .unwrap_or(trimmed);

        let normalized = base.replace('_', "-").to_ascii_lowercase();

        if normalized == "c" || normalized == "posix" {
            return Some(Locale::En);
        }

        let mut parts = normalized.split('-');
        let primary = parts.next().unwrap_or("");
        let mut script: Option<&str> = None;
        let mut region: Option<&str> = None;

        for subtag in parts {
            if subtag.is_empty() {
                continue;
            }
            // Stop at singleton-led extension or private-use sections (e.g. -x-, -u-, -t-).
            if subtag.len() == 1 {
                break;
            }
            if script.is_none()
                && region.is_none()
                && subtag.len() == 4
                && subtag.bytes().all(|b| b.is_ascii_alphabetic())
            {
                script = Some(subtag);
            } else if region.is_none()
                && ((subtag.len() == 2 && subtag.bytes().all(|b| b.is_ascii_alphabetic()))
                    || (subtag.len() == 3 && subtag.bytes().all(|b| b.is_ascii_digit())))
            {
                region = Some(subtag);
            }
        }

        match primary {
            "ar" => match region {
                Some("eg") => Some(Locale::ArEg),
                Some("ma") => Some(Locale::ArMa),
                _ => Some(Locale::Ar),
            },
            "fr" => match region {
                Some("ma") => Some(Locale::FrMa),
                Some("sn") => Some(Locale::FrSn),
                _ => Some(Locale::Fr),
            },
            "zh" => match script {
                Some("hans") => Some(Locale::ZhCn),
                Some("hant") => Some(Locale::ZhTw),
                _ => match region {
                    Some("tw" | "hk" | "mo") => Some(Locale::ZhTw),
                    _ => Some(Locale::ZhCn),
                },
            },
            "en" => Some(Locale::En),
            "de" => Some(Locale::De),
            "es" => Some(Locale::Es),
            "it" => Some(Locale::It),
            "pt" => Some(Locale::PtBr),
            "nl" => Some(Locale::Nl),
            "ru" => Some(Locale::Ru),
            "uk" => Some(Locale::Uk),
            "ja" => Some(Locale::Ja),
            "ko" => Some(Locale::Ko),
            "hi" => Some(Locale::Hi),
            "pl" => Some(Locale::Pl),
            "tr" => Some(Locale::Tr),
            "sv" => Some(Locale::Sv),
            "vi" => Some(Locale::Vi),
            "id" | "in" => Some(Locale::Id),
            "af" => Some(Locale::AfZa),
            "am" => Some(Locale::AmEt),
            "ha" => Some(Locale::HaNg),
            "ig" => Some(Locale::IgNg),
            "mg" => Some(Locale::MgMg),
            "rw" => Some(Locale::RwRw),
            "so" => Some(Locale::SoSo),
            "sw" => Some(Locale::SwKe),
            "xh" => Some(Locale::XhZa),
            "yo" => Some(Locale::YoNg),
            "zu" => Some(Locale::ZuZa),
            "bn" => Some(Locale::Bn),
            "fa" => Some(Locale::Fa),
            "he" | "iw" => Some(Locale::He),
            "mr" => Some(Locale::Mr),
            "ta" => Some(Locale::Ta),
            "te" => Some(Locale::Te),
            "th" => Some(Locale::Th),
            "ur" => Some(Locale::Ur),
            "ms" => Some(Locale::Ms),
            "fil" | "tl" => Some(Locale::Fil),
            _ => None,
        }
    }

    /// Detect locale from POSIX environment variables (`LC_ALL`, `LC_MESSAGES`, `LANG`).
    pub fn detect() -> Locale {
        for var in ["LC_ALL", "LC_MESSAGES", "LANG"] {
            if let Ok(value) = std::env::var(var) {
                let trimmed = value.trim();
                if trimmed.is_empty() {
                    continue;
                }
                return Self::from_tag(trimmed).unwrap_or(Locale::En);
            }
        }
        Locale::En
    }

    /// Resolve locale from an explicit CLI/env value, falling back to `AWB_LANG` and POSIX detection.
    pub fn resolve(explicit: Option<&str>) -> Locale {
        if let Some(tag) = explicit {
            return Self::from_tag(tag).unwrap_or(Locale::En);
        }
        if let Ok(env_lang) = std::env::var("AWB_LANG") {
            let trimmed = env_lang.trim();
            if !trimmed.is_empty() {
                return Self::from_tag(trimmed).unwrap_or(Locale::En);
            }
        }
        Self::detect()
    }

    /// Return the static message catalog for this locale.
    pub const fn catalog(self) -> &'static Catalog {
        catalog::for_locale(self)
    }
}

/// Extract `--lang <value>` or `--lang=<value>` from raw CLI arguments so `--help`
/// output can be localized before full parsing runs.
pub fn extract_cli_lang(args: &[String]) -> Option<String> {
    let mut iter = args.iter().skip(1);
    while let Some(arg) = iter.next() {
        if arg == "--" {
            break;
        }
        if arg == "--lang" {
            return iter.next().cloned();
        }
        if let Some(val) = arg.strip_prefix("--lang=") {
            return Some(val.to_string());
        }
    }
    None
}

/// Extract `--lang <value>` or `--lang=<value>` from OS-native CLI arguments
/// without losing or panicking on non-UTF-8 arguments (such as file paths).
pub fn extract_cli_lang_os(args: &[std::ffi::OsString]) -> Option<String> {
    let mut iter = args.iter().skip(1);
    while let Some(arg) = iter.next() {
        let Some(arg_str) = arg.to_str() else {
            continue;
        };
        if arg_str == "--" {
            break;
        }
        if arg_str == "--lang" {
            return iter.next().and_then(|v| v.to_str()).map(str::to_string);
        }
        if let Some(val) = arg_str.strip_prefix("--lang=") {
            return Some(val.to_string());
        }
    }
    None
}

/// Replace a single `{key}` placeholder in a catalog template.
pub fn fmt1(template: &str, key: &str, value: &str) -> String {
    template.replace(&format!("{{{key}}}"), value)
}

/// Replace two `{key}` placeholders in a catalog template in a single pass
/// so inserted values are never re-scanned for the other placeholder.
pub fn fmt2(template: &str, key1: &str, val1: &str, key2: &str, val2: &str) -> String {
    let token1 = format!("{{{key1}}}");
    let token2 = format!("{{{key2}}}");
    let mut out = String::with_capacity(template.len() + val1.len() + val2.len());
    let mut rest = template;
    while !rest.is_empty() {
        let next1 = rest.find(&token1);
        let next2 = rest.find(&token2);
        match (next1, next2) {
            (Some(i1), Some(i2)) if i1 <= i2 => {
                out.push_str(&rest[..i1]);
                out.push_str(val1);
                rest = &rest[i1 + token1.len()..];
            }
            (Some(_), Some(i2)) => {
                out.push_str(&rest[..i2]);
                out.push_str(val2);
                rest = &rest[i2 + token2.len()..];
            }
            (Some(i1), None) => {
                out.push_str(&rest[..i1]);
                out.push_str(val1);
                rest = &rest[i1 + token1.len()..];
            }
            (None, Some(i2)) => {
                out.push_str(&rest[..i2]);
                out.push_str(val2);
                rest = &rest[i2 + token2.len()..];
            }
            (None, None) => {
                out.push_str(rest);
                break;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extract_placeholders(text: &str) -> Vec<String> {
        let mut placeholders = Vec::new();
        let bytes = text.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'{'
                && let Some(end) = text[i + 1..].find('}')
            {
                let token = &text[i + 1..i + 1 + end];
                if !token.is_empty() && token.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                {
                    placeholders.push(format!("{{{token}}}"));
                    i += end + 2;
                    continue;
                }
            }
            i += 1;
        }
        placeholders.sort();
        placeholders
    }

    #[test]
    fn test_all_45_locales_parse() {
        assert_eq!(Locale::ALL.len(), 45);
        for locale in Locale::ALL {
            assert_eq!(Locale::from_tag(locale.code()), Some(locale));
        }
        assert_eq!(Locale::from_tag("C"), Some(Locale::En));
        assert_eq!(Locale::from_tag("POSIX"), Some(Locale::En));
        assert_eq!(Locale::from_tag("en_US.UTF-8"), Some(Locale::En));
        assert_eq!(Locale::from_tag("de_DE.UTF-8@euro"), Some(Locale::De));
        assert_eq!(Locale::from_tag("pt_BR.UTF-8"), Some(Locale::PtBr));
        assert_eq!(Locale::from_tag("zh_Hans_CN.UTF-8"), Some(Locale::ZhCn));
        assert_eq!(Locale::from_tag("zh_TW.UTF-8"), Some(Locale::ZhTw));
        assert_eq!(Locale::from_tag("zh-Hant"), Some(Locale::ZhTw));
        assert_eq!(Locale::from_tag(""), None);
        assert_eq!(Locale::from_tag("xx-UNKNOWN"), None);
    }

    #[test]
    fn test_regional_tags_and_legacy_fallbacks() {
        assert_eq!(Locale::from_tag("ar_EG.UTF-8"), Some(Locale::ArEg));
        assert_eq!(Locale::from_tag("ar-Arab-EG"), Some(Locale::ArEg));
        assert_eq!(Locale::from_tag("ar_MA.UTF-8"), Some(Locale::ArMa));
        assert_eq!(Locale::from_tag("ar-Arab-MA"), Some(Locale::ArMa));
        assert_eq!(Locale::from_tag("ar_SA.UTF-8"), Some(Locale::Ar));
        assert_eq!(Locale::from_tag("fr_MA.UTF-8"), Some(Locale::FrMa));
        assert_eq!(Locale::from_tag("fr-Latn-MA"), Some(Locale::FrMa));
        assert_eq!(Locale::from_tag("fr_SN.UTF-8"), Some(Locale::FrSn));
        assert_eq!(Locale::from_tag("fr-Latn-SN"), Some(Locale::FrSn));
        assert_eq!(Locale::from_tag("fr_CA.UTF-8"), Some(Locale::Fr));
        assert_eq!(Locale::from_tag("iw"), Some(Locale::He));
        assert_eq!(Locale::from_tag("iw_IL.UTF-8"), Some(Locale::He));
        assert_eq!(Locale::from_tag("in"), Some(Locale::Id));
        assert_eq!(Locale::from_tag("in_ID.UTF-8"), Some(Locale::Id));
        assert_eq!(Locale::from_tag("tl"), Some(Locale::Fil));
        assert_eq!(Locale::from_tag("tl_PH.UTF-8"), Some(Locale::Fil));
        assert_eq!(Locale::from_tag("sw_TZ.UTF-8"), Some(Locale::SwKe));
        assert_eq!(Locale::from_tag("ha-Latn-NG"), Some(Locale::HaNg));
        assert_eq!(Locale::from_tag("unknown-tag"), None);
    }

    #[test]
    fn test_is_rtl_locales() {
        let rtl_locales: Vec<Locale> = Locale::ALL.into_iter().filter(|l| l.is_rtl()).collect();
        let ltr_locales: Vec<Locale> = Locale::ALL.into_iter().filter(|l| !l.is_rtl()).collect();

        assert_eq!(rtl_locales.len(), 6);
        assert_eq!(ltr_locales.len(), 39);
        assert_eq!(
            rtl_locales,
            vec![
                Locale::Ar,
                Locale::ArEg,
                Locale::ArMa,
                Locale::Fa,
                Locale::He,
                Locale::Ur,
            ]
        );
    }

    #[test]
    fn test_all_catalog_entries_non_empty_and_placeholder_parity() {
        let en = Locale::En.catalog().entries();
        assert_eq!(en.len(), 169);

        for locale in Locale::ALL {
            let entries = locale.catalog().entries();
            assert_eq!(entries.len(), en.len());
            for ((en_key, en_val), (loc_key, loc_val)) in en.iter().zip(entries.iter()) {
                assert_eq!(en_key, loc_key);
                assert!(
                    !loc_val.trim().is_empty(),
                    "Empty translation for {loc_key} in {}",
                    locale.code()
                );
                assert_eq!(
                    extract_placeholders(loc_val),
                    extract_placeholders(en_val),
                    "Placeholder mismatch for {loc_key} in {}: got {loc_val:?}",
                    locale.code()
                );
            }
        }
    }

    #[test]
    fn test_default_english_strings_unchanged() {
        let en = Locale::En.catalog();
        assert_eq!(
            en.cli_about,
            "AutoWikiBrowser in Rust - Wikipedia bot framework"
        );
        assert_eq!(en.login_title, "Login to MediaWiki");
        assert_eq!(en.report_header, "=== Bot Run Summary ===");
        assert_eq!(en.gtk_status_ready, "Ready");
    }

    #[test]
    fn test_bcp47_extensions_and_chinese_script_precedence() {
        // Explicit Chinese script takes precedence over region fallback
        assert_eq!(Locale::from_tag("zh-Hans-HK"), Some(Locale::ZhCn));
        assert_eq!(Locale::from_tag("zh-Hant-CN"), Some(Locale::ZhTw));
        assert_eq!(Locale::from_tag("zh-HK"), Some(Locale::ZhTw));
        assert_eq!(Locale::from_tag("zh-MO"), Some(Locale::ZhTw));
        assert_eq!(Locale::from_tag("zh-TW"), Some(Locale::ZhTw));
        assert_eq!(Locale::from_tag("zh-CN"), Some(Locale::ZhCn));
        assert_eq!(Locale::from_tag("zh"), Some(Locale::ZhCn));
        assert_eq!(Locale::from_tag("zh-Latn-TW"), Some(Locale::ZhTw));

        // Singleton-led extension and private-use subtags are ignored
        assert_eq!(Locale::from_tag("fr-SN-x-ma"), Some(Locale::FrSn));
        assert_eq!(Locale::from_tag("fr-MA-x-sn"), Some(Locale::FrMa));
        assert_eq!(Locale::from_tag("fr-x-ma"), Some(Locale::Fr));
        assert_eq!(Locale::from_tag("zh-CN-x-tw"), Some(Locale::ZhCn));
        assert_eq!(Locale::from_tag("zh-x-tw"), Some(Locale::ZhCn));
        assert_eq!(Locale::from_tag("ar-EG-u-nu-latn"), Some(Locale::ArEg));
        assert_eq!(Locale::from_tag("ar-MA-t-eg"), Some(Locale::ArMa));
        assert_eq!(Locale::from_tag("ar-u-eg"), Some(Locale::Ar));

        // 3-digit UN M.49 regions and empty subtags
        assert_eq!(Locale::from_tag("ar-001"), Some(Locale::Ar));
        assert_eq!(Locale::from_tag("es-419"), Some(Locale::Es));
        assert_eq!(Locale::from_tag("fr--MA"), Some(Locale::FrMa));
        assert_eq!(Locale::from_tag("   "), None);
    }

    #[test]
    fn test_fmt1_and_fmt2() {
        assert_eq!(fmt1("Hello {name}!", "name", "AWB"), "Hello AWB!");
        assert_eq!(
            fmt2(
                "Saved {page} (rev {rev})",
                "page",
                "Main Page",
                "rev",
                "12345"
            ),
            "Saved Main Page (rev 12345)"
        );
        // Single-pass substitution must not re-scan inserted values
        assert_eq!(
            fmt2("Saved: {page} (rev {rev})", "page", "{rev}", "rev", "42"),
            "Saved: {rev} (rev 42)"
        );
        // Reverse order and mutual placeholder swap
        assert_eq!(
            fmt2("Rev {rev}: {page}", "page", "{rev}", "rev", "{page}"),
            "Rev {page}: {rev}"
        );
        // Template without matching placeholders stays unchanged
        assert_eq!(fmt2("Plain text", "a", "1", "b", "2"), "Plain text");
    }

    #[test]
    fn test_extract_cli_lang() {
        let args = vec![
            "awb-rs".to_string(),
            "--lang".to_string(),
            "fr".to_string(),
            "--help".to_string(),
        ];
        assert_eq!(extract_cli_lang(&args), Some("fr".to_string()));

        let args_eq = vec!["awb-rs".to_string(), "--lang=zh-CN".to_string()];
        assert_eq!(extract_cli_lang(&args_eq), Some("zh-CN".to_string()));

        let args_after_sep = vec![
            "awb-rs".to_string(),
            "--".to_string(),
            "--lang".to_string(),
            "de".to_string(),
        ];
        assert_eq!(extract_cli_lang(&args_after_sep), None);

        let args_none = vec!["awb-rs".to_string(), "list".to_string()];
        assert_eq!(extract_cli_lang(&args_none), None);

        let args_missing_val = vec!["awb-rs".to_string(), "--lang".to_string()];
        assert_eq!(extract_cli_lang(&args_missing_val), None);
    }

    #[test]
    fn test_extract_cli_lang_os() {
        use std::ffi::OsString;

        let args = vec![
            OsString::from("awb-rs"),
            OsString::from("--lang"),
            OsString::from("ja"),
            OsString::from("--help"),
        ];
        assert_eq!(extract_cli_lang_os(&args), Some("ja".to_string()));

        let args_eq = vec![OsString::from("awb-rs"), OsString::from("--lang=ar-EG")];
        assert_eq!(extract_cli_lang_os(&args_eq), Some("ar-EG".to_string()));

        let args_after_sep = vec![
            OsString::from("awb-rs"),
            OsString::from("--"),
            OsString::from("--lang=de"),
        ];
        assert_eq!(extract_cli_lang_os(&args_after_sep), None);

        let args_none = vec![OsString::from("awb-rs"), OsString::from("run")];
        assert_eq!(extract_cli_lang_os(&args_none), None);

        let args_missing_val = vec![OsString::from("awb-rs"), OsString::from("--lang")];
        assert_eq!(extract_cli_lang_os(&args_missing_val), None);
    }

    #[cfg(unix)]
    #[test]
    fn test_extract_cli_lang_os_non_utf8_unix() {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;

        let non_utf8 = OsString::from_vec(vec![b'f', b'o', b'o', 0xff, b'b', b'a', b'r']);

        // Non-UTF-8 path argument before --lang is preserved/skipped cleanly
        let args = vec![
            OsString::from("awb-rs"),
            non_utf8.clone(),
            OsString::from("--lang"),
            OsString::from("de"),
        ];
        assert_eq!(extract_cli_lang_os(&args), Some("de".to_string()));

        // Non-UTF-8 value following --lang yields None without panicking
        let args_bad_val = vec![OsString::from("awb-rs"), OsString::from("--lang"), non_utf8];
        assert_eq!(extract_cli_lang_os(&args_bad_val), None);

        // Non-UTF-8 --lang=<bytes> is skipped cleanly
        let mut eq_bytes = b"--lang=".to_vec();
        eq_bytes.push(0xfe);
        let args_bad_eq = vec![OsString::from("awb-rs"), OsString::from_vec(eq_bytes)];
        assert_eq!(extract_cli_lang_os(&args_bad_eq), None);
    }

    #[test]
    fn test_detect_and_resolve_env() {
        use std::sync::Mutex;

        static ENV_LOCK: Mutex<()> = Mutex::new(());
        const VARS: [&str; 4] = ["AWB_LANG", "LC_ALL", "LC_MESSAGES", "LANG"];

        struct EnvGuard(Vec<(&'static str, Option<std::ffi::OsString>)>);
        impl EnvGuard {
            fn new() -> Self {
                let saved = VARS
                    .iter()
                    .map(|&k| (k, std::env::var_os(k)))
                    .collect::<Vec<_>>();
                for &k in &VARS {
                    // SAFETY: Serialized by ENV_LOCK within this unit test.
                    unsafe { std::env::remove_var(k) };
                }
                Self(saved)
            }
            fn set(&self, key: &str, val: &str) {
                // SAFETY: Serialized by ENV_LOCK within this unit test.
                unsafe { std::env::set_var(key, val) };
            }
            fn remove(&self, key: &str) {
                // SAFETY: Serialized by ENV_LOCK within this unit test.
                unsafe { std::env::remove_var(key) };
            }
        }
        impl Drop for EnvGuard {
            fn drop(&mut self) {
                for (k, v) in &self.0 {
                    // SAFETY: Serialized by ENV_LOCK within this unit test.
                    unsafe {
                        match v {
                            Some(val) => std::env::set_var(k, val),
                            None => std::env::remove_var(k),
                        }
                    }
                }
            }
        }

        let _lock = ENV_LOCK.lock().expect("env lock poisoned");
        let guard = EnvGuard::new();

        // All unset -> English default
        assert_eq!(Locale::detect(), Locale::En);
        assert_eq!(Locale::resolve(None), Locale::En);

        // LANG fallback when LC_ALL / LC_MESSAGES are unset
        guard.set("LANG", "de_DE.UTF-8");
        assert_eq!(Locale::detect(), Locale::De);

        // Whitespace LC_ALL is ignored; LC_MESSAGES wins over LANG
        guard.set("LC_ALL", "   ");
        guard.set("LC_MESSAGES", "fr-SN-x-ma");
        assert_eq!(Locale::detect(), Locale::FrSn);

        // LC_ALL wins over LC_MESSAGES and LANG
        guard.set("LC_ALL", "ar-EG-u-nu-latn");
        assert_eq!(Locale::detect(), Locale::ArEg);

        // Unknown LC_ALL falls back to English
        guard.set("LC_ALL", "zz-UNKNOWN");
        assert_eq!(Locale::detect(), Locale::En);

        // Whitespace AWB_LANG falls through to detect()
        guard.set("LC_ALL", "uk_UA.UTF-8");
        guard.set("AWB_LANG", "  ");
        assert_eq!(Locale::resolve(None), Locale::Uk);

        // AWB_LANG takes precedence over POSIX variables
        guard.set("AWB_LANG", "zh-Hans-HK");
        assert_eq!(Locale::resolve(None), Locale::ZhCn);

        // Unknown AWB_LANG falls back to English
        guard.set("AWB_LANG", "xx-bad");
        assert_eq!(Locale::resolve(None), Locale::En);

        // Explicit CLI override wins over everything
        assert_eq!(Locale::resolve(Some("zh-Hant-CN")), Locale::ZhTw);
        assert_eq!(Locale::resolve(Some("xx-bad")), Locale::En);

        guard.remove("AWB_LANG");
        assert_eq!(Locale::resolve(None), Locale::Uk);
    }
}
