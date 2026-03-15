//! Satellite packet protocol — compact binary packets for low-bandwidth
//! satellite communication channels.
//!
//! Packets are designed for extreme bandwidth constraints (< 1 kbps)
//! with built-in error detection and priority handling.

use aurora_core::types::EntityId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

/// Packet type for satellite communication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PacketType {
    /// Position report (lat/lon/alt/accuracy).
    PositionReport,
    /// Emergency distress signal.
    DistressSignal,
    /// Status heartbeat.
    Heartbeat,
    /// Short text message.
    TextMessage,
    /// Telemetry data summary.
    TelemetrySummary,
    /// Acknowledgement.
    Ack,
    /// Navigation waypoint update.
    WaypointUpdate,
    /// Fleet status report.
    FleetStatus,
}

/// Priority level for satellite packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PacketPriority {
    /// Best-effort delivery.
    Low,
    /// Normal priority.
    Normal,
    /// Elevated — skip queue.
    High,
    /// Emergency — immediate transmission.
    Emergency,
}

/// A satellite communication packet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SatellitePacket {
    pub id: EntityId,
    pub packet_type: PacketType,
    pub priority: PacketPriority,
    pub source_id: String,
    pub destination_id: Option<String>,
    pub payload: Vec<u8>,
    pub payload_size_bytes: usize,
    pub sequence_number: u32,
    pub created_at: DateTime<Utc>,
    pub ttl_seconds: u64,
    pub checksum: u32,
    pub status: PacketStatus,
    pub retries: u32,
}

/// Status of a satellite packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PacketStatus {
    /// Queued for transmission.
    Queued,
    /// Currently being transmitted.
    Transmitting,
    /// Successfully delivered.
    Delivered,
    /// Delivery failed.
    Failed,
    /// Acknowledgement received.
    Acknowledged,
    /// Expired (TTL exceeded).
    Expired,
}

/// Satellite packet encoder/decoder.
pub struct PacketCodec {
    /// Maximum packet payload size in bytes.
    max_payload_bytes: usize,
    /// Sequence counter.
    sequence: u32,
    /// Source identifier for this node.
    source_id: String,
    /// Packets created.
    total_created: u64,
    /// Packets encoded.
    _total_encoded: u64,
}

impl PacketCodec {
    pub fn new(source_id: &str, max_payload_bytes: usize) -> Self {
        Self {
            max_payload_bytes,
            sequence: 0,
            source_id: source_id.to_string(),
            total_created: 0,
            _total_encoded: 0,
        }
    }

    /// Create a new packet.
    pub fn create_packet(
        &mut self,
        packet_type: PacketType,
        priority: PacketPriority,
        destination: Option<&str>,
        payload: &[u8],
        ttl_seconds: u64,
    ) -> Option<SatellitePacket> {
        if payload.len() > self.max_payload_bytes {
            warn!(
                size = payload.len(),
                max = self.max_payload_bytes,
                "payload exceeds maximum packet size"
            );
            return None;
        }

        self.sequence += 1;
        self.total_created += 1;

        let checksum = Self::compute_checksum(payload);

        let packet = SatellitePacket {
            id: EntityId::new(),
            packet_type,
            priority,
            source_id: self.source_id.clone(),
            destination_id: destination.map(|s| s.to_string()),
            payload: payload.to_vec(),
            payload_size_bytes: payload.len(),
            sequence_number: self.sequence,
            created_at: Utc::now(),
            ttl_seconds,
            checksum,
            status: PacketStatus::Queued,
            retries: 0,
        };

        debug!(
            seq = self.sequence,
            ptype = ?packet_type,
            size = payload.len(),
            "satellite packet created"
        );

        Some(packet)
    }

    /// Create a position report packet.
    pub fn position_report(
        &mut self,
        lat: f64,
        lon: f64,
        alt: f64,
        accuracy_m: f64,
    ) -> Option<SatellitePacket> {
        // Compact encoding: 4 x f64 = 32 bytes.
        let mut payload = Vec::with_capacity(32);
        payload.extend_from_slice(&lat.to_le_bytes());
        payload.extend_from_slice(&lon.to_le_bytes());
        payload.extend_from_slice(&alt.to_le_bytes());
        payload.extend_from_slice(&accuracy_m.to_le_bytes());

        self.create_packet(
            PacketType::PositionReport,
            PacketPriority::Normal,
            None,
            &payload,
            3600,
        )
    }

    /// Create a distress signal packet (highest priority, broadcast).
    pub fn distress_signal(&mut self, message: &str) -> Option<SatellitePacket> {
        self.create_packet(
            PacketType::DistressSignal,
            PacketPriority::Emergency,
            None,
            message.as_bytes(),
            86400, // 24 hour TTL.
        )
    }

    /// Create an acknowledgement packet.
    pub fn ack(&mut self, original_id: &EntityId, destination: &str) -> Option<SatellitePacket> {
        let payload = original_id.to_string();
        self.create_packet(
            PacketType::Ack,
            PacketPriority::High,
            Some(destination),
            payload.as_bytes(),
            300,
        )
    }

    /// Verify a packet's checksum.
    pub fn verify_checksum(packet: &SatellitePacket) -> bool {
        Self::compute_checksum(&packet.payload) == packet.checksum
    }

