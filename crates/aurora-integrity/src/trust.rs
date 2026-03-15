//! Per-source trust scoring and management.

use aurora_core::types::NavigationSource;
use std::collections::HashMap;
use tracing::{info, warn};

/// Initial trust score for a new source.
const INITIAL_TRUST: f64 = 0.8;
/// Trust decay per anomaly event.
const ANOMALY_PENALTY: f64 = 0.15;
/// Trust recovery per clean epoch.
const RECOVERY_INCREMENT: f64 = 0.02;
/// Trust below which a source is excluded.
const EXCLUSION_THRESHOLD: f64 = 0.3;
/// Trust above which a source is re-admitted after exclusion.
const READMISSION_THRESHOLD: f64 = 0.5;

/// Manages per-source trust scores with decay, recovery, and exclusion logic.
pub struct TrustManager {
    scores: HashMap<NavigationSource, f64>,
    excluded: HashMap<NavigationSource, String>,
}

impl TrustManager {
    pub fn new() -> Self {
        Self {
            scores: HashMap::new(),
            excluded: HashMap::new(),
        }
    }

    /// Get the trust score for a source, initialising if needed.
    pub fn trust_score(&mut self, source: NavigationSource) -> f64 {
        *self.scores.entry(source).or_insert(INITIAL_TRUST)
    }

    /// Report a clean (anomaly-free) epoch for a source.
    pub fn report_clean(&mut self, source: NavigationSource) {
        let score = self.scores.entry(source).or_insert(INITIAL_TRUST);
        *score = (*score + RECOVERY_INCREMENT).min(1.0);

        // Check for re-admission.
        if self.excluded.contains_key(&source) && *score >= READMISSION_THRESHOLD {
            info!(source = ?source, score = *score, "source re-admitted after trust recovery");
            self.excluded.remove(&source);
        }
    }

    /// Report an anomaly for a source.
    pub fn report_anomaly(&mut self, source: NavigationSource, reason: &str) {
        let score = self.scores.entry(source).or_insert(INITIAL_TRUST);
        *score = (*score - ANOMALY_PENALTY).max(0.0);
        warn!(source = ?source, score = *score, reason, "trust penalty applied");

        // Check for exclusion.
        if *score < EXCLUSION_THRESHOLD && !self.excluded.contains_key(&source) {
            warn!(source = ?source, "source excluded due to low trust");
            self.excluded.insert(source, reason.to_string());
        }
    }

    /// Whether a source is currently excluded.
    pub fn is_excluded(&self, source: &NavigationSource) -> bool {
        self.excluded.contains_key(source)
    }

    /// Get all excluded sources.
    pub fn excluded_sources(&self) -> &HashMap<NavigationSource, String> {
        &self.excluded
    }

    /// Manually exclude a source.
    pub fn exclude(&mut self, source: NavigationSource, reason: String) {
        self.excluded.insert(source, reason);
    }

    /// Manually reinstate a source.
    pub fn reinstate(&mut self, source: NavigationSource) {
        self.excluded.remove(&source);
        // Set trust to readmission threshold.
        self.scores.insert(source, READMISSION_THRESHOLD);
    }

    /// Get all trust scores.
    pub fn all_scores(&self) -> &HashMap<NavigationSource, f64> {
        &self.scores
    }

    /// Reset all trust scores and exclusions.
    pub fn reset(&mut self) {
        self.scores.clear();
        self.excluded.clear();
    }
}

impl Default for TrustManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_trust_is_set() {
        let mut tm = TrustManager::new();
        let score = tm.trust_score(NavigationSource::GpsL1);
        assert!((score - INITIAL_TRUST).abs() < f64::EPSILON);
    }

    #[test]
    fn anomalies_reduce_trust() {
        let mut tm = TrustManager::new();
        tm.report_anomaly(NavigationSource::GpsL1, "test");
        let score = tm.trust_score(NavigationSource::GpsL1);
        assert!(score < INITIAL_TRUST);
    }

    #[test]
    fn repeated_anomalies_cause_exclusion() {
        let mut tm = TrustManager::new();
        for _ in 0..10 {
            tm.report_anomaly(NavigationSource::GpsL1, "test");
        }
        assert!(tm.is_excluded(&NavigationSource::GpsL1));
    }

    #[test]
    fn clean_epochs_recover_trust() {
        let mut tm = TrustManager::new();
        tm.report_anomaly(NavigationSource::GpsL1, "test");
        let score_after_anomaly = tm.trust_score(NavigationSource::GpsL1);

        for _ in 0..10 {
            tm.report_clean(NavigationSource::GpsL1);
        }
        let score_after_recovery = tm.trust_score(NavigationSource::GpsL1);
        assert!(score_after_recovery > score_after_anomaly);
    }

    #[test]
    fn excluded_source_is_readmitted_after_recovery() {
        let mut tm = TrustManager::new();
        for _ in 0..10 {
            tm.report_anomaly(NavigationSource::GlonassL1, "test");
        }
        assert!(tm.is_excluded(&NavigationSource::GlonassL1));

        for _ in 0..50 {
            tm.report_clean(NavigationSource::GlonassL1);
        }
        assert!(!tm.is_excluded(&NavigationSource::GlonassL1));
    }
}
