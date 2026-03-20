/// Revenue model: B2C, B2B, Fleet, Marketplace, Ads.
#[derive(Debug, Clone, PartialEq)]
pub enum RevenueStream { B2cPremium, B2bApi, FleetManagement, Marketplace, LocationAds }
#[derive(Debug, Clone)]
pub struct RevenueSource { pub stream: RevenueStream, pub monthly_revenue: f64, pub user_count: u64, pub margin_pct: f64 }
impl RevenueSource {
    pub fn arpu(&self) -> f64 { if self.user_count == 0 { 0.0 } else { self.monthly_revenue / self.user_count as f64 } }
    pub fn monthly_profit(&self) -> f64 { self.monthly_revenue * self.margin_pct }
}
#[derive(Debug, Clone)]
pub struct RevenueModel { pub sources: Vec<RevenueSource> }
impl Default for RevenueModel {
    fn default() -> Self { Self::new() }
}
impl RevenueModel {
    pub fn new() -> Self { Self { sources: Vec::new() } }
    pub fn add_source(&mut self, s: RevenueSource) { self.sources.push(s); }
    pub fn total_monthly_revenue(&self) -> f64 { self.sources.iter().map(|s| s.monthly_revenue).sum() }
    pub fn total_monthly_profit(&self) -> f64 { self.sources.iter().map(|s| s.monthly_profit()).sum() }
    pub fn blended_arpu(&self) -> f64 { let u: u64 = self.sources.iter().map(|s| s.user_count).sum(); if u == 0 { 0.0 } else { self.total_monthly_revenue() / u as f64 } }
    pub fn revenue_concentration(&self) -> f64 { let t = self.total_monthly_revenue(); if t <= 0.0 { 0.0 } else { self.sources.iter().map(|s| s.monthly_revenue).fold(0.0_f64, f64::max) / t } }
    pub fn project_annual(&self, g: f64) -> f64 { let m = self.total_monthly_revenue(); (0..12).fold(0.0, |a, i| a + m * (1.0 + g).powi(i)) }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_arpu() { let s = RevenueSource { stream: RevenueStream::B2cPremium, monthly_revenue: 10000.0, user_count: 1000, margin_pct: 0.7 }; assert!((s.arpu()-10.0).abs()<0.01); }
    #[test] fn test_model() { let mut m = RevenueModel::new(); m.add_source(RevenueSource { stream: RevenueStream::B2cPremium, monthly_revenue: 50000.0, user_count: 5000, margin_pct: 0.8 }); assert!(m.total_monthly_revenue() > 0.0); }
    #[test] fn test_annual() { let mut m = RevenueModel::new(); m.add_source(RevenueSource { stream: RevenueStream::B2cPremium, monthly_revenue: 10000.0, user_count: 1000, margin_pct: 0.7 }); assert!(m.project_annual(0.05) > 120000.0); }
    #[test] fn test_empty() { let m = RevenueModel::new(); assert_eq!(m.total_monthly_revenue(), 0.0); assert_eq!(m.blended_arpu(), 0.0); }
}
