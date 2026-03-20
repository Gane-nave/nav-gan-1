/// Chaos testing engine: fault injection, resilience verification.
#[derive(Debug, Clone, PartialEq)]
pub enum FaultType { NetworkLatency, NetworkPartition, DiskFull, CpuSpike, MemoryPressure, ServiceCrash, ClockSkew }
#[derive(Debug, Clone)]
pub struct ChaosExperiment { pub name: String, pub fault_type: FaultType, pub target_service: String, pub duration_s: u32, pub intensity: f64 }
impl ChaosExperiment {
    pub fn risk_level(&self) -> f64 {
        let base = match self.fault_type { FaultType::ServiceCrash => 0.9, FaultType::NetworkPartition => 0.8, FaultType::DiskFull => 0.7, FaultType::MemoryPressure => 0.6, FaultType::CpuSpike => 0.5, FaultType::NetworkLatency => 0.3, FaultType::ClockSkew => 0.4 };
        (base * self.intensity.clamp(0.0, 1.0)).clamp(0.0, 1.0)
    }
    pub fn is_safe_for_prod(&self) -> bool { self.risk_level() < 0.5 && self.duration_s <= 60 }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ExperimentResult { Passed, Failed, Degraded }
#[derive(Debug, Clone)]
pub struct ChaosReport { pub experiment: ChaosExperiment, pub result: ExperimentResult, pub recovery_time_ms: u64, pub data_loss: bool }
impl ChaosReport {
    pub fn resilience_score(&self) -> f64 {
        let base = match self.result { ExperimentResult::Passed => 1.0, ExperimentResult::Degraded => 0.5, ExperimentResult::Failed => 0.0 };
        let recovery = (1.0 - (self.recovery_time_ms as f64 / 30000.0).min(1.0)) * 0.3;
        let loss = if self.data_loss { 0.0 } else { 0.2 };
        (base * 0.5 + recovery + loss).clamp(0.0, 1.0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_risk() { let e = ChaosExperiment { name: "net".into(), fault_type: FaultType::NetworkLatency, target_service: "gw".into(), duration_s: 30, intensity: 0.5 }; assert!(e.risk_level() < 0.5); assert!(e.is_safe_for_prod()); }
    #[test] fn test_crash_risk() { let e = ChaosExperiment { name: "crash".into(), fault_type: FaultType::ServiceCrash, target_service: "core".into(), duration_s: 10, intensity: 1.0 }; assert!(!e.is_safe_for_prod()); }
    #[test] fn test_report() { let e = ChaosExperiment { name: "t".into(), fault_type: FaultType::NetworkLatency, target_service: "s".into(), duration_s: 10, intensity: 0.3 }; let r = ChaosReport { experiment: e, result: ExperimentResult::Passed, recovery_time_ms: 500, data_loss: false }; assert!(r.resilience_score() > 0.5); }
}
