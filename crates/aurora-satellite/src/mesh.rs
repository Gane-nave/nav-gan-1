//! Mesh networking — peer-to-peer communication between nearby devices.
//!
//! Enables multi-hop message relay when satellite/internet connectivity
//! is unavailable. Uses flooding with TTL for message propagation.

use aurora_core::types::EntityId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use tracing::{debug, info, warn};

/// A mesh network node (peer device).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshNode {
    pub id: EntityId,
    pub name: String,
    pub last_seen: DateTime<Utc>,
    pub signal_strength_dbm: f64,
    pub hop_count: u32,
    pub capabilities: Vec<MeshCapability>,
    pub status: NodeStatus,
}

/// Capabilities a mesh node can advertise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MeshCapability {
    /// Can relay messages.
    Relay,
    /// Has satellite uplink.
    SatelliteUplink,
    /// Has internet connectivity.
    InternetGateway,
    /// Has GNSS fix.
    GnssFix,
    /// Has emergency services contact.
    EmergencyRelay,
}

/// Status of a mesh node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeStatus {
    /// Node is active and reachable.
    Active,
    /// Node was recently seen but is now unreachable.
    Stale,
    /// Node has been lost (no response for extended period).
    Lost,
}

/// A mesh message routed through the network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshMessage {
    pub id: EntityId,
    pub origin_node: EntityId,
    pub destination_node: Option<EntityId>,
    pub payload: String,
    pub hop_count: u32,
    pub max_hops: u32,
    pub created_at: DateTime<Utc>,
    pub relayed_by: Vec<EntityId>,
}

/// Mesh network manager — discovers peers and routes messages.
pub struct MeshNetwork {
    local_node_id: EntityId,
    _local_name: String,
    peers: HashMap<EntityId, MeshNode>,
    /// Messages we've already seen (for deduplication).
    seen_messages: HashSet<EntityId>,
    /// Messages awaiting relay.
    outbox: Vec<MeshMessage>,
    /// Received messages.
    inbox: Vec<MeshMessage>,
    /// Maximum hops for flood routing.
    max_hops: u32,
    /// Stale timeout in seconds.
    stale_timeout_secs: u64,
    /// Total messages relayed.
    total_relayed: u64,
    /// Total messages received.
    total_received: u64,
}

impl MeshNetwork {
    pub fn new(name: &str, max_hops: u32) -> Self {
        Self {
            local_node_id: EntityId::new(),
            _local_name: name.to_string(),
            peers: HashMap::new(),
            seen_messages: HashSet::new(),
            outbox: Vec::new(),
            inbox: Vec::new(),
            max_hops,
            stale_timeout_secs: 300,
            total_relayed: 0,
            total_received: 0,
        }
    }

    /// Register a discovered peer.
    pub fn discover_peer(
        &mut self,
        name: &str,
        signal_dbm: f64,
        hop_count: u32,
        capabilities: Vec<MeshCapability>,
    ) -> EntityId {
        let id = EntityId::new();
        let node = MeshNode {
            id,
            name: name.to_string(),
            last_seen: Utc::now(),
            signal_strength_dbm: signal_dbm,
            hop_count,
            capabilities,
            status: NodeStatus::Active,
        };
        info!(peer = name, signal = signal_dbm, "discovered mesh peer");
        self.peers.insert(id, node);
        id
    }

    /// Update a peer's last-seen timestamp and signal strength.
    pub fn update_peer(&mut self, peer_id: &EntityId, signal_dbm: f64) -> bool {
        if let Some(peer) = self.peers.get_mut(peer_id) {
            peer.last_seen = Utc::now();
            peer.signal_strength_dbm = signal_dbm;
            peer.status = NodeStatus::Active;
            true
        } else {
            false
        }
    }

    /// Mark stale peers (not seen within timeout).
    pub fn mark_stale_peers(&mut self) {
        let now = Utc::now();
        for peer in self.peers.values_mut() {
            if peer.status == NodeStatus::Active {
                let elapsed = (now - peer.last_seen).num_seconds() as u64;
                if elapsed > self.stale_timeout_secs * 2 {
                    peer.status = NodeStatus::Lost;
                    warn!(peer = %peer.name, "mesh peer lost");
                } else if elapsed > self.stale_timeout_secs {
                    peer.status = NodeStatus::Stale;
                    debug!(peer = %peer.name, "mesh peer stale");
                }
            }
        }
    }

