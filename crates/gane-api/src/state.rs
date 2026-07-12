//! Application state shared across API handlers.

use gane_auth::api_key::ApiKeyStore;
use gane_auth::jwt::JwtManager;
use gane_auth::rbac::PolicyEngine;
use gane_auth::session::SessionManager;
use gane_continuity::ContinuityManager;
use gane_core::types::FusedPosition;
use gane_events::EventBus;
use gane_fusion::FusionEngine;
use gane_gnss::ConstellationManager;
use gane_integrity::IntegrityEngine;
use gane_metrics::registry::MetricRegistry;
use gane_observability::probes::ProbeManager;
use gane_security::headers::SecurityHeadersConfig;
use gane_security::rate_limit::RateLimiter;
use gane_telemetry::TelemetryRecorder;
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
    pub metrics: Arc<MetricRegistry>,
    pub probes: Arc<ProbeManager>,
    pub jwt_manager: Arc<JwtManager>,
    pub api_key_store: Arc<RwLock<ApiKeyStore>>,
    pub policy_engine: Arc<RwLock<PolicyEngine>>,
    pub session_manager: Arc<RwLock<SessionManager>>,
    pub rate_limiter: Arc<RateLimiter>,
    pub security_headers: Arc<SecurityHeadersConfig>,
}

impl AppState {
    pub fn new() -> Self {
        let policy_engine = PolicyEngine::new();

        Self {
            gnss: Arc::new(RwLock::new(ConstellationManager::new())),
            fusion: Arc::new(RwLock::new(FusionEngine::new())),
            integrity: Arc::new(RwLock::new(IntegrityEngine::new())),
            continuity: Arc::new(RwLock::new(ContinuityManager::new())),
            telemetry: Arc::new(TelemetryRecorder::new(50_000)),
            event_bus: Arc::new(EventBus::new()),
            last_position: Arc::new(RwLock::new(None)),
            metrics: Arc::new(MetricRegistry::new()),
            probes: Arc::new(ProbeManager::new()),
            jwt_manager: Arc::new(JwtManager::default()),
            api_key_store: Arc::new(RwLock::new(ApiKeyStore::new())),
            policy_engine: Arc::new(RwLock::new(policy_engine)),
            session_manager: Arc::new(RwLock::new(SessionManager::default())),
            rate_limiter: Arc::new(RateLimiter::default()),
            security_headers: Arc::new(SecurityHeadersConfig::default()),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
