//! Calendar — holiday detection, event awareness, and schedule-based
//! routing adjustments for culturally significant dates.

use chrono::{Datelike, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};

/// Type of calendar event.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventType {
    /// National/public holiday
    PublicHoliday,
    /// Religious observance
    ReligiousObservance,
    /// School holiday period
    SchoolHoliday,
    /// Major sporting event
    SportingEvent,
    /// Festival or cultural celebration
    Festival,
    /// Memorial/remembrance day
    MemorialDay,
    /// Market day / bazaar
    MarketDay,
    /// Custom event
    Custom(String),
}

/// A calendar event that may affect navigation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    /// Event name
    pub name: String,
    /// Event type
    pub event_type: EventType,
    /// Start date
    pub start_date: NaiveDate,
    /// End date (inclusive)
    pub end_date: NaiveDate,
    /// Country/region codes affected (ISO 3166-1 alpha-2)
    pub regions: Vec<String>,
    /// Traffic impact estimate (0.0 = none, 1.0 = severe)
    pub traffic_impact: f64,
    /// Whether businesses are typically closed
    pub businesses_closed: bool,
    /// Whether public transport runs reduced schedule
    pub reduced_transit: bool,
    /// Description for user display
    pub description: String,
}

impl CalendarEvent {
    /// Check if this event is active on the given date.
    pub fn is_active_on(&self, date: NaiveDate) -> bool {
        date >= self.start_date && date <= self.end_date
    }

    /// Check if this event affects the given region.
    pub fn affects_region(&self, region: &str) -> bool {
        self.regions.iter().any(|r| r == region)
    }

    /// Duration of the event in days.
    pub fn duration_days(&self) -> i64 {
        (self.end_date - self.start_date).num_days() + 1
    }
}

/// Day-of-week traffic pattern.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DayPattern {
    /// Day of week
    pub day: Weekday,
    /// Morning rush hour multiplier (1.0 = normal)
    pub morning_rush_factor: f64,
    /// Evening rush hour multiplier
    pub evening_rush_factor: f64,
    /// Whether this is considered a rest day
    pub is_rest_day: bool,
}

/// Calendar engine for event detection and schedule awareness.
#[derive(Debug)]
pub struct CalendarEngine {
    events: Vec<CalendarEvent>,
    day_patterns: Vec<DayPattern>,
    default_region: String,
}

impl CalendarEngine {
    /// Create a new calendar engine for the given region.
    pub fn new(region: &str) -> Self {
        Self {
            events: Vec::new(),
            day_patterns: default_day_patterns(region),
            default_region: region.to_string(),
        }
    }

    /// Add a calendar event.
    pub fn add_event(&mut self, event: CalendarEvent) {
        self.events.push(event);
    }

    /// Find all active events for a given date and region.
    pub fn active_events(&self, date: NaiveDate, region: &str) -> Vec<&CalendarEvent> {
        self.events
            .iter()
            .filter(|e| e.is_active_on(date) && e.affects_region(region))
            .collect()
    }

    /// Find events active today in the default region.
    pub fn today_events(&self) -> Vec<&CalendarEvent> {
        let today = chrono::Utc::now().date_naive();
        self.active_events(today, &self.default_region)
    }

    /// Get the traffic impact factor for a given date (0.0 to 1.0).
    pub fn traffic_impact(&self, date: NaiveDate, region: &str) -> f64 {
        self.active_events(date, region)
            .iter()
            .map(|e| e.traffic_impact)
            .fold(0.0_f64, |a, b| a.max(b))
    }

    /// Check if businesses are likely closed on the given date.
    pub fn businesses_closed(&self, date: NaiveDate, region: &str) -> bool {
        self.active_events(date, region)
            .iter()
            .any(|e| e.businesses_closed)
    }

    /// Get the day pattern for a specific weekday.
    pub fn day_pattern(&self, day: Weekday) -> Option<&DayPattern> {
        self.day_patterns.iter().find(|p| p.day == day)
    }

    /// Check if a given date is a rest day (weekend or holiday).
    pub fn is_rest_day(&self, date: NaiveDate, region: &str) -> bool {
        // Check day pattern
        let weekday = date.weekday();
        let pattern_rest = self
            .day_patterns
            .iter()
            .find(|p| p.day == weekday)
            .is_some_and(|p| p.is_rest_day);

        // Check holiday
        let holiday = self
            .active_events(date, region)
            .iter()
            .any(|e| matches!(e.event_type, EventType::PublicHoliday));

        pattern_rest || holiday
    }

