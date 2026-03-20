/// Warranty manager: claim, coverage, extend, transfer
/// Phase 970

#[derive(Debug, Clone)]
pub struct WarrantyMgr {
    pub claim_ok: bool,
    pub coverage_ok: bool,
    pub extend_ok: bool,
    pub transfer_ok: bool,
    pub database_ok: bool,
}

impl Default for WarrantyMgr {
    fn default() -> Self {
        Self::new()
    }
}

impl WarrantyMgr {
    pub fn new() -> Self {
        Self {
            claim_ok: true,
            coverage_ok: true,
            extend_ok: true,
            transfer_ok: true,
            database_ok: true,
        }
    }

    pub fn claims_ok(&self) -> bool {
        self.claim_ok && self.coverage_ok && self.database_ok
    }

    pub fn management_ok(&self) -> bool {
        self.extend_ok && self.transfer_ok
    }

    pub fn all_ok(&self) -> bool {
        self.claims_ok() && self.management_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.database_ok || !self.coverage_ok
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
    fn test_claims() {
        let c = WarrantyMgr::new();
        assert!(c.claims_ok());
    }

    #[test]
    fn test_management() {
        let c = WarrantyMgr::new();
        assert!(c.management_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WarrantyMgr::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = WarrantyMgr::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_database() {
        let mut c = WarrantyMgr::new();
        c.database_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = WarrantyMgr::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
