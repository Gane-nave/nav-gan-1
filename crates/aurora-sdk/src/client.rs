//! SDK client builder — fluent API for constructing and configuring
//! an AURORA NAV SDK client instance.

use aurora_core::types::EntityId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use tracing::{debug, info, warn};

use crate::config::SdkConfig;
use crate::extension::{Extension, ExtensionRegistry};
use crate::plugin::{Plugin, PluginManager};

// ---------------------------------------------------------------------------
// Client builder
// ---------------------------------------------------------------------------

/// Fluent builder for constructing an [`AuroraClient`].
pub struct ClientBuilder {
    api_key: Option<String>,
    endpoint: Option<String>,
    config: SdkConfig,
    plugins: Vec<Box<dyn Plugin>>,
    extensions: Vec<Box<dyn Extension>>,
    metadata: HashMap<String, String>,
}

impl ClientBuilder {
    /// Create a new builder with default configuration.
    pub fn new() -> Self {
        Self {
            api_key: None,
            endpoint: None,
            config: SdkConfig::default(),
            plugins: Vec::new(),
            extensions: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Set the API key for authentication.
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Set the API endpoint URL.
    pub fn endpoint(mut self, url: impl Into<String>) -> Self {
        self.endpoint = Some(url.into());
        self
    }

    /// Apply a custom configuration.
    pub fn config(mut self, config: SdkConfig) -> Self {
        self.config = config;
        self
    }

    /// Register a plugin.
    pub fn plugin(mut self, plugin: Box<dyn Plugin>) -> Self {
        self.plugins.push(plugin);
        self
    }

    /// Register an extension.
    pub fn extension(mut self, ext: Box<dyn Extension>) -> Self {
        self.extensions.push(ext);
        self
    }

    /// Add custom metadata.
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Build the client. Returns an error if required fields are missing.
    pub fn build(self) -> Result<AuroraClient, ClientError> {
        let api_key = self.api_key.ok_or(ClientError::MissingApiKey)?;
        let endpoint = self
            .endpoint
            .unwrap_or_else(|| self.config.default_endpoint.clone());

        let mut plugin_manager = PluginManager::new(self.config.max_plugins);
        for p in self.plugins {
            plugin_manager.register(p)?;
        }

        let mut extension_registry = ExtensionRegistry::new();
        for ext in self.extensions {
            extension_registry.register(ext);
        }

        info!(
            endpoint = %endpoint,
            plugins = plugin_manager.count(),
            extensions = extension_registry.count(),
            "AURORA SDK client built"
        );

        Ok(AuroraClient {
            id: EntityId::new(),
            api_key,
            endpoint,
            config: self.config,
            plugin_manager,
            extension_registry,
            metadata: self.metadata,
            state: ClientState::Disconnected,
            created_at: Utc::now(),
            request_count: 0,
        })
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Client
// ---------------------------------------------------------------------------

/// The main SDK client for interacting with AURORA NAV services.
pub struct AuroraClient {
    id: EntityId,
    api_key: String,
    endpoint: String,
    config: SdkConfig,
    plugin_manager: PluginManager,
    extension_registry: ExtensionRegistry,
    metadata: HashMap<String, String>,
    state: ClientState,
    created_at: DateTime<Utc>,
    request_count: u64,
}

impl fmt::Debug for AuroraClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AuroraClient")
            .field("id", &self.id)
            .field("endpoint", &self.endpoint)
            .field("state", &self.state)
            .field("request_count", &self.request_count)
            .field("created_at", &self.created_at)
            .finish()
    }
}

/// Client connection state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClientState {
    /// Not connected to the server.
    Disconnected,
    /// Connection in progress.
    Connecting,
    /// Connected and ready.
    Connected,
    /// Connection lost, attempting reconnect.
    Reconnecting,
    /// Permanently failed.
    Failed,
}

/// Client health snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientHealth {
    pub client_id: EntityId,
    pub state: ClientState,
    pub endpoint: String,
    pub request_count: u64,
    pub plugin_count: usize,
    pub extension_count: usize,
    pub uptime_seconds: f64,
    pub checked_at: DateTime<Utc>,
}

/// Client errors.
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("API key is required")]
    MissingApiKey,
    #[error("client is not connected")]
    NotConnected,
    #[error("request failed: {0}")]
    RequestFailed(String),
    #[error("plugin error: {0}")]
    PluginError(String),
    #[error("too many plugins (max {0})")]
    TooManyPlugins(usize),
    #[error("rate limited — retry after {retry_after_ms}ms")]
    RateLimited { retry_after_ms: u64 },
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),
}

