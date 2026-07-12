//! Anomaly detector — identifies suspicious patterns in navigation data and reports.

use chrono::{DateTime, Utc};
use gane_core::types::{EntityId, GeoPosition};
use std::collections::HashMap;
use tracing::{debug, warn};

/// An observation fed to the anomaly detector.
#[derive(Debug, Clone)]
pub struct Observation {
    pub source_id: EntityId,
    pub position: GeoPosition,
    pub speed_mps: f64,
    pub heading_deg: f64,
    pub timestamp: DateTime<Utc>,
}

/// Detected anomaly with classification and severity.
#[derive(Debug, Clone)]
pub struct Anomaly {
    pub id: EntityId,
    pub source_id: EntityId,
    pub anomaly_type: AnomalyType,
    /// Severity [0, 1] where 1 = most severe.
    pub severity: f64,
    /// Confidence that this is a real anomaly [0, 1].
    pub confidence: f64,
    pub detected_at: DateTime<Utc>,
    pub description: String,
}

/// Types of anomalies the detector can identify.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnomalyType {
    /// Position jumped impossibly far between observations.
    PositionTeleport,
    /// Speed exceeds physically plausible limits.
    ImpossibleSpeed,
    /// Heading changed impossibly fast.
    HeadingWhiplash,
    /// Data timestamp is in the future or too far in the past.
    TimestampAnomaly,
    /// Position is in an impossible location (ocean, restricted area).
    ImpossibleLocation,
    /// Identical data repeated from multiple sources (replay attack).
    DataReplay,
    /// Statistical outlier in a stream of observations.
    StatisticalOutlier,
}

/// Configuration for anomaly detection thresholds.
#[derive(Debug, Clone)]
pub struct AnomalyConfig {
    /// Maximum plausible speed in m/s (default: ~300 km/h).
    pub max_speed_mps: f64,
    /// Maximum plausible position jump per second in metres.
    pub max_jump_mps: f64,
    /// Maximum heading change rate in degrees per second.
    pub max_heading_rate_dps: f64,
    /// Maximum allowed timestamp drift from now in seconds.
    pub max_timestamp_drift_s: i64,
    /// Minimum number of observations before statistical analysis kicks in.
    pub min_observations_for_stats: usize,
    /// Z-score threshold for statistical outlier detection.
    pub z_score_threshold: f64,
}

impl Default for AnomalyConfig {
    fn default() -> Self {
        Self {
            max_speed_mps: 83.3,        // ~300 km/h
            max_jump_mps: 100.0,        // 100 m/s teleport threshold
            max_heading_rate_dps: 90.0, // 90 deg/s
            max_timestamp_drift_s: 60,
            min_observations_for_stats: 10,
            z_score_threshold: 3.0,
        }
    }
}

/// Per-source statistics for anomaly detection.
#[derive(Debug, Clone)]
struct SourceStats {
    last_observation: Option<Observation>,
    speed_history: Vec<f64>,
    speed_sum: f64,
    speed_sum_sq: f64,
    observation_count: usize,
}

impl SourceStats {
    fn new() -> Self {
        Self {
            last_observation: None,
            speed_history: Vec::new(),
            speed_sum: 0.0,
            speed_sum_sq: 0.0,
            observation_count: 0,
        }
    }

    fn record_speed(&mut self, speed: f64) {
        self.speed_history.push(speed);
        self.speed_sum += speed;
        self.speed_sum_sq += speed * speed;
        self.observation_count += 1;

        // Keep only last 100 observations for memory efficiency.
        if self.speed_history.len() > 100 {
            let old = self.speed_history.remove(0);
            self.speed_sum -= old;
            self.speed_sum_sq -= old * old;
            self.observation_count -= 1;
        }
    }

    fn mean_speed(&self) -> f64 {
        if self.observation_count == 0 {
            return 0.0;
        }
        self.speed_sum / self.observation_count as f64
    }

    fn std_dev_speed(&self) -> f64 {
        if self.observation_count < 2 {
            return 0.0;
        }
        let n = self.observation_count as f64;
        let variance = (self.speed_sum_sq / n) - (self.speed_sum / n).powi(2);
        variance.max(0.0).sqrt()
    }
}

/// Detects anomalies in navigation data streams.
pub struct AnomalyDetector {
    config: AnomalyConfig,
    source_stats: HashMap<EntityId, SourceStats>,
    /// Recent data hashes for replay detection.
    recent_hashes: Vec<u64>,
}

impl AnomalyDetector {
    pub fn new() -> Self {
        Self {
            config: AnomalyConfig::default(),
            source_stats: HashMap::new(),
            recent_hashes: Vec::new(),
        }
    }

    pub fn with_config(config: AnomalyConfig) -> Self {
        Self {
            config,
            source_stats: HashMap::new(),
            recent_hashes: Vec::new(),
        }
    }

