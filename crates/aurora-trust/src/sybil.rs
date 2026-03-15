//! Sybil detector — identifies fake accounts, collusion patterns, and anomalous behavior.

use aurora_core::types::{EntityId, GeoPosition};
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use tracing::{info, warn};

/// A report event for sybil analysis.
#[derive(Debug, Clone)]
pub struct ReportEvent {
    pub reporter_id: EntityId,
    pub position: GeoPosition,
    pub timestamp: DateTime<Utc>,
    pub incident_id: EntityId,
}

/// Result of sybil analysis for a set of reports.
#[derive(Debug, Clone)]
pub struct SybilAnalysis {
    /// Probability that the reports are from coordinated sybil accounts [0, 1].
    pub sybil_probability: f64,
    /// Detected suspicious clusters (groups of reporters acting in concert).
    pub suspicious_clusters: Vec<SuspiciousCluster>,
    /// Individual anomaly scores per reporter.
    pub reporter_anomalies: HashMap<EntityId, f64>,
}

/// A cluster of reporters exhibiting coordinated behavior.
#[derive(Debug, Clone)]
pub struct SuspiciousCluster {
    /// Reporter IDs in this cluster.
    pub members: Vec<EntityId>,
    /// Type of suspicious pattern detected.
    pub pattern: CollusionPattern,
    /// Confidence that this cluster is genuine collusion [0, 1].
    pub confidence: f64,
}

/// Types of collusion patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollusionPattern {
    /// Multiple reports from near-identical positions within a short time.
    SpatioTemporalClustering,
    /// Same incident reported by accounts with correlated report histories.
    CorrelatedReporting,
    /// Burst of reports from new/low-trust accounts.
    NewAccountBurst,
    /// Reports filed at inhuman speed.
    RapidFireReporting,
}

/// Configuration for sybil detection.
#[derive(Debug, Clone)]
pub struct SybilConfig {
    /// Maximum distance (metres) between reports to consider them co-located.
    pub co_location_radius_m: f64,
    /// Maximum time gap (seconds) for temporal clustering.
    pub temporal_window_s: i64,
    /// Minimum cluster size to flag as suspicious.
    pub min_cluster_size: usize,
    /// Minimum reports per minute to flag as rapid-fire.
    pub rapid_fire_threshold: u32,
    /// Account age (days) below which the account is considered "new".
    pub new_account_days: u32,
}

impl Default for SybilConfig {
    fn default() -> Self {
        Self {
            co_location_radius_m: 50.0,
            temporal_window_s: 60,
            min_cluster_size: 3,
            rapid_fire_threshold: 10,
            new_account_days: 7,
        }
    }
}

/// Detects sybil attacks, collusion, and anomalous reporting patterns.
pub struct SybilDetector {
    config: SybilConfig,
    /// Account creation dates for age checks.
    account_ages: HashMap<EntityId, DateTime<Utc>>,
    /// Historical report counts per reporter.
    report_history: HashMap<EntityId, Vec<DateTime<Utc>>>,
}

impl SybilDetector {
    pub fn new() -> Self {
        Self {
            config: SybilConfig::default(),
            account_ages: HashMap::new(),
            report_history: HashMap::new(),
        }
    }

    pub fn with_config(config: SybilConfig) -> Self {
        Self {
            config,
            account_ages: HashMap::new(),
            report_history: HashMap::new(),
        }
    }

    /// Register an account with its creation date.
    pub fn register_account(&mut self, entity_id: EntityId, created_at: DateTime<Utc>) {
        self.account_ages.insert(entity_id, created_at);
    }

    /// Record a report for history tracking.
    pub fn record_report(&mut self, reporter_id: EntityId, timestamp: DateTime<Utc>) {
        self.report_history
            .entry(reporter_id)
            .or_default()
            .push(timestamp);
    }

    /// Analyze a set of reports for sybil/collusion patterns.
    pub fn analyze(&self, reports: &[ReportEvent]) -> SybilAnalysis {
        let mut suspicious_clusters = Vec::new();
        let mut reporter_anomalies: HashMap<EntityId, f64> = HashMap::new();

        // 1. Spatio-temporal clustering detection.
        let st_clusters = self.detect_spatiotemporal_clusters(reports);
        for cluster in &st_clusters {
            for member in &cluster.members {
                let entry = reporter_anomalies.entry(*member).or_insert(0.0);
                *entry = (*entry + cluster.confidence).min(1.0);
            }
        }
        suspicious_clusters.extend(st_clusters);

        // 2. Rapid-fire detection.
        let rf_clusters = self.detect_rapid_fire(reports);
        for cluster in &rf_clusters {
            for member in &cluster.members {
                let entry = reporter_anomalies.entry(*member).or_insert(0.0);
                *entry = (*entry + cluster.confidence * 0.5).min(1.0);
            }
        }
        suspicious_clusters.extend(rf_clusters);

        // 3. New account burst detection.
        let nab_clusters = self.detect_new_account_burst(reports);
        for cluster in &nab_clusters {
            for member in &cluster.members {
                let entry = reporter_anomalies.entry(*member).or_insert(0.0);
                *entry = (*entry + cluster.confidence * 0.3).min(1.0);
            }
        }
        suspicious_clusters.extend(nab_clusters);

        // Compute overall sybil probability.
        let sybil_probability = if suspicious_clusters.is_empty() {
            0.0
        } else {
            let max_confidence = suspicious_clusters
                .iter()
                .map(|c| c.confidence)
                .fold(0.0_f64, f64::max);
            let cluster_ratio = suspicious_clusters.len() as f64 / reports.len().max(1) as f64;
            (max_confidence * 0.7 + cluster_ratio * 0.3).min(1.0)
        };

        if sybil_probability > 0.5 {
            warn!(
                sybil_probability,
                clusters = suspicious_clusters.len(),
                "high sybil probability detected"
            );
        }

        SybilAnalysis {
            sybil_probability,
            suspicious_clusters,
            reporter_anomalies,
        }
    }

