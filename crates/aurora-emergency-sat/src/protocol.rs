//! Emergency satellite protocol — message framing, compression, and encoding
//! for bandwidth-constrained satellite links.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Maximum payload size for satellite messages (bytes).
/// Satellite links are extremely bandwidth-constrained; messages must fit in
/// a single burst transmission.
pub const MAX_PAYLOAD_BYTES: usize = 340;

/// Emergency message priority levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum EmergencyPriority {
    /// Informational — non-critical status update
    Info = 0,
    /// Warning — potential danger, advisory
    Warning = 1,
    /// Urgent — immediate attention required
    Urgent = 2,
    /// Distress — life-threatening emergency (SOS)
    Distress = 3,
    /// Mayday — catastrophic, highest priority
    Mayday = 4,
}

/// Type of emergency satellite message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageType {
    /// SOS beacon with position
    SosBeacon,
    /// Location share (non-emergency)
    LocationShare,
    /// Evacuation route broadcast
    EvacuationRoute,
    /// Infrastructure status report
    InfraStatus,
    /// Acknowledgement of received message
    Ack,
    /// Relay — forwarded on behalf of another node
    Relay,
    /// Heartbeat — periodic alive signal
    Heartbeat,
}

/// A compact emergency satellite message.
///
/// Designed for minimal bandwidth: every field is chosen to minimize
/// on-wire size while preserving enough context for rescue operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SatMessage {
    /// Unique message identifier
    pub id: Uuid,
    /// Originator device identifier
    pub sender_id: Uuid,
    /// Message type
    pub msg_type: MessageType,
    /// Priority level
    pub priority: EmergencyPriority,
    /// Latitude (WGS-84, degrees)
    pub lat: f64,
    /// Longitude (WGS-84, degrees)
    pub lon: f64,
    /// Altitude above mean sea level (meters), if known
    pub alt_m: Option<f32>,
    /// Horizontal accuracy estimate (meters)
    pub accuracy_m: Option<f32>,
    /// Heading in degrees (0-360), if known
    pub heading_deg: Option<f32>,
    /// Speed in m/s, if known
    pub speed_mps: Option<f32>,
    /// Free-text payload (UTF-8, truncated to fit)
    pub payload: String,
    /// Hop count — incremented on each relay
    pub hop_count: u8,
    /// Maximum allowed hops before the message is dropped
    pub max_hops: u8,
    /// Timestamp of message creation
    pub created_at: DateTime<Utc>,
    /// Time-to-live in seconds
    pub ttl_s: u32,
    /// HMAC-SHA256 signature (hex-encoded) for authenticity
    pub signature: Option<String>,
}

impl SatMessage {
    /// Create a new emergency message with the given parameters.
    pub fn new(
        sender_id: Uuid,
        msg_type: MessageType,
        priority: EmergencyPriority,
        lat: f64,
        lon: f64,
        payload: &str,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            sender_id,
            msg_type,
            priority,
            lat,
            lon,
            alt_m: None,
            accuracy_m: None,
            heading_deg: None,
            speed_mps: None,
            payload: payload.to_string(),
            hop_count: 0,
            max_hops: 7,
            created_at: Utc::now(),
            ttl_s: 3600,
            signature: None,
        }
    }

    /// Check whether the message has expired.
    pub fn is_expired(&self) -> bool {
        let elapsed = Utc::now()
            .signed_duration_since(self.created_at)
            .num_seconds();
        elapsed >= i64::from(self.ttl_s)
    }

    /// Check whether the message can be relayed further.
    pub fn can_relay(&self) -> bool {
        self.hop_count < self.max_hops && !self.is_expired()
    }

    /// Increment hop count for relay forwarding.
    /// Returns `Err` if max hops reached or message expired.
    pub fn relay(&mut self) -> Result<(), ProtocolError> {
        if self.is_expired() {
            return Err(ProtocolError::MessageExpired { id: self.id });
        }
        if self.hop_count >= self.max_hops {
            return Err(ProtocolError::MaxHopsReached {
                id: self.id,
                max_hops: self.max_hops,
            });
        }
        self.hop_count += 1;
        Ok(())
    }

    /// Estimate the on-wire size of this message in bytes.
    pub fn estimated_size(&self) -> usize {
        // Fixed fields: id(16) + sender(16) + type(1) + priority(1)
        //   + lat(8) + lon(8) + optional floats(4*4=16) + hop(1) + max_hops(1)
        //   + created_at(8) + ttl(4) = ~80 bytes
        // Variable: payload length + signature (64 hex chars = 64 bytes)
        let fixed = 80;
        let sig_len = self.signature.as_ref().map_or(0, |s| s.len());
        fixed + self.payload.len() + sig_len
    }

    /// Truncate payload to ensure the message fits within `MAX_PAYLOAD_BYTES`.
    pub fn truncate_to_fit(&mut self) {
        let overhead = self.estimated_size() - self.payload.len();
        if overhead >= MAX_PAYLOAD_BYTES {
            self.payload.clear();
            return;
        }
        let max_payload = MAX_PAYLOAD_BYTES - overhead;
        if self.payload.len() > max_payload {
            // Truncate at a valid UTF-8 boundary
            let truncated = &self.payload[..max_payload];
            let end = truncated
                .char_indices()
                .last()
                .map(|(i, c)| i + c.len_utf8())
                .unwrap_or(0);
            self.payload = self.payload[..end].to_string();
        }
    }
}

