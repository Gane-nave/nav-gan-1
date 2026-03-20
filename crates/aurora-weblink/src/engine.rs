/// WebLink: browser, render, touch, scroll, bookmark
/// Phase 994

#[derive(Debug, Clone)]
pub struct WebLink {
    pub browser_ok: bool,
    pub render_ok: bool,
    pub touch_ok: bool,
    pub scroll_ok: bool,
    pub bookmark_ok: bool,
}

impl Default for WebLink {
    fn default() -> Self {
        Self::new()
    }
}

impl WebLink {
    pub fn new() -> Self {
        Self {
            browser_ok: true,
            render_ok: true,
            touch_ok: true,
            scroll_ok: true,
            bookmark_ok: true,
        }
    }

    pub fn display_ok(&self) -> bool {
        self.browser_ok && self.render_ok
    }

    pub fn interaction_ok(&self) -> bool {
        self.touch_ok && self.scroll_ok && self.bookmark_ok
    }

    pub fn all_ok(&self) -> bool {
        self.display_ok() && self.interaction_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.browser_ok || !self.render_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.browser_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let c = WebLink::new();
        assert!(c.display_ok());
    }

    #[test]
    fn test_interaction() {
        let c = WebLink::new();
        assert!(c.interaction_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WebLink::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = WebLink::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_browser() {
        let mut c = WebLink::new();
        c.browser_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = WebLink::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
