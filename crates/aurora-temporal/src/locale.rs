//! Locale — language detection, number/date formatting, pluralization,
//! and locale-sensitive string rendering for navigation instructions.

use serde::{Deserialize, Serialize};

/// Supported locale.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Locale {
    /// Language code (ISO 639-1, e.g., "en", "he", "ar")
    pub language: String,
    /// Country code (ISO 3166-1 alpha-2, e.g., "US", "IL")
    pub country: String,
    /// Script code (ISO 15924, e.g., "Latn", "Hebr", "Arab")
    pub script: Option<String>,
}

impl Locale {
    /// Create a new locale from language and country codes.
    pub fn new(language: &str, country: &str) -> Self {
        Self {
            language: language.to_string(),
            country: country.to_string(),
            script: None,
        }
    }

    /// Create a locale with a script.
    pub fn with_script(language: &str, country: &str, script: &str) -> Self {
        Self {
            language: language.to_string(),
            country: country.to_string(),
            script: Some(script.to_string()),
        }
    }

    /// BCP 47 tag (e.g., "en-US", "he-IL").
    pub fn bcp47_tag(&self) -> String {
        match &self.script {
            Some(s) => format!("{}-{}-{}", self.language, s, self.country),
            None => format!("{}-{}", self.language, self.country),
        }
    }
}

/// Number formatting options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberFormat {
    /// Decimal separator ("." or ",")
    pub decimal_sep: char,
    /// Thousands separator ("," or "." or " ")
    pub thousands_sep: char,
    /// Number of decimal places for distance display
    pub distance_decimals: u8,
}

impl NumberFormat {
    /// Format a number as a string.
    pub fn format(&self, value: f64, decimals: u8) -> String {
        let rounded = format!("{:.prec$}", value, prec = decimals as usize);
        let parts: Vec<&str> = rounded.split('.').collect();
        let integer_part = parts[0];

        // Add thousands separators
        let negative = integer_part.starts_with('-');
        let digits: &str = if negative {
            &integer_part[1..]
        } else {
            integer_part
        };

        let mut with_seps = String::new();
        for (i, ch) in digits.chars().rev().enumerate() {
            if i > 0 && i % 3 == 0 {
                with_seps.push(self.thousands_sep);
            }
            with_seps.push(ch);
        }
        let with_seps: String = with_seps.chars().rev().collect();

        let mut result = String::new();
        if negative {
            result.push('-');
        }
        result.push_str(&with_seps);

        if decimals > 0 {
            result.push(self.decimal_sep);
            if parts.len() > 1 {
                result.push_str(parts[1]);
            } else {
                for _ in 0..decimals {
                    result.push('0');
                }
            }
        }

        result
    }
}

/// Get the number format for a locale.
pub fn number_format_for(locale: &Locale) -> NumberFormat {
    match locale.country.as_str() {
        "US" | "GB" | "AU" | "IL" | "JP" | "CN" | "KR" => NumberFormat {
            decimal_sep: '.',
            thousands_sep: ',',
            distance_decimals: 1,
        },
        "DE" | "FR" | "ES" | "IT" | "BR" | "PT" | "RU" | "TR" => NumberFormat {
            decimal_sep: ',',
            thousands_sep: '.',
            distance_decimals: 1,
        },
        _ => NumberFormat {
            decimal_sep: '.',
            thousands_sep: ',',
            distance_decimals: 1,
        },
    }
}

/// Navigation instruction templates for a language.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavInstructionSet {
    /// Language code
    pub language: String,
    /// "Turn left" template
    pub turn_left: String,
    /// "Turn right" template
    pub turn_right: String,
    /// "Continue straight" template
    pub continue_straight: String,
    /// "In {distance}" template
    pub in_distance: String,
    /// "Arrive at destination" template
    pub arrive: String,
    /// "Enter roundabout" template
    pub enter_roundabout: String,
    /// "Take exit {n}" template
    pub take_exit: String,
    /// "Merge onto {road}" template
    pub merge_onto: String,
    /// "U-turn" template
    pub u_turn: String,
}