    /// Process an observation and return any detected anomalies.
    pub fn process(&mut self, obs: &Observation) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();

        // 1. Check timestamp.
        if let Some(a) = self.check_timestamp(obs) {
            anomalies.push(a);
        }

        // 2. Check impossible speed.
        if let Some(a) = self.check_speed(obs) {
            anomalies.push(a);
        }

        // 3. Check position teleport.
        if let Some(a) = self.check_teleport(obs) {
            anomalies.push(a);
        }

        // 4. Check heading whiplash.
        if let Some(a) = self.check_heading(obs) {
            anomalies.push(a);
        }

        // 5. Check statistical outlier.
        if let Some(a) = self.check_statistical_outlier(obs) {
            anomalies.push(a);
        }

        // 6. Check data replay.
        if let Some(a) = self.check_replay(obs) {
            anomalies.push(a);
        }

        // Update stats.
        let stats = self
            .source_stats
            .entry(obs.source_id)
            .or_insert_with(SourceStats::new);
        stats.record_speed(obs.speed_mps);
        stats.last_observation = Some(obs.clone());

        if !anomalies.is_empty() {
            warn!(
                source_id = %obs.source_id,
                anomaly_count = anomalies.len(),
                "anomalies detected"
            );
        }

        anomalies
    }

    fn check_timestamp(&self, obs: &Observation) -> Option<Anomaly> {
        let now = Utc::now();
        let drift = (now - obs.timestamp).num_seconds().unsigned_abs() as i64;

        if drift > self.config.max_timestamp_drift_s {
            Some(Anomaly {
                id: EntityId::new(),
                source_id: obs.source_id,
                anomaly_type: AnomalyType::TimestampAnomaly,
                severity: (drift as f64 / 3600.0).min(1.0),
                confidence: 0.9,
                detected_at: now,
                description: format!("Timestamp drift: {}s", drift),
            })
        } else {
            None
        }
    }

    fn check_speed(&self, obs: &Observation) -> Option<Anomaly> {
        if obs.speed_mps > self.config.max_speed_mps {
            Some(Anomaly {
                id: EntityId::new(),
                source_id: obs.source_id,
                anomaly_type: AnomalyType::ImpossibleSpeed,
                severity: (obs.speed_mps / self.config.max_speed_mps - 1.0).min(1.0),
                confidence: 0.95,
                detected_at: Utc::now(),
                description: format!(
                    "Speed {:.1} m/s exceeds limit {:.1} m/s",
                    obs.speed_mps, self.config.max_speed_mps
                ),
            })
        } else {
            None
        }
    }

    fn check_teleport(&self, obs: &Observation) -> Option<Anomaly> {
        let stats = self.source_stats.get(&obs.source_id)?;
        let prev = stats.last_observation.as_ref()?;

        let dt = (obs.timestamp - prev.timestamp).num_milliseconds().max(1) as f64 / 1000.0;
        let dist = haversine_m(&prev.position, &obs.position);
        let implied_speed = dist / dt;

        if implied_speed > self.config.max_jump_mps {
            Some(Anomaly {
                id: EntityId::new(),
                source_id: obs.source_id,
                anomaly_type: AnomalyType::PositionTeleport,
                severity: (implied_speed / self.config.max_jump_mps - 1.0).min(1.0),
                confidence: 0.85,
                detected_at: Utc::now(),
                description: format!(
                    "Position jumped {:.0}m in {:.1}s (implied {:.1} m/s)",
                    dist, dt, implied_speed
                ),
            })
        } else {
            None
        }
    }

    fn check_heading(&self, obs: &Observation) -> Option<Anomaly> {
        let stats = self.source_stats.get(&obs.source_id)?;
        let prev = stats.last_observation.as_ref()?;

        let dt = (obs.timestamp - prev.timestamp).num_milliseconds().max(1) as f64 / 1000.0;
        let mut d_heading = (obs.heading_deg - prev.heading_deg).abs();
        if d_heading > 180.0 {
            d_heading = 360.0 - d_heading;
        }
        let heading_rate = d_heading / dt;

        if heading_rate > self.config.max_heading_rate_dps {
            Some(Anomaly {
                id: EntityId::new(),
                source_id: obs.source_id,
                anomaly_type: AnomalyType::HeadingWhiplash,
                severity: (heading_rate / self.config.max_heading_rate_dps - 1.0).min(1.0),
                confidence: 0.7,
                detected_at: Utc::now(),
                description: format!(
                    "Heading changed {:.0} deg in {:.2}s ({:.0} deg/s)",
                    d_heading, dt, heading_rate
                ),
            })
        } else {
            None
        }
    }

    fn check_statistical_outlier(&self, obs: &Observation) -> Option<Anomaly> {
        let stats = self.source_stats.get(&obs.source_id)?;

        if stats.observation_count < self.config.min_observations_for_stats {
            return None;
        }

        let mean = stats.mean_speed();
        let std_dev = stats.std_dev_speed();

        if std_dev < 0.001 {
            return None; // can't compute z-score with zero variance
        }

        let z_score = (obs.speed_mps - mean).abs() / std_dev;

        if z_score > self.config.z_score_threshold {
            Some(Anomaly {
                id: EntityId::new(),
                source_id: obs.source_id,
                anomaly_type: AnomalyType::StatisticalOutlier,
                severity: ((z_score - self.config.z_score_threshold) / 3.0).min(1.0),
                confidence: 0.6,
                detected_at: Utc::now(),
                description: format!(
                    "Speed z-score: {:.2} (threshold: {:.1})",
                    z_score, self.config.z_score_threshold
                ),
            })
        } else {
            None
        }
    }

    fn check_replay(&mut self, obs: &Observation) -> Option<Anomaly> {
        // Simple hash-based replay detection.
        let hash = compute_observation_hash(obs);

        if self.recent_hashes.contains(&hash) {
            debug!(source_id = %obs.source_id, "potential data replay detected");
            return Some(Anomaly {
                id: EntityId::new(),
                source_id: obs.source_id,
                anomaly_type: AnomalyType::DataReplay,
                severity: 0.8,
                confidence: 0.75,
                detected_at: Utc::now(),
                description: "Duplicate observation hash detected".into(),
            });
        }

        self.recent_hashes.push(hash);
        // Keep only last 1000 hashes.
        if self.recent_hashes.len() > 1000 {
            self.recent_hashes.remove(0);
        }

        None
    }

    /// Get the number of tracked sources.
    pub fn tracked_sources(&self) -> usize {
        self.source_stats.len()
    }
}

