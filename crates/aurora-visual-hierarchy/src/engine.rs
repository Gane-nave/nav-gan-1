/// Perceptual hierarchy: one dominant element per moment, depth layering.
#[derive(Debug, Clone, PartialEq)]
pub enum VisualLayer {
    Primary,
    Secondary,
    Tertiary,
    Background,
}
#[derive(Debug, Clone)]
pub struct VisualElement {
    pub name: String,
    pub layer: VisualLayer,
    pub importance: f64,
    pub opacity: f64,
    pub z_order: i32,
}
impl VisualElement {
    pub fn effective_weight(&self) -> f64 {
        let layer_w = match self.layer {
            VisualLayer::Primary => 1.0,
            VisualLayer::Secondary => 0.6,
            VisualLayer::Tertiary => 0.3,
            VisualLayer::Background => 0.1,
        };
        (layer_w * self.importance.clamp(0.0, 1.0) * self.opacity.clamp(0.0, 1.0)).clamp(0.0, 1.0)
    }
}
#[derive(Debug, Clone)]
pub struct HierarchyManager {
    pub elements: Vec<VisualElement>,
}
impl Default for HierarchyManager {
    fn default() -> Self {
        Self::new()
    }
}
impl HierarchyManager {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
    pub fn add_element(&mut self, e: VisualElement) {
        self.elements.push(e);
    }
    pub fn dominant_element(&self) -> Option<&VisualElement> {
        self.elements.iter().max_by(|a, b| {
            a.effective_weight()
                .partial_cmp(&b.effective_weight())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
    pub fn sorted_by_depth(&self) -> Vec<&VisualElement> {
        let mut s: Vec<_> = self.elements.iter().collect();
        s.sort_by_key(|e| e.z_order);
        s
    }
    pub fn primary_count(&self) -> usize {
        self.elements
            .iter()
            .filter(|e| e.layer == VisualLayer::Primary)
            .count()
    }
    pub fn has_single_focus(&self) -> bool {
        self.primary_count() <= 1
    }
    pub fn total_visual_weight(&self) -> f64 {
        self.elements.iter().map(|e| e.effective_weight()).sum()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dominant() {
        let mut m = HierarchyManager::new();
        m.add_element(VisualElement {
            name: "turn".into(),
            layer: VisualLayer::Primary,
            importance: 1.0,
            opacity: 1.0,
            z_order: 10,
        });
        m.add_element(VisualElement {
            name: "map".into(),
            layer: VisualLayer::Background,
            importance: 0.5,
            opacity: 0.8,
            z_order: 0,
        });
        assert_eq!(m.dominant_element().unwrap().name, "turn");
    }
    #[test]
    fn test_single_focus() {
        let mut m = HierarchyManager::new();
        m.add_element(VisualElement {
            name: "a".into(),
            layer: VisualLayer::Primary,
            importance: 1.0,
            opacity: 1.0,
            z_order: 1,
        });
        assert!(m.has_single_focus());
    }
    #[test]
    fn test_depth_sort() {
        let mut m = HierarchyManager::new();
        m.add_element(VisualElement {
            name: "top".into(),
            layer: VisualLayer::Primary,
            importance: 1.0,
            opacity: 1.0,
            z_order: 10,
        });
        m.add_element(VisualElement {
            name: "bot".into(),
            layer: VisualLayer::Background,
            importance: 0.5,
            opacity: 0.5,
            z_order: 0,
        });
        assert_eq!(m.sorted_by_depth()[0].name, "bot");
    }
    #[test]
    fn test_empty() {
        let m = HierarchyManager::new();
        assert!(m.dominant_element().is_none());
        assert!(m.has_single_focus());
    }
}
