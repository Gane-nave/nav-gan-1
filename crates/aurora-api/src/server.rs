//! HTTP server setup and configuration.

use axum::routing::{get, post};
use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::routes;
use crate::state::AppState;

/// Build the Axum router with all routes.
pub fn build_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(routes::health))
        .route("/position", get(routes::get_position))
        .route("/integrity", get(routes::get_integrity))
        .route("/status", get(routes::get_status))
        .route("/telemetry", get(routes::get_telemetry))
        .route("/constellation", get(routes::get_constellations))
        .route("/metrics", get(routes::get_metrics))
        .route("/openapi.json", get(routes::get_openapi))
        .route("/swagger-ui", get(routes::get_swagger_ui))
        .route("/readiness", get(routes::get_readiness))
        .route("/liveness", get(routes::get_liveness))
        .route("/auth/status", get(routes::get_auth_status))
        .route("/auth/token", post(routes::post_auth_token))
        .route("/security/headers", get(routes::get_security_headers))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// Start the API server on the given address.
pub async fn run_server(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(AppState::new());
    let app = build_router(state);

    info!("AURORA NAV API server starting on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Create a default server on port 3000.
pub async fn run_default() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    run_server(addr).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt;

    fn test_app() -> Router {
        let state = Arc::new(AppState::new());
        build_router(state)
    }

    #[tokio::test]
    async fn health_endpoint_returns_200() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn position_returns_503_when_no_fix() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/position")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn status_endpoint_returns_200() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/status")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn integrity_endpoint_returns_200() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/integrity")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn metrics_endpoint_returns_200() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/metrics")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn openapi_endpoint_returns_json() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/openapi.json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn swagger_ui_endpoint_returns_html() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/swagger-ui")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn readiness_returns_503_before_mark_ready() {
        let state = Arc::new(AppState::new());
        let app = build_router(state.clone());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/readiness")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn readiness_returns_200_after_mark_ready() {
        let state = Arc::new(AppState::new());
        state.probes.mark_ready();
        let app = build_router(state);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/readiness")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn liveness_returns_200() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/liveness")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
