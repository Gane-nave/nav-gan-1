//! Cultural adaptation — region-specific UI themes, icon sets,
//! greeting messages, and culturally appropriate navigation guidance.

use serde::{Deserialize, Serialize};

/// Text direction for the UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextDirection {
    /// Left-to-right (English, French, etc.)
    Ltr,
    /// Right-to-left (Arabic, Hebrew, etc.)
    Rtl,
}

/// Driving side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DrivingSide {
    /// Right-hand traffic (most countries)
    Right,
    /// Left-hand traffic (UK, Japan, Australia, etc.)
    Left,
}

/// Cultural profile for a region.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalProfile {
    /// Region code (ISO 3166-1 alpha-2)
    pub region: String,
    /// Primary language code (ISO 639-1)
    pub language: String,
    /// Text direction
    pub text_direction: TextDirection,
    /// Driving side
    pub driving_side: DrivingSide,
    /// Distance unit preference
    pub distance_unit: DistancePreference,
    /// Temperature unit preference
    pub temperature_unit: TemperatureUnit,
    /// Time format (12h or 24h)
    pub time_format_24h: bool,
    /// Date format string (e.g., "DD/MM/YYYY", "MM/DD/YYYY")
    pub date_format: String,
    /// Default map style theme
    pub default_theme: String,
    /// Greeting templates for different times of day
    pub greetings: DayGreetings,
    /// Speed unit for display
    pub speed_unit: SpeedDisplayUnit,
}

/// Speed display unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpeedDisplayUnit {
    Kmh,
    Mph,
}

/// Distance unit preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DistancePreference {
    Metric,
    Imperial,
}

/// Temperature unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

impl TemperatureUnit {
    /// Convert from Celsius to this unit.
    pub fn from_celsius(&self, celsius: f64) -> f64 {
        match self {
            Self::Celsius => celsius,
            Self::Fahrenheit => celsius * 9.0 / 5.0 + 32.0,
        }
    }

    /// Unit suffix.
    pub fn suffix(&self) -> &'static str {
        match self {
            Self::Celsius => "°C",
            Self::Fahrenheit => "°F",
        }
    }
}

/// Time-of-day greetings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayGreetings {
    pub morning: String,
    pub afternoon: String,
    pub evening: String,
    pub night: String,
}

impl Default for DayGreetings {
    fn default() -> Self {
        Self {
            morning: "Good morning".to_string(),
            afternoon: "Good afternoon".to_string(),
            evening: "Good evening".to_string(),
            night: "Good night".to_string(),
        }
    }
}

impl DayGreetings {
    /// Get the appropriate greeting for the given hour (0-23).
    pub fn for_hour(&self, hour: u32) -> &str {
        match hour {
            5..=11 => &self.morning,
            12..=16 => &self.afternoon,
            17..=20 => &self.evening,
            _ => &self.night,
        }
    }
}

/// Cultural adaptation engine.
#[derive(Debug)]
pub struct CulturalEngine {
    profiles: Vec<CulturalProfile>,
    active_profile: Option<String>,
}

impl CulturalEngine {
    /// Create a new engine with built-in profiles.
    pub fn new() -> Self {
        let mut engine = Self {
            profiles: Vec::new(),
            active_profile: None,
        };
        engine.load_builtin_profiles();
        engine
    }

    /// Load built-in cultural profiles.
    fn load_builtin_profiles(&mut self) {
        self.profiles.push(CulturalProfile {
            region: "IL".to_string(),
            language: "he".to_string(),
            text_direction: TextDirection::Rtl,
            driving_side: DrivingSide::Right,
            distance_unit: DistancePreference::Metric,
            temperature_unit: TemperatureUnit::Celsius,
            time_format_24h: true,
            date_format: "DD/MM/YYYY".to_string(),
            default_theme: "gane-light".to_string(),
            greetings: DayGreetings {
                morning: "בוקר טוב".to_string(),
                afternoon: "צהריים טובים".to_string(),
                evening: "ערב טוב".to_string(),
                night: "לילה טוב".to_string(),
            },
            speed_unit: SpeedDisplayUnit::Kmh,
        });

        self.profiles.push(CulturalProfile {
            region: "US".to_string(),
            language: "en".to_string(),
            text_direction: TextDirection::Ltr,
            driving_side: DrivingSide::Right,
            distance_unit: DistancePreference::Imperial,
            temperature_unit: TemperatureUnit::Fahrenheit,
            time_format_24h: false,
            date_format: "MM/DD/YYYY".to_string(),
            default_theme: "gane-light".to_string(),
            greetings: DayGreetings::default(),
            speed_unit: SpeedDisplayUnit::Mph,
        });

        self.profiles.push(CulturalProfile {
            region: "GB".to_string(),
            language: "en".to_string(),
            text_direction: TextDirection::Ltr,
            driving_side: DrivingSide::Left,
            distance_unit: DistancePreference::Imperial,
            temperature_unit: TemperatureUnit::Celsius,
            time_format_24h: true,
            date_format: "DD/MM/YYYY".to_string(),
            default_theme: "gane-light".to_string(),
            greetings: DayGreetings::default(),
            speed_unit: SpeedDisplayUnit::Mph,
        });

        self.profiles.push(CulturalProfile {
            region: "JP".to_string(),
            language: "ja".to_string(),
            text_direction: TextDirection::Ltr,
            driving_side: DrivingSide::Left,
            distance_unit: DistancePreference::Metric,
            temperature_unit: TemperatureUnit::Celsius,
            time_format_24h: true,
            date_format: "YYYY/MM/DD".to_string(),
            default_theme: "gane-light".to_string(),
            greetings: DayGreetings {
                morning: "おはようございます".to_string(),
                afternoon: "こんにちは".to_string(),
                evening: "こんばんは".to_string(),
                night: "おやすみなさい".to_string(),
            },
            speed_unit: SpeedDisplayUnit::Kmh,
        });

        self.profiles.push(CulturalProfile {
            region: "AE".to_string(),
            language: "ar".to_string(),
            text_direction: TextDirection::Rtl,
            driving_side: DrivingSide::Right,
            distance_unit: DistancePreference::Metric,
            temperature_unit: TemperatureUnit::Celsius,
            time_format_24h: false,
            date_format: "DD/MM/YYYY".to_string(),
            default_theme: "gane-dark".to_string(),
            greetings: DayGreetings {
                morning: "صباح الخير".to_string(),
                afternoon: "مساء الخير".to_string(),
                evening: "مساء الخير".to_string(),
                night: "تصبح على خير".to_string(),
            },
            speed_unit: SpeedDisplayUnit::Kmh,
        });
    }