    /// Total event count.
    pub fn event_count(&self) -> usize {
        self.events.len()
    }
}

/// Generate default day patterns for a region.
fn default_day_patterns(region: &str) -> Vec<DayPattern> {
    // Most regions: Saturday-Sunday rest, some regions Friday-Saturday
    let rest_days: (Weekday, Weekday) = match region {
        "IL" | "SA" | "AE" | "BH" | "KW" | "QA" | "OM" => (Weekday::Fri, Weekday::Sat),
        _ => (Weekday::Sat, Weekday::Sun),
    };

    [
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
        Weekday::Sat,
        Weekday::Sun,
    ]
    .iter()
    .map(|&day| {
        let is_rest = day == rest_days.0 || day == rest_days.1;
        DayPattern {
            day,
            morning_rush_factor: if is_rest { 0.5 } else { 1.5 },
            evening_rush_factor: if is_rest { 0.6 } else { 1.4 },
            is_rest_day: is_rest,
        }
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_event() -> CalendarEvent {
        CalendarEvent {
            name: "Independence Day".to_string(),
            event_type: EventType::PublicHoliday,
            start_date: NaiveDate::from_ymd_opt(2026, 4, 14).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2026, 4, 14).unwrap(),
            regions: vec!["IL".to_string()],
            traffic_impact: 0.3,
            businesses_closed: true,
            reduced_transit: true,
            description: "Israeli Independence Day".to_string(),
        }
    }

    #[test]
    fn test_event_active_on_date() {
        let event = sample_event();
        let active_date = NaiveDate::from_ymd_opt(2026, 4, 14).unwrap();
        let other_date = NaiveDate::from_ymd_opt(2026, 4, 15).unwrap();
        assert!(event.is_active_on(active_date));
        assert!(!event.is_active_on(other_date));
    }

    #[test]
    fn test_event_affects_region() {
        let event = sample_event();
        assert!(event.affects_region("IL"));
        assert!(!event.affects_region("US"));
    }

    #[test]
    fn test_event_duration() {
        let mut event = sample_event();
        assert_eq!(event.duration_days(), 1);

        event.end_date = NaiveDate::from_ymd_opt(2026, 4, 16).unwrap();
        assert_eq!(event.duration_days(), 3);
    }

    #[test]
    fn test_calendar_engine_active_events() {
        let mut engine = CalendarEngine::new("IL");
        engine.add_event(sample_event());

        let date = NaiveDate::from_ymd_opt(2026, 4, 14).unwrap();
        let active = engine.active_events(date, "IL");
        assert_eq!(active.len(), 1);

        let no_events = engine.active_events(date, "US");
        assert_eq!(no_events.len(), 0);
    }

    #[test]
    fn test_calendar_traffic_impact() {
        let mut engine = CalendarEngine::new("IL");
        engine.add_event(sample_event());

        let date = NaiveDate::from_ymd_opt(2026, 4, 14).unwrap();
        assert!((engine.traffic_impact(date, "IL") - 0.3).abs() < f64::EPSILON);

        let normal_date = NaiveDate::from_ymd_opt(2026, 4, 20).unwrap();
        assert!((engine.traffic_impact(normal_date, "IL")).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calendar_businesses_closed() {
        let mut engine = CalendarEngine::new("IL");
        engine.add_event(sample_event());

        let date = NaiveDate::from_ymd_opt(2026, 4, 14).unwrap();
        assert!(engine.businesses_closed(date, "IL"));
    }

    #[test]
    fn test_day_patterns_israel() {
        let engine = CalendarEngine::new("IL");
        let friday = engine.day_pattern(Weekday::Fri).unwrap();
        assert!(friday.is_rest_day);

        let sunday = engine.day_pattern(Weekday::Sun).unwrap();
        assert!(!sunday.is_rest_day);
    }

    #[test]
    fn test_day_patterns_us() {
        let engine = CalendarEngine::new("US");
        let saturday = engine.day_pattern(Weekday::Sat).unwrap();
        assert!(saturday.is_rest_day);

        let friday = engine.day_pattern(Weekday::Fri).unwrap();
        assert!(!friday.is_rest_day);
    }

    #[test]
    fn test_is_rest_day() {
        let mut engine = CalendarEngine::new("IL");
        engine.add_event(sample_event());

        // Holiday is a rest day
        let holiday = NaiveDate::from_ymd_opt(2026, 4, 14).unwrap();
        assert!(engine.is_rest_day(holiday, "IL"));
    }
}
