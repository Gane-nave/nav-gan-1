//! Infrastructure health monitoring — road condition detection, maintenance
//! scheduling, health scoring, and issue lifecycle management.

use aurora_core::infrastructure::{
    InfrastructureCondition, InfrastructureHealthRecord, InfrastructureIssue,
    InfrastructureIssueType, InfrastructureState,
};
use aurora_core::types::{EntityId, GeoPosition};
use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use tracing::{debug, info};

// ---------------------------------------------------------------------------
// Maintenance
// ---------------------------------------------------------------------------

/// Priority level for maintenance work orders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MaintenancePriority {
    Low,
    Medium,
    High,
    Urgent,
}

/// A maintenance work order generated from detected issues.
#[derive(Debug, Clone)]
pub struct MaintenanceOrder {
    pub id: EntityId,
    pub segment_id: EntityId,
    pub issue_type: InfrastructureIssueType,
    pub priority: MaintenancePriority,
    pub position: GeoPosition,
    pub created_at: DateTime<Utc>,
    pub scheduled_for: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Condition trend direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionTrend {
    Improving,
    Stable,
    Degrading,
}

/// Detailed health assessment for a region.
#[derive(Debug, Clone)]
pub struct RegionHealthAssessment {
    pub region: String,
    pub overall_score: f64,
    pub segment_count: usize,
    pub critical_count: usize,
    pub poor_count: usize,
    pub trend: ConditionTrend,
    pub assessed_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Infrastructure monitor
// ---------------------------------------------------------------------------

/// Configuration for the infrastructure monitor.
#[derive(Debug, Clone)]
pub struct InfrastructureMonitorConfig {
    /// Severity threshold above which an issue is considered urgent.
    pub urgent_severity_threshold: f64,
    /// Number of days after which an uninspected segment is flagged.
    pub inspection_overdue_days: i64,
    /// Weight of road condition in overall health score.
    pub road_condition_weight: f64,
    /// Weight of signal health in overall health score.
    pub signal_health_weight: f64,
    /// Weight of marking condition in overall health score.
    pub marking_weight: f64,
}

impl Default for InfrastructureMonitorConfig {
    fn default() -> Self {
        Self {
            urgent_severity_threshold: 0.8,
            inspection_overdue_days: 90,
            road_condition_weight: 0.5,
            signal_health_weight: 0.3,
            marking_weight: 0.2,
        }
    }
}

/// Monitors infrastructure health across the road network, detects
/// degradation, generates maintenance orders, and computes regional
/// health scores.
pub struct InfrastructureMonitor {
    config: InfrastructureMonitorConfig,
    /// Per-segment infrastructure state.
    segments: HashMap<EntityId, InfrastructureState>,
    /// Historical health records per region.
    health_history: HashMap<String, Vec<InfrastructureHealthRecord>>,
    /// Active maintenance orders.
    maintenance_orders: Vec<MaintenanceOrder>,
}

impl InfrastructureMonitor {
    pub fn new() -> Self {
        Self {
            config: InfrastructureMonitorConfig::default(),
            segments: HashMap::new(),
            health_history: HashMap::new(),
            maintenance_orders: Vec::new(),
        }
    }

    pub fn with_config(config: InfrastructureMonitorConfig) -> Self {
        Self {
            config,
            ..Self::new()
        }
    }

    // -----------------------------------------------------------------------
    // Segment management
    // -----------------------------------------------------------------------

    /// Register or update an infrastructure segment.
    pub fn update_segment(&mut self, state: InfrastructureState) {
        debug!(segment = %state.id, condition = ?state.condition, "segment updated");
        self.segments.insert(state.id, state);
    }

    /// Get the current state of a segment.
    pub fn get_segment(&self, id: &EntityId) -> Option<&InfrastructureState> {
        self.segments.get(id)
    }

    /// Number of monitored segments.
    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }

    // -----------------------------------------------------------------------
    // Issue detection & maintenance
    // -----------------------------------------------------------------------

