/// Backup manager: schedule, snapshot, verify, restore, rotate
/// Phase 1084

#[derive(Debug, Clone)]
pub struct BackupMgr {
    pub schedule_ok: bool,
    pub snapshot_ok: bool,
    pub verify_ok: bool,
    pub restore_ok: bool,
    pub rotate_ok: bool,
}

impl Default for BackupMgr {
    fn default() -> Self {
        Self::new()
    }
}

impl BackupMgr {
    pub fn new() -> Self {
        Self {
            schedule_ok: true,
            snapshot_ok: true,
            verify_ok: true,
            restore_ok: true,
            rotate_ok: true,
        }
    }

    pub fn backup_ok(&self) -> bool {
        self.schedule_ok && self.snapshot_ok && self.verify_ok
    }

    pub fn recovery_ok(&self) -> bool {
        self.restore_ok && self.rotate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.backup_ok() && self.recovery_ok()
    }

    pub fn needs_verify(&self) -> bool {
        !self.snapshot_ok || !self.verify_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.schedule_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup() {
        let c = BackupMgr::new();
        assert!(c.backup_ok());
    }

    #[test]
    fn test_recovery() {
        let c = BackupMgr::new();
        assert!(c.recovery_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BackupMgr::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_verify() {
        let c = BackupMgr::new();
        assert!(!c.needs_verify());
    }

    #[test]
    fn test_snapshot() {
        let mut c = BackupMgr::new();
        c.snapshot_ok = false;
        assert!(c.needs_verify());
    }

    #[test]
    fn test_health() {
        let c = BackupMgr::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
