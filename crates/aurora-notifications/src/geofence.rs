//! Geo-fence alerts — trigger notifications when entering/exiting geographic regions.

use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Geo-fence trigger type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriggerType {
    Enter,
    Exit,
    Both,
    Dwell,
}

/// Shape of a geo-fence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FenceShape {
    Circle {
        center_lat: f64,
        center_lon: f64,
        radius_m: f64,
    },
    Rect {
        min_lat: f64,
        min_lon: f64,
        max_lat: f64,
        max_lon: f64,
    },
}

/// A geo-fence definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoFence {
    pub id: Uuid,
    pub name: String,
    pub shape: FenceShape,
    pub trigger: TriggerType,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub cooldown_secs: u64,
    pub message: String,
}

/// Event generated when a fence is triggered.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FenceEvent {
    pub fence_id: Uuid,
    pub fence_name: String,
    pub trigger: TriggerType,
    pub timestamp: DateTime<Utc>,
    pub lat: f64,
    pub lon: f64,
}

/// Geo-fence manager — monitors position against registered fences.
pub struct GeoFenceManager {
    fences: RwLock<Vec<GeoFence>>,
    last_triggered: RwLock<std::collections::HashMap<Uuid, DateTime<Utc>>>,
    was_inside: RwLock<std::collections::HashMap<Uuid, bool>>,
}

