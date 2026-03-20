/// Door panel: trim, speaker grille, handle, pocket
/// Phase 778

#[derive(Debug, Clone)]
pub struct DoorPanel {
    pub trim_ok: bool,
    pub grille_ok: bool,
    pub handle_ok: bool,
    pub pocket_ok: bool,
    pub clip_ok: bool,
}

impl Default for DoorPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl DoorPanel {
    pub fn new() -> Self {
        Self {
            trim_ok: true,
            grille_ok: true,
            handle_ok: true,
            pocket_ok: true,
            clip_ok: true,
        }
    }

    pub fn appearance_ok(&self) -> bool {
        self.trim_ok && self.grille_ok
    }

    pub fn function_ok(&self) -> bool {
        self.handle_ok && self.pocket_ok && self.clip_ok
    }

    pub fn all_ok(&self) -> bool {
        self.appearance_ok() && self.function_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.trim_ok || !self.clip_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.clip_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_appearance() {
        let c = DoorPanel::new();
        assert!(c.appearance_ok());
    }

    #[test]
    fn test_function() {
        let c = DoorPanel::new();
        assert!(c.function_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DoorPanel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = DoorPanel::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_clip() {
        let mut c = DoorPanel::new();
        c.clip_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = DoorPanel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