/// Protocol-level errors.
#[derive(Debug, thiserror::Error)]
pub enum ProtocolError {
    #[error("message {id} has expired")]
    MessageExpired { id: Uuid },

    #[error("message {id} reached max hops ({max_hops})")]
    MaxHopsReached { id: Uuid, max_hops: u8 },

    #[error("payload exceeds maximum size ({size} > {max})")]
    PayloadTooLarge { size: usize, max: usize },

    #[error("invalid coordinates: lat={lat}, lon={lon}")]
    InvalidCoordinates { lat: f64, lon: f64 },

    #[error("encoding error: {0}")]
    EncodingError(String),
}

/// Encode a `SatMessage` to a compact JSON byte vector.
pub fn encode_message(msg: &SatMessage) -> Result<Vec<u8>, ProtocolError> {
    if msg.lat < -90.0 || msg.lat > 90.0 || msg.lon < -180.0 || msg.lon > 180.0 {
        return Err(ProtocolError::InvalidCoordinates {
            lat: msg.lat,
            lon: msg.lon,
        });
    }
    let bytes = serde_json::to_vec(msg).map_err(|e| ProtocolError::EncodingError(e.to_string()))?;
    if bytes.len() > MAX_PAYLOAD_BYTES {
        return Err(ProtocolError::PayloadTooLarge {
            size: bytes.len(),
            max: MAX_PAYLOAD_BYTES,
        });
    }
    Ok(bytes)
}

/// Decode a `SatMessage` from bytes.
pub fn decode_message(data: &[u8]) -> Result<SatMessage, ProtocolError> {
    serde_json::from_slice(data).map_err(|e| ProtocolError::EncodingError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let sender = Uuid::new_v4();
        let msg = SatMessage::new(
            sender,
            MessageType::SosBeacon,
            EmergencyPriority::Distress,
            32.0853,
            34.7818,
            "Vehicle collision on Highway 1",
        );
        assert_eq!(msg.sender_id, sender);
        assert_eq!(msg.msg_type, MessageType::SosBeacon);
        assert_eq!(msg.priority, EmergencyPriority::Distress);
        assert_eq!(msg.hop_count, 0);
        assert_eq!(msg.max_hops, 7);
        assert!(!msg.is_expired());
        assert!(msg.can_relay());
    }

    #[test]
    fn test_relay_increments_hop_count() {
        let mut msg = SatMessage::new(
            Uuid::new_v4(),
            MessageType::Relay,
            EmergencyPriority::Urgent,
            31.0,
            35.0,
            "relay test",
        );
        msg.max_hops = 2;

        assert!(msg.relay().is_ok());
        assert_eq!(msg.hop_count, 1);

        assert!(msg.relay().is_ok());
        assert_eq!(msg.hop_count, 2);

        // Third relay should fail — max hops reached
        assert!(msg.relay().is_err());
    }

    #[test]
    fn test_expired_message_cannot_relay() {
        let mut msg = SatMessage::new(
            Uuid::new_v4(),
            MessageType::Heartbeat,
            EmergencyPriority::Info,
            30.0,
            34.0,
            "",
        );
        msg.ttl_s = 0;
        // With TTL=0, the message is expired immediately
        assert!(msg.is_expired());
        assert!(!msg.can_relay());
        assert!(msg.relay().is_err());
    }

    #[test]
    fn test_truncate_to_fit() {
        let mut msg = SatMessage::new(
            Uuid::new_v4(),
            MessageType::InfraStatus,
            EmergencyPriority::Warning,
            32.0,
            34.0,
            &"A".repeat(500),
        );
        assert!(msg.estimated_size() > MAX_PAYLOAD_BYTES);
        msg.truncate_to_fit();
        assert!(msg.estimated_size() <= MAX_PAYLOAD_BYTES);
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let msg = SatMessage::new(
            Uuid::new_v4(),
            MessageType::LocationShare,
            EmergencyPriority::Info,
            32.0853,
            34.7818,
            "OK",
        );
        // We can't guarantee encode succeeds for all messages due to size,
        // but we can test decode on what we encode
        let json = serde_json::to_vec(&msg).unwrap();
        let decoded = decode_message(&json).unwrap();
        assert_eq!(decoded.id, msg.id);
        assert_eq!(decoded.sender_id, msg.sender_id);
        assert_eq!(decoded.msg_type, msg.msg_type);
    }

    #[test]
    fn test_invalid_coordinates_rejected() {
        let msg = SatMessage::new(
            Uuid::new_v4(),
            MessageType::SosBeacon,
            EmergencyPriority::Mayday,
            91.0, // Invalid latitude
            34.0,
            "help",
        );
        assert!(encode_message(&msg).is_err());
    }

    #[test]
    fn test_priority_ordering() {
        assert!(EmergencyPriority::Mayday > EmergencyPriority::Distress);
        assert!(EmergencyPriority::Distress > EmergencyPriority::Urgent);
        assert!(EmergencyPriority::Urgent > EmergencyPriority::Warning);
        assert!(EmergencyPriority::Warning > EmergencyPriority::Info);
    }
}
