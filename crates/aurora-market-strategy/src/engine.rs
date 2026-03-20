/// Market penetration strategy: niche targeting, expansion.
#[derive(Debug, Clone, PartialEq)]
pub enum MarketSegment { Logistics, Delivery, EmergencyVehicles, PublicTransit, RideHailing, ConsumerGeneral, AutonomousFleet }
#[derive(Debug, Clone)]
pub struct NicheAnalysis { pub segment: MarketSegment, pub tam_users: u64, pub current_penetration: f64, pub competitive_advantage: f64, pub entry_barrier: f64, pub revenue_potential: f64 }
impl NicheAnalysis {
    pub fn niche_score(&self) -> f64 {
        let o = (1.0 - self.current_penetration) * 0.25;
        let a = self.competitive_advantage.clamp(0.0, 1.0) * 0.3;
        let b = (1.0 - self.entry_barrier.clamp(0.0, 1.0)) * 0.2;
        let r = self.revenue_potential.clamp(0.0, 1.0) * 0.25;
        (o + a + b + r).clamp(0.0, 1.0)
    }
    pub fn addressable_users(&self) -> u64 { ((self.tam_users as f64) * (1.0 - self.current_penetration)) as u64 }
}
#[derive(Debug, Clone)]
pub struct ExpansionPlan { pub niches: Vec<NicheAnalysis> }
impl Default for ExpansionPlan {
    fn default() -> Self { Self::new() }
}
impl ExpansionPlan {
    pub fn new() -> Self { Self { niches: Vec::new() } }
    pub fn add_niche(&mut self, n: NicheAnalysis) { self.niches.push(n); }
    pub fn ranked_niches(&self) -> Vec<&NicheAnalysis> { let mut s: Vec<_> = self.niches.iter().collect(); s.sort_by(|a, b| b.niche_score().partial_cmp(&a.niche_score()).unwrap_or(std::cmp::Ordering::Equal)); s }
    pub fn best_entry_niche(&self) -> Option<&NicheAnalysis> { self.ranked_niches().into_iter().next() }
    pub fn total_addressable(&self) -> u64 { self.niches.iter().map(|n| n.addressable_users()).sum() }
    pub fn expansion_readiness(&self, p: f64) -> f64 { if p < 0.3 { 0.0 } else { ((p - 0.3) / 0.7).clamp(0.0, 1.0) } }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_niche() { let n = NicheAnalysis { segment: MarketSegment::Logistics, tam_users: 100_000, current_penetration: 0.1, competitive_advantage: 0.8, entry_barrier: 0.3, revenue_potential: 0.9 }; assert!(n.niche_score() > 0.5); }
    #[test] fn test_ranked() { let mut p = ExpansionPlan::new(); p.add_niche(NicheAnalysis { segment: MarketSegment::ConsumerGeneral, tam_users: 10_000_000, current_penetration: 0.8, competitive_advantage: 0.3, entry_barrier: 0.9, revenue_potential: 0.5 }); p.add_niche(NicheAnalysis { segment: MarketSegment::Logistics, tam_users: 50_000, current_penetration: 0.05, competitive_advantage: 0.9, entry_barrier: 0.2, revenue_potential: 0.9 }); assert_eq!(p.best_entry_niche().unwrap().segment, MarketSegment::Logistics); }
    #[test] fn test_expand() { let p = ExpansionPlan::new(); assert_eq!(p.expansion_readiness(0.1), 0.0); assert!(p.expansion_readiness(0.8) > 0.5); }
}
