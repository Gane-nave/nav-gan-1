//! Service Level Agreement (SLA) monitoring — tracks fleet performance
//! against defined targets and generates alerts for violations.

use aurora_core::types::EntityId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::debug;

/// SLA target definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceLevelAgreement {
    pub id: EntityId,
    pub name: String,
    pub metric: SlaMetric,
    pub target_value: f64,
    pub warning_threshold: f64,
    pub critical_threshold: f64,
    pub measurement_window_min: f64,
    pub created_at: DateTime<Utc>,
}

/// Measurable SLA metric.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlaMetric {
    /// Percentage of tasks completed on time.
    OnTimeDeliveryPct,
    /// Average time from dispatch to arrival (minutes).
    AvgResponseTimeMin,
    /// Percentage of tasks completed successfully.
    CompletionRatePct,
    /// Average customer rating (0-5).
    CustomerSatisfaction,
    /// Percentage of tasks with valid proof.
    ProofCompliancePct,
    /// Average stops per hour per driver.
    StopsPerHour,
}

/// Current SLA status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlaStatus {
    /// Meeting or exceeding target.
    Met,
    /// Below target but above warning threshold.
    Warning,
    /// Below warning threshold — critical.
    Critical,
    /// Not enough data to evaluate.
    InsufficientData,
}

/// A recorded SLA measurement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlaMeasurement {
    pub sla_id: EntityId,
    pub value: f64,
    pub status: SlaStatus,
    pub sample_count: usize,
    pub measured_at: DateTime<Utc>,
}

/// SLA violation event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlaViolation {
    pub id: EntityId,
    pub sla_id: EntityId,
    pub sla_name: String,
    pub metric: SlaMetric,
    pub target: f64,
    pub actual: f64,
    pub status: SlaStatus,
    pub timestamp: DateTime<Utc>,
}

/// Task completion record for SLA computation.
#[derive(Debug, Clone)]
pub struct TaskRecord {
    pub task_id: EntityId,
    pub driver_id: EntityId,
    pub on_time: bool,
    pub completed: bool,
    pub response_time_min: f64,
    pub has_proof: bool,
    pub customer_rating: Option<f64>,
    pub timestamp: DateTime<Utc>,
}

/// SLA monitor — evaluates fleet performance against SLA targets.
pub struct SlaMonitor {
    slas: Vec<ServiceLevelAgreement>,
    records: Vec<TaskRecord>,
    violations: Vec<SlaViolation>,
    min_samples: usize,
}

impl SlaMonitor {
    pub fn new() -> Self {
        Self {
            slas: Vec::new(),
            records: Vec::new(),
            violations: Vec::new(),
            min_samples: 5,
        }
    }

    /// Register an SLA.
    pub fn add_sla(&mut self, sla: ServiceLevelAgreement) {
        debug!(sla_id = %sla.id, name = %sla.name, "SLA registered");
        self.slas.push(sla);
    }

    /// Record a task completion for SLA evaluation.
    pub fn record_task(&mut self, record: TaskRecord) {
        self.records.push(record);
    }

    /// Evaluate all SLAs and return measurements.
    pub fn evaluate_all(&mut self) -> Vec<SlaMeasurement> {
        let mut measurements = Vec::new();
        let slas: Vec<ServiceLevelAgreement> = self.slas.clone();

        for sla in &slas {
            let measurement = self.evaluate_sla(sla);
            if measurement.status == SlaStatus::Warning || measurement.status == SlaStatus::Critical
            {
                let violation = SlaViolation {
                    id: EntityId::new(),
                    sla_id: sla.id,
                    sla_name: sla.name.clone(),
                    metric: sla.metric,
                    target: sla.target_value,
                    actual: measurement.value,
                    status: measurement.status,
                    timestamp: Utc::now(),
                };
                debug!(
                    sla = %sla.name,
                    target = sla.target_value,
                    actual = measurement.value,
                    status = ?measurement.status,
                    "SLA violation detected"
                );
                self.violations.push(violation);
            }
            measurements.push(measurement);
        }
        measurements
    }

