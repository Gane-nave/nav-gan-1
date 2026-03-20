//! Event tracking — captures navigation events, user actions, and system metrics
//! for analytics processing.

use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Category of analytics event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventCategory {
    /// Navigation-related events (route start, reroute, arrival).
    Navigation,
    /// User interaction events (tap, swipe, voice command).
    Interaction,
    /// System performance events (latency, memory, CPU).
    System,
    /// Search events (POI lookup, address search).
    Search,
    /// Safety events (hard brake, collision warning).
    Safety,
    /// Commerce events (fuel purchase, parking payment).
    Commerce,
}

/// A single analytics event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    pub id: Uuid,
    pub category: EventCategory,
    pub name: String,
    pub timestamp: DateTime<Utc>,
    pub session_id: Uuid,
    pub user_id: Option<Uuid>,
    pub properties: HashMap<String, serde_json::Value>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

impl AnalyticsEvent {
    /// Create a new analytics event.
    pub fn new(category: EventCategory, name: &str, session_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            category,
            name: name.to_string(),
            timestamp: Utc::now(),
            session_id,
            user_id: None,
            properties: HashMap::new(),
            lat: None,
            lon: None,
        }
    }

    /// Attach a user ID.
    pub fn with_user(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Attach a location.
    pub fn with_location(mut self, lat: f64, lon: f64) -> Self {
        self.lat = Some(lat);
        self.lon = Some(lon);
        self
    }

    /// Set a property.
    pub fn set_property(&mut self, key: &str, value: serde_json::Value) {
        self.properties.insert(key.to_string(), value);
    }
}

/// Time-bucketed aggregation of event counts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventAggregate {
    pub category: EventCategory,
    pub name: String,
    pub count: u64,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}

/// Event collector that stores and queries analytics events.
pub struct EventCollector {
    events: RwLock<Vec<AnalyticsEvent>>,
    max_events: usize,
}

impl EventCollector {
    /// Create a new collector with a maximum event buffer size.
    pub fn new(max_events: usize) -> Self {
        Self {
            events: RwLock::new(Vec::new()),
            max_events,
        }
    }

    /// Record an event. If buffer is full, oldest events are evicted.
    pub fn track(&self, event: AnalyticsEvent) {
        let mut events = self.events.write();
        if events.len() >= self.max_events {
            let drain_count = (self.max_events / 10).max(1); // evict 10%, minimum 1
            events.drain(..drain_count);
        }
        events.push(event);
    }

    /// Total number of events in the buffer.
    pub fn len(&self) -> usize {
        self.events.read().len()
    }

