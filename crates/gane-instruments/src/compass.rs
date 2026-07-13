//! Compass — magnetic and true heading computation, declination correction,
//! and heading smoothing for stable compass display.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Compass heading source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeadingSource {
    /// Magnetometer (magnetic heading)
    Magnetometer,
    /// GNSS course-over-ground
    GnssCog,
    /// Fused (magnetometer + GNSS)
    Fused,
    /// Manual override
    Manual,
}

/// A single compass reading.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CompassReading {
    /// Magnetic heading (degrees, 0-360)
    pub magnetic_heading_deg: f64,
    /// True heading (degrees, 0-360), after declination correction
    pub true_heading_deg: f64,
    /// Magnetic declination applied (degrees, east positive)
    pub declination_deg: f64,
    /// Accuracy of the reading (degrees, lower is better)
    pub accuracy_deg: f64,
    /// Source of the heading
    pub source: HeadingSource,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Compass engine with heading smoothing and declination correction.
#[derive(Debug)]
pub struct Compass {
    /// Current magnetic declination (degrees, east positive)
    declination_deg: f64,
    /// Heading history for smoothing (newest first)
    history: Vec<f64>,
    /// Maximum history size
    max_history: usize,
    /// Current smoothed heading
    smoothed_heading: f64,
    /// Whether compass is calibrated
    calibrated: bool,
    /// Last reading
    last_reading: Option<CompassReading>,
}

impl Compass {
    /// Create a new compass with the given magnetic declination.
    pub fn new(declination_deg: f64) -> Self {
        Self {
            declination_deg,
            history: Vec::new(),
            max_history: 10,
            smoothed_heading: 0.0,
            calibrated: false,
            last_reading: None,
        }
    }

    /// Update the magnetic declination.
    pub fn set_declination(&mut self, declination_deg: f64) {
        self.declination_deg = declination_deg;
    }

    /// Process a new magnetic heading reading and return the computed result.
    pub fn update(&mut self, magnetic_heading_deg: f64, accuracy_deg: f64) -> CompassReading {
        let mag = normalize_angle(magnetic_heading_deg);
        let true_heading = normalize_angle(mag + self.declination_deg);

        // Add to history for smoothing
        self.history.insert(0, true_heading);
        if self.history.len() > self.max_history {
            self.history.truncate(self.max_history);
        }

        self.smoothed_heading = circular_mean(&self.history);
        self.calibrated = true;

        let reading = CompassReading {
            magnetic_heading_deg: mag,
            true_heading_deg: self.smoothed_heading,
            declination_deg: self.declination_deg,
            accuracy_deg,
            source: HeadingSource::Magnetometer,
            timestamp: Utc::now(),
        };
        self.last_reading = Some(reading);
        reading
    }

    /// Fuse magnetic heading with GNSS course-over-ground.
    pub fn fuse_with_gnss(
        &mut self,
        magnetic_heading_deg: f64,
        gnss_cog_deg: f64,
        gnss_weight: f64,
    ) -> CompassReading {
        let mag = normalize_angle(magnetic_heading_deg);
        let true_mag = normalize_angle(mag + self.declination_deg);
        let gnss = normalize_angle(gnss_cog_deg);

        // Weighted circular average
        let weight = gnss_weight.clamp(0.0, 1.0);
        let fused = weighted_circular_mean(true_mag, gnss, weight);

        self.history.insert(0, fused);
        if self.history.len() > self.max_history {
            self.history.truncate(self.max_history);
        }
        self.smoothed_heading = circular_mean(&self.history);
        self.calibrated = true;

        let reading = CompassReading {
            magnetic_heading_deg: mag,
            true_heading_deg: self.smoothed_heading,
            declination_deg: self.declination_deg,
            accuracy_deg: 5.0, // Fused accuracy estimate
            source: HeadingSource::Fused,
            timestamp: Utc::now(),
        };
        self.last_reading = Some(reading);
        reading
    }

    /// Get the current smoothed true heading.
    pub fn heading(&self) -> f64 {
        self.smoothed_heading
    }

    /// Whether the compass is calibrated (has received at least one reading).
    pub fn is_calibrated(&self) -> bool {
        self.calibrated
    }

