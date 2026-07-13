//! OpenAPI 3.0 specification generator for G.A.N.E NAV API.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// OpenAPI 3.0 specification document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiSpec {
    pub openapi: String,
    pub info: Info,
    pub servers: Vec<Server>,
    pub paths: BTreeMap<String, PathItem>,
    pub components: Option<Components>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub title: String,
    pub description: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "operationId")]
    pub operation_id: String,
    pub tags: Vec<String>,
    pub responses: BTreeMap<String, Response>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<BTreeMap<String, MediaType>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaType {
    pub schema: SchemaRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchemaRef {
    Ref {
        #[serde(rename = "$ref")]
        reference: String,
    },
    Inline(Schema),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "type")]
    pub schema_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, SchemaProperty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaProperty {
    #[serde(rename = "type")]
    pub prop_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Components {
    pub schemas: BTreeMap<String, Schema>,
}

/// Build the complete OpenAPI specification for G.A.N.E NAV.
pub fn build_spec() -> OpenApiSpec {
    let mut paths = BTreeMap::new();
    let mut schemas = BTreeMap::new();

    // /health
    paths.insert(
        "/health".to_string(),
        PathItem {
            get: Some(Operation {
                summary: "Health check".to_string(),
                description: Some(
                    "Returns the current health status of all subsystems.".to_string(),
                ),
                operation_id: "getHealth".to_string(),
                tags: vec!["System".to_string()],
                responses: json_response(
                    "200",
                    "Health status",
                    "#/components/schemas/HealthResponse",
                ),
            }),
            post: None,
            put: None,
            delete: None,
        },
    );

    // /position
    paths.insert(
        "/position".to_string(),
        PathItem {
            get: Some(Operation {
                summary: "Current position".to_string(),
                description: Some(
                    "Returns the current fused position with uncertainty estimates.".to_string(),
                ),
                operation_id: "getPosition".to_string(),
                tags: vec!["Navigation".to_string()],
                responses: json_response_with_error(
                    "200",
                    "Current fused position",
                    "#/components/schemas/PositionResponse",
                    "503",
                    "No GNSS fix available",
                ),
            }),
            post: None,
            put: None,
            delete: None,
        },
    );

    // /integrity
    paths.insert(
        "/integrity".to_string(),
        PathItem {
            get: Some(Operation {
                summary: "Integrity status".to_string(),
                description: Some(
                    "Returns current integrity monitoring level and protection limits.".to_string(),
                ),
                operation_id: "getIntegrity".to_string(),
                tags: vec!["Navigation".to_string()],
                responses: json_response(
                    "200",
                    "Integrity status",
                    "#/components/schemas/IntegrityResponse",
                ),
            }),
            post: None,
            put: None,
            delete: None,
        },
    );

    // /status
    paths.insert(
        "/status".to_string(),
        PathItem {
            get: Some(Operation {
                summary: "System status".to_string(),
                description: Some(
                    "Returns overall system status including all subsystem states.".to_string(),
                ),
                operation_id: "getStatus".to_string(),
                tags: vec!["System".to_string()],
                responses: json_response(
                    "200",
                    "System status",
                    "#/components/schemas/SystemStatusResponse",
                ),
            }),
            post: None,
            put: None,
            delete: None,
        },
    );

    // /telemetry
    paths.insert(
        "/telemetry".to_string(),
        PathItem {
            get: Some(Operation {
                summary: "Telemetry data".to_string(),
                description: Some(
                    "Returns buffered telemetry samples for analysis and replay.".to_string(),
                ),
                operation_id: "getTelemetry".to_string(),
                tags: vec!["Telemetry".to_string()],
                responses: json_response(
                    "200",
                    "Telemetry samples",
                    "#/components/schemas/TelemetryResponse",
                ),
            }),
            post: None,
            put: None,
            delete: None,
        },
    );

    // /constellation
    paths.insert(
        "/constellation".to_string(),
        PathItem {
            get: Some(Operation {
                summary: "Constellation status".to_string(),
                description: Some(
                    "Returns tracked GNSS constellations and satellite count.".to_string(),
                ),
                operation_id: "getConstellations".to_string(),
                tags: vec!["Navigation".to_string()],
                responses: json_response(
                    "200",
                    "Constellation data",
                    "#/components/schemas/ConstellationResponse",
                ),
            }),
            post: None,
            put: None,
            delete: None,
        },
    );

    // /metrics
    paths.insert(
        "/metrics".to_string(),
        PathItem {
            get: Some(Operation {
                summary: "Prometheus metrics".to_string(),
                description: Some(
                    "Returns all metrics in Prometheus text exposition format.".to_string(),
                ),
                operation_id: "getMetrics".to_string(),
                tags: vec!["System".to_string()],
                responses: {
                    let mut r = BTreeMap::new();
                    r.insert(
                        "200".to_string(),
                        Response {
                            description: "Prometheus metrics".to_string(),
                            content: Some({
                                let mut m = BTreeMap::new();
                                m.insert(
                                    "text/plain".to_string(),
                                    MediaType {
                                        schema: SchemaRef::Inline(Schema {
                                            schema_type: "string".to_string(),
                                            properties: None,
                                            required: None,
                                            description: Some(
                                                "Prometheus text exposition format".to_string(),
                                            ),
                                        }),
                                    },
                                );
                                m
                            }),
                        },
                    );
                    r
                },
            }),
            post: None,
            put: None,
            delete: None,
        },
    );

    // /readiness
    paths.insert(
        "/readiness".to_string(),
        PathItem {
            get: Some(Operation {
                summary: "Readiness probe".to_string(),
                description: Some(
                    "Kubernetes readiness probe — returns 200 when ready to serve traffic."
                        .to_string(),
                ),
                operation_id: "getReadiness".to_string(),
                tags: vec!["System".to_string()],
                responses: json_response("200", "Ready", "#/components/schemas/ProbeResponse"),
            }),
            post: None,
            put: None,
            delete: None,
        },
    );

    // /liveness
    paths.insert(
        "/liveness".to_string(),
        PathItem {
            get: Some(Operation {
                summary: "Liveness probe".to_string(),
                description: Some(
                    "Kubernetes liveness probe — returns 200 when the process is alive."
                        .to_string(),
                ),
                operation_id: "getLiveness".to_string(),
                tags: vec!["System".to_string()],
                responses: json_response("200", "Alive", "#/components/schemas/ProbeResponse"),
            }),
            post: None,
            put: None,
            delete: None,
        },
    );

    // Schemas
    schemas.insert(
        "HealthResponse".to_string(),
        Schema {
            schema_type: "object".to_string(),
            description: Some("Health status of the navigation system.".to_string()),
            properties: Some(build_properties(&[
                (
                    "status",
                    "string",
                    "Overall health status (operational, degraded, critical)",
                ),
                ("version", "string", "Application version"),
                ("uptime_seconds", "integer", "Seconds since server start"),
                ("subsystems", "object", "Per-subsystem health details"),
            ])),
            required: Some(vec!["status".to_string(), "version".to_string()]),
        },
    );

    schemas.insert(
        "PositionResponse".to_string(),
        Schema {
            schema_type: "object".to_string(),
            description: Some("Fused navigation position.".to_string()),
            properties: Some(build_properties(&[
                ("latitude", "number", "Latitude in degrees (WGS84)"),
                ("longitude", "number", "Longitude in degrees (WGS84)"),
                (
                    "altitude",
                    "number",
                    "Altitude in metres above WGS84 ellipsoid",
                ),
                (
                    "horizontal_uncertainty_m",
                    "number",
                    "Horizontal uncertainty (1-sigma) in metres",
                ),
                (
                    "vertical_uncertainty_m",
                    "number",
                    "Vertical uncertainty (1-sigma) in metres",
                ),
                ("timestamp", "string", "ISO 8601 timestamp of the fix"),
                (
                    "source",
                    "string",
                    "Position source (gnss, fused, dead_reckoning)",
                ),
            ])),
            required: Some(vec![
                "latitude".to_string(),
                "longitude".to_string(),
                "altitude".to_string(),
                "timestamp".to_string(),
            ]),
        },
    );

    schemas.insert(
        "IntegrityResponse".to_string(),
        Schema {
            schema_type: "object".to_string(),
            description: Some("Navigation integrity monitoring status.".to_string()),
            properties: Some(build_properties(&[
                ("level", "string", "Current integrity level"),
                (
                    "horizontal_protection_m",
                    "number",
                    "Horizontal protection level in metres",
                ),
                (
                    "vertical_protection_m",
                    "number",
                    "Vertical protection level in metres",
                ),
                ("alert_limit_m", "number", "Alert limit in metres"),
                ("time_to_alert_s", "number", "Time to alert in seconds"),
            ])),
            required: Some(vec!["level".to_string()]),
        },
    );

    schemas.insert(
        "SystemStatusResponse".to_string(),
        Schema {
            schema_type: "object".to_string(),
            description: Some("Overall system status.".to_string()),
            properties: Some(build_properties(&[
                ("status", "string", "Overall system status"),
                (
                    "satellites_tracked",
                    "integer",
                    "Number of tracked GNSS satellites",
                ),
                ("integrity_level", "string", "Current integrity level"),
                ("continuity_mode", "string", "Current continuity mode"),
                ("event_count", "integer", "Total events processed"),
                (
                    "telemetry_buffer_size",
                    "integer",
                    "Number of buffered telemetry samples",
                ),
            ])),
            required: Some(vec!["status".to_string()]),
        },
    );

    schemas.insert(
        "TelemetryResponse".to_string(),
        Schema {
            schema_type: "object".to_string(),
            description: Some("Telemetry data from the ring buffer.".to_string()),
            properties: Some(build_properties(&[
                ("buffer_size", "integer", "Number of samples in buffer"),
                (
                    "total_recorded",
                    "integer",
                    "Total samples recorded since start",
                ),
                ("sample_rate_hz", "number", "Current sample rate in Hz"),
            ])),
            required: Some(vec!["buffer_size".to_string()]),
        },
    );

    schemas.insert(
        "ConstellationResponse".to_string(),
        Schema {
            schema_type: "object".to_string(),
            description: Some("GNSS constellation tracking status.".to_string()),
            properties: Some(build_properties(&[
                (
                    "total_tracked",
                    "integer",
                    "Total satellites tracked across all constellations",
                ),
                ("gps", "integer", "GPS satellites tracked"),
                ("galileo", "integer", "Galileo satellites tracked"),
                ("glonass", "integer", "GLONASS satellites tracked"),
                ("beidou", "integer", "BeiDou satellites tracked"),
            ])),
            required: Some(vec!["total_tracked".to_string()]),
        },
    );

    schemas.insert(
        "ProbeResponse".to_string(),
        Schema {
            schema_type: "object".to_string(),
            description: Some("Kubernetes health probe response.".to_string()),
            properties: Some(build_properties(&[
                ("status", "string", "Probe status (ok)"),
                ("timestamp", "string", "ISO 8601 timestamp"),
            ])),
            required: Some(vec!["status".to_string()]),
        },
    );

    OpenApiSpec {
        openapi: "3.0.3".to_string(),
        info: Info {
            title: "G.A.N.E NAV API".to_string(),
            description: "Global Autonomous Navigation Ecosystem — Navigation, positioning, routing, fleet management, and smart city integration API."
                .to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            contact: Some(Contact {
                name: Some("G.A.N.E NAV Team".to_string()),
                url: None,
                email: None,
            }),
            license: Some(License {
                name: "Proprietary".to_string(),
                url: None,
            }),
        },
        servers: vec![
            Server {
                url: "http://localhost:3000".to_string(),
                description: Some("Local development server".to_string()),
            },
        ],
        paths,
        components: Some(Components { schemas }),
    }
}

/// Render the OpenAPI spec as JSON.
pub fn render_json() -> String {
    let spec = build_spec();
    serde_json::to_string_pretty(&spec).unwrap_or_else(|_| "{}".to_string())
}

/// Render a minimal Swagger UI HTML page that loads the spec.
pub fn swagger_ui_html() -> String {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>G.A.N.E NAV — API Documentation</title>
    <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist@5/swagger-ui.css" >
</head>
<body>
<div id="swagger-ui"></div>
<script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-bundle.js"> </script>
<script>
SwaggerUIBundle({
    url: "/openapi.json",
    dom_id: '#swagger-ui',
    deepLinking: true,
    presets: [
        SwaggerUIBundle.presets.apis,
        SwaggerUIBundle.SwaggerUIStandalonePreset
    ],
    layout: "StandaloneLayout"
})
</script>
</body>
</html>"#
        .to_string()
}

fn json_response(status: &str, desc: &str, schema_ref: &str) -> BTreeMap<String, Response> {
    let mut r = BTreeMap::new();
    r.insert(
        status.to_string(),
        Response {
            description: desc.to_string(),
            content: Some({
                let mut m = BTreeMap::new();
                m.insert(
                    "application/json".to_string(),
                    MediaType {
                        schema: SchemaRef::Ref {
                            reference: schema_ref.to_string(),
                        },
                    },
                );
                m
            }),
        },
    );
    r
}

fn json_response_with_error(
    ok_status: &str,
    ok_desc: &str,
    schema_ref: &str,
    err_status: &str,
    err_desc: &str,
) -> BTreeMap<String, Response> {
    let mut r = json_response(ok_status, ok_desc, schema_ref);
    r.insert(
        err_status.to_string(),
        Response {
            description: err_desc.to_string(),
            content: None,
        },
    );
    r
}

fn build_properties(fields: &[(&str, &str, &str)]) -> BTreeMap<String, SchemaProperty> {
    let mut props = BTreeMap::new();
    for (name, typ, desc) in fields {
        props.insert(
            name.to_string(),
            SchemaProperty {
                prop_type: typ.to_string(),
                description: Some(desc.to_string()),
                format: match *typ {
                    "integer" => Some("int64".to_string()),
                    "number" => Some("double".to_string()),
                    _ => None,
                },
                example: None,
            },
        );
    }
    props
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_spec_has_all_endpoints() {
        let spec = build_spec();
        assert_eq!(spec.openapi, "3.0.3");
        assert!(spec.paths.contains_key("/health"));
        assert!(spec.paths.contains_key("/position"));
        assert!(spec.paths.contains_key("/integrity"));
        assert!(spec.paths.contains_key("/status"));
        assert!(spec.paths.contains_key("/telemetry"));
        assert!(spec.paths.contains_key("/constellation"));
        assert!(spec.paths.contains_key("/metrics"));
        assert!(spec.paths.contains_key("/readiness"));
        assert!(spec.paths.contains_key("/liveness"));
        assert_eq!(spec.paths.len(), 9);
    }

    #[test]
    fn spec_has_all_schemas() {
        let spec = build_spec();
        let schemas = spec.components.as_ref().unwrap();
        assert!(schemas.schemas.contains_key("HealthResponse"));
        assert!(schemas.schemas.contains_key("PositionResponse"));
        assert!(schemas.schemas.contains_key("IntegrityResponse"));
        assert!(schemas.schemas.contains_key("SystemStatusResponse"));
        assert!(schemas.schemas.contains_key("TelemetryResponse"));
        assert!(schemas.schemas.contains_key("ConstellationResponse"));
        assert!(schemas.schemas.contains_key("ProbeResponse"));
        assert_eq!(schemas.schemas.len(), 7);
    }

    #[test]
    fn render_json_is_valid() {
        let json = render_json();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["openapi"], "3.0.3");
        assert_eq!(parsed["info"]["title"], "G.A.N.E NAV API");
    }

    #[test]
    fn swagger_ui_html_contains_script() {
        let html = swagger_ui_html();
        assert!(html.contains("swagger-ui"));
        assert!(html.contains("/openapi.json"));
        assert!(html.contains("SwaggerUIBundle"));
    }

    #[test]
    fn position_has_error_response() {
        let spec = build_spec();
        let position = spec.paths.get("/position").unwrap();
        let get = position.get.as_ref().unwrap();
        assert!(get.responses.contains_key("200"));
        assert!(get.responses.contains_key("503"));
    }

    #[test]
    fn health_endpoint_tags() {
        let spec = build_spec();
        let health = spec.paths.get("/health").unwrap();
        let get = health.get.as_ref().unwrap();
        assert_eq!(get.tags, vec!["System"]);
        assert_eq!(get.operation_id, "getHealth");
    }

    #[test]
    fn schema_properties_types() {
        let spec = build_spec();
        let pos = spec
            .components
            .as_ref()
            .unwrap()
            .schemas
            .get("PositionResponse")
            .unwrap();
        let props = pos.properties.as_ref().unwrap();
        assert_eq!(props.get("latitude").unwrap().prop_type, "number");
        assert_eq!(
            props.get("latitude").unwrap().format.as_ref().unwrap(),
            "double"
        );
        assert_eq!(props.get("timestamp").unwrap().prop_type, "string");
    }

    #[test]
    fn metrics_endpoint_returns_text() {
        let spec = build_spec();
        let metrics = spec.paths.get("/metrics").unwrap();
        let get = metrics.get.as_ref().unwrap();
        let response = get.responses.get("200").unwrap();
        let content = response.content.as_ref().unwrap();
        assert!(content.contains_key("text/plain"));
    }

    #[test]
    fn spec_serialization_roundtrip() {
        let spec = build_spec();
        let json = serde_json::to_string(&spec).unwrap();
        let restored: OpenApiSpec = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.paths.len(), spec.paths.len());
    }
}
