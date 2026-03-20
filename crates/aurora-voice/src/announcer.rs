//! Contextual announcer.
//!
//! Generates context-aware announcements for points of interest,
//! speed changes, road conditions, and safety alerts.

/// Announcement category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnnouncementCategory {
    /// Informational (e.g., "entering city limits").
    Info,
    /// Points of interest.
    Poi,
    /// Speed/road condition alerts.
    SpeedAlert,
    /// Safety warnings.
    Safety,
    /// Emergency alerts.
    Emergency,
}

impl AnnouncementCategory {
    /// Whether this category should interrupt current speech.
    pub fn interrupts(self) -> bool {
        matches!(self, Self::Safety | Self::Emergency)
    }
}

/// An announcement to be spoken.
#[derive(Debug, Clone)]
pub struct Announcement {
    pub category: AnnouncementCategory,
    pub message: String,
    /// Minimum time before repeating this announcement (seconds).
    pub cooldown_s: f64,
}

/// Speed zone announcement generator.
#[derive(Debug)]
pub struct SpeedAnnouncer {
    /// Last announced speed limit.
    last_limit_kmh: Option<u32>,
    /// Current speed in km/h.
    current_speed_kmh: f64,
    /// Speed limit tolerance before warning (km/h over limit).
    tolerance_kmh: f64,
}

impl SpeedAnnouncer {
    pub fn new(tolerance_kmh: f64) -> Self {
        Self {
            last_limit_kmh: None,
            current_speed_kmh: 0.0,
            tolerance_kmh,
        }
    }

    /// Update current speed.
    pub fn update_speed(&mut self, speed_kmh: f64) {
        self.current_speed_kmh = speed_kmh;
    }

    /// Check if a speed zone change should be announced.
    pub fn zone_change(&mut self, new_limit_kmh: u32) -> Option<Announcement> {
        if self.last_limit_kmh == Some(new_limit_kmh) {
            return None;
        }
        self.last_limit_kmh = Some(new_limit_kmh);
        Some(Announcement {
            category: AnnouncementCategory::SpeedAlert,
            message: format!("Speed limit is now {} km/h", new_limit_kmh),
            cooldown_s: 30.0,
        })
    }

    /// Check if speeding warning should fire.
    pub fn speeding_warning(&self) -> Option<Announcement> {
        if let Some(limit) = self.last_limit_kmh {
            let over = self.current_speed_kmh - limit as f64;
            if over > self.tolerance_kmh {
                return Some(Announcement {
                    category: AnnouncementCategory::Safety,
                    message: format!("You are {} km/h over the speed limit", over.round() as u32),
                    cooldown_s: 60.0,
                });
            }
        }
        None
    }
}

/// POI announcer.
#[derive(Debug)]
pub struct PoiAnnouncer {
    /// IDs of already-announced POIs (to avoid repeats).
    announced: Vec<String>,
    /// Maximum announcements per minute.
    max_per_minute: u32,
    /// Count in current window.
    count: u32,
}

impl PoiAnnouncer {
    pub fn new(max_per_minute: u32) -> Self {
        Self {
            announced: Vec::new(),
            max_per_minute,
            count: 0,
        }
    }

    /// Try to announce a POI. Returns None if already announced or rate-limited.
    pub fn announce(
        &mut self,
        poi_id: &str,
        poi_name: &str,
        category: &str,
    ) -> Option<Announcement> {
        if self.announced.contains(&poi_id.to_string()) {
            return None;
        }
        if self.count >= self.max_per_minute {
            return None;
        }
        self.announced.push(poi_id.to_string());
        self.count += 1;
        Some(Announcement {
            category: AnnouncementCategory::Poi,
            message: format!("{category}: {poi_name} nearby"),
            cooldown_s: 120.0,
        })
    }

    /// Reset the per-minute counter.
    pub fn reset_window(&mut self) {
        self.count = 0;
    }

    /// Clear all history.
    pub fn clear(&mut self) {
        self.announced.clear();
        self.count = 0;
    }

    pub fn announced_count(&self) -> usize {
        self.announced.len()
    }
}

/// Safety announcer for road conditions.
#[derive(Debug)]
pub struct SafetyAnnouncer;

impl SafetyAnnouncer {
    /// Generate a camera warning.
    pub fn speed_camera(distance_m: f64) -> Announcement {
        let d = (distance_m / 50.0).round() * 50.0;
        Announcement {
            category: AnnouncementCategory::Safety,
            message: format!("Speed camera in {} metres", d as u32),
            cooldown_s: 300.0,
        }
    }