    /// Send a message (broadcast to all peers or to a specific destination).
    pub fn send_message(&mut self, destination: Option<EntityId>, payload: &str) -> EntityId {
        let id = EntityId::new();
        let msg = MeshMessage {
            id,
            origin_node: self.local_node_id,
            destination_node: destination,
            payload: payload.to_string(),
            hop_count: 0,
            max_hops: self.max_hops,
            created_at: Utc::now(),
            relayed_by: vec![self.local_node_id],
        };
        self.seen_messages.insert(id);
        self.outbox.push(msg);
        debug!(msg_id = %id, "message queued for mesh transmission");
        id
    }

    /// Receive a message from a peer. Returns true if accepted (not a duplicate).
    pub fn receive_message(&mut self, mut msg: MeshMessage) -> bool {
        // Deduplication.
        if self.seen_messages.contains(&msg.id) {
            debug!(msg_id = %msg.id, "duplicate mesh message — dropping");
            return false;
        }
        self.seen_messages.insert(msg.id);
        self.total_received += 1;

        // Check if we're the destination.
        let for_us =
            msg.destination_node.is_none() || msg.destination_node == Some(self.local_node_id);

        if for_us {
            self.inbox.push(msg.clone());
        }

        // Relay if hops remaining and we're not the origin.
        if msg.hop_count < msg.max_hops && msg.origin_node != self.local_node_id {
            msg.hop_count += 1;
            msg.relayed_by.push(self.local_node_id);
            self.outbox.push(msg);
            self.total_relayed += 1;
        }

        true
    }

    /// Drain the outbox (messages ready for transmission).
    pub fn drain_outbox(&mut self) -> Vec<MeshMessage> {
        std::mem::take(&mut self.outbox)
    }

    /// Get received messages.
    pub fn inbox(&self) -> &[MeshMessage] {
        &self.inbox
    }

    /// Clear the inbox.
    pub fn clear_inbox(&mut self) {
        self.inbox.clear();
    }

    /// Get all active peers.
    pub fn active_peers(&self) -> Vec<&MeshNode> {
        self.peers
            .values()
            .filter(|p| p.status == NodeStatus::Active)
            .collect()
    }

    /// Get peers with a specific capability.
    pub fn peers_with_capability(&self, cap: MeshCapability) -> Vec<&MeshNode> {
        self.peers
            .values()
            .filter(|p| p.status == NodeStatus::Active && p.capabilities.contains(&cap))
            .collect()
    }

    /// Find the nearest internet gateway.
    pub fn nearest_gateway(&self) -> Option<&MeshNode> {
        self.peers_with_capability(MeshCapability::InternetGateway)
            .into_iter()
            .min_by_key(|p| p.hop_count)
    }

    /// Local node ID.
    pub fn local_id(&self) -> EntityId {
        self.local_node_id
    }

    /// Total peers.
    pub fn peer_count(&self) -> usize {
        self.peers.len()
    }

    /// Total messages relayed.
    pub fn total_relayed(&self) -> u64 {
        self.total_relayed
    }

    /// Total messages received.
    pub fn total_received(&self) -> u64 {
        self.total_received
    }

    /// Outbox size.
    pub fn outbox_size(&self) -> usize {
        self.outbox.len()
    }
}

