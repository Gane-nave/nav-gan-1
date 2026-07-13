//! AR overlay — augmented reality overlay positioning, anchor management,
//! and screen-space placement for navigation elements.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::scene::Vec3;

/// Type of AR overlay element.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OverlayType {
    /// Navigation arrow pointing direction
    DirectionArrow,
    /// Distance label
    DistanceLabel,
    /// Speed limit indicator
    SpeedLimit,
    /// Turn instruction
    TurnInstruction,
    /// Lane guidance overlay
    LaneGuidance,
    /// POI information bubble
    PoiBubble,
    /// Hazard warning
    HazardWarning,
    /// Street name label
    StreetName,
    /// Traffic light status
    TrafficLight,
    /// Parking indicator
    ParkingIndicator,
}

/// Screen-space anchor for AR elements.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ScreenAnchor {
    /// Normalized X position (0.0 = left, 1.0 = right)
    pub x: f32,
    /// Normalized Y position (0.0 = top, 1.0 = bottom)
    pub y: f32,
    /// Depth from camera (meters)
    pub depth: f32,
}

impl ScreenAnchor {
    pub fn new(x: f32, y: f32, depth: f32) -> Self {
        Self {
            x: x.clamp(0.0, 1.0),
            y: y.clamp(0.0, 1.0),
            depth: depth.max(0.0),
        }
    }

    /// Center of screen.
    pub fn center() -> Self {
        Self::new(0.5, 0.5, 1.0)
    }
}

/// An AR overlay element.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArElement {
    /// Unique identifier
    pub id: Uuid,
    /// Overlay type
    pub overlay_type: OverlayType,
    /// World-space position
    pub world_position: Vec3,
    /// Screen-space anchor (computed from projection)
    pub screen_anchor: ScreenAnchor,
    /// Display text
    pub text: String,
    /// Icon identifier
    pub icon: Option<String>,
    /// Whether the element is currently visible
    pub visible: bool,
    /// Opacity (0.0 to 1.0)
    pub opacity: f32,
    /// Scale factor
    pub scale: f32,
    /// Priority (higher = rendered on top)
    pub priority: u32,
    /// Expiration time (auto-hide after this)
    pub expires_at: Option<DateTime<Utc>>,
}

impl ArElement {
    /// Create a new AR element.
    pub fn new(overlay_type: OverlayType, world_pos: Vec3, text: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            overlay_type,
            world_position: world_pos,
            screen_anchor: ScreenAnchor::center(),
            text: text.to_string(),
            icon: None,
            visible: true,
            opacity: 1.0,
            scale: 1.0,
            priority: 0,
            expires_at: None,
        }
    }

    /// Check if the element has expired.
    pub fn is_expired(&self) -> bool {
        self.expires_at.is_some_and(|exp| Utc::now() > exp)
    }
}

/// AR overlay manager — manages the set of active AR elements.
#[derive(Debug)]
pub struct ArOverlayManager {
    elements: Vec<ArElement>,
    max_elements: usize,
}

impl ArOverlayManager {
    /// Create a new overlay manager.
    pub fn new(max_elements: usize) -> Self {
        Self {
            elements: Vec::new(),
            max_elements,
        }
    }

    /// Add an element. If at capacity, removes lowest-priority expired element.
    pub fn add(&mut self, element: ArElement) -> bool {
        if self.elements.len() >= self.max_elements {
            // Try to remove an expired element
            if let Some(idx) = self.elements.iter().position(|e| e.is_expired()) {
                self.elements.remove(idx);
            } else {
                return false;
            }
        }
        self.elements.push(element);
        true
    }

    /// Remove an element by ID.
    pub fn remove(&mut self, id: Uuid) -> bool {
        let len_before = self.elements.len();
        self.elements.retain(|e| e.id != id);
        self.elements.len() < len_before
    }

    /// Get an element by ID.
    pub fn get(&self, id: Uuid) -> Option<&ArElement> {
        self.elements.iter().find(|e| e.id == id)
    }

