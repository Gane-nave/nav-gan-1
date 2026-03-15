//! API route handlers.

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;
use serde::Serialize;
use std::sync::Arc;

use crate::state::AppState;
use aurora_metrics::export::render_prometheus;
use aurora_observability::openapi;

// ---------------------------------------------------------------------------
// Response types
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_s: f64,
    pub event_count: u64,
    pub telemetry_buffer_size: usize,
}

#[derive(Debug, Serialize)]
pub struct PositionResponse {
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub altitude_m: Option<f64>,
    pub heading_deg: f64,
    pub speed_mps: f64,
    pub horizontal_accuracy_m: f64,
    pub vertical_accuracy_m: f64,
    pub confidence: f64,
    pub integrity: String,
    pub continuity_mode: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize)]
pub struct IntegrityResponse {
    pub level: String,
    pub continuity_mode: String,
    pub recovery_pending: bool,
    pub time_in_mode_s: f64,
    pub source_count: usize,
}

#[derive(Debug, Serialize)]
pub struct SystemStatusResponse {
    pub gnss_tracked_satellites: usize,
    pub fusion_measurement_count: u64,
    pub integrity_level: String,
    pub continuity_mode: String,
    pub telemetry_samples: usize,
    pub event_bus_subscribers: usize,
    pub event_bus_total_events: u64,
}

#[derive(Debug, Serialize)]
pub struct TelemetryResponse {
    pub total_recorded: u64,
    pub buffer_size: usize,
    pub samples: Vec<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// GET /health
pub async fn health(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "operational".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_s: 0.0, // Would track actual uptime in production.
        event_count: state.event_bus.total_events(),
        telemetry_buffer_size: state.telemetry.buffer_size(),
    })
}

/// GET /position
pub async fn get_position(
    State(state): State<Arc<AppState>>,
) -> Result<Json<PositionResponse>, StatusCode> {
    let pos = state.last_position.read();
    match pos.as_ref() {
        Some(fused) => {
            let speed = (fused.velocity.east_mps.powi(2) + fused.velocity.north_mps.powi(2)).sqrt();

            Ok(Json(PositionResponse {
                latitude_deg: fused.position.latitude_deg,
                longitude_deg: fused.position.longitude_deg,
                altitude_m: fused.position.altitude_m,
                heading_deg: fused.heading.true_heading_deg,
                speed_mps: speed,
                horizontal_accuracy_m: fused.uncertainty.semi_major_m,
                vertical_accuracy_m: fused.uncertainty.semi_vertical_m,
                confidence: fused.confidence,
                integrity: format!("{:?}", fused.integrity_state),
                continuity_mode: format!("{}", fused.continuity_mode),
                timestamp: fused.timestamp.to_rfc3339(),
            }))
        }
        None => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

/// GET /integrity
pub async fn get_integrity(State(state): State<Arc<AppState>>) -> Json<IntegrityResponse> {
    let integrity = state.integrity.read();
    let continuity = state.continuity.read();

    Json(IntegrityResponse {
        level: format!("{:?}", integrity.current_level()),
        continuity_mode: format!("{}", continuity.current_mode()),
        recovery_pending: continuity.is_recovery_pending(),
        time_in_mode_s: continuity.time_in_current_mode_s(),
        source_count: integrity.trust_manager().all_scores().len(),
    })
}

/// GET /status
pub async fn get_status(State(state): State<Arc<AppState>>) -> Json<SystemStatusResponse> {
    let gnss = state.gnss.read();
    let fusion = state.fusion.read();
    let integrity = state.integrity.read();
    let continuity = state.continuity.read();

    Json(SystemStatusResponse {
        gnss_tracked_satellites: gnss.receiver().tracked_count(),
        fusion_measurement_count: fusion.measurement_count(),
        integrity_level: format!("{:?}", integrity.current_level()),
        continuity_mode: format!("{}", continuity.current_mode()),
        telemetry_samples: state.telemetry.buffer_size(),
        event_bus_subscribers: state.event_bus.subscriber_count(),
        event_bus_total_events: state.event_bus.total_events(),
    })
}

/// GET /telemetry
pub async fn get_telemetry(State(state): State<Arc<AppState>>) -> Json<TelemetryResponse> {
    let samples: Vec<serde_json::Value> = state
        .telemetry
        .samples()
        .iter()
        .rev()
        .take(100)
        .filter_map(|s| serde_json::to_value(s).ok())
        .collect();

    Json(TelemetryResponse {
        total_recorded: state.telemetry.total_recorded(),
        buffer_size: state.telemetry.buffer_size(),
        samples,
    })
}

/// GET /constellation
pub async fn get_constellations(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<serde_json::Value>> {
    let gnss = state.gnss.read();
    let states: Vec<serde_json::Value> = gnss
        .receiver()
        .constellation_states()
        .values()
        .filter_map(|s| serde_json::to_value(s).ok())
        .collect();

    Json(states)
}

/// GET /metrics — Prometheus text exposition format.
pub async fn get_metrics(
    State(state): State<Arc<AppState>>,
) -> (
    StatusCode,
    [(axum::http::header::HeaderName, &'static str); 1],
    String,
) {
    let body = render_prometheus(&state.metrics);
    (
        StatusCode::OK,
        [(
            axum::http::header::CONTENT_TYPE,
            "text/plain; version=0.0.4; charset=utf-8",
        )],
        body,
    )
}

/// GET /openapi.json — OpenAPI 3.0 specification.
pub async fn get_openapi() -> (StatusCode, Json<serde_json::Value>) {
    let spec = openapi::build_spec();
    let value = serde_json::to_value(spec).unwrap_or_default();
    (StatusCode::OK, Json(value))
}

/// GET /swagger-ui — Swagger UI HTML page.
pub async fn get_swagger_ui() -> (
    StatusCode,
    [(axum::http::header::HeaderName, &'static str); 1],
    String,
) {
    let html = openapi::swagger_ui_html();
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
        html,
    )
}

/// GET /readiness — Kubernetes readiness probe.
pub async fn get_readiness(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<serde_json::Value>) {
    let (code, response) = state.probes.readiness_response();
    let status = if code == 200 {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };
    (
        status,
        Json(serde_json::to_value(response).unwrap_or_default()),
    )
}

/// GET /liveness — Kubernetes liveness probe.
pub async fn get_liveness(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<serde_json::Value>) {
    let (code, response) = state.probes.liveness_response();
    let status = if code == 200 {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };
    (
        status,
        Json(serde_json::to_value(response).unwrap_or_default()),
    )
}
