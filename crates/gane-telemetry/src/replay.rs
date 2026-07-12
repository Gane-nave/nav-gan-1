//! Replay controller — plays back recorded telemetry for diagnostics.

use chrono::{DateTime, Utc};
use gane_core::sync::ReplaySession;
use gane_core::types::EntityId;

use crate::recorder::{TelemetryRecorder, TelemetrySample};

/// Controls replay of recorded telemetry sessions.
pub struct ReplayController {
    sessions: Vec<ReplaySessionState>,
}

struct ReplaySessionState {
    session: ReplaySession,
    samples: Vec<TelemetrySample>,
    current_index: usize,
    playing: bool,
    playback_speed: f64,
}

impl ReplayController {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
        }
    }

    /// Create a new replay session from a time range in the recorder.
    pub fn create_session(
        &mut self,
        recorder: &TelemetryRecorder,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        description: Option<String>,
    ) -> EntityId {
        let samples = recorder.samples_in_range(from, to);
        let session = ReplaySession {
            id: EntityId::new(),
            start_time: from,
            end_time: to,
            device_id: EntityId::new(),
            description,
            status: gane_core::sync::ReplayStatus::Completed,
            created_at: Utc::now(),
        };

        let id = session.id;
        self.sessions.push(ReplaySessionState {
            session,
            samples,
            current_index: 0,
            playing: false,
            playback_speed: 1.0,
        });

        id
    }

    /// Start playback of a session.
    pub fn play(&mut self, session_id: EntityId) -> bool {
        if let Some(state) = self
            .sessions
            .iter_mut()
            .find(|s| s.session.id == session_id)
        {
            state.playing = true;
            state.current_index = 0;
            true
        } else {
            false
        }
    }

    /// Get the next sample in playback.
    pub fn next_sample(&mut self, session_id: EntityId) -> Option<TelemetrySample> {
        let state = self
            .sessions
            .iter_mut()
            .find(|s| s.session.id == session_id)?;

        if !state.playing || state.current_index >= state.samples.len() {
            state.playing = false;
            return None;
        }

        let sample = state.samples[state.current_index].clone();
        state.current_index += 1;
        Some(sample)
    }

    /// Stop playback.
    pub fn stop(&mut self, session_id: EntityId) {
        if let Some(state) = self
            .sessions
            .iter_mut()
            .find(|s| s.session.id == session_id)
        {
            state.playing = false;
        }
    }

    /// Set playback speed (1.0 = real-time, 2.0 = 2x, etc.).
    pub fn set_speed(&mut self, session_id: EntityId, speed: f64) {
        if let Some(state) = self
            .sessions
            .iter_mut()
            .find(|s| s.session.id == session_id)
        {
            state.playback_speed = speed.clamp(0.1, 100.0);
        }
    }

    /// Get all session IDs.
    pub fn session_ids(&self) -> Vec<EntityId> {
        self.sessions.iter().map(|s| s.session.id).collect()
    }

    /// Get playback progress (0.0 to 1.0).
    pub fn progress(&self, session_id: EntityId) -> Option<f64> {
        self.sessions
            .iter()
            .find(|s| s.session.id == session_id)
            .map(|s| {
                if s.samples.is_empty() {
                    0.0
                } else {
                    s.current_index as f64 / s.samples.len() as f64
                }
            })
    }
}

impl Default for ReplayController {
    fn default() -> Self {
        Self::new()
    }
}
