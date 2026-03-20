/// Attention steering: dimming, highlighting, focus management.
#[derive(Debug, Clone)]
pub struct AttentionTarget {
    pub element_name: String,
    pub importance: f64,
    pub highlight_intensity: f64,
    pub dim_surroundings: bool,
}
#[derive(Debug, Clone)]
pub struct AttentionManager {
    pub targets: Vec<AttentionTarget>,
    pub global_dim_level: f64,
}
impl Default for AttentionManager {
    fn default() -> Self {
        Self::new()
    }
}
impl AttentionManager {
    pub fn new() -> Self {
        Self {
            targets: Vec::new(),
            global_dim_level: 0.0,
        }
    }
    pub fn set_focus(&mut self, target: AttentionTarget) {
        self.global_dim_level = if target.dim_surroundings { 0.5 } else { 0.0 };
        self.targets.push(target);
    }
    pub fn clear_focus(&mut self) {
        self.targets.clear();
        self.global_dim_level = 0.0;
    }
    pub fn current_focus(&self) -> Option<&AttentionTarget> {
        self.targets.last()
    }
    pub fn element_opacity(&self, element_name: &str) -> f64 {
        if let Some(focus) = self.current_focus() {
            if focus.element_name == element_name {
                1.0
            } else {
                1.0 - self.global_dim_level
            }
        } else {
            1.0
        }
    }
    pub fn is_focused(&self) -> bool {
        !self.targets.is_empty()
    }
    pub fn focus_strength(&self) -> f64 {
        self.current_focus()
            .map_or(0.0, |t| t.highlight_intensity.clamp(0.0, 1.0))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_focus() {
        let mut m = AttentionManager::new();
        m.set_focus(AttentionTarget {
            element_name: "turn".into(),
            importance: 1.0,
            highlight_intensity: 0.9,
            dim_surroundings: true,
        });
        assert!(m.is_focused());
        assert_eq!(m.element_opacity("turn"), 1.0);
        assert!(m.element_opacity("map") < 1.0);
    }
    #[test]
    fn test_clear() {
        let mut m = AttentionManager::new();
        m.set_focus(AttentionTarget {
            element_name: "x".into(),
            importance: 1.0,
            highlight_intensity: 0.5,
            dim_surroundings: true,
        });
        m.clear_focus();
        assert!(!m.is_focused());
        assert_eq!(m.element_opacity("x"), 1.0);
    }
    #[test]
    fn test_no_focus() {
        let m = AttentionManager::new();
        assert_eq!(m.element_opacity("any"), 1.0);
        assert_eq!(m.focus_strength(), 0.0);
    }
}
