/// Cylinder head: gasket integrity, port flow, combustion chamber
/// Phase 317

#[derive(Debug, Clone)]
pub struct CylinderHead {
    pub gasket_ok: bool,
    pub head_temp_c: f64,
    pub coolant_leak: bool,
    pub port_flow_cfm: f64,
    pub warped: bool,
}

impl Default for CylinderHead {
    fn default() -> Self {
        Self::new()
    }
}

impl CylinderHead {
    pub fn new() -> Self {
        Self {
            gasket_ok: true,
            head_temp_c: 95.0,
            coolant_leak: false,
            port_flow_cfm: 200.0,
            warped: false,
        }
    }

    pub fn integrity_ok(&self) -> bool {
        self.gasket_ok && !self.coolant_leak && !self.warped
    }

    pub fn overheating(&self) -> bool {
        self.head_temp_c > 120.0
    }

    pub fn flow_ok(&self) -> bool {
        self.port_flow_cfm > 150.0
    }

    pub fn needs_replacement(&self) -> bool {
        self.warped || !self.gasket_ok
    }

    pub fn health_score(&self) -> f64 {
        if self.warped {
            return 0.0;
        }
        if !self.gasket_ok {
            return 10.0;
        }
        if self.coolant_leak {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integrity() {
        let c = CylinderHead::new();
        assert!(c.integrity_ok());
    }

    #[test]
    fn test_not_hot() {
        let c = CylinderHead::new();
        assert!(!c.overheating());
    }

    #[test]
    fn test_flow() {
        let c = CylinderHead::new();
        assert!(c.flow_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = CylinderHead::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_warped() {
        let mut c = CylinderHead::new();
        c.warped = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = CylinderHead::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