impl AuroraClient {
    /// Get the client ID.
    pub fn id(&self) -> EntityId {
        self.id
    }

    /// Get the current connection state.
    pub fn state(&self) -> ClientState {
        self.state
    }

    /// Get the API endpoint.
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Get custom metadata value.
    pub fn metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(String::as_str)
    }

    /// Connect to the AURORA NAV service.
    pub fn connect(&mut self) -> Result<(), ClientError> {
        match self.state {
            ClientState::Connected => {
                debug!("already connected");
                return Ok(());
            }
            ClientState::Failed => {
                warn!("cannot connect from failed state");
                return Err(ClientError::NotConnected);
            }
            _ => {}
        }

        self.state = ClientState::Connecting;

        // Validate API key format (must be non-empty and reasonable length).
        if self.api_key.is_empty() || self.api_key.len() > 256 {
            self.state = ClientState::Failed;
            return Err(ClientError::InvalidConfig(
                "API key must be 1-256 characters".into(),
            ));
        }

        // Initialise all plugins.
        self.plugin_manager
            .initialise_all()
            .map_err(|e| ClientError::PluginError(e.to_string()))?;

        self.state = ClientState::Connected;
        info!(endpoint = %self.endpoint, "SDK client connected");
        Ok(())
    }

    /// Disconnect from the service.
    pub fn disconnect(&mut self) {
        if self.state == ClientState::Connected {
            self.plugin_manager.shutdown_all();
            self.state = ClientState::Disconnected;
            info!("SDK client disconnected");
        }
    }

    /// Execute a request through the SDK pipeline.
    pub fn request(&mut self, req: SdkRequest) -> Result<SdkResponse, ClientError> {
        if self.state != ClientState::Connected {
            return Err(ClientError::NotConnected);
        }

        // Check rate limit.
        if self.request_count >= self.config.max_requests_per_session {
            return Err(ClientError::RateLimited {
                retry_after_ms: 1000,
            });
        }

        // Run pre-request hooks from plugins.
        let processed_req = self
            .plugin_manager
            .pre_request(req)
            .map_err(|e| ClientError::PluginError(e.to_string()))?;

        // Run extensions.
        let extended_req = self
            .extension_registry
            .apply_request_extensions(processed_req);

        self.request_count += 1;
        debug!(
            request_type = %extended_req.request_type,
            count = self.request_count,
            "SDK request processed"
        );

        // Build response.
        let response = SdkResponse {
            request_id: extended_req.id,
            status: ResponseStatus::Success,
            data: serde_json::json!({
                "request_type": extended_req.request_type,
                "processed": true,
                "request_number": self.request_count,
            }),
            latency_ms: 0.0,
            responded_at: Utc::now(),
        };

        // Run post-response hooks.
        let final_response = self
            .plugin_manager
            .post_response(response)
            .map_err(|e| ClientError::PluginError(e.to_string()))?;

        Ok(final_response)
    }

    /// Get client health information.
    pub fn health(&self) -> ClientHealth {
        let uptime = (Utc::now() - self.created_at).num_milliseconds() as f64 / 1000.0;
        ClientHealth {
            client_id: self.id,
            state: self.state,
            endpoint: self.endpoint.clone(),
            request_count: self.request_count,
            plugin_count: self.plugin_manager.count(),
            extension_count: self.extension_registry.count(),
            uptime_seconds: uptime,
            checked_at: Utc::now(),
        }
    }

    /// Get the total number of requests made.
    pub fn request_count(&self) -> u64 {
        self.request_count
    }

    /// Get the plugin manager (read-only).
    pub fn plugins(&self) -> &PluginManager {
        &self.plugin_manager
    }

    /// Get the extension registry (read-only).
    pub fn extensions(&self) -> &ExtensionRegistry {
        &self.extension_registry
    }
}

// ---------------------------------------------------------------------------
// Request / Response
// ---------------------------------------------------------------------------

/// An SDK request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdkRequest {
    pub id: EntityId,
    pub request_type: String,
    pub params: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

impl SdkRequest {
    pub fn new(request_type: impl Into<String>, params: serde_json::Value) -> Self {
        Self {
            id: EntityId::new(),
            request_type: request_type.into(),
            params,
            created_at: Utc::now(),
        }
    }
}

