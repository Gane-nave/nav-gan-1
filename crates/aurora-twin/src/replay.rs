//! Historical event replay — records events from digital twins and allows
//! time-travel playback for diagnostics and analysis.

use aurora_core::types::EntityId;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};

// ---------------------------------------------------------------------------
// Replay types
// ---------------------------------------------------------------------------

/// Kind of recorded event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventKind {
    PropertyChange,
    StateTransition,
    Alert,
    Measurement,
    Command,
    SystemEvent,
}

/// A recorded event in the timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordedEvent {
    pub id: EntityId,
    pub entity_id: EntityId,
    pub kind: EventKind,
    pub key: String,
    pub old_value: Option<serde_json::Value>,
    pub new_value: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub source: String,
}

/// Status of a replay session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayStatus {
    /// Ready to play.
    Ready,
    /// Currently playing forward.
    Playing,
    /// Paused at a specific point.
    Paused,
    /// Reached the end of the recording.
    Finished,
}

/// Playback speed multiplier.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlaybackSpeed(pub f64);

impl PlaybackSpeed {
    pub const NORMAL: Self = Self(1.0);
    pub const DOUBLE: Self = Self(2.0);
    pub const HALF: Self = Self(0.5);
    pub const QUARTER: Self = Self(0.25);
}

impl Default for PlaybackSpeed {
    fn default() -> Self {
        Self::NORMAL
    }
}

/// A time range for filtering events.
#[derive(Debug, Clone)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl TimeRange {
    pub fn contains(&self, t: DateTime<Utc>) -> bool {
        (self.start..=self.end).contains(&t)
    }

    pub fn duration(&self) -> Duration {
        self.end - self.start
    }
}

/// A bookmark marking a point of interest in the timeline.
#[derive(Debug, Clone)]
pub struct Bookmark {
    pub id: EntityId,
    pub label: String,
    pub timestamp: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Summary statistics for a replay session.
#[derive(Debug, Clone)]
pub struct ReplaySummary {
    pub total_events: usize,
    pub unique_entities: usize,
    pub time_span: Option<Duration>,
    pub events_by_kind: HashMap<String, usize>,
    pub first_event_at: Option<DateTime<Utc>>,
    pub last_event_at: Option<DateTime<Utc>>,
}

// ---------------------------------------------------------------------------
// Event recorder
// ---------------------------------------------------------------------------

/// Records events from digital twins for later replay.
pub struct EventRecorder {
    events: Vec<RecordedEvent>,
    /// Maximum events to retain (ring buffer).
    max_events: usize,
    recording: bool,
}

impl EventRecorder {
    pub fn new(max_events: usize) -> Self {
        Self {
            events: Vec::new(),
            max_events,
            recording: true,
        }
    }

    /// Start recording.
    pub fn start_recording(&mut self) {
        self.recording = true;
        info!("event recording started");
    }

    /// Stop recording.
    pub fn stop_recording(&mut self) {
        self.recording = false;
        info!(events = self.events.len(), "event recording stopped");
    }

    /// Whether currently recording.
    pub fn is_recording(&self) -> bool {
        self.recording
    }

    /// Record an event.
    pub fn record(&mut self, event: RecordedEvent) -> bool {
        if !self.recording {
            return false;
        }

        if self.events.len() >= self.max_events {
            self.events.remove(0);
        }

        debug!(
            entity = %event.entity_id,
            kind = ?event.kind,
            key = %event.key,
            "event recorded"
        );
        self.events.push(event);
        true
    }

    /// Number of recorded events.
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Get all events.
    pub fn events(&self) -> &[RecordedEvent] {
        &self.events
    }

    /// Get events for a specific entity.
    pub fn events_for_entity(&self, entity_id: &EntityId) -> Vec<&RecordedEvent> {
        self.events
            .iter()
            .filter(|e| e.entity_id == *entity_id)
            .collect()
    }

    /// Get events within a time range.
    pub fn events_in_range(&self, range: &TimeRange) -> Vec<&RecordedEvent> {
        self.events
            .iter()
            .filter(|e| range.contains(e.timestamp))
            .collect()
    }

    /// Get events by kind.
    pub fn events_by_kind(&self, kind: EventKind) -> Vec<&RecordedEvent> {
        self.events.iter().filter(|e| e.kind == kind).collect()
    }

    /// Clear all recorded events.
    pub fn clear(&mut self) {
        self.events.clear();
        info!("event recorder cleared");
    }

