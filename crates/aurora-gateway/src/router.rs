//! Request routing — matches incoming requests to backend services
//! based on path patterns, headers, and query parameters.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// HTTP method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Options,
    Head,
}

/// A route definition mapping a path pattern to a backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: Uuid,
    pub path_pattern: String,
    pub methods: Vec<HttpMethod>,
    pub backend: String,
    pub strip_prefix: bool,
    pub timeout_ms: u64,
    pub retry_count: u32,
    pub metadata: HashMap<String, String>,
}

/// An incoming request to be routed.
#[derive(Debug, Clone)]
pub struct GatewayRequest {
    pub id: Uuid,
    pub method: HttpMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body_size: usize,
}

/// Result of routing a request.
#[derive(Debug, Clone)]
pub struct RouteMatch {
    pub route: Route,
    pub captured_path: String,
    pub backend_path: String,
}

/// Request router that matches paths to backends.
pub struct RequestRouter {
    routes: Vec<Route>,
}

impl RequestRouter {
    /// Create a new router.
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    /// Add a route.
    pub fn add_route(&mut self, route: Route) {
        self.routes.push(route);
    }

    /// Remove a route by ID.
    pub fn remove_route(&mut self, id: Uuid) -> bool {
        if let Some(idx) = self.routes.iter().position(|r| r.id == id) {
            self.routes.remove(idx);
            true
        } else {
            false
        }
    }

    /// Match a request to a route. Returns the first matching route.
    pub fn route(&self, request: &GatewayRequest) -> Option<RouteMatch> {
        for route in &self.routes {
            if !route.methods.contains(&request.method) {
                continue;
            }

            if let Some(captured) = match_pattern(&route.path_pattern, &request.path) {
                let backend_path = if route.strip_prefix {
                    captured.clone()
                } else {
                    request.path.clone()
                };

                return Some(RouteMatch {
                    route: route.clone(),
                    captured_path: captured,
                    backend_path,
                });
            }
        }
        None
    }

    /// Number of registered routes.
    pub fn route_count(&self) -> usize {
        self.routes.len()
    }

    /// Get all routes.
    pub fn routes(&self) -> &[Route] {
        &self.routes
    }
}

impl Default for RequestRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple path pattern matching.
/// Supports:
/// - Exact match: `/api/v1/health`
/// - Prefix match with wildcard: `/api/v1/*` matches `/api/v1/anything/here`
/// - Single segment wildcard: `/api/v1/:id/details` matches `/api/v1/123/details`
///
/// Returns the captured suffix (for prefix match) or the full path (for exact match).
fn match_pattern(pattern: &str, path: &str) -> Option<String> {
    // Exact match
    if pattern == path {
        return Some(String::new());
    }

    // Prefix wildcard: /prefix/*
    if let Some(prefix) = pattern.strip_suffix("/*") {
        if let Some(suffix) = path.strip_prefix(prefix) {
            if suffix.is_empty() || suffix.starts_with('/') {
                return Some(suffix.to_string());
            }
        }
        return None;
    }

    // Segment wildcard: /prefix/:param/suffix
    let pattern_parts: Vec<&str> = pattern.split('/').collect();
    let path_parts: Vec<&str> = path.split('/').collect();

    if pattern_parts.len() != path_parts.len() {
        return None;
    }

    for (pp, pathp) in pattern_parts.iter().zip(path_parts.iter()) {
        if pp.starts_with(':') {
            continue; // wildcard segment matches anything
        }
        if pp != pathp {
            return None;
        }
    }

    Some(path.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_route(pattern: &str, methods: &[HttpMethod], backend: &str) -> Route {
        Route {
            id: Uuid::new_v4(),
            path_pattern: pattern.to_string(),
            methods: methods.to_vec(),
            backend: backend.to_string(),
            strip_prefix: false,
            timeout_ms: 5000,
            retry_count: 3,
            metadata: HashMap::new(),
        }
    }

    fn make_request(method: HttpMethod, path: &str) -> GatewayRequest {
        GatewayRequest {
            id: Uuid::new_v4(),
            method,
            path: path.to_string(),
            headers: HashMap::new(),
            body_size: 0,
        }
    }

    #[test]
    fn test_exact_match() {
        let mut router = RequestRouter::new();
        router.add_route(make_route("/api/health", &[HttpMethod::Get], "health-svc"));

        let req = make_request(HttpMethod::Get, "/api/health");
        let m = router.route(&req).unwrap();
        assert_eq!(m.route.backend, "health-svc");
    }

    #[test]
    fn test_prefix_wildcard() {
        let mut router = RequestRouter::new();
        router.add_route(make_route(
            "/api/v1/*",
            &[HttpMethod::Get, HttpMethod::Post],
            "api-svc",
        ));

        let req = make_request(HttpMethod::Get, "/api/v1/users/123");
        let m = router.route(&req).unwrap();
        assert_eq!(m.route.backend, "api-svc");
        assert_eq!(m.captured_path, "/users/123");
    }

    #[test]
    fn test_segment_wildcard() {
        let mut router = RequestRouter::new();
        router.add_route(make_route(
            "/api/users/:id/profile",
            &[HttpMethod::Get],
            "profile-svc",
        ));

        let req = make_request(HttpMethod::Get, "/api/users/abc123/profile");
        let m = router.route(&req).unwrap();
        assert_eq!(m.route.backend, "profile-svc");
    }

    #[test]
    fn test_method_mismatch() {
        let mut router = RequestRouter::new();
        router.add_route(make_route("/api/health", &[HttpMethod::Get], "health-svc"));

        let req = make_request(HttpMethod::Post, "/api/health");
        assert!(router.route(&req).is_none());
    }

    #[test]
    fn test_no_match() {
        let mut router = RequestRouter::new();
        router.add_route(make_route("/api/health", &[HttpMethod::Get], "health-svc"));

        let req = make_request(HttpMethod::Get, "/api/other");
        assert!(router.route(&req).is_none());
    }

    #[test]
    fn test_strip_prefix() {
        let mut router = RequestRouter::new();
        let mut route = make_route("/api/v1/*", &[HttpMethod::Get], "api-svc");
        route.strip_prefix = true;
        router.add_route(route);

        let req = make_request(HttpMethod::Get, "/api/v1/users");
        let m = router.route(&req).unwrap();
        assert_eq!(m.backend_path, "/users");
    }

    #[test]
    fn test_remove_route() {
        let mut router = RequestRouter::new();
        let route = make_route("/api/test", &[HttpMethod::Get], "test-svc");
        let rid = route.id;
        router.add_route(route);
        assert_eq!(router.route_count(), 1);
        assert!(router.remove_route(rid));
        assert_eq!(router.route_count(), 0);
    }

    #[test]
    fn test_first_match_wins() {
        let mut router = RequestRouter::new();
        router.add_route(make_route("/api/*", &[HttpMethod::Get], "general-svc"));
        router.add_route(make_route("/api/health", &[HttpMethod::Get], "health-svc"));

        let req = make_request(HttpMethod::Get, "/api/health");
        let m = router.route(&req).unwrap();
        assert_eq!(m.route.backend, "general-svc"); // first match wins
    }

    #[test]
    fn test_serialization() {
        let route = make_route("/api/test", &[HttpMethod::Get], "test-svc");
        let json = serde_json::to_string(&route).unwrap();
        let de: Route = serde_json::from_str(&json).unwrap();
        assert_eq!(de.path_pattern, "/api/test");
        assert_eq!(de.backend, "test-svc");
    }
}
