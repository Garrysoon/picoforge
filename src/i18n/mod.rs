//! Internationalization (i18n) module for PicoForge.
//!
//! Provides localization support. FTL files are parsed at startup into
//! plain HashMaps so there are no Send/Sync issues with Fluent internals.

use std::collections::HashMap;

use sys_locale::get_locale;

/// Supported languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    /// English
    English,
    /// Russian
    Russian,
    /// German
    German,
    /// French
    French,
    /// Spanish
    Spanish,
}

impl Language {
    /// Get all supported languages
    pub fn all() -> &'static [Language] {
        &[
            Language::English,
            Language::Russian,
            Language::German,
            Language::French,
            Language::Spanish,
        ]
    }

    /// Get language code (e.g., "en", "ru")
    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Russian => "ru",
            Language::German => "de",
            Language::French => "fr",
            Language::Spanish => "es",
        }
    }

    /// Get display name in English
    pub fn display_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Russian => "Русский",
            Language::German => "Deutsch",
            Language::French => "Français",
            Language::Spanish => "Español",
        }
    }

    /// Parse from language code
    pub fn from_code(code: &str) -> Option<Language> {
        match code.to_lowercase().as_str() {
            "en" => Some(Language::English),
            "ru" => Some(Language::Russian),
            "de" => Some(Language::German),
            "fr" => Some(Language::French),
            "es" => Some(Language::Spanish),
            _ => None,
        }
    }
}

/// Get system locale as Language
pub fn get_system_language() -> Language {
    get_locale()
        .and_then(|locale| {
            let code = locale.split('-').next()?.split('_').next()?;
            Language::from_code(code)
        })
        .unwrap_or(Language::English)
}

/// Parse a simple FTL file into a HashMap of key -> value.
/// Handles simple messages: `key = value` (single line).
/// Multi-line values and attributes are collapsed.
fn parse_ftl(content: &str) -> HashMap<String, String> {
    let mut messages = HashMap::new();
    let mut current_key: Option<String> = None;
    let mut current_value = String::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            // Flush previous message
            if let Some(key) = current_key.take() {
                messages.insert(key, current_value.trim().to_string());
                current_value.clear();
            }
            continue;
        }

        // Check if this is a new message (key = value)
        if let Some(eq_pos) = trimmed.find(" = ") {
            // Flush previous message
            if let Some(key) = current_key.take() {
                messages.insert(key, current_value.trim().to_string());
                current_value.clear();
            }

            let key = trimmed[..eq_pos].trim().to_string();
            let value = trimmed[eq_pos + 3..].trim();
            current_key = Some(key);
            current_value = value.to_string();
        } else if current_key.is_some() {
            // Continuation line (multi-line value)
            current_value.push(' ');
            current_value.push_str(trimmed);
        }
    }

    // Flush last message
    if let Some(key) = current_key {
        messages.insert(key, current_value.trim().to_string());
    }

    messages
}

fn load_ftl(language: Language) -> &'static str {
    match language {
        Language::English => include_str!("locales/en/main.ftl"),
        Language::Russian => include_str!("locales/ru/main.ftl"),
        Language::German => include_str!("locales/de/main.ftl"),
        Language::French => include_str!("locales/fr/main.ftl"),
        Language::Spanish => include_str!("locales/es/main.ftl"),
    }
}

// Thread-local state for translations
thread_local! {
    static MESSAGES: HashMap<Language, HashMap<String, String>> = {
        let mut all = HashMap::new();
        for &lang in Language::all() {
            let ftl = load_ftl(lang);
            all.insert(lang, parse_ftl(ftl));
        }
        all
    };
    static CURRENT_LANG: std::cell::Cell<Language> = std::cell::Cell::new(get_system_language());
}

/// Initialize the i18n system
pub fn init() {
    CURRENT_LANG.with(|cell| {
        cell.set(get_system_language());
    });
}

fn get_translation(key: &str, args: Option<&[(&str, &str)]>) -> String {
    let lang = CURRENT_LANG.with(|cell| cell.get());

    MESSAGES.with(|messages| {
        let msgs = messages
            .get(&lang)
            .or_else(|| messages.get(&Language::English));

        if let Some(msgs) = msgs {
            if let Some(value) = msgs.get(key) {
                if let Some(args) = args {
                    let mut result = value.clone();
                    for (k, v) in args {
                        result = result.replace(&format!("{{{k}}}"), v);
                    }
                    return result;
                }
                return value.clone();
            }
        }

        key.to_string()
    })
}

/// Get a translation by key
pub fn t(key: &str) -> String {
    get_translation(key, None)
}

/// Get a translation by key with arguments
pub fn t_args(key: &str, args: &[(&str, &str)]) -> String {
    get_translation(key, Some(args))
}

/// Set the current language
pub fn set_language(language: Language) {
    CURRENT_LANG.with(|cell| cell.set(language));
}

/// Get the current language
pub fn current_language() -> Language {
    CURRENT_LANG.with(|cell| cell.get())
}

/// Macro for getting translations with optional format args
#[macro_export]
macro_rules! t {
    ($key:expr) => {
        $crate::i18n::t($key)
    };
    ($key:expr, $($arg_key:expr => $arg_val:expr),*) => {
        $crate::i18n::t_args($key, &[$(($arg_key, $arg_val),)*])
    };
}
