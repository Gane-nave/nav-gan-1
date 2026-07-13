//! Telemetry recorder — captures navigation pipeline data for diagnostics.

use chrono::{DateTime, Utc};
use gane_core::types::{ContinuityMode, FusedPosition, IntegrityLevel, NavigationSource};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// A single telemetry sample capturing the full pipeline state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetrySample {
    pub timestamp: DateTime<Utc>,
    pub sample_type: TelemetrySampleType,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TelemetrySampleType {
    GnssRaw,
    FusionOutput,
    IntegrityCheck,
    ModeTransition,
    ThreatAlert,
    TrustScoreChange,
    SensorHealth,
    CorrectionStatus,
    PerformanceMetric,
}

/// Ring-buffer telemetry recorder with configurable capacity.
pub struct TelemetryRecorder {
    buffer: Mutex<VecDeque<TelemetrySample>>,
    capacity: usize,
    total_recorded: Mutex<u64>,
    recording: Mutex<bool>,
}

impl TelemetryRecorder {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Mutex::new(VecDeque::with_capacity(capacity)),
            capacity,
            total_recorded: Mutex::new(0),
            recording: Mutex::new(true),
        }
    }

    /// Record a telemetry sample.
    pub fn record(&self, sample: TelemetrySample) {
        if !*self.recording.lock() {
            return;
        }

        let mut buf = self.buffer.lock();
        if buf.len() >= self.capacity {
            buf.pop_front();
        }
        buf.push_back(sample);
        *self.total_recorded.lock() += 1;
    }

    /// Record a fusion output.
    pub fn record_fusion(&self, position: &FusedPosition) {
        if let Ok(data) = serde_json::to_value(position) {
            self.record(TelemetrySample {
                timestamp: position.timestamp,
                sample_type: TelemetrySampleType::FusionOutput,
                data,
            });
        }
    }

    /// Record a mode transition.
    pub fn record_mode_transition(&self, from: ContinuityMode, to: ContinuityMode, reason: &str) {
        self.record(TelemetrySample {
            timestamp: Utc::now(),
            sample_type: TelemetrySampleType::ModeTransition,
            data: serde_json::json!({
                "from": format!("{from}"),
                "to": format!("{to}"),
                "reason": reason,
            }),
        });
    }

    /// Record an integrity check result.
    pub fn record_integrity(&self, level: IntegrityLevel, details: &str) {
        self.record(TelemetrySample {
            timestamp: Utc::now(),
            sample_type: TelemetrySampleType::IntegrityCheck,
            data: serde_json::json!({
                "level": format!("{level:?}"),
                "details": details,
            }),
        });
    }

    /// Record a trust score change.
    pub fn record_trust_change(&self, source: NavigationSource, old: f64, new: f64, reason: &str) {
        self.record(TelemetrySample {
            timestamp: Utc::now(),
            sample_type: TelemetrySampleType::TrustScoreChange,
            data: serde_json::json!({
                "source": format!("{source:?}"),
                "old_score": old,
                "new_score": new,
                "reason": reason,
            }),
        });
    }

    /// Get all samples in the buffer.
    pub fn samples(&self) -> Vec<TelemetrySample> {
        self.buffer.lock().iter().cloned().collect()
    }

    /// Get samples of a specific type.
    pub fn samples_of_type(&self, sample_type: TelemetrySampleType) -> Vec<TelemetrySample> {
        self.buffer
            .lock()
            .iter()
            .filter(|s| s.sample_type == sample_type)
            .cloned()
            .collect()
    }

    /// Get samples within a time range.
    pub fn samples_in_range(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> Vec<TelemetrySample> {
        self.buffer
            .lock()
            .iter()
            .filter(|s| (from..=to).contains(&s.timestamp))
            .cloned()
            .collect()
    }

    /// Total number of samples recorded (including evicted ones).
    pub fn total_recorded(&self) -> u64 {
        *self.total_recorded.lock()
    }

    /// Current buffer size.
    pub fn buffer_size(&self) -> usize {
        self.buffer.lock().len()
    }

    /// Pause recording.
    pub fn pause(&self) {
        *self.recording.lock() = false;
    }

    /// Resume recording.
    pub fn resume(&self) {
        *self.recording.lock() = true;
    }

    /// Clear the buffer.
    pub fn clear(&self) {
        self.buffer.lock().clear();
    }

    /// Export all samples as JSON.
    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        let samples = self.samples();
        serde_json::to_string_pretty(&samples)
    }
}

impl Default for TelemetryRecorder {
    fn default() -> Self {
        Self::new(10_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_and_retrieves_samples() {
        let recorder = TelemetryRecorder::new(100);
        recorder.record(TelemetrySample {
            timestamp: Utc::now(),
            sample_type: TelemetrySampleType::PerformanceMetric,
            data: serde_json::json!({"test": true}),
        });

        assert_eq!(recorder.buffer_size(), 1);
        assert_eq!(recorder.total_recorded(), 1);
    }

    #[test]
    fn ring_buffer_evicts_oldest() {
        let recorder = TelemetryRecorder::new(2);
        for i in 0..5 {
            recorder.record(TelemetrySample {
                timestamp: Utc::now(),
                sample_type: TelemetrySampleType::PerformanceMetric,
                data: serde_json::json!({"index": i}),
            });
        }

        assert_eq!(recorder.buffer_size(), 2);
        assert_eq!(recorder.total_recorded(), 5);
    }

    #[test]
    fn pause_stops_recording() {
        let recorder = TelemetryRecorder::new(100);
        recorder.pause();
        recorder.record(TelemetrySample {
            timestamp: Utc::now(),
            sample_type: TelemetrySampleType::PerformanceMetric,
            data: serde_json::json!({}),
        });
        assert_eq!(recorder.buffer_size(), 0);
    }
}
