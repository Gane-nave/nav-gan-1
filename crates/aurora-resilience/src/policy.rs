//! Degradation policy — rules for graceful degradation when resources are scarce.
//!
//! Defines feature tiers and automatic feature shedding based on
//! connectivity, battery, storage, and signal quality.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

/// Resource constraint that can trigger degradation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceConstraint {
    /// No internet connectivity.
    NoConnectivity,
    /// Low battery (below threshold).
    LowBattery,
    /// Storage nearly full.
    LowStorage,
    /// GNSS signal degraded.
    DegradedGnss,
    /// High CPU temperature / thermal throttling.
    ThermalThrottle,
    /// High memory pressure.
    MemoryPressure,
}

/// Feature tier — higher tiers are shed first when resources are scarce.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FeatureTier {
    /// Essential — never shed (basic navigation, safety alerts).
    Essential,
    /// Important — shed only under severe constraints (turn-by-turn, offline maps).
    Important,
    /// Standard — shed under moderate constraints (traffic, POI, routing alternatives).
    Standard,
    /// Enhanced — shed first (3D buildings, satellite imagery, analytics).
    Enhanced,
    /// Luxury — shed immediately when any constraint active (animations, themes, HQ audio).
    Luxury,
}

/// A feature with its tier and current state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedFeature {
    pub name: String,
    pub tier: FeatureTier,
    pub enabled: bool,
    pub shed_reason: Option<ResourceConstraint>,
    pub shed_at: Option<DateTime<Utc>>,
}

/// A degradation rule mapping a constraint to a minimum tier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DegradationRule {
    pub constraint: ResourceConstraint,
    /// Features at or above this tier will be shed.
    pub shed_above_tier: FeatureTier,
    /// Description of the rule.
    pub description: String,
}

/// Degradation policy engine — manages feature shedding.
pub struct DegradationPolicy {
    features: Vec<ManagedFeature>,
    rules: Vec<DegradationRule>,
    active_constraints: Vec<ResourceConstraint>,
    total_shed_events: u64,
    total_restore_events: u64,
}

impl DegradationPolicy {
    pub fn new() -> Self {
        Self {
            features: Vec::new(),
            rules: Vec::new(),
            active_constraints: Vec::new(),
            total_shed_events: 0,
            total_restore_events: 0,
        }
    }

    /// Register a feature with its tier.
    pub fn register_feature(&mut self, name: &str, tier: FeatureTier) {
        self.features.push(ManagedFeature {
            name: name.to_string(),
            tier,
            enabled: true,
            shed_reason: None,
            shed_at: None,
        });
    }

    /// Add a degradation rule.
    pub fn add_rule(
        &mut self,
        constraint: ResourceConstraint,
        shed_above: FeatureTier,
        desc: &str,
    ) {
        self.rules.push(DegradationRule {
            constraint,
            shed_above_tier: shed_above,
            description: desc.to_string(),
        });
    }

    /// Report a resource constraint. Triggers feature shedding per rules.
    /// Returns the number of features shed.
    pub fn report_constraint(&mut self, constraint: ResourceConstraint) -> usize {
        if self.active_constraints.contains(&constraint) {
            return 0;
        }
        self.active_constraints.push(constraint);

        let mut shed_count = 0;
        // Find the minimum shed tier for this constraint.
        let shed_tier = self
            .rules
            .iter()
            .filter(|r| r.constraint == constraint)
            .map(|r| r.shed_above_tier)
            .min();

        let Some(min_tier) = shed_tier else {
            debug!(constraint = ?constraint, "no degradation rule for constraint");
            return 0;
        };

        for feature in &mut self.features {
            if feature.enabled && feature.tier >= min_tier {
                feature.enabled = false;
                feature.shed_reason = Some(constraint);
                feature.shed_at = Some(Utc::now());
                shed_count += 1;
                self.total_shed_events += 1;
                info!(
                    feature = %feature.name,
                    tier = ?feature.tier,
                    constraint = ?constraint,
                    "feature shed due to resource constraint"
                );
            }
        }

        if shed_count > 0 {
            warn!(
                constraint = ?constraint,
                shed = shed_count,
                "degradation policy applied"
            );
        }

        shed_count
    }

