/// Favorites manager: save, organize, sync, share, backup
/// Phase 908

#[derive(Debug, Clone)]
pub struct FavoriteMgr {
    pub save_ok: bool,
    pub organize_ok: bool,
    pub sync_ok: bool,
    pub share_ok: bool,
    pub backup_ok: bool,
}

impl Default for FavoriteMgr {
    fn default() -> Self {
        Self::new()
    }
}

impl FavoriteMgr {
    pub fn new() -> Self {
        Self {
            save_ok: true,
            organize_ok: true,
            sync_ok: true,
            share_ok: true,
            backup_ok: true,
        }
    }

    pub fn management_ok(&self) -> bool {
        self.save_ok && self.organize_ok
    }

    pub fn cloud_ok(&self) -> bool {
        self.sync_ok && self.share_ok && self.backup_ok
    }

    pub fn all_ok(&self) -> bool {
        self.management_ok() && self.cloud_ok()
    }

    pub fn needs_sync(&self) -> bool {
        !self.sync_ok || !self.backup_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sync_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_management() {
        let c = FavoriteMgr::new();
        assert!(c.management_ok());
    }

    #[test]
    fn test_cloud() {
        let c = FavoriteMgr::new();
        assert!(c.cloud_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FavoriteMgr::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_sync() {
        let c = FavoriteMgr::new();
        assert!(!c.needs_sync());
    }

    #[test]
    fn test_sync() {
        let mut c = FavoriteMgr::new();
        c.sync_ok = false;
        assert!(c.needs_sync());
    }

    #[test]
    fn test_health() {
        let c = FavoriteMgr::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
