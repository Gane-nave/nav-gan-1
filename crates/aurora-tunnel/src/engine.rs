use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TunnelState {
    Normal,
    Entering,
    InTunnel,
    Exiting,
    Reacquiring,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelPosition {
    pub lat: f64,
    pub lon: f64,
    pub heading_deg: f64,
    pub speed_mps: f64,
    pub confidence: f64,
    pub distance_in_tunnel_m: f64,
    pub state: TunnelState,
}

pub struct TunnelMode {
    state: TunnelState,
    current: TunnelPosition,
    confidence_decay_rate: f64,
    total_tunnel_distance_m: f64,
    tunnel_count: u32,
}

impl TunnelMode {
    pub fn new() -> Self {
        Self {
            state: TunnelState::Normal,
            current: TunnelPosition {
                lat: 0.0,
                lon: 0.0,
                heading_deg: 0.0,
                speed_mps: 0.0,
                confidence: 1.0,
                distance_in_tunnel_m: 0.0,
                state: TunnelState::Normal,
            },
            confidence_decay_rate: 0.02,
            total_tunnel_distance_m: 0.0,
            tunnel_count: 0,
        }
    }
    pub fn state(&self) -> TunnelState {
        self.state
    }
    pub fn is_in_tunnel(&self) -> bool {
        matches!(self.state, TunnelState::InTunnel | TunnelState::Entering)
    }
    pub fn current_position(&self) -> &TunnelPosition {
        &self.current
    }
    pub fn tunnel_count(&self) -> u32 {
        self.tunnel_count
    }
    pub fn total_tunnel_distance(&self) -> f64 {
        self.total_tunnel_distance_m
    }
    pub fn gnss_lost(&mut self, lat: f64, lon: f64, heading: f64, speed: f64, _ts: u64) {
        if self.state == TunnelState::Normal {
            self.current = TunnelPosition {
                lat,
                lon,
                heading_deg: heading,
                speed_mps: speed,
                confidence: 0.95,
                distance_in_tunnel_m: 0.0,
                state: TunnelState::Entering,
            };
            self.tunnel_count += 1;
        }
        self.state = TunnelState::InTunnel;
        self.current.state = TunnelState::InTunnel;
    }
    pub fn propagate(&mut self, dt_s: f64) {
        if !self.is_in_tunnel() {
            return;
        }
        let dist = self.current.speed_mps * dt_s;
        let hr = self.current.heading_deg.to_radians();
        self.current.lat += (hr.cos() * dist) / 111_111.0;
        self.current.lon +=
            (hr.sin() * dist) / (111_111.0 * self.current.lat.to_radians().cos().max(0.01));
        self.current.distance_in_tunnel_m += dist;
        self.total_tunnel_distance_m += dist;
        self.current.confidence =
            (self.current.confidence - self.confidence_decay_rate * dt_s).max(0.1);
    }
    pub fn gnss_recovered(&mut self, lat: f64, lon: f64, heading: f64, speed: f64) {
        self.state = TunnelState::Reacquiring;
        self.current = TunnelPosition {
            lat,
            lon,
            heading_deg: heading,
            speed_mps: speed,
            confidence: 0.7,
            distance_in_tunnel_m: self.current.distance_in_tunnel_m,
            state: TunnelState::Reacquiring,
        };
    }
    pub fn reacquisition_complete(&mut self) {
        self.state = TunnelState::Normal;
        self.current.state = TunnelState::Normal;
        self.current.confidence = 1.0;
    }
    pub fn set_confidence_decay(&mut self, r: f64) {
        self.confidence_decay_rate = r.max(0.0);
    }
}

impl Default for TunnelMode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_normal() {
        assert_eq!(TunnelMode::new().state(), TunnelState::Normal);
    }
    #[test]
    fn default_ok() {
        assert!(!TunnelMode::default().is_in_tunnel());
    }
    #[test]
    fn enter() {
        let mut t = TunnelMode::new();
        t.gnss_lost(32.0, 34.0, 90.0, 20.0, 0);
        assert!(t.is_in_tunnel());
    }
    #[test]
    fn propagate_moves() {
        let mut t = TunnelMode::new();
        t.gnss_lost(32.0, 34.0, 90.0, 20.0, 0);
        t.propagate(1.0);
        assert!(t.current_position().distance_in_tunnel_m > 0.0);
    }
    #[test]
    fn conf_decay() {
        let mut t = TunnelMode::new();
        t.gnss_lost(32.0, 34.0, 90.0, 20.0, 0);
        t.propagate(10.0);
        assert!(t.current_position().confidence < 0.95);
    }
    #[test]
    fn recover() {
        let mut t = TunnelMode::new();
        t.gnss_lost(32.0, 34.0, 90.0, 20.0, 0);
        t.gnss_recovered(32.01, 34.01, 90.0, 20.0);
        assert_eq!(t.state(), TunnelState::Reacquiring);
    }
    #[test]
    fn full_cycle() {
        let mut t = TunnelMode::new();
        t.gnss_lost(32.0, 34.0, 90.0, 20.0, 0);
        t.propagate(5.0);
        t.gnss_recovered(32.01, 34.01, 90.0, 20.0);
        t.reacquisition_complete();
        assert_eq!(t.state(), TunnelState::Normal);
    }
    #[test]
    fn count_tunnels() {
        let mut t = TunnelMode::new();
        t.gnss_lost(32.0, 34.0, 90.0, 20.0, 0);
        t.gnss_recovered(32.01, 34.01, 90.0, 20.0);
        t.reacquisition_complete();
        t.gnss_lost(32.02, 34.02, 180.0, 15.0, 1000);
        assert_eq!(t.tunnel_count(), 2);
    }
}
