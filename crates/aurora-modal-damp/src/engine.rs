/// Modal damping: structural resonance, panel vibration, tuned mass damper
/// Phase 374

#[derive(Debug, Clone)]
pub struct ModalDamp {
    pub mode_freq_hz: f64,
    pub damping_ratio: f64,
    pub panel_vibration_ok: bool,
    pub tmd_installed: bool,
    pub effective: bool,
}

impl Default for ModalDamp {
    fn default() -> Self {
        Self::new()
    }
}

impl ModalDamp {
    pub fn new() -> Self {
        Self {
            mode_freq_hz: 45.0,
            damping_ratio: 0.05,
            panel_vibration_ok: true,
            tmd_installed: false,
            effective: true,
        }
    }

    pub fn well_damped(&self) -> bool {
        self.damping_ratio > 0.03
    }

    pub fn resonance_risk(&self) -> bool {
        self.mode_freq_hz > 20.0 && self.mode_freq_hz < 80.0 && self.damping_ratio < 0.02
    }

    pub fn panels_ok(&self) -> bool {
        self.panel_vibration_ok
    }

    pub fn needs_tmd(&self) -> bool {
        self.resonance_risk() && !self.tmd_installed
    }

    pub fn health_score(&self) -> f64 {
        if self.resonance_risk() {
            return 30.0;
        }
        if !self.panel_vibration_ok {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damped() {
        let m = ModalDamp::new();
        assert!(m.well_damped());
    }

    #[test]
    fn test_no_resonance() {
        let m = ModalDamp::new();
        assert!(!m.resonance_risk());
    }

    #[test]
    fn test_panels() {
        let m = ModalDamp::new();
        assert!(m.panels_ok());
    }

    #[test]
    fn test_no_tmd() {
        let m = ModalDamp::new();
        assert!(!m.needs_tmd());
    }

    #[test]
    fn test_resonance() {
        let mut m = ModalDamp::new();
        m.damping_ratio = 0.01;
        assert!(m.resonance_risk());
    }

    #[test]
    fn test_health() {
        let m = ModalDamp::new();
        assert!((m.health_score() - 100.0).abs() < 0.1);
    }
}
