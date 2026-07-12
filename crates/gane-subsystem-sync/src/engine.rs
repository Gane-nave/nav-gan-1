//! Subsystem consistency layer — detects and resolves contradictions
//! between routing, map matching, and position subsystems.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Subsystem {
    Routing,
    MapMatch,
    Position,
    Imu,
    Gnss,
    Ui,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsystemClaim {
    pub source: Subsystem,
    pub lat: f64,
    pub lon: f64,
    pub confidence: f64,
    pub timestamp_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyArbiter {
    claims: Vec<SubsystemClaim>,
    max_disagreement_m: f64,
    conflicts_detected: u64,
    resolutions: u64,
}

impl ConsistencyArbiter {
    pub fn new(max_disagreement_m: f64) -> Self {
        Self {
            claims: Vec::new(),
            max_disagreement_m,
            conflicts_detected: 0,
            resolutions: 0,
        }
    }

    pub fn submit_claim(&mut self, claim: SubsystemClaim) {
        self.claims.retain(|c| c.source != claim.source);
        self.claims.push(claim);
    }

    /// Approximate distance in metres between two lat/lon points.
    fn approx_distance_m(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let dlat = (lat2 - lat1).to_radians() * 6_371_000.0;
        let dlon =
            (lon2 - lon1).to_radians() * 6_371_000.0 * ((lat1 + lat2) / 2.0).to_radians().cos();
        (dlat * dlat + dlon * dlon).sqrt()
    }

    /// Check for conflicts between subsystem claims.
    pub fn check_conflicts(&mut self) -> Vec<(Subsystem, Subsystem, f64)> {
        let mut conflicts = Vec::new();
        for i in 0..self.claims.len() {
            for j in (i + 1)..self.claims.len() {
                let d = Self::approx_distance_m(
                    self.claims[i].lat,
                    self.claims[i].lon,
                    self.claims[j].lat,
                    self.claims[j].lon,
                );
                if d > self.max_disagreement_m {
                    conflicts.push((self.claims[i].source, self.claims[j].source, d));
                    self.conflicts_detected += 1;
                }
            }
        }
        conflicts
    }

    /// Resolve by weighted confidence average.
    pub fn resolve(&mut self) -> Option<(f64, f64, f64)> {
        if self.claims.is_empty() {
            return None;
        }
        let total_conf: f64 = self.claims.iter().map(|c| c.confidence).sum();
        if total_conf < 1e-15 {
            return None;
        }
        let lat = self
            .claims
            .iter()
            .map(|c| c.lat * c.confidence)
            .sum::<f64>()
            / total_conf;
        let lon = self
            .claims
            .iter()
            .map(|c| c.lon * c.confidence)
            .sum::<f64>()
            / total_conf;
        self.resolutions += 1;
        Some((lat, lon, total_conf / self.claims.len() as f64))
    }

    pub fn clear_claims(&mut self) {
        self.claims.clear();
    }
    pub fn conflicts_detected(&self) -> u64 {
        self.conflicts_detected
    }
    pub fn resolutions(&self) -> u64 {
        self.resolutions
    }
    pub fn claim_count(&self) -> usize {
        self.claims.len()
    }
}

impl Default for ConsistencyArbiter {
    fn default() -> Self {
        Self::new(50.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let a = ConsistencyArbiter::new(100.0);
        assert_eq!(a.claim_count(), 0);
    }

    #[test]
    fn test_default() {
        let a = ConsistencyArbiter::default();
        assert_eq!(a.conflicts_detected(), 0);
    }

    #[test]
    fn test_submit_claim() {
        let mut a = ConsistencyArbiter::default();
        a.submit_claim(SubsystemClaim {
            source: Subsystem::Gnss,
            lat: 32.0,
            lon: 34.0,
            confidence: 0.9,
            timestamp_ms: 1000,
        });
        assert_eq!(a.claim_count(), 1);
    }

    #[test]
    fn test_replace_claim() {
        let mut a = ConsistencyArbiter::default();
        a.submit_claim(SubsystemClaim {
            source: Subsystem::Gnss,
            lat: 32.0,
            lon: 34.0,
            confidence: 0.9,
            timestamp_ms: 1000,
        });
        a.submit_claim(SubsystemClaim {
            source: Subsystem::Gnss,
            lat: 32.1,
            lon: 34.1,
            confidence: 0.8,
            timestamp_ms: 2000,
        });
        assert_eq!(a.claim_count(), 1);
    }

    #[test]
    fn test_no_conflict_close() {
        let mut a = ConsistencyArbiter::new(1000.0);
        a.submit_claim(SubsystemClaim {
            source: Subsystem::Gnss,
            lat: 32.0,
            lon: 34.0,
            confidence: 0.9,
            timestamp_ms: 1000,
        });
        a.submit_claim(SubsystemClaim {
            source: Subsystem::MapMatch,
            lat: 32.0001,
            lon: 34.0001,
            confidence: 0.8,
            timestamp_ms: 1000,
        });
        assert!(a.check_conflicts().is_empty());
    }

    #[test]
    fn test_conflict_far() {
        let mut a = ConsistencyArbiter::new(50.0);
        a.submit_claim(SubsystemClaim {
            source: Subsystem::Gnss,
            lat: 32.0,
            lon: 34.0,
            confidence: 0.9,
            timestamp_ms: 1000,
        });
        a.submit_claim(SubsystemClaim {
            source: Subsystem::MapMatch,
            lat: 32.01,
            lon: 34.01,
            confidence: 0.8,
            timestamp_ms: 1000,
        });
        assert!(!a.check_conflicts().is_empty());
    }

    #[test]
    fn test_resolve() {
        let mut a = ConsistencyArbiter::default();
        a.submit_claim(SubsystemClaim {
            source: Subsystem::Gnss,
            lat: 32.0,
            lon: 34.0,
            confidence: 1.0,
            timestamp_ms: 1000,
        });
        a.submit_claim(SubsystemClaim {
            source: Subsystem::Imu,
            lat: 32.1,
            lon: 34.1,
            confidence: 1.0,
            timestamp_ms: 1000,
        });
        let (lat, lon, _) = a.resolve().unwrap();
        assert!((lat - 32.05).abs() < 0.001);
        assert!((lon - 34.05).abs() < 0.001);
    }

    #[test]
    fn test_weighted_resolve() {
        let mut a = ConsistencyArbiter::default();
        a.submit_claim(SubsystemClaim {
            source: Subsystem::Gnss,
            lat: 32.0,
            lon: 34.0,
            confidence: 0.9,
            timestamp_ms: 1000,
        });
        a.submit_claim(SubsystemClaim {
            source: Subsystem::Imu,
            lat: 32.1,
            lon: 34.1,
            confidence: 0.1,
            timestamp_ms: 1000,
        });
        let (lat, _, _) = a.resolve().unwrap();
        assert!(lat < 32.05); // closer to GNSS due to higher confidence
    }
}
