//! Reputation engine — user trust scoring with multi-source validation.

use chrono::Utc;
use gane_core::scoring::{TrustEntityType, TrustScore};
use gane_core::types::EntityId;
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// Configuration for the reputation engine.
#[derive(Debug, Clone)]
pub struct ReputationConfig {
    /// Initial trust score for new entities.
    pub initial_trust: f64,
    /// Amount of trust gained per successful validation.
    pub validation_bonus: f64,
    /// Amount of trust lost per false report.
    pub false_report_penalty: f64,
    /// Minimum trust score (floor).
    pub min_trust: f64,
    /// Maximum trust score (ceiling).
    pub max_trust: f64,
    /// Trust decay rate per day of inactivity.
    pub decay_rate_per_day: f64,
    /// Maximum reports per hour before rate limiting.
    pub max_reports_per_hour: u32,
}

impl Default for ReputationConfig {
    fn default() -> Self {
        Self {
            initial_trust: 0.5,
            validation_bonus: 0.02,
            false_report_penalty: 0.10,
            min_trust: 0.0,
            max_trust: 1.0,
            decay_rate_per_day: 0.001,
            max_reports_per_hour: 20,
        }
    }
}

/// Activity record for rate limiting.
#[derive(Debug, Clone)]
struct ActivityRecord {
    /// Timestamps of recent reports.
    report_timestamps: Vec<chrono::DateTime<chrono::Utc>>,
}

impl ActivityRecord {
    fn new() -> Self {
        Self {
            report_timestamps: Vec::new(),
        }
    }

    /// Count reports in the last hour.
    fn reports_last_hour(&self) -> u32 {
        let cutoff = Utc::now() - chrono::Duration::hours(1);
        self.report_timestamps
            .iter()
            .filter(|t| **t > cutoff)
            .count() as u32
    }

    /// Record a new report.
    fn record_report(&mut self) {
        self.report_timestamps.push(Utc::now());
        // Prune old timestamps (older than 2 hours).
        let cutoff = Utc::now() - chrono::Duration::hours(2);
        self.report_timestamps.retain(|t| *t > cutoff);
    }
}

/// Manages trust/reputation scores for entities (users, sources, data).
pub struct ReputationEngine {
    config: ReputationConfig,
    scores: HashMap<EntityId, TrustScore>,
    activity: HashMap<EntityId, ActivityRecord>,
}

impl ReputationEngine {
    pub fn new() -> Self {
        Self {
            config: ReputationConfig::default(),
            scores: HashMap::new(),
            activity: HashMap::new(),
        }
    }

    pub fn with_config(config: ReputationConfig) -> Self {
        Self {
            config,
            scores: HashMap::new(),
            activity: HashMap::new(),
        }
    }

    /// Get or initialize a trust score for an entity.
    pub fn get_trust(&mut self, entity_id: EntityId, entity_type: TrustEntityType) -> &TrustScore {
        self.scores.entry(entity_id).or_insert_with(|| {
            debug!(entity_id = %entity_id, "initializing trust score");
            TrustScore {
                id: EntityId::new(),
                entity_type,
                entity_id,
                score: self.config.initial_trust,
                validation_count: 0,
                false_report_count: 0,
                computed_at: Utc::now(),
            }
        })
    }

    /// Record a successful validation — boosts trust.
    pub fn record_validation(&mut self, entity_id: EntityId, entity_type: TrustEntityType) -> f64 {
        let config = self.config.clone();
        let score = self.scores.entry(entity_id).or_insert_with(|| TrustScore {
            id: EntityId::new(),
            entity_type,
            entity_id,
            score: config.initial_trust,
            validation_count: 0,
            false_report_count: 0,
            computed_at: Utc::now(),
        });

        score.validation_count += 1;
        score.score = (score.score + config.validation_bonus).min(config.max_trust);
        score.computed_at = Utc::now();

        debug!(entity_id = %entity_id, trust = score.score, "validation recorded");
        score.score
    }

    /// Record a false report — reduces trust.
    pub fn record_false_report(
        &mut self,
        entity_id: EntityId,
        entity_type: TrustEntityType,
    ) -> f64 {
        let config = self.config.clone();
        let score = self.scores.entry(entity_id).or_insert_with(|| TrustScore {
            id: EntityId::new(),
            entity_type,
            entity_id,
            score: config.initial_trust,
            validation_count: 0,
            false_report_count: 0,
            computed_at: Utc::now(),
        });

        score.false_report_count += 1;
        score.score = (score.score - config.false_report_penalty).max(config.min_trust);
        score.computed_at = Utc::now();

        warn!(entity_id = %entity_id, trust = score.score, "false report recorded");
        score.score
    }

    /// Check if an entity is rate-limited.
    pub fn check_rate_limit(&self, entity_id: &EntityId) -> bool {
        self.activity
            .get(entity_id)
            .map(|a| a.reports_last_hour() >= self.config.max_reports_per_hour)
            .unwrap_or(false)
    }

    /// Record a report for rate-limiting purposes. Returns false if rate-limited.
    pub fn record_report(&mut self, entity_id: EntityId) -> bool {
        let activity = self
            .activity
            .entry(entity_id)
            .or_insert_with(ActivityRecord::new);

        if activity.reports_last_hour() >= self.config.max_reports_per_hour {
            warn!(entity_id = %entity_id, "rate limited");
            return false;
        }

        activity.record_report();
        true
    }

