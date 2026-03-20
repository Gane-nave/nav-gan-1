/// Data flywheel and competitive moat engine.
#[derive(Debug, Clone)]
pub struct DataFlywheel {
    pub user_count: u64,
    pub data_points_per_day: u64,
    pub data_quality_score: f64,
    pub partner_count: u32,
    pub exclusive_data_sources: u32,
}
impl Default for DataFlywheel {
    fn default() -> Self { Self::new() }
}
impl DataFlywheel {
    pub fn new() -> Self {
        Self { user_count: 0, data_points_per_day: 0, data_quality_score: 0.0, partner_count: 0, exclusive_data_sources: 0 }
    }
    pub fn network_effect_multiplier(&self) -> f64 {
        if self.user_count == 0 { return 1.0; }
        (1.0 + (self.user_count as f64).ln() * 0.1).min(5.0)
    }
    pub fn switching_cost(&self, history_months: u32, personalization_depth: f64) -> f64 {
        let t = (history_months as f64 / 24.0).min(1.0);
        let p = personalization_depth.clamp(0.0, 1.0);
        (t * 0.4 + p * 0.6).clamp(0.0, 1.0)
    }
    pub fn moat_strength(&self) -> f64 {
        let d = self.data_quality_score.clamp(0.0, 1.0) * 0.3;
        let n = (self.network_effect_multiplier() / 5.0) * 0.3;
        let p = (self.partner_count as f64 / 50.0).min(1.0) * 0.2;
        let e = (self.exclusive_data_sources as f64 / 10.0).min(1.0) * 0.2;
        (d + n + p + e).clamp(0.0, 1.0)
    }
    pub fn flywheel_velocity(&self) -> f64 {
        (self.data_points_per_day as f64).ln().max(0.0) * self.data_quality_score * self.network_effect_multiplier()
    }
}
#[derive(Debug, Clone)]
pub struct LockInMetrics {
    pub accumulated_history_days: u32,
    pub personalized_routes: u32,
    pub community_contributions: u32,
    pub integrated_services: u32,
}
impl LockInMetrics {
    pub fn stickiness_score(&self) -> f64 {
        let h = (self.accumulated_history_days as f64 / 365.0).min(1.0) * 0.25;
        let r = (self.personalized_routes as f64 / 100.0).min(1.0) * 0.25;
        let c = (self.community_contributions as f64 / 50.0).min(1.0) * 0.25;
        let s = (self.integrated_services as f64 / 10.0).min(1.0) * 0.25;
        (h + r + c + s).clamp(0.0, 1.0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_empty_flywheel() { let fw = DataFlywheel::new(); assert_eq!(fw.network_effect_multiplier(), 1.0); }
    #[test] fn test_network_grows() { let fw = DataFlywheel { user_count: 1_000_000, ..DataFlywheel::new() }; assert!(fw.network_effect_multiplier() > 1.0); }
    #[test] fn test_switching_cost() { let fw = DataFlywheel::new(); assert!(fw.switching_cost(24, 1.0) > fw.switching_cost(0, 0.0)); }
    #[test] fn test_moat_range() { let fw = DataFlywheel { user_count: 1_000_000, data_points_per_day: 10_000_000, data_quality_score: 0.9, partner_count: 30, exclusive_data_sources: 5 }; assert!(fw.moat_strength() > 0.0 && fw.moat_strength() <= 1.0); }
    #[test] fn test_stickiness() { let m = LockInMetrics { accumulated_history_days: 365, personalized_routes: 100, community_contributions: 50, integrated_services: 10 }; assert!((m.stickiness_score() - 1.0).abs() < 0.01); }
}
