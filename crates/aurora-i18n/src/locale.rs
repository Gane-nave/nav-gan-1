//! Locale management — language/region detection, fallback chains, and locale switching.

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A locale identifier (e.g., "en-US", "he-IL", "ar-SA").
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Locale {
    pub language: String,
    pub region: Option<String>,
    pub script: Option<String>,
}

impl Locale {
    /// Create a locale from a language tag (e.g., "en-US", "he", "ar-SA").
    pub fn new(tag: &str) -> Self {
        let parts: Vec<&str> = tag.split('-').collect();
        Self {
            language: parts[0].to_lowercase(),
            region: parts.get(1).map(|r| r.to_uppercase()),
            script: parts.get(2).map(|s| s.to_string()),
        }
    }

    /// Get the full locale tag (e.g., "en-US").
    pub fn tag(&self) -> String {
        match &self.region {
            Some(r) => format!("{}-{}", self.language, r),
            None => self.language.clone(),
        }
    }

    /// Check if this locale is a right-to-left language.
    pub fn is_rtl(&self) -> bool {
        matches!(
            self.language.as_str(),
            "ar" | "he" | "fa" | "ur" | "yi" | "ps" | "sd" | "ckb"
        )
    }
}

/// Text direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextDirection {
    Ltr,
    Rtl,
}

/// Number formatting style.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberFormat {
    pub decimal_separator: char,
    pub thousands_separator: char,
    pub currency_symbol: String,
    pub currency_position: CurrencyPosition,
}

/// Currency symbol position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurrencyPosition {
    Before,
    After,
}

/// Locale-specific formatting rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocaleConfig {
    pub locale: Locale,
    pub direction: TextDirection,
    pub number_format: NumberFormat,
    pub date_format: String,
    pub time_format: String,
    pub distance_unit: DistanceUnit,
    pub speed_unit: SpeedUnit,
}

/// Distance unit system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DistanceUnit {
    Metric,
    Imperial,
}

/// Speed unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpeedUnit {
    Kmh,
    Mph,
}

impl LocaleConfig {
    /// Create a default config for a locale.
    pub fn for_locale(locale: &Locale) -> Self {
        let direction = if locale.is_rtl() {
            TextDirection::Rtl
        } else {
            TextDirection::Ltr
        };

        let (distance_unit, speed_unit) = match locale.region.as_deref() {
            Some("US") | Some("GB") | Some("MM") | Some("LR") => {
                (DistanceUnit::Imperial, SpeedUnit::Mph)
            }
            _ => (DistanceUnit::Metric, SpeedUnit::Kmh),
        };

        let number_format = match locale.language.as_str() {
            "de" | "fr" | "es" | "pt" | "it" => NumberFormat {
                decimal_separator: ',',
                thousands_separator: '.',
                currency_symbol: "€".to_string(),
                currency_position: CurrencyPosition::After,
            },
            "ar" | "he" | "fa" => NumberFormat {
                decimal_separator: '.',
                thousands_separator: ',',
                currency_symbol: "$".to_string(),
                currency_position: CurrencyPosition::Before,
            },
            _ => NumberFormat {
                decimal_separator: '.',
                thousands_separator: ',',
                currency_symbol: "$".to_string(),
                currency_position: CurrencyPosition::Before,
            },
        };

        Self {
            locale: locale.clone(),
            direction,
            number_format,
            date_format: "%Y-%m-%d".to_string(),
            time_format: "%H:%M".to_string(),
            distance_unit,
            speed_unit,
        }
    }

    /// Format a distance value according to locale settings.
    pub fn format_distance(&self, meters: f64) -> String {
        match self.distance_unit {
            DistanceUnit::Metric => {
                if meters >= 1000.0 {
                    format!("{} km", self.format_number(meters / 1000.0, 1))
                } else {
                    format!("{} m", self.format_number(meters, 0))
                }
            }
            DistanceUnit::Imperial => {
                let miles = meters / 1609.344;
                if miles >= 0.1 {
                    format!("{} mi", self.format_number(miles, 1))
                } else {
                    let feet = meters * 3.28084;
                    format!("{} ft", self.format_number(feet, 0))
                }
            }
        }
    }

    /// Format a speed value according to locale settings.
    pub fn format_speed(&self, mps: f64) -> String {
        match self.speed_unit {
            SpeedUnit::Kmh => format!("{} km/h", self.format_number(mps * 3.6, 0)),
            SpeedUnit::Mph => format!("{} mph", self.format_number(mps * 2.23694, 0)),
        }
    }

    /// Format a number with locale-appropriate separators.
    pub fn format_number(&self, value: f64, decimals: usize) -> String {
        let rounded = format!("{:.prec$}", value, prec = decimals);
        let parts: Vec<&str> = rounded.split('.').collect();
        let integer = parts[0];

        let mut result = String::new();
        let chars: Vec<char> = integer.chars().collect();
        let start = if chars[0] == '-' { 1 } else { 0 };
        let digits = &chars[start..];

        if start == 1 {
            result.push('-');
        }

        for (i, ch) in digits.iter().enumerate() {
            #[allow(unknown_lints, clippy::manual_is_multiple_of)]
            if i > 0 && (digits.len() - i) % 3 == 0 {
                result.push(self.number_format.thousands_separator);
            }
            result.push(*ch);
        }

        if decimals > 0 {
            result.push(self.number_format.decimal_separator);
            if let Some(frac) = parts.get(1) {
                result.push_str(frac);
            }
        }

        result
    }
}

