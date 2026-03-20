/// ball joint: pivot, articulate, bear, seal, inspect
/// Phase 1201

#[derive(Debug, Clone)]
pub struct BallJoint {
    pub pivot_ok: bool,
    pub articulate_ok: bool,
    pub bear_ok: bool,
    pub seal_ok: bool,
    pub inspect_ok: bool,
}

impl Default for BallJoint {
    fn default() -> Self {
        Self::new()
    }
}

impl BallJoint {
    pub fn new() -> Self {
        Self {
            pivot_ok: true,
            articulate_ok: true,
            bear_ok: true,
            seal_ok: true,
            inspect_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.pivot_ok && self.articulate_ok && self.bear_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.seal_ok && self.inspect_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.pivot_ok || !self.articulate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.pivot_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = BallJoint::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = BallJoint::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BallJoint::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = BallJoint::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = BallJoint::new();
        c.pivot_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = BallJoint::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
