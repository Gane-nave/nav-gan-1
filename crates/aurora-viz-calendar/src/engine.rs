/// aurora-viz-calendar: viz calendar
/// Phase 2434

#[derive(Debug, Clone)]
pub struct VizCalendar {
    pub render_ok: bool,
    pub date_ok: bool,
    pub event_ok: bool,
    pub color_ok: bool,
    pub select_ok: bool,
}

impl Default for VizCalendar {
    fn default() -> Self {
        Self::new()
    }
}

impl VizCalendar {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            date_ok: true,
            event_ok: true,
            color_ok: true,
            select_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.date_ok && self.event_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.color_ok && self.select_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.render_ok || !self.date_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.render_ok {
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
        let c = VizCalendar::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = VizCalendar::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VizCalendar::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = VizCalendar::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = VizCalendar::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = VizCalendar::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = VizCalendar::default();
        assert!(c.all_ok());
    }
}
