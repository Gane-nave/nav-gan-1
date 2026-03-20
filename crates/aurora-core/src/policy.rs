//! Policy, alert rules, and model artifact types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::EntityId;

// ---------------------------------------------------------------------------
// Policy
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: EntityId,
    pub name: String,
    pub policy_type: PolicyType,
    pub rules: Vec<PolicyRule>,
    pub active: bool,
    pub priority: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyType {
    Routing,
    Safety,
    Privacy,
    DataRetention,
    FlowControl,
    ResidentialProtection,
    Emergency,
    Integrity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    pub condition: String,
    pub action: String,
    pub parameters: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Alert rules
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub id: EntityId,
    pub name: String,
    pub condition: AlertCondition,
    pub severity: AlertSeverity,
    pub action: AlertAction,
    pub active: bool,
    pub cooldown_s: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertCondition {
    pub metric: String,
    pub operator: ComparisonOperator,
    pub threshold: f64,
    pub window_s: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonOperator {
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
    Equal,
    NotEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertAction {
    Notify,
    Log,
    Reroute,
    Degrade,
    EmergencyStop,
    ExcludeSource,
}

// ---------------------------------------------------------------------------
// Model artifact (ML model registry)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelArtifact {
    pub id: EntityId,
    pub name: String,
    pub version: ModelVersion,
    pub model_type: ModelType,
    pub artifact_path: String,
    pub metrics: serde_json::Value,
    pub deployed: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub hash: String,
}

impl std::fmt::Display for ModelVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}-{}",
            self.major,
            self.minor,
            self.patch,
            &self.hash[..8.min(self.hash.len())]
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelType {
    TrafficPrediction,
    RiskScoring,
    DriverBehavior,
    AnomalyDetection,
    EtaPrediction,
    ParkingPrediction,
    EnergyPrediction,
    SpoofDetection,
}
