//! Weather integration — current conditions, forecasts, road weather
//! impact assessment, and environmental sensing.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Weather condition type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeatherCondition {
    Clear,
    PartlyCloudy,
    Cloudy,
    Overcast,
    LightRain,
    Rain,
    HeavyRain,
    Thunderstorm,
    LightSnow,
    Snow,
    HeavySnow,
    Sleet,
    Hail,
    Fog,
    DenseFog,
    Mist,
    Dust,
    Sandstorm,
    Smoke,
    WindyCalm,
    WindyStrong,
    WindyGale,
}

impl WeatherCondition {
    /// Road safety impact score (0.0 = no impact, 1.0 = severe impact).
    pub fn road_impact(&self) -> f64 {
        match self {
            Self::Clear | Self::PartlyCloudy | Self::WindyCalm => 0.0,
            Self::Cloudy | Self::Overcast | Self::Mist => 0.1,
            Self::LightRain | Self::LightSnow => 0.3,
            Self::Rain | Self::Fog | Self::Dust => 0.5,
            Self::HeavyRain | Self::Snow | Self::Sleet | Self::WindyStrong => 0.7,
            Self::Thunderstorm | Self::HeavySnow | Self::Hail | Self::Smoke => 0.8,
            Self::DenseFog | Self::Sandstorm | Self::WindyGale => 0.9,
        }
    }

    /// Visibility multiplier (1.0 = full visibility, 0.0 = zero visibility).
    pub fn visibility_factor(&self) -> f64 {
        match self {
            Self::Clear | Self::PartlyCloudy | Self::WindyCalm => 1.0,
            Self::Cloudy | Self::Overcast => 0.95,
            Self::LightRain | Self::LightSnow => 0.8,
            Self::Rain | Self::Snow | Self::Mist => 0.6,
            Self::HeavyRain | Self::HeavySnow | Self::Sleet | Self::Dust | Self::Smoke => 0.3,
            Self::Thunderstorm | Self::Hail | Self::WindyStrong | Self::WindyGale => 0.4,
            Self::Fog => 0.2,
            Self::DenseFog | Self::Sandstorm => 0.05,
        }
    }
}

/// Wind direction and speed.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Wind {
    /// Wind direction (degrees, 0=N, 90=E)
    pub direction_deg: f64,
    /// Wind speed (m/s)
    pub speed_mps: f64,
    /// Gust speed (m/s)
    pub gust_mps: Option<f64>,
}

/// A weather observation at a specific location and time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherObservation {
    /// Primary weather condition
    pub condition: WeatherCondition,
    /// Temperature (Celsius)
    pub temperature_c: f64,
    /// Feels-like temperature (Celsius)
    pub feels_like_c: f64,
    /// Relative humidity (0-100%)
    pub humidity_pct: f64,
    /// Atmospheric pressure (hPa)
    pub pressure_hpa: f64,
    /// Visibility (meters)
    pub visibility_m: f64,
    /// Wind data
    pub wind: Wind,
    /// UV index (0-11+)
    pub uv_index: Option<f64>,
    /// Precipitation rate (mm/h)
    pub precipitation_rate_mmh: f64,
    /// Latitude of observation
    pub lat: f64,
    /// Longitude of observation
    pub lon: f64,
    /// Observation timestamp
    pub timestamp: DateTime<Utc>,
    /// Data source identifier
    pub source: String,
}

/// Road weather assessment for navigation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoadWeatherAssessment {
    /// Overall road safety score (0.0 = safe, 1.0 = dangerous)
    pub danger_score: f64,
    /// Whether to recommend reduced speed
    pub recommend_reduced_speed: bool,
    /// Recommended speed reduction factor (e.g., 0.8 = reduce by 20%)
    pub speed_factor: f64,
    /// Specific warnings
    pub warnings: Vec<String>,
    /// Recommended headlight mode
    pub headlight_recommendation: HeadlightMode,
    /// Assessment timestamp
    pub assessed_at: DateTime<Utc>,
}

/// Headlight recommendation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeadlightMode {
    /// Normal daytime (no headlights needed)
    Off,
    /// Daytime running lights
    Drl,
    /// Low beam
    LowBeam,
    /// Fog lights
    FogLights,
    /// Full beam (no oncoming traffic)
    HighBeam,
}

/// Weather engine for road impact assessment.
#[derive(Debug)]
pub struct WeatherEngine {
    /// Latest observation
    current: Option<WeatherObservation>,
    /// History of observations
    history: Vec<WeatherObservation>,
    /// Max history entries
    max_history: usize,
}

impl WeatherEngine {
    /// Create a new weather engine.
    pub fn new() -> Self {
        Self {
            current: None,
            history: Vec::new(),
            max_history: 24,
        }
    }

    /// Update with a new weather observation.
    pub fn update(&mut self, obs: WeatherObservation) {
        if let Some(prev) = self.current.take() {
            self.history.insert(0, prev);
            if self.history.len() > self.max_history {
                self.history.truncate(self.max_history);
            }
        }
        self.current = Some(obs);
    }

    /// Get the current observation.
    pub fn current(&self) -> Option<&WeatherObservation> {
        self.current.as_ref()
    }

