use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Constellation {
    Gps,
    Glonass,
    BeiDou,
    Galileo,
    Qzss,
    Sbas,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SatelliteInfo {
    pub prn: u16,
    pub constellation: Constellation,
    pub elevation_deg: f64,
    pub azimuth_deg: f64,
    pub snr_dbhz: f64,
    pub used_in_fix: bool,
    pub frequency: FrequencyBand,
    pub health: SatHealth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrequencyBand {
    L1,
    L2,
    L5,
    E1,
    E5a,
    E5b,
    B1,
    B2,
    B3,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SatHealth {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GnssFix {
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub altitude_m: f64,
    pub hdop: f64,
    pub vdop: f64,
    pub pdop: f64,
    pub accuracy_m: f64,
    pub fix_type: FixType,
    pub satellites_used: u32,
    pub constellations_used: Vec<Constellation>,
    pub timestamp_ms: u64,
    pub quality_score: f64,
    pub dual_frequency: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FixType {
    NoFix,
    Fix2D,
    Fix3D,
    DgnSS,
    Rtk,
    RtkFloat,
    Ppp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstellationStatus {
    pub constellation: Constellation,
    pub tracked: u32,
    pub used: u32,
    pub avg_snr: f64,
    pub quality_score: f64,
    pub available: bool,
}

pub struct MultiGnssEngine {
    satellites: Vec<SatelliteInfo>,
    last_fix: Option<GnssFix>,
    constellation_weights: Vec<(Constellation, f64)>,
    dual_freq_enabled: bool,
    min_elevation_deg: f64,
    min_snr_dbhz: f64,
    fixes_computed: u64,
    agnss_enabled: bool,
}

impl MultiGnssEngine {
    pub fn new() -> Self {
        Self {
            satellites: Vec::new(),
            last_fix: None,
            constellation_weights: vec![
                (Constellation::Gps, 1.0),
                (Constellation::Glonass, 0.95),
                (Constellation::BeiDou, 0.95),
                (Constellation::Galileo, 1.0),
                (Constellation::Qzss, 0.9),
                (Constellation::Sbas, 0.85),
            ],
            dual_freq_enabled: false,
            min_elevation_deg: 10.0,
            min_snr_dbhz: 20.0,
            fixes_computed: 0,
            agnss_enabled: false,
        }
    }
    pub fn enable_dual_frequency(&mut self, e: bool) {
        self.dual_freq_enabled = e;
    }
    pub fn enable_agnss(&mut self, e: bool) {
        self.agnss_enabled = e;
    }
    pub fn is_dual_frequency(&self) -> bool {
        self.dual_freq_enabled
    }
    pub fn is_agnss_enabled(&self) -> bool {
        self.agnss_enabled
    }
    pub fn update_satellites(&mut self, sats: Vec<SatelliteInfo>) {
        self.satellites = sats;
    }
    pub fn tracked_count(&self) -> usize {
        self.satellites.len()
    }
    pub fn tracked_by_constellation(&self, c: Constellation) -> usize {
        self.satellites
            .iter()
            .filter(|s| s.constellation == c)
            .count()
    }
    pub fn usable_satellites(&self) -> Vec<&SatelliteInfo> {
        self.satellites
            .iter()
            .filter(|s| {
                s.elevation_deg >= self.min_elevation_deg
                    && s.snr_dbhz >= self.min_snr_dbhz
                    && s.health != SatHealth::Unhealthy
            })
            .collect()
    }
    pub fn constellation_status(&self) -> Vec<ConstellationStatus> {
        [
            Constellation::Gps,
            Constellation::Glonass,
            Constellation::BeiDou,
            Constellation::Galileo,
            Constellation::Qzss,
            Constellation::Sbas,
        ]
        .iter()
        .map(|&c| {
            let sats: Vec<_> = self
                .satellites
                .iter()
                .filter(|s| s.constellation == c)
                .collect();
            let used = sats.iter().filter(|s| s.used_in_fix).count() as u32;
            let avg_snr = if sats.is_empty() {
                0.0
            } else {
                sats.iter().map(|s| s.snr_dbhz).sum::<f64>() / sats.len() as f64
            };
            let quality = (avg_snr / 50.0).min(1.0) * if used == 0 { 0.5 } else { 1.0 };
            ConstellationStatus {
                constellation: c,
                tracked: sats.len() as u32,
                used,
                avg_snr,
                quality_score: quality,
                available: !sats.is_empty(),
            }
        })
        .collect()
    }
    pub fn compute_fix(&mut self) -> Option<GnssFix> {
        let min_elev = self.min_elevation_deg;
        let min_snr = self.min_snr_dbhz;
        let usable_indices: Vec<usize> = self
            .satellites
            .iter()
            .enumerate()
            .filter(|(_, s)| {
                s.elevation_deg >= min_elev
                    && s.snr_dbhz >= min_snr
                    && s.health != SatHealth::Unhealthy
            })
            .map(|(i, _)| i)
            .collect();
        if usable_indices.len() < 4 {
            return None;
        }
        let mut total_lat = 0.0;
        let mut total_lon = 0.0;
        let mut total_w = 0.0;
        let mut used_c: Vec<Constellation> = Vec::new();
        let mut snr_sum = 0.0;
        for &idx in &usable_indices {
            let sat = &self.satellites[idx];
            let w = self
                .constellation_weights
                .iter()
                .find(|(c, _)| *c == sat.constellation)
                .map(|(_, w)| *w)
                .unwrap_or(0.5)
                * (sat.snr_dbhz / 50.0);
            total_lat += sat.elevation_deg * w;
            total_lon += sat.azimuth_deg * w;
            total_w += w;
            snr_sum += sat.snr_dbhz;
            if !used_c.contains(&sat.constellation) {
                used_c.push(sat.constellation);
            }
        }
        if total_w < 0.001 {
            return None;
        }
        let n = usable_indices.len();
        let avg_snr = snr_sum / n as f64;
        let quality = (avg_snr / 45.0).min(1.0) * (n as f64 / 20.0).min(1.0);
        let hdop = (4.0 / n as f64).max(0.5);
        self.fixes_computed += 1;
        let fix = GnssFix {
            latitude_deg: total_lat / total_w,
            longitude_deg: total_lon / total_w,
            altitude_m: 0.0,
            hdop,
            vdop: hdop * 1.5,
            pdop: (hdop * hdop + (hdop * 1.5) * (hdop * 1.5)).sqrt(),
            accuracy_m: hdop * 2.5,
            fix_type: FixType::Fix3D,
            satellites_used: n as u32,
            constellations_used: used_c,
            timestamp_ms: 0,
            quality_score: quality,
            dual_frequency: self.dual_freq_enabled,
        };
        self.last_fix = Some(fix.clone());
        Some(fix)
    }
    pub fn last_fix(&self) -> Option<&GnssFix> {
        self.last_fix.as_ref()
    }
    pub fn fixes_computed(&self) -> u64 {
        self.fixes_computed
    }
    pub fn set_weight(&mut self, c: Constellation, w: f64) {
        if let Some(e) = self
            .constellation_weights
            .iter_mut()
            .find(|(cc, _)| *cc == c)
        {
            e.1 = w.clamp(0.0, 1.0);
        }
    }
    pub fn set_min_elevation(&mut self, d: f64) {
        self.min_elevation_deg = d.max(0.0);
    }
    pub fn set_min_snr(&mut self, d: f64) {
        self.min_snr_dbhz = d.max(0.0);
    }
}

impl Default for MultiGnssEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn mk(c: Constellation, prn: u16, snr: f64) -> SatelliteInfo {
        SatelliteInfo {
            prn,
            constellation: c,
            elevation_deg: 45.0,
            azimuth_deg: 90.0,
            snr_dbhz: snr,
            used_in_fix: true,
            frequency: FrequencyBand::L1,
            health: SatHealth::Healthy,
        }
    }
    #[test]
    fn new_engine() {
        assert_eq!(MultiGnssEngine::new().tracked_count(), 0);
    }
    #[test]
    fn default_impl() {
        assert_eq!(MultiGnssEngine::default().fixes_computed(), 0);
    }
    #[test]
    fn update_sats() {
        let mut e = MultiGnssEngine::new();
        e.update_satellites(vec![mk(Constellation::Gps, 1, 40.0)]);
        assert_eq!(e.tracked_count(), 1);
    }
    #[test]
    fn tracked_by() {
        let mut e = MultiGnssEngine::new();
        e.update_satellites(vec![
            mk(Constellation::Gps, 1, 40.0),
            mk(Constellation::Glonass, 1, 35.0),
            mk(Constellation::Gps, 2, 38.0),
        ]);
        assert_eq!(e.tracked_by_constellation(Constellation::Gps), 2);
    }
    #[test]
    fn fix_needs_4() {
        let mut e = MultiGnssEngine::new();
        e.update_satellites(vec![mk(Constellation::Gps, 1, 40.0)]);
        assert!(e.compute_fix().is_none());
    }
    #[test]
    fn fix_ok() {
        let mut e = MultiGnssEngine::new();
        e.update_satellites((1..=8).map(|i| mk(Constellation::Gps, i, 40.0)).collect());
        let f = e.compute_fix().unwrap();
        assert_eq!(f.fix_type, FixType::Fix3D);
    }
    #[test]
    fn status() {
        let mut e = MultiGnssEngine::new();
        e.update_satellites(vec![mk(Constellation::Galileo, 1, 42.0)]);
        let s = e.constellation_status();
        assert!(s
            .iter()
            .any(|x| x.constellation == Constellation::Galileo && x.tracked == 1));
    }
    #[test]
    fn dual() {
        let mut e = MultiGnssEngine::new();
        e.enable_dual_frequency(true);
        assert!(e.is_dual_frequency());
    }
}
