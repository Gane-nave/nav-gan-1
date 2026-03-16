//! Navigation preferences engine.
//!
//! Manages routing preferences, UI settings, notification
//! preferences, and privacy controls per user profile.

/// Routing preference preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutePreset {
    /// Fastest route regardless of cost.
    Fastest,
    /// Shortest distance.
    Shortest,
    /// Most fuel-efficient / eco-friendly.
    Eco,
    /// Avoid highways.
    NoHighways,
    /// Scenic / tourist route.
    Scenic,
    /// Balanced (default).
    Balanced,
}

impl RoutePreset {
    /// Time weight for route scoring.
    pub fn time_weight(self) -> f64 {
        match self {
            Self::Fastest => 1.0,
            Self::Shortest => 0.3,
            Self::Eco => 0.5,
            Self::NoHighways => 0.8,
            Self::Scenic => 0.4,
            Self::Balanced => 0.7,
        }
    }

    /// Distance weight for route scoring.
    pub fn distance_weight(self) -> f64 {
        match self {
            Self::Fastest => 0.2,
            Self::Shortest => 1.0,
            Self::Eco => 0.4,
            Self::NoHighways => 0.5,
            Self::Scenic => 0.3,
            Self::Balanced => 0.5,
        }
    }

    /// Fuel/energy weight for route scoring.
    pub fn fuel_weight(self) -> f64 {
        match self {
            Self::Eco => 1.0,
            Self::Balanced => 0.3,
            _ => 0.1,
        }
    }
}

/// UI theme preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
    Auto,
    HighContrast,
}

/// Map style preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapStyle {
    Standard,
    Satellite,
    Terrain,
    Night,
    Minimal,
}

/// Unit system preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitSystem {
    Metric,
    Imperial,
    /// Mixed (e.g., km for distance, feet for altitude).
    Mixed,
}

impl UnitSystem {
    /// Format a distance in metres.
    pub fn format_distance(&self, metres: f64) -> String {
        match self {
            Self::Metric | Self::Mixed => {
                if metres >= 1000.0 {
                    format!("{:.1} km", metres / 1000.0)
                } else {
                    format!("{} m", metres.round() as u32)
                }
            }
            Self::Imperial => {
                let miles = metres / 1609.344;
                if miles >= 0.1 {
                    format!("{miles:.1} mi")
                } else {
                    format!("{} ft", (metres * 3.28084).round() as u32)
                }
            }
        }
    }

    /// Format a speed in m/s.
    pub fn format_speed(&self, ms: f64) -> String {
        match self {
            Self::Metric | Self::Mixed => format!("{} km/h", (ms * 3.6).round() as u32),
            Self::Imperial => format!("{} mph", (ms * 2.23694).round() as u32),
        }
    }
}

/// Notification preference level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NotificationLevel {
    /// Only critical safety alerts.
    Minimal,
    /// Safety + navigation instructions.
    Normal,
    /// Everything including POIs and info.
    Verbose,
}

/// Privacy settings.
#[derive(Debug, Clone)]
pub struct PrivacySettings {
    /// Share anonymous usage analytics.
    pub analytics: bool,
    /// Share location for traffic data.
    pub share_location: bool,
    /// Store route history locally.
    pub save_history: bool,
    /// Maximum history entries to keep.
    pub max_history: usize,
    /// Enable crash reporting.
    pub crash_reports: bool,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            analytics: true,
            share_location: true,
            save_history: true,
            max_history: 100,
            crash_reports: true,
        }
    }
}

/// Complete user navigation preferences.
#[derive(Debug, Clone)]
pub struct NavPreferences {
    pub route_preset: RoutePreset,
    pub theme: Theme,
    pub map_style: MapStyle,
    pub units: UnitSystem,
    pub notification_level: NotificationLevel,
    pub privacy: PrivacySettings,
    /// Avoid tolls.
    pub avoid_tolls: bool,
    /// Avoid ferries.
    pub avoid_ferries: bool,
    /// Avoid unpaved roads.
    pub avoid_unpaved: bool,
    /// Voice guidance enabled.
    pub voice_enabled: bool,
    /// Auto-reroute on deviation.
    pub auto_reroute: bool,
    /// Show speed limit on HUD.
    pub show_speed_limit: bool,
    /// Show traffic layer on map.
    pub show_traffic: bool,
}