    /// Assess road weather conditions for navigation.
    pub fn assess_road_conditions(&self) -> Option<RoadWeatherAssessment> {
        let obs = self.current.as_ref()?;
        let danger = obs.condition.road_impact();
        let visibility_factor = obs.condition.visibility_factor();

        let mut warnings = Vec::new();
        let mut speed_factor: f64 = 1.0;

        // Temperature-based warnings
        if obs.temperature_c <= 3.0 {
            warnings.push("Risk of ice on road surface".to_string());
            speed_factor = speed_factor.min(0.7);
        }
        if obs.temperature_c <= 0.0 {
            warnings.push("Freezing conditions — black ice likely".to_string());
            speed_factor = speed_factor.min(0.5);
        }

        // Visibility-based warnings
        if obs.visibility_m < 200.0 {
            warnings.push("Very low visibility".to_string());
            speed_factor = speed_factor.min(0.4);
        } else if obs.visibility_m < 500.0 {
            warnings.push("Reduced visibility".to_string());
            speed_factor = speed_factor.min(0.6);
        }

        // Wind-based warnings
        if obs.wind.speed_mps > 20.0 {
            warnings.push("Strong crosswinds — maintain firm grip".to_string());
            speed_factor = speed_factor.min(0.7);
        }

        // Precipitation-based adjustments
        if obs.precipitation_rate_mmh > 10.0 {
            speed_factor = speed_factor.min(0.6);
            warnings.push("Heavy precipitation — increased stopping distance".to_string());
        }

        // Weather condition-based speed adjustment
        let condition_factor = 1.0 - (danger * 0.5);
        speed_factor = speed_factor.min(condition_factor);

        // Headlight recommendation
        let headlight = if visibility_factor < 0.2 {
            HeadlightMode::FogLights
        } else if visibility_factor < 0.6 {
            HeadlightMode::LowBeam
        } else if visibility_factor < 0.9 {
            HeadlightMode::Drl
        } else {
            HeadlightMode::Off
        };

        Some(RoadWeatherAssessment {
            danger_score: danger,
            recommend_reduced_speed: speed_factor < 1.0,
            speed_factor,
            warnings,
            headlight_recommendation: headlight,
            assessed_at: Utc::now(),
        })
    }

    /// Get observation history.
    pub fn history(&self) -> &[WeatherObservation] {
        &self.history
    }
}

impl Default for WeatherEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_obs(condition: WeatherCondition, temp_c: f64, vis_m: f64) -> WeatherObservation {
        WeatherObservation {
            condition,
            temperature_c: temp_c,
            feels_like_c: temp_c - 2.0,
            humidity_pct: 60.0,
            pressure_hpa: 1013.0,
            visibility_m: vis_m,
            wind: Wind {
                direction_deg: 180.0,
                speed_mps: 5.0,
                gust_mps: None,
            },
            uv_index: Some(5.0),
            precipitation_rate_mmh: 0.0,
            lat: 32.0,
            lon: 34.0,
            timestamp: Utc::now(),
            source: "test".to_string(),
        }
    }

    #[test]
    fn test_weather_condition_impact() {
        assert!((WeatherCondition::Clear.road_impact()).abs() < f64::EPSILON);
        assert!(WeatherCondition::DenseFog.road_impact() > 0.8);
        assert!(WeatherCondition::Rain.road_impact() > 0.0);
    }

    #[test]
    fn test_weather_visibility_factor() {
        assert!((WeatherCondition::Clear.visibility_factor() - 1.0).abs() < f64::EPSILON);
        assert!(WeatherCondition::DenseFog.visibility_factor() < 0.1);
    }

    #[test]
    fn test_weather_engine_assess_clear() {
        let mut engine = WeatherEngine::new();
        engine.update(make_obs(WeatherCondition::Clear, 20.0, 10000.0));

        let assessment = engine.assess_road_conditions().unwrap();
        assert!((assessment.danger_score).abs() < f64::EPSILON);
        assert!(!assessment.recommend_reduced_speed);
        assert_eq!(assessment.headlight_recommendation, HeadlightMode::Off);
    }

    #[test]
    fn test_weather_engine_assess_fog() {
        let mut engine = WeatherEngine::new();
        engine.update(make_obs(WeatherCondition::DenseFog, 5.0, 50.0));

        let assessment = engine.assess_road_conditions().unwrap();
        assert!(assessment.danger_score > 0.8);
        assert!(assessment.recommend_reduced_speed);
        assert!(assessment.speed_factor < 0.5);
        assert_eq!(
            assessment.headlight_recommendation,
            HeadlightMode::FogLights
        );
    }

    #[test]
    fn test_weather_engine_freezing() {
        let mut engine = WeatherEngine::new();
        engine.update(make_obs(WeatherCondition::Clear, -5.0, 10000.0));

        let assessment = engine.assess_road_conditions().unwrap();
        assert!(assessment.recommend_reduced_speed);
        assert!(!assessment.warnings.is_empty());
    }

    #[test]
    fn test_weather_engine_no_observation() {
        let engine = WeatherEngine::new();
        assert!(engine.assess_road_conditions().is_none());
    }

    #[test]
    fn test_weather_engine_history() {
        let mut engine = WeatherEngine::new();
        engine.update(make_obs(WeatherCondition::Clear, 20.0, 10000.0));
        engine.update(make_obs(WeatherCondition::Rain, 15.0, 5000.0));

        assert_eq!(engine.history().len(), 1);
        assert_eq!(engine.current().unwrap().condition, WeatherCondition::Rain);
    }
}
