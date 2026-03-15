//! Altimeter — altitude measurement, barometric correction, vertical
//! speed computation, and elevation profile tracking.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Altitude source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AltitudeSource {
    /// GNSS-derived altitude
    Gnss,
    /// Barometric pressure sensor
    Barometric,
    /// Fused (GNSS + barometric)
    Fused,
    /// Digital elevation model lookup
    Dem,
    /// Manual entry
    Manual,
}

/// A single altitude reading.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AltitudeReading {
    /// Altitude above mean sea level (meters)
    pub altitude_msl_m: f64,
    /// Altitude above WGS-84 ellipsoid (meters)
    pub altitude_wgs84_m: Option<f64>,
    /// Vertical accuracy (meters, 95% confidence)
    pub accuracy_m: f64,
    /// Source of measurement
    pub source: AltitudeSource,
    /// Raw pressure (hPa), if barometric
    pub pressure_hpa: Option<f64>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Altimeter with vertical speed computation and elevation tracking.
#[derive(Debug)]
pub struct Altimeter {
    /// Current altitude MSL (meters)
    current_altitude_m: f64,
    /// History for smoothing and vertical speed
    history: Vec<(f64, DateTime<Utc>)>,
    /// Max history entries
    max_history: usize,
    /// QNH setting (sea-level pressure, hPa)
    qnh_hpa: f64,
    /// Maximum altitude recorded
    max_altitude_m: f64,
    /// Minimum altitude recorded
    min_altitude_m: f64,
    /// Total ascent (meters)
    total_ascent_m: f64,
    /// Total descent (meters)
    total_descent_m: f64,
    /// Last reading
    last_reading: Option<AltitudeReading>,
}

impl Altimeter {
    /// Create a new altimeter with default QNH (1013.25 hPa).
    pub fn new() -> Self {
        Self {
            current_altitude_m: 0.0,
            history: Vec::new(),
            max_history: 20,
            qnh_hpa: 1013.25,
            max_altitude_m: f64::MIN,
            min_altitude_m: f64::MAX,
            total_ascent_m: 0.0,
            total_descent_m: 0.0,
            last_reading: None,
        }
    }

    /// Set the QNH (sea-level pressure) for barometric altitude correction.
    pub fn set_qnh(&mut self, qnh_hpa: f64) {
        self.qnh_hpa = qnh_hpa;
    }

    /// Get current QNH setting.
    pub fn qnh(&self) -> f64 {
        self.qnh_hpa
    }

    /// Convert barometric pressure to altitude using the international
    /// barometric formula.
    pub fn pressure_to_altitude(&self, pressure_hpa: f64) -> f64 {
        // International barometric formula
        // h = 44330 * (1 - (P/P0)^(1/5.255))
        44330.0 * (1.0 - (pressure_hpa / self.qnh_hpa).powf(1.0 / 5.255))
    }

    /// Process a new altitude reading.
    pub fn update(&mut self, reading: AltitudeReading) {
        let alt = reading.altitude_msl_m;
        let prev_alt = self.current_altitude_m;

        self.current_altitude_m = alt;
        self.history.insert(0, (alt, reading.timestamp));
        if self.history.len() > self.max_history {
            self.history.truncate(self.max_history);
        }

        // Update min/max
        if alt > self.max_altitude_m {
            self.max_altitude_m = alt;
        }
        if alt < self.min_altitude_m {
            self.min_altitude_m = alt;
        }

        // Update total ascent/descent (with noise threshold)
        if self.last_reading.is_some() {
            let diff = alt - prev_alt;
            if diff > 1.0 {
                self.total_ascent_m += diff;
            } else if diff < -1.0 {
                self.total_descent_m += diff.abs();
            }
        }

        self.last_reading = Some(reading);
    }

    /// Get current altitude in meters.
    pub fn altitude(&self) -> f64 {
        self.current_altitude_m
    }

