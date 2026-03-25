/// Neural network: layer, activate, backprop, optimize, infer
/// Phase 1014

#[derive(Debug, Clone)]
pub struct NeuralNet {
    pub layer_ok: bool,
    pub activate_ok: bool,
    pub backprop_ok: bool,
    pub optimize_ok: bool,
    pub infer_ok: bool,
}

impl Default for NeuralNet {
    fn default() -> Self {
        Self::new()
    }
}

impl NeuralNet {
    pub fn new() -> Self {
        Self {
            layer_ok: true,
            activate_ok: true,
            backprop_ok: true,
            optimize_ok: true,
            infer_ok: true,
        }
    }

    pub fn training_ok(&self) -> bool {
        self.layer_ok && self.activate_ok && self.backprop_ok
    }

    pub fn inference_ok(&self) -> bool {
        self.optimize_ok && self.infer_ok
    }

    pub fn all_ok(&self) -> bool {
        self.training_ok() && self.inference_ok()
    }

    pub fn needs_train(&self) -> bool {
        !self.backprop_ok || !self.optimize_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.layer_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_training() {
        let c = NeuralNet::new();
        assert!(c.training_ok());
    }

    #[test]
    fn test_inference() {
        let c = NeuralNet::new();
        assert!(c.inference_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NeuralNet::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_train() {
        let c = NeuralNet::new();
        assert!(!c.needs_train());
    }

    #[test]
    fn test_backprop() {
        let mut c = NeuralNet::new();
        c.backprop_ok = false;
        assert!(c.needs_train());
    }

    #[test]
    fn test_health() {
        let c = NeuralNet::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
