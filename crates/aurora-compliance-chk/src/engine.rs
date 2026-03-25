/// Compliance check: GDPR, CCPA, SOC2, ISO, PCI
/// Phase 1013

#[derive(Debug, Clone)]
pub struct ComplianceChk {
    pub gdpr_ok: bool,
    pub ccpa_ok: bool,
    pub soc2_ok: bool,
    pub iso_ok: bool,
    pub pci_ok: bool,
}

impl Default for ComplianceChk {
    fn default() -> Self {
        Self::new()
    }
}

impl ComplianceChk {
    pub fn new() -> Self {
        Self {
            gdpr_ok: true,
            ccpa_ok: true,
            soc2_ok: true,
            iso_ok: true,
            pci_ok: true,
        }
    }

    pub fn privacy_ok(&self) -> bool {
        self.gdpr_ok && self.ccpa_ok
    }

    pub fn security_ok(&self) -> bool {
        self.soc2_ok && self.iso_ok && self.pci_ok
    }

    pub fn all_ok(&self) -> bool {
        self.privacy_ok() && self.security_ok()
    }

    pub fn needs_audit(&self) -> bool {
        !self.soc2_ok || !self.iso_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.gdpr_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privacy() {
        let c = ComplianceChk::new();
        assert!(c.privacy_ok());
    }

    #[test]
    fn test_security() {
        let c = ComplianceChk::new();
        assert!(c.security_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ComplianceChk::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_audit() {
        let c = ComplianceChk::new();
        assert!(!c.needs_audit());
    }

    #[test]
    fn test_soc2() {
        let mut c = ComplianceChk::new();
        c.soc2_ok = false;
        assert!(c.needs_audit());
    }

    #[test]
    fn test_health() {
        let c = ComplianceChk::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
