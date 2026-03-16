//! Keyboard navigation — focus management, tab order, shortcut registry.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A focusable UI element in the tab order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusableElement {
    pub id: String,
    pub tab_index: i32,
    pub group: Option<String>,
    pub label: String,
    pub enabled: bool,
    pub visible: bool,
}

impl FocusableElement {
    /// Create a new focusable element.
    pub fn new(id: &str, tab_index: i32, label: &str) -> Self {
        Self {
            id: id.to_string(),
            tab_index,
            group: None,
            label: label.to_string(),
            enabled: true,
            visible: true,
        }
    }

    /// Set the focus group.
    pub fn with_group(mut self, group: &str) -> Self {
        self.group = Some(group.to_string());
        self
    }

    /// Check if this element can receive focus.
    pub fn is_focusable(&self) -> bool {
        self.enabled && self.visible && self.tab_index >= 0
    }
}

/// Focus manager — tracks focus state and tab order.
pub struct FocusManager {
    elements: Vec<FocusableElement>,
    current_index: Option<usize>,
    trap_group: Option<String>,
}

impl FocusManager {
    /// Create a new focus manager.
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            current_index: None,
            trap_group: None,
        }
    }

    /// Register a focusable element.
    pub fn register(&mut self, element: FocusableElement) {
        // Preserve focus on the currently focused element across re-sort
        let focused_id = self
            .current_index
            .and_then(|i| self.elements.get(i))
            .map(|e| e.id.clone());

        self.elements.push(element);
        self.sort_elements();

        // Restore current_index to the new position of the previously focused element
        self.current_index =
            focused_id.and_then(|id| self.elements.iter().position(|e| e.id == id));
    }

    /// Sort elements by tab index.
    fn sort_elements(&mut self) {
        self.elements.sort_by_key(|e| e.tab_index);
    }

    /// Get focusable elements (filtered by trap group if active).
    fn focusable(&self) -> Vec<usize> {
        self.elements
            .iter()
            .enumerate()
            .filter(|(_, e)| {
                if !e.is_focusable() {
                    return false;
                }
                if let Some(trap) = &self.trap_group {
                    e.group.as_ref() == Some(trap)
                } else {
                    true
                }
            })
            .map(|(i, _)| i)
            .collect()
    }

    /// Move focus to the next element.
    pub fn focus_next(&mut self) -> Option<&FocusableElement> {
        let focusable = self.focusable();
        if focusable.is_empty() {
            return None;
        }

        let next = match self.current_index {
            None => focusable[0],
            Some(current) => {
                let pos = focusable.iter().position(|&i| i > current);
                match pos {
                    Some(p) => focusable[p],
                    None => focusable[0], // wrap around
                }
            }
        };

        self.current_index = Some(next);
        self.elements.get(next)
    }

    /// Move focus to the previous element.
    pub fn focus_prev(&mut self) -> Option<&FocusableElement> {
        let focusable = self.focusable();
        if focusable.is_empty() {
            return None;
        }

        let prev = match self.current_index {
            None => *focusable.last().unwrap(),
            Some(current) => {
                let pos = focusable.iter().rposition(|&i| i < current);
                match pos {
                    Some(p) => focusable[p],
                    None => *focusable.last().unwrap(), // wrap around
                }
            }
        };

        self.current_index = Some(prev);
        self.elements.get(prev)
    }

    /// Get the currently focused element.
    pub fn current(&self) -> Option<&FocusableElement> {
        self.current_index.and_then(|i| self.elements.get(i))
    }

    /// Focus a specific element by ID.
    pub fn focus_by_id(&mut self, id: &str) -> Option<&FocusableElement> {
        if let Some(idx) = self.elements.iter().position(|e| e.id == id) {
            if self.elements[idx].is_focusable() {
                self.current_index = Some(idx);
                return self.elements.get(idx);
            }
        }
        None
    }

    /// Set a focus trap — restricts tab navigation to a group.
    pub fn set_trap(&mut self, group: &str) {
        self.trap_group = Some(group.to_string());
        self.current_index = None;
    }

    /// Release the focus trap.
    pub fn release_trap(&mut self) {
        self.trap_group = None;
    }

    /// Number of registered elements.
    pub fn element_count(&self) -> usize {
        self.elements.len()
    }

    /// Number of currently focusable elements.
    pub fn focusable_count(&self) -> usize {
        self.focusable().len()
    }
}

impl Default for FocusManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Keyboard shortcut definition.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyCombo {
    pub key: String,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub meta: bool,
}

impl KeyCombo {
    /// Create a simple key shortcut.
    pub fn key(key: &str) -> Self {
        Self {
            key: key.to_string(),
            ctrl: false,
            alt: false,
            shift: false,
            meta: false,
        }
    }

    /// Add ctrl modifier.
    pub fn with_ctrl(mut self) -> Self {
        self.ctrl = true;
        self
    }

    /// Add alt modifier.
    pub fn with_alt(mut self) -> Self {
        self.alt = true;
        self
    }

    /// Add shift modifier.
    pub fn with_shift(mut self) -> Self {
        self.shift = true;
        self
    }

    /// Human-readable shortcut string.
    pub fn display(&self) -> String {
        let mut parts = Vec::new();
        if self.ctrl {
            parts.push("Ctrl");
        }
        if self.alt {
            parts.push("Alt");
        }
        if self.shift {
            parts.push("Shift");
        }
        if self.meta {
            parts.push("Meta");
        }
        parts.push(&self.key);
        parts.join("+")
    }
}

/// Keyboard shortcut action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutAction {
    pub id: String,
    pub description: String,
    pub combo: KeyCombo,
    pub category: String,
    pub enabled: bool,
}