impl Default for MeshNetwork {
    fn default() -> Self {
        Self::new("aurora-mesh", 5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discover_and_list_peers() {
        let mut mesh = MeshNetwork::new("node-a", 3);
        mesh.discover_peer("node-b", -60.0, 1, vec![MeshCapability::Relay]);
        mesh.discover_peer("node-c", -70.0, 2, vec![MeshCapability::InternetGateway]);

        assert_eq!(mesh.peer_count(), 2);
        assert_eq!(mesh.active_peers().len(), 2);
    }

    #[test]
    fn send_and_drain_outbox() {
        let mut mesh = MeshNetwork::new("node-a", 3);
        let msg_id = mesh.send_message(None, "hello mesh");

        assert_eq!(mesh.outbox_size(), 1);
        let outbox = mesh.drain_outbox();
        assert_eq!(outbox.len(), 1);
        assert_eq!(outbox[0].id, msg_id);
        assert_eq!(mesh.outbox_size(), 0);
    }

    #[test]
    fn receive_message_deduplication() {
        let mut mesh = MeshNetwork::new("node-a", 3);

        let msg = MeshMessage {
            id: EntityId::new(),
            origin_node: EntityId::new(),
            destination_node: None,
            payload: "test".to_string(),
            hop_count: 0,
            max_hops: 3,
            created_at: Utc::now(),
            relayed_by: vec![],
        };

        let id = msg.id;
        assert!(mesh.receive_message(msg.clone()));

        // Second time — duplicate, rejected.
        let mut dup = msg;
        dup.id = id; // Same ID.
        assert!(!mesh.receive_message(dup));
    }

    #[test]
    fn message_relay_increments_hop() {
        let mut mesh = MeshNetwork::new("relay-node", 5);

        let msg = MeshMessage {
            id: EntityId::new(),
            origin_node: EntityId::new(), // Different origin.
            destination_node: None,
            payload: "relay me".to_string(),
            hop_count: 1,
            max_hops: 5,
            created_at: Utc::now(),
            relayed_by: vec![],
        };

        mesh.receive_message(msg);

        // Should be in inbox (broadcast) and relayed in outbox.
        assert_eq!(mesh.inbox().len(), 1);
        let outbox = mesh.drain_outbox();
        assert_eq!(outbox.len(), 1);
        assert_eq!(outbox[0].hop_count, 2); // Incremented.
        assert_eq!(mesh.total_relayed(), 1);
    }

    #[test]
    fn message_not_relayed_at_max_hops() {
        let mut mesh = MeshNetwork::new("relay-node", 5);

        let msg = MeshMessage {
            id: EntityId::new(),
            origin_node: EntityId::new(),
            destination_node: None,
            payload: "max hops".to_string(),
            hop_count: 5,
            max_hops: 5,
            created_at: Utc::now(),
            relayed_by: vec![],
        };

        mesh.receive_message(msg);
        let outbox = mesh.drain_outbox();
        assert!(outbox.is_empty()); // Not relayed.
    }

    #[test]
    fn peers_with_capability() {
        let mut mesh = MeshNetwork::new("node-a", 3);
        mesh.discover_peer("relay", -60.0, 1, vec![MeshCapability::Relay]);
        mesh.discover_peer(
            "gateway",
            -50.0,
            1,
            vec![MeshCapability::InternetGateway, MeshCapability::Relay],
        );
        mesh.discover_peer("basic", -70.0, 2, vec![]);

        assert_eq!(mesh.peers_with_capability(MeshCapability::Relay).len(), 2);
        assert_eq!(
            mesh.peers_with_capability(MeshCapability::InternetGateway)
                .len(),
            1
        );
    }

    #[test]
    fn nearest_gateway() {
        let mut mesh = MeshNetwork::new("node-a", 3);
        mesh.discover_peer("far-gw", -80.0, 3, vec![MeshCapability::InternetGateway]);
        mesh.discover_peer("near-gw", -50.0, 1, vec![MeshCapability::InternetGateway]);

        let nearest = mesh.nearest_gateway().unwrap();
        assert_eq!(nearest.name, "near-gw");
        assert_eq!(nearest.hop_count, 1);
    }

    #[test]
    fn no_gateway_returns_none() {
        let mesh = MeshNetwork::new("isolated", 3);
        assert!(mesh.nearest_gateway().is_none());
    }

    #[test]
    fn update_peer_refreshes_state() {
        let mut mesh = MeshNetwork::new("node-a", 3);
        let id = mesh.discover_peer("node-b", -60.0, 1, vec![]);
        assert!(mesh.update_peer(&id, -40.0));

        let peer = mesh.peers.get(&id).unwrap();
        assert!((peer.signal_strength_dbm - (-40.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn clear_inbox() {
        let mut mesh = MeshNetwork::new("node-a", 3);
        let msg = MeshMessage {
            id: EntityId::new(),
            origin_node: EntityId::new(),
            destination_node: None,
            payload: "msg".to_string(),
            hop_count: 0,
            max_hops: 3,
            created_at: Utc::now(),
            relayed_by: vec![],
        };
        mesh.receive_message(msg);
        assert_eq!(mesh.inbox().len(), 1);

        mesh.clear_inbox();
        assert!(mesh.inbox().is_empty());
    }

    #[test]
    fn default_mesh_config() {
        let mesh = MeshNetwork::default();
        assert_eq!(mesh._local_name, "aurora-mesh");
        assert_eq!(mesh.max_hops, 5);
    }
}
