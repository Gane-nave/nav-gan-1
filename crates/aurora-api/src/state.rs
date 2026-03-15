//! Application state shared across API handlers.

use aurora_continuity::ContinuityManager;
use aurora_core::types::FusedPosition;
use aurora_events::EventBus;
use aurora_fusion::FusionEngine;
use aurora_gnss::ConstellationManager;
use aurora_integrity::IntegrityEngine;
use aurora_telemetry::TelemetryRecorder;
use parking_lot::RwLock;
use std::sync::Arc;

/// Shared application state accessible from all API handlers.
pub struct AppState {
    pub gnss: Arc<RwLock<ConstellationManager>>,
    pub fusion: Arc<RwLock<FusionEngine>>,
    pub integrity: Arc<RwLock<IntegrityEngine>>,
    pub continuity: Arc<RwLock<ContinuityManager>>,
    pub telemetry: Arc<TelemetryRecorder>,
    pub event_bus: Arc<EventBus>,
    pub last_position: Arc<RwLock<Option<FusedPosition>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            gnss: Arc::new(RwLock::new(ConstellationManager::new())),
            fusion: Arc::new(RwLock::new(FusionEngine::new())),
            integrity: Arc::new(RwLock::new(IntegrityEngine::new())),
            continuity: Arc::new(RwLock::new(ContinuityManager::new())),
            telemetry: Arc::new(TelemetryRecorder::new(50_000)),
            event_bus: Arc::new(EventBus::new()),
            last_position: Arc::new(RwLock::new(None)),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