    /// Clear a resource constraint. Restores features if no other constraint
    /// requires them to be shed.
    /// Returns the number of features restored.
    pub fn clear_constraint(&mut self, constraint: ResourceConstraint) -> usize {
        self.active_constraints.retain(|c| *c != constraint);

        let mut restored = 0;

        for feature in &mut self.features {
            if !feature.enabled {
                // Check if any remaining active constraint still requires shedding.
                let still_shed = self.rules.iter().any(|r| {
                    self.active_constraints.contains(&r.constraint)
                        && feature.tier >= r.shed_above_tier
                });

                if !still_shed {
                    feature.enabled = true;
                    feature.shed_reason = None;
                    feature.shed_at = None;
                    restored += 1;
                    self.total_restore_events += 1;
                    info!(feature = %feature.name, "feature restored");
                }
            }
        }

        restored
    }

    /// Get all currently enabled features.
    pub fn enabled_features(&self) -> Vec<&ManagedFeature> {
        self.features.iter().filter(|f| f.enabled).collect()
    }

    /// Get all currently shed features.
    pub fn shed_features(&self) -> Vec<&ManagedFeature> {
        self.features.iter().filter(|f| !f.enabled).collect()
    }

    /// Check if a named feature is currently enabled.
    pub fn is_enabled(&self, name: &str) -> bool {
        self.features
            .iter()
            .find(|f| f.name == name)
            .map(|f| f.enabled)
            .unwrap_or(false)
    }

    /// Get active constraints.
    pub fn active_constraints(&self) -> &[ResourceConstraint] {
        &self.active_constraints
    }

    /// Total shed events.
    pub fn total_shed_events(&self) -> u64 {
        self.total_shed_events
    }

    /// Total restore events.
    pub fn total_restore_events(&self) -> u64 {
        self.total_restore_events
    }

    /// Number of registered features.
    pub fn feature_count(&self) -> usize {
        self.features.len()
    }
}

