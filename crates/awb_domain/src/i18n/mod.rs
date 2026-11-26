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
        let rest: Vec<&str> = parts.collect();
        let has_subtag = |target: &str| rest.contains(&target);

        match primary {
            "ar" => {
                if has_subtag("eg") {
                    Some(Locale::ArEg)
                } else if has_subtag("ma") {
                    Some(Locale::ArMa)
                } else {
                    Some(Locale::Ar)
                }
            }
            "fr" => {
                if has_subtag("ma") {
                    Some(Locale::FrMa)
                } else if has_subtag("sn") {
                    Some(Locale::FrSn)
                } else {
                    Some(Locale::Fr)
                }
            }
            "zh" => {
                if has_subtag("hant") || has_subtag("tw") || has_subtag("hk") || has_subtag("mo") {
                    Some(Locale::ZhTw)
                } else {
                    Some(Locale::ZhCn)
                }
            }
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

/// Replace a single `{key}` placeholder in a catalog template.
pub fn fmt1(template: &str, key: &str, value: &str) -> String {
    template.replace(&format!("{{{key}}}"), value)
}

/// Replace two `{key}` placeholders in a catalog template.
pub fn fmt2(template: &str, key1: &str, val1: &str, key2: &str, val2: &str) -> String {
    template
        .replace(&format!("{{{key1}}}"), val1)
        .replace(&format!("{{{key2}}}"), val2)
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
    }
}