/// Locale manager — manages active locale, fallback chains, and available locales.
pub struct LocaleManager {
    active: RwLock<Locale>,
    configs: RwLock<HashMap<String, LocaleConfig>>,
    fallback_chain: RwLock<Vec<String>>,
}

impl LocaleManager {
    /// Create a new locale manager with a default locale.
    pub fn new(default_locale: &str) -> Self {
        let locale = Locale::new(default_locale);
        let config = LocaleConfig::for_locale(&locale);
        let tag = locale.tag();
        let mut configs = HashMap::new();
        configs.insert(tag.clone(), config);

        Self {
            active: RwLock::new(locale),
            configs: RwLock::new(configs),
            fallback_chain: RwLock::new(vec![tag, "en".to_string()]),
        }
    }

    /// Set the active locale.
    pub fn set_locale(&self, tag: &str) {
        let locale = Locale::new(tag);
        let config = LocaleConfig::for_locale(&locale);
        let key = locale.tag();
        self.configs.write().insert(key.clone(), config);
        *self.active.write() = locale;
        let mut chain = self.fallback_chain.write();
        chain.retain(|t| t != &key);
        chain.insert(0, key);
    }

    /// Get the active locale.
    pub fn active_locale(&self) -> Locale {
        self.active.read().clone()
    }

    /// Get the active locale config.
    pub fn active_config(&self) -> LocaleConfig {
        let locale = self.active.read().clone();
        self.configs
            .read()
            .get(&locale.tag())
            .cloned()
            .unwrap_or_else(|| LocaleConfig::for_locale(&locale))
    }

    /// Get the fallback chain.
    pub fn fallback_chain(&self) -> Vec<String> {
        self.fallback_chain.read().clone()
    }

    /// Get available locale tags.
    pub fn available_locales(&self) -> Vec<String> {
        self.configs.read().keys().cloned().collect()
    }

    /// Check if a locale is RTL.
    pub fn is_rtl(&self) -> bool {
        self.active.read().is_rtl()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_parsing() {
        let locale = Locale::new("en-US");
        assert_eq!(locale.language, "en");
        assert_eq!(locale.region, Some("US".to_string()));
        assert_eq!(locale.tag(), "en-US");
    }

    #[test]
    fn test_locale_no_region() {
        let locale = Locale::new("he");
        assert_eq!(locale.language, "he");
        assert!(locale.region.is_none());
        assert_eq!(locale.tag(), "he");
    }

    #[test]
    fn test_rtl_detection() {
        assert!(Locale::new("ar-SA").is_rtl());
        assert!(Locale::new("he-IL").is_rtl());
        assert!(Locale::new("fa").is_rtl());
        assert!(!Locale::new("en-US").is_rtl());
        assert!(!Locale::new("de-DE").is_rtl());
    }

    #[test]
    fn test_locale_config_metric() {
        let config = LocaleConfig::for_locale(&Locale::new("de-DE"));
        assert_eq!(config.distance_unit, DistanceUnit::Metric);
        assert_eq!(config.speed_unit, SpeedUnit::Kmh);
        assert_eq!(config.direction, TextDirection::Ltr);
    }

    #[test]
    fn test_locale_config_imperial() {
        let config = LocaleConfig::for_locale(&Locale::new("en-US"));
        assert_eq!(config.distance_unit, DistanceUnit::Imperial);
        assert_eq!(config.speed_unit, SpeedUnit::Mph);
    }

    #[test]
    fn test_locale_config_rtl() {
        let config = LocaleConfig::for_locale(&Locale::new("ar-SA"));
        assert_eq!(config.direction, TextDirection::Rtl);
    }

    #[test]
    fn test_format_distance_metric() {
        let config = LocaleConfig::for_locale(&Locale::new("de-DE"));
        assert_eq!(config.format_distance(500.0), "500 m");
        assert_eq!(config.format_distance(2500.0), "2,5 km");
    }

    #[test]
    fn test_format_distance_imperial() {
        let config = LocaleConfig::for_locale(&Locale::new("en-US"));
        assert_eq!(config.format_distance(1609.344), "1.0 mi"); // en-US uses '.'
        assert!(config.format_distance(10.0).contains("ft"));
    }

    #[test]
    fn test_format_speed() {
        let metric = LocaleConfig::for_locale(&Locale::new("de-DE"));
        assert_eq!(metric.format_speed(27.78), "100 km/h"); // ~100 km/h

        let imperial = LocaleConfig::for_locale(&Locale::new("en-US"));
        let speed = imperial.format_speed(27.78);
        assert!(speed.contains("mph"));
    }

    #[test]
    fn test_format_number_en() {
        let config = LocaleConfig::for_locale(&Locale::new("en-US"));
        assert_eq!(config.format_number(1234567.89, 2), "1,234,567.89");
    }

    #[test]
    fn test_format_number_de() {
        let config = LocaleConfig::for_locale(&Locale::new("de-DE"));
        assert_eq!(config.format_number(1234567.89, 2), "1.234.567,89");
    }

    #[test]
    fn test_locale_manager_switch() {
        let mgr = LocaleManager::new("en-US");
        assert_eq!(mgr.active_locale().tag(), "en-US");
        assert!(!mgr.is_rtl());

        mgr.set_locale("he-IL");
        assert_eq!(mgr.active_locale().tag(), "he-IL");
        assert!(mgr.is_rtl());
    }

    #[test]
    fn test_locale_manager_fallback() {
        let mgr = LocaleManager::new("en-US");
        let chain = mgr.fallback_chain();
        assert_eq!(chain[0], "en-US");
        assert!(chain.contains(&"en".to_string()));
    }
}
