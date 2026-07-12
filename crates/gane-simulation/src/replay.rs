//! Route replay engine — records and replays navigation sessions for analysis and testing.

use std::time::{Duration, Instant};

/// A recorded waypoint in a navigation session.
#[derive(Debug, Clone)]
pub struct RecordedWaypoint {
    /// Latitude.
    pub lat: f64,
    /// Longitude.
    pub lon: f64,
    /// Speed at this point (m/s).
    pub speed_mps: f64,
    /// Heading (degrees, 0=north, clockwise).
    pub heading_deg: f64,
    /// Time offset from session start.
    pub time_offset: Duration,
    /// Optional event tag (e.g., "reroute", "arrival").
    pub event: Option<String>,
}

/// Replay playback state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackState {
    /// Not started.
    Stopped,
    /// Currently playing.
    Playing,
    /// Paused.
    Paused,
    /// Reached end of recording.
    Finished,
}

/// Replay speed multiplier.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlaybackSpeed(pub f64);

impl Default for PlaybackSpeed {
    fn default() -> Self {
        Self(1.0)
    }
}

/// Session recording.
#[derive(Debug, Clone)]
pub struct SessionRecording {
    /// Unique recording ID.
    pub id: u64,
    /// Description / label.
    pub label: String,
    /// Recorded waypoints in chronological order.
    pub waypoints: Vec<RecordedWaypoint>,
    /// Total duration of the session.
    pub total_duration: Duration,
}

/// Replay engine for recorded navigation sessions.
pub struct ReplayEngine {
    recordings: Vec<SessionRecording>,
    current_recording: Option<usize>,
    current_index: usize,
    state: PlaybackState,
    speed: PlaybackSpeed,
    started_at: Option<Instant>,
    elapsed_before_pause: Duration,
    next_id: u64,
}

impl ReplayEngine {
    /// Create a new replay engine.
    pub fn new() -> Self {
        Self {
            recordings: Vec::new(),
            current_recording: None,
            current_index: 0,
            state: PlaybackState::Stopped,
            speed: PlaybackSpeed::default(),
            started_at: None,
            elapsed_before_pause: Duration::ZERO,
            next_id: 1,
        }
    }

    /// Add a recording to the library.
    pub fn add_recording(&mut self, label: &str, waypoints: Vec<RecordedWaypoint>) -> u64 {
        let total_duration = waypoints
            .last()
            .map(|w| w.time_offset)
            .unwrap_or(Duration::ZERO);
        let id = self.next_id;
        self.next_id += 1;
        self.recordings.push(SessionRecording {
            id,
            label: label.to_string(),
            waypoints,
            total_duration,
        });
        id
    }

    /// Get the number of recordings.
    pub fn recording_count(&self) -> usize {
        self.recordings.len()
    }

    /// Load a recording for playback by ID.
    pub fn load(&mut self, recording_id: u64) -> bool {
        if let Some(idx) = self.recordings.iter().position(|r| r.id == recording_id) {
            self.current_recording = Some(idx);
            self.current_index = 0;
            self.state = PlaybackState::Stopped;
            self.elapsed_before_pause = Duration::ZERO;
            self.started_at = None;
            true
        } else {
            false
        }
    }

    /// Start or resume playback.
    pub fn play(&mut self) -> bool {
        if self.current_recording.is_none() {
            return false;
        }
        match self.state {
            PlaybackState::Stopped | PlaybackState::Paused => {
                self.state = PlaybackState::Playing;
                self.started_at = Some(Instant::now());
                true
            }
            _ => false,
        }
    }

    /// Pause playback.
    pub fn pause(&mut self) -> bool {
        if self.state != PlaybackState::Playing {
            return false;
        }
        if let Some(started) = self.started_at.take() {
            self.elapsed_before_pause += started.elapsed();
        }
        self.state = PlaybackState::Paused;
        true
    }

    /// Stop playback and reset to beginning.
    pub fn stop(&mut self) {
        self.state = PlaybackState::Stopped;
        self.current_index = 0;
        self.started_at = None;
        self.elapsed_before_pause = Duration::ZERO;
    }

    /// Set playback speed (1.0 = real-time, 2.0 = double speed).
    pub fn set_speed(&mut self, speed: f64) {
        self.speed = PlaybackSpeed(speed.clamp(0.1, 100.0));
    }

    /// Get current playback state.
    pub fn state(&self) -> PlaybackState {
        self.state
    }

    /// Get current playback speed.
    pub fn playback_speed(&self) -> f64 {
        self.speed.0
    }

