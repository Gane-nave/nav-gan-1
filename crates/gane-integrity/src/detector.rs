//! Fault and anomaly detectors — spoofing, jamming, multipath, NLOS.

use chrono::Utc;
use gane_core::gnss::{SatelliteId, SatelliteMeasurement, ThreatAlert, ThreatSeverity, ThreatType};
use gane_core::types::EntityId;
use std::collections::HashMap;
use tracing::warn;

/// Thresholds for anomaly detection.
pub struct DetectionThresholds {
    /// CN0 drop that triggers multipath suspicion (dB-Hz).
    pub cn0_drop_threshold: f64,
    /// Maximum pseudorange residual before flagging (m).
    pub max_residual_m: f64,
    /// CN0 below which jamming is suspected (dB-Hz).
    pub jamming_cn0_threshold: f64,
    /// Minimum satellites that must agree for cross-check.
    pub cross_check_min_sources: usize,
    /// Maximum position jump between epochs (m).
    pub max_position_jump_m: f64,
    /// Maximum clock jump between epochs (ns).
    pub max_clock_jump_ns: f64,
}

impl Default for DetectionThresholds {
    fn default() -> Self {
        Self {
            cn0_drop_threshold: 10.0,
            max_residual_m: 50.0,
            jamming_cn0_threshold: 15.0,
            cross_check_min_sources: 3,
            max_position_jump_m: 100.0,
            max_clock_jump_ns: 1000.0,
        }
    }
}

/// Detects GNSS anomalies: spoofing, jamming, multipath, NLOS.
pub struct AnomalyDetector {
    thresholds: DetectionThresholds,
    /// Historical CN0 per satellite for trend analysis.
    cn0_history: HashMap<SatelliteId, Vec<f64>>,
    /// Maximum history length per satellite.
    max_history: usize,
}

impl AnomalyDetector {
    pub fn new(thresholds: DetectionThresholds) -> Self {
        Self {
            thresholds,
            cn0_history: HashMap::new(),
            max_history: 60,
        }
    }

    /// Analyse measurements for jamming indicators.
    ///
    /// Jamming causes a broad CN0 drop across all satellites/constellations.
    pub fn detect_jamming(&mut self, measurements: &[SatelliteMeasurement]) -> Option<ThreatAlert> {
        if measurements.is_empty() {
            return None;
        }

        let avg_cn0: f64 =
            measurements.iter().map(|m| m.cn0_dbhz).sum::<f64>() / measurements.len() as f64;

        if avg_cn0 < self.thresholds.jamming_cn0_threshold {
            warn!(avg_cn0, "jamming suspected — average CN0 critically low");
            return Some(ThreatAlert {
                id: EntityId::new(),
                timestamp: Utc::now(),
                threat_type: ThreatType::Jamming,
                severity: if avg_cn0 < 10.0 {
                    ThreatSeverity::Critical
                } else {
                    ThreatSeverity::Suspected
                },
                affected_constellation: None,
                affected_satellites: measurements.iter().map(|m| m.satellite).collect(),
                reason: format!("average CN0 = {avg_cn0:.1} dB-Hz"),
                mitigated: false,
            });
        }

        None
    }