impl GeoFenceManager {
    /// Create a new manager.
    pub fn new() -> Self {
        Self {
            fences: RwLock::new(Vec::new()),
            last_triggered: RwLock::new(std::collections::HashMap::new()),
            was_inside: RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// Register a new geo-fence.
    pub fn add_fence(&self, fence: GeoFence) {
        let id = fence.id;
        self.fences.write().push(fence);
        self.was_inside.write().insert(id, false);
    }

    /// Remove a fence by ID.
    pub fn remove_fence(&self, id: Uuid) -> bool {
        let mut fences = self.fences.write();
        let before = fences.len();
        fences.retain(|f| f.id != id);
        self.was_inside.write().remove(&id);
        self.last_triggered.write().remove(&id);
        fences.len() < before
    }

    /// Check if a point is inside a fence shape.
    fn is_inside(shape: &FenceShape, lat: f64, lon: f64) -> bool {
        match shape {
            FenceShape::Circle {
                center_lat,
                center_lon,
                radius_m,
            } => {
                let dlat = (lat - center_lat).to_radians();
                let dlon = (lon - center_lon).to_radians();
                let a = (dlat / 2.0).sin().powi(2)
                    + center_lat.to_radians().cos()
                        * lat.to_radians().cos()
                        * (dlon / 2.0).sin().powi(2);
                let dist = 2.0 * 6_371_000.0 * a.sqrt().asin();
                dist <= *radius_m
            }
            FenceShape::Rect {
                min_lat,
                min_lon,
                max_lat,
                max_lon,
            } => lat >= *min_lat && lat <= *max_lat && lon >= *min_lon && lon <= *max_lon,
        }
    }

    /// Update position and check all fences. Returns triggered events.
    pub fn update_position(&self, lat: f64, lon: f64, now: DateTime<Utc>) -> Vec<FenceEvent> {
        let fences = self.fences.read();
        let mut was_inside = self.was_inside.write();
        let mut last_triggered = self.last_triggered.write();
        let mut events = Vec::new();

        for fence in fences.iter() {
            if !fence.active {
                continue;
            }

            let inside = Self::is_inside(&fence.shape, lat, lon);
            let prev_inside = was_inside.get(&fence.id).copied().unwrap_or(false);

            // Check cooldown
            if let Some(last) = last_triggered.get(&fence.id) {
                let elapsed = (now - *last).num_seconds();
                if elapsed >= 0 && (elapsed as u64) < fence.cooldown_secs {
                    was_inside.insert(fence.id, inside);
                    continue;
                }
            }

            let triggered = match fence.trigger {
                TriggerType::Enter => inside && !prev_inside,
                TriggerType::Exit => !inside && prev_inside,
                TriggerType::Both => inside != prev_inside,
                TriggerType::Dwell => inside, // triggers every update while inside
            };

            if triggered {
                let trigger_type = match fence.trigger {
                    TriggerType::Dwell => TriggerType::Dwell,
                    _ => {
                        if inside {
                            TriggerType::Enter
                        } else {
                            TriggerType::Exit
                        }
                    }
                };
                events.push(FenceEvent {
                    fence_id: fence.id,
                    fence_name: fence.name.clone(),
                    trigger: trigger_type,
                    timestamp: now,
                    lat,
                    lon,
                });
                last_triggered.insert(fence.id, now);
            }

            was_inside.insert(fence.id, inside);
        }

        events
    }

    /// Get all registered fences.
    pub fn fences(&self) -> Vec<GeoFence> {
        self.fences.read().clone()
    }

    /// Toggle a fence's active state.
    pub fn set_active(&self, id: Uuid, active: bool) -> bool {
        let mut fences = self.fences.write();
        if let Some(f) = fences.iter_mut().find(|f| f.id == id) {
            f.active = active;
            return true;
        }
        false
    }

    /// Count active fences.
    pub fn active_count(&self) -> usize {
        self.fences.read().iter().filter(|f| f.active).count()
    }
}

impl Default for GeoFenceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn circle_fence(lat: f64, lon: f64, radius: f64) -> GeoFence {
        GeoFence {
            id: Uuid::new_v4(),
            name: "test-circle".to_string(),
            shape: FenceShape::Circle {
                center_lat: lat,
                center_lon: lon,
                radius_m: radius,
            },
            trigger: TriggerType::Enter,
            active: true,
            created_at: Utc::now(),
            cooldown_secs: 0,
            message: "Entered zone".to_string(),
        }
    }

    fn rect_fence(min_lat: f64, min_lon: f64, max_lat: f64, max_lon: f64) -> GeoFence {
        GeoFence {
            id: Uuid::new_v4(),
            name: "test-rect".to_string(),
            shape: FenceShape::Rect {
                min_lat,
                min_lon,
                max_lat,
                max_lon,
            },
            trigger: TriggerType::Both,
            active: true,
            created_at: Utc::now(),
            cooldown_secs: 0,
            message: "Zone transition".to_string(),
        }
    }

    #[test]
    fn test_circle_enter() {
        let mgr = GeoFenceManager::new();
        let fence = circle_fence(32.0, 34.0, 1000.0);
        mgr.add_fence(fence);
        let now = Utc::now();
        // Start outside
        let events = mgr.update_position(33.0, 35.0, now);
        assert!(events.is_empty());
        // Move inside
        let events = mgr.update_position(32.0001, 34.0001, now);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].trigger, TriggerType::Enter);
    }

    #[test]
    fn test_rect_enter_exit() {
        let mgr = GeoFenceManager::new();
        let fence = rect_fence(31.0, 33.0, 33.0, 35.0);
        mgr.add_fence(fence);
        let now = Utc::now();
        // Outside
        mgr.update_position(30.0, 30.0, now);
        // Enter
        let events = mgr.update_position(32.0, 34.0, now);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].trigger, TriggerType::Enter);
        // Exit
        let events = mgr.update_position(30.0, 30.0, now);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].trigger, TriggerType::Exit);
    }

    #[test]
    fn test_inactive_fence_ignored() {
        let mgr = GeoFenceManager::new();
        let mut fence = circle_fence(32.0, 34.0, 1000.0);
        fence.active = false;
        mgr.add_fence(fence);
        let now = Utc::now();
        mgr.update_position(50.0, 50.0, now);
        let events = mgr.update_position(32.0, 34.0, now);
        assert!(events.is_empty());
    }

    #[test]
    fn test_cooldown() {
        let mgr = GeoFenceManager::new();
        let mut fence = circle_fence(32.0, 34.0, 1000.0);
        fence.cooldown_secs = 60;
        mgr.add_fence(fence);
        let now = Utc::now();
        mgr.update_position(50.0, 50.0, now);
        // Enter triggers
        let events = mgr.update_position(32.0, 34.0, now);
        assert_eq!(events.len(), 1);
        // Exit + re-enter within cooldown — no trigger
        mgr.update_position(50.0, 50.0, now);
        let events = mgr.update_position(32.0, 34.0, now + chrono::Duration::seconds(30));
        assert!(events.is_empty());
        // After cooldown
        mgr.update_position(50.0, 50.0, now + chrono::Duration::seconds(61));
        let events = mgr.update_position(32.0, 34.0, now + chrono::Duration::seconds(61));
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_remove_fence() {
        let mgr = GeoFenceManager::new();
        let fence = circle_fence(32.0, 34.0, 1000.0);
        let id = fence.id;
        mgr.add_fence(fence);
        assert!(mgr.remove_fence(id));
        assert!(!mgr.remove_fence(id));
        assert!(mgr.fences().is_empty());
    }

    #[test]
    fn test_toggle_active() {
        let mgr = GeoFenceManager::new();
        let fence = circle_fence(32.0, 34.0, 1000.0);
        let id = fence.id;
        mgr.add_fence(fence);
        assert_eq!(mgr.active_count(), 1);
        mgr.set_active(id, false);
        assert_eq!(mgr.active_count(), 0);
        mgr.set_active(id, true);
        assert_eq!(mgr.active_count(), 1);
    }

    #[test]
    fn test_dwell_trigger() {
        let mgr = GeoFenceManager::new();
        let mut fence = rect_fence(31.0, 33.0, 33.0, 35.0);
        fence.trigger = TriggerType::Dwell;
        fence.cooldown_secs = 0;
        mgr.add_fence(fence);
        let now = Utc::now();
        // Inside — triggers every update with Dwell type
        let e1 = mgr.update_position(32.0, 34.0, now);
        let e2 = mgr.update_position(32.0, 34.0, now);
        assert_eq!(e1.len(), 1);
        assert_eq!(e1[0].trigger, TriggerType::Dwell);
        assert_eq!(e2.len(), 1);
        assert_eq!(e2[0].trigger, TriggerType::Dwell);
        // Outside — no trigger
        let e3 = mgr.update_position(30.0, 30.0, now);
        assert!(e3.is_empty());
    }
}
