//! Integrity engine — orchestrates all integrity checks.

use gane_core::gnss::{SatelliteMeasurement, ThreatAlert};
use gane_core::types::{IntegrityLevel, NavigationSource};

use crate::detector::{AnomalyDetector, DetectionThresholds};
use crate::trust::TrustManager;

/// Result of an integrity check cycle.
#[derive(Debug, Clone)]
pub struct IntegrityReport {
    pub level: IntegrityLevel,
    pub threats: Vec<ThreatAlert>,
    pub excluded_sources: Vec<NavigationSource>,
    pub reason: Option<String>,
}

/// High-level integrity engine that coordinates all checks per Section 12.
///
/// Rules enforced:
/// - Source without valid timestamp is rejected.
/// - Source without uncertainty is rejected from fusion.
/// - Source with low trust gets low weight or is excluded.
/// - Source contradicting 3+ other sources cannot serve as anchor.
/// - No fused output without valid covariance.
/// - No auto-recovery without re-validation.
pub struct IntegrityEngine {
    detector: AnomalyDetector,
    trust_manager: TrustManager,
    current_level: IntegrityLevel,
    last_threats: Vec<ThreatAlert>,
}

impl IntegrityEngine {
    pub fn new() -> Self {
        Self {
            detector: AnomalyDetector::new(DetectionThresholds::default()),
            trust_manager: TrustManager::new(),
            current_level: IntegrityLevel::NoSolution,
            last_threats: Vec::new(),
        }
    }

    /// Run a full integrity check cycle on new GNSS measurements.
    pub fn check_measurements(
        &mut self,
        measurements: &[SatelliteMeasurement],
        residuals: &[f64],
    ) -> IntegrityReport {
        let mut threats = Vec::new();

        // 1. Check for jamming.
        if let Some(alert) = self.detector.detect_jamming(measurements) {
            threats.push(alert);
        }

        // 2. Check for spoofing.
        if let Some(alert) = self.detector.detect_spoofing(measurements) {
            threats.push(alert);
        }

        // 3. Check residuals for outliers.
        let sats: Vec<_> = measurements.iter().map(|m| m.satellite).collect();
        let outlier_sats = self.detector.check_residuals(residuals, &sats);

        // 4. Update trust for outlier sources.
        for meas in measurements {
            let source = meas.signal.to_nav_source();
            if outlier_sats.contains(&meas.satellite) {
                self.trust_manager
                    .report_anomaly(source, "outlier residual");
            } else {
                self.trust_manager.report_clean(source);
            }
        }

        // 5. Check multipath on individual satellites.
        for meas in measurements {
            if let Some(_threat_type) = self.detector.detect_multipath(meas) {
                let source = meas.signal.to_nav_source();
                self.trust_manager
                    .report_anomaly(source, "multipath suspected");
            }
        }

        // 6. Determine overall integrity level.
        let level = self.determine_level(&threats, measurements.len());

        // 7. Collect excluded sources.
        let excluded: Vec<NavigationSource> = self
            .trust_manager
            .excluded_sources()
            .keys()
            .copied()
            .collect();

        self.current_level = level;
        self.last_threats = threats.clone();

        let reason = if !threats.is_empty() {
            Some(format!("{} threat(s) detected", threats.len()))
        } else if !excluded.is_empty() {
            Some(format!("{} source(s) excluded", excluded.len()))
        } else {
            None
        };

        IntegrityReport {
            level,
            threats,
            excluded_sources: excluded,
            reason,
        }
    }

    /// Get the trust weight for a navigation source (for use in fusion).
    pub fn source_weight(&mut self, source: NavigationSource) -> f64 {
        if self.trust_manager.is_excluded(&source) {
            0.0
        } else {
            self.trust_manager.trust_score(source)
        }
    }

    /// Current integrity level.
    pub fn current_level(&self) -> IntegrityLevel {
        self.current_level
    }

    /// Access the trust manager.
    pub fn trust_manager(&self) -> &TrustManager {
        &self.trust_manager
    }

    /// Mutable access to the trust manager.
    pub fn trust_manager_mut(&mut self) -> &mut TrustManager {
        &mut self.trust_manager
    }

    fn determine_level(&self, threats: &[ThreatAlert], sat_count: usize) -> IntegrityLevel {
        use gane_core::gnss::ThreatSeverity;

        let has_critical = threats
            .iter()
            .any(|t| t.severity == ThreatSeverity::Critical);
        let has_confirmed = threats
            .iter()
            .any(|t| t.severity == ThreatSeverity::Confirmed);
        let has_suspected = threats
            .iter()
            .any(|t| t.severity == ThreatSeverity::Suspected);

        if has_critical || sat_count == 0 {
            IntegrityLevel::Alert
        } else if has_confirmed {
            IntegrityLevel::Warning
        } else if has_suspected {
            IntegrityLevel::Caution
        } else {
            IntegrityLevel::Nominal
        }
    }
}

impl Default for IntegrityEngine {
    fn default() -> Self {
        Self::new()
    }
}
