//! WebSocket connection management — connection lifecycle, state tracking, and metadata.

use std::collections::HashMap;

/// Unique connection identifier.
pub type ConnId = u64;

/// Connection state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnState {
    /// Handshake in progress.
    Connecting,
    /// Connection established and active.
    Open,
    /// Close frame sent, waiting for acknowledgment.
    Closing,
    /// Connection fully closed.
    Closed,
}

/// Metadata about a WebSocket connection.
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    /// Unique connection ID.
    pub id: ConnId,
    /// Remote address (e.g., "192.168.1.1:54321").
    pub remote_addr: String,
    /// Current state.
    pub state: ConnState,
    /// Timestamp of connection creation (epoch ms).
    pub connected_at: u64,
    /// Number of messages sent.
    pub messages_sent: u64,
    /// Number of messages received.
    pub messages_received: u64,
    /// Custom properties.
    pub properties: HashMap<String, String>,
}

impl ConnectionInfo {
    /// Create a new connection info.
    pub fn new(id: ConnId, remote_addr: String, connected_at: u64) -> Self {
        Self {
            id,
            remote_addr,
            state: ConnState::Connecting,
            connected_at,
            messages_sent: 0,
            messages_received: 0,
            properties: HashMap::new(),
        }
    }

    /// Mark connection as open.
    pub fn open(&mut self) {
        self.state = ConnState::Open;
    }

    /// Mark connection as closing.
    pub fn close(&mut self) {
        self.state = ConnState::Closing;
    }

    /// Mark connection as fully closed.
    pub fn closed(&mut self) {
        self.state = ConnState::Closed;
    }

    /// Record a sent message.
    pub fn record_sent(&mut self) {
        self.messages_sent += 1;
    }

    /// Record a received message.
    pub fn record_received(&mut self) {
        self.messages_received += 1;
    }

    /// Set a property.
    pub fn set_property(&mut self, key: String, value: String) {
        self.properties.insert(key, value);
    }
}

/// Manages a pool of WebSocket connections.
pub struct ConnectionPool {
    connections: HashMap<ConnId, ConnectionInfo>,
    next_id: ConnId,
    max_connections: usize,
}

impl ConnectionPool {
    /// Create a new connection pool with a maximum size.
    pub fn new(max_connections: usize) -> Self {
        Self {
            connections: HashMap::new(),
            next_id: 1,
            max_connections,
        }
    }

    /// Accept a new connection. Returns None if pool is full.
    pub fn accept(&mut self, remote_addr: String, timestamp: u64) -> Option<ConnId> {
        if self.connections.len() >= self.max_connections {
            return None;
        }
        let id = self.next_id;
        self.next_id += 1;
        let mut info = ConnectionInfo::new(id, remote_addr, timestamp);
        info.open();
        self.connections.insert(id, info);
        Some(id)
    }

    /// Get connection info by ID.
    pub fn get(&self, id: ConnId) -> Option<&ConnectionInfo> {
        self.connections.get(&id)
    }

    /// Get mutable connection info by ID.
    pub fn get_mut(&mut self, id: ConnId) -> Option<&mut ConnectionInfo> {
        self.connections.get_mut(&id)
    }

    /// Close and remove a connection.
    pub fn disconnect(&mut self, id: ConnId) -> bool {
        self.connections.remove(&id).is_some()
    }

    /// Get all active connection IDs.
    pub fn active_connections(&self) -> Vec<ConnId> {
        self.connections
            .iter()
            .filter(|(_, c)| c.state == ConnState::Open)
            .map(|(&id, _)| id)
            .collect()
    }

    /// Total number of connections (all states).
    pub fn count(&self) -> usize {
        self.connections.len()
    }

    /// Number of open connections.
    pub fn active_count(&self) -> usize {
        self.connections
            .values()
            .filter(|c| c.state == ConnState::Open)
            .count()
    }

    /// Remove all closed connections.
    pub fn cleanup_closed(&mut self) -> usize {
        let before = self.connections.len();
        self.connections.retain(|_, c| c.state != ConnState::Closed);
        before - self.connections.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_lifecycle() {
        let mut info = ConnectionInfo::new(1, "127.0.0.1:5000".into(), 1000);
        assert_eq!(info.state, ConnState::Connecting);

        info.open();
        assert_eq!(info.state, ConnState::Open);

        info.record_sent();
        info.record_sent();
        info.record_received();
        assert_eq!(info.messages_sent, 2);
        assert_eq!(info.messages_received, 1);

        info.close();
        assert_eq!(info.state, ConnState::Closing);

        info.closed();
        assert_eq!(info.state, ConnState::Closed);
    }

    #[test]
    fn test_connection_properties() {
        let mut info = ConnectionInfo::new(1, "10.0.0.1:8080".into(), 500);
        info.set_property("user_id".into(), "42".into());
        assert_eq!(info.properties.get("user_id"), Some(&"42".to_string()));
    }

    #[test]
    fn test_pool_accept_and_disconnect() {
        let mut pool = ConnectionPool::new(3);
        let id1 = pool.accept("a:1".into(), 100).unwrap();
        let id2 = pool.accept("b:2".into(), 200).unwrap();
        let id3 = pool.accept("c:3".into(), 300).unwrap();
        assert_eq!(pool.count(), 3);
        assert_eq!(pool.active_count(), 3);

        // Pool full
        assert!(pool.accept("d:4".into(), 400).is_none());

        // Disconnect one
        assert!(pool.disconnect(id2));
        assert_eq!(pool.count(), 2);
        assert!(pool.accept("e:5".into(), 500).is_some());

        assert!(!pool.disconnect(id2)); // already removed
        assert!(pool.active_connections().contains(&id1));
        assert!(pool.active_connections().contains(&id3));
    }

    #[test]
    fn test_pool_cleanup_closed() {
        let mut pool = ConnectionPool::new(10);
        let id1 = pool.accept("a:1".into(), 100).unwrap();
        let id2 = pool.accept("b:2".into(), 200).unwrap();
        pool.accept("c:3".into(), 300).unwrap();

        pool.get_mut(id1).unwrap().closed();
        pool.get_mut(id2).unwrap().closed();

        let removed = pool.cleanup_closed();
        assert_eq!(removed, 2);
        assert_eq!(pool.count(), 1);
    }

    #[test]
    fn test_pool_max_connections() {
        let mut pool = ConnectionPool::new(1);
        assert!(pool.accept("a:1".into(), 100).is_some());
        assert!(pool.accept("b:2".into(), 200).is_none());
    }
}