/// Shortcut registry — manages keyboard shortcuts.
pub struct ShortcutRegistry {
    shortcuts: HashMap<String, ShortcutAction>,
}

impl ShortcutRegistry {
    /// Create a new shortcut registry.
    pub fn new() -> Self {
        Self {
            shortcuts: HashMap::new(),
        }
    }

    /// Register a shortcut.
    pub fn register(&mut self, action: ShortcutAction) {
        self.shortcuts.insert(action.id.clone(), action);
    }

    /// Find a shortcut by key combo.
    pub fn find_by_combo(&self, combo: &KeyCombo) -> Option<&ShortcutAction> {
        self.shortcuts
            .values()
            .find(|a| a.enabled && a.combo == *combo)
    }

    /// Get all shortcuts in a category.
    pub fn by_category(&self, category: &str) -> Vec<&ShortcutAction> {
        self.shortcuts
            .values()
            .filter(|a| a.category == category)
            .collect()
    }

    /// Get all registered shortcuts.
    pub fn all(&self) -> Vec<&ShortcutAction> {
        self.shortcuts.values().collect()
    }

    /// Number of shortcuts.
    pub fn len(&self) -> usize {
        self.shortcuts.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.shortcuts.is_empty()
    }
}

impl Default for ShortcutRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_focusable_element() {
        let el = FocusableElement::new("btn1", 0, "Start");
        assert!(el.is_focusable());

        let hidden = FocusableElement {
            visible: false,
            ..el.clone()
        };
        assert!(!hidden.is_focusable());

        let negative = FocusableElement {
            tab_index: -1,
            ..el
        };
        assert!(!negative.is_focusable());
    }

    #[test]
    fn test_focus_manager_tab_order() {
        let mut mgr = FocusManager::new();
        mgr.register(FocusableElement::new("a", 1, "First"));
        mgr.register(FocusableElement::new("b", 2, "Second"));
        mgr.register(FocusableElement::new("c", 3, "Third"));

        let first = mgr.focus_next().unwrap();
        assert_eq!(first.id, "a");

        let second = mgr.focus_next().unwrap();
        assert_eq!(second.id, "b");

        let third = mgr.focus_next().unwrap();
        assert_eq!(third.id, "c");

        // Wrap around
        let wrapped = mgr.focus_next().unwrap();
        assert_eq!(wrapped.id, "a");
    }

    #[test]
    fn test_focus_manager_prev() {
        let mut mgr = FocusManager::new();
        mgr.register(FocusableElement::new("a", 1, "First"));
        mgr.register(FocusableElement::new("b", 2, "Second"));

        // No current → goes to last
        let last = mgr.focus_prev().unwrap();
        assert_eq!(last.id, "b");

        let prev = mgr.focus_prev().unwrap();
        assert_eq!(prev.id, "a");

        // Wrap around
        let wrapped = mgr.focus_prev().unwrap();
        assert_eq!(wrapped.id, "b");
    }

    #[test]
    fn test_focus_by_id() {
        let mut mgr = FocusManager::new();
        mgr.register(FocusableElement::new("a", 1, "First"));
        mgr.register(FocusableElement::new("b", 2, "Second"));

        let el = mgr.focus_by_id("b").unwrap();
        assert_eq!(el.id, "b");
        assert_eq!(mgr.current().unwrap().id, "b");
    }

    #[test]
    fn test_focus_trap() {
        let mut mgr = FocusManager::new();
        mgr.register(FocusableElement::new("a", 1, "Outside").with_group("main"));
        mgr.register(FocusableElement::new("b", 2, "Dialog OK").with_group("dialog"));
        mgr.register(FocusableElement::new("c", 3, "Dialog Cancel").with_group("dialog"));

        mgr.set_trap("dialog");
        assert_eq!(mgr.focusable_count(), 2);

        let first = mgr.focus_next().unwrap();
        assert_eq!(first.id, "b");

        let second = mgr.focus_next().unwrap();
        assert_eq!(second.id, "c");

        // Wrap within trap
        let wrapped = mgr.focus_next().unwrap();
        assert_eq!(wrapped.id, "b");

        mgr.release_trap();
        assert_eq!(mgr.focusable_count(), 3);
    }

    #[test]
    fn test_key_combo() {
        let combo = KeyCombo::key("S").with_ctrl();
        assert_eq!(combo.display(), "Ctrl+S");

        let combo2 = KeyCombo::key("N").with_ctrl().with_shift();
        assert_eq!(combo2.display(), "Ctrl+Shift+N");
    }

    #[test]
    fn test_shortcut_registry() {
        let mut registry = ShortcutRegistry::new();
        registry.register(ShortcutAction {
            id: "zoom_in".to_string(),
            description: "Zoom in".to_string(),
            combo: KeyCombo::key("+").with_ctrl(),
            category: "map".to_string(),
            enabled: true,
        });
        registry.register(ShortcutAction {
            id: "zoom_out".to_string(),
            description: "Zoom out".to_string(),
            combo: KeyCombo::key("-").with_ctrl(),
            category: "map".to_string(),
            enabled: true,
        });

        assert_eq!(registry.len(), 2);
        assert_eq!(registry.by_category("map").len(), 2);

        let found = registry.find_by_combo(&KeyCombo::key("+").with_ctrl());
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "zoom_in");
    }

    #[test]
    fn test_shortcut_disabled() {
        let mut registry = ShortcutRegistry::new();
        registry.register(ShortcutAction {
            id: "test".to_string(),
            description: "Test".to_string(),
            combo: KeyCombo::key("T"),
            category: "nav".to_string(),
            enabled: false,
        });

        let found = registry.find_by_combo(&KeyCombo::key("T"));
        assert!(found.is_none());
    }
}
