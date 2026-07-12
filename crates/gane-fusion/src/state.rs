//! Fusion state vector and covariance management.

use chrono::{DateTime, Utc};
use gane_core::types::{
    ContinuityMode, CovarianceMatrix, EnuVelocity, FusedPosition, GeoPosition, Heading,
    IntegrityLevel, SourceContribution, UncertaintyEllipse,
};
use serde::{Deserialize, Serialize};

/// The navigation state maintained by the fusion filter.
///
/// State vector: [lat, lon, alt, ve, vn, vu, heading, clock_bias]
/// (simplified — full implementation would use ECEF + quaternion).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusionState {
    /// Current estimated position.
    pub position: GeoPosition,
    /// Current estimated velocity (ENU).
    pub velocity: EnuVelocity,
    /// Current estimated heading.
    pub heading: Heading,
    /// Position covariance (ENU, 3×3).
    pub position_covariance: CovarianceMatrix,
    /// Velocity covariance (ENU, 3×3).
    pub velocity_covariance: CovarianceMatrix,
    /// Receiver clock bias estimate (ns).
    pub clock_bias_ns: f64,
    /// Last update time.
    pub timestamp: DateTime<Utc>,
    /// Active continuity mode.
    pub continuity_mode: ContinuityMode,
    /// Integrity assessment.
    pub integrity: IntegrityLevel,
    /// Confidence score [0, 1].
    pub confidence: f64,
    /// Per-source contributions and weights.
    pub source_weights: Vec<SourceContribution>,
    /// Whether the filter has been initialised.
    pub initialised: bool,
}

impl FusionState {
    /// Create an uninitialised state.
    pub fn new() -> Self {
        Self {
            position: GeoPosition {
                latitude_deg: 0.0,
                longitude_deg: 0.0,
                altitude_m: Some(0.0),
            },
            velocity: EnuVelocity {
                east_mps: 0.0,
                north_mps: 0.0,
                up_mps: 0.0,
            },
            heading: Heading {
                true_heading_deg: 0.0,
                magnetic_heading_deg: None,
                uncertainty_deg: 360.0,
            },
            position_covariance: CovarianceMatrix::identity_scaled(1e6),
            velocity_covariance: CovarianceMatrix::identity_scaled(100.0),
            clock_bias_ns: 0.0,
            timestamp: Utc::now(),
            continuity_mode: ContinuityMode::ModeA,
            integrity: IntegrityLevel::NoSolution,
            confidence: 0.0,
            source_weights: Vec::new(),
            initialised: false,
        }
    }

    /// Initialise the state from a GNSS fix.
    pub fn initialise_from_gnss(
        &mut self,
        position: GeoPosition,
        velocity: EnuVelocity,
        heading_deg: f64,
        timestamp: DateTime<Utc>,
        accuracy_m: f64,
    ) {
        self.position = position;
        self.velocity = velocity;
        self.heading.true_heading_deg = heading_deg;
        self.heading.uncertainty_deg = 15.0;
        self.position_covariance = CovarianceMatrix::identity_scaled(accuracy_m * accuracy_m);
        self.velocity_covariance = CovarianceMatrix::identity_scaled(1.0);
        self.timestamp = timestamp;
        self.integrity = IntegrityLevel::Nominal;
        self.confidence = 0.8;
        self.initialised = true;
    }

    /// Convert to the public FusedPosition output.
    pub fn to_fused_position(&self) -> FusedPosition {
        let cov = self.position_covariance.to_nalgebra();
        let semi_major = cov[(0, 0)].sqrt().max(cov[(1, 1)].sqrt());
        let semi_minor = cov[(0, 0)].sqrt().min(cov[(1, 1)].sqrt());
        let semi_vertical = cov[(2, 2)].sqrt();

        FusedPosition {
            timestamp: self.timestamp,
            position: self.position,
            ecef: None,
            velocity: self.velocity,
            heading: self.heading,
            uncertainty: UncertaintyEllipse {
                semi_major_m: semi_major,
                semi_minor_m: semi_minor,
                semi_vertical_m: semi_vertical,
                orientation_deg: 0.0,
            },
            covariance: self.position_covariance.clone(),
            confidence: self.confidence,
            integrity_state: self.integrity,
            continuity_mode: self.continuity_mode,
            source_contributions: self.source_weights.clone(),
        }
    }

    /// Horizontal accuracy (1-σ) in metres.
    pub fn horizontal_accuracy_m(&self) -> f64 {
        let cov = self.position_covariance.to_nalgebra();
        (cov[(0, 0)] + cov[(1, 1)]).sqrt()
    }

    /// Vertical accuracy (1-σ) in metres.
    pub fn vertical_accuracy_m(&self) -> f64 {
        let cov = self.position_covariance.to_nalgebra();
        cov[(2, 2)].sqrt()
    }
}

impl Default for FusionState {
    fn default() -> Self {
        Self::new()
    }
}