    /// Evaluate a single SLA.
    fn evaluate_sla(&self, sla: &ServiceLevelAgreement) -> SlaMeasurement {
        let now = Utc::now();
        let window_start = now - chrono::Duration::minutes(sla.measurement_window_min as i64);

        let recent_records: Vec<&TaskRecord> = self
            .records
            .iter()
            .filter(|r| r.timestamp >= window_start)
            .collect();

        if recent_records.len() < self.min_samples {
            return SlaMeasurement {
                sla_id: sla.id,
                value: 0.0,
                status: SlaStatus::InsufficientData,
                sample_count: recent_records.len(),
                measured_at: now,
            };
        }

        let value = self.compute_metric(sla.metric, &recent_records);
        let status = self.classify_status(sla, value);

        SlaMeasurement {
            sla_id: sla.id,
            value,
            status,
            sample_count: recent_records.len(),
            measured_at: now,
        }
    }

    /// Compute the value of a metric from task records.
    fn compute_metric(&self, metric: SlaMetric, records: &[&TaskRecord]) -> f64 {
        if records.is_empty() {
            return 0.0;
        }
        match metric {
            SlaMetric::OnTimeDeliveryPct => {
                let on_time = records.iter().filter(|r| r.on_time).count();
                (on_time as f64 / records.len() as f64) * 100.0
            }
            SlaMetric::AvgResponseTimeMin => {
                let sum: f64 = records.iter().map(|r| r.response_time_min).sum();
                sum / records.len() as f64
            }
            SlaMetric::CompletionRatePct => {
                let completed = records.iter().filter(|r| r.completed).count();
                (completed as f64 / records.len() as f64) * 100.0
            }
            SlaMetric::CustomerSatisfaction => {
                let rated: Vec<f64> = records.iter().filter_map(|r| r.customer_rating).collect();
                if rated.is_empty() {
                    return 0.0;
                }
                rated.iter().sum::<f64>() / rated.len() as f64
            }
            SlaMetric::ProofCompliancePct => {
                let with_proof = records.iter().filter(|r| r.has_proof).count();
                (with_proof as f64 / records.len() as f64) * 100.0
            }
            SlaMetric::StopsPerHour => {
                // Approximate: total completed / time span in hours.
                let completed = records.iter().filter(|r| r.completed).count();
                if records.len() < 2 {
                    return completed as f64;
                }
                let earliest = records.iter().map(|r| r.timestamp).min().unwrap();
                let latest = records.iter().map(|r| r.timestamp).max().unwrap();
                let hours = (latest - earliest).num_minutes() as f64 / 60.0;
                if hours < 0.01 {
                    return completed as f64;
                }
                completed as f64 / hours
            }
        }
    }

    /// Classify SLA status based on thresholds.
    fn classify_status(&self, sla: &ServiceLevelAgreement, value: f64) -> SlaStatus {
        match sla.metric {
            // Higher is better metrics.
            SlaMetric::OnTimeDeliveryPct
            | SlaMetric::CompletionRatePct
            | SlaMetric::CustomerSatisfaction
            | SlaMetric::ProofCompliancePct
            | SlaMetric::StopsPerHour => {
                // Higher is better: target >= warning >= critical.
                if value >= sla.target_value {
                    SlaStatus::Met
                } else if value >= sla.critical_threshold {
                    // Between target and critical (inclusive) → Warning.
                    SlaStatus::Warning
                } else {
                    SlaStatus::Critical
                }
            }
            // Lower is better metrics.
            SlaMetric::AvgResponseTimeMin => {
                // Lower is better: target <= warning <= critical.
                if value <= sla.target_value {
                    SlaStatus::Met
                } else if value <= sla.critical_threshold {
                    // Between target and critical (inclusive) → Warning.
                    SlaStatus::Warning
                } else {
                    SlaStatus::Critical
                }
            }
        }
    }

