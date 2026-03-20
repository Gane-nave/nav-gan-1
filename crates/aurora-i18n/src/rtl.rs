//! RTL (right-to-left) layout support — text alignment, mirroring, bidirectional text.

use serde::{Deserialize, Serialize};

/// Layout direction for UI rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayoutDirection {
    Ltr,
    Rtl,
}

/// Text alignment respecting layout direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextAlignment {
    Start,
    End,
    Center,
    /// Resolved physical alignment.
    Left,
    Right,
}

impl TextAlignment {
    /// Resolve a logical alignment to a physical one given a layout direction.
    pub fn resolve(self, direction: LayoutDirection) -> Self {
        match (self, direction) {
            (TextAlignment::Start, LayoutDirection::Ltr) => TextAlignment::Left,
            (TextAlignment::Start, LayoutDirection::Rtl) => TextAlignment::Right,
            (TextAlignment::End, LayoutDirection::Ltr) => TextAlignment::Right,
            (TextAlignment::End, LayoutDirection::Rtl) => TextAlignment::Left,
            (other, _) => other,
        }
    }
}

/// Horizontal mirroring for icons and UI elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MirrorPolicy {
    /// Always mirror in RTL (e.g., back arrow).
    Mirror,
    /// Never mirror (e.g., clock icon, media controls).
    NoMirror,
    /// Mirror only if directional (e.g., progress bars).
    Directional,
}

/// A UI element with RTL-awareness.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtlElement {
    pub id: String,
    pub mirror_policy: MirrorPolicy,
    pub text_alignment: TextAlignment,
}

impl RtlElement {
    /// Create a new RTL-aware element.
    pub fn new(id: &str, mirror: MirrorPolicy, align: TextAlignment) -> Self {
        Self {
            id: id.to_string(),
            mirror_policy: mirror,
            text_alignment: align,
        }
    }

    /// Check if this element should be mirrored in a given direction.
    pub fn should_mirror(&self, direction: LayoutDirection) -> bool {
        if direction == LayoutDirection::Ltr {
            return false;
        }
        match self.mirror_policy {
            MirrorPolicy::Mirror => true,
            MirrorPolicy::NoMirror => false,
            MirrorPolicy::Directional => true,
        }
    }

    /// Resolve the text alignment for a given direction.
    pub fn resolved_alignment(&self, direction: LayoutDirection) -> TextAlignment {
        self.text_alignment.resolve(direction)
    }
}

/// Navigation instruction mirroring for RTL layouts.
/// Turn directions need to be visually mirrored but logically preserved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NavDirection {
    Left,
    Right,
    Straight,
    UTurn,
}

/// Mirror a navigation arrow icon position for RTL display.
/// The logical direction stays the same, but the icon position flips.
pub fn mirror_nav_icon_position(x_position: f64, container_width: f64) -> f64 {
    container_width - x_position
}

/// Bidirectional text segment — tracks runs of LTR and RTL text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BidiSegment {
    pub text: String,
    pub direction: LayoutDirection,
    pub level: u8,
}

/// Simple bidirectional text analysis.
/// Splits text into segments based on Unicode script detection.
pub fn analyze_bidi(text: &str) -> Vec<BidiSegment> {
    if text.is_empty() {
        return vec![];
    }

    let mut segments = Vec::new();
    let mut current_text = String::new();
    let mut current_dir = LayoutDirection::Ltr;

    for ch in text.chars() {
        let char_dir = if is_rtl_char(ch) {
            LayoutDirection::Rtl
        } else {
            LayoutDirection::Ltr
        };

        if !current_text.is_empty() && char_dir != current_dir && !ch.is_whitespace() {
            segments.push(BidiSegment {
                text: current_text.clone(),
                direction: current_dir,
                level: if current_dir == LayoutDirection::Rtl {
                    1
                } else {
                    0
                },
            });
            current_text.clear();
        }

        if !ch.is_whitespace() || current_text.is_empty() {
            current_dir = char_dir;
        }
        current_text.push(ch);
    }

    if !current_text.is_empty() {
        segments.push(BidiSegment {
            text: current_text,
            direction: current_dir,
            level: if current_dir == LayoutDirection::Rtl {
                1
            } else {
                0
            },
        });
    }

    segments
}

/// Check if a character is in an RTL script (Arabic, Hebrew, etc.).
fn is_rtl_char(ch: char) -> bool {
    let cp = ch as u32;
    // Arabic range
    (0x0600..=0x06FF).contains(&cp)
        || (0x0750..=0x077F).contains(&cp)
        || (0x08A0..=0x08FF).contains(&cp)
        || (0xFB50..=0xFDFF).contains(&cp)
        || (0xFE70..=0xFEFF).contains(&cp)
        // Hebrew range
        || (0x0590..=0x05FF).contains(&cp)
        || (0xFB1D..=0xFB4F).contains(&cp)
        // Syriac, Thaana, etc.
        || (0x0700..=0x074F).contains(&cp)
        || (0x0780..=0x07BF).contains(&cp)
}