    /// Detect reports from near-identical positions within a short time window.
    fn detect_spatiotemporal_clusters(&self, reports: &[ReportEvent]) -> Vec<SuspiciousCluster> {
        let mut clusters: Vec<SuspiciousCluster> = Vec::new();

        // Group reports by incident.
        let mut by_incident: HashMap<EntityId, Vec<&ReportEvent>> = HashMap::new();
        for r in reports {
            by_incident.entry(r.incident_id).or_default().push(r);
        }

        for (_incident_id, incident_reports) in &by_incident {
            if incident_reports.len() < self.config.min_cluster_size {
                continue;
            }

            // Check for spatial and temporal proximity.
            let mut co_located: Vec<EntityId> = Vec::new();

            for i in 0..incident_reports.len() {
                for j in (i + 1)..incident_reports.len() {
                    let dist =
                        haversine_m(&incident_reports[i].position, &incident_reports[j].position);
                    let time_diff = (incident_reports[i].timestamp - incident_reports[j].timestamp)
                        .num_seconds()
                        .unsigned_abs();

                    if dist < self.config.co_location_radius_m
                        && time_diff < self.config.temporal_window_s as u64
                    {
                        co_located.push(incident_reports[i].reporter_id);
                        co_located.push(incident_reports[j].reporter_id);
                    }
                }
            }

            // Deduplicate.
            let unique: HashSet<EntityId> = co_located.into_iter().collect();
            if unique.len() >= self.config.min_cluster_size {
                let members: Vec<EntityId> = unique.into_iter().collect();
                let confidence = (members.len() as f64 / incident_reports.len() as f64).min(1.0);

                clusters.push(SuspiciousCluster {
                    members,
                    pattern: CollusionPattern::SpatioTemporalClustering,
                    confidence,
                });
            }
        }

        clusters
    }

    /// Detect reporters filing reports at inhuman speed.
    fn detect_rapid_fire(&self, reports: &[ReportEvent]) -> Vec<SuspiciousCluster> {
        let mut clusters = Vec::new();

        // Group by reporter.
        let mut by_reporter: HashMap<EntityId, Vec<&ReportEvent>> = HashMap::new();
        for r in reports {
            by_reporter.entry(r.reporter_id).or_default().push(r);
        }

        for (reporter_id, reporter_reports) in &by_reporter {
            if reporter_reports.len() < self.config.rapid_fire_threshold as usize {
                continue;
            }

            // Check if they all fall within 1 minute.
            let mut timestamps: Vec<DateTime<Utc>> =
                reporter_reports.iter().map(|r| r.timestamp).collect();
            timestamps.sort();

            if let (Some(first), Some(last)) = (timestamps.first(), timestamps.last()) {
                let span_s = (*last - *first).num_seconds();
                if span_s <= 60 {
                    clusters.push(SuspiciousCluster {
                        members: vec![*reporter_id],
                        pattern: CollusionPattern::RapidFireReporting,
                        confidence: 0.8,
                    });
                }
            }
        }

        clusters
    }

    /// Detect bursts of reports from new accounts.
    fn detect_new_account_burst(&self, reports: &[ReportEvent]) -> Vec<SuspiciousCluster> {
        let now = Utc::now();
        let threshold_days = self.config.new_account_days as i64;

        let new_reporters: Vec<EntityId> = reports
            .iter()
            .map(|r| r.reporter_id)
            .collect::<HashSet<_>>()
            .into_iter()
            .filter(|id| {
                self.account_ages
                    .get(id)
                    .map(|created| (now - *created).num_days() < threshold_days)
                    .unwrap_or(true) // unknown accounts treated as new
            })
            .collect();

        if new_reporters.len() >= self.config.min_cluster_size {
            info!(
                new_account_count = new_reporters.len(),
                "new account burst detected"
            );
            vec![SuspiciousCluster {
                members: new_reporters,
                pattern: CollusionPattern::NewAccountBurst,
                confidence: 0.5,
            }]
        } else {
            Vec::new()
        }
    }
}

