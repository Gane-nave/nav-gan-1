//! User reputation system — tracks reliability and trustworthiness of community contributors.

use std::collections::HashMap;

/// Reputation tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReputationTier {
    /// New user, unverified.
    Newcomer,
    /// Some contributions, building trust.
    Contributor,
    /// Reliable contributor.
    Trusted,
    /// Highly reliable, many verified contributions.
    Expert,
    /// Top-tier community member.
    Guardian,
}

/// A reputation event (positive or negative).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReputationEvent {
    /// Report was confirmed by community.
    ReportConfirmed,
    /// Report was rejected by community.
    ReportRejected,
    /// Shared route was used by others.
    RouteUsed,
    /// Map correction was verified.
    CorrectionVerified,
    /// Spam or abuse detected.
    SpamDetected,
    /// Daily active participation bonus.
    DailyActive,
}

/// User reputation profile.
#[derive(Debug, Clone)]
pub struct UserReputation {
    /// User ID.
    pub user_id: u64,
    /// Reputation score (0.0 to 1.0).
    pub score: f64,
    /// Current tier.
    pub tier: ReputationTier,
    /// Total positive events.
    pub positive_events: u64,
    /// Total negative events.
    pub negative_events: u64,
    /// Contribution count.
    pub contributions: u64,
}

/// Configuration for reputation system.
#[derive(Debug, Clone)]
pub struct ReputationConfig {
    /// Score gain per positive event.
    pub positive_gain: f64,
    /// Score loss per negative event.
    pub negative_loss: f64,
    /// Score threshold for Contributor tier.
    pub contributor_threshold: f64,
    /// Score threshold for Trusted tier.
    pub trusted_threshold: f64,
    /// Score threshold for Expert tier.
    pub expert_threshold: f64,
    /// Score threshold for Guardian tier.
    pub guardian_threshold: f64,
    /// Initial score for new users.
    pub initial_score: f64,
    /// Spam penalty multiplier.
    pub spam_penalty_multiplier: f64,
}

impl Default for ReputationConfig {
    fn default() -> Self {
        Self {
            positive_gain: 0.02,
            negative_loss: 0.05,
            contributor_threshold: 0.3,
            trusted_threshold: 0.5,
            expert_threshold: 0.7,
            guardian_threshold: 0.9,
            initial_score: 0.2,
            spam_penalty_multiplier: 3.0,
        }
    }
}

/// Reputation management engine.
pub struct ReputationManager {
    config: ReputationConfig,
    users: HashMap<u64, UserReputation>,
}

impl ReputationManager {
    /// Create a new reputation manager.
    pub fn new(config: ReputationConfig) -> Self {
        Self {
            config,
            users: HashMap::new(),
        }
    }

