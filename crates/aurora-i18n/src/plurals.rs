//! Pluralization engine — language-specific plural rules for correct translations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Plural category (CLDR standard).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PluralCategory {
    Zero,
    One,
    Two,
    Few,
    Many,
    Other,
}

/// A plural rule function that maps a count to a category.
pub type PluralRuleFn = fn(n: u64) -> PluralCategory;

/// English plural rules: 1 → One, else → Other.
pub fn english_rules(n: u64) -> PluralCategory {
    if n == 1 {
        PluralCategory::One
    } else {
        PluralCategory::Other
    }
}

/// Arabic plural rules: 0 → Zero, 1 → One, 2 → Two, 3-10 → Few, 11-99 → Many, else → Other.
pub fn arabic_rules(n: u64) -> PluralCategory {
    let mod100 = n % 100;
    if n == 0 {
        PluralCategory::Zero
    } else if n == 1 {
        PluralCategory::One
    } else if n == 2 {
        PluralCategory::Two
    } else if (3..=10).contains(&mod100) {
        PluralCategory::Few
    } else if (11..=99).contains(&mod100) {
        PluralCategory::Many
    } else {
        PluralCategory::Other
    }
}

/// Hebrew plural rules: 1 → One, 2 → Two, else → Other.
pub fn hebrew_rules(n: u64) -> PluralCategory {
    match n {
        1 => PluralCategory::One,
        2 => PluralCategory::Two,
        _ => PluralCategory::Other,
    }
}

/// Russian/Slavic plural rules.
pub fn slavic_rules(n: u64) -> PluralCategory {
    let mod10 = n % 10;
    let mod100 = n % 100;
    if mod10 == 1 && mod100 != 11 {
        PluralCategory::One
    } else if (2..=4).contains(&mod10) && !(12..=14).contains(&mod100) {
        PluralCategory::Few
    } else {
        PluralCategory::Many
    }
}

/// French/Portuguese rules: 0 or 1 → One, else → Other.
pub fn french_rules(n: u64) -> PluralCategory {
    if n <= 1 {
        PluralCategory::One
    } else {
        PluralCategory::Other
    }
}

/// Get the plural rule function for a language code.
pub fn rules_for_language(lang: &str) -> PluralRuleFn {
    match lang {
        "ar" => arabic_rules,
        "he" | "yi" => hebrew_rules,
        "ru" | "uk" | "pl" | "cs" | "sk" | "hr" | "sr" | "bg" => slavic_rules,
        "fr" | "pt" => french_rules,
        _ => english_rules,
    }
}

/// A pluralized message set for a single key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluralMessage {
    pub key: String,
    pub forms: HashMap<PluralCategory, String>,
}

impl PluralMessage {
    /// Create a new plural message.
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            forms: HashMap::new(),
        }
    }

    /// Add a form for a plural category.
    pub fn with_form(mut self, category: PluralCategory, text: &str) -> Self {
        self.forms.insert(category, text.to_string());
        self
    }

    /// Select the correct form for a count, given a language's plural rules.
    pub fn select(&self, n: u64, rules: PluralRuleFn) -> Option<&str> {
        let category = rules(n);
        self.forms
            .get(&category)
            .or_else(|| self.forms.get(&PluralCategory::Other))
            .map(|s| s.as_str())
    }
}

/// Plural message catalog — stores plural messages for a locale.
pub struct PluralCatalog {
    pub locale: String,
    rules: PluralRuleFn,
    messages: HashMap<String, PluralMessage>,
}

impl PluralCatalog {
    /// Create a new catalog for a locale.
    pub fn new(locale: &str) -> Self {
        let lang = locale.split('-').next().unwrap_or(locale);
        Self {
            locale: locale.to_string(),
            rules: rules_for_language(lang),
            messages: HashMap::new(),
        }
    }

    /// Add a plural message.
    pub fn add(&mut self, msg: PluralMessage) {
        self.messages.insert(msg.key.clone(), msg);
    }

    /// Translate a key with a count.
    pub fn translate(&self, key: &str, n: u64) -> Option<String> {
        self.messages.get(key).and_then(|msg| {
            msg.select(n, self.rules)
                .map(|s| s.replace("{n}", &n.to_string()))
        })
    }

