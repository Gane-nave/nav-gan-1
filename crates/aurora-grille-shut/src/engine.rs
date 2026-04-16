/// Active grille shutter: airflow control, drag reduction, thermal management
/// Phase 359

#[derive(Debug, Clone)]
pub struct GrilleShutter {
    pub position_pct: f64,
    pub motor_ok: bool,
    pub temp_controlled: bool,
    pub speed_controlled: bool,
    pub stuck: bool,
}

impl Default for GrilleShutter {
    fn default() -> Self {
        Self::new()
    }
}

impl GrilleShutter {
    pub fn new() -> Self {
        Self {
            position_pct: 50.0,
            motor_ok: true,
            temp_controlled: true,
            speed_controlled: true,
            stuck: false,
        }
    }

    pub fn fully_open(&self) -> bool {
        self.position_pct > 95.0
    }

    pub fn fully_closed(&self) -> bool {
        self.position_pct < 5.0
    }

    pub fn functioning(&self) -> bool {
        self.motor_ok && !self.stuck
    }

    pub fn drag_reducing(&self) -> bool {
        self.position_pct < 30.0
    }

    pub fn health_score(&self) -> f64 {
        if self.stuck {
            return 0.0;
        }
        if !self.motor_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_full_open() {
        let g = GrilleShutter::new();
        assert!(!g.fully_open());
    }

    #[test]
    fn test_not_closed() {
        let g = GrilleShutter::new();
        assert!(!g.fully_closed());
    }

    #[test]
    fn test_functioning() {
        let g = GrilleShutter::new();
        assert!(g.functioning());
    }

    #[test]
    fn test_not_reducing() {
        let g = GrilleShutter::new();
        assert!(!g.drag_reducing());
    }

    #[test]
    fn test_stuck() {
        let mut g = GrilleShutter::new();
        g.stuck = true;
        assert!(!g.functioning());
    }

    #[test]
    fn test_health() {
        let g = GrilleShutter::new();
        assert!((g.health_score() - 100.0).abs() < 0.1);
    }
}
