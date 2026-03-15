//! Application state shared across API handlers.

use aurora_auth::api_key::ApiKeyStore;
use aurora_auth::jwt::JwtManager;
use aurora_auth::rbac::PolicyEngine;
use aurora_auth::session::SessionManager;
use aurora_continuity::ContinuityManager;
use aurora_core::types::FusedPosition;
use aurora_events::EventBus;
use aurora_fusion::FusionEngine;
use aurora_gnss::ConstellationManager;
use aurora_integrity::IntegrityEngine;
use aurora_metrics::registry::MetricRegistry;
use aurora_observability::probes::ProbeManager;
use aurora_security::headers::SecurityHeadersConfig;
use aurora_security::rate_limit::RateLimiter;
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
