//! GNSS receiver abstraction — ingests raw measurements from all constellations.

use aurora_core::gnss::{
    Constellation, ConstellationHealth, ConstellationState, GnssMeasurement, SatelliteId,
    SatelliteMeasurement,
};
use chrono::Utc;
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// Minimum CN0 (dB-Hz) to consider a satellite signal usable.
const MIN_CN0_DBHZ: f64 = 20.0;

/// Minimum elevation angle (degrees) to use a satellite.
const MIN_ELEVATION_DEG: f64 = 5.0;

/// GNSS receiver that manages raw measurement ingestion and per-satellite quality.
pub struct GnssReceiver {
    /// Latest measurements indexed by satellite.
    measurements: HashMap<SatelliteId, SatelliteMeasurement>,
    /// Per-constellation state.
    constellation_states: HashMap<Constellation, ConstellationState>,
    /// Satellites currently excluded (manual or automatic).
    excluded_satellites: HashMap<SatelliteId, String>,
    /// Excluded constellations.
    excluded_constellations: HashMap<Constellation, String>,
}

impl GnssReceiver {
    pub fn new() -> Self {
        Self {
            measurements: HashMap::new(),
            constellation_states: HashMap::new(),
            excluded_satellites: HashMap::new(),
            excluded_constellations: HashMap::new(),
        }
    }

    /// Ingest a complete GNSS measurement epoch.
    /// Filters out unhealthy / low-quality satellites and updates constellation states.
    pub fn ingest(&mut self, epoch: &GnssMeasurement) -> Vec<SatelliteMeasurement> {
        let mut accepted = Vec::new();

        for meas in &epoch.measurements {
            // Rule: unhealthy satellites are never used.
            if !meas.healthy {
                debug!(sat = %meas.satellite, "satellite flagged unhealthy — skipping");
                continue;
            }

            // Rule: below minimum CN0 — skip.
            if meas.cn0_dbhz < MIN_CN0_DBHZ {
                debug!(sat = %meas.satellite, cn0 = meas.cn0_dbhz, "CN0 below threshold");
                continue;
            }

            // Rule: below minimum elevation — skip.
            if let Some(elev) = meas.elevation_deg {
                if elev < MIN_ELEVATION_DEG {
                    debug!(sat = %meas.satellite, elev, "elevation below threshold");
                    continue;
                }
            }

            // Check satellite exclusion list.
            if self.excluded_satellites.contains_key(&meas.satellite) {
                debug!(sat = %meas.satellite, "satellite is excluded");
                continue;
            }

            // Check constellation exclusion.
            if self
                .excluded_constellations
                .contains_key(&meas.satellite.constellation)
            {
                debug!(constellation = %meas.satellite.constellation, "constellation excluded");
                continue;
            }

            self.measurements.insert(meas.satellite, meas.clone());
            accepted.push(meas.clone());
        }

        // Refresh constellation states.
        self.update_constellation_states();

        info!(
            total = epoch.measurements.len(),
            accepted = accepted.len(),
            "GNSS epoch ingested"
        );

        accepted
    }

    /// Get all currently valid measurements grouped by constellation.
    pub fn measurements_by_constellation(
        &self,
    ) -> HashMap<Constellation, Vec<&SatelliteMeasurement>> {
        let mut groups: HashMap<Constellation, Vec<&SatelliteMeasurement>> = HashMap::new();
        for meas in self.measurements.values() {
            groups
                .entry(meas.satellite.constellation)
                .or_default()
                .push(meas);
        }
        groups
    }

    /// Get all accepted measurements.
    pub fn all_measurements(&self) -> Vec<&SatelliteMeasurement> {
        self.measurements.values().collect()
    }

    /// Exclude a specific satellite.
    pub fn exclude_satellite(&mut self, sat: SatelliteId, reason: String) {
        warn!(sat = %sat, reason = %reason, "excluding satellite");
        self.excluded_satellites.insert(sat, reason);
        self.measurements.remove(&sat);
    }

    /// Reinstate a previously excluded satellite.
    pub fn reinstate_satellite(&mut self, sat: &SatelliteId) {
        if self.excluded_satellites.remove(sat).is_some() {
            info!(sat = %sat, "satellite reinstated — pending re-validation");
        }
    }

    /// Exclude an entire constellation.
    pub fn exclude_constellation(&mut self, constellation: Constellation, reason: String) {
        warn!(constellation = %constellation, reason = %reason, "excluding constellation");
        self.excluded_constellations
            .insert(constellation, reason);
        self.measurements
            .retain(|k, _| k.constellation != constellation);
    }

    /// Reinstate a previously excluded constellation.
    pub fn reinstate_constellation(&mut self, constellation: &Constellation) {
        if self.excluded_constellations.remove(constellation).is_some() {
            info!(constellation = %constellation, "constellation reinstated");
        }
    }

