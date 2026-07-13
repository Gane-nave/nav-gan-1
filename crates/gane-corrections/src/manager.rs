//! Correction manager — selects the best correction source and manages fallback.

use gane_core::gnss::{CorrectionSource, CorrectionState};
use gane_core::types::ContinuityMode;
use tracing::{info, warn};

use crate::source::{CachedCorrectionProvider, CorrectionData, CorrectionProvider};

/// Manages all correction sources and determines the current correction mode.
pub struct CorrectionManager {
    providers: Vec<Box<dyn CorrectionProvider>>,
    cache: CachedCorrectionProvider,
    active_source: Option<CorrectionSource>,
}

impl CorrectionManager {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
            cache: CachedCorrectionProvider::new(300.0), // 5 min cache
            active_source: None,
        }
    }

    /// Register a correction provider.
    pub fn add_provider(&mut self, provider: Box<dyn CorrectionProvider>) {
        info!(source = ?provider.source_type(), "correction provider registered");
        self.providers.push(provider);
    }

    /// Get the best available corrections, following the priority cascade.
    ///
    /// Priority: RTK > NRTK > PPP > SBAS > Internet > Cached Fallback.
    pub fn best_corrections(&mut self) -> (Vec<CorrectionData>, CorrectionSource) {
        let priority_order = [
            CorrectionSource::Rtk,
            CorrectionSource::Nrtk,
            CorrectionSource::Ppp,
            CorrectionSource::Sbas,
            CorrectionSource::InternetCorrection,
        ];

        for source_type in &priority_order {
            for provider in &self.providers {
                if provider.source_type() == *source_type && provider.is_active() {
                    let corrections = provider.latest_corrections();
                    if !corrections.is_empty() {
                        // Update cache with these good corrections.
                        self.cache.update_cache(corrections.clone());
                        self.active_source = Some(*source_type);
                        return (corrections, *source_type);
                    }
                }
            }
        }

        // Fallback to cached corrections.
        if self.cache.has_valid_cache() {
            warn!("using cached fallback corrections");
            self.active_source = Some(CorrectionSource::CachedFallback);
            return (
                self.cache.latest_corrections(),
                CorrectionSource::CachedFallback,
            );
        }

        // No corrections available.
        self.active_source = None;
        (Vec::new(), CorrectionSource::CachedFallback)
    }

    /// Determine which continuity mode the correction layer supports.
    pub fn correction_mode(&self) -> ContinuityMode {
        match self.active_source {
            Some(CorrectionSource::Rtk)
            | Some(CorrectionSource::Nrtk)
            | Some(CorrectionSource::Ppp)
            | Some(CorrectionSource::Sbas) => ContinuityMode::ModeA,
            Some(CorrectionSource::InternetCorrection) => ContinuityMode::ModeA,
            Some(CorrectionSource::CachedFallback) => ContinuityMode::ModeB,
            None => ContinuityMode::ModeB,
        }
    }

    /// Get all provider states for diagnostics.
    pub fn provider_states(&self) -> Vec<CorrectionState> {
        let mut states: Vec<CorrectionState> = self.providers.iter().map(|p| p.state()).collect();
        states.push(self.cache.state());
        states
    }

    /// Currently active correction source.
    pub fn active_source(&self) -> Option<CorrectionSource> {
        self.active_source
    }
}

impl Default for CorrectionManager {
    fn default() -> Self {
        Self::new()
    }
}