    /// Compute vertical speed (m/s) from recent history.
    /// Positive = ascending, negative = descending.
    pub fn vertical_speed_mps(&self) -> f64 {
        if self.history.len() < 2 {
            return 0.0;
        }
        let newest = &self.history[0];
        let oldest = &self.history[self.history.len() - 1];
        let dt = newest.1.signed_duration_since(oldest.1).num_milliseconds() as f64 / 1000.0;
        if dt.abs() < 0.01 {
            return 0.0;
        }
        (newest.0 - oldest.0) / dt
    }

    /// Maximum altitude recorded.
    pub fn max_altitude(&self) -> f64 {
        if self.max_altitude_m == f64::MIN {
            0.0
        } else {
            self.max_altitude_m
        }
    }

    /// Minimum altitude recorded.
    pub fn min_altitude(&self) -> f64 {
        if self.min_altitude_m == f64::MAX {
            0.0
        } else {
            self.min_altitude_m
        }
    }

    /// Total ascent (meters).
    pub fn total_ascent(&self) -> f64 {
        self.total_ascent_m
    }

    /// Total descent (meters).
    pub fn total_descent(&self) -> f64 {
        self.total_descent_m
    }

    /// Elevation gain (ascent - descent).
    pub fn elevation_gain(&self) -> f64 {
        self.total_ascent_m - self.total_descent_m
    }

    /// Format altitude as string with unit.
    pub fn formatted_altitude(&self) -> String {
        format!("{:.0} m", self.current_altitude_m)
    }

    /// Get the last reading.
    pub fn last_reading(&self) -> Option<&AltitudeReading> {
        self.last_reading.as_ref()
    }
}

impl Default for Altimeter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_reading(alt: f64) -> AltitudeReading {
        AltitudeReading {
            altitude_msl_m: alt,
            altitude_wgs84_m: None,
            accuracy_m: 3.0,
            source: AltitudeSource::Gnss,
            pressure_hpa: None,
            timestamp: Utc::now(),
        }
    }

    #[test]
    fn test_altimeter_update() {
        let mut alt = Altimeter::new();
        alt.update(make_reading(100.0));
        assert!((alt.altitude() - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_altimeter_min_max() {
        let mut alt = Altimeter::new();
        alt.update(make_reading(100.0));
        alt.update(make_reading(200.0));
        alt.update(make_reading(150.0));
        assert!((alt.max_altitude() - 200.0).abs() < f64::EPSILON);
        assert!((alt.min_altitude() - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_altimeter_ascent_descent() {
        let mut alt = Altimeter::new();
        alt.update(make_reading(100.0));
        alt.update(make_reading(150.0)); // +50 ascent
        alt.update(make_reading(120.0)); // -30 descent
        assert!((alt.total_ascent() - 50.0).abs() < 0.1);
        assert!((alt.total_descent() - 30.0).abs() < 0.1);
    }

    #[test]
    fn test_pressure_to_altitude() {
        let alt = Altimeter::new();
        // Standard pressure at sea level should give ~0 altitude
        let sea_level = alt.pressure_to_altitude(1013.25);
        assert!(sea_level.abs() < 1.0);

        // Lower pressure = higher altitude
        let higher = alt.pressure_to_altitude(900.0);
        assert!(higher > sea_level);
    }

    #[test]
    fn test_altimeter_no_readings() {
        let alt = Altimeter::new();
        assert!((alt.altitude()).abs() < f64::EPSILON);
        assert!((alt.max_altitude()).abs() < f64::EPSILON);
        assert!((alt.min_altitude()).abs() < f64::EPSILON);
        assert!((alt.vertical_speed_mps()).abs() < f64::EPSILON);
    }

    #[test]
    fn test_formatted_altitude() {
        let mut alt = Altimeter::new();
        alt.update(make_reading(1234.0));
        assert_eq!(alt.formatted_altitude(), "1234 m");
    }
}