    /// Analyse measurements for spoofing indicators.
    ///
    /// Spoofing indicators: sudden CN0 increase, all satellites at similar CN0,
    /// abnormal Doppler consistency.
    pub fn detect_spoofing(
        &mut self,
        measurements: &[SatelliteMeasurement],
    ) -> Option<ThreatAlert> {
        if measurements.len() < 4 {
            return None;
        }

        // Check CN0 variance — spoofed signals often have very similar CN0.
        let cn0_values: Vec<f64> = measurements.iter().map(|m| m.cn0_dbhz).collect();
        let mean = cn0_values.iter().sum::<f64>() / cn0_values.len() as f64;
        let variance =
            cn0_values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / cn0_values.len() as f64;

        // Very low CN0 variance across many satellites is suspicious.
        if variance < 1.0 && measurements.len() >= 6 {
            warn!(
                variance,
                sats = measurements.len(),
                "spoofing suspected — abnormally uniform CN0"
            );
            return Some(ThreatAlert {
                id: EntityId::new(),
                timestamp: Utc::now(),
                threat_type: ThreatType::Spoofing,
                severity: ThreatSeverity::Suspected,
                affected_constellation: None,
                affected_satellites: measurements.iter().map(|m| m.satellite).collect(),
                reason: format!(
                    "CN0 variance = {variance:.2} across {} satellites",
                    measurements.len()
                ),
                mitigated: false,
            });
        }

        // Check for sudden CN0 jumps in history.
        for meas in measurements {
            let history = self.cn0_history.entry(meas.satellite).or_default();

            if let Some(&prev_cn0) = history.last() {
                let jump = meas.cn0_dbhz - prev_cn0;
                if jump > self.thresholds.cn0_drop_threshold {
                    warn!(
                        sat = %meas.satellite,
                        jump,
                        "spoofing suspected — sudden CN0 increase"
                    );
                    return Some(ThreatAlert {
                        id: EntityId::new(),
                        timestamp: Utc::now(),
                        threat_type: ThreatType::Spoofing,
                        severity: ThreatSeverity::Suspected,
                        affected_constellation: Some(meas.satellite.constellation),
                        affected_satellites: vec![meas.satellite],
                        reason: format!("CN0 jump of {jump:.1} dB-Hz on {}", meas.satellite),
                        mitigated: false,
                    });
                }
            }

            history.push(meas.cn0_dbhz);
            if history.len() > self.max_history {
                history.remove(0);
            }
        }

        None
    }

    /// Check PVT residuals for outlier satellites.
    pub fn check_residuals(
        &self,
        residuals: &[f64],
        satellites: &[SatelliteId],
    ) -> Vec<SatelliteId> {
        let mut outliers = Vec::new();
        for (i, &residual) in residuals.iter().enumerate() {
            if residual.abs() > self.thresholds.max_residual_m {
                if let Some(sat) = satellites.get(i) {
                    warn!(
                        sat = %sat,
                        residual,
                        "outlier residual — satellite may have multipath/NLOS"
                    );
                    outliers.push(*sat);
                }
            }
        }
        outliers
    }

    /// Detect multipath/NLOS on individual satellites from elevation + CN0 pattern.
    pub fn detect_multipath(&self, meas: &SatelliteMeasurement) -> Option<ThreatType> {
        // Low elevation + moderate CN0 + high pseudorange residual → multipath likely.
        if let Some(elev) = meas.elevation_deg {
            if elev < 20.0 && meas.cn0_dbhz < 30.0 {
                return Some(ThreatType::Multipath);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gane_core::gnss::*;

    fn make_meas(prn: u8, cn0: f64) -> SatelliteMeasurement {
        SatelliteMeasurement {
            id: EntityId::new(),
            satellite: SatelliteId {
                constellation: Constellation::Gps,
                prn,
            },
            signal: SignalType::GpsL1CA,
            timestamp: Utc::now(),
            pseudorange_m: 20_000_000.0,
            carrier_phase_cycles: None,
            doppler_hz: None,
            cn0_dbhz: cn0,
            healthy: true,
            satellite_position: None,
            elevation_deg: Some(45.0),
            azimuth_deg: None,
        }
    }

    #[test]
    fn jamming_detected_on_low_cn0() {
        let mut detector = AnomalyDetector::new(DetectionThresholds::default());
        let measurements: Vec<SatelliteMeasurement> =
            (1..=8).map(|prn| make_meas(prn, 12.0)).collect();

        let alert = detector.detect_jamming(&measurements);
        assert!(alert.is_some());
        assert_eq!(alert.unwrap().threat_type, ThreatType::Jamming);
    }

    #[test]
    fn no_jamming_on_normal_cn0() {
        let mut detector = AnomalyDetector::new(DetectionThresholds::default());
        let measurements: Vec<SatelliteMeasurement> =
            (1..=8).map(|prn| make_meas(prn, 40.0)).collect();

        assert!(detector.detect_jamming(&measurements).is_none());
    }

    #[test]
    fn spoofing_suspected_on_uniform_cn0() {
        let mut detector = AnomalyDetector::new(DetectionThresholds::default());
        // All satellites at almost identical CN0 → suspicious.
        let measurements: Vec<SatelliteMeasurement> =
            (1..=10).map(|prn| make_meas(prn, 42.0)).collect();

        let alert = detector.detect_spoofing(&measurements);
        assert!(alert.is_some());
        assert_eq!(alert.unwrap().threat_type, ThreatType::Spoofing);
    }
}
