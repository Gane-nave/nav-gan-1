/// Brake wear monitoring: pad life, rotor condition, wear prediction
/// Phase 170

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WearLevel {
    New,
    Good,
    Fair,
    Low,
    Critical,
}

#[derive(Debug, Clone)]
pub struct BrakeWear {
    pub pad_thickness_mm: f64,
    pub new_thickness_mm: f64,
    pub rotor_thickness_mm: f64,
    pub min_rotor_mm: f64,
    pub km_driven: f64,
}

impl Default for BrakeWear {
    fn default() -> Self {
        Self::new()
    }
}

impl BrakeWear {
    pub fn new() -> Self {
        Self {
            pad_thickness_mm: 12.0,
            new_thickness_mm: 12.0,
            rotor_thickness_mm: 28.0,
            min_rotor_mm: 22.0,
            km_driven: 0.0,
        }
    }

    pub fn pad_life_pct(&self) -> f64 {
        (self.pad_thickness_mm / self.new_thickness_mm * 100.0).clamp(0.0, 100.0)
    }

    pub fn wear_level(&self) -> WearLevel {
        let life = self.pad_life_pct();
        if life > 80.0 {
            WearLevel::New
        } else if life > 50.0 {
            WearLevel::Good
        } else if life > 25.0 {
            WearLevel::Fair
        } else if life > 10.0 {
            WearLevel::Low
        } else {
            WearLevel::Critical
        }
    }

    pub fn rotor_ok(&self) -> bool {
        self.rotor_thickness_mm > self.min_rotor_mm
    }

    pub fn needs_service(&self) -> bool {
        matches!(self.wear_level(), WearLevel::Low | WearLevel::Critical) || !self.rotor_ok()
    }

    pub fn estimated_km_remaining(&self) -> f64 {
        if self.km_driven <= 0.0 || self.pad_thickness_mm >= self.new_thickness_mm {
            return f64::MAX;
        }
        let wear_rate = (self.new_thickness_mm - self.pad_thickness_mm) / self.km_driven;
        if wear_rate <= 0.0 {
            return f64::MAX;
        }
        (self.pad_thickness_mm - 2.0).max(0.0) / wear_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_pad() {
        let b = BrakeWear::new();
        assert_eq!(b.wear_level(), WearLevel::New);
    }

    #[test]
    fn test_worn_pad() {
        let mut b = BrakeWear::new();
        b.pad_thickness_mm = 2.0;
        assert_eq!(b.wear_level(), WearLevel::Low);
    }

    #[test]
    fn test_critical() {
        let mut b = BrakeWear::new();
        b.pad_thickness_mm = 0.5;
        assert_eq!(b.wear_level(), WearLevel::Critical);
    }

    #[test]
    fn test_rotor_ok() {
        let b = BrakeWear::new();
        assert!(b.rotor_ok());
    }

    #[test]
    fn test_rotor_worn() {
        let mut b = BrakeWear::new();
        b.rotor_thickness_mm = 20.0;
        assert!(!b.rotor_ok());
    }

    #[test]
    fn test_needs_service() {
        let mut b = BrakeWear::new();
        b.pad_thickness_mm = 1.0;
        assert!(b.needs_service());
    }

    #[test]
    fn test_pad_life() {
        let b = BrakeWear::new();
        assert!((b.pad_life_pct() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_km_remaining() {
        let mut b = BrakeWear::new();
        b.pad_thickness_mm = 6.0;
        b.km_driven = 30000.0;
        assert!(b.estimated_km_remaining() > 0.0);
    }
}
