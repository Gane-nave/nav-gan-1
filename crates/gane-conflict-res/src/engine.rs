//! Conflict resolution — decides which source to trust when
//! GNSS, map matching, and IMU disagree.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Source {
    Gnss,
    MapMatch,
    Imu,
    Fused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceEstimate {
    pub source: Source,
    pub lat: f64,
    pub lon: f64,
    pub confidence: f64,
    pub age_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionStrategy {
    HighestConfidence,
    WeightedAverage,
    PhysicsConstrained,
    HistoryBased,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolver {
    strategy: ResolutionStrategy,
    history: Vec<(f64, f64)>,
    max_history: usize,
    max_jump_m: f64,
    resolutions: u64,
}

impl ConflictResolver {
    pub fn new(strategy: ResolutionStrategy) -> Self {
        Self {
            strategy,
            history: Vec::new(),
            max_history: 100,
            max_jump_m: 50.0,
            resolutions: 0,
        }
    }

    fn distance_m(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let dlat = (lat2 - lat1) * 111_111.0;
        let dlon = (lon2 - lon1) * 111_111.0 * ((lat1 + lat2) / 2.0).to_radians().cos();
        (dlat * dlat + dlon * dlon).sqrt()
    }

    pub fn resolve(&mut self, estimates: &[SourceEstimate]) -> Option<(f64, f64, f64, Source)> {
        if estimates.is_empty() {
            return None;
        }

        let result = match self.strategy {
            ResolutionStrategy::HighestConfidence => {
                let best = estimates
                    .iter()
                    .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap())?;
                (best.lat, best.lon, best.confidence, best.source)
            }
            ResolutionStrategy::WeightedAverage => {
                let total: f64 = estimates.iter().map(|e| e.confidence).sum();
                if total < 1e-15 {
                    return None;
                }
                let lat = estimates.iter().map(|e| e.lat * e.confidence).sum::<f64>() / total;
                let lon = estimates.iter().map(|e| e.lon * e.confidence).sum::<f64>() / total;
                (lat, lon, total / estimates.len() as f64, Source::Fused)
            }
            ResolutionStrategy::PhysicsConstrained => {
                // Reject estimates that jump too far from history
                let valid: Vec<_> = if let Some(last) = self.history.last() {
                    estimates
                        .iter()
                        .filter(|e| {
                            Self::distance_m(last.0, last.1, e.lat, e.lon) <= self.max_jump_m
                        })
                        .collect()
                } else {
                    estimates.iter().collect()
                };
                if valid.is_empty() {
                    let best = estimates
                        .iter()
                        .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap())?;
                    (best.lat, best.lon, best.confidence * 0.5, best.source)
                } else {
                    let best = valid
                        .iter()
                        .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap())?;
                    (best.lat, best.lon, best.confidence, best.source)
                }
            }
            ResolutionStrategy::HistoryBased => {
                // Weight by both confidence and consistency with history
                if self.history.is_empty() {
                    let best = estimates
                        .iter()
                        .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap())?;
                    (best.lat, best.lon, best.confidence, best.source)
                } else {
                    let last = self.history.last().unwrap();
                    let mut best_score = f64::MIN;
                    let mut best_idx = 0;
                    for (i, e) in estimates.iter().enumerate() {
                        let dist = Self::distance_m(last.0, last.1, e.lat, e.lon);
                        let consistency = 1.0 / (1.0 + dist / 10.0);
                        let score = e.confidence * 0.6 + consistency * 0.4;
                        if score > best_score {
                            best_score = score;
                            best_idx = i;
                        }
                    }
                    let e = &estimates[best_idx];
                    (e.lat, e.lon, best_score, e.source)
                }
            }
        };

        self.history.push((result.0, result.1));
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
        self.resolutions += 1;
        Some(result)
    }

    pub fn resolutions(&self) -> u64 {
        self.resolutions
    }
    pub fn strategy(&self) -> ResolutionStrategy {
        self.strategy
    }
}