    /// Advance playback and return the current waypoint, if any.
    pub fn tick(&mut self) -> Option<&RecordedWaypoint> {
        let rec_idx = self.current_recording?;
        let recording = &self.recordings[rec_idx];

        if self.state != PlaybackState::Playing {
            return None;
        }

        let real_elapsed = self.elapsed_before_pause
            + self
                .started_at
                .map(|s| s.elapsed())
                .unwrap_or(Duration::ZERO);
        let sim_elapsed = real_elapsed.mul_f64(self.speed.0);

        // Find the waypoint at or just before current sim time
        if self.current_index < recording.waypoints.len()
            && recording.waypoints[self.current_index].time_offset <= sim_elapsed
        {
            let wp = &recording.waypoints[self.current_index];
            self.current_index += 1;
            return Some(wp);
        }

        if self.current_index >= recording.waypoints.len() {
            self.state = PlaybackState::Finished;
        }

        None
    }

    /// Get waypoint count for current recording.
    pub fn current_waypoint_count(&self) -> usize {
        self.current_recording
            .map(|idx| self.recordings[idx].waypoints.len())
            .unwrap_or(0)
    }

    /// Seek to a specific waypoint index.
    pub fn seek(&mut self, index: usize) -> bool {
        if let Some(rec_idx) = self.current_recording {
            if index < self.recordings[rec_idx].waypoints.len() {
                self.current_index = index;
                return true;
            }
        }
        false
    }
}

impl Default for ReplayEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_waypoints() -> Vec<RecordedWaypoint> {
        vec![
            RecordedWaypoint {
                lat: 32.0,
                lon: 34.0,
                speed_mps: 10.0,
                heading_deg: 0.0,
                time_offset: Duration::from_millis(0),
                event: Some("start".to_string()),
            },
            RecordedWaypoint {
                lat: 32.001,
                lon: 34.001,
                speed_mps: 15.0,
                heading_deg: 45.0,
                time_offset: Duration::from_millis(100),
                event: None,
            },
            RecordedWaypoint {
                lat: 32.002,
                lon: 34.002,
                speed_mps: 20.0,
                heading_deg: 90.0,
                time_offset: Duration::from_millis(200),
                event: Some("arrival".to_string()),
            },
        ]
    }

    #[test]
    fn test_add_recording() {
        let mut engine = ReplayEngine::new();
        let id = engine.add_recording("test route", sample_waypoints());
        assert_eq!(id, 1);
        assert_eq!(engine.recording_count(), 1);
    }

    #[test]
    fn test_sequential_ids() {
        let mut engine = ReplayEngine::new();
        let id1 = engine.add_recording("route 1", vec![]);
        let id2 = engine.add_recording("route 2", vec![]);
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }

    #[test]
    fn test_load_recording() {
        let mut engine = ReplayEngine::new();
        let id = engine.add_recording("test", sample_waypoints());
        assert!(engine.load(id));
        assert_eq!(engine.current_waypoint_count(), 3);
    }

    #[test]
    fn test_load_nonexistent() {
        let mut engine = ReplayEngine::new();
        assert!(!engine.load(999));
    }

    #[test]
    fn test_play_without_load() {
        let mut engine = ReplayEngine::new();
        assert!(!engine.play());
    }

    #[test]
    fn test_play_pause_stop() {
        let mut engine = ReplayEngine::new();
        let id = engine.add_recording("test", sample_waypoints());
        engine.load(id);

        assert!(engine.play());
        assert_eq!(engine.state(), PlaybackState::Playing);

        assert!(engine.pause());
        assert_eq!(engine.state(), PlaybackState::Paused);

        engine.stop();
        assert_eq!(engine.state(), PlaybackState::Stopped);
    }

    #[test]
    fn test_set_speed_clamped() {
        let mut engine = ReplayEngine::new();
        engine.set_speed(0.01); // Below min
        assert!((engine.playback_speed() - 0.1).abs() < 0.01);
        engine.set_speed(200.0); // Above max
        assert!((engine.playback_speed() - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_seek() {
        let mut engine = ReplayEngine::new();
        let id = engine.add_recording("test", sample_waypoints());
        engine.load(id);
        assert!(engine.seek(2));
        assert!(!engine.seek(10)); // Out of bounds
    }

    #[test]
    fn test_tick_returns_waypoints() {
        let mut engine = ReplayEngine::new();
        let id = engine.add_recording("test", sample_waypoints());
        engine.load(id);
        engine.set_speed(100.0); // Fast forward
        engine.play();

        std::thread::sleep(Duration::from_millis(10));

        let mut count = 0;
        while engine.tick().is_some() {
            count += 1;
        }
        assert!(count > 0, "Should have returned at least one waypoint");
    }

    #[test]
    fn test_tick_not_playing_returns_none() {
        let mut engine = ReplayEngine::new();
        let id = engine.add_recording("test", sample_waypoints());
        engine.load(id);
        // Not playing
        assert!(engine.tick().is_none());
    }

    #[test]
    fn test_double_play_fails() {
        let mut engine = ReplayEngine::new();
        let id = engine.add_recording("test", sample_waypoints());
        engine.load(id);
        assert!(engine.play());
        assert!(!engine.play()); // Already playing
    }

    #[test]
    fn test_pause_not_playing_fails() {
        let mut engine = ReplayEngine::new();
        assert!(!engine.pause()); // Not playing
    }
}
