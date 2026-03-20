/// Seatbelt tensioner: pre-tensioner, load limiter, buckle status
/// Phase 226

#[derive(Debug, Clone)]
pub struct SeatbeltTensioner {
    pub position: String,
    pub buckled: bool,
    pub pretensioner_armed: bool,
    pub pretensioner_fired: bool,
    pub load_limiter_ok: bool,
    pub webbing_wear_pct: f64,
}

impl Default for SeatbeltTensioner {
    fn default() -> Self {
        Self::new()
    }
}

impl SeatbeltTensioner {
    pub fn new() -> Self {
        Self {
            position: "driver".into(),
            buckled: true,
            pretensioner_armed: true,
            pretensioner_fired: false,
            load_limiter_ok: true,
            webbing_wear_pct: 5.0,
        }
    }

    pub fn system_ok(&self) -> bool {
        self.pretensioner_armed && !self.pretensioner_fired && self.load_limiter_ok
    }

    pub fn webbing_ok(&self) -> bool {
        self.webbing_wear_pct < 30.0
    }

    pub fn needs_replacement(&self) -> bool {
        self.pretensioner_fired || !self.load_limiter_ok || self.webbing_wear_pct > 50.0
    }

    pub fn warning_active(&self) -> bool {
        !self.buckled
    }

    pub fn health_score(&self) -> f64 {
        if self.pretensioner_fired {
            return 0.0;
        }
        let mut score: f64 = 100.0;
        if !self.pretensioner_armed {
            score -= 40.0;
        }
        if !self.load_limiter_ok {
            score -= 30.0;
        }
        if !self.webbing_ok() {
            score -= 20.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_ok() {
        let s = SeatbeltTensioner::new();
        assert!(s.system_ok());
    }

    #[test]
    fn test_webbing_ok() {
        let s = SeatbeltTensioner::new();
        assert!(s.webbing_ok());
    }

    #[test]
    fn test_no_replacement() {
        let s = SeatbeltTensioner::new();
        assert!(!s.needs_replacement());
    }

    #[test]
    fn test_no_warning() {
        let s = SeatbeltTensioner::new();
        assert!(!s.warning_active());
    }

    #[test]
    fn test_fired() {
        let mut s = SeatbeltTensioner::new();
        s.pretensioner_fired = true;
        assert!(s.needs_replacement());
    }

    #[test]
    fn test_health() {
        let s = SeatbeltTensioner::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
