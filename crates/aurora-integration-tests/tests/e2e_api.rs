//! End-to-end API server integration tests.
//!
//! Proves that the API server, wired through the application pipeline,
//! returns correct responses for all endpoints.

use aurora_api::server::build_router;
use aurora_api::state::AppState;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use std::sync::Arc;
use tower::util::ServiceExt;

fn test_app() -> axum::Router {
    let state = Arc::new(AppState::new());
    build_router(state)
}

#[tokio::test]
async fn api_health_returns_operational() {
    let app = test_app();
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = axum::body::to_bytes(resp.into_body(), 1024 * 1024)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "operational");
    assert_eq!(json["version"], env!("CARGO_PKG_VERSION"));
}

#[tokio::test]
async fn api_position_returns_503_without_fix() {
    let app = test_app();
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/position")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        resp.status(),
        StatusCode::SERVICE_UNAVAILABLE,
        "position must return 503 when no GNSS fix is available"
    );
}

#[tokio::test]
async fn api_integrity_returns_no_solution() {
    let app = test_app();
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/integrity")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = axum::body::to_bytes(resp.into_body(), 1024 * 1024)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["level"], "NoSolution");
}

#[tokio::test]
async fn api_status_returns_valid_json() {
    let app = test_app();
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = axum::body::to_bytes(resp.into_body(), 1024 * 1024)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["gnss_tracked_satellites"].is_number());
    assert!(json["fusion_measurement_count"].is_number());
    assert!(json["event_bus_total_events"].is_number());
}

#[tokio::test]
async fn api_telemetry_returns_empty_initially() {
    let app = test_app();
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/telemetry")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = axum::body::to_bytes(resp.into_body(), 1024 * 1024)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["total_recorded"], 0);
    assert!(json["samples"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn api_constellation_returns_empty_initially() {
    let app = test_app();
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/constellation")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = axum::body::to_bytes(resp.into_body(), 1024 * 1024)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json.as_array().unwrap().is_empty());
}
