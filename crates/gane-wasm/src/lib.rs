//! G.A.N.E NAV — WebAssembly engine facade.
//!
//! Thin `#[wasm_bindgen]` boundary over the native navigation crates so the
//! exact same fusion/routing code runs in the browser, in Capacitor, and on
//! the server. All FFI payloads are JSON strings validated against the
//! shared contracts; no logic lives in this crate.

use aurora_core::map::RoadGraph;
use aurora_core::vehicle::VehicleEnvelope;
use aurora_fusion::ekf::NavigationEkf;
use aurora_fusion::eskf15::Eskf15;
use aurora_map::graph::RoadGraphIndex;
use aurora_routing::dijkstra::{cost, shortest_path, CostFn};
use aurora_routing::vehicle_aware::by_time_for_vehicle;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Route request crossing the FFI boundary (JSON).
#[derive(Debug, Deserialize)]
struct RouteRequest {
    from_node: String,
    to_node: String,
    /// Optional vehicle envelope — absent ⇒ unconstrained routing.
    envelope: Option<VehicleEnvelope>,
}

/// Route response crossing the FFI boundary (JSON).
#[derive(Debug, Serialize)]
struct RouteResponse {
    nodes: Vec<String>,
    segments: Vec<String>,
    total_cost_s: f64,
    constrained: bool,
}

/// The navigation engine instance exposed to JavaScript.
#[wasm_bindgen]
pub struct GaneEngine {
    ekf: NavigationEkf,
    eskf: Eskf15,
    graph: Option<RoadGraphIndex>,
    graph_data: Option<RoadGraph>,
}

