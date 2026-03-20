/// Investor packaging: metrics, story, demo, roadmap.
#[derive(Debug, Clone)]
pub struct InvestorMetric { pub name: String, pub value: f64, pub unit: String, pub trend_pct: f64 }
impl InvestorMetric {
    pub fn is_positive_trend(&self) -> bool { self.trend_pct > 0.0 }
    pub fn formatted(&self) -> String { format!("{}: {:.1}{} ({:+.1}%)", self.name, self.value, self.unit, self.trend_pct) }
}
#[derive(Debug, Clone, PartialEq)]
pub enum RoadmapPhase { Completed, InProgress, Planned, Stretch }
#[derive(Debug, Clone)]
pub struct RoadmapItem { pub name: String, pub phase: RoadmapPhase, pub quarter: String, pub impact_score: f64 }
#[derive(Debug, Clone)]
pub struct InvestorPackage { pub metrics: Vec<InvestorMetric>, pub roadmap: Vec<RoadmapItem>, pub tam_usd: f64, pub sam_usd: f64, pub current_arr: f64 }
impl Default for InvestorPackage {
    fn default() -> Self { Self { metrics: Vec::new(), roadmap: Vec::new(), tam_usd: 0.0, sam_usd: 0.0, current_arr: 0.0 } }
}
impl InvestorPackage {
    pub fn new() -> Self { Self::default() }
    pub fn add_metric(&mut self, m: InvestorMetric) { self.metrics.push(m); }
    pub fn add_roadmap(&mut self, r: RoadmapItem) { self.roadmap.push(r); }
    pub fn positive_metrics_pct(&self) -> f64 { if self.metrics.is_empty() { 0.0 } else { self.metrics.iter().filter(|m| m.is_positive_trend()).count() as f64 / self.metrics.len() as f64 } }
    pub fn roadmap_completion(&self) -> f64 { if self.roadmap.is_empty() { 0.0 } else { self.roadmap.iter().filter(|r| r.phase == RoadmapPhase::Completed).count() as f64 / self.roadmap.len() as f64 } }
    pub fn market_penetration(&self) -> f64 { if self.sam_usd <= 0.0 { 0.0 } else { (self.current_arr / self.sam_usd).clamp(0.0, 1.0) } }
    pub fn investment_readiness(&self) -> f64 {
        let metrics = self.positive_metrics_pct() * 0.3;
        let roadmap = self.roadmap_completion() * 0.2;
        let market = self.market_penetration().min(0.1) * 10.0 * 0.2;
        let arr = (self.current_arr / 1_000_000.0).min(1.0) * 0.3;
        (metrics + roadmap + market + arr).clamp(0.0, 1.0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_metric() { let m = InvestorMetric { name: "DAU".into(), value: 50000.0, unit: "users".into(), trend_pct: 15.0 }; assert!(m.is_positive_trend()); }
    #[test] fn test_roadmap() { let mut p = InvestorPackage::new(); p.add_roadmap(RoadmapItem { name: "MVP".into(), phase: RoadmapPhase::Completed, quarter: "Q1".into(), impact_score: 0.9 }); p.add_roadmap(RoadmapItem { name: "Scale".into(), phase: RoadmapPhase::InProgress, quarter: "Q2".into(), impact_score: 0.8 }); assert!((p.roadmap_completion()-0.5).abs()<0.01); }
    #[test] fn test_readiness() { let mut p = InvestorPackage::new(); p.tam_usd = 10_000_000_000.0; p.sam_usd = 1_000_000_000.0; p.current_arr = 500_000.0; p.add_metric(InvestorMetric { name: "Growth".into(), value: 30.0, unit: "%".into(), trend_pct: 5.0 }); assert!(p.investment_readiness() > 0.0); }
    #[test] fn test_empty() { let p = InvestorPackage::new(); assert_eq!(p.positive_metrics_pct(), 0.0); }
}