    /// Get trust score value for an entity (None if not tracked).
    pub fn trust_score(&self, entity_id: &EntityId) -> Option<f64> {
        self.scores.get(entity_id).map(|s| s.score)
    }

    /// Check if an entity is trusted (score above threshold).
    pub fn is_trusted(&self, entity_id: &EntityId, threshold: f64) -> bool {
        self.scores
            .get(entity_id)
            .map(|s| s.score >= threshold)
            .unwrap_or(false)
    }

    /// Apply trust decay to all scores based on time since last computation.
    pub fn apply_decay(&mut self) {
        let now = Utc::now();
        let config = &self.config;

        for score in self.scores.values_mut() {
            let days_since = (now - score.computed_at).num_seconds() as f64 / 86400.0;
            if days_since > 0.0 {
                let decay = config.decay_rate_per_day * days_since;
                score.score = (score.score - decay).max(config.min_trust);
                score.computed_at = now;
            }
        }

        info!("trust decay applied to all scores");
    }

    /// Compute event trust based on multiple reporter trust scores.
    /// Uses weighted average where reporter trust is the weight.
    pub fn compute_event_trust(&self, reporter_ids: &[EntityId]) -> f64 {
        if reporter_ids.is_empty() {
            return 0.0;
        }

        let mut total_weight: f64 = 0.0;
        let mut weighted_sum: f64 = 0.0;

        for id in reporter_ids {
            let trust = self.trust_score(id).unwrap_or(self.config.initial_trust);
            weighted_sum += trust * trust; // weight by trust squared for stronger effect
            total_weight += trust;
        }

        if total_weight <= 0.0 {
            return 0.0;
        }

        (weighted_sum / total_weight).clamp(0.0, 1.0)
    }

    /// Number of tracked entities.
    pub fn entity_count(&self) -> usize {
        self.scores.len()
    }

    /// Get all entities below a trust threshold.
    pub fn low_trust_entities(&self, threshold: f64) -> Vec<EntityId> {
        self.scores
            .iter()
            .filter(|(_, s)| s.score < threshold)
            .map(|(id, _)| *id)
            .collect()
    }
}

impl Default for ReputationEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_trust_is_default() {
        let mut engine = ReputationEngine::new();
        let user = EntityId::new();
        let score = engine.get_trust(user, TrustEntityType::User);
        assert!((score.score - 0.5).abs() < 0.001);
    }

    #[test]
    fn validation_increases_trust() {
        let mut engine = ReputationEngine::new();
        let user = EntityId::new();

        let t1 = engine.record_validation(user, TrustEntityType::User);
        let t2 = engine.record_validation(user, TrustEntityType::User);

        assert!(t2 > t1);
        assert!(t2 > 0.5);
    }

    #[test]
    fn false_report_decreases_trust() {
        let mut engine = ReputationEngine::new();
        let user = EntityId::new();

        engine.record_validation(user, TrustEntityType::User);
        let before = engine.trust_score(&user).unwrap();

        engine.record_false_report(user, TrustEntityType::User);
        let after = engine.trust_score(&user).unwrap();

        assert!(after < before);
    }

    #[test]
    fn trust_clamped_to_bounds() {
        let mut engine = ReputationEngine::new();
        let user = EntityId::new();

        // Drive trust to minimum.
        for _ in 0..20 {
            engine.record_false_report(user, TrustEntityType::User);
        }
        assert!((engine.trust_score(&user).unwrap()).abs() < 0.001);

        // Drive trust to maximum.
        let user2 = EntityId::new();
        for _ in 0..100 {
            engine.record_validation(user2, TrustEntityType::User);
        }
        assert!((engine.trust_score(&user2).unwrap() - 1.0).abs() < 0.001);
    }

    #[test]
    fn rate_limiting_works() {
        let config = ReputationConfig {
            max_reports_per_hour: 3,
            ..Default::default()
        };
        let mut engine = ReputationEngine::with_config(config);
        let user = EntityId::new();

        assert!(engine.record_report(user));
        assert!(engine.record_report(user));
        assert!(engine.record_report(user));
        assert!(!engine.record_report(user)); // rate limited
        assert!(engine.check_rate_limit(&user));
    }

    #[test]
    fn event_trust_from_multiple_reporters() {
        let mut engine = ReputationEngine::new();
        let u1 = EntityId::new();
        let u2 = EntityId::new();
        let u3 = EntityId::new();

        // Give u1 high trust.
        for _ in 0..10 {
            engine.record_validation(u1, TrustEntityType::User);
        }
        // Give u2 low trust.
        for _ in 0..3 {
            engine.record_false_report(u2, TrustEntityType::User);
        }
        // u3 has default trust.
        engine.get_trust(u3, TrustEntityType::User);

        let event_trust = engine.compute_event_trust(&[u1, u2, u3]);
        assert!(event_trust > 0.0 && event_trust <= 1.0);
    }

    #[test]
    fn low_trust_entities_detected() {
        let mut engine = ReputationEngine::new();
        let good = EntityId::new();
        let bad = EntityId::new();

        for _ in 0..5 {
            engine.record_validation(good, TrustEntityType::User);
        }
        for _ in 0..5 {
            engine.record_false_report(bad, TrustEntityType::User);
        }

        let low = engine.low_trust_entities(0.3);
        assert!(low.contains(&bad));
        assert!(!low.contains(&good));
    }
}