impl Default for NavPreferences {
    fn default() -> Self {
        Self {
            route_preset: RoutePreset::Balanced,
            theme: Theme::Auto,
            map_style: MapStyle::Standard,
            units: UnitSystem::Metric,
            notification_level: NotificationLevel::Normal,
            privacy: PrivacySettings::default(),
            avoid_tolls: false,
            avoid_ferries: false,
            avoid_unpaved: false,
            voice_enabled: true,
            auto_reroute: true,
            show_speed_limit: true,
            show_traffic: true,
        }
    }
}

impl NavPreferences {
    /// Apply a route preset, updating relevant fields.
    pub fn apply_preset(&mut self, preset: RoutePreset) {
        self.route_preset = preset;
        if preset == RoutePreset::NoHighways {
            // No additional avoidance changes needed — routing engine handles it
        }
        if preset == RoutePreset::Eco {
            self.show_traffic = true; // eco needs traffic data
        }
    }

    /// Toggle between metric and imperial.
    pub fn toggle_units(&mut self) {
        self.units = match self.units {
            UnitSystem::Metric => UnitSystem::Imperial,
            UnitSystem::Imperial => UnitSystem::Metric,
            UnitSystem::Mixed => UnitSystem::Metric,
        };
    }

    /// Whether voice guidance should be active given current settings.
    pub fn should_speak(&self) -> bool {
        self.voice_enabled && self.notification_level >= NotificationLevel::Normal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_preset_weights() {
        assert!(RoutePreset::Fastest.time_weight() > RoutePreset::Scenic.time_weight());
        assert!(RoutePreset::Shortest.distance_weight() > RoutePreset::Fastest.distance_weight());
        assert!(RoutePreset::Eco.fuel_weight() > RoutePreset::Fastest.fuel_weight());
    }

    #[test]
    fn test_unit_format_distance_metric() {
        assert_eq!(UnitSystem::Metric.format_distance(500.0), "500 m");
        assert_eq!(UnitSystem::Metric.format_distance(1500.0), "1.5 km");
    }

    #[test]
    fn test_unit_format_distance_imperial() {
        let text = UnitSystem::Imperial.format_distance(1609.344);
        assert!(text.contains("1.0 mi"));
    }

    #[test]
    fn test_unit_format_speed() {
        // 10 m/s = 36 km/h
        assert_eq!(UnitSystem::Metric.format_speed(10.0), "36 km/h");
        // 10 m/s ≈ 22 mph
        assert_eq!(UnitSystem::Imperial.format_speed(10.0), "22 mph");
    }

    #[test]
    fn test_default_preferences() {
        let prefs = NavPreferences::default();
        assert_eq!(prefs.route_preset, RoutePreset::Balanced);
        assert!(prefs.auto_reroute);
        assert!(prefs.voice_enabled);
    }

    #[test]
    fn test_apply_preset() {
        let mut prefs = NavPreferences::default();
        prefs.apply_preset(RoutePreset::Eco);
        assert_eq!(prefs.route_preset, RoutePreset::Eco);
        assert!(prefs.show_traffic);
    }

    #[test]
    fn test_toggle_units() {
        let mut prefs = NavPreferences::default();
        assert_eq!(prefs.units, UnitSystem::Metric);
        prefs.toggle_units();
        assert_eq!(prefs.units, UnitSystem::Imperial);
        prefs.toggle_units();
        assert_eq!(prefs.units, UnitSystem::Metric);
    }

    #[test]
    fn test_should_speak() {
        let mut prefs = NavPreferences::default();
        assert!(prefs.should_speak());
        prefs.voice_enabled = false;
        assert!(!prefs.should_speak());
        prefs.voice_enabled = true;
        prefs.notification_level = NotificationLevel::Minimal;
        assert!(!prefs.should_speak());
    }

    #[test]
    fn test_privacy_defaults() {
        let p = PrivacySettings::default();
        assert!(p.analytics);
        assert!(p.save_history);
        assert_eq!(p.max_history, 100);
    }

    #[test]
    fn test_short_imperial_distance() {
        let text = UnitSystem::Imperial.format_distance(30.0);
        assert!(text.contains("ft"));
    }
}