    /// Whether the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.events.read().is_empty()
    }

    /// Count events by category.
    pub fn count_by_category(&self, category: EventCategory) -> u64 {
        self.events
            .read()
            .iter()
            .filter(|e| e.category == category)
            .count() as u64
    }

    /// Count events by name within a time range.
    pub fn count_in_range(&self, name: &str, from: DateTime<Utc>, to: DateTime<Utc>) -> u64 {
        self.events
            .read()
            .iter()
            .filter(|e| e.name == name && (from..=to).contains(&e.timestamp))
            .count() as u64
    }

    /// Aggregate events by name, returning counts and time ranges.
    pub fn aggregate_by_name(&self) -> Vec<EventAggregate> {
        let events = self.events.read();
        let mut map: HashMap<String, EventAggregate> = HashMap::new();

        for e in events.iter() {
            let entry = map.entry(e.name.clone()).or_insert_with(|| EventAggregate {
                category: e.category,
                name: e.name.clone(),
                count: 0,
                first_seen: e.timestamp,
                last_seen: e.timestamp,
            });
            entry.count += 1;
            if e.timestamp < entry.first_seen {
                entry.first_seen = e.timestamp;
            }
            if e.timestamp > entry.last_seen {
                entry.last_seen = e.timestamp;
            }
        }

        let mut aggs: Vec<EventAggregate> = map.into_values().collect();
        aggs.sort_by(|a, b| b.count.cmp(&a.count));
        aggs
    }

    /// Get events for a specific session.
    pub fn events_for_session(&self, session_id: Uuid) -> Vec<AnalyticsEvent> {
        self.events
            .read()
            .iter()
            .filter(|e| e.session_id == session_id)
            .cloned()
            .collect()
    }

    /// Get unique user count.
    pub fn unique_users(&self) -> usize {
        let events = self.events.read();
        let users: std::collections::HashSet<Uuid> =
            events.iter().filter_map(|e| e.user_id).collect();
        users.len()
    }

    /// Drain all events from the buffer.
    pub fn drain(&self) -> Vec<AnalyticsEvent> {
        let mut events = self.events.write();
        std::mem::take(&mut *events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_event(cat: EventCategory, name: &str, session: Uuid) -> AnalyticsEvent {
        AnalyticsEvent::new(cat, name, session)
    }

    #[test]
    fn test_track_and_count() {
        let collector = EventCollector::new(1000);
        let sid = Uuid::new_v4();
        collector.track(make_event(EventCategory::Navigation, "route_start", sid));
        collector.track(make_event(EventCategory::Navigation, "reroute", sid));
        collector.track(make_event(EventCategory::Search, "poi_search", sid));
        assert_eq!(collector.len(), 3);
        assert_eq!(collector.count_by_category(EventCategory::Navigation), 2);
        assert_eq!(collector.count_by_category(EventCategory::Search), 1);
        assert_eq!(collector.count_by_category(EventCategory::Safety), 0);
    }

    #[test]
    fn test_eviction_on_full_buffer() {
        let collector = EventCollector::new(10);
        let sid = Uuid::new_v4();
        for i in 0..15 {
            collector.track(make_event(
                EventCategory::System,
                &format!("event_{i}"),
                sid,
            ));
        }
        // Buffer should never exceed max + 1 per insertion
        assert!(collector.len() <= 10);
    }

    #[test]
    fn test_aggregate_by_name() {
        let collector = EventCollector::new(1000);
        let sid = Uuid::new_v4();
        for _ in 0..5 {
            collector.track(make_event(EventCategory::Navigation, "route_start", sid));
        }
        for _ in 0..3 {
            collector.track(make_event(EventCategory::Navigation, "arrival", sid));
        }
        let aggs = collector.aggregate_by_name();
        assert_eq!(aggs.len(), 2);
        assert_eq!(aggs[0].name, "route_start");
        assert_eq!(aggs[0].count, 5);
        assert_eq!(aggs[1].name, "arrival");
        assert_eq!(aggs[1].count, 3);
    }

    #[test]
    fn test_events_for_session() {
        let collector = EventCollector::new(1000);
        let s1 = Uuid::new_v4();
        let s2 = Uuid::new_v4();
        collector.track(make_event(EventCategory::Navigation, "start", s1));
        collector.track(make_event(EventCategory::Navigation, "start", s2));
        collector.track(make_event(EventCategory::Navigation, "end", s1));
        assert_eq!(collector.events_for_session(s1).len(), 2);
        assert_eq!(collector.events_for_session(s2).len(), 1);
    }

    #[test]
    fn test_unique_users() {
        let collector = EventCollector::new(1000);
        let sid = Uuid::new_v4();
        let u1 = Uuid::new_v4();
        let u2 = Uuid::new_v4();
        collector.track(make_event(EventCategory::Navigation, "a", sid).with_user(u1));
        collector.track(make_event(EventCategory::Navigation, "b", sid).with_user(u1));
        collector.track(make_event(EventCategory::Navigation, "c", sid).with_user(u2));
        collector.track(make_event(EventCategory::Navigation, "d", sid)); // no user
        assert_eq!(collector.unique_users(), 2);
    }

    #[test]
    fn test_event_with_location_and_properties() {
        let sid = Uuid::new_v4();
        let mut event = AnalyticsEvent::new(EventCategory::Safety, "hard_brake", sid)
            .with_location(32.05, 34.78);
        event.set_property("deceleration_g", serde_json::json!(0.8));
        assert_eq!(event.lat, Some(32.05));
        assert_eq!(event.lon, Some(34.78));
        assert_eq!(event.properties["deceleration_g"], serde_json::json!(0.8));
    }

    #[test]
    fn test_drain() {
        let collector = EventCollector::new(1000);
        let sid = Uuid::new_v4();
        collector.track(make_event(EventCategory::Commerce, "fuel_purchase", sid));
        collector.track(make_event(EventCategory::Commerce, "parking_pay", sid));
        let drained = collector.drain();
        assert_eq!(drained.len(), 2);
        assert!(collector.is_empty());
    }

    #[test]
    fn test_serialization_roundtrip() {
        let sid = Uuid::new_v4();
        let event = AnalyticsEvent::new(EventCategory::Interaction, "tap", sid)
            .with_user(Uuid::new_v4())
            .with_location(40.7128, -74.0060);
        let json = serde_json::to_string(&event).unwrap();
        let de: AnalyticsEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(de.id, event.id);
        assert_eq!(de.category, event.category);
        assert_eq!(de.name, event.name);
    }
}