    /// Report an issue on a segment. Automatically generates a maintenance
    /// order if severity warrants it.
    pub fn report_issue(
        &mut self,
        segment_id: EntityId,
        issue: InfrastructureIssue,
    ) -> Option<MaintenanceOrder> {
        let segment = self.segments.get_mut(&segment_id)?;
        let severity = issue.severity;
        let issue_type = issue.issue_type;
        let position = issue.position;
        segment.issues.push(issue);
        segment.updated_at = Utc::now();

        // Recalculate condition based on worst issue.
        let worst = segment
            .issues
            .iter()
            .map(|i| i.severity)
            .fold(0.0_f64, f64::max);
        segment.condition = severity_to_condition(worst);

        // Generate maintenance order for urgent issues.
        let priority = severity_to_priority(severity, &self.config);
        if priority >= MaintenancePriority::High {
            let order = MaintenanceOrder {
                id: EntityId::new(),
                segment_id,
                issue_type,
                priority,
                position,
                created_at: Utc::now(),
                scheduled_for: None,
                completed_at: None,
            };
            info!(
                segment = %segment_id,
                priority = ?priority,
                issue = ?issue_type,
                "maintenance order created"
            );
            self.maintenance_orders.push(order.clone());
            return Some(order);
        }

        None
    }

    /// Resolve an issue on a segment (e.g., after maintenance).
    pub fn resolve_issue(
        &mut self,
        segment_id: &EntityId,
        issue_type: InfrastructureIssueType,
    ) -> bool {
        let Some(segment) = self.segments.get_mut(segment_id) else {
            return false;
        };

        let before = segment.issues.len();
        segment.issues.retain(|i| i.issue_type != issue_type);
        let removed = segment.issues.len() < before;

        if removed {
            // Recalculate condition.
            let worst = segment
                .issues
                .iter()
                .map(|i| i.severity)
                .fold(0.0_f64, f64::max);
            segment.condition = severity_to_condition(worst);
            segment.updated_at = Utc::now();
            debug!(segment = %segment_id, new_condition = ?segment.condition, "issue resolved");
        }

        removed
    }

    /// Get segments whose last inspection is overdue.
    pub fn overdue_inspections(&self) -> Vec<EntityId> {
        let now = Utc::now();
        let cutoff = now - Duration::days(self.config.inspection_overdue_days);

        self.segments
            .iter()
            .filter(|(_, s)| s.last_inspection.map(|t| t < cutoff).unwrap_or(true))
            .map(|(id, _)| *id)
            .collect()
    }

    /// Get all active maintenance orders.
    pub fn active_maintenance_orders(&self) -> Vec<&MaintenanceOrder> {
        self.maintenance_orders
            .iter()
            .filter(|o| o.completed_at.is_none())
            .collect()
    }

    /// Complete a maintenance order.
    pub fn complete_maintenance(&mut self, order_id: &EntityId) -> bool {
        if let Some(order) = self
            .maintenance_orders
            .iter_mut()
            .find(|o| o.id == *order_id)
        {
            order.completed_at = Some(Utc::now());
            info!(order = %order_id, "maintenance completed");
            return true;
        }
        false
    }

    // -----------------------------------------------------------------------
    // Health scoring
    // -----------------------------------------------------------------------

    /// Compute regional health assessment by aggregating segment conditions.
    pub fn assess_region(
        &mut self,
        region: &str,
        segment_ids: &[EntityId],
    ) -> RegionHealthAssessment {
        let mut total_score = 0.0;
        let mut count = 0usize;
        let mut critical = 0usize;
        let mut poor = 0usize;

        for id in segment_ids {
            if let Some(seg) = self.segments.get(id) {
                count += 1;
                let score = condition_to_score(seg.condition);
                total_score += score;
                match seg.condition {
                    InfrastructureCondition::Critical => critical += 1,
                    InfrastructureCondition::Poor => poor += 1,
                    _ => {}
                }
            }
        }

        let overall = if count > 0 {
            total_score / count as f64
        } else {
            1.0
        };

        // Determine trend from history.
        let trend = self.compute_trend(region, overall);

        // Store in history.
        let record = InfrastructureHealthRecord {
            id: EntityId::new(),
            region: region.to_string(),
            overall_score: overall,
            road_condition_score: overall, // simplified
            signal_health_score: overall,
            marking_score: overall,
            computed_at: Utc::now(),
        };
        self.health_history
            .entry(region.to_string())
            .or_default()
            .push(record);

        RegionHealthAssessment {
            region: region.to_string(),
            overall_score: overall,
            segment_count: count,
            critical_count: critical,
            poor_count: poor,
            trend,
            assessed_at: Utc::now(),
        }
    }

