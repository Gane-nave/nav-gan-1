//! Scoring domain types — risk, trust, stability, confidence.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::EntityId;

// ---------------------------------------------------------------------------
// Risk scoring
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskScore {
    pub id: EntityId,
    pub entity_type: RiskEntityType,
    pub entity_id: EntityId,
    /// Overall risk score [0.0, 1.0] where 1.0 = extreme risk.
    pub score: f64,
    pub components: Vec<RiskComponent>,
    pub computed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskEntityType {
    Segment,
    Route,
    Intersection,
    Region,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskComponent {
    pub factor: RiskFactor,
    pub score: f64,
    pub weight: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskFactor {
    AccidentHistory,
    HardBrakeDensity,
    Weather,
    Visibility,
    Lighting,
    InfrastructureCondition,
    HumanErrorLikelihood,
    Congestion,
    SpeedVariance,
    CurvatureComplexity,
    IntersectionComplexity,
}

// ---------------------------------------------------------------------------
// Trust scoring
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustScore {
    pub id: EntityId,
    pub entity_type: TrustEntityType,
    pub entity_id: EntityId,
    /// Trust score [0.0, 1.0] where 1.0 = fully trusted.
    pub score: f64,
    pub validation_count: u32,
    pub false_report_count: u32,
    pub computed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrustEntityType {
    User,
    NavigationSource,
    Satellite,
    Incident,
    Evidence,
    MapData,
}

// ---------------------------------------------------------------------------
// Stability scoring
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StabilityScore {
    pub id: EntityId,
    pub entity_type: StabilityEntityType,
    pub entity_id: EntityId,
    /// Network stability index [0.0, 1.0] where 1.0 = perfectly stable.
    pub score: f64,
    pub congestion_entropy: f64,
    pub computed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StabilityEntityType {
    Segment,
    Corridor,
    Region,
    Network,
}

// ---------------------------------------------------------------------------
// Position confidence
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionConfidence {
    pub timestamp: DateTime<Utc>,
    /// Horizontal confidence [0.0, 1.0].
    pub horizontal: f64,
    /// Vertical confidence [0.0, 1.0].
    pub vertical: f64,
    /// Heading confidence [0.0, 1.0].
    pub heading: f64,
    /// Velocity confidence [0.0, 1.0].
    pub velocity: f64,
    /// Overall position confidence [0.0, 1.0].
    pub overall: f64,
}
