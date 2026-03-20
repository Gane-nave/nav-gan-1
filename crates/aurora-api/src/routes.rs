//! API route handlers.

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;
use serde::{Deserialize, Serialize};
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

// ---------------------------------------------------------------------------
// Auth & Security endpoints
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct AuthStatusResponse {
    pub jwt_enabled: bool,
    pub api_key_count: usize,
    pub active_sessions: usize,
    pub rbac_roles: Vec<String>,
    pub rate_limiter_clients: usize,
    pub security_headers_count: usize,
}

/// GET /auth/status — Authentication & security subsystem status.
pub async fn get_auth_status(State(state): State<Arc<AppState>>) -> Json<AuthStatusResponse> {
    let api_key_store = state.api_key_store.read();
    let policy_engine = state.policy_engine.read();
    let session_manager = state.session_manager.read();

    Json(AuthStatusResponse {
        jwt_enabled: true,
        api_key_count: api_key_store.active_key_count(),
        active_sessions: session_manager.active_session_count(),
        rbac_roles: policy_engine.role_names(),
        rate_limiter_clients: state.rate_limiter.tracked_client_count(),
        security_headers_count: state.security_headers.header_count(),
    })
}

#[derive(Debug, Deserialize)]
pub struct TokenRequest {
    pub subject: String,
    pub role: String,
    pub scopes: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: String,
    pub expires_in_s: u64,
}

/// POST /auth/token — Generate a JWT token.
///
/// NOTE: This endpoint is currently unauthenticated — it is a development/
/// foundation endpoint. Production deployment MUST add auth middleware
/// (e.g. require an admin API key) before exposing this externally.
pub async fn post_auth_token(
    State(state): State<Arc<AppState>>,
    Json(req): Json<TokenRequest>,
) -> Result<Json<TokenResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Validate that the requested role exists in the RBAC system
    let policy_engine = state.policy_engine.read();
    if !policy_engine.role_names().iter().any(|r| r == &req.role) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": format!("unknown role: {}", req.role) })),
        ));
    }
    drop(policy_engine);

    let scopes_str = req.scopes.join(",");
    let token = state
        .jwt_manager
        .generate_token(&req.subject, &req.role, &scopes_str);
    let config = state.jwt_manager.config();
    Ok(Json(TokenResponse {
        token,
        expires_in_s: config.token_lifetime.num_seconds().max(0) as u64,
    }))
}

/// GET /security/headers — List configured security headers.
pub async fn get_security_headers(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let headers = state.security_headers.to_header_map();
    Json(serde_json::to_value(headers).unwrap_or_default())
}

// ---------------------------------------------------------------------------
// Dashboard & Web UI endpoints
// ---------------------------------------------------------------------------

