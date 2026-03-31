/// aurora-dash-export: dash export
/// Phase 2446

#[derive(Debug, Clone)]
pub struct DashExport {
    pub pdf_ok: bool,
    pub png_ok: bool,
    pub csv_ok: bool,
    pub json_ok: bool,
    pub clipboard_ok: bool,
}

impl Default for DashExport {
    fn default() -> Self {
        Self::new()
    }
}

impl DashExport {
    pub fn new() -> Self {
        Self {
            pdf_ok: true,
            png_ok: true,
            csv_ok: true,
            json_ok: true,
            clipboard_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.pdf_ok && self.png_ok && self.csv_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.json_ok && self.clipboard_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.pdf_ok || !self.png_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.pdf_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = DashExport::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DashExport::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DashExport::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DashExport::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DashExport::new();
        c.pdf_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DashExport::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = DashExport::default();
        assert!(c.all_ok());
    }
}