/// Response status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseStatus {
    Success,
    PartialSuccess,
    Error,
    RateLimited,
}

/// An SDK response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdkResponse {
    pub request_id: EntityId,
    pub status: ResponseStatus,
    pub data: serde_json::Value,
    pub latency_ms: f64,
    pub responded_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_requires_api_key() {
        let result = ClientBuilder::new().build();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ClientError::MissingApiKey));
    }

    #[test]
    fn builder_creates_client_with_defaults() {
        let client = ClientBuilder::new()
            .api_key("test-key-123")
            .build()
            .unwrap();
        assert_eq!(client.state(), ClientState::Disconnected);
        assert_eq!(client.request_count(), 0);
        assert!(client.endpoint().contains("aurora-nav"));
    }

    #[test]
    fn builder_custom_endpoint() {
        let client = ClientBuilder::new()
            .api_key("key")
            .endpoint("https://custom.api.example.com")
            .build()
            .unwrap();
        assert_eq!(client.endpoint(), "https://custom.api.example.com");
    }

    #[test]
    fn builder_with_metadata() {
        let client = ClientBuilder::new()
            .api_key("key")
            .metadata("app_name", "TestApp")
            .metadata("version", "1.0")
            .build()
            .unwrap();
        assert_eq!(client.metadata("app_name"), Some("TestApp"));
        assert_eq!(client.metadata("version"), Some("1.0"));
        assert_eq!(client.metadata("nonexistent"), None);
    }

    #[test]
    fn connect_and_disconnect() {
        let mut client = ClientBuilder::new().api_key("valid-key").build().unwrap();
        assert_eq!(client.state(), ClientState::Disconnected);

        client.connect().unwrap();
        assert_eq!(client.state(), ClientState::Connected);

        // Idempotent connect.
        client.connect().unwrap();
        assert_eq!(client.state(), ClientState::Connected);

        client.disconnect();
        assert_eq!(client.state(), ClientState::Disconnected);
    }

    #[test]
    fn request_requires_connection() {
        let mut client = ClientBuilder::new().api_key("key").build().unwrap();
        let req = SdkRequest::new("position", serde_json::json!({}));
        let result = client.request(req);
        assert!(matches!(result.unwrap_err(), ClientError::NotConnected));
    }

    #[test]
    fn request_succeeds_when_connected() {
        let mut client = ClientBuilder::new().api_key("key").build().unwrap();
        client.connect().unwrap();

        let req = SdkRequest::new("position", serde_json::json!({"lat": 32.0}));
        let resp = client.request(req).unwrap();
        assert_eq!(resp.status, ResponseStatus::Success);
        assert_eq!(client.request_count(), 1);
    }

    #[test]
    fn rate_limiting_enforced() {
        let config = SdkConfig {
            max_requests_per_session: 2,
            ..SdkConfig::default()
        };
        let mut client = ClientBuilder::new()
            .api_key("key")
            .config(config)
            .build()
            .unwrap();
        client.connect().unwrap();

        // First two succeed.
        client
            .request(SdkRequest::new("a", serde_json::json!({})))
            .unwrap();
        client
            .request(SdkRequest::new("b", serde_json::json!({})))
            .unwrap();

        // Third is rate limited.
        let result = client.request(SdkRequest::new("c", serde_json::json!({})));
        assert!(matches!(
            result.unwrap_err(),
            ClientError::RateLimited { .. }
        ));
    }

    #[test]
    fn health_reports_correct_state() {
        let mut client = ClientBuilder::new().api_key("key").build().unwrap();
        let health = client.health();
        assert_eq!(health.state, ClientState::Disconnected);
        assert_eq!(health.request_count, 0);
        assert_eq!(health.plugin_count, 0);

        client.connect().unwrap();
        let health = client.health();
        assert_eq!(health.state, ClientState::Connected);
    }

    #[test]
    fn failed_client_cannot_reconnect() {
        let mut client = ClientBuilder::new()
            .api_key("") // empty → will fail
            .build()
            .unwrap();

        // First connect attempt should fail due to empty API key validation.
        let result = client.connect();
        assert!(result.is_err());
        assert_eq!(client.state(), ClientState::Failed);

        // Second attempt should also fail — cannot recover from Failed.
        let result = client.connect();
        assert!(matches!(result.unwrap_err(), ClientError::NotConnected));
    }
}