impl Default for ConflictResolver {
    fn default() -> Self {
        Self::new(ResolutionStrategy::WeightedAverage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let r = ConflictResolver::new(ResolutionStrategy::HighestConfidence);
        assert_eq!(r.resolutions(), 0);
    }

    #[test]
    fn test_default() {
        let r = ConflictResolver::default();
        assert_eq!(r.strategy(), ResolutionStrategy::WeightedAverage);
    }

    #[test]
    fn test_highest_confidence() {
        let mut r = ConflictResolver::new(ResolutionStrategy::HighestConfidence);
        let est = vec![
            SourceEstimate {
                source: Source::Gnss,
                lat: 32.0,
                lon: 34.0,
                confidence: 0.9,
                age_ms: 100,
            },
            SourceEstimate {
                source: Source::Imu,
                lat: 32.1,
                lon: 34.1,
                confidence: 0.3,
                age_ms: 10,
            },
        ];
        let (lat, _, _, src) = r.resolve(&est).unwrap();
        assert_eq!(src, Source::Gnss);
        assert!((lat - 32.0).abs() < 0.001);
    }

    #[test]
    fn test_weighted_average() {
        let mut r = ConflictResolver::default();
        let est = vec![
            SourceEstimate {
                source: Source::Gnss,
                lat: 32.0,
                lon: 34.0,
                confidence: 1.0,
                age_ms: 100,
            },
            SourceEstimate {
                source: Source::Imu,
                lat: 32.1,
                lon: 34.1,
                confidence: 1.0,
                age_ms: 10,
            },
        ];
        let (lat, _, _, src) = r.resolve(&est).unwrap();
        assert_eq!(src, Source::Fused);
        assert!((lat - 32.05).abs() < 0.001);
    }

    #[test]
    fn test_physics_rejects_jump() {
        let mut r = ConflictResolver::new(ResolutionStrategy::PhysicsConstrained);
        r.resolve(&[SourceEstimate {
            source: Source::Gnss,
            lat: 32.0,
            lon: 34.0,
            confidence: 0.9,
            age_ms: 0,
        }]);
        let est = vec![
            SourceEstimate {
                source: Source::Gnss,
                lat: 33.0,
                lon: 34.0,
                confidence: 0.9,
                age_ms: 0,
            }, // huge jump
            SourceEstimate {
                source: Source::Imu,
                lat: 32.0001,
                lon: 34.0,
                confidence: 0.5,
                age_ms: 0,
            }, // close
        ];
        let (lat, _, _, src) = r.resolve(&est).unwrap();
        assert_eq!(src, Source::Imu);
        assert!((lat - 32.0001).abs() < 0.001);
    }

    #[test]
    fn test_empty() {
        let mut r = ConflictResolver::default();
        assert!(r.resolve(&[]).is_none());
    }

    #[test]
    fn test_history_based() {
        let mut r = ConflictResolver::new(ResolutionStrategy::HistoryBased);
        r.resolve(&[SourceEstimate {
            source: Source::Gnss,
            lat: 32.0,
            lon: 34.0,
            confidence: 0.9,
            age_ms: 0,
        }]);
        let est = vec![
            SourceEstimate {
                source: Source::Gnss,
                lat: 32.1,
                lon: 34.0,
                confidence: 0.9,
                age_ms: 0,
            },
            SourceEstimate {
                source: Source::MapMatch,
                lat: 32.0001,
                lon: 34.0,
                confidence: 0.7,
                age_ms: 0,
            },
        ];
        let (_, _, _, src) = r.resolve(&est).unwrap();
        assert_eq!(src, Source::MapMatch); // more consistent with history
    }

    #[test]
    fn test_resolution_count() {
        let mut r = ConflictResolver::default();
        r.resolve(&[SourceEstimate {
            source: Source::Gnss,
            lat: 32.0,
            lon: 34.0,
            confidence: 0.9,
            age_ms: 0,
        }]);
        r.resolve(&[SourceEstimate {
            source: Source::Gnss,
            lat: 32.0,
            lon: 34.0,
            confidence: 0.9,
            age_ms: 0,
        }]);
        assert_eq!(r.resolutions(), 2);
    }

    #[test]
    fn test_single_source() {
        let mut r = ConflictResolver::default();
        let (lat, _, _, _) = r
            .resolve(&[SourceEstimate {
                source: Source::Gnss,
                lat: 32.0,
                lon: 34.0,
                confidence: 0.9,
                age_ms: 0,
            }])
            .unwrap();
        assert!((lat - 32.0).abs() < 0.001);
    }
}