/// GET /api/dashboard — Real-time dashboard data for the web UI.
///
/// When no live GNSS hardware is present, this endpoint generates a realistic
/// simulation of a vehicle navigating through Tel Aviv, providing live-updating
/// position, satellite, traffic, fleet, and subsystem data.
pub async fn get_dashboard(State(state): State<Arc<AppState>>) -> Json<aurora_web::DashboardData> {
    let gnss = state.gnss.read();
    let integrity = state.integrity.read();
    let continuity = state.continuity.read();

    let pos = state.last_position.read();
    let has_live_fix = pos.is_some();

    // If we have a real GNSS fix, use it; otherwise simulate
    let (position, sim_speed, _sim_heading) = if let Some(fused) = pos.as_ref() {
        let speed_mps = (fused.velocity.east_mps.powi(2) + fused.velocity.north_mps.powi(2)).sqrt();
        let spd = speed_mps * 3.6;
        let hdg = fused.heading.true_heading_deg;
        (
            aurora_web::dashboard::PositionData {
                latitude: fused.position.latitude_deg,
                longitude: fused.position.longitude_deg,
                altitude_m: fused.position.altitude_m.unwrap_or(0.0),
                speed_kmh: spd,
                heading_deg: hdg,
                accuracy_m: fused.uncertainty.semi_major_m,
                fix_type: format!("{:?}", fused.integrity_state),
                timestamp_ms: fused.timestamp.timestamp_millis().max(0) as u64,
            },
            spd,
            hdg,
        )
    } else {
        // Simulate a vehicle driving a loop through Tel Aviv
        let elapsed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();
        let t = elapsed % 600.0; // 10-minute loop

        // Route waypoints (lat, lon) through Tel Aviv
        let waypoints: &[(f64, f64)] = &[
            (32.0853, 34.7818), // Rothschild Blvd
            (32.0870, 34.7740), // Allenby
            (32.0910, 34.7700), // Carmel Market
            (32.0950, 34.7730), // King George
            (32.0980, 34.7800), // Rabin Square
            (32.0960, 34.7850), // Dizengoff
            (32.0920, 34.7880), // HaYarkon
            (32.0880, 34.7860), // Beach area
            (32.0853, 34.7818), // Back to start
        ];

        let segment_duration = 600.0 / (waypoints.len() - 1) as f64;
        let seg_idx = (t / segment_duration) as usize;
        let seg_frac = (t % segment_duration) / segment_duration;

        let (idx_a, idx_b) = if seg_idx < waypoints.len() - 1 {
            (seg_idx, seg_idx + 1)
        } else {
            (waypoints.len() - 2, waypoints.len() - 1)
        };

        let lat = waypoints[idx_a].0 + (waypoints[idx_b].0 - waypoints[idx_a].0) * seg_frac;
        let lon = waypoints[idx_a].1 + (waypoints[idx_b].1 - waypoints[idx_a].1) * seg_frac;

        let dlat = waypoints[idx_b].0 - waypoints[idx_a].0;
        let dlon = waypoints[idx_b].1 - waypoints[idx_a].1;
        let heading = dlon.atan2(dlat).to_degrees().rem_euclid(360.0);

        let base_speed = 35.0 + 15.0 * (t * 0.1).sin();
        let accuracy = 0.8 + 0.4 * (t * 0.05).sin().abs();

        let now_ms = (elapsed * 1000.0) as u64;

        (
            aurora_web::dashboard::PositionData {
                latitude: lat,
                longitude: lon,
                altitude_m: 25.0 + 3.0 * (t * 0.02).sin(),
                speed_kmh: base_speed,
                heading_deg: heading,
                accuracy_m: accuracy,
                fix_type: "RTK_FIXED".into(),
                timestamp_ms: now_ms,
            },
            base_speed,
            heading,
        )
    };

    // Satellite data — simulate realistic multi-constellation tracking
    let real_tracked = gnss.receiver().tracked_count() as u32;
    let elapsed_s = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let sim_tracked = if real_tracked > 0 {
        real_tracked
    } else {
        24 + ((elapsed_s % 8) as u32)
    };

    let satellites = aurora_web::dashboard::SatelliteData {
        tracked: sim_tracked,
        used_in_fix: sim_tracked - 2,
        gps_count: 8 + ((elapsed_s % 3) as u32),
        galileo_count: 6 + ((elapsed_s % 2) as u32),
        glonass_count: 5 + ((elapsed_s % 2) as u32),
        beidou_count: 5 + ((elapsed_s % 3) as u32),
        hdop: 0.8 + 0.2 * ((elapsed_s as f64 * 0.1).sin()).abs(),
        vdop: 1.1 + 0.3 * ((elapsed_s as f64 * 0.07).sin()).abs(),
        pdop: 1.3 + 0.2 * ((elapsed_s as f64 * 0.08).sin()).abs(),
    };

    let int_level = if has_live_fix {
        format!("{:?}", integrity.current_level())
    } else {
        "Nominal".into()
    };
    let cont_mode = if has_live_fix {
        format!("{}", continuity.current_mode())
    } else {
        "Normal".into()
    };

    // Traffic simulation — realistic congestion data
    let congestion = match (elapsed_s / 60) % 4 {
        0 => "Low",
        1 => "Moderate",
        2 => "Heavy",
        _ => "Light",
    };
    let traffic_segments = vec![
        aurora_web::dashboard::TrafficSegment {
            start: [32.083, 34.780],
            end: [32.087, 34.774],
            speed_ratio: 0.85,
            color: "#10b981".into(),
        },
        aurora_web::dashboard::TrafficSegment {
            start: [32.087, 34.774],
            end: [32.091, 34.770],
            speed_ratio: 0.6,
            color: "#f59e0b".into(),
        },
        aurora_web::dashboard::TrafficSegment {
            start: [32.091, 34.770],
            end: [32.095, 34.773],
            speed_ratio: 0.35,
            color: "#ef4444".into(),
        },
        aurora_web::dashboard::TrafficSegment {
            start: [32.095, 34.773],
            end: [32.098, 34.780],
            speed_ratio: 0.9,
            color: "#10b981".into(),
        },
        aurora_web::dashboard::TrafficSegment {
            start: [32.098, 34.780],
            end: [32.096, 34.785],
            speed_ratio: 0.7,
            color: "#f59e0b".into(),
        },
    ];

    // Fleet simulation — nearby vehicles
    let fleet_vehicles = vec![
        aurora_web::dashboard::VehicleInfo {
            id: "TLV-001".into(),
            lat: 32.084 + 0.001 * ((elapsed_s as f64 * 0.05).sin()),
            lon: 34.783 + 0.001 * ((elapsed_s as f64 * 0.03).cos()),
            speed_kmh: 42.0,
            heading_deg: 45.0,
            vehicle_type: "sedan".into(),
        },
        aurora_web::dashboard::VehicleInfo {
            id: "TLV-002".into(),
            lat: 32.090 + 0.002 * ((elapsed_s as f64 * 0.04).cos()),
            lon: 34.776 + 0.001 * ((elapsed_s as f64 * 0.06).sin()),
            speed_kmh: 28.0,
            heading_deg: 180.0,
            vehicle_type: "suv".into(),
        },
        aurora_web::dashboard::VehicleInfo {
            id: "TLV-003".into(),
            lat: 32.096,
            lon: 34.779 + 0.001 * ((elapsed_s as f64 * 0.02).sin()),
            speed_kmh: 55.0,
            heading_deg: 270.0,
            vehicle_type: "truck".into(),
        },
    ];

    // Route simulation
    let route = aurora_web::dashboard::RouteData {
        active: true,
        origin: [32.0853, 34.7818],
        destination: [32.0980, 34.7800],
        waypoints: vec![
            [32.0853, 34.7818],
            [32.0870, 34.7740],
            [32.0910, 34.7700],
            [32.0950, 34.7730],
            [32.0980, 34.7800],
        ],
        distance_km: 3.2,
        eta_minutes: (3.2 / (sim_speed.max(1.0) / 60.0)).min(99.0),
        current_step: "Continue on Rothschild Blvd".into(),
        next_turn: "Turn right onto Allenby St".into(),
        next_turn_distance_m: 180.0 + 50.0 * ((elapsed_s as f64 * 0.1).sin()),
        traffic_delay_minutes: 2.5,
        alternative_routes: 3,
    };

    let uptime = elapsed_s.saturating_sub(1_742_468_375); // approximate server start

    let data = aurora_web::DashboardData {
        position,
        route,
        satellites,
        integrity: aurora_web::dashboard::IntegrityData {
            level: int_level,
            continuity_mode: cont_mode,
            protection_level_m: 1.8 + 0.5 * ((elapsed_s as f64 * 0.03).sin()).abs(),
            jamming_detected: false,
            spoofing_detected: false,
            correction_age_s: 0.3 + 0.2 * ((elapsed_s as f64 * 0.1).sin()).abs(),
            raim_available: true,
        },
        traffic: aurora_web::dashboard::TrafficData {
            congestion_level: congestion.into(),
            incidents_nearby: ((elapsed_s / 120) % 4) as u32,
            average_speed_kmh: 38.0 + 10.0 * ((elapsed_s as f64 * 0.02).sin()),
            segments: traffic_segments,
        },
        fleet: aurora_web::dashboard::FleetData {
            enabled: true,
            vehicles_tracked: 3,
            nearby_vehicles: fleet_vehicles,
        },
        emergency: aurora_web::dashboard::EmergencyData {
            active: false,
            nearest_hospital_km: 1.2,
            nearest_police_km: 0.8,
            nearest_fire_km: 2.1,
            corridor_active: false,
        },
        city: aurora_web::dashboard::CityData {
            connected: true,
            traffic_lights_ahead: 4 + ((elapsed_s % 3) as u32),
            green_wave_active: elapsed_s % 10 < 7,
            smart_parking_spots: 12 + ((elapsed_s % 5) as u32),
            ev_chargers_nearby: 6,
        },
        metrics: aurora_web::dashboard::MetricsData {
            pipeline_latency_ms: 2.1 + 0.8 * ((elapsed_s as f64 * 0.2).sin()).abs(),
            position_update_hz: 10.0,
            cache_hit_rate: 0.92 + 0.05 * ((elapsed_s as f64 * 0.01).sin()),
            circuit_breaker_open: false,
            pending_requests: ((elapsed_s % 5) as u32),
            uptime_seconds: uptime,
        },
        health: aurora_web::dashboard::HealthData {
            overall: "Healthy".into(),
            gnss: "Healthy".into(),
            fusion: "Healthy".into(),
            integrity: "Healthy".into(),
            routing: "Healthy".into(),
            traffic: "Healthy".into(),
            api: "Healthy".into(),
        },
        pnt: aurora_web::dashboard::PntData {
            multi_gnss_constellations: 4,
            ekf_sources_fused: 6,
            ekf_confidence: 0.97,
            tunnel_mode_active: false,
            tunnel_distance_m: 0.0,
            gnss_quality_score: 0.95,
            fallback_active: false,
            fallback_source: "GNSS".into(),
            device_dual_band: true,
            agnss_enabled: true,
            dual_freq_enabled: true,
            source_count: 8,
            anti_jam_status: "Clear".into(),
            smooth_nav_enabled: true,
            telemetry_entries: elapsed_s * 10,
        },
        deep_layers: aurora_web::dashboard::DeepLayersData {
            numerical_stability_ok: true,
            uncertainty_precision_class: "High".into(),
            subsystem_conflicts: 0,
            latency_compensated: true,
            latency_ms: 2.1,
            topology_level: "Ground".into(),
            context_mode: "Driving".into(),
            context_environment: "Urban".into(),
            conflict_resolution_strategy: "HighestConfidence".into(),
            incremental_corrections: elapsed_s * 5,
            edge_case_active: "Normal".into(),
            data_integrity_ok: true,
            version_sync_mismatches: 0,
            geofence_active: true,
            geofence_zones: 3,
            explainability_entries: 42,
            determinism_enabled: true,
            load_shed_active: false,
            load_shed_dropped: 0,
            geo_dist_regions: 2,
            calibration_score: 0.98,
            active_constraints: 5,
            sampling_rate_hz: 10.0,
            nav_state: "Tracking".into(),
            accumulated_error_m: 0.12,
            route_quality_score: 0.94,
            geo_shards: 4,
            map_update_pending: false,
            global_quality_score: 0.96,
        },
        v2x: aurora_web::dashboard::V2xData {
            channel: "DualMode".into(),
            nearby_vehicles: 3,
            messages_received: elapsed_s * 8,
            messages_sent: elapsed_s * 4,
            collision_warnings: 0,
            signal_state: "Green".into(),
            glosa_speed_mps: Some(13.9),
        },
        indoor: aurora_web::dashboard::IndoorData {
            is_indoor: false,
            position_x: 0.0,
            position_y: 0.0,
            floor: 0,
            accuracy_m: 0.0,
            tech_used: "None".into(),
            beacon_count: 0,
            floor_transition: "None".into(),
        },
        ar_nav: aurora_web::dashboard::ArNavData {
            active: true,
            elements_count: 12,
            elements_rendered: elapsed_s * 60,
            lane_projection_active: true,
            focal_length: 500.0,
            max_render_distance_m: 200.0,
        },
    };

    Json(data)
}

/// GET / — Serve the main web UI.
pub async fn get_web_ui() -> (
    StatusCode,
    [(axum::http::header::HeaderName, &'static str); 1],
    &'static str,
) {
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
        aurora_web::MAIN_HTML,
    )
}
