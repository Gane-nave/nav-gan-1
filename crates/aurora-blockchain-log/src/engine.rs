/// Blockchain log: hash, chain, verify, immutable, audit
/// Phase 996

#[derive(Debug, Clone)]
pub struct BlockchainLog {
    pub hash_ok: bool,
    pub chain_ok: bool,
    pub verify_ok: bool,
    pub immutable_ok: bool,
    pub audit_ok: bool,
}

impl Default for BlockchainLog {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockchainLog {
    pub fn new() -> Self {
        Self {
            hash_ok: true,
            chain_ok: true,
            verify_ok: true,
            immutable_ok: true,
            audit_ok: true,
        }
    }

    pub fn integrity_ok(&self) -> bool {
        self.hash_ok && self.chain_ok && self.verify_ok
    }

    pub fn compliance_ok(&self) -> bool {
        self.immutable_ok && self.audit_ok
    }

    pub fn all_ok(&self) -> bool {
        self.integrity_ok() && self.compliance_ok()
    }

    pub fn needs_sync(&self) -> bool {
        !self.chain_ok || !self.verify_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.hash_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integrity() {
        let c = BlockchainLog::new();
        assert!(c.integrity_ok());
    }

    #[test]
    fn test_compliance() {
        let c = BlockchainLog::new();
        assert!(c.compliance_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BlockchainLog::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_sync() {
        let c = BlockchainLog::new();
        assert!(!c.needs_sync());
    }

    #[test]
    fn test_chain() {
        let mut c = BlockchainLog::new();
        c.chain_ok = false;
        assert!(c.needs_sync());
    }

    #[test]
    fn test_health() {
        let c = BlockchainLog::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