    /// Compute summary statistics.
    pub fn summary(&self) -> ReplaySummary {
        let mut unique_entities = std::collections::HashSet::new();
        let mut by_kind: HashMap<String, usize> = HashMap::new();

        for event in &self.events {
            unique_entities.insert(event.entity_id);
            *by_kind.entry(format!("{:?}", event.kind)).or_default() += 1;
        }

        let first = self.events.first().map(|e| e.timestamp);
        let last = self.events.last().map(|e| e.timestamp);
        let span = match (first, last) {
            (Some(f), Some(l)) => Some(l - f),
            _ => None,
        };

        ReplaySummary {
            total_events: self.events.len(),
            unique_entities: unique_entities.len(),
            time_span: span,
            events_by_kind: by_kind,
            first_event_at: first,
            last_event_at: last,
        }
    }
}

impl Default for EventRecorder {
    fn default() -> Self {
        Self::new(100_000)
    }
}

// ---------------------------------------------------------------------------
// Replay player
// ---------------------------------------------------------------------------

/// Plays back recorded events, allowing navigation through the timeline.
pub struct ReplayPlayer {
    events: Vec<RecordedEvent>,
    cursor: usize,
    status: ReplayStatus,
    speed: PlaybackSpeed,
    bookmarks: Vec<Bookmark>,
    /// Reconstructed state at current cursor position.
    state: HashMap<(EntityId, String), serde_json::Value>,
}

impl ReplayPlayer {
    /// Create a player from recorded events.
    pub fn from_events(events: Vec<RecordedEvent>) -> Self {
        Self {
            events,
            cursor: 0,
            status: ReplayStatus::Ready,
            speed: PlaybackSpeed::NORMAL,
            bookmarks: Vec::new(),
            state: HashMap::new(),
        }
    }

    /// Start playback.
    pub fn play(&mut self) {
        self.status = ReplayStatus::Playing;
        info!("replay playback started");
    }

    /// Pause playback.
    pub fn pause(&mut self) {
        if self.status == ReplayStatus::Playing {
            self.status = ReplayStatus::Paused;
        }
    }

    /// Get current status.
    pub fn status(&self) -> ReplayStatus {
        self.status
    }

    /// Set playback speed.
    pub fn set_speed(&mut self, speed: PlaybackSpeed) {
        self.speed = speed;
    }

    /// Get playback speed.
    pub fn speed(&self) -> PlaybackSpeed {
        self.speed
    }

    /// Advance to the next event. Returns the event if available.
    pub fn step_forward(&mut self) -> Option<&RecordedEvent> {
        if self.cursor >= self.events.len() {
            self.status = ReplayStatus::Finished;
            return None;
        }

        let event = &self.events[self.cursor];

        // Update reconstructed state.
        self.state.insert(
            (event.entity_id, event.key.clone()),
            event.new_value.clone(),
        );

        self.cursor += 1;
        if self.cursor >= self.events.len() {
            self.status = ReplayStatus::Finished;
        }

        Some(event)
    }

    /// Step backward one event.
    pub fn step_backward(&mut self) -> Option<&RecordedEvent> {
        if self.cursor == 0 {
            return None;
        }

        self.cursor -= 1;
        let event = &self.events[self.cursor];

        // Restore previous state.
        if let Some(old) = &event.old_value {
            self.state
                .insert((event.entity_id, event.key.clone()), old.clone());
        } else {
            self.state.remove(&(event.entity_id, event.key.clone()));
        }

        if self.status == ReplayStatus::Finished {
            self.status = ReplayStatus::Paused;
        }

        Some(event)
    }

    /// Jump to a specific event index.
    pub fn seek(&mut self, index: usize) {
        // Reset state and replay from beginning to target.
        self.state.clear();
        self.cursor = 0;

        let target = index.min(self.events.len());
        for i in 0..target {
            let event = &self.events[i];
            self.state.insert(
                (event.entity_id, event.key.clone()),
                event.new_value.clone(),
            );
        }
        self.cursor = target;

        if self.cursor >= self.events.len() {
            self.status = ReplayStatus::Finished;
        } else {
            self.status = ReplayStatus::Paused;
        }
    }

    /// Jump to a specific timestamp (finds the nearest event).
    pub fn seek_to_time(&mut self, target: DateTime<Utc>) {
        let idx = self
            .events
            .iter()
            .enumerate()
            .find(|(_, e)| e.timestamp >= target)
            .map(|(i, _)| i)
            .unwrap_or(self.events.len());
        self.seek(idx);
    }

