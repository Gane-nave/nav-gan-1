use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SensorSource {
    Gnss,
    Imu,
    Magnetometer,
    Barometer,
    CellTower,
    WiFi,
    Odometry,
    MapMatch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorMeasurement {
    pub source: SensorSource,
    pub value: [f64; 3],
    pub accuracy: f64,
    pub timestamp_ms: u64,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusedState {
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub altitude_m: f64,
    pub velocity_north_mps: f64,
    pub velocity_east_mps: f64,
    pub velocity_down_mps: f64,
    pub heading_deg: f64,
    pub accuracy_m: f64,
    pub confidence: f64,
    pub sources_used: Vec<SensorSource>,
    pub timestamp_ms: u64,
}

pub struct EkfFusion {
    state: [f64; 9],
    covariance: [f64; 9],
    measurements: Vec<SensorMeasurement>,
    source_weights: Vec<(SensorSource, f64)>,
    updates: u64,
    last_fused: Option<FusedState>,
}

impl EkfFusion {
    pub fn new() -> Self {
        Self {
            state: [0.0; 9],
            covariance: [1.0; 9],
            measurements: Vec::new(),
            source_weights: vec![
                (SensorSource::Gnss, 1.0),
                (SensorSource::Imu, 0.8),
                (SensorSource::Magnetometer, 0.6),
                (SensorSource::Barometer, 0.5),
                (SensorSource::CellTower, 0.3),
                (SensorSource::WiFi, 0.4),
                (SensorSource::Odometry, 0.7),
                (SensorSource::MapMatch, 0.9),
            ],
            updates: 0,
            last_fused: None,
        }
    }
    pub fn predict(&mut self, dt_s: f64) {
        self.state[0] += self.state[3] * dt_s * 0.00001;
        self.state[1] += self.state[4] * dt_s * 0.00001;
        self.state[2] += self.state[5] * dt_s;
        for c in &mut self.covariance {
            *c *= 1.0 + dt_s * 0.01;
        }
    }
    pub fn add_measurement(&mut self, m: SensorMeasurement) {
        self.measurements.push(m);
    }
    pub fn update(&mut self) -> FusedState {
        let mut tw = 0.0;
        let mut wlat = 0.0;
        let mut wlon = 0.0;
        let mut walt = 0.0;
        let mut srcs = Vec::new();
        for m in &self.measurements {
            let bw = self
                .source_weights
                .iter()
                .find(|(s, _)| *s == m.source)
                .map(|(_, w)| *w)
                .unwrap_or(0.5);
            let w = bw * m.weight / m.accuracy.max(0.1);
            wlat += m.value[0] * w;
            wlon += m.value[1] * w;
            walt += m.value[2] * w;
            tw += w;
            if !srcs.contains(&m.source) {
                srcs.push(m.source);
            }
        }
        if tw > 0.001 {
            self.state[0] = wlat / tw;
            self.state[1] = wlon / tw;
            self.state[2] = walt / tw;
            for c in &mut self.covariance {
                *c *= 0.9;
            }
        }
        self.updates += 1;
        self.measurements.clear();
        let confidence =
            (srcs.len() as f64 / 4.0).min(1.0) * (1.0 - self.covariance[0].min(1.0) * 0.5);
        let fused = FusedState {
            latitude_deg: self.state[0],
            longitude_deg: self.state[1],
            altitude_m: self.state[2],
            velocity_north_mps: self.state[3],
            velocity_east_mps: self.state[4],
            velocity_down_mps: self.state[5],
            heading_deg: self.state[6],
            accuracy_m: self.covariance[0].max(0.1) * 2.5,
            confidence,
            sources_used: srcs,
            timestamp_ms: 0,
        };
        self.last_fused = Some(fused.clone());
        fused
    }
    pub fn set_source_weight(&mut self, s: SensorSource, w: f64) {
        if let Some(e) = self.source_weights.iter_mut().find(|(ss, _)| *ss == s) {
            e.1 = w.clamp(0.0, 2.0);
        }
    }
    pub fn update_count(&self) -> u64 {
        self.updates
    }
    pub fn last_fused(&self) -> Option<&FusedState> {
        self.last_fused.as_ref()
    }
    pub fn pending_measurements(&self) -> usize {
        self.measurements.len()
    }
    pub fn state(&self) -> &[f64; 9] {
        &self.state
    }
    pub fn reset(&mut self) {
        self.state = [0.0; 9];
        self.covariance = [1.0; 9];
        self.measurements.clear();
        self.last_fused = None;
    }
}

impl Default for EkfFusion {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn gm(lat: f64, lon: f64) -> SensorMeasurement {
        SensorMeasurement {
            source: SensorSource::Gnss,
            value: [lat, lon, 0.0],
            accuracy: 2.5,
            timestamp_ms: 0,
            weight: 1.0,
        }
    }
    #[test]
    fn new_e() {
        assert_eq!(EkfFusion::new().update_count(), 0);
    }
    #[test]
    fn default_e() {
        assert!(EkfFusion::default().last_fused().is_none());
    }
    #[test]
    fn add_upd() {
        let mut e = EkfFusion::new();
        e.add_measurement(gm(32.0, 34.0));
        let f = e.update();
        assert!(f.confidence > 0.0);
    }
    #[test]
    fn predict_t() {
        let mut e = EkfFusion::new();
        e.predict(1.0);
        assert_eq!(e.update_count(), 0);
    }
    #[test]
    fn multi_src() {
        let mut e = EkfFusion::new();
        e.add_measurement(gm(32.0, 34.0));
        e.add_measurement(SensorMeasurement {
            source: SensorSource::Imu,
            value: [32.001, 34.001, 0.0],
            accuracy: 5.0,
            timestamp_ms: 0,
            weight: 1.0,
        });
        let f = e.update();
        assert!(f.sources_used.len() >= 2);
    }
    #[test]
    fn reset_c() {
        let mut e = EkfFusion::new();
        e.add_measurement(gm(32.0, 34.0));
        e.update();
        e.reset();
        assert!(e.last_fused().is_none());
    }
    #[test]
    fn pending() {
        let mut e = EkfFusion::new();
        e.add_measurement(gm(32.0, 34.0));
        assert_eq!(e.pending_measurements(), 1);
    }
    #[test]
    fn state_a() {
        assert_eq!(EkfFusion::new().state().len(), 9);
    }
}