impl Default for GaneEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl GaneEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GaneEngine {
        GaneEngine {
            ekf: NavigationEkf::new(),
            eskf: Eskf15::new(),
            graph: None,
            graph_data: None,
        }
    }

    /// Engine version (single source: the workspace version).
    pub fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    // -- Fusion ------------------------------------------------------------

    /// Advance the filter by `dt` seconds.
    pub fn predict(&mut self, dt_s: f64) {
        self.ekf.predict(dt_s);
    }

    /// Feed a position fix in local ENU metres with 1-sigma accuracy.
    pub fn update_position(&mut self, east_m: f64, north_m: f64, up_m: f64, sigma_m: f64) {
        self.ekf.update_position(east_m, north_m, up_m, sigma_m);
    }

    /// Feed a velocity measurement (ENU, m/s).
    pub fn update_velocity(&mut self, ve: f64, vn: f64, vu: f64, sigma_mps: f64) {
        self.ekf.update_velocity(ve, vn, vu, sigma_mps);
    }

    /// Feed a heading measurement (radians).
    pub fn update_heading(&mut self, heading_rad: f64, sigma_rad: f64) {
        self.ekf.update_heading(heading_rad, sigma_rad);
    }

    /// Fused state as JSON: position/velocity/heading/uncertainty.
    pub fn position(&self) -> String {
        let p = self.ekf.position_enu();
        let v = self.ekf.velocity_enu();
        serde_json::json!({
            "east_m": p.x, "north_m": p.y, "up_m": p.z,
            "ve_mps": v.x, "vn_mps": v.y, "vu_mps": v.z,
            "heading_rad": self.ekf.heading_rad(),
            "horizontal_uncertainty_m": self.ekf.position_uncertainty_m(),
            "vertical_uncertainty_m": self.ekf.vertical_uncertainty_m(),
        })
        .to_string()
    }

    /// Reset the filter (e.g. after teleport/replay restart).
    pub fn reset(&mut self) {
        self.ekf.reset();
    }

    // -- Canonical 15-state ESKF (TD-3) -------------------------------------

    /// Strapdown IMU propagation on the canonical ESKF (body frame, seconds).
    #[allow(clippy::too_many_arguments)]
    pub fn eskf_imu(&mut self, ax: f64, ay: f64, az: f64, gx: f64, gy: f64, gz: f64, dt_s: f64) {
        self.eskf.predict(
            nalgebra::Vector3::new(ax, ay, az),
            nalgebra::Vector3::new(gx, gy, gz),
            dt_s,
        );
    }

    /// ESKF GNSS position update (ENU metres). Returns the NIS gate value.
    pub fn eskf_update_position(&mut self, e: f64, n: f64, u: f64, sigma_m: f64) -> f64 {
        self.eskf
            .update_position(nalgebra::Vector3::new(e, n, u), sigma_m)
    }

    /// ESKF zero-velocity update. Returns the NIS gate value.
    pub fn eskf_zupt(&mut self, sigma_mps: f64) -> f64 {
        self.eskf.update_zupt(sigma_mps)
    }

    /// Canonical ESKF state as JSON (position, velocity, heading, biases, σ).
    pub fn eskf_state(&self) -> String {
        let f = &self.eskf;
        serde_json::json!({
            "p": [f.position.x, f.position.y, f.position.z],
            "v": [f.velocity.x, f.velocity.y, f.velocity.z],
            "heading_rad": f.heading_rad(),
            "accel_bias": [f.accel_bias.x, f.accel_bias.y, f.accel_bias.z],
            "gyro_bias": [f.gyro_bias.x, f.gyro_bias.y, f.gyro_bias.z],
            "horizontal_uncertainty_m": f.horizontal_uncertainty_m(),
        })
        .to_string()
    }

    // -- Map & routing -----------------------------------------------------

    /// Load a road graph (JSON per contracts RoadGraph schema).
    pub fn load_graph(&mut self, graph_json: &str) -> Result<(), JsError> {
        let graph: RoadGraph =
            serde_json::from_str(graph_json).map_err(|e| JsError::new(&e.to_string()))?;
        self.graph = Some(RoadGraphIndex::from_graph(&graph));
        self.graph_data = Some(graph);
        Ok(())
    }

    /// Number of nodes in the loaded graph (0 if none).
    pub fn graph_nodes(&self) -> usize {
        self.graph_data.as_ref().map_or(0, |g| g.nodes.len())
    }

    /// Route between two geographic coordinates (nearest-node snap).
    ///
    /// `envelope_json` optionally carries a VehicleEnvelope; hard constraints
    /// are enforced — an illegal route is never returned. The response is a
    /// GeoRoute JSON with polyline, length, and drive time.
    pub fn route_geo(
        &self,
        from_lat: f64,
        from_lon: f64,
        to_lat: f64,
        to_lon: f64,
        envelope_json: Option<String>,
    ) -> Result<String, JsError> {
        let (index, graph) = match (&self.graph, &self.graph_data) {
            (Some(i), Some(g)) => (i, g),
            _ => return Err(JsError::new("no graph loaded")),
        };
        let envelope: Option<VehicleEnvelope> = match envelope_json {
            Some(s) if !s.is_empty() => {
                Some(serde_json::from_str(&s).map_err(|e| JsError::new(&e.to_string()))?)
            }
            _ => None,
        };
        let route = gane_osm_import::route::route_geo(
            graph,
            index,
            (from_lat, from_lon),
            (to_lat, to_lon),
            envelope,
        )
        .ok_or_else(|| JsError::new("no legal route"))?;
        serde_json::to_string(&route).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Compute a route. Request/response are JSON strings.
    ///
    /// With an `envelope` in the request, hard vehicle constraints are
    /// enforced: an illegal route is never returned.
    pub fn route(&self, request_json: &str) -> Result<String, JsError> {
        let req: RouteRequest =
            serde_json::from_str(request_json).map_err(|e| JsError::new(&e.to_string()))?;
        let (index, graph) = match (&self.graph, &self.graph_data) {
            (Some(i), Some(g)) => (i, g),
            _ => return Err(JsError::new("no graph loaded")),
        };

        let find = |raw: &str| {
            graph
                .nodes
                .iter()
                .find(|n| n.id.to_string() == raw)
                .map(|n| n.id)
        };
        let from = find(&req.from_node).ok_or_else(|| JsError::new("unknown from_node"))?;
        let to = find(&req.to_node).ok_or_else(|| JsError::new("unknown to_node"))?;

        let constrained = req.envelope.is_some();
        let cost_fn: CostFn = match req.envelope {
            Some(env) => by_time_for_vehicle(env),
            None => Box::new(cost::by_time),
        };

        let path = shortest_path(index, from, to, &cost_fn)
            .filter(|p| p.total_cost.is_finite())
            .ok_or_else(|| JsError::new("no legal route"))?;

        let resp = RouteResponse {
            nodes: path.nodes.iter().map(|n| n.to_string()).collect(),
            segments: path.segments.iter().map(|s| s.to_string()).collect(),
            total_cost_s: path.total_cost,
            constrained,
        };
        serde_json::to_string(&resp).map_err(|e| JsError::new(&e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_constructs_and_reports_version() {
        let engine = GaneEngine::new();
        assert!(!engine.version().is_empty());
        assert_eq!(engine.graph_nodes(), 0);
    }

    #[test]
    fn fusion_roundtrip() {
        let mut engine = GaneEngine::new();
        engine.update_position(100.0, 200.0, 10.0, 3.0);
        engine.predict(1.0);
        let state: serde_json::Value = serde_json::from_str(&engine.position()).unwrap();
        assert!(state["east_m"].as_f64().unwrap() > 0.0);
        assert!(state["horizontal_uncertainty_m"].as_f64().unwrap() > 0.0);
    }
}
