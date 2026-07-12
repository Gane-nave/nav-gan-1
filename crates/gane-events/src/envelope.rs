//! Event envelope — the canonical wrapper for every event in the system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::event_type::EventType;

/// Every event in G.A.N.E NAV is wrapped in this envelope.
/// Provides traceability, idempotency, and integrity metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    /// Globally unique event identifier.
    pub event_id: Uuid,
    /// Wall-clock timestamp when the event was created.
    pub timestamp: DateTime<Utc>,
    /// Origin module / service that produced the event.
    pub origin: String,
    /// Type of the entity this event relates to.
    pub entity_type: String,
    /// Identifier of the entity this event relates to.
    pub entity_id: Uuid,
    /// Discriminated event type.
    pub event_type: EventType,
    /// Serialised payload (JSON).
    pub payload: serde_json::Value,
    /// Schema version for forward/backward compatibility.
    pub schema_version: u32,
    /// Optional cryptographic signature over the envelope.
    pub signature: Option<String>,
    /// Idempotency key to prevent duplicate processing.
    pub idempotency_key: Uuid,
}

impl EventEnvelope {
    /// Create a new envelope with auto-generated IDs and current timestamp.
    pub fn new(
        origin: impl Into<String>,
        entity_type: impl Into<String>,
        entity_id: Uuid,
        event_type: EventType,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            origin: origin.into(),
            entity_type: entity_type.into(),
            entity_id,
            event_type,
            payload,
            schema_version: 1,
            signature: None,
            idempotency_key: Uuid::new_v4(),
        }
    }

    /// Attach a cryptographic signature.
    pub fn with_signature(mut self, sig: String) -> Self {
        self.signature = Some(sig);
        self
    }

    /// Override schema version.
    pub fn with_schema_version(mut self, v: u32) -> Self {
        self.schema_version = v;
        self
    }
}
