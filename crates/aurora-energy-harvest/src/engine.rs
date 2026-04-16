/// Energy harvesting: kinetic recovery, thermal recovery, vibration harvesting
/// Phase 165

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HarvestSource {
    Kinetic,
    Thermal,
    Vibration,
    Solar,
    Wind,
}

impl HarvestSource {
    pub fn max_power_watts(&self) -> f64 {
        match self {
            HarvestSource::Kinetic => 5000.0,
            HarvestSource::Thermal => 200.0,
            HarvestSource::Vibration => 50.0,
            HarvestSource::Solar => 300.0,
            HarvestSource::Wind => 100.0,
        }
    }

    pub fn efficiency_pct(&self) -> f64 {
        match self {
            HarvestSource::Kinetic => 85.0,
            HarvestSource::Thermal => 15.0,
            HarvestSource::Vibration => 5.0,
            HarvestSource::Solar => 20.0,
            HarvestSource::Wind => 10.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnergyHarvester {
    pub source: HarvestSource,
    pub active: bool,
    pub current_power_w: f64,
    pub total_harvested_wh: f64,
}

impl EnergyHarvester {
    pub fn new(source: HarvestSource) -> Self {
        Self {
            source,
            active: true,
            current_power_w: 0.0,
            total_harvested_wh: 0.0,
        }
    }

    pub fn effective_power_w(&self) -> f64 {
        if self.active {
            self.current_power_w.min(self.source.max_power_watts())
        } else {
            0.0
        }
    }
}

#[derive(Debug, Clone)]
pub struct HarvestSystem {
    pub harvesters: Vec<EnergyHarvester>,
}

impl Default for HarvestSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl HarvestSystem {
    pub fn new() -> Self {
        Self {
            harvesters: vec![
                EnergyHarvester::new(HarvestSource::Kinetic),
                EnergyHarvester::new(HarvestSource::Thermal),
                EnergyHarvester::new(HarvestSource::Vibration),
            ],
        }
    }

    pub fn total_power_w(&self) -> f64 {
        self.harvesters.iter().map(|h| h.effective_power_w()).sum()
    }

    pub fn total_harvested_wh(&self) -> f64 {
        self.harvesters.iter().map(|h| h.total_harvested_wh).sum()
    }

    pub fn active_sources(&self) -> usize {
        self.harvesters.iter().filter(|h| h.active).count()
    }

    pub fn range_extension_km(&self) -> f64 {
        self.total_harvested_wh() / 1000.0 * 5.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_power() {
        assert!(
            HarvestSource::Kinetic.max_power_watts() > HarvestSource::Vibration.max_power_watts()
        );
    }

    #[test]
    fn test_efficiency() {
        assert!(HarvestSource::Kinetic.efficiency_pct() > 50.0);
    }

    #[test]
    fn test_effective_power() {
        let mut h = EnergyHarvester::new(HarvestSource::Kinetic);
        h.current_power_w = 3000.0;
        assert!((h.effective_power_w() - 3000.0).abs() < 0.1);
    }

    #[test]
    fn test_inactive() {
        let mut h = EnergyHarvester::new(HarvestSource::Thermal);
        h.active = false;
        assert!((h.effective_power_w() - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_system_total() {
        let s = HarvestSystem::new();
        assert!((s.total_power_w() - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_active_sources() {
        let s = HarvestSystem::new();
        assert_eq!(s.active_sources(), 3);
    }

    #[test]
    fn test_range_extension() {
        let mut s = HarvestSystem::new();
        s.harvesters[0].total_harvested_wh = 2000.0;
        assert!(s.range_extension_km() > 9.0);
    }

    #[test]
    fn test_total_harvested() {
        let mut s = HarvestSystem::new();
        s.harvesters[0].total_harvested_wh = 100.0;
        s.harvesters[1].total_harvested_wh = 50.0;
        assert!((s.total_harvested_wh() - 150.0).abs() < 0.1);
    }
}
