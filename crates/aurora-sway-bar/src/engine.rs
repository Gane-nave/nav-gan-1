/// Sway bar monitoring: end link wear, bushing condition, roll control
/// Phase 200

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BushingCondition {
    Good,
    Worn,
    Cracked,
    Missing,
}

#[derive(Debug, Clone)]
pub struct SwayBar {
    pub bushing_condition: BushingCondition,
    pub end_link_play_mm: f64,
    pub clunking_over_bumps: bool,
    pub roll_stiffness_nm_deg: f64,
}

impl Default for SwayBar {
    fn default() -> Self {
        Self::new()
    }
}

impl SwayBar {
    pub fn new() -> Self {
        Self {
            bushing_condition: BushingCondition::Good,
            end_link_play_mm: 0.5,
            clunking_over_bumps: false,
            roll_stiffness_nm_deg: 150.0,
        }
    }

    pub fn bushings_ok(&self) -> bool {
        self.bushing_condition == BushingCondition::Good
    }

    pub fn end_links_ok(&self) -> bool {
        self.end_link_play_mm < 3.0
    }

    pub fn needs_service(&self) -> bool {
        !self.bushings_ok() || !self.end_links_ok() || self.clunking_over_bumps
    }

    pub fn roll_control_pct(&self) -> f64 {
        (self.roll_stiffness_nm_deg / 200.0 * 100.0).clamp(0.0, 100.0)
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if !self.bushings_ok() {
            score -= 30.0;
        }
        if !self.end_links_ok() {
            score -= 25.0;
        }
        if self.clunking_over_bumps {
            score -= 20.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_healthy() {
        let s = SwayBar::new();
        assert!(!s.needs_service());
    }

    #[test]
    fn test_bushings_ok() {
        let s = SwayBar::new();
        assert!(s.bushings_ok());
    }

    #[test]
    fn test_end_links_ok() {
        let s = SwayBar::new();
        assert!(s.end_links_ok());
    }

    #[test]
    fn test_roll_control() {
        let s = SwayBar::new();
        assert!(s.roll_control_pct() > 70.0);
    }

    #[test]
    fn test_worn_bushings() {
        let mut s = SwayBar::new();
        s.bushing_condition = BushingCondition::Worn;
        assert!(s.needs_service());
    }

    #[test]
    fn test_health() {
        let s = SwayBar::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
