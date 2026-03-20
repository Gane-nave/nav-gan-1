/// Vertical integration: own map engine, GNSS processing, routing stack.
#[derive(Debug, Clone, PartialEq)]
pub enum IntegrationLevel { FullyOwned, PartiallyOwned, ThirdParty, OpenSource }
#[derive(Debug, Clone)]
pub struct StackComponent { pub name: String, pub level: IntegrationLevel, pub criticality: f64, pub replacement_months: u32 }
impl StackComponent {
    pub fn ownership_score(&self) -> f64 { match self.level { IntegrationLevel::FullyOwned => 1.0, IntegrationLevel::PartiallyOwned => 0.6, IntegrationLevel::OpenSource => 0.3, IntegrationLevel::ThirdParty => 0.0 } }
    pub fn risk_score(&self) -> f64 { (1.0 - self.ownership_score()) * self.criticality.clamp(0.0, 1.0) }
}
#[derive(Debug, Clone)]
pub struct VerticalStack { pub components: Vec<StackComponent> }
impl Default for VerticalStack {
    fn default() -> Self { Self::new() }
}
impl VerticalStack {
    pub fn new() -> Self { Self { components: Vec::new() } }
    pub fn add(&mut self, c: StackComponent) { self.components.push(c); }
    pub fn overall_ownership(&self) -> f64 { if self.components.is_empty() { 0.0 } else { self.components.iter().map(|c| c.ownership_score() * c.criticality).sum::<f64>() / self.components.iter().map(|c| c.criticality).sum::<f64>().max(0.001) } }
    pub fn dependency_risk(&self) -> f64 { if self.components.is_empty() { 0.0 } else { self.components.iter().map(|c| c.risk_score()).sum::<f64>() / self.components.len() as f64 } }
    pub fn third_party_count(&self) -> usize { self.components.iter().filter(|c| c.level == IntegrationLevel::ThirdParty).count() }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_owned() { let c = StackComponent { name: "Map".into(), level: IntegrationLevel::FullyOwned, criticality: 1.0, replacement_months: 24 }; assert_eq!(c.ownership_score(), 1.0); assert_eq!(c.risk_score(), 0.0); }
    #[test] fn test_third_party() { let c = StackComponent { name: "Tiles".into(), level: IntegrationLevel::ThirdParty, criticality: 0.8, replacement_months: 12 }; assert!(c.risk_score() > 0.0); }
    #[test] fn test_stack() { let mut s = VerticalStack::new(); s.add(StackComponent { name: "GNSS".into(), level: IntegrationLevel::FullyOwned, criticality: 1.0, replacement_months: 36 }); assert!(s.overall_ownership() > 0.9); assert_eq!(s.third_party_count(), 0); }
}
