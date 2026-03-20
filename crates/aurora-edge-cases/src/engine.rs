//! Edge case robustness — detects and handles reverse driving,
//! spinning, long stops, parking entry/exit, and unmapped roads.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeCase {
    ReverseDriving,
    Spinning,
    LongStop,
    ParkingManeuver,
    UnmappedRoad,
    StationaryDrift,
    Normal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeCaseDetector {
    heading_history: Vec<f64>,
    speed_history: Vec<f64>,
    stationary_since_ms: Option<u64>,
    current: EdgeCase,
    detections: u64,
    max_history: usize,
}

impl EdgeCaseDetector {
    pub fn new() -> Self {
        Self {
            heading_history: Vec::new(),
            speed_history: Vec::new(),
            stationary_since_ms: None,
            current: EdgeCase::Normal,
            detections: 0,
            max_history: 50,
        }
    }

    pub fn update(&mut self, heading_deg: f64, speed_mps: f64, on_road: bool, ts_ms: u64) {
        self.heading_history.push(heading_deg);
        self.speed_history.push(speed_mps);
        if self.heading_history.len() > self.max_history {
            self.heading_history.remove(0);
        }
        if self.speed_history.len() > self.max_history {
            self.speed_history.remove(0);
        }

        let prev = self.current;

        if speed_mps < 0.5 {
            if self.stationary_since_ms.is_none() {
                self.stationary_since_ms = Some(ts_ms);
            }
            if ts_ms.saturating_sub(self.stationary_since_ms.unwrap_or(ts_ms)) > 120_000 {
                self.current = EdgeCase::LongStop;
            } else if self.is_drifting() {
                self.current = EdgeCase::StationaryDrift;
            } else {
                self.current = EdgeCase::Normal;
            }
        } else {
            self.stationary_since_ms = None;
            if self.is_spinning() {
                self.current = EdgeCase::Spinning;
            } else if self.is_reverse() {
                self.current = EdgeCase::ReverseDriving;
            } else if speed_mps < 5.0 && !on_road {
                self.current = EdgeCase::ParkingManeuver;
            } else if !on_road {
                self.current = EdgeCase::UnmappedRoad;
            } else {
                self.current = EdgeCase::Normal;
            }
        }

        if self.current != prev && self.current != EdgeCase::Normal {
            self.detections += 1;
        }
    }

    fn is_spinning(&self) -> bool {
        if self.heading_history.len() < 10 {
            return false;
        }
        let recent = &self.heading_history[self.heading_history.len() - 10..];
        let total_turn: f64 = recent
            .windows(2)
            .map(|w| {
                let d = (w[1] - w[0]).abs();
                if d > 180.0 {
                    360.0 - d
                } else {
                    d
                }
            })
            .sum();
        total_turn > 720.0 // 2 full rotations in 10 samples
    }

    fn is_reverse(&self) -> bool {
        if self.heading_history.len() < 3 {
            return false;
        }
        let n = self.heading_history.len();
        let diff = (self.heading_history[n - 1] - self.heading_history[n - 3]).abs();
        let diff = if diff > 180.0 { 360.0 - diff } else { diff };
        diff > 150.0 && self.speed_history.last().copied().unwrap_or(0.0) > 1.0
    }

    fn is_drifting(&self) -> bool {
        if self.heading_history.len() < 5 {
            return false;
        }
        let recent = &self.heading_history[self.heading_history.len() - 5..];
        let variance: f64 = {
            let mean = recent.iter().sum::<f64>() / recent.len() as f64;
            recent.iter().map(|h| (h - mean).powi(2)).sum::<f64>() / recent.len() as f64
        };
        variance > 100.0
    }

    pub fn current(&self) -> EdgeCase {
        self.current
    }
    pub fn detections(&self) -> u64 {
        self.detections
    }
}

impl Default for EdgeCaseDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let d = EdgeCaseDetector::new();
        assert_eq!(d.current(), EdgeCase::Normal);
    }

    #[test]
    fn test_default() {
        let d = EdgeCaseDetector::default();
        assert_eq!(d.detections(), 0);
    }

    #[test]
    fn test_normal_driving() {
        let mut d = EdgeCaseDetector::new();
        d.update(90.0, 15.0, true, 1000);
        assert_eq!(d.current(), EdgeCase::Normal);
    }

    #[test]
    fn test_long_stop() {
        let mut d = EdgeCaseDetector::new();
        d.update(90.0, 0.0, true, 0);
        d.update(90.0, 0.0, true, 130_000);
        assert_eq!(d.current(), EdgeCase::LongStop);
    }

    #[test]
    fn test_unmapped_road() {
        let mut d = EdgeCaseDetector::new();
        d.update(90.0, 10.0, false, 1000);
        assert_eq!(d.current(), EdgeCase::UnmappedRoad);
    }

    #[test]
    fn test_parking() {
        let mut d = EdgeCaseDetector::new();
        d.update(90.0, 3.0, false, 1000);
        assert_eq!(d.current(), EdgeCase::ParkingManeuver);
    }

    #[test]
    fn test_spinning() {
        let mut d = EdgeCaseDetector::new();
        for i in 0..20 {
            d.update((i as f64 * 90.0) % 360.0, 5.0, true, i * 100);
        }
        assert_eq!(d.current(), EdgeCase::Spinning);
    }

    #[test]
    fn test_reverse() {
        let mut d = EdgeCaseDetector::new();
        d.update(0.0, 10.0, true, 1000);
        d.update(90.0, 10.0, true, 2000);
        d.update(180.0, 10.0, true, 3000);
        assert_eq!(d.current(), EdgeCase::ReverseDriving);
    }
}