/// Get navigation instructions for a language.
pub fn nav_instructions(language: &str) -> NavInstructionSet {
    match language {
        "he" => NavInstructionSet {
            language: "he".to_string(),
            turn_left: "פנה שמאלה".to_string(),
            turn_right: "פנה ימינה".to_string(),
            continue_straight: "המשך ישר".to_string(),
            in_distance: "בעוד {distance}".to_string(),
            arrive: "הגעת ליעד".to_string(),
            enter_roundabout: "היכנס לכיכר".to_string(),
            take_exit: "קח יציאה {n}".to_string(),
            merge_onto: "התמזג לכביש {road}".to_string(),
            u_turn: "בצע פניית פרסה".to_string(),
        },
        "ar" => NavInstructionSet {
            language: "ar".to_string(),
            turn_left: "انعطف يساراً".to_string(),
            turn_right: "انعطف يميناً".to_string(),
            continue_straight: "تابع مباشرة".to_string(),
            in_distance: "بعد {distance}".to_string(),
            arrive: "وصلت إلى وجهتك".to_string(),
            enter_roundabout: "ادخل الدوار".to_string(),
            take_exit: "خذ المخرج {n}".to_string(),
            merge_onto: "اندمج في {road}".to_string(),
            u_turn: "استدر للخلف".to_string(),
        },
        "ja" => NavInstructionSet {
            language: "ja".to_string(),
            turn_left: "左折してください".to_string(),
            turn_right: "右折してください".to_string(),
            continue_straight: "直進してください".to_string(),
            in_distance: "あと{distance}".to_string(),
            arrive: "目的地に到着しました".to_string(),
            enter_roundabout: "ロータリーに入ってください".to_string(),
            take_exit: "第{n}出口を出てください".to_string(),
            merge_onto: "{road}に合流してください".to_string(),
            u_turn: "Uターンしてください".to_string(),
        },
        _ => NavInstructionSet {
            language: "en".to_string(),
            turn_left: "Turn left".to_string(),
            turn_right: "Turn right".to_string(),
            continue_straight: "Continue straight".to_string(),
            in_distance: "In {distance}".to_string(),
            arrive: "You have arrived at your destination".to_string(),
            enter_roundabout: "Enter the roundabout".to_string(),
            take_exit: "Take exit {n}".to_string(),
            merge_onto: "Merge onto {road}".to_string(),
            u_turn: "Make a U-turn".to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_bcp47() {
        let locale = Locale::new("en", "US");
        assert_eq!(locale.bcp47_tag(), "en-US");

        let locale_script = Locale::with_script("zh", "CN", "Hans");
        assert_eq!(locale_script.bcp47_tag(), "zh-Hans-CN");
    }

    #[test]
    fn test_number_format_us() {
        let locale = Locale::new("en", "US");
        let fmt = number_format_for(&locale);
        assert_eq!(fmt.format(1234.5, 1), "1,234.5");
    }

    #[test]
    fn test_number_format_de() {
        let locale = Locale::new("de", "DE");
        let fmt = number_format_for(&locale);
        assert_eq!(fmt.format(1234.5, 1), "1.234,5");
    }

    #[test]
    fn test_number_format_no_decimals() {
        let locale = Locale::new("en", "US");
        let fmt = number_format_for(&locale);
        assert_eq!(fmt.format(1000.0, 0), "1,000");
    }

    #[test]
    fn test_nav_instructions_english() {
        let instr = nav_instructions("en");
        assert_eq!(instr.turn_left, "Turn left");
        assert_eq!(instr.turn_right, "Turn right");
    }

    #[test]
    fn test_nav_instructions_hebrew() {
        let instr = nav_instructions("he");
        assert!(instr.turn_left.contains("שמאלה"));
        assert!(instr.turn_right.contains("ימינה"));
    }

    #[test]
    fn test_nav_instructions_arabic() {
        let instr = nav_instructions("ar");
        assert!(instr.turn_left.contains("يساراً"));
    }

    #[test]
    fn test_nav_instructions_japanese() {
        let instr = nav_instructions("ja");
        assert!(instr.turn_left.contains("左折"));
    }

    #[test]
    fn test_nav_instructions_fallback() {
        let instr = nav_instructions("unknown");
        assert_eq!(instr.language, "en");
    }
}
