/// IP & data protection: patents, exclusive agreements, contracts.
#[derive(Debug, Clone, PartialEq)]
pub enum ProtectionType {
    Patent,
    TradeSecret,
    ExclusiveDataAgreement,
    LongTermContract,
    Copyright,
}
#[derive(Debug, Clone)]
pub struct ProtectionAsset {
    pub name: String,
    pub protection_type: ProtectionType,
    pub coverage_regions: Vec<String>,
    pub expiry_months: u32,
    pub strength_score: f64,
}
impl ProtectionAsset {
    pub fn is_active(&self) -> bool {
        self.expiry_months > 0
    }
    pub fn time_weighted_strength(&self) -> f64 {
        self.strength_score.clamp(0.0, 1.0) * (self.expiry_months as f64 / 120.0).min(1.0)
    }
}
#[derive(Debug, Clone)]
pub struct IpPortfolio {
    pub assets: Vec<ProtectionAsset>,
}
impl Default for IpPortfolio {
    fn default() -> Self {
        Self::new()
    }
}
impl IpPortfolio {
    pub fn new() -> Self {
        Self { assets: Vec::new() }
    }
    pub fn add_asset(&mut self, a: ProtectionAsset) {
        self.assets.push(a);
    }
    pub fn total_coverage(&self) -> f64 {
        let a: Vec<_> = self.assets.iter().filter(|a| a.is_active()).collect();
        if a.is_empty() {
            0.0
        } else {
            a.iter().map(|x| x.time_weighted_strength()).sum::<f64>() / a.len() as f64
        }
    }
    pub fn patents_count(&self) -> usize {
        self.assets
            .iter()
            .filter(|a| a.protection_type == ProtectionType::Patent)
            .count()
    }
    pub fn exclusive_count(&self) -> usize {
        self.assets
            .iter()
            .filter(|a| a.protection_type == ProtectionType::ExclusiveDataAgreement)
            .count()
    }
    pub fn protection_score(&self) -> f64 {
        let p = (self.patents_count() as f64 / 20.0).min(1.0) * 0.3;
        let e = (self.exclusive_count() as f64 / 10.0).min(1.0) * 0.3;
        let c = self.total_coverage() * 0.4;
        (p + e + c).clamp(0.0, 1.0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_active() {
        let a = ProtectionAsset {
            name: "P".into(),
            protection_type: ProtectionType::Patent,
            coverage_regions: vec!["US".into()],
            expiry_months: 120,
            strength_score: 0.9,
        };
        assert!(a.is_active());
        assert!((a.time_weighted_strength() - 0.9).abs() < 0.01);
    }
    #[test]
    fn test_expired() {
        let a = ProtectionAsset {
            name: "O".into(),
            protection_type: ProtectionType::Patent,
            coverage_regions: vec![],
            expiry_months: 0,
            strength_score: 0.9,
        };
        assert!(!a.is_active());
    }
    #[test]
    fn test_portfolio() {
        let mut p = IpPortfolio::new();
        p.add_asset(ProtectionAsset {
            name: "P1".into(),
            protection_type: ProtectionType::Patent,
            coverage_regions: vec!["US".into()],
            expiry_months: 60,
            strength_score: 0.8,
        });
        assert_eq!(p.patents_count(), 1);
        assert!(p.protection_score() > 0.0);
    }
    #[test]
    fn test_empty() {
        let p = IpPortfolio::new();
        assert_eq!(p.total_coverage(), 0.0);
    }
}