impl Default for SybilDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Haversine distance in metres.
fn haversine_m(a: &GeoPosition, b: &GeoPosition) -> f64 {
    let r = 6_371_000.0;
    let d_lat = (b.latitude_deg - a.latitude_deg).to_radians();
    let d_lon = (b.longitude_deg - a.longitude_deg).to_radians();
    let lat1 = a.latitude_deg.to_radians();
    let lat2 = b.latitude_deg.to_radians();

    let a_val = (d_lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (d_lon / 2.0).sin().powi(2);
    let c = 2.0 * a_val.sqrt().asin();
    r * c
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pos(lat: f64, lon: f64) -> GeoPosition {
        GeoPosition {
            latitude_deg: lat,
            longitude_deg: lon,
            altitude_m: None,
        }
    }

    fn report_event(reporter: EntityId, incident: EntityId, lat: f64, lon: f64) -> ReportEvent {
        ReportEvent {
            reporter_id: reporter,
            position: pos(lat, lon),
            timestamp: Utc::now(),
            incident_id: incident,
        }
    }

    #[test]
    fn no_sybil_for_legitimate_reports() {
        let detector = SybilDetector::new();
        let incident = EntityId::new();

        // Two different reporters, far apart — legitimate.
        let reports = vec![
            report_event(EntityId::new(), incident, 32.08, 34.78),
            report_event(EntityId::new(), incident, 33.00, 35.00),
        ];

        let analysis = detector.analyze(&reports);
        assert!(analysis.sybil_probability < 0.3);
        assert!(analysis.suspicious_clusters.is_empty());
    }

    #[test]
    fn spatiotemporal_cluster_detected() {
        let detector = SybilDetector::with_config(SybilConfig {
            min_cluster_size: 3,
            co_location_radius_m: 100.0,
            temporal_window_s: 120,
            ..Default::default()
        });

        let incident = EntityId::new();

        // 4 reporters at nearly the same location, same time.
        let reports = vec![
            report_event(EntityId::new(), incident, 32.0800, 34.7800),
            report_event(EntityId::new(), incident, 32.0801, 34.7801),
            report_event(EntityId::new(), incident, 32.0800, 34.7802),
            report_event(EntityId::new(), incident, 32.0801, 34.7800),
        ];

        let analysis = detector.analyze(&reports);
        assert!(!analysis.suspicious_clusters.is_empty());

        let cluster = &analysis.suspicious_clusters[0];
        assert_eq!(cluster.pattern, CollusionPattern::SpatioTemporalClustering);
        assert!(cluster.members.len() >= 3);
    }

    #[test]
    fn new_account_burst_detected() {
        let mut detector = SybilDetector::with_config(SybilConfig {
            min_cluster_size: 3,
            new_account_days: 7,
            ..Default::default()
        });

        let incident = EntityId::new();
        let now = Utc::now();

        // Register 4 new accounts (created today).
        let reporters: Vec<EntityId> = (0..4).map(|_| EntityId::new()).collect();
        for r in &reporters {
            detector.register_account(*r, now);
        }

        let reports: Vec<ReportEvent> = reporters
            .iter()
            .enumerate()
            .map(|(i, r)| report_event(*r, incident, 32.08 + i as f64 * 0.1, 34.78))
            .collect();

        let analysis = detector.analyze(&reports);
        let nab = analysis
            .suspicious_clusters
            .iter()
            .find(|c| c.pattern == CollusionPattern::NewAccountBurst);
        assert!(nab.is_some());
    }

    #[test]
    fn old_accounts_not_flagged_as_burst() {
        let mut detector = SybilDetector::with_config(SybilConfig {
            min_cluster_size: 3,
            new_account_days: 7,
            ..Default::default()
        });

        let incident = EntityId::new();
        let old = Utc::now() - chrono::Duration::days(30);

        // Register 4 old accounts.
        let reporters: Vec<EntityId> = (0..4).map(|_| EntityId::new()).collect();
        for r in &reporters {
            detector.register_account(*r, old);
        }

        let reports: Vec<ReportEvent> = reporters
            .iter()
            .enumerate()
            .map(|(i, r)| report_event(*r, incident, 32.08 + i as f64 * 0.1, 34.78))
            .collect();

        let analysis = detector.analyze(&reports);
        let nab = analysis
            .suspicious_clusters
            .iter()
            .find(|c| c.pattern == CollusionPattern::NewAccountBurst);
        assert!(nab.is_none());
    }

    #[test]
    fn anomaly_scores_assigned_to_suspicious_reporters() {
        let detector = SybilDetector::with_config(SybilConfig {
            min_cluster_size: 3,
            co_location_radius_m: 100.0,
            temporal_window_s: 120,
            ..Default::default()
        });

        let incident = EntityId::new();
        let reporters: Vec<EntityId> = (0..4).map(|_| EntityId::new()).collect();

        let reports: Vec<ReportEvent> = reporters
            .iter()
            .map(|r| report_event(*r, incident, 32.0800, 34.7800))
            .collect();

        let analysis = detector.analyze(&reports);
        // At least some reporters should have non-zero anomaly scores.
        let has_anomaly = analysis
            .reporter_anomalies
            .values()
            .any(|score| *score > 0.0);
        assert!(has_anomaly);
    }
}
