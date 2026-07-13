//! Speedometer — speed measurement, unit conversion, speed statistics,
//! and speed limit comparison.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Speed measurement source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpeedSource {
    /// GNSS-derived speed
    Gnss,
    /// Wheel speed sensor
    WheelSensor,
    /// OBD-II vehicle data
    Obd2,
    /// Fused (multiple sources)
    Fused,
    /// IMU-based (accelerometer integration)
    Imu,
}

/// Speed units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpeedUnit {
    /// Meters per second
    Mps,
    /// Kilometers per hour
    Kmh,
    /// Miles per hour
    Mph,
    /// Knots (nautical)
    Knots,
}

impl SpeedUnit {
    /// Convert m/s to this unit.
    pub fn from_mps(&self, mps: f64) -> f64 {
        match self {
            Self::Mps => mps,
            Self::Kmh => mps * 3.6,
            Self::Mph => mps * 2.23694,
            Self::Knots => mps * 1.94384,
        }
    }

    /// Convert from this unit to m/s.
    pub fn to_mps(&self, value: f64) -> f64 {
        match self {
            Self::Mps => value,
            Self::Kmh => value / 3.6,
            Self::Mph => value / 2.23694,
            Self::Knots => value / 1.94384,
        }
    }

    /// Unit suffix string.
    pub fn suffix(&self) -> &'static str {
        match self {
            Self::Mps => "m/s",
            Self::Kmh => "km/h",
            Self::Mph => "mph",
            Self::Knots => "kn",
        }
    }
}

/// A speed reading.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SpeedReading {
    /// Speed in m/s
    pub speed_mps: f64,
    /// Source of measurement
    pub source: SpeedSource,
    /// Accuracy estimate (m/s)
    pub accuracy_mps: f64,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Speedometer with smoothing, statistics, and speed limit tracking.
#[derive(Debug)]
pub struct Speedometer {
    /// Current speed (m/s)
    current_mps: f64,
    /// History for smoothing
    history: Vec<f64>,
    /// Max history entries
    max_history: usize,
    /// Display unit
    display_unit: SpeedUnit,
    /// Maximum speed recorded (m/s)
    max_speed_mps: f64,
    /// Average speed accumulator (sum of speeds)
    speed_sum: f64,
    /// Number of readings for average
    reading_count: u64,
    /// Current speed limit (m/s)
    speed_limit_mps: Option<f64>,
    /// Total distance traveled (meters)
    odometer_m: f64,
    /// Timestamp of last reading
    last_timestamp: Option<DateTime<Utc>>,
}

impl Speedometer {
    /// Create a new speedometer with the given display unit.
    pub fn new(display_unit: SpeedUnit) -> Self {
        Self {
            current_mps: 0.0,
            history: Vec::new(),
            max_history: 5,
            display_unit,
            max_speed_mps: 0.0,
            speed_sum: 0.0,
            reading_count: 0,
            speed_limit_mps: None,
            odometer_m: 0.0,
            last_timestamp: None,
        }
    }

    /// Process a new speed reading.
    pub fn update(&mut self, reading: SpeedReading) {
        // Update odometer based on time elapsed
        if let Some(last_ts) = self.last_timestamp {
            let dt = reading
                .timestamp
                .signed_duration_since(last_ts)
                .num_milliseconds() as f64
                / 1000.0;
            if dt > 0.0 && dt < 10.0 {
                self.odometer_m += self.current_mps * dt;
            }
        }

        self.history.insert(0, reading.speed_mps);
        if self.history.len() > self.max_history {
            self.history.truncate(self.max_history);
        }

        // Simple moving average
        self.current_mps = self.history.iter().sum::<f64>() / self.history.len() as f64;

        if reading.speed_mps > self.max_speed_mps {
            self.max_speed_mps = reading.speed_mps;
        }
        self.speed_sum += reading.speed_mps;
        self.reading_count += 1;
        self.last_timestamp = Some(reading.timestamp);
    }

    /// Get current speed in display units.
    pub fn current_speed(&self) -> f64 {
        self.display_unit.from_mps(self.current_mps)
    }

    /// Get current speed in m/s.
    pub fn current_speed_mps(&self) -> f64 {
        self.current_mps
    }