    /// Current cursor position.
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Total number of events.
    pub fn total_events(&self) -> usize {
        self.events.len()
    }

    /// Get the current event (at cursor).
    pub fn current_event(&self) -> Option<&RecordedEvent> {
        self.events.get(self.cursor)
    }

    /// Get the reconstructed state at the current position.
    pub fn current_state(&self) -> &HashMap<(EntityId, String), serde_json::Value> {
        &self.state
    }

    /// Get a specific entity's property at the current position.
    pub fn get_entity_state(&self, entity_id: &EntityId, key: &str) -> Option<&serde_json::Value> {
        self.state.get(&(*entity_id, key.to_string()))
    }

    // -----------------------------------------------------------------------
    // Bookmarks
    // -----------------------------------------------------------------------

    /// Add a bookmark at the current position.
    pub fn add_bookmark(&mut self, label: impl Into<String>) -> EntityId {
        let timestamp = self
            .events
            .get(self.cursor)
            .map(|e| e.timestamp)
            .unwrap_or_else(Utc::now);

        let id = EntityId::new();
        self.bookmarks.push(Bookmark {
            id,
            label: label.into(),
            timestamp,
            created_at: Utc::now(),
        });
        id
    }

    /// Get all bookmarks.
    pub fn bookmarks(&self) -> &[Bookmark] {
        &self.bookmarks
    }

    /// Jump to a bookmark.
    pub fn goto_bookmark(&mut self, bookmark_id: &EntityId) -> bool {
        let Some(bookmark) = self.bookmarks.iter().find(|b| b.id == *bookmark_id) else {
            return false;
        };
        let ts = bookmark.timestamp;
        self.seek_to_time(ts);
        true
    }

