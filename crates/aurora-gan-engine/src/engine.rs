/// GAN engine: generator, discriminator, loss, train, sample
/// Phase 1018

#[derive(Debug, Clone)]
pub struct GanEngine {
    pub generator_ok: bool,
    pub discriminator_ok: bool,
    pub loss_ok: bool,
    pub train_ok: bool,
    pub sample_ok: bool,
}

impl Default for GanEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl GanEngine {
    pub fn new() -> Self {
        Self {
            generator_ok: true,
            discriminator_ok: true,
            loss_ok: true,
            train_ok: true,
            sample_ok: true,
        }
    }

    pub fn adversarial_ok(&self) -> bool {
        self.generator_ok && self.discriminator_ok && self.loss_ok
    }

    pub fn production_ok(&self) -> bool {
        self.train_ok && self.sample_ok
    }

    pub fn all_ok(&self) -> bool {
        self.adversarial_ok() && self.production_ok()
    }

    pub fn needs_balance(&self) -> bool {
        !self.generator_ok || !self.discriminator_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.generator_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adversarial() {
        let c = GanEngine::new();
        assert!(c.adversarial_ok());
    }

    #[test]
    fn test_production() {
        let c = GanEngine::new();
        assert!(c.production_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GanEngine::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_balance() {
        let c = GanEngine::new();
        assert!(!c.needs_balance());
    }

    #[test]
    fn test_generator() {
        let mut c = GanEngine::new();
        c.generator_ok = false;
        assert!(c.needs_balance());
    }

    #[test]
    fn test_health() {
        let c = GanEngine::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