impl Default for AnomalyDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple hash for replay detection (not cryptographic).
fn compute_observation_hash(obs: &Observation) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    let bytes_lat = obs.position.latitude_deg.to_bits();
    let bytes_lon = obs.position.longitude_deg.to_bits();
    let bytes_speed = obs.speed_mps.to_bits();
    let bytes_heading = obs.heading_deg.to_bits();

    // Include source_id to avoid cross-source false positives.
    for b in obs
        .source_id
        .0
        .as_bytes()
        .iter()
        .chain(bytes_lat.to_le_bytes().iter())
        .chain(bytes_lon.to_le_bytes().iter())
        .chain(bytes_speed.to_le_bytes().iter())
        .chain(bytes_heading.to_le_bytes().iter())
    {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
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

    fn obs(source: EntityId, lat: f64, lon: f64, speed: f64, heading: f64) -> Observation {
        Observation {
            source_id: source,
            position: GeoPosition {
                latitude_deg: lat,
                longitude_deg: lon,
                altitude_m: None,
            },
            speed_mps: speed,
            heading_deg: heading,
            timestamp: Utc::now(),
        }
    }

    #[test]
    fn normal_observation_no_anomalies() {
        let mut detector = AnomalyDetector::new();
        let source = EntityId::new();
        let anomalies = detector.process(&obs(source, 32.08, 34.78, 20.0, 90.0));
        assert!(anomalies.is_empty());
    }

    #[test]
    fn impossible_speed_detected() {
        let mut detector = AnomalyDetector::new();
        let source = EntityId::new();
        let anomalies = detector.process(&obs(source, 32.08, 34.78, 200.0, 90.0)); // 720 km/h
        assert!(anomalies
            .iter()
            .any(|a| a.anomaly_type == AnomalyType::ImpossibleSpeed));
    }

    #[test]
    fn data_replay_detected() {
        let mut detector = AnomalyDetector::new();
        let source = EntityId::new();

        // First observation is fine.
        let o = obs(source, 32.08, 34.78, 20.0, 90.0);
        detector.process(&o);

        // Exact same observation again — replay.
        let anomalies = detector.process(&o);
        assert!(anomalies
            .iter()
            .any(|a| a.anomaly_type == AnomalyType::DataReplay));
    }

    #[test]
    fn timestamp_anomaly_detected() {
        let mut detector = AnomalyDetector::new();
        let source = EntityId::new();

        let mut o = obs(source, 32.08, 34.78, 20.0, 90.0);
        o.timestamp = Utc::now() - chrono::Duration::hours(2); // 2 hours old

        let anomalies = detector.process(&o);
        assert!(anomalies
            .iter()
            .any(|a| a.anomaly_type == AnomalyType::TimestampAnomaly));
    }

    #[test]
    fn tracked_sources_increments() {
        let mut detector = AnomalyDetector::new();
        assert_eq!(detector.tracked_sources(), 0);

        detector.process(&obs(EntityId::new(), 32.08, 34.78, 20.0, 90.0));
        assert_eq!(detector.tracked_sources(), 1);

        detector.process(&obs(EntityId::new(), 32.08, 34.78, 20.0, 90.0));
        assert_eq!(detector.tracked_sources(), 2);
    }
}
