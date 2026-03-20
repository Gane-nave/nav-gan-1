/// Invisible interaction: UI appears only when needed, disappears when not.
#[derive(Debug, Clone, PartialEq)]
pub enum UiElementState {
    Hidden,
    FadingIn,
    Visible,
    FadingOut,
}
#[derive(Debug, Clone)]
pub struct SmartElement {
    pub name: String,
    pub state: UiElementState,
    pub relevance: f64,
    pub show_threshold: f64,
    pub hide_delay_ms: u64,
    pub visible_since_ms: u64,
}
impl SmartElement {
    pub fn should_show(&self) -> bool {
        self.relevance >= self.show_threshold
    }
    pub fn should_hide(&self, elapsed_ms: u64) -> bool {
        !self.should_show() && elapsed_ms > self.hide_delay_ms
    }
    pub fn next_state(&self, elapsed_ms: u64) -> UiElementState {
        match self.state {
            UiElementState::Hidden => {
                if self.should_show() {
                    UiElementState::FadingIn
                } else {
                    UiElementState::Hidden
                }
            }
            UiElementState::FadingIn => UiElementState::Visible,
            UiElementState::Visible => {
                if self.should_hide(elapsed_ms) {
                    UiElementState::FadingOut
                } else {
                    UiElementState::Visible
                }
            }
            UiElementState::FadingOut => UiElementState::Hidden,
        }
    }
}
#[derive(Debug, Clone)]
pub struct InvisibleUiManager {
    pub elements: Vec<SmartElement>,
}
impl Default for InvisibleUiManager {
    fn default() -> Self {
        Self::new()
    }
}
impl InvisibleUiManager {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
    pub fn add_element(&mut self, e: SmartElement) {
        self.elements.push(e);
    }
    pub fn visible_count(&self) -> usize {
        self.elements
            .iter()
            .filter(|e| e.state == UiElementState::Visible || e.state == UiElementState::FadingIn)
            .count()
    }
    pub fn invisibility_ratio(&self) -> f64 {
        if self.elements.is_empty() {
            1.0
        } else {
            1.0 - self.visible_count() as f64 / self.elements.len() as f64
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_show() {
        let e = SmartElement {
            name: "turn".into(),
            state: UiElementState::Hidden,
            relevance: 0.9,
            show_threshold: 0.5,
            hide_delay_ms: 2000,
            visible_since_ms: 0,
        };
        assert!(e.should_show());
        assert_eq!(e.next_state(0), UiElementState::FadingIn);
    }
    #[test]
    fn test_hide() {
        let e = SmartElement {
            name: "info".into(),
            state: UiElementState::Visible,
            relevance: 0.1,
            show_threshold: 0.5,
            hide_delay_ms: 1000,
            visible_since_ms: 0,
        };
        assert!(e.should_hide(2000));
    }
    #[test]
    fn test_invisibility() {
        let mut m = InvisibleUiManager::new();
        m.add_element(SmartElement {
            name: "a".into(),
            state: UiElementState::Hidden,
            relevance: 0.1,
            show_threshold: 0.5,
            hide_delay_ms: 1000,
            visible_since_ms: 0,
        });
        assert!((m.invisibility_ratio() - 1.0).abs() < 0.01);
    }
}