impl Default for DegradationPolicy {
    fn default() -> Self {
        let mut policy = Self::new();

        // Register default features per tier.
        policy.register_feature("basic_navigation", FeatureTier::Essential);
        policy.register_feature("safety_alerts", FeatureTier::Essential);
        policy.register_feature("turn_by_turn", FeatureTier::Important);
        policy.register_feature("offline_maps", FeatureTier::Important);
        policy.register_feature("traffic_overlay", FeatureTier::Standard);
        policy.register_feature("route_alternatives", FeatureTier::Standard);
        policy.register_feature("poi_search", FeatureTier::Standard);
        policy.register_feature("3d_buildings", FeatureTier::Enhanced);
        policy.register_feature("satellite_imagery", FeatureTier::Enhanced);
        policy.register_feature("analytics", FeatureTier::Enhanced);
        policy.register_feature("animations", FeatureTier::Luxury);
        policy.register_feature("hq_audio", FeatureTier::Luxury);
        policy.register_feature("theme_engine", FeatureTier::Luxury);

        // Default degradation rules.
        policy.add_rule(
            ResourceConstraint::LowBattery,
            FeatureTier::Enhanced,
            "Shed enhanced+ features on low battery",
        );
        policy.add_rule(
            ResourceConstraint::NoConnectivity,
            FeatureTier::Standard,
            "Shed standard+ features when offline (need server)",
        );
        policy.add_rule(
            ResourceConstraint::ThermalThrottle,
            FeatureTier::Enhanced,
            "Shed enhanced+ on thermal throttle",
        );
        policy.add_rule(
            ResourceConstraint::LowStorage,
            FeatureTier::Luxury,
            "Shed luxury features on low storage",
        );
        policy.add_rule(
            ResourceConstraint::MemoryPressure,
            FeatureTier::Enhanced,
            "Shed enhanced+ on memory pressure",
        );

        policy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_policy_has_13_features() {
        let policy = DegradationPolicy::default();
        assert_eq!(policy.feature_count(), 13);
        assert_eq!(policy.enabled_features().len(), 13);
    }

    #[test]
    fn low_battery_sheds_enhanced_and_luxury() {
        let mut policy = DegradationPolicy::default();
        let shed = policy.report_constraint(ResourceConstraint::LowBattery);
        assert_eq!(shed, 6); // 3 Enhanced + 3 Luxury

        // Essential and Important still enabled.
        assert!(policy.is_enabled("basic_navigation"));
        assert!(policy.is_enabled("turn_by_turn"));
        assert!(policy.is_enabled("traffic_overlay")); // Standard, not shed by battery.
        assert!(!policy.is_enabled("3d_buildings")); // Enhanced, shed.
        assert!(!policy.is_enabled("animations")); // Luxury, shed.
    }

    #[test]
    fn no_connectivity_sheds_standard_and_above() {
        let mut policy = DegradationPolicy::default();
        let shed = policy.report_constraint(ResourceConstraint::NoConnectivity);
        // Standard(3) + Enhanced(3) + Luxury(3) = 9
        assert_eq!(shed, 9);

        assert!(policy.is_enabled("basic_navigation"));
        assert!(policy.is_enabled("offline_maps"));
        assert!(!policy.is_enabled("traffic_overlay"));
        assert!(!policy.is_enabled("poi_search"));
    }

    #[test]
    fn clear_constraint_restores_features() {
        let mut policy = DegradationPolicy::default();
        policy.report_constraint(ResourceConstraint::LowBattery);
        assert!(!policy.is_enabled("3d_buildings"));

        let restored = policy.clear_constraint(ResourceConstraint::LowBattery);
        assert_eq!(restored, 6);
        assert!(policy.is_enabled("3d_buildings"));
    }

    #[test]
    fn overlapping_constraints_keep_features_shed() {
        let mut policy = DegradationPolicy::default();

        // Both low battery and thermal throttle shed Enhanced+.
        policy.report_constraint(ResourceConstraint::LowBattery);
        policy.report_constraint(ResourceConstraint::ThermalThrottle);

        // Clearing battery doesn't restore — thermal still active.
        let restored = policy.clear_constraint(ResourceConstraint::LowBattery);
        assert_eq!(restored, 0);
        assert!(!policy.is_enabled("3d_buildings"));

        // Clearing thermal restores.
        let restored = policy.clear_constraint(ResourceConstraint::ThermalThrottle);
        assert_eq!(restored, 6);
        assert!(policy.is_enabled("3d_buildings"));
    }

    #[test]
    fn duplicate_constraint_is_idempotent() {
        let mut policy = DegradationPolicy::default();
        let shed1 = policy.report_constraint(ResourceConstraint::LowBattery);
        let shed2 = policy.report_constraint(ResourceConstraint::LowBattery);
        assert_eq!(shed1, 6);
        assert_eq!(shed2, 0); // Already applied.
    }

    #[test]
    fn essential_never_shed() {
        let mut policy = DegradationPolicy::default();

        // Apply all constraints.
        policy.report_constraint(ResourceConstraint::NoConnectivity);
        policy.report_constraint(ResourceConstraint::LowBattery);
        policy.report_constraint(ResourceConstraint::LowStorage);
        policy.report_constraint(ResourceConstraint::ThermalThrottle);
        policy.report_constraint(ResourceConstraint::MemoryPressure);

        assert!(policy.is_enabled("basic_navigation"));
        assert!(policy.is_enabled("safety_alerts"));
    }

    #[test]
    fn shed_and_restore_counters() {
        let mut policy = DegradationPolicy::default();
        policy.report_constraint(ResourceConstraint::LowBattery);
        assert_eq!(policy.total_shed_events(), 6);

        policy.clear_constraint(ResourceConstraint::LowBattery);
        assert_eq!(policy.total_restore_events(), 6);
    }

    #[test]
    fn shed_features_listed() {
        let mut policy = DegradationPolicy::default();
        policy.report_constraint(ResourceConstraint::LowStorage);
        // Low storage sheds Luxury only.
        assert_eq!(policy.shed_features().len(), 3);
    }

    #[test]
    fn is_enabled_unknown_feature_returns_false() {
        let policy = DegradationPolicy::default();
        assert!(!policy.is_enabled("nonexistent_feature"));
    }

    #[test]
    fn active_constraints_tracked() {
        let mut policy = DegradationPolicy::default();
        assert!(policy.active_constraints().is_empty());

        policy.report_constraint(ResourceConstraint::LowBattery);
        assert_eq!(policy.active_constraints().len(), 1);

        policy.clear_constraint(ResourceConstraint::LowBattery);
        assert!(policy.active_constraints().is_empty());
    }
}
