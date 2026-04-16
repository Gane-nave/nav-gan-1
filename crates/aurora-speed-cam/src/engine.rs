/// Speed camera: detection, alert, database, update
/// Phase 914

#[derive(Debug, Clone)]
pub struct SpeedCam {
    pub detect_ok: bool,
    pub alert_ok: bool,
    pub database_ok: bool,
    pub update_ok: bool,
    pub legal_ok: bool,
}

impl Default for SpeedCam {
    fn default() -> Self {
        Self::new()
    }
}

impl SpeedCam {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            alert_ok: true,
            database_ok: true,
            update_ok: true,
            legal_ok: true,
        }
    }

    pub fn warning_ok(&self) -> bool {
        self.detect_ok && self.alert_ok && self.legal_ok
    }

    pub fn data_ok(&self) -> bool {
        self.database_ok && self.update_ok
    }

    pub fn all_ok(&self) -> bool {
        self.warning_ok() && self.data_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.database_ok || !self.update_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.database_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_warning() {
        let c = SpeedCam::new();
        assert!(c.warning_ok());
    }

    #[test]
    fn test_data() {
        let c = SpeedCam::new();
        assert!(c.data_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SpeedCam::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = SpeedCam::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_database() {
        let mut c = SpeedCam::new();
        c.database_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = SpeedCam::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
