//! Correction source abstraction.

use chrono::{DateTime, Utc};
use gane_core::gnss::{CorrectionSource, CorrectionState};
use serde::{Deserialize, Serialize};

/// Correction data for a specific satellite or regional area.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrectionData {
    pub source: CorrectionSource,
    pub timestamp: DateTime<Utc>,
    /// Clock correction in metres.
    pub clock_correction_m: f64,
    /// Orbit correction in metres (radial, along-track, cross-track).
    pub orbit_correction_m: [f64; 3],
    /// Ionospheric delay correction in metres (if available).
    pub ionospheric_correction_m: Option<f64>,
    /// Tropospheric delay correction in metres (if available).
    pub tropospheric_correction_m: Option<f64>,
    /// Confidence in the correction [0.0, 1.0].
    pub confidence: f64,
    /// How old this correction is in seconds.
    pub age_s: f64,
    /// Maximum validity window in seconds.
    pub validity_window_s: f64,
}

impl CorrectionData {
    /// Whether the correction is still valid based on its age and window.
    pub fn is_valid(&self) -> bool {
        self.age_s <= self.validity_window_s && self.confidence > 0.0
    }

    /// Whether the correction is getting stale (> 80% of validity window).
    pub fn is_stale(&self) -> bool {
        self.age_s > self.validity_window_s * 0.8
    }
}

/// Trait for correction data providers (SBAS, PPP, RTK, NRTK, Internet, Cache).
pub trait CorrectionProvider: Send + Sync {
    /// The correction source type.
    fn source_type(&self) -> CorrectionSource;

    /// Current connection status.
    fn state(&self) -> CorrectionState;

    /// Whether this provider is currently connected and delivering corrections.
    fn is_active(&self) -> bool;

    /// Fetch the latest correction data.
    fn latest_corrections(&self) -> Vec<CorrectionData>;

    /// Age of the most recent correction in seconds.
    fn correction_age_s(&self) -> f64;
}

/// A cached/fallback correction provider that stores the last known good corrections.
pub struct CachedCorrectionProvider {
    cached: Vec<CorrectionData>,
    cached_at: DateTime<Utc>,
    max_cache_age_s: f64,
}

impl CachedCorrectionProvider {
    pub fn new(max_cache_age_s: f64) -> Self {
        Self {
            cached: Vec::new(),
            cached_at: Utc::now(),
            max_cache_age_s,
        }
    }

    /// Update the cache with fresh corrections from another provider.
    pub fn update_cache(&mut self, corrections: Vec<CorrectionData>) {
        self.cached = corrections;
        self.cached_at = Utc::now();
    }

    /// Whether the cache contains usable data.
    pub fn has_valid_cache(&self) -> bool {
        let age = (Utc::now() - self.cached_at).num_seconds() as f64;
        !self.cached.is_empty() && age <= self.max_cache_age_s
    }
}

impl CorrectionProvider for CachedCorrectionProvider {
    fn source_type(&self) -> CorrectionSource {
        CorrectionSource::CachedFallback
    }

    fn state(&self) -> CorrectionState {
        let age = (Utc::now() - self.cached_at).num_seconds() as f64;
        CorrectionState {
            source: CorrectionSource::CachedFallback,
            active: self.has_valid_cache(),
            age_seconds: age,
            confidence: if self.has_valid_cache() {
                (1.0 - age / self.max_cache_age_s).max(0.0)
            } else {
                0.0
            },
            validity_window_s: self.max_cache_age_s,
            updated_at: self.cached_at,
        }
    }

    fn is_active(&self) -> bool {
        self.has_valid_cache()
    }

    fn latest_corrections(&self) -> Vec<CorrectionData> {
        if self.has_valid_cache() {
            self.cached.clone()
        } else {
            Vec::new()
        }
    }

    fn correction_age_s(&self) -> f64 {
        (Utc::now() - self.cached_at).num_seconds() as f64
    }
}
