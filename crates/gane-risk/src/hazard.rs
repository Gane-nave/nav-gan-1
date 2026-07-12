//! Hazard forecaster — predicts environmental hazards along a route.

use chrono::{DateTime, Utc};
use gane_core::types::EntityId;
use serde::{Deserialize, Serialize};
use tracing::debug;

/// A forecasted hazard on a road segment or region.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HazardForecast {
    pub id: EntityId,
    pub hazard_type: HazardType,
    /// Probability of the hazard occurring [0, 1].
    pub probability: f64,
    /// Severity if it occurs [0, 1].
    pub severity: f64,
    /// Combined risk = probability * severity.
    pub risk: f64,
    /// Affected segment or region ID.
    pub affected_entity: EntityId,
    /// When the hazard is expected.
    pub expected_onset: DateTime<Utc>,
    /// Expected duration in seconds.
    pub expected_duration_s: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HazardType {
    Rain,
    Snow,
    Ice,
    Fog,
    StrongWind,
    Flooding,
    Sandstorm,
    Wildfire,
    RoadDebris,
    Construction,
    Accident,
    AnimalCrossing,
    Landslide,
    PoorLighting,
}

/// Forecasts hazards for segments based on environmental data.
pub struct HazardForecaster {
    /// Base probability threshold for rain hazard.
    pub rain_threshold: f64,
    /// Base probability threshold for fog hazard.
    pub fog_threshold: f64,
}

impl HazardForecaster {
    pub fn new() -> Self {
        Self {
            rain_threshold: 0.3,
            fog_threshold: 0.5,
        }
    }

    /// Forecast hazards for a segment given weather observations.
    pub fn forecast_segment(
        &self,
        segment_id: EntityId,
        obs: &WeatherObservation,
    ) -> Vec<HazardForecast> {
        let mut hazards = Vec::new();
        let now = Utc::now();

        // Rain hazard.
        if obs.precipitation_mm_h > 0.0 {
            let probability = (obs.precipitation_mm_h / 20.0).min(1.0);
            let severity = match obs.precipitation_mm_h {
                p if p < 2.5 => 0.2,  // light
                p if p < 7.5 => 0.5,  // moderate
                p if p < 20.0 => 0.7, // heavy
                _ => 0.9,             // extreme
            };
            hazards.push(HazardForecast {
                id: EntityId::new(),
                hazard_type: HazardType::Rain,
                probability,
                severity,
                risk: probability * severity,
                affected_entity: segment_id,
                expected_onset: now,
                expected_duration_s: 3600.0,
            });
        }

        // Fog hazard.
        if obs.visibility_km < 1.0 {
            let probability = (1.0 - obs.visibility_km).clamp(0.0, 1.0);
            let severity = if obs.visibility_km < 0.2 {
                0.9
            } else if obs.visibility_km < 0.5 {
                0.6
            } else {
                0.3
            };
            hazards.push(HazardForecast {
                id: EntityId::new(),
                hazard_type: HazardType::Fog,
                probability,
                severity,
                risk: probability * severity,
                affected_entity: segment_id,
                expected_onset: now,
                expected_duration_s: 7200.0,
            });
        }

        // Ice hazard.
        if obs.temperature_c < 3.0 && obs.precipitation_mm_h > 0.0 {
            let probability = if obs.temperature_c < 0.0 { 0.9 } else { 0.4 };
            let severity = 0.8;
            hazards.push(HazardForecast {
                id: EntityId::new(),
                hazard_type: HazardType::Ice,
                probability,
                severity,
                risk: probability * severity,
                affected_entity: segment_id,
                expected_onset: now,
                expected_duration_s: 14400.0,
            });
        }

        // Strong wind hazard.
        if obs.wind_speed_kmh > 60.0 {
            let probability = ((obs.wind_speed_kmh - 60.0) / 60.0).min(1.0);
            let severity = if obs.wind_speed_kmh > 100.0 { 0.9 } else { 0.5 };
            hazards.push(HazardForecast {
                id: EntityId::new(),
                hazard_type: HazardType::StrongWind,
                probability,
                severity,
                risk: probability * severity,
                affected_entity: segment_id,
                expected_onset: now,
                expected_duration_s: 3600.0,
            });
        }

        // Poor lighting.
        if obs.is_night && !obs.has_street_lighting {
            hazards.push(HazardForecast {
                id: EntityId::new(),
                hazard_type: HazardType::PoorLighting,
                probability: 1.0,
                severity: 0.3,
                risk: 0.3,
                affected_entity: segment_id,
                expected_onset: now,
                expected_duration_s: 28800.0,
            });
        }

        debug!(
            segment_id = %segment_id,
            hazard_count = hazards.len(),
            "hazards forecasted"
        );

        hazards
    }