    /// Compute CRC32-like checksum.
    fn compute_checksum(data: &[u8]) -> u32 {
        let mut hash: u32 = 0xFFFF_FFFF;
        for &byte in data {
            hash ^= byte as u32;
            for _ in 0..8 {
                if hash & 1 != 0 {
                    hash = (hash >> 1) ^ 0xEDB8_8320;
                } else {
                    hash >>= 1;
                }
            }
        }
        hash ^ 0xFFFF_FFFF
    }

    /// Decode a position report payload back to coordinates.
    pub fn decode_position(payload: &[u8]) -> Option<(f64, f64, f64, f64)> {
        if payload.len() < 32 {
            return None;
        }
        let lat = f64::from_le_bytes(payload[0..8].try_into().ok()?);
        let lon = f64::from_le_bytes(payload[8..16].try_into().ok()?);
        let alt = f64::from_le_bytes(payload[16..24].try_into().ok()?);
        let acc = f64::from_le_bytes(payload[24..32].try_into().ok()?);
        Some((lat, lon, alt, acc))
    }

    /// Total packets created.
    pub fn total_created(&self) -> u64 {
        self.total_created
    }

    /// Current sequence number.
    pub fn sequence(&self) -> u32 {
        self.sequence
    }

    /// Maximum payload size.
    pub fn max_payload_bytes(&self) -> usize {
        self.max_payload_bytes
    }

    /// Source ID.
    pub fn source_id(&self) -> &str {
        &self.source_id
    }
}

impl Default for PacketCodec {
    fn default() -> Self {
        Self::new("aurora-default", 256) // 256 byte max payload
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_packet() {
        let mut codec = PacketCodec::new("node-1", 256);
        let pkt = codec
            .create_packet(
                PacketType::Heartbeat,
                PacketPriority::Low,
                None,
                b"alive",
                60,
            )
            .unwrap();

        assert_eq!(pkt.packet_type, PacketType::Heartbeat);
        assert_eq!(pkt.source_id, "node-1");
        assert_eq!(pkt.payload_size_bytes, 5);
        assert_eq!(pkt.sequence_number, 1);
        assert_eq!(pkt.status, PacketStatus::Queued);
    }

    #[test]
    fn payload_too_large_rejected() {
        let mut codec = PacketCodec::new("node-1", 10);
        let large_payload = vec![0u8; 100];
        let result = codec.create_packet(
            PacketType::TextMessage,
            PacketPriority::Normal,
            None,
            &large_payload,
            60,
        );
        assert!(result.is_none());
    }

    #[test]
    fn position_report_encode_decode() {
        let mut codec = PacketCodec::new("nav-1", 256);
        let pkt = codec.position_report(32.0853, 34.7818, 50.0, 2.5).unwrap();

        assert_eq!(pkt.packet_type, PacketType::PositionReport);
        assert_eq!(pkt.payload_size_bytes, 32);

        let (lat, lon, alt, acc) = PacketCodec::decode_position(&pkt.payload).unwrap();
        assert!((lat - 32.0853).abs() < f64::EPSILON);
        assert!((lon - 34.7818).abs() < f64::EPSILON);
        assert!((alt - 50.0).abs() < f64::EPSILON);
        assert!((acc - 2.5).abs() < f64::EPSILON);
    }

    #[test]
    fn distress_signal_emergency_priority() {
        let mut codec = PacketCodec::new("sos-1", 256);
        let pkt = codec.distress_signal("MAYDAY").unwrap();

        assert_eq!(pkt.packet_type, PacketType::DistressSignal);
        assert_eq!(pkt.priority, PacketPriority::Emergency);
        assert_eq!(pkt.ttl_seconds, 86400);
    }

    #[test]
    fn checksum_verification() {
        let mut codec = PacketCodec::new("node-1", 256);
        let pkt = codec
            .create_packet(
                PacketType::TextMessage,
                PacketPriority::Normal,
                None,
                b"hello satellite",
                60,
            )
            .unwrap();

        assert!(PacketCodec::verify_checksum(&pkt));

        // Tamper with payload.
        let mut tampered = pkt;
        tampered.payload[0] = 0xFF;
        assert!(!PacketCodec::verify_checksum(&tampered));
    }

    #[test]
    fn sequence_increments() {
        let mut codec = PacketCodec::new("node-1", 256);
        codec.create_packet(PacketType::Heartbeat, PacketPriority::Low, None, b"a", 60);
        codec.create_packet(PacketType::Heartbeat, PacketPriority::Low, None, b"b", 60);
        assert_eq!(codec.sequence(), 2);
        assert_eq!(codec.total_created(), 2);
    }

    #[test]
    fn ack_packet() {
        let mut codec = PacketCodec::new("node-1", 256);
        let original_id = EntityId::new();
        let ack = codec.ack(&original_id, "node-2").unwrap();

        assert_eq!(ack.packet_type, PacketType::Ack);
        assert_eq!(ack.priority, PacketPriority::High);
        assert_eq!(ack.destination_id.as_deref(), Some("node-2"));
    }

    #[test]
    fn decode_position_short_payload() {
        assert!(PacketCodec::decode_position(&[0u8; 16]).is_none());
    }

    #[test]
    fn default_codec() {
        let codec = PacketCodec::default();
        assert_eq!(codec.max_payload_bytes(), 256);
        assert_eq!(codec.source_id(), "aurora-default");
    }

    #[test]
    fn empty_payload_checksum() {
        let mut codec = PacketCodec::new("node-1", 256);
        let pkt = codec
            .create_packet(PacketType::Heartbeat, PacketPriority::Low, None, &[], 60)
            .unwrap();
        assert!(PacketCodec::verify_checksum(&pkt));
    }
}