    /// Get the last reading.
    pub fn last_reading(&self) -> Option<&CompassReading> {
        self.last_reading.as_ref()
    }

    /// Cardinal direction string for the current heading.
    pub fn cardinal_direction(&self) -> &'static str {
        heading_to_cardinal(self.smoothed_heading)
    }
}

/// Normalize an angle to [0, 360).
pub fn normalize_angle(deg: f64) -> f64 {
    let mut result = deg % 360.0;
    if result < 0.0 {
        result += 360.0;
    }
    result
}

/// Compute circular mean of a set of angles in degrees.
fn circular_mean(angles: &[f64]) -> f64 {
    if angles.is_empty() {
        return 0.0;
    }
    let sum_sin: f64 = angles.iter().map(|a| a.to_radians().sin()).sum();
    let sum_cos: f64 = angles.iter().map(|a| a.to_radians().cos()).sum();
    normalize_angle(sum_sin.atan2(sum_cos).to_degrees())
}

/// Weighted circular average of two angles.
fn weighted_circular_mean(a: f64, b: f64, weight_b: f64) -> f64 {
    let weight_a = 1.0 - weight_b;
    let sin_sum = weight_a * a.to_radians().sin() + weight_b * b.to_radians().sin();
    let cos_sum = weight_a * a.to_radians().cos() + weight_b * b.to_radians().cos();
    normalize_angle(sin_sum.atan2(cos_sum).to_degrees())
}

/// Convert heading to cardinal direction string.
pub fn heading_to_cardinal(heading: f64) -> &'static str {
    let h = normalize_angle(heading);
    match h {
        h if !(22.5..337.5).contains(&h) => "N",
        h if h < 67.5 => "NE",
        h if h < 112.5 => "E",
        h if h < 157.5 => "SE",
        h if h < 202.5 => "S",
        h if h < 247.5 => "SW",
        h if h < 292.5 => "W",
        _ => "NW",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_angle() {
        assert!((normalize_angle(0.0)).abs() < 1e-10);
        assert!((normalize_angle(360.0)).abs() < 1e-10);
        assert!((normalize_angle(-90.0) - 270.0).abs() < 1e-10);
        assert!((normalize_angle(450.0) - 90.0).abs() < 1e-10);
    }

    #[test]
    fn test_compass_update() {
        let mut compass = Compass::new(3.5); // 3.5° east declination
        let reading = compass.update(90.0, 5.0);
        assert!((reading.magnetic_heading_deg - 90.0).abs() < 1e-10);
        assert!((reading.true_heading_deg - 93.5).abs() < 1.0);
        assert!(compass.is_calibrated());
    }

    #[test]
    fn test_compass_smoothing() {
        let mut compass = Compass::new(0.0);
        compass.update(10.0, 5.0);
        compass.update(20.0, 5.0);
        compass.update(15.0, 5.0);
        // Smoothed heading should be between 10 and 20
        let heading = compass.heading();
        assert!(heading > 5.0 && heading < 25.0);
    }

    #[test]
    fn test_compass_fuse_gnss() {
        let mut compass = Compass::new(0.0);
        let reading = compass.fuse_with_gnss(90.0, 100.0, 0.5);
        // Fused heading should be between 90 and 100
        assert!(reading.true_heading_deg > 85.0 && reading.true_heading_deg < 105.0);
        assert_eq!(reading.source, HeadingSource::Fused);
    }

    #[test]
    fn test_cardinal_direction() {
        assert_eq!(heading_to_cardinal(0.0), "N");
        assert_eq!(heading_to_cardinal(45.0), "NE");
        assert_eq!(heading_to_cardinal(90.0), "E");
        assert_eq!(heading_to_cardinal(135.0), "SE");
        assert_eq!(heading_to_cardinal(180.0), "S");
        assert_eq!(heading_to_cardinal(225.0), "SW");
        assert_eq!(heading_to_cardinal(270.0), "W");
        assert_eq!(heading_to_cardinal(315.0), "NW");
    }

    #[test]
    fn test_circular_mean_opposite_angles() {
        // Mean of 350° and 10° should be near 0°/360°
        let mean = circular_mean(&[350.0, 10.0]);
        assert!(!(10.0..=350.0).contains(&mean));
    }
}