    /// Get max speed in display units.
    pub fn max_speed(&self) -> f64 {
        self.display_unit.from_mps(self.max_speed_mps)
    }

    /// Get average speed in display units.
    pub fn average_speed(&self) -> f64 {
        if self.reading_count == 0 {
            return 0.0;
        }
        let avg_mps = self.speed_sum / self.reading_count as f64;
        self.display_unit.from_mps(avg_mps)
    }

    /// Set the current speed limit.
    pub fn set_speed_limit(&mut self, limit_mps: f64) {
        self.speed_limit_mps = Some(limit_mps);
    }

    /// Clear the speed limit.
    pub fn clear_speed_limit(&mut self) {
        self.speed_limit_mps = None;
    }

    /// Check if currently exceeding speed limit.
    pub fn is_over_limit(&self) -> bool {
        self.speed_limit_mps
            .is_some_and(|limit| self.current_mps > limit)
    }

    /// How much over the limit (in display units). Negative if under.
    pub fn speed_over_limit(&self) -> Option<f64> {
        self.speed_limit_mps
            .map(|limit| self.display_unit.from_mps(self.current_mps - limit))
    }

    /// Total distance traveled in display-appropriate units.
    pub fn odometer_km(&self) -> f64 {
        self.odometer_m / 1000.0
    }

    /// Format current speed as a string.
    pub fn formatted_speed(&self) -> String {
        format!("{:.0} {}", self.current_speed(), self.display_unit.suffix())
    }

    /// Get the display unit.
    pub fn display_unit(&self) -> SpeedUnit {
        self.display_unit
    }

    /// Change the display unit.
    pub fn set_display_unit(&mut self, unit: SpeedUnit) {
        self.display_unit = unit;
    }
}

impl Default for Speedometer {
    fn default() -> Self {
        Self::new(SpeedUnit::Kmh)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_reading(speed_mps: f64) -> SpeedReading {
        SpeedReading {
            speed_mps,
            source: SpeedSource::Gnss,
            accuracy_mps: 0.5,
            timestamp: Utc::now(),
        }
    }

    #[test]
    fn test_speed_unit_conversion() {
        let kmh = SpeedUnit::Kmh;
        assert!((kmh.from_mps(10.0) - 36.0).abs() < 0.01);
        assert!((kmh.to_mps(36.0) - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_speedometer_update() {
        let mut speedo = Speedometer::new(SpeedUnit::Kmh);
        speedo.update(make_reading(10.0));
        assert!((speedo.current_speed() - 36.0).abs() < 0.1);
    }

    #[test]
    fn test_speedometer_smoothing() {
        let mut speedo = Speedometer::new(SpeedUnit::Mps);
        speedo.update(make_reading(10.0));
        speedo.update(make_reading(20.0));
        // Average of 10 and 20 = 15
        assert!((speedo.current_speed_mps() - 15.0).abs() < 0.01);
    }

    #[test]
    fn test_speedometer_max_speed() {
        let mut speedo = Speedometer::new(SpeedUnit::Mps);
        speedo.update(make_reading(10.0));
        speedo.update(make_reading(20.0));
        speedo.update(make_reading(5.0));
        assert!((speedo.max_speed() - 20.0).abs() < 1.0);
    }

    #[test]
    fn test_speed_limit() {
        let mut speedo = Speedometer::new(SpeedUnit::Kmh);
        speedo.set_speed_limit(16.67); // 60 km/h in m/s
        speedo.update(make_reading(20.0)); // ~72 km/h
        assert!(speedo.is_over_limit());

        speedo.update(make_reading(10.0)); // will be average with prev
                                           // Still might be over depending on smoothing
    }

    #[test]
    fn test_speed_limit_not_set() {
        let speedo = Speedometer::new(SpeedUnit::Kmh);
        assert!(!speedo.is_over_limit());
        assert!(speedo.speed_over_limit().is_none());
    }

    #[test]
    fn test_formatted_speed() {
        let mut speedo = Speedometer::new(SpeedUnit::Kmh);
        speedo.update(make_reading(10.0));
        let formatted = speedo.formatted_speed();
        assert!(formatted.contains("km/h"));
    }
}
