/// External benchmarking framework vs competitors.
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub system_name: String,
    pub median_eta_error_s: f64,
    pub p95_eta_error_s: f64,
    pub off_route_rate: f64,
    pub time_to_first_fix_ms: f64,
    pub reacquisition_time_ms: f64,
    pub position_jitter_m: f64,
    pub heading_jitter_deg: f64,
}
impl BenchmarkResult {
    pub fn quality_score(&self) -> f64 {
        let eta = (1.0 - (self.median_eta_error_s / 300.0).min(1.0)) * 0.25;
        let route = (1.0 - self.off_route_rate.min(1.0)) * 0.2;
        let ttff = (1.0 - (self.time_to_first_fix_ms / 30000.0).min(1.0)) * 0.15;
        let reacq = (1.0 - (self.reacquisition_time_ms / 10000.0).min(1.0)) * 0.15;
        let jp = (1.0 - (self.position_jitter_m / 10.0).min(1.0)) * 0.15;
        let jh = (1.0 - (self.heading_jitter_deg / 30.0).min(1.0)) * 0.1;
        (eta + route + ttff + reacq + jp + jh).clamp(0.0, 1.0)
    }
}
#[derive(Debug, Clone)]
pub struct ComparisonReport {
    pub ours: BenchmarkResult,
    pub competitor: BenchmarkResult,
}
impl ComparisonReport {
    pub fn advantage(&self) -> f64 {
        self.ours.quality_score() - self.competitor.quality_score()
    }
    pub fn metrics_won(&self) -> usize {
        let mut w = 0;
        if self.ours.median_eta_error_s < self.competitor.median_eta_error_s {
            w += 1;
        }
        if self.ours.off_route_rate < self.competitor.off_route_rate {
            w += 1;
        }
        if self.ours.time_to_first_fix_ms < self.competitor.time_to_first_fix_ms {
            w += 1;
        }
        if self.ours.reacquisition_time_ms < self.competitor.reacquisition_time_ms {
            w += 1;
        }
        if self.ours.position_jitter_m < self.competitor.position_jitter_m {
            w += 1;
        }
        w
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn mk(n: &str, eta: f64, ofr: f64, ttff: f64) -> BenchmarkResult {
        BenchmarkResult {
            system_name: n.into(),
            median_eta_error_s: eta,
            p95_eta_error_s: eta * 2.0,
            off_route_rate: ofr,
            time_to_first_fix_ms: ttff,
            reacquisition_time_ms: 2000.0,
            position_jitter_m: 1.5,
            heading_jitter_deg: 3.0,
        }
    }
    #[test]
    fn test_quality() {
        assert!(mk("a", 15.0, 0.02, 1500.0).quality_score() > 0.0);
    }
    #[test]
    fn test_wins() {
        let r = ComparisonReport {
            ours: mk("a", 10.0, 0.01, 1000.0),
            competitor: mk("b", 30.0, 0.05, 5000.0),
        };
        assert!(r.advantage() > 0.0);
        assert!(r.metrics_won() >= 3);
    }
    #[test]
    fn test_perfect() {
        let r = BenchmarkResult {
            system_name: "p".into(),
            median_eta_error_s: 0.0,
            p95_eta_error_s: 0.0,
            off_route_rate: 0.0,
            time_to_first_fix_ms: 0.0,
            reacquisition_time_ms: 0.0,
            position_jitter_m: 0.0,
            heading_jitter_deg: 0.0,
        };
        assert!((r.quality_score() - 1.0).abs() < 0.01);
    }
}