    /// Get all violations.
    pub fn violations(&self) -> &[SlaViolation] {
        &self.violations
    }

    /// Get SLAs.
    pub fn sla_count(&self) -> usize {
        self.slas.len()
    }

    /// Set minimum samples for evaluation.
    pub fn set_min_samples(&mut self, min: usize) {
        self.min_samples = min;
    }
}

impl Default for SlaMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_sla(metric: SlaMetric, target: f64, warn: f64, critical: f64) -> ServiceLevelAgreement {
        ServiceLevelAgreement {
            id: EntityId::new(),
            name: format!("{:?} SLA", metric),
            metric,
            target_value: target,
            warning_threshold: warn,
            critical_threshold: critical,
            measurement_window_min: 60.0, // 1 hour window
            created_at: Utc::now(),
        }
    }

    fn make_records(n: usize, on_time_pct: f64) -> Vec<TaskRecord> {
        let on_time_count = (n as f64 * on_time_pct) as usize;
        (0..n)
            .map(|i| TaskRecord {
                task_id: EntityId::new(),
                driver_id: EntityId::new(),
                on_time: i < on_time_count,
                completed: true,
                response_time_min: 15.0 + i as f64,
                has_proof: true,
                customer_rating: Some(4.0),
                timestamp: Utc::now(),
            })
            .collect()
    }

    #[test]
    fn on_time_delivery_met() {
        let mut monitor = SlaMonitor::new();
        monitor.set_min_samples(3);
        monitor.add_sla(make_sla(SlaMetric::OnTimeDeliveryPct, 90.0, 80.0, 70.0));

        for r in make_records(10, 0.95) {
            monitor.record_task(r);
        }

        let measurements = monitor.evaluate_all();
        assert_eq!(measurements.len(), 1);
        assert_eq!(measurements[0].status, SlaStatus::Met);
        assert!(measurements[0].value >= 90.0);
    }

    #[test]
    fn on_time_delivery_warning() {
        let mut monitor = SlaMonitor::new();
        monitor.set_min_samples(3);
        monitor.add_sla(make_sla(SlaMetric::OnTimeDeliveryPct, 90.0, 80.0, 70.0));

        // 85% on time → Warning (below 90% target, above 80% warning).
        for r in make_records(20, 0.85) {
            monitor.record_task(r);
        }

        let measurements = monitor.evaluate_all();
        assert_eq!(measurements[0].status, SlaStatus::Warning);
        assert_eq!(monitor.violations().len(), 1);
    }

    #[test]
    fn on_time_delivery_critical() {
        let mut monitor = SlaMonitor::new();
        monitor.set_min_samples(3);
        monitor.add_sla(make_sla(SlaMetric::OnTimeDeliveryPct, 90.0, 80.0, 70.0));

        // 50% on time → Critical.
        for r in make_records(10, 0.5) {
            monitor.record_task(r);
        }

        let measurements = monitor.evaluate_all();
        assert_eq!(measurements[0].status, SlaStatus::Critical);
    }

    #[test]
    fn insufficient_data() {
        let mut monitor = SlaMonitor::new();
        monitor.set_min_samples(10);
        monitor.add_sla(make_sla(SlaMetric::OnTimeDeliveryPct, 90.0, 80.0, 70.0));

        // Only 3 records → InsufficientData.
        for r in make_records(3, 1.0) {
            monitor.record_task(r);
        }

        let measurements = monitor.evaluate_all();
        assert_eq!(measurements[0].status, SlaStatus::InsufficientData);
    }

    #[test]
    fn response_time_sla() {
        let mut monitor = SlaMonitor::new();
        monitor.set_min_samples(3);
        // Target: ≤ 20 min, Warning: ≤ 30 min, Critical: > 30 min.
        monitor.add_sla(make_sla(SlaMetric::AvgResponseTimeMin, 20.0, 30.0, 40.0));

        for r in make_records(5, 1.0) {
            monitor.record_task(r);
        }

        let measurements = monitor.evaluate_all();
        // Avg response = 15+16+17+18+19 = 85/5 = 17.0 → Met.
        assert_eq!(measurements[0].status, SlaStatus::Met);
    }

    #[test]
    fn completion_rate_sla() {
        let mut monitor = SlaMonitor::new();
        monitor.set_min_samples(3);
        monitor.add_sla(make_sla(SlaMetric::CompletionRatePct, 95.0, 90.0, 80.0));

        let mut records = make_records(10, 1.0);
        // Mark 1 as not completed.
        records[0].completed = false;
        for r in records {
            monitor.record_task(r);
        }

        let measurements = monitor.evaluate_all();
        // 9/10 = 90% → Warning (at 90% warning threshold).
        assert_eq!(measurements[0].status, SlaStatus::Warning);
    }

    #[test]
    fn proof_compliance_sla() {
        let mut monitor = SlaMonitor::new();
        monitor.set_min_samples(3);
        monitor.add_sla(make_sla(SlaMetric::ProofCompliancePct, 100.0, 90.0, 80.0));

        let mut records = make_records(10, 1.0);
        records[0].has_proof = false;
        records[1].has_proof = false;
        for r in records {
            monitor.record_task(r);
        }

        let measurements = monitor.evaluate_all();
        // 8/10 = 80% → Warning (below 90% warning, at 80% critical threshold).
        assert_eq!(measurements[0].status, SlaStatus::Warning);
        // Note: 80% is at the critical_threshold boundary (>=), so it's Warning, not Critical.
    }

    #[test]
    fn customer_satisfaction_sla() {
        let mut monitor = SlaMonitor::new();
        monitor.set_min_samples(3);
        monitor.add_sla(make_sla(SlaMetric::CustomerSatisfaction, 4.5, 4.0, 3.5));

        let records = make_records(5, 1.0); // All rated 4.0.
        for r in records {
            monitor.record_task(r);
        }

        let measurements = monitor.evaluate_all();
        // Avg rating 4.0 → Warning (at 4.0 warning threshold).
        assert_eq!(measurements[0].status, SlaStatus::Warning);
    }

    #[test]
    fn critical_threshold_distinguishes_warning_from_critical() {
        let mut monitor = SlaMonitor::new();
        monitor.set_min_samples(3);
        // target=90, warning=80, critical=70
        monitor.add_sla(make_sla(SlaMetric::OnTimeDeliveryPct, 90.0, 80.0, 70.0));

        // 75% on time → below warning(80) but above critical(70) → Warning.
        for r in make_records(20, 0.75) {
            monitor.record_task(r);
        }
        let measurements = monitor.evaluate_all();
        assert_eq!(
            measurements[0].status,
            SlaStatus::Warning,
            "75% is below warning(80) but above critical(70), should be Warning"
        );

        // Now test truly critical: 60% → below critical(70) → Critical.
        let mut monitor2 = SlaMonitor::new();
        monitor2.set_min_samples(3);
        monitor2.add_sla(make_sla(SlaMetric::OnTimeDeliveryPct, 90.0, 80.0, 70.0));
        for r in make_records(20, 0.60) {
            monitor2.record_task(r);
        }
        let measurements2 = monitor2.evaluate_all();
        assert_eq!(
            measurements2[0].status,
            SlaStatus::Critical,
            "60% is below critical(70), should be Critical"
        );
    }

    #[test]
    fn multiple_slas_evaluated() {
        let mut monitor = SlaMonitor::new();
        monitor.set_min_samples(3);
        monitor.add_sla(make_sla(SlaMetric::OnTimeDeliveryPct, 90.0, 80.0, 70.0));
        monitor.add_sla(make_sla(SlaMetric::CompletionRatePct, 95.0, 90.0, 80.0));

        for r in make_records(10, 1.0) {
            monitor.record_task(r);
        }

        let measurements = monitor.evaluate_all();
        assert_eq!(measurements.len(), 2);
    }
}
