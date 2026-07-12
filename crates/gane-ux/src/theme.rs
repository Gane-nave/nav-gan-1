//! Theme engine — manages visual themes (day, night, minimal, rain)
//! and conditional interface rules for the luxury visual language.

use serde::{Deserialize, Serialize};
use tracing::debug;

/// A colour represented as RGBA [0, 255].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Colour {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

/// A complete visual theme.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub background: Colour,
    pub foreground: Colour,
    pub accent: Colour,
    pub warning_colour: Colour,
    pub error_colour: Colour,
    pub road_primary: Colour,
    pub road_secondary: Colour,
    pub route_line: Colour,
    pub route_alternative: Colour,
    pub text_primary: Colour,
    pub text_secondary: Colour,
    pub brightness: f64,
    pub contrast: f64,
    pub font_scale: f64,
}

impl Theme {
    /// The default day theme — light, high-contrast, premium feel.
    pub fn day() -> Self {
        Self {
            name: "Day".into(),
            background: Colour::rgb(250, 250, 252),
            foreground: Colour::rgb(20, 20, 30),
            accent: Colour::rgb(0, 122, 255),
            warning_colour: Colour::rgb(255, 149, 0),
            error_colour: Colour::rgb(255, 59, 48),
            road_primary: Colour::rgb(180, 180, 190),
            road_secondary: Colour::rgb(210, 210, 218),
            route_line: Colour::rgb(0, 122, 255),
            route_alternative: Colour::rgba(0, 122, 255, 100),
            text_primary: Colour::rgb(20, 20, 30),
            text_secondary: Colour::rgb(120, 120, 135),
            brightness: 1.0,
            contrast: 1.0,
            font_scale: 1.0,
        }
    }

    /// The default night theme — dark, reduced brightness, easy on eyes.
    pub fn night() -> Self {
        Self {
            name: "Night".into(),
            background: Colour::rgb(15, 15, 25),
            foreground: Colour::rgb(220, 220, 230),
            accent: Colour::rgb(50, 150, 255),
            warning_colour: Colour::rgb(255, 180, 50),
            error_colour: Colour::rgb(255, 80, 70),
            road_primary: Colour::rgb(60, 60, 75),
            road_secondary: Colour::rgb(40, 40, 55),
            route_line: Colour::rgb(50, 150, 255),
            route_alternative: Colour::rgba(50, 150, 255, 80),
            text_primary: Colour::rgb(220, 220, 230),
            text_secondary: Colour::rgb(130, 130, 150),
            brightness: 0.6,
            contrast: 0.9,
            font_scale: 1.0,
        }
    }

    /// Minimal theme — very low distraction, high contrast essentials only.
    pub fn minimal() -> Self {
        Self {
            name: "Minimal".into(),
            background: Colour::rgb(10, 10, 15),
            foreground: Colour::rgb(240, 240, 245),
            accent: Colour::rgb(0, 200, 120),
            warning_colour: Colour::rgb(255, 200, 50),
            error_colour: Colour::rgb(255, 60, 60),
            road_primary: Colour::rgb(50, 50, 60),
            road_secondary: Colour::rgb(30, 30, 40),
            route_line: Colour::rgb(0, 200, 120),
            route_alternative: Colour::rgba(0, 200, 120, 60),
            text_primary: Colour::rgb(240, 240, 245),
            text_secondary: Colour::rgb(100, 100, 115),
            brightness: 0.4,
            contrast: 1.2,
            font_scale: 1.2,
        }
    }

    /// Rain theme — slightly warmer tones, higher contrast for wet conditions.
    pub fn rain() -> Self {
        Self {
            name: "Rain".into(),
            background: Colour::rgb(30, 35, 45),
            foreground: Colour::rgb(230, 230, 240),
            accent: Colour::rgb(60, 180, 255),
            warning_colour: Colour::rgb(255, 165, 0),
            error_colour: Colour::rgb(255, 70, 60),
            road_primary: Colour::rgb(70, 75, 90),
            road_secondary: Colour::rgb(50, 55, 65),
            route_line: Colour::rgb(60, 180, 255),
            route_alternative: Colour::rgba(60, 180, 255, 90),
            text_primary: Colour::rgb(230, 230, 240),
            text_secondary: Colour::rgb(140, 145, 160),
            brightness: 0.7,
            contrast: 1.1,
            font_scale: 1.0,
        }
    }
}

/// Conditions that drive automatic theme selection.
#[derive(Debug, Clone, Copy)]
pub struct EnvironmentConditions {
    pub is_night: bool,
    pub is_raining: bool,
    pub ambient_brightness: f64,
    pub speed_kmh: f64,
}

