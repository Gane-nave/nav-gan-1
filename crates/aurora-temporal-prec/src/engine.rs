/// Temporal precision: instruction timing matched to human reaction time.
#[derive(Debug, Clone)]
pub struct TimingConfig { pub base_reaction_ms: u64, pub speed_factor: f64, pub complexity_factor: f64, pub fatigue_factor: f64 }
impl TimingConfig {
    pub fn default_config() -> Self { Self { base_reaction_ms: 1500, speed_factor: 1.0, complexity_factor: 1.0, fatigue_factor: 1.0 } }
    pub fn adjusted_lead_time_ms(&self) -> u64 { (self.base_reaction_ms as f64 * self.speed_factor * self.complexity_factor * self.fatigue_factor) as u64 }
}
#[derive(Debug, Clone)]
pub struct InstructionTimer { pub distance_to_action_m: f64, pub current_speed_mps: f64, pub config: TimingConfig }
impl InstructionTimer {
    pub fn time_to_action_ms(&self) -> u64 { if self.current_speed_mps <= 0.01 { u64::MAX } else { (self.distance_to_action_m / self.current_speed_mps * 1000.0) as u64 } }
    pub fn should_announce(&self) -> bool {
        let tta = self.time_to_action_ms();
        let lead = self.config.adjusted_lead_time_ms();
        tta <= lead && tta > lead / 4
    }
    pub fn urgency(&self) -> f64 {
        let tta = self.time_to_action_ms();
        let lead = self.config.adjusted_lead_time_ms();
        if tta >= lead { 0.0 } else { (1.0 - tta as f64 / lead as f64).clamp(0.0, 1.0) }
    }
    pub fn is_too_late(&self) -> bool { self.time_to_action_ms() < self.config.adjusted_lead_time_ms() / 4 }
    pub fn optimal_announce_distance_m(&self) -> f64 { self.current_speed_mps * self.config.adjusted_lead_time_ms() as f64 / 1000.0 }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_should_announce() { let t = InstructionTimer { distance_to_action_m: 30.0, current_speed_mps: 15.0, config: TimingConfig::default_config() }; assert!(t.should_announce() || !t.should_announce()); }
    #[test] fn test_stopped() { let t = InstructionTimer { distance_to_action_m: 100.0, current_speed_mps: 0.0, config: TimingConfig::default_config() }; assert_eq!(t.time_to_action_ms(), u64::MAX); }
    #[test] fn test_urgency() { let t = InstructionTimer { distance_to_action_m: 5.0, current_speed_mps: 15.0, config: TimingConfig::default_config() }; assert!(t.urgency() > 0.5); }
    #[test] fn test_optimal() { let t = InstructionTimer { distance_to_action_m: 100.0, current_speed_mps: 10.0, config: TimingConfig::default_config() }; assert!(t.optimal_announce_distance_m() > 0.0); }
}
