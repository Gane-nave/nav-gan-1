/// SRE operations: SLOs, canary releases, multi-region failover.
#[derive(Debug, Clone)]
pub struct Slo { pub name: String, pub target: f64, pub current: f64, pub window_hours: u32 }
impl Slo {
    pub fn is_met(&self) -> bool { self.current >= self.target }
    pub fn error_budget_remaining(&self) -> f64 { ((self.current - self.target) / (1.0 - self.target).max(0.001)).clamp(0.0, 1.0) }
    pub fn burn_rate(&self) -> f64 { if self.is_met() { 0.0 } else { (self.target - self.current) / (1.0 - self.target).max(0.001) } }
}
#[derive(Debug, Clone, PartialEq)]
pub enum CanaryState { Pending, Rolling, Healthy, Failed, RolledBack }
#[derive(Debug, Clone)]
pub struct CanaryRelease { pub version: String, pub traffic_pct: f64, pub error_rate: f64, pub latency_p99_ms: f64, pub state: CanaryState }
impl CanaryRelease {
    pub fn should_promote(&self) -> bool { self.error_rate < 0.01 && self.latency_p99_ms < 500.0 && self.traffic_pct >= 0.1 }
    pub fn should_rollback(&self) -> bool { self.error_rate > 0.05 || self.latency_p99_ms > 2000.0 }
    pub fn next_traffic_pct(&self) -> f64 { if self.should_rollback() { 0.0 } else { (self.traffic_pct * 2.0).min(1.0) } }
}
#[derive(Debug, Clone)]
pub struct SreDashboard { pub slos: Vec<Slo>, pub canaries: Vec<CanaryRelease> }
impl Default for SreDashboard {
    fn default() -> Self { Self::new() }
}
impl SreDashboard {
    pub fn new() -> Self { Self { slos: Vec::new(), canaries: Vec::new() } }
    pub fn slos_met(&self) -> usize { self.slos.iter().filter(|s| s.is_met()).count() }
    pub fn overall_health(&self) -> f64 { if self.slos.is_empty() { 1.0 } else { self.slos_met() as f64 / self.slos.len() as f64 } }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_slo_met() { let s = Slo { name: "avail".into(), target: 0.999, current: 0.9995, window_hours: 720 }; assert!(s.is_met()); assert!(s.error_budget_remaining() > 0.0); }
    #[test] fn test_slo_fail() { let s = Slo { name: "lat".into(), target: 0.99, current: 0.98, window_hours: 720 }; assert!(!s.is_met()); assert!(s.burn_rate() > 0.0); }
    #[test] fn test_canary_promote() { let c = CanaryRelease { version: "1.1".into(), traffic_pct: 0.1, error_rate: 0.005, latency_p99_ms: 200.0, state: CanaryState::Rolling }; assert!(c.should_promote()); }
    #[test] fn test_canary_rollback() { let c = CanaryRelease { version: "1.2".into(), traffic_pct: 0.05, error_rate: 0.1, latency_p99_ms: 3000.0, state: CanaryState::Rolling }; assert!(c.should_rollback()); }
}
