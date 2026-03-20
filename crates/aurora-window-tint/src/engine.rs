/// Window tint control: electrochromic tinting, UV protection, privacy
/// Phase 139

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TintLevel {
    Clear,
    Light,
    Medium,
    Dark,
    Privacy,
}

impl TintLevel {
    pub fn transmission_pct(&self) -> f64 {
        match self {
            TintLevel::Clear => 90.0,
            TintLevel::Light => 70.0,
            TintLevel::Medium => 50.0,
            TintLevel::Dark => 20.0,
            TintLevel::Privacy => 5.0,
        }
    }
    pub fn uv_block_pct(&self) -> f64 {
        match self {
            TintLevel::Clear => 30.0,
            TintLevel::Light => 60.0,
            TintLevel::Medium => 80.0,
            TintLevel::Dark => 95.0,
            TintLevel::Privacy => 99.0,
        }
    }
    pub fn legal_for_front(&self) -> bool {
        matches!(self, TintLevel::Clear | TintLevel::Light)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowPosition {
    Windshield,
    FrontLeft,
    FrontRight,
    RearLeft,
    RearRight,
    Rear,
    Sunroof,
}

#[derive(Debug, Clone)]
pub struct SmartWindow {
    pub position: WindowPosition,
    pub tint: TintLevel,
    pub solar_load_wm2: f64,
    pub auto_mode: bool,
}

impl SmartWindow {
    pub fn new(pos: WindowPosition) -> Self {
        Self {
            position: pos,
            tint: TintLevel::Clear,
            solar_load_wm2: 0.0,
            auto_mode: true,
        }
    }
    pub fn recommended_tint(&self) -> TintLevel {
        if self.solar_load_wm2 > 800.0 {
            TintLevel::Dark
        } else if self.solar_load_wm2 > 500.0 {
            TintLevel::Medium
        } else if self.solar_load_wm2 > 200.0 {
            TintLevel::Light
        } else {
            TintLevel::Clear
        }
    }
    pub fn heat_rejection_pct(&self) -> f64 {
        100.0 - self.tint.transmission_pct()
    }
    pub fn cabin_temp_reduction_c(&self) -> f64 {
        self.heat_rejection_pct() * 0.15
    }
    pub fn is_legal(&self) -> bool {
        match self.position {
            WindowPosition::Windshield | WindowPosition::FrontLeft | WindowPosition::FrontRight => {
                self.tint.legal_for_front()
            }
            _ => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TintSystem {
    pub windows: Vec<SmartWindow>,
}
impl Default for TintSystem {
    fn default() -> Self {
        Self::new()
    }
}
impl TintSystem {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
        }
    }
    pub fn add(&mut self, w: SmartWindow) {
        self.windows.push(w);
    }
    pub fn all_legal(&self) -> bool {
        self.windows.iter().all(|w| w.is_legal())
    }
    pub fn total_uv_protection(&self) -> f64 {
        if self.windows.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.windows.iter().map(|w| w.tint.uv_block_pct()).sum();
        sum / self.windows.len() as f64
    }
    pub fn estimated_temp_reduction_c(&self) -> f64 {
        if self.windows.is_empty() {
            return 0.0;
        }
        let sum: f64 = self
            .windows
            .iter()
            .map(|w| w.cabin_temp_reduction_c())
            .sum();
        sum / self.windows.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_transmission() {
        assert!((TintLevel::Medium.transmission_pct() - 50.0).abs() < 0.1);
    }
    #[test]
    fn test_uv_block() {
        assert!(TintLevel::Dark.uv_block_pct() > 90.0);
    }
    #[test]
    fn test_legal_front() {
        assert!(TintLevel::Light.legal_for_front());
        assert!(!TintLevel::Dark.legal_for_front());
    }
    #[test]
    fn test_recommended_tint() {
        let mut w = SmartWindow::new(WindowPosition::Sunroof);
        w.solar_load_wm2 = 900.0;
        assert_eq!(w.recommended_tint(), TintLevel::Dark);
    }
    #[test]
    fn test_heat_rejection() {
        let mut w = SmartWindow::new(WindowPosition::Rear);
        w.tint = TintLevel::Dark;
        assert!(w.heat_rejection_pct() > 70.0);
    }
    #[test]
    fn test_is_legal() {
        let mut w = SmartWindow::new(WindowPosition::FrontLeft);
        w.tint = TintLevel::Dark;
        assert!(!w.is_legal());
    }
    #[test]
    fn test_rear_legal() {
        let mut w = SmartWindow::new(WindowPosition::Rear);
        w.tint = TintLevel::Privacy;
        assert!(w.is_legal());
    }
    #[test]
    fn test_system_legal() {
        let mut s = TintSystem::new();
        s.add(SmartWindow::new(WindowPosition::FrontLeft));
        assert!(s.all_legal());
    }
    #[test]
    fn test_uv_protection() {
        let mut s = TintSystem::new();
        let mut w = SmartWindow::new(WindowPosition::Rear);
        w.tint = TintLevel::Dark;
        s.add(w);
        assert!(s.total_uv_protection() > 90.0);
    }
    #[test]
    fn test_temp_reduction() {
        let mut s = TintSystem::new();
        let mut w = SmartWindow::new(WindowPosition::Sunroof);
        w.tint = TintLevel::Dark;
        s.add(w);
        assert!(s.estimated_temp_reduction_c() > 5.0);
    }
}
