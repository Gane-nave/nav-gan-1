/// RNN engine: cell, gate, memory, sequence, predict
/// Phase 1017

#[derive(Debug, Clone)]
pub struct RnnEngine {
    pub cell_ok: bool,
    pub gate_ok: bool,
    pub memory_ok: bool,
    pub sequence_ok: bool,
    pub predict_ok: bool,
}

impl Default for RnnEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl RnnEngine {
    pub fn new() -> Self {
        Self {
            cell_ok: true,
            gate_ok: true,
            memory_ok: true,
            sequence_ok: true,
            predict_ok: true,
        }
    }

    pub fn recurrence_ok(&self) -> bool {
        self.cell_ok && self.gate_ok && self.memory_ok
    }

    pub fn output_ok(&self) -> bool {
        self.sequence_ok && self.predict_ok
    }

    pub fn all_ok(&self) -> bool {
        self.recurrence_ok() && self.output_ok()
    }

    pub fn needs_reset(&self) -> bool {
        !self.memory_ok || !self.cell_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.cell_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recurrence() {
        let c = RnnEngine::new();
        assert!(c.recurrence_ok());
    }

    #[test]
    fn test_output() {
        let c = RnnEngine::new();
        assert!(c.output_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RnnEngine::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_reset() {
        let c = RnnEngine::new();
        assert!(!c.needs_reset());
    }

    #[test]
    fn test_memory() {
        let mut c = RnnEngine::new();
        c.memory_ok = false;
        assert!(c.needs_reset());
    }

    #[test]
    fn test_health() {
        let c = RnnEngine::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