    /// Get the maximum hazard risk for a set of forecasts.
    pub fn max_risk(forecasts: &[HazardForecast]) -> f64 {
        forecasts.iter().map(|h| h.risk).fold(0.0_f64, f64::max)
    }

    /// Filter forecasts above a risk threshold.
    pub fn critical_hazards(forecasts: &[HazardForecast], threshold: f64) -> Vec<&HazardForecast> {
        forecasts.iter().filter(|h| h.risk >= threshold).collect()
    }
}

impl Default for HazardForecaster {
    fn default() -> Self {
        Self::new()
    }
}

/// Weather observation data used for hazard forecasting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherObservation {
    pub temperature_c: f64,
    pub precipitation_mm_h: f64,
    pub visibility_km: f64,
    pub wind_speed_kmh: f64,
    pub humidity_pct: f64,
    pub is_night: bool,
    pub has_street_lighting: bool,
}

impl Default for WeatherObservation {
    fn default() -> Self {
        Self {
            temperature_c: 20.0,
            precipitation_mm_h: 0.0,
            visibility_km: 10.0,
            wind_speed_kmh: 10.0,
            humidity_pct: 50.0,
            is_night: false,
            has_street_lighting: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_hazards_in_clear_weather() {
        let forecaster = HazardForecaster::new();
        let segment_id = EntityId::new();
        let obs = WeatherObservation::default();

        let hazards = forecaster.forecast_segment(segment_id, &obs);
        assert!(hazards.is_empty());
    }

    #[test]
    fn rain_hazard_detected() {
        let forecaster = HazardForecaster::new();
        let segment_id = EntityId::new();
        let obs = WeatherObservation {
            precipitation_mm_h: 10.0,
            ..Default::default()
        };

        let hazards = forecaster.forecast_segment(segment_id, &obs);
        assert_eq!(hazards.len(), 1);
        assert_eq!(hazards[0].hazard_type, HazardType::Rain);
        assert!(hazards[0].probability > 0.0);
        assert!(hazards[0].severity > 0.0);
    }

    #[test]
    fn fog_and_ice_together() {
        let forecaster = HazardForecaster::new();
        let segment_id = EntityId::new();
        let obs = WeatherObservation {
            temperature_c: -2.0,
            precipitation_mm_h: 1.0,
            visibility_km: 0.3,
            ..Default::default()
        };

        let hazards = forecaster.forecast_segment(segment_id, &obs);
        let types: Vec<HazardType> = hazards.iter().map(|h| h.hazard_type).collect();
        assert!(types.contains(&HazardType::Rain));
        assert!(types.contains(&HazardType::Fog));
        assert!(types.contains(&HazardType::Ice));
    }

    #[test]
    fn poor_lighting_at_night() {
        let forecaster = HazardForecaster::new();
        let segment_id = EntityId::new();
        let obs = WeatherObservation {
            is_night: true,
            has_street_lighting: false,
            ..Default::default()
        };

        let hazards = forecaster.forecast_segment(segment_id, &obs);
        assert_eq!(hazards.len(), 1);
        assert_eq!(hazards[0].hazard_type, HazardType::PoorLighting);
    }

    #[test]
    fn max_risk_calculation() {
        let h1 = HazardForecast {
            id: EntityId::new(),
            hazard_type: HazardType::Rain,
            probability: 0.5,
            severity: 0.4,
            risk: 0.2,
            affected_entity: EntityId::new(),
            expected_onset: Utc::now(),
            expected_duration_s: 3600.0,
        };
        let h2 = HazardForecast {
            id: EntityId::new(),
            hazard_type: HazardType::Ice,
            probability: 0.9,
            severity: 0.8,
            risk: 0.72,
            affected_entity: EntityId::new(),
            expected_onset: Utc::now(),
            expected_duration_s: 7200.0,
        };

        assert!((HazardForecaster::max_risk(&[h1.clone(), h2.clone()]) - 0.72).abs() < 0.001);
        assert_eq!(HazardForecaster::critical_hazards(&[h1, h2], 0.5).len(), 1);
    }
}
