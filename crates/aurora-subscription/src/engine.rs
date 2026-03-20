/// Subscription: feature unlock, trial, payment, entitlement
/// Phase 895

#[derive(Debug, Clone)]
pub struct Subscription {
    pub feature_ok: bool,
    pub trial_ok: bool,
    pub payment_ok: bool,
    pub entitle_ok: bool,
    pub sync_ok: bool,
}

impl Default for Subscription {
    fn default() -> Self {
        Self::new()
    }
}

impl Subscription {
    pub fn new() -> Self {
        Self {
            feature_ok: true,
            trial_ok: true,
            payment_ok: true,
            entitle_ok: true,
            sync_ok: true,
        }
    }

    pub fn access_ok(&self) -> bool {
        self.feature_ok && self.entitle_ok && self.sync_ok
    }

    pub fn billing_ok(&self) -> bool {
        self.trial_ok && self.payment_ok
    }

    pub fn all_ok(&self) -> bool {
        self.access_ok() && self.billing_ok()
    }

    pub fn needs_renewal(&self) -> bool {
        !self.payment_ok || !self.entitle_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.payment_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access() {
        let c = Subscription::new();
        assert!(c.access_ok());
    }

    #[test]
    fn test_billing() {
        let c = Subscription::new();
        assert!(c.billing_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Subscription::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_renewal() {
        let c = Subscription::new();
        assert!(!c.needs_renewal());
    }

    #[test]
    fn test_payment() {
        let mut c = Subscription::new();
        c.payment_ok = false;
        assert!(c.needs_renewal());
    }

    #[test]
    fn test_health() {
        let c = Subscription::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