    /// Get the profile for a region.
    pub fn profile(&self, region: &str) -> Option<&CulturalProfile> {
        self.profiles.iter().find(|p| p.region == region)
    }

    /// Set the active region.
    pub fn set_active(&mut self, region: &str) -> bool {
        if self.profiles.iter().any(|p| p.region == region) {
            self.active_profile = Some(region.to_string());
            true
        } else {
            false
        }
    }

    /// Get the active profile.
    pub fn active(&self) -> Option<&CulturalProfile> {
        self.active_profile.as_ref().and_then(|r| self.profile(r))
    }

    /// Add a custom cultural profile.
    pub fn add_profile(&mut self, profile: CulturalProfile) {
        // Replace existing if same region
        self.profiles.retain(|p| p.region != profile.region);
        self.profiles.push(profile);
    }

    /// List all available region codes.
    pub fn available_regions(&self) -> Vec<&str> {
        self.profiles.iter().map(|p| p.region.as_str()).collect()
    }

    /// Total number of profiles.
    pub fn profile_count(&self) -> usize {
        self.profiles.len()
    }
}

impl Default for CulturalEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_profiles() {
        let engine = CulturalEngine::new();
        assert!(engine.profile_count() >= 5);
        assert!(engine.profile("IL").is_some());
        assert!(engine.profile("US").is_some());
        assert!(engine.profile("GB").is_some());
        assert!(engine.profile("JP").is_some());
        assert!(engine.profile("AE").is_some());
    }

    #[test]
    fn test_israel_profile() {
        let engine = CulturalEngine::new();
        let il = engine.profile("IL").unwrap();
        assert_eq!(il.text_direction, TextDirection::Rtl);
        assert_eq!(il.driving_side, DrivingSide::Right);
        assert_eq!(il.language, "he");
        assert!(il.time_format_24h);
    }

    #[test]
    fn test_uk_profile() {
        let engine = CulturalEngine::new();
        let gb = engine.profile("GB").unwrap();
        assert_eq!(gb.driving_side, DrivingSide::Left);
        assert_eq!(gb.speed_unit, SpeedDisplayUnit::Mph);
    }

    #[test]
    fn test_set_active() {
        let mut engine = CulturalEngine::new();
        assert!(engine.set_active("IL"));
        assert!(engine.active().is_some());
        assert_eq!(engine.active().unwrap().region, "IL");

        assert!(!engine.set_active("XX")); // Non-existent region
    }

    #[test]
    fn test_temperature_conversion() {
        assert!((TemperatureUnit::Celsius.from_celsius(0.0)).abs() < f64::EPSILON);
        assert!((TemperatureUnit::Fahrenheit.from_celsius(0.0) - 32.0).abs() < 0.1);
        assert!((TemperatureUnit::Fahrenheit.from_celsius(100.0) - 212.0).abs() < 0.1);
    }

    #[test]
    fn test_greetings_for_hour() {
        let greetings = DayGreetings::default();
        assert_eq!(greetings.for_hour(8), "Good morning");
        assert_eq!(greetings.for_hour(14), "Good afternoon");
        assert_eq!(greetings.for_hour(19), "Good evening");
        assert_eq!(greetings.for_hour(23), "Good night");
        assert_eq!(greetings.for_hour(3), "Good night");
    }

    #[test]
    fn test_add_custom_profile() {
        let mut engine = CulturalEngine::new();
        let initial_count = engine.profile_count();

        let profile = CulturalProfile {
            region: "FR".to_string(),
            language: "fr".to_string(),
            text_direction: TextDirection::Ltr,
            driving_side: DrivingSide::Right,
            distance_unit: DistancePreference::Metric,
            temperature_unit: TemperatureUnit::Celsius,
            time_format_24h: true,
            date_format: "DD/MM/YYYY".to_string(),
            default_theme: "gane-light".to_string(),
            greetings: DayGreetings {
                morning: "Bonjour".to_string(),
                afternoon: "Bon après-midi".to_string(),
                evening: "Bonsoir".to_string(),
                night: "Bonne nuit".to_string(),
            },
            speed_unit: SpeedDisplayUnit::Kmh,
        };

        engine.add_profile(profile);
        assert_eq!(engine.profile_count(), initial_count + 1);
        assert!(engine.profile("FR").is_some());
    }
}
