//! Uncertainty modeling — covariance matrices, propagation,
//! precision vs confidence separation.

use serde::{Deserialize, Serialize};

/// 2D covariance matrix for position uncertainty.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Covariance2D {
    pub xx: f64,
    pub xy: f64,
    pub yy: f64,
}

impl Covariance2D {
    pub fn new(xx: f64, yy: f64) -> Self {
        Self { xx, xy: 0.0, yy }
    }
    pub fn with_correlation(xx: f64, xy: f64, yy: f64) -> Self {
        Self { xx, xy, yy }
    }

    /// Determinant of the covariance matrix.
    pub fn determinant(&self) -> f64 {
        self.xx * self.yy - self.xy * self.xy
    }

    /// 95% confidence ellipse semi-axes.
    pub fn confidence_ellipse_95(&self) -> (f64, f64) {
        let scale = 5.991; // chi-squared 2 DOF, 95%
        let trace = self.xx + self.yy;
        let det = self.determinant();
        let disc = ((trace * trace / 4.0) - det).max(0.0).sqrt();
        let l1 = (trace / 2.0 + disc).max(0.0);
        let l2 = (trace / 2.0 - disc).max(0.0);
        ((l1 * scale).sqrt(), (l2 * scale).sqrt())
    }

    /// Propagate uncertainty forward in time given velocity uncertainty.
    pub fn propagate(&self, dt: f64, vel_var: f64) -> Self {
        Self {
            xx: self.xx + vel_var * dt * dt,
            xy: self.xy,
            yy: self.yy + vel_var * dt * dt,
        }
    }

    /// Merge two independent uncertainty estimates.
    pub fn fuse(&self, other: &Self) -> Self {
        let denom_xx = self.xx + other.xx;
        let denom_yy = self.yy + other.yy;
        if denom_xx < 1e-15 || denom_yy < 1e-15 {
            return *self;
        }
        Self {
            xx: (self.xx * other.xx) / denom_xx,
            xy: (self.xy * other.xy) / (self.xy.abs() + other.xy.abs()).max(1e-15),
            yy: (self.yy * other.yy) / denom_yy,
        }
    }

    /// Check if uncertainty is below acceptable threshold.
    pub fn is_acceptable(&self, max_std_m: f64) -> bool {
        self.xx.sqrt() <= max_std_m && self.yy.sqrt() <= max_std_m
    }
}

impl Default for Covariance2D {
    fn default() -> Self {
        Self::new(25.0, 25.0)
    } // 5m std dev
}

/// Full uncertainty state for a navigation fix.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncertaintyState {
    pub position: Covariance2D,
    pub velocity_var: f64,
    pub heading_var_rad: f64,
    pub altitude_var: f64,
    pub confidence: f64,
    pub precision_class: PrecisionClass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrecisionClass {
    CmLevel,
    SubMeter,
    MeterLevel,
    Degraded,
    Unknown,
}

impl UncertaintyState {
    pub fn new(position: Covariance2D) -> Self {
        let pc = if position.xx.sqrt() <= 0.1 {
            PrecisionClass::CmLevel
        } else if position.xx.sqrt() < 1.0 {
            PrecisionClass::SubMeter
        } else if position.xx.sqrt() < 10.0 {
            PrecisionClass::MeterLevel
        } else {
            PrecisionClass::Degraded
        };
        Self {
            position,
            velocity_var: 1.0,
            heading_var_rad: 0.1,
            altitude_var: 4.0,
            confidence: 0.8,
            precision_class: pc,
        }
    }

    pub fn propagate(&mut self, dt: f64) {
        self.position = self.position.propagate(dt, self.velocity_var);
        self.confidence = (self.confidence - 0.01 * dt).max(0.0);
    }

    pub fn update_precision_class(&mut self) {
        let std = self.position.xx.sqrt();
        self.precision_class = if std < 0.1 {
            PrecisionClass::CmLevel
        } else if std < 1.0 {
            PrecisionClass::SubMeter
        } else if std < 10.0 {
            PrecisionClass::MeterLevel
        } else {
            PrecisionClass::Degraded
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cov_new() {
        let c = Covariance2D::new(4.0, 9.0);
        assert_eq!(c.xx, 4.0);
    }

    #[test]
    fn test_determinant() {
        let c = Covariance2D::new(4.0, 9.0);
        assert!((c.determinant() - 36.0).abs() < 1e-10);
    }

    #[test]
    fn test_confidence_ellipse() {
        let c = Covariance2D::new(4.0, 4.0);
        let (a, b) = c.confidence_ellipse_95();
        assert!(a > 0.0 && b > 0.0);
        assert!((a - b).abs() < 1e-10); // symmetric
    }

    #[test]
    fn test_propagate_grows() {
        let c = Covariance2D::new(1.0, 1.0);
        let c2 = c.propagate(1.0, 0.5);
        assert!(c2.xx > c.xx);
    }

    #[test]
    fn test_fuse_reduces() {
        let a = Covariance2D::new(4.0, 4.0);
        let b = Covariance2D::new(4.0, 4.0);
        let f = a.fuse(&b);
        assert!(f.xx < a.xx);
    }

    #[test]
    fn test_acceptable() {
        let c = Covariance2D::new(1.0, 1.0);
        assert!(c.is_acceptable(2.0));
        assert!(!c.is_acceptable(0.5));
    }

    #[test]
    fn test_uncertainty_state() {
        let s = UncertaintyState::new(Covariance2D::new(0.01, 0.01));
        assert_eq!(s.precision_class, PrecisionClass::CmLevel);
    }

    #[test]
    fn test_propagate_degrades() {
        let mut s = UncertaintyState::new(Covariance2D::new(0.01, 0.01));
        s.propagate(10.0);
        s.update_precision_class();
        assert_ne!(s.precision_class, PrecisionClass::CmLevel);
    }
}