    /// Current constellation states.
    pub fn constellation_states(&self) -> &HashMap<Constellation, ConstellationState> {
        &self.constellation_states
    }

    /// Number of satellites currently tracked.
    pub fn tracked_count(&self) -> usize {
        self.measurements.len()
    }

    // -- internal --------------------------------------------------------

    fn update_constellation_states(&mut self) {
        for constellation in &[
            Constellation::Gps,
            Constellation::Galileo,
            Constellation::Glonass,
            Constellation::BeiDou,
        ] {
            let sats: Vec<&SatelliteMeasurement> = self
                .measurements
                .values()
                .filter(|m| m.satellite.constellation == *constellation)
                .collect();

            let visible = sats.len() as u32;
            let avg_cn0 = if sats.is_empty() {
                0.0
            } else {
                sats.iter().map(|s| s.cn0_dbhz).sum::<f64>() / sats.len() as f64
            };

            let health = if self.excluded_constellations.contains_key(constellation)
                || visible == 0
            {
                ConstellationHealth::Unavailable
            } else if visible < 4 {
                ConstellationHealth::Degraded
            } else {
                ConstellationHealth::Nominal
            };

            self.constellation_states.insert(
                *constellation,
                ConstellationState {
                    constellation: *constellation,
                    health,
                    visible_count: visible,
                    used_count: visible,
                    avg_cn0_dbhz: avg_cn0,
                    pdop: None,
                    hdop: None,
                    vdop: None,
                    updated_at: Utc::now(),
                },
            );
        }
    }
}

impl Default for GnssReceiver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aurora_core::gnss::*;
    use aurora_core::types::EntityId;
    use chrono::Utc;

    fn make_measurement(prn: u8, constellation: Constellation, cn0: f64) -> SatelliteMeasurement {
        SatelliteMeasurement {
            id: EntityId::new(),
            satellite: SatelliteId { constellation, prn },
            signal: SignalType::GpsL1CA,
            timestamp: Utc::now(),
            pseudorange_m: 20_000_000.0 + prn as f64 * 1000.0,
            carrier_phase_cycles: None,
            doppler_hz: Some(-1500.0),
            cn0_dbhz: cn0,
            healthy: true,
            satellite_position: None,
            elevation_deg: Some(45.0),
            azimuth_deg: Some(180.0),
        }
    }

    fn make_epoch(measurements: Vec<SatelliteMeasurement>) -> GnssMeasurement {
        GnssMeasurement {
            id: EntityId::new(),
            timestamp: Utc::now(),
            measurements,
            receiver_clock_bias_ns: None,
            receiver_clock_drift_nps: None,
        }
    }

    #[test]
    fn low_cn0_satellites_are_filtered() {
        let mut rx = GnssReceiver::new();
        let epoch = make_epoch(vec![
            make_measurement(1, Constellation::Gps, 35.0),
            make_measurement(2, Constellation::Gps, 15.0), // below threshold
        ]);

        let accepted = rx.ingest(&epoch);
        assert_eq!(accepted.len(), 1);
        assert_eq!(rx.tracked_count(), 1);
    }

    #[test]
    fn exclude_satellite_removes_it() {
        let mut rx = GnssReceiver::new();
        let epoch = make_epoch(vec![
            make_measurement(1, Constellation::Gps, 40.0),
            make_measurement(2, Constellation::Gps, 40.0),
        ]);
        rx.ingest(&epoch);
        assert_eq!(rx.tracked_count(), 2);

        let sat = SatelliteId {
            constellation: Constellation::Gps,
            prn: 1,
        };
        rx.exclude_satellite(sat, "test exclusion".into());
        assert_eq!(rx.tracked_count(), 1);
    }

    #[test]
    fn constellation_exclusion_filters_all() {
        let mut rx = GnssReceiver::new();
        let epoch = make_epoch(vec![
            make_measurement(1, Constellation::Gps, 40.0),
            make_measurement(2, Constellation::Gps, 40.0),
            make_measurement(1, Constellation::Galileo, 40.0),
        ]);
        rx.ingest(&epoch);
        assert_eq!(rx.tracked_count(), 3);

        rx.exclude_constellation(Constellation::Gps, "spoofing suspected".into());
        assert_eq!(rx.tracked_count(), 1);

        // New epoch — GPS should still be filtered.
        let epoch2 = make_epoch(vec![
            make_measurement(5, Constellation::Gps, 45.0),
            make_measurement(3, Constellation::Galileo, 42.0),
        ]);
        let accepted = rx.ingest(&epoch2);
        assert_eq!(accepted.len(), 1);
    }
}
