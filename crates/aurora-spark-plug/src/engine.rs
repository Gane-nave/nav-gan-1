/// Spark plug monitoring: ignition quality, misfire detection, gap measurement
/// Phase 172

#[derive(Debug, Clone)]
pub struct SparkPlug {
    pub cylinder: u8,
    pub gap_mm: f64,
    pub target_gap_mm: f64,
    pub wear_pct: f64,
    pub misfire_count: u64,
}

impl SparkPlug {
    pub fn new(cylinder: u8) -> Self {
        Self {
            cylinder,
            gap_mm: 0.8,
            target_gap_mm: 0.8,
            wear_pct: 0.0,
            misfire_count: 0,
        }
    }

    pub fn gap_ok(&self) -> bool {
        (self.gap_mm - self.target_gap_mm).abs() < 0.15
    }

    pub fn needs_replacement(&self) -> bool {
        self.wear_pct > 80.0 || !self.gap_ok() || self.misfire_count > 100
    }

    pub fn ignition_quality_pct(&self) -> f64 {
        let gap_score = if self.gap_ok() { 50.0 } else { 20.0 };
        let wear_score = (100.0 - self.wear_pct) / 100.0 * 50.0;
        gap_score + wear_score
    }
}

#[derive(Debug, Clone)]
pub struct IgnitionSystem {
    pub plugs: Vec<SparkPlug>,
}

impl Default for IgnitionSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl IgnitionSystem {
    pub fn new() -> Self {
        Self {
            plugs: (1..=4).map(SparkPlug::new).collect(),
        }
    }

    pub fn all_ok(&self) -> bool {
        self.plugs.iter().all(|p| !p.needs_replacement())
    }

    pub fn total_misfires(&self) -> u64 {
        self.plugs.iter().map(|p| p.misfire_count).sum()
    }

    pub fn avg_quality(&self) -> f64 {
        if self.plugs.is_empty() {
            return 0.0;
        }
        let total: f64 = self.plugs.iter().map(|p| p.ignition_quality_pct()).sum();
        total / self.plugs.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gap_ok() {
        let p = SparkPlug::new(1);
        assert!(p.gap_ok());
    }

    #[test]
    fn test_gap_bad() {
        let mut p = SparkPlug::new(1);
        p.gap_mm = 1.2;
        assert!(!p.gap_ok());
    }

    #[test]
    fn test_no_replacement() {
        let p = SparkPlug::new(1);
        assert!(!p.needs_replacement());
    }

    #[test]
    fn test_needs_replacement() {
        let mut p = SparkPlug::new(1);
        p.wear_pct = 90.0;
        assert!(p.needs_replacement());
    }

    #[test]
    fn test_quality() {
        let p = SparkPlug::new(1);
        assert!(p.ignition_quality_pct() > 90.0);
    }

    #[test]
    fn test_system_ok() {
        let s = IgnitionSystem::new();
        assert!(s.all_ok());
    }

    #[test]
    fn test_avg_quality() {
        let s = IgnitionSystem::new();
        assert!(s.avg_quality() > 90.0);
    }

    #[test]
    fn test_total_misfires() {
        let s = IgnitionSystem::new();
        assert_eq!(s.total_misfires(), 0);
    }
}
