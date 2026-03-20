/// dsrc radio: channel, power, modulate, demodulate, scan
/// Phase 1126

#[derive(Debug, Clone)]
pub struct DsrcRadio {
    pub channel_ok: bool,
    pub power_ok: bool,
    pub modulate_ok: bool,
    pub demodulate_ok: bool,
    pub scan_ok: bool,
}

impl Default for DsrcRadio {
    fn default() -> Self {
        Self::new()
    }
}

impl DsrcRadio {
    pub fn new() -> Self {
        Self {
            channel_ok: true,
            power_ok: true,
            modulate_ok: true,
            demodulate_ok: true,
            scan_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.channel_ok && self.power_ok && self.modulate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.demodulate_ok && self.scan_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.channel_ok || !self.power_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.channel_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = DsrcRadio::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DsrcRadio::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DsrcRadio::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DsrcRadio::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DsrcRadio::new();
        c.channel_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DsrcRadio::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