    /// Generate a hazard warning.
    pub fn road_hazard(description: &str) -> Announcement {
        Announcement {
            category: AnnouncementCategory::Safety,
            message: format!("Caution: {description} ahead"),
            cooldown_s: 120.0,
        }
    }

    /// Generate an emergency vehicle alert.
    pub fn emergency_vehicle(direction: &str) -> Announcement {
        Announcement {
            category: AnnouncementCategory::Emergency,
            message: format!("Emergency vehicle approaching from {direction}"),
            cooldown_s: 30.0,
        }
    }

    /// Generate a school zone warning.
    pub fn school_zone(active: bool) -> Announcement {
        if active {
            Announcement {
                category: AnnouncementCategory::Safety,
                message: "Entering active school zone — reduce speed".into(),
                cooldown_s: 600.0,
            }
        } else {
            Announcement {
                category: AnnouncementCategory::Info,
                message: "Leaving school zone".into(),
                cooldown_s: 600.0,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_interrupts() {
        assert!(AnnouncementCategory::Emergency.interrupts());
        assert!(AnnouncementCategory::Safety.interrupts());
        assert!(!AnnouncementCategory::Info.interrupts());
        assert!(!AnnouncementCategory::Poi.interrupts());
    }

    #[test]
    fn test_speed_zone_change() {
        let mut sa = SpeedAnnouncer::new(5.0);
        let ann = sa.zone_change(50).unwrap();
        assert!(ann.message.contains("50"));
        // Repeat same limit → None
        assert!(sa.zone_change(50).is_none());
        // New limit → Some
        assert!(sa.zone_change(80).is_some());
    }

    #[test]
    fn test_speeding_warning() {
        let mut sa = SpeedAnnouncer::new(5.0);
        sa.zone_change(50);
        sa.update_speed(53.0); // within tolerance
        assert!(sa.speeding_warning().is_none());
        sa.update_speed(60.0); // 10 over
        let w = sa.speeding_warning().unwrap();
        assert!(w.message.contains("10"));
        assert_eq!(w.category, AnnouncementCategory::Safety);
    }

    #[test]
    fn test_poi_announcer() {
        let mut pa = PoiAnnouncer::new(3);
        let ann = pa.announce("p1", "Gas Station", "Fuel").unwrap();
        assert!(ann.message.contains("Gas Station"));
        // Duplicate → None
        assert!(pa.announce("p1", "Gas Station", "Fuel").is_none());
        // Different POI → Some
        assert!(pa.announce("p2", "Restaurant", "Food").is_some());
    }

    #[test]
    fn test_poi_rate_limit() {
        let mut pa = PoiAnnouncer::new(2);
        pa.announce("a", "A", "X");
        pa.announce("b", "B", "X");
        assert!(pa.announce("c", "C", "X").is_none()); // rate limited
        pa.reset_window();
        assert!(pa.announce("c", "C", "X").is_some()); // window reset
    }

    #[test]
    fn test_speed_camera() {
        let ann = SafetyAnnouncer::speed_camera(320.0);
        assert!(ann.message.contains("300")); // rounds to nearest 50
        assert_eq!(ann.category, AnnouncementCategory::Safety);
    }

    #[test]
    fn test_emergency_vehicle() {
        let ann = SafetyAnnouncer::emergency_vehicle("behind");
        assert_eq!(ann.category, AnnouncementCategory::Emergency);
        assert!(ann.message.contains("behind"));
    }

    #[test]
    fn test_school_zone() {
        let active = SafetyAnnouncer::school_zone(true);
        assert_eq!(active.category, AnnouncementCategory::Safety);
        let inactive = SafetyAnnouncer::school_zone(false);
        assert_eq!(inactive.category, AnnouncementCategory::Info);
    }

    #[test]
    fn test_road_hazard() {
        let ann = SafetyAnnouncer::road_hazard("flooding");
        assert!(ann.message.contains("flooding"));
    }

    #[test]
    fn test_poi_clear() {
        let mut pa = PoiAnnouncer::new(5);
        pa.announce("x", "X", "C");
        pa.clear();
        assert_eq!(pa.announced_count(), 0);
        // Can now re-announce
        assert!(pa.announce("x", "X", "C").is_some());
    }
}