/// RTL layout manager — tracks direction and applies transformations.
pub struct RtlLayoutManager {
    direction: LayoutDirection,
    elements: Vec<RtlElement>,
}

impl RtlLayoutManager {
    /// Create a new layout manager.
    pub fn new(direction: LayoutDirection) -> Self {
        Self {
            direction,
            elements: Vec::new(),
        }
    }

    /// Set the layout direction.
    pub fn set_direction(&mut self, direction: LayoutDirection) {
        self.direction = direction;
    }

    /// Get the current direction.
    pub fn direction(&self) -> LayoutDirection {
        self.direction
    }

    /// Register a UI element.
    pub fn add_element(&mut self, element: RtlElement) {
        self.elements.push(element);
    }

    /// Get all elements that should be mirrored.
    pub fn mirrored_elements(&self) -> Vec<&RtlElement> {
        self.elements
            .iter()
            .filter(|e| e.should_mirror(self.direction))
            .collect()
    }

    /// Get element count.
    pub fn element_count(&self) -> usize {
        self.elements.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_alignment_resolve_ltr() {
        assert_eq!(
            TextAlignment::Start.resolve(LayoutDirection::Ltr),
            TextAlignment::Left
        );
        assert_eq!(
            TextAlignment::End.resolve(LayoutDirection::Ltr),
            TextAlignment::Right
        );
    }

    #[test]
    fn test_text_alignment_resolve_rtl() {
        assert_eq!(
            TextAlignment::Start.resolve(LayoutDirection::Rtl),
            TextAlignment::Right
        );
        assert_eq!(
            TextAlignment::End.resolve(LayoutDirection::Rtl),
            TextAlignment::Left
        );
    }

    #[test]
    fn test_center_alignment_unchanged() {
        assert_eq!(
            TextAlignment::Center.resolve(LayoutDirection::Ltr),
            TextAlignment::Center
        );
        assert_eq!(
            TextAlignment::Center.resolve(LayoutDirection::Rtl),
            TextAlignment::Center
        );
    }

    #[test]
    fn test_element_mirror_policy() {
        let el = RtlElement::new("back_arrow", MirrorPolicy::Mirror, TextAlignment::Start);
        assert!(!el.should_mirror(LayoutDirection::Ltr));
        assert!(el.should_mirror(LayoutDirection::Rtl));

        let clock = RtlElement::new("clock", MirrorPolicy::NoMirror, TextAlignment::Center);
        assert!(!clock.should_mirror(LayoutDirection::Rtl));
    }

    #[test]
    fn test_mirror_nav_icon() {
        let mirrored = mirror_nav_icon_position(30.0, 100.0);
        assert!((mirrored - 70.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_bidi_analysis_ltr() {
        let segments = analyze_bidi("Hello world");
        assert!(!segments.is_empty());
        assert_eq!(segments[0].direction, LayoutDirection::Ltr);
    }

    #[test]
    fn test_bidi_analysis_rtl() {
        let segments = analyze_bidi("שלום");
        assert!(!segments.is_empty());
        assert_eq!(segments[0].direction, LayoutDirection::Rtl);
        assert_eq!(segments[0].level, 1);
    }

    #[test]
    fn test_bidi_analysis_mixed() {
        let segments = analyze_bidi("Hello שלום");
        assert!(segments.len() >= 2);
    }

    #[test]
    fn test_bidi_empty() {
        let segments = analyze_bidi("");
        assert!(segments.is_empty());
    }

    #[test]
    fn test_is_rtl_char() {
        assert!(is_rtl_char('א')); // Hebrew
        assert!(is_rtl_char('ع')); // Arabic
        assert!(!is_rtl_char('A'));
        assert!(!is_rtl_char('1'));
    }

    #[test]
    fn test_rtl_layout_manager() {
        let mut mgr = RtlLayoutManager::new(LayoutDirection::Rtl);
        mgr.add_element(RtlElement::new(
            "arrow",
            MirrorPolicy::Mirror,
            TextAlignment::Start,
        ));
        mgr.add_element(RtlElement::new(
            "clock",
            MirrorPolicy::NoMirror,
            TextAlignment::Center,
        ));
        mgr.add_element(RtlElement::new(
            "progress",
            MirrorPolicy::Directional,
            TextAlignment::Start,
        ));

        assert_eq!(mgr.element_count(), 3);
        let mirrored = mgr.mirrored_elements();
        assert_eq!(mirrored.len(), 2); // arrow + progress
    }

    #[test]
    fn test_rtl_layout_manager_ltr() {
        let mut mgr = RtlLayoutManager::new(LayoutDirection::Ltr);
        mgr.add_element(RtlElement::new(
            "arrow",
            MirrorPolicy::Mirror,
            TextAlignment::Start,
        ));

        let mirrored = mgr.mirrored_elements();
        assert_eq!(mirrored.len(), 0); // nothing mirrored in LTR
    }
}