    fn compute_trend(&self, region: &str, current_score: f64) -> ConditionTrend {
        let history = match self.health_history.get(region) {
            Some(h) if !h.is_empty() => h,
            _ => return ConditionTrend::Stable,
        };

        let prev = history
            .last()
            .map(|r| r.overall_score)
            .unwrap_or(current_score);
        let delta = current_score - prev;

        if delta > 0.05 {
            ConditionTrend::Improving
        } else if delta < -0.05 {
            ConditionTrend::Degrading
        } else {
            ConditionTrend::Stable
        }
    }

    /// Get segments in a specific condition.
    pub fn segments_in_condition(&self, condition: InfrastructureCondition) -> Vec<EntityId> {
        self.segments
            .iter()
            .filter(|(_, s)| s.condition == condition)
            .map(|(id, _)| *id)
            .collect()
    }
}

impl Default for InfrastructureMonitor {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn severity_to_condition(severity: f64) -> InfrastructureCondition {
    match severity {
        s if s >= 0.9 => InfrastructureCondition::Critical,
        s if s >= 0.6 => InfrastructureCondition::Poor,
        s if s >= 0.3 => InfrastructureCondition::Fair,
        s if s > 0.0 => InfrastructureCondition::Good,
        _ => InfrastructureCondition::Good,
    }
}

fn severity_to_priority(
    severity: f64,
    config: &InfrastructureMonitorConfig,
) -> MaintenancePriority {
    if severity >= config.urgent_severity_threshold {
        MaintenancePriority::Urgent
    } else if severity >= 0.6 {
        MaintenancePriority::High
    } else if severity >= 0.3 {
        MaintenancePriority::Medium
    } else {
        MaintenancePriority::Low
    }
}

fn condition_to_score(condition: InfrastructureCondition) -> f64 {
    match condition {
        InfrastructureCondition::Good => 1.0,
        InfrastructureCondition::Fair => 0.75,
        InfrastructureCondition::Poor => 0.4,
        InfrastructureCondition::Critical => 0.1,
        InfrastructureCondition::Unknown => 0.5,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_segment(id: EntityId, condition: InfrastructureCondition) -> InfrastructureState {
        InfrastructureState {
            id,
            segment_id: id,
            condition,
            last_inspection: Some(Utc::now()),
            issues: Vec::new(),
            updated_at: Utc::now(),
        }
    }

    fn make_issue(issue_type: InfrastructureIssueType, severity: f64) -> InfrastructureIssue {
        InfrastructureIssue {
            issue_type,
            position: GeoPosition {
                latitude_deg: 32.0,
                longitude_deg: 34.0,
                altitude_m: None,
            },
            severity,
            reported_at: Utc::now(),
        }
    }

    #[test]
    fn register_and_query_segment() {
        let mut monitor = InfrastructureMonitor::new();
        let id = EntityId::new();
        monitor.update_segment(make_segment(id, InfrastructureCondition::Good));

        assert_eq!(monitor.segment_count(), 1);
        assert_eq!(
            monitor.get_segment(&id).unwrap().condition,
            InfrastructureCondition::Good
        );
    }

    #[test]
    fn report_issue_updates_condition() {
        let mut monitor = InfrastructureMonitor::new();
        let id = EntityId::new();
        monitor.update_segment(make_segment(id, InfrastructureCondition::Good));

        let issue = make_issue(InfrastructureIssueType::Pothole, 0.7);
        monitor.report_issue(id, issue);

        assert_eq!(
            monitor.get_segment(&id).unwrap().condition,
            InfrastructureCondition::Poor
        );
    }

    #[test]
    fn urgent_issue_generates_maintenance_order() {
        let mut monitor = InfrastructureMonitor::new();
        let id = EntityId::new();
        monitor.update_segment(make_segment(id, InfrastructureCondition::Good));

        let issue = make_issue(InfrastructureIssueType::Flooding, 0.95);
        let order = monitor.report_issue(id, issue);

        assert!(order.is_some());
        let order = order.unwrap();
        assert_eq!(order.priority, MaintenancePriority::Urgent);
        assert_eq!(order.issue_type, InfrastructureIssueType::Flooding);
    }

    #[test]
    fn low_severity_no_maintenance_order() {
        let mut monitor = InfrastructureMonitor::new();
        let id = EntityId::new();
        monitor.update_segment(make_segment(id, InfrastructureCondition::Good));

        let issue = make_issue(InfrastructureIssueType::SurfaceCracking, 0.2);
        let order = monitor.report_issue(id, issue);
        assert!(order.is_none());
    }

    #[test]
    fn resolve_issue_improves_condition() {
        let mut monitor = InfrastructureMonitor::new();
        let id = EntityId::new();
        monitor.update_segment(make_segment(id, InfrastructureCondition::Good));

        let issue = make_issue(InfrastructureIssueType::Pothole, 0.7);
        monitor.report_issue(id, issue);
        assert_eq!(
            monitor.get_segment(&id).unwrap().condition,
            InfrastructureCondition::Poor
        );

        assert!(monitor.resolve_issue(&id, InfrastructureIssueType::Pothole));
        assert_eq!(
            monitor.get_segment(&id).unwrap().condition,
            InfrastructureCondition::Good
        );
    }

    #[test]
    fn overdue_inspections_detected() {
        let mut monitor = InfrastructureMonitor::new();
        let id = EntityId::new();

        let mut seg = make_segment(id, InfrastructureCondition::Good);
        seg.last_inspection = Some(Utc::now() - Duration::days(200));
        monitor.update_segment(seg);

        let overdue = monitor.overdue_inspections();
        assert!(overdue.contains(&id));
    }

    #[test]
    fn no_inspection_is_overdue() {
        let mut monitor = InfrastructureMonitor::new();
        let id = EntityId::new();

        let mut seg = make_segment(id, InfrastructureCondition::Good);
        seg.last_inspection = None;
        monitor.update_segment(seg);

        let overdue = monitor.overdue_inspections();
        assert!(overdue.contains(&id));
    }

    #[test]
    fn complete_maintenance_order() {
        let mut monitor = InfrastructureMonitor::new();
        let id = EntityId::new();
        monitor.update_segment(make_segment(id, InfrastructureCondition::Good));

        let issue = make_issue(InfrastructureIssueType::Flooding, 0.95);
        let order = monitor.report_issue(id, issue).unwrap();
        let order_id = order.id;

        assert_eq!(monitor.active_maintenance_orders().len(), 1);
        assert!(monitor.complete_maintenance(&order_id));
        assert_eq!(monitor.active_maintenance_orders().len(), 0);
    }

    #[test]
    fn region_health_assessment() {
        let mut monitor = InfrastructureMonitor::new();

        let s1 = EntityId::new();
        let s2 = EntityId::new();
        let s3 = EntityId::new();

        monitor.update_segment(make_segment(s1, InfrastructureCondition::Good));
        monitor.update_segment(make_segment(s2, InfrastructureCondition::Fair));
        monitor.update_segment(make_segment(s3, InfrastructureCondition::Critical));

        let assessment = monitor.assess_region("downtown", &[s1, s2, s3]);
        assert_eq!(assessment.segment_count, 3);
        assert_eq!(assessment.critical_count, 1);
        assert!(assessment.overall_score > 0.0);
        assert!(assessment.overall_score < 1.0);
    }

    #[test]
    fn segments_in_condition_filter() {
        let mut monitor = InfrastructureMonitor::new();
        let s1 = EntityId::new();
        let s2 = EntityId::new();

        monitor.update_segment(make_segment(s1, InfrastructureCondition::Critical));
        monitor.update_segment(make_segment(s2, InfrastructureCondition::Good));

        let critical = monitor.segments_in_condition(InfrastructureCondition::Critical);
        assert_eq!(critical.len(), 1);
        assert!(critical.contains(&s1));
    }

    #[test]
    fn trend_stable_on_first_assessment() {
        let mut monitor = InfrastructureMonitor::new();
        let s1 = EntityId::new();
        monitor.update_segment(make_segment(s1, InfrastructureCondition::Good));

        let assessment = monitor.assess_region("suburbs", &[s1]);
        assert_eq!(assessment.trend, ConditionTrend::Stable);
    }
}