    /// Progress as a percentage (0.0 to 100.0).
    pub fn progress_pct(&self) -> f64 {
        if self.events.is_empty() {
            return 100.0;
        }
        (self.cursor as f64 / self.events.len() as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_event(
        entity_id: EntityId,
        key: &str,
        old: f64,
        new: f64,
        ts: DateTime<Utc>,
    ) -> RecordedEvent {
        RecordedEvent {
            id: EntityId::new(),
            entity_id,
            kind: EventKind::PropertyChange,
            key: key.into(),
            old_value: Some(serde_json::json!(old)),
            new_value: serde_json::json!(new),
            timestamp: ts,
            source: "test".into(),
        }
    }

    fn sample_events() -> Vec<RecordedEvent> {
        let entity = EntityId::new();
        let now = Utc::now();
        vec![
            make_event(entity, "speed", 0.0, 30.0, now),
            make_event(entity, "speed", 30.0, 50.0, now + Duration::seconds(1)),
            make_event(entity, "speed", 50.0, 70.0, now + Duration::seconds(2)),
            make_event(entity, "fuel", 100.0, 98.0, now + Duration::seconds(3)),
            make_event(entity, "speed", 70.0, 60.0, now + Duration::seconds(4)),
        ]
    }

    // -----------------------------------------------------------------------
    // Recorder tests
    // -----------------------------------------------------------------------

    #[test]
    fn record_events() {
        let mut recorder = EventRecorder::new(100);
        let entity = EntityId::new();
        let now = Utc::now();

        assert!(recorder.record(make_event(entity, "speed", 0.0, 30.0, now)));
        assert_eq!(recorder.event_count(), 1);
    }

    #[test]
    fn stop_recording_rejects() {
        let mut recorder = EventRecorder::new(100);
        recorder.stop_recording();

        let entity = EntityId::new();
        assert!(!recorder.record(make_event(entity, "speed", 0.0, 30.0, Utc::now())));
    }

    #[test]
    fn ring_buffer_eviction() {
        let mut recorder = EventRecorder::new(3);
        let entity = EntityId::new();
        let now = Utc::now();

        for i in 0..5 {
            recorder.record(make_event(entity, "x", i as f64, (i + 1) as f64, now));
        }

        assert_eq!(recorder.event_count(), 3);
    }

    #[test]
    fn events_for_entity() {
        let mut recorder = EventRecorder::new(100);
        let e1 = EntityId::new();
        let e2 = EntityId::new();
        let now = Utc::now();

        recorder.record(make_event(e1, "speed", 0.0, 30.0, now));
        recorder.record(make_event(e2, "temp", 20.0, 25.0, now));
        recorder.record(make_event(e1, "speed", 30.0, 50.0, now));

        assert_eq!(recorder.events_for_entity(&e1).len(), 2);
        assert_eq!(recorder.events_for_entity(&e2).len(), 1);
    }

    #[test]
    fn events_in_range() {
        let mut recorder = EventRecorder::new(100);
        let entity = EntityId::new();
        let now = Utc::now();

        recorder.record(make_event(
            entity,
            "a",
            0.0,
            1.0,
            now - Duration::seconds(10),
        ));
        recorder.record(make_event(entity, "b", 0.0, 1.0, now));
        recorder.record(make_event(
            entity,
            "c",
            0.0,
            1.0,
            now + Duration::seconds(10),
        ));

        let range = TimeRange {
            start: now - Duration::seconds(1),
            end: now + Duration::seconds(1),
        };
        assert_eq!(recorder.events_in_range(&range).len(), 1);
    }

    #[test]
    fn summary_statistics() {
        let mut recorder = EventRecorder::new(100);
        let e1 = EntityId::new();
        let e2 = EntityId::new();
        let now = Utc::now();

        recorder.record(make_event(e1, "speed", 0.0, 30.0, now));
        recorder.record(make_event(
            e2,
            "temp",
            20.0,
            25.0,
            now + Duration::seconds(5),
        ));

        let summary = recorder.summary();
        assert_eq!(summary.total_events, 2);
        assert_eq!(summary.unique_entities, 2);
        assert!(summary.time_span.is_some());
    }

    // -----------------------------------------------------------------------
    // Player tests
    // -----------------------------------------------------------------------

    #[test]
    fn step_forward_reconstructs_state() {
        let events = sample_events();
        let entity = events[0].entity_id;
        let mut player = ReplayPlayer::from_events(events);

        player.play();
        player.step_forward(); // speed: 0→30
        assert_eq!(
            player.get_entity_state(&entity, "speed"),
            Some(&serde_json::json!(30.0))
        );

        player.step_forward(); // speed: 30→50
        assert_eq!(
            player.get_entity_state(&entity, "speed"),
            Some(&serde_json::json!(50.0))
        );
    }

    #[test]
    fn step_backward_restores_state() {
        let events = sample_events();
        let entity = events[0].entity_id;
        let mut player = ReplayPlayer::from_events(events);

        player.step_forward(); // speed: 0→30
        player.step_forward(); // speed: 30→50

        player.step_backward(); // undo: 30→50, restore to 30
        assert_eq!(
            player.get_entity_state(&entity, "speed"),
            Some(&serde_json::json!(30.0))
        );
    }

    #[test]
    fn seek_rebuilds_state() {
        let events = sample_events();
        let entity = events[0].entity_id;
        let mut player = ReplayPlayer::from_events(events);

        player.seek(3); // After events 0,1,2 → speed=70
        assert_eq!(player.cursor(), 3);
        assert_eq!(
            player.get_entity_state(&entity, "speed"),
            Some(&serde_json::json!(70.0))
        );
    }

    #[test]
    fn finished_at_end() {
        let events = sample_events();
        let total = events.len();
        let mut player = ReplayPlayer::from_events(events);

        for _ in 0..total {
            player.step_forward();
        }
        assert_eq!(player.status(), ReplayStatus::Finished);
    }

    #[test]
    fn bookmark_and_goto() {
        let events = sample_events();
        let mut player = ReplayPlayer::from_events(events);

        player.seek(2);
        let bm = player.add_bookmark("interesting point");

        player.seek(4);
        assert!(player.goto_bookmark(&bm));
        // Should be back near position 2.
        assert!(player.cursor() <= 3);
    }

    #[test]
    fn progress_percentage() {
        let events = sample_events();
        let total = events.len();
        let mut player = ReplayPlayer::from_events(events);

        assert!((player.progress_pct() - 0.0).abs() < f64::EPSILON);

        player.seek(total);
        assert!((player.progress_pct() - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn pause_and_resume() {
        let events = sample_events();
        let mut player = ReplayPlayer::from_events(events);

        player.play();
        assert_eq!(player.status(), ReplayStatus::Playing);

        player.pause();
        assert_eq!(player.status(), ReplayStatus::Paused);
    }

    #[test]
    fn empty_player() {
        let mut player = ReplayPlayer::from_events(vec![]);
        assert!(player.step_forward().is_none());
        assert_eq!(player.status(), ReplayStatus::Finished);
        assert!((player.progress_pct() - 100.0).abs() < f64::EPSILON);
    }
}
