//! Constellation manager — orchestrates independent acquisition, joint solution,
//! selective exclusion, and continuous re-entry across all constellations.

use aurora_core::gnss::{
    Constellation, GnssMeasurement, PvtSolution,
    SatelliteMeasurement,
};
use std::collections::HashMap;
use tracing::{info, warn};

use crate::pvt::{PvtError, PvtSolver};
use crate::quality::{QualityScorer, SatelliteQuality};
use crate::receiver::GnssReceiver;

/// Orchestrates the full GNSS pipeline per the spec's operating rules:
/// Independent Acquisition + Joint Solution + Selective Exclusion + Continuous Re-Entry.
pub struct ConstellationManager {
    receiver: GnssReceiver,
    quality_scorer: QualityScorer,
    pvt_solver: PvtSolver,
    /// Last per-constellation PVT solutions.
    per_constellation_pvt: HashMap<Constellation, PvtSolution>,
    /// Last combined PVT solution.
    combined_pvt: Option<PvtSolution>,
}

impl ConstellationManager {
    pub fn new() -> Self {
        Self {
            receiver: GnssReceiver::new(),
            quality_scorer: QualityScorer::new(),
            pvt_solver: PvtSolver::new(),
            per_constellation_pvt: HashMap::new(),
            combined_pvt: None,
        }
    }

    /// Process a new measurement epoch through the full pipeline.
    ///
    /// Returns the combined PVT solution if available, or the best single-constellation solution.
    pub fn process_epoch(&mut self, epoch: &GnssMeasurement) -> Result<PvtSolution, PvtError> {
        // Step 1: Ingest — independent acquisition with quality filtering.
        let accepted = self.receiver.ingest(epoch);

        if accepted.is_empty() {
            return Err(PvtError::InsufficientSatellites(0));
        }

        // Step 2: Quality scoring for each accepted measurement.
        let qualities: Vec<SatelliteQuality> = self.quality_scorer.score_batch(&accepted);
        let usable: Vec<&SatelliteMeasurement> = accepted
            .iter()
            .zip(qualities.iter())
            .filter(|(_, q)| q.usable)
            .map(|(m, _)| m)
            .collect();

        if usable.len() < 4 {
            return Err(PvtError::InsufficientSatellites(usable.len()));
        }

        // Step 3: Per-constellation independent PVT.
        let by_constellation = self.receiver.measurements_by_constellation();
        for (constellation, meas) in &by_constellation {
            if meas.len() >= 4 {
                match self
                    .pvt_solver
                    .solve_single_constellation(meas, *constellation)
                {
                    Ok(pvt) => {
                        info!(constellation = %constellation, sats = meas.len(), "per-constellation PVT computed");
                        self.per_constellation_pvt.insert(*constellation, pvt);
                    }
                    Err(e) => {
                        warn!(constellation = %constellation, error = %e, "per-constellation PVT failed");
                    }
                }
            }
        }

        // Step 4: Combined multi-constellation PVT.
        match self.pvt_solver.solve_combined(&usable) {
            Ok(pvt) => {
                info!(sats = usable.len(), "combined PVT computed");
                self.combined_pvt = Some(pvt.clone());
                Ok(pvt)
            }
            Err(e) => {
                // Fallback to best single-constellation solution.
                warn!(error = %e, "combined PVT failed — trying single-constellation fallback");
                self.best_single_constellation_pvt()
                    .ok_or(PvtError::InsufficientSatellites(usable.len()))
            }
        }
    }

    /// Get the last combined PVT solution.
    pub fn last_combined_pvt(&self) -> Option<&PvtSolution> {
        self.combined_pvt.as_ref()
    }

    /// Get per-constellation solutions.
    pub fn per_constellation_solutions(&self) -> &HashMap<Constellation, PvtSolution> {
        &self.per_constellation_pvt
    }

    /// Access the underlying receiver for exclusion/reinstatement.
    pub fn receiver_mut(&mut self) -> &mut GnssReceiver {
        &mut self.receiver
    }

    /// Access the underlying receiver for read.
    pub fn receiver(&self) -> &GnssReceiver {
        &self.receiver
    }

    fn best_single_constellation_pvt(&self) -> Option<PvtSolution> {
        self.per_constellation_pvt
            .values()
            .min_by(|a, b| a.pdop.partial_cmp(&b.pdop).unwrap_or(std::cmp::Ordering::Equal))
            .cloned()
    }
}

impl Default for ConstellationManager {
    fn default() -> Self {
        Self::new()
    }
}