    /// Number of plural messages.
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_english_plurals() {
        assert_eq!(english_rules(0), PluralCategory::Other);
        assert_eq!(english_rules(1), PluralCategory::One);
        assert_eq!(english_rules(2), PluralCategory::Other);
        assert_eq!(english_rules(100), PluralCategory::Other);
    }

    #[test]
    fn test_arabic_plurals() {
        assert_eq!(arabic_rules(0), PluralCategory::Zero);
        assert_eq!(arabic_rules(1), PluralCategory::One);
        assert_eq!(arabic_rules(2), PluralCategory::Two);
        assert_eq!(arabic_rules(5), PluralCategory::Few);
        assert_eq!(arabic_rules(11), PluralCategory::Many);
        assert_eq!(arabic_rules(100), PluralCategory::Other);
    }

    #[test]
    fn test_hebrew_plurals() {
        assert_eq!(hebrew_rules(1), PluralCategory::One);
        assert_eq!(hebrew_rules(2), PluralCategory::Two);
        assert_eq!(hebrew_rules(5), PluralCategory::Other);
    }

    #[test]
    fn test_slavic_plurals() {
        assert_eq!(slavic_rules(1), PluralCategory::One);
        assert_eq!(slavic_rules(2), PluralCategory::Few);
        assert_eq!(slavic_rules(5), PluralCategory::Many);
        assert_eq!(slavic_rules(11), PluralCategory::Many);
        assert_eq!(slavic_rules(21), PluralCategory::One);
        assert_eq!(slavic_rules(22), PluralCategory::Few);
    }

    #[test]
    fn test_french_plurals() {
        assert_eq!(french_rules(0), PluralCategory::One);
        assert_eq!(french_rules(1), PluralCategory::One);
        assert_eq!(french_rules(2), PluralCategory::Other);
    }

    #[test]
    fn test_plural_message_select() {
        let msg = PluralMessage::new("items")
            .with_form(PluralCategory::One, "{n} item")
            .with_form(PluralCategory::Other, "{n} items");

        assert_eq!(msg.select(1, english_rules), Some("{n} item"));
        assert_eq!(msg.select(5, english_rules), Some("{n} items"));
    }

    #[test]
    fn test_plural_catalog() {
        let mut catalog = PluralCatalog::new("en");
        catalog.add(
            PluralMessage::new("routes")
                .with_form(PluralCategory::One, "{n} route found")
                .with_form(PluralCategory::Other, "{n} routes found"),
        );

        assert_eq!(
            catalog.translate("routes", 1),
            Some("1 route found".to_string())
        );
        assert_eq!(
            catalog.translate("routes", 5),
            Some("5 routes found".to_string())
        );
    }

    #[test]
    fn test_arabic_plural_catalog() {
        let mut catalog = PluralCatalog::new("ar");
        catalog.add(
            PluralMessage::new("km")
                .with_form(PluralCategory::Zero, "صفر كيلومترات")
                .with_form(PluralCategory::One, "كيلومتر واحد")
                .with_form(PluralCategory::Two, "كيلومتران")
                .with_form(PluralCategory::Few, "{n} كيلومترات")
                .with_form(PluralCategory::Many, "{n} كيلومتراً")
                .with_form(PluralCategory::Other, "{n} كيلومتر"),
        );

        assert_eq!(
            catalog.translate("km", 0),
            Some("صفر كيلومترات".to_string())
        );
        assert_eq!(catalog.translate("km", 1), Some("كيلومتر واحد".to_string()));
        assert_eq!(catalog.translate("km", 2), Some("كيلومتران".to_string()));
        assert_eq!(catalog.translate("km", 5), Some("5 كيلومترات".to_string()));
        assert_eq!(catalog.translate("km", 15), Some("15 كيلومتراً".to_string()));
    }

    #[test]
    fn test_rules_for_language_lookup() {
        // Should not panic for any known language
        let _ = rules_for_language("en");
        let _ = rules_for_language("ar");
        let _ = rules_for_language("he");
        let _ = rules_for_language("ru");
        let _ = rules_for_language("fr");
        let _ = rules_for_language("unknown");
    }
}
