/// power steer: assist, sense, boost, save, check
/// Phase 1208

#[derive(Debug, Clone)]
pub struct PowerSteer {
    pub assist_ok: bool,
    pub sense_ok: bool,
    pub boost_ok: bool,
    pub save_ok: bool,
    pub check_ok: bool,
}

impl Default for PowerSteer {
    fn default() -> Self {
        Self::new()
    }
}

impl PowerSteer {
    pub fn new() -> Self {
        Self {
            assist_ok: true,
            sense_ok: true,
            boost_ok: true,
            save_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.assist_ok && self.sense_ok && self.boost_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.save_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.assist_ok || !self.sense_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.assist_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = PowerSteer::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = PowerSteer::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PowerSteer::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = PowerSteer::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = PowerSteer::new();
        c.assist_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = PowerSteer::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
