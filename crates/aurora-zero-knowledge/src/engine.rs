/// Zero knowledge: proof, verify, circuit, witness, commit
/// Phase 997

#[derive(Debug, Clone)]
pub struct ZeroKnowledge {
    pub proof_ok: bool,
    pub verify_ok: bool,
    pub circuit_ok: bool,
    pub witness_ok: bool,
    pub commit_ok: bool,
}

impl Default for ZeroKnowledge {
    fn default() -> Self {
        Self::new()
    }
}

impl ZeroKnowledge {
    pub fn new() -> Self {
        Self {
            proof_ok: true,
            verify_ok: true,
            circuit_ok: true,
            witness_ok: true,
            commit_ok: true,
        }
    }

    pub fn generation_ok(&self) -> bool {
        self.proof_ok && self.circuit_ok && self.witness_ok
    }

    pub fn validation_ok(&self) -> bool {
        self.verify_ok && self.commit_ok
    }

    pub fn all_ok(&self) -> bool {
        self.generation_ok() && self.validation_ok()
    }

    pub fn needs_setup(&self) -> bool {
        !self.circuit_ok || !self.witness_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.proof_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generation() {
        let c = ZeroKnowledge::new();
        assert!(c.generation_ok());
    }

    #[test]
    fn test_validation() {
        let c = ZeroKnowledge::new();
        assert!(c.validation_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ZeroKnowledge::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_setup() {
        let c = ZeroKnowledge::new();
        assert!(!c.needs_setup());
    }

    #[test]
    fn test_circuit() {
        let mut c = ZeroKnowledge::new();
        c.circuit_ok = false;
        assert!(c.needs_setup());
    }

    #[test]
    fn test_health() {
        let c = ZeroKnowledge::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
