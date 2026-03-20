/// Subwoofer: low frequency reproduction, enclosure, ported/sealed
/// Phase 431

#[derive(Debug, Clone)]
pub struct Subwoofer {
    pub output_db: f64,
    pub freq_hz: f64,
    pub distortion_pct: f64,
    pub enclosure_ok: bool,
    pub cone_ok: bool,
}

impl Default for Subwoofer {
    fn default() -> Self {
        Self::new()
    }
}

impl Subwoofer {
    pub fn new() -> Self {
        Self {
            output_db: 95.0,
            freq_hz: 40.0,
            distortion_pct: 2.0,
            enclosure_ok: true,
            cone_ok: true,
        }
    }

    pub fn powerful(&self) -> bool {
        self.output_db > 90.0
    }

    pub fn clean(&self) -> bool {
        self.distortion_pct < 5.0
    }

    pub fn all_ok(&self) -> bool {
        self.enclosure_ok && self.cone_ok && self.clean()
    }

    pub fn needs_service(&self) -> bool {
        !self.cone_ok || !self.enclosure_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.cone_ok {
            return 0.0;
        }
        if !self.enclosure_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_powerful() {
        let s = Subwoofer::new();
        assert!(s.powerful());
    }

    #[test]
    fn test_clean() {
        let s = Subwoofer::new();
        assert!(s.clean());
    }

    #[test]
    fn test_all_ok() {
        let s = Subwoofer::new();
        assert!(s.all_ok());
    }

    #[test]
    fn test_no_service() {
        let s = Subwoofer::new();
        assert!(!s.needs_service());
    }

    #[test]
    fn test_blown() {
        let mut s = Subwoofer::new();
        s.cone_ok = false;
        assert!(s.needs_service());
    }

    #[test]
    fn test_health() {
        let s = Subwoofer::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