    /// Get all visible, non-expired elements sorted by priority (highest first).
    pub fn active_elements(&self) -> Vec<&ArElement> {
        let mut active: Vec<&ArElement> = self
            .elements
            .iter()
            .filter(|e| e.visible && !e.is_expired())
            .collect();
        active.sort_by_key(|r| std::cmp::Reverse(r.priority));
        active
    }

    /// Remove all expired elements. Returns count of removed.
    pub fn prune_expired(&mut self) -> usize {
        let len_before = self.elements.len();
        self.elements.retain(|e| !e.is_expired());
        len_before - self.elements.len()
    }

    /// Clear all elements.
    pub fn clear(&mut self) {
        self.elements.clear();
    }

    /// Current element count.
    pub fn count(&self) -> usize {
        self.elements.len()
    }

    /// Update the screen anchor for an element (after camera projection).
    pub fn update_anchor(&mut self, id: Uuid, anchor: ScreenAnchor) {
        if let Some(elem) = self.elements.iter_mut().find(|e| e.id == id) {
            elem.screen_anchor = anchor;
        }
    }
}

impl Default for ArOverlayManager {
    fn default() -> Self {
        Self::new(64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ar_element_creation() {
        let elem = ArElement::new(
            OverlayType::DirectionArrow,
            Vec3::new(0.0, 0.0, -10.0),
            "Turn right in 200m",
        );
        assert!(elem.visible);
        assert!(!elem.is_expired());
        assert_eq!(elem.overlay_type, OverlayType::DirectionArrow);
    }

    #[test]
    fn test_ar_element_expiration() {
        let mut elem = ArElement::new(OverlayType::DistanceLabel, Vec3::new(0.0, 0.0, 0.0), "500m");
        // Not expired by default
        assert!(!elem.is_expired());

        // Set expiration to past
        elem.expires_at = Some(Utc::now() - chrono::Duration::seconds(10));
        assert!(elem.is_expired());
    }

    #[test]
    fn test_overlay_manager_add_remove() {
        let mut mgr = ArOverlayManager::new(10);
        let elem = ArElement::new(OverlayType::SpeedLimit, Vec3::zero(), "60");
        let id = elem.id;
        assert!(mgr.add(elem));
        assert_eq!(mgr.count(), 1);
        assert!(mgr.get(id).is_some());

        assert!(mgr.remove(id));
        assert_eq!(mgr.count(), 0);
    }

    #[test]
    fn test_overlay_manager_capacity() {
        let mut mgr = ArOverlayManager::new(2);
        mgr.add(ArElement::new(OverlayType::SpeedLimit, Vec3::zero(), "60"));
        mgr.add(ArElement::new(
            OverlayType::StreetName,
            Vec3::zero(),
            "Main St",
        ));

        // Third add should fail (at capacity, none expired)
        let result = mgr.add(ArElement::new(
            OverlayType::HazardWarning,
            Vec3::zero(),
            "Ice",
        ));
        assert!(!result);
    }

    #[test]
    fn test_overlay_manager_active_sorted() {
        let mut mgr = ArOverlayManager::new(10);

        let mut low = ArElement::new(OverlayType::StreetName, Vec3::zero(), "Elm St");
        low.priority = 1;

        let mut high = ArElement::new(OverlayType::HazardWarning, Vec3::zero(), "Ice");
        high.priority = 10;

        mgr.add(low);
        mgr.add(high);

        let active = mgr.active_elements();
        assert_eq!(active.len(), 2);
        assert_eq!(active[0].priority, 10); // Highest first
    }

    #[test]
    fn test_screen_anchor_clamping() {
        let anchor = ScreenAnchor::new(1.5, -0.5, -10.0);
        assert!((anchor.x - 1.0).abs() < f32::EPSILON);
        assert!(anchor.y.abs() < f32::EPSILON);
        assert!(anchor.depth.abs() < f32::EPSILON);
    }
}
