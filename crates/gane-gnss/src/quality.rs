//! Per-satellite and per-signal quality scoring.

use gane_core::gnss::{SatelliteId, SatelliteMeasurement};
use serde::{Deserialize, Serialize};

/// Quality assessment for a single satellite measurement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SatelliteQuality {
    pub satellite: SatelliteId,
    /// Overall quality score [0.0, 1.0].
    pub score: f64,
    /// CN0 contribution [0.0, 1.0].
    pub cn0_score: f64,
    /// Elevation contribution [0.0, 1.0].
    pub elevation_score: f64,
    /// Multipath suspicion [0.0, 1.0] where 1.0 = no suspicion.
    pub multipath_score: f64,
    /// Whether this satellite should be used in PVT.
    pub usable: bool,
}

/// Evaluates the quality of individual satellite measurements.
pub struct QualityScorer {
    /// CN0 value considered "excellent" (dB-Hz).
    excellent_cn0: f64,
    /// CN0 value considered "minimum acceptable" (dB-Hz).
    minimum_cn0: f64,
    /// Minimum quality score to be usable.
    usability_threshold: f64,
}

impl QualityScorer {
    pub fn new() -> Self {
        Self {
            excellent_cn0: 45.0,
            minimum_cn0: 20.0,
            usability_threshold: 0.3,
        }
    }

    /// Score a single satellite measurement.
    pub fn score(&self, meas: &SatelliteMeasurement) -> SatelliteQuality {
        let cn0_score = self.score_cn0(meas.cn0_dbhz);
        let elevation_score = self.score_elevation(meas.elevation_deg);
        let multipath_score = self.estimate_multipath_quality(meas);

        // Weighted combination.
        let score = 0.4 * cn0_score + 0.3 * elevation_score + 0.3 * multipath_score;
        let usable = score >= self.usability_threshold && meas.healthy;

        SatelliteQuality {
            satellite: meas.satellite,
            score,
            cn0_score,
            elevation_score,
            multipath_score,
            usable,
        }
    }

    /// Score a batch of measurements.
    pub fn score_batch(&self, measurements: &[SatelliteMeasurement]) -> Vec<SatelliteQuality> {
        measurements.iter().map(|m| self.score(m)).collect()
    }

    fn score_cn0(&self, cn0: f64) -> f64 {
        if cn0 >= self.excellent_cn0 {
            1.0
        } else if cn0 <= self.minimum_cn0 {
            0.0
        } else {
            (cn0 - self.minimum_cn0) / (self.excellent_cn0 - self.minimum_cn0)
        }
    }

    fn score_elevation(&self, elevation: Option<f64>) -> f64 {
        match elevation {
            Some(e) if e >= 60.0 => 1.0,
            Some(e) if e >= 15.0 => (e - 15.0) / 45.0,
            Some(e) if e >= 5.0 => 0.1,
            Some(_) => 0.0,
            None => 0.5, // Unknown elevation — neutral score.
        }
    }

    fn estimate_multipath_quality(&self, meas: &SatelliteMeasurement) -> f64 {
        // Heuristic: low elevation + moderate CN0 suggests multipath.
        // Carrier phase vs pseudorange divergence would be more accurate
        // but requires history. For now, use elevation as a proxy.
        match meas.elevation_deg {
            Some(e) if e < 15.0 => 0.3,
            Some(e) if e < 30.0 => 0.6,
            Some(_) => 0.9,
            None => 0.5,
        }
    }
}

impl Default for QualityScorer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use gane_core::gnss::*;
    use gane_core::types::EntityId;

    fn make_meas(cn0: f64, elevation: Option<f64>) -> SatelliteMeasurement {
        SatelliteMeasurement {
            id: EntityId::new(),
            satellite: SatelliteId {
                constellation: Constellation::Gps,
                prn: 1,
            },
            signal: SignalType::GpsL1CA,
            timestamp: Utc::now(),
            pseudorange_m: 20_000_000.0,
            carrier_phase_cycles: None,
            doppler_hz: None,
            cn0_dbhz: cn0,
            healthy: true,
            satellite_position: None,
            elevation_deg: elevation,
            azimuth_deg: None,
        }
    }

    #[test]
    fn high_quality_satellite_scores_high() {
        let scorer = QualityScorer::new();
        let q = scorer.score(&make_meas(45.0, Some(70.0)));
        assert!(q.score > 0.8);
        assert!(q.usable);
    }

    #[test]
    fn low_cn0_satellite_scores_low() {
        let scorer = QualityScorer::new();
        let q = scorer.score(&make_meas(18.0, Some(10.0)));
        assert!(q.score < 0.2);
        assert!(!q.usable);
    }
}