/// Theme engine — selects and interpolates themes based on conditions.
pub struct ThemeEngine {
    active: Theme,
    themes: Vec<Theme>,
    auto_switch: bool,
}

impl ThemeEngine {
    pub fn new() -> Self {
        Self {
            active: Theme::day(),
            themes: vec![
                Theme::day(),
                Theme::night(),
                Theme::minimal(),
                Theme::rain(),
            ],
            auto_switch: true,
        }
    }

    /// Switch to a named theme.
    pub fn switch(&mut self, name: &str) -> bool {
        if let Some(theme) = self.themes.iter().find(|t| t.name == name).cloned() {
            debug!(theme = %theme.name, "theme switched");
            self.active = theme;
            true
        } else {
            false
        }
    }

    /// Automatically select the best theme for current conditions.
    pub fn auto_select(&mut self, conditions: &EnvironmentConditions) -> bool {
        if !self.auto_switch {
            return false;
        }

        let target_name = if conditions.speed_kmh > 100.0 {
            "Minimal"
        } else if conditions.is_raining {
            "Rain"
        } else if conditions.is_night || conditions.ambient_brightness < 0.2 {
            "Night"
        } else {
            "Day"
        };

        if self.active.name != target_name {
            self.switch(target_name)
        } else {
            false
        }
    }

    /// Enable or disable automatic theme switching.
    pub fn set_auto_switch(&mut self, enabled: bool) {
        self.auto_switch = enabled;
    }

    /// Get the active theme.
    pub fn active(&self) -> &Theme {
        &self.active
    }

    /// Get the active theme name.
    pub fn active_name(&self) -> &str {
        &self.active.name
    }

    /// Add a custom theme.
    pub fn add_theme(&mut self, theme: Theme) {
        self.themes.push(theme);
    }

    /// Get effective brightness, considering the theme and ambient conditions.
    pub fn effective_brightness(&self, ambient: f64) -> f64 {
        // In very dark environments, reduce brightness further.
        let ambient_factor = if ambient < 0.3 { 0.5 + ambient } else { 1.0 };
        (self.active.brightness * ambient_factor).clamp(0.1, 1.0)
    }
}

impl Default for ThemeEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_theme_is_day() {
        let engine = ThemeEngine::new();
        assert_eq!(engine.active_name(), "Day");
    }

    #[test]
    fn switch_to_night() {
        let mut engine = ThemeEngine::new();
        assert!(engine.switch("Night"));
        assert_eq!(engine.active_name(), "Night");
        assert!(engine.active().brightness < 1.0);
    }

    #[test]
    fn switch_nonexistent_fails() {
        let mut engine = ThemeEngine::new();
        assert!(!engine.switch("Aurora"));
        assert_eq!(engine.active_name(), "Day");
    }

    #[test]
    fn auto_select_night() {
        let mut engine = ThemeEngine::new();
        let conditions = EnvironmentConditions {
            is_night: true,
            is_raining: false,
            ambient_brightness: 0.1,
            speed_kmh: 50.0,
        };
        assert!(engine.auto_select(&conditions));
        assert_eq!(engine.active_name(), "Night");
    }

    #[test]
    fn auto_select_rain() {
        let mut engine = ThemeEngine::new();
        let conditions = EnvironmentConditions {
            is_night: false,
            is_raining: true,
            ambient_brightness: 0.5,
            speed_kmh: 60.0,
        };
        assert!(engine.auto_select(&conditions));
        assert_eq!(engine.active_name(), "Rain");
    }

    #[test]
    fn auto_select_minimal_at_high_speed() {
        let mut engine = ThemeEngine::new();
        let conditions = EnvironmentConditions {
            is_night: false,
            is_raining: false,
            ambient_brightness: 0.8,
            speed_kmh: 120.0,
        };
        assert!(engine.auto_select(&conditions));
        assert_eq!(engine.active_name(), "Minimal");
    }

    #[test]
    fn auto_switch_disabled() {
        let mut engine = ThemeEngine::new();
        engine.set_auto_switch(false);
        let conditions = EnvironmentConditions {
            is_night: true,
            is_raining: false,
            ambient_brightness: 0.1,
            speed_kmh: 50.0,
        };
        assert!(!engine.auto_select(&conditions));
        assert_eq!(engine.active_name(), "Day");
    }

    #[test]
    fn effective_brightness_dark_ambient() {
        let engine = ThemeEngine::new();
        let bright = engine.effective_brightness(0.1);
        assert!(bright < engine.active().brightness);
    }

    #[test]
    fn colour_constructors() {
        let c = Colour::rgb(255, 128, 0);
        assert_eq!(c.a, 255);
        let c2 = Colour::rgba(255, 128, 0, 128);
        assert_eq!(c2.a, 128);
    }
}
