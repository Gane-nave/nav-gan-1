/// Weight distribution analysis for vehicle stability
/// Phase 132: Monitors load balance, center of gravity, tipping risk

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoadZone {
    FrontLeft,
    FrontRight,
    RearLeft,
    RearRight,
    Center,
    Roof,
}

impl LoadZone {
    pub fn is_front(&self) -> bool {
        matches!(self, LoadZone::FrontLeft | LoadZone::FrontRight)
    }

    pub fn is_rear(&self) -> bool {
        matches!(self, LoadZone::RearLeft | LoadZone::RearRight)
    }

    pub fn is_left(&self) -> bool {
        matches!(self, LoadZone::FrontLeft | LoadZone::RearLeft)
    }

    pub fn height_factor(&self) -> f64 {
        match self {
            LoadZone::Roof => 2.0,
            LoadZone::Center => 1.0,
            _ => 0.8,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoadItem {
    pub zone: LoadZone,
    pub weight_kg: f64,
    pub secured: bool,
}

impl LoadItem {
    pub fn new(zone: LoadZone, weight_kg: f64, secured: bool) -> Self {
        Self {
            zone,
            weight_kg,
            secured,
        }
    }

    pub fn tipping_contribution(&self) -> f64 {
        self.weight_kg * self.zone.height_factor()
    }
}

#[derive(Debug, Clone)]
pub struct WeightDistribution {
    pub vehicle_base_kg: f64,
    pub max_payload_kg: f64,
    pub items: Vec<LoadItem>,
}

impl WeightDistribution {
    pub fn new(base_kg: f64, max_payload: f64) -> Self {
        Self {
            vehicle_base_kg: base_kg,
            max_payload_kg: max_payload,
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: LoadItem) {
        self.items.push(item);
    }

    pub fn total_payload_kg(&self) -> f64 {
        self.items.iter().map(|i| i.weight_kg).sum()
    }

    pub fn total_weight_kg(&self) -> f64 {
        self.vehicle_base_kg + self.total_payload_kg()
    }

    pub fn is_overloaded(&self) -> bool {
        self.total_payload_kg() > self.max_payload_kg
    }

    pub fn overload_percent(&self) -> f64 {
        if self.max_payload_kg <= 0.0 {
            return 0.0;
        }
        let ratio = self.total_payload_kg() / self.max_payload_kg * 100.0;
        if ratio > 100.0 {
            ratio - 100.0
        } else {
            0.0
        }
    }

    pub fn front_weight_kg(&self) -> f64 {
        self.items
            .iter()
            .filter(|i| i.zone.is_front())
            .map(|i| i.weight_kg)
            .sum()
    }

    pub fn rear_weight_kg(&self) -> f64 {
        self.items
            .iter()
            .filter(|i| i.zone.is_rear())
            .map(|i| i.weight_kg)
            .sum()
    }

    pub fn left_weight_kg(&self) -> f64 {
        self.items
            .iter()
            .filter(|i| i.zone.is_left())
            .map(|i| i.weight_kg)
            .sum()
    }

    pub fn front_rear_ratio(&self) -> f64 {
        let rear = self.rear_weight_kg();
        if rear <= 0.0 {
            return f64::MAX;
        }
        self.front_weight_kg() / rear
    }

    pub fn balance_score(&self) -> f64 {
        let total = self.total_payload_kg();
        if total <= 0.0 {
            return 100.0;
        }
        let front_pct = self.front_weight_kg() / total * 100.0;
        let ideal = 50.0;
        let deviation = (front_pct - ideal).abs();
        (100.0 - deviation * 2.0).max(0.0)
    }

    pub fn all_secured(&self) -> bool {
        self.items.iter().all(|i| i.secured)
    }

    pub fn unsecured_count(&self) -> usize {
        self.items.iter().filter(|i| !i.secured).count()
    }

    pub fn tipping_risk(&self) -> f64 {
        let total_contribution: f64 = self.items.iter().map(|i| i.tipping_contribution()).sum();
        let max_safe = self.max_payload_kg * 1.5;
        if max_safe <= 0.0 {
            return 0.0;
        }
        let risk: f64 = (total_contribution / max_safe * 100.0).min(100.0);
        risk.max(0.0)
    }

    pub fn roof_load_kg(&self) -> f64 {
        self.items
            .iter()
            .filter(|i| i.zone == LoadZone::Roof)
            .map(|i| i.weight_kg)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_zone_front() {
        assert!(LoadZone::FrontLeft.is_front());
        assert!(!LoadZone::RearLeft.is_front());
    }

    #[test]
    fn test_height_factor() {
        assert_eq!(LoadZone::Roof.height_factor(), 2.0);
        assert_eq!(LoadZone::Center.height_factor(), 1.0);
    }

    #[test]
    fn test_tipping_contribution() {
        let item = LoadItem::new(LoadZone::Roof, 50.0, true);
        assert!((item.tipping_contribution() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_total_weight() {
        let mut wd = WeightDistribution::new(1500.0, 500.0);
        wd.add_item(LoadItem::new(LoadZone::RearLeft, 100.0, true));
        wd.add_item(LoadItem::new(LoadZone::RearRight, 100.0, true));
        assert!((wd.total_weight_kg() - 1700.0).abs() < 0.1);
    }

    #[test]
    fn test_overloaded() {
        let mut wd = WeightDistribution::new(1500.0, 200.0);
        wd.add_item(LoadItem::new(LoadZone::Center, 300.0, true));
        assert!(wd.is_overloaded());
    }

    #[test]
    fn test_not_overloaded() {
        let mut wd = WeightDistribution::new(1500.0, 500.0);
        wd.add_item(LoadItem::new(LoadZone::Center, 200.0, true));
        assert!(!wd.is_overloaded());
    }

    #[test]
    fn test_front_rear_ratio() {
        let mut wd = WeightDistribution::new(1500.0, 500.0);
        wd.add_item(LoadItem::new(LoadZone::FrontLeft, 100.0, true));
        wd.add_item(LoadItem::new(LoadZone::RearRight, 100.0, true));
        assert!((wd.front_rear_ratio() - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_balance_score() {
        let mut wd = WeightDistribution::new(1500.0, 500.0);
        wd.add_item(LoadItem::new(LoadZone::FrontLeft, 100.0, true));
        wd.add_item(LoadItem::new(LoadZone::RearRight, 100.0, true));
        assert!(wd.balance_score() >= 90.0);
    }

    #[test]
    fn test_unsecured() {
        let mut wd = WeightDistribution::new(1500.0, 500.0);
        wd.add_item(LoadItem::new(LoadZone::Center, 50.0, false));
        wd.add_item(LoadItem::new(LoadZone::Center, 50.0, true));
        assert_eq!(wd.unsecured_count(), 1);
        assert!(!wd.all_secured());
    }

    #[test]
    fn test_roof_load() {
        let mut wd = WeightDistribution::new(1500.0, 500.0);
        wd.add_item(LoadItem::new(LoadZone::Roof, 75.0, true));
        assert!((wd.roof_load_kg() - 75.0).abs() < 0.1);
    }

    #[test]
    fn test_tipping_risk() {
        let mut wd = WeightDistribution::new(1500.0, 500.0);
        wd.add_item(LoadItem::new(LoadZone::Roof, 400.0, true));
        assert!(wd.tipping_risk() > 50.0);
    }

    #[test]
    fn test_overload_percent() {
        let mut wd = WeightDistribution::new(1500.0, 200.0);
        wd.add_item(LoadItem::new(LoadZone::Center, 300.0, true));
        assert!((wd.overload_percent() - 50.0).abs() < 0.1);
    }
}