    /// Create with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(ReputationConfig::default())
    }

    /// Get or create a user's reputation.
    pub fn get_or_create(&mut self, user_id: u64) -> &UserReputation {
        self.users.entry(user_id).or_insert_with(|| UserReputation {
            user_id,
            score: self.config.initial_score,
            tier: ReputationTier::Newcomer,
            positive_events: 0,
            negative_events: 0,
            contributions: 0,
        });
        self.users.get(&user_id).unwrap()
    }

    /// Record a reputation event.
    pub fn record_event(&mut self, user_id: u64, event: ReputationEvent) -> &UserReputation {
        let initial_score = self.config.initial_score;
        let positive_gain = self.config.positive_gain;
        let negative_loss = self.config.negative_loss;
        let spam_multiplier = self.config.spam_penalty_multiplier;

        let user = self.users.entry(user_id).or_insert(UserReputation {
            user_id,
            score: initial_score,
            tier: ReputationTier::Newcomer,
            positive_events: 0,
            negative_events: 0,
            contributions: 0,
        });

        match event {
            ReputationEvent::ReportConfirmed
            | ReputationEvent::RouteUsed
            | ReputationEvent::CorrectionVerified
            | ReputationEvent::DailyActive => {
                user.score = (user.score + positive_gain).min(1.0);
                user.positive_events += 1;
                user.contributions += 1;
            }
            ReputationEvent::ReportRejected => {
                user.score = (user.score - negative_loss).max(0.0);
                user.negative_events += 1;
            }
            ReputationEvent::SpamDetected => {
                let penalty = negative_loss * spam_multiplier;
                user.score = (user.score - penalty).max(0.0);
                user.negative_events += 1;
            }
        }

        // Update tier
        user.tier = Self::score_to_tier_with_config(&self.config, user.score);
        self.users.get(&user_id).unwrap()
    }

    /// Get a user's reputation (read-only).
    pub fn get(&self, user_id: u64) -> Option<&UserReputation> {
        self.users.get(&user_id)
    }

    /// Get the total number of tracked users.
    pub fn user_count(&self) -> usize {
        self.users.len()
    }

    /// Get top users by reputation score.
    pub fn top_users(&self, limit: usize) -> Vec<&UserReputation> {
        let mut users: Vec<&UserReputation> = self.users.values().collect();
        users.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        users.truncate(limit);
        users
    }

    fn score_to_tier_with_config(config: &ReputationConfig, score: f64) -> ReputationTier {
        if score >= config.guardian_threshold {
            ReputationTier::Guardian
        } else if score >= config.expert_threshold {
            ReputationTier::Expert
        } else if score >= config.trusted_threshold {
            ReputationTier::Trusted
        } else if score >= config.contributor_threshold {
            ReputationTier::Contributor
        } else {
            ReputationTier::Newcomer
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_user_starts_as_newcomer() {
        let mut mgr = ReputationManager::with_defaults();
        let rep = mgr.get_or_create(1);
        assert_eq!(rep.tier, ReputationTier::Newcomer);
        assert!((rep.score - 0.2).abs() < 0.01);
    }

    #[test]
    fn test_positive_events_increase_score() {
        let mut mgr = ReputationManager::with_defaults();
        mgr.record_event(1, ReputationEvent::ReportConfirmed);
        let rep = mgr.get(1).unwrap();
        assert!(rep.score > 0.2);
        assert_eq!(rep.positive_events, 1);
        assert_eq!(rep.contributions, 1);
    }

    #[test]
    fn test_negative_events_decrease_score() {
        let mut mgr = ReputationManager::with_defaults();
        mgr.record_event(1, ReputationEvent::ReportRejected);
        let rep = mgr.get(1).unwrap();
        assert!(rep.score < 0.2);
        assert_eq!(rep.negative_events, 1);
    }

    #[test]
    fn test_spam_penalty_larger() {
        let mut mgr = ReputationManager::with_defaults();
        let rep_before = mgr.get_or_create(1).score;
        mgr.record_event(1, ReputationEvent::SpamDetected);
        let rep_after = mgr.get(1).unwrap().score;
        let spam_loss = rep_before - rep_after;
        assert!(
            spam_loss > 0.05,
            "Spam penalty ({spam_loss}) should exceed normal loss (0.05)"
        );
    }

    #[test]
    fn test_tier_progression() {
        let mut mgr = ReputationManager::with_defaults();
        // Start at 0.2, each positive event adds 0.02
        // Need to reach 0.3 for Contributor → 5 events
        for _ in 0..6 {
            mgr.record_event(1, ReputationEvent::ReportConfirmed);
        }
        assert_eq!(mgr.get(1).unwrap().tier, ReputationTier::Contributor);
    }

    #[test]
    fn test_score_clamped_at_zero() {
        let mut mgr = ReputationManager::with_defaults();
        for _ in 0..50 {
            mgr.record_event(1, ReputationEvent::SpamDetected);
        }
        assert_eq!(mgr.get(1).unwrap().score, 0.0);
    }

    #[test]
    fn test_score_clamped_at_one() {
        let mut mgr = ReputationManager::with_defaults();
        for _ in 0..100 {
            mgr.record_event(1, ReputationEvent::ReportConfirmed);
        }
        assert_eq!(mgr.get(1).unwrap().score, 1.0);
    }

    #[test]
    fn test_user_count() {
        let mut mgr = ReputationManager::with_defaults();
        mgr.record_event(1, ReputationEvent::DailyActive);
        mgr.record_event(2, ReputationEvent::DailyActive);
        mgr.record_event(3, ReputationEvent::DailyActive);
        assert_eq!(mgr.user_count(), 3);
    }

    #[test]
    fn test_top_users() {
        let mut mgr = ReputationManager::with_defaults();
        for _ in 0..10 {
            mgr.record_event(1, ReputationEvent::ReportConfirmed);
        }
        for _ in 0..5 {
            mgr.record_event(2, ReputationEvent::ReportConfirmed);
        }
        mgr.record_event(3, ReputationEvent::DailyActive);
        let top = mgr.top_users(2);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].user_id, 1); // Highest score first
    }

    #[test]
    fn test_tier_ordering() {
        assert!(ReputationTier::Newcomer < ReputationTier::Contributor);
        assert!(ReputationTier::Contributor < ReputationTier::Trusted);
        assert!(ReputationTier::Trusted < ReputationTier::Expert);
        assert!(ReputationTier::Expert < ReputationTier::Guardian);
    }
}
