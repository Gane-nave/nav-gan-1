//! Heartbeat / ping-pong liveness detection for WebSocket connections.

use std::collections::HashMap;

use crate::connection::ConnId;

/// Heartbeat state for a single connection.
#[derive(Debug, Clone)]
struct HeartbeatEntry {
    /// Last time a pong was received (epoch ms).
    last_pong: u64,
    /// Last time a ping was sent (epoch ms).
    last_ping: u64,
    /// Number of unanswered pings.
    unanswered_pings: u32,
}

/// Heartbeat manager — tracks liveness across connections.
pub struct HeartbeatManager {
    entries: HashMap<ConnId, HeartbeatEntry>,
    /// Interval between pings (ms).
    ping_interval_ms: u64,
    /// Maximum unanswered pings before declaring dead.
    max_unanswered: u32,
    /// Timeout for a pong response (ms).
    pong_timeout_ms: u64,
}

impl HeartbeatManager {
    /// Create a new heartbeat manager.
    pub fn new(ping_interval_ms: u64, pong_timeout_ms: u64, max_unanswered: u32) -> Self {
        Self {
            entries: HashMap::new(),
            ping_interval_ms,
            max_unanswered,
            pong_timeout_ms,
        }
    }

    /// Register a connection for heartbeat tracking.
    pub fn register(&mut self, conn_id: ConnId, now: u64) {
        self.entries.insert(
            conn_id,
            HeartbeatEntry {
                last_pong: now,
                last_ping: 0,
                unanswered_pings: 0,
            },
        );
    }

    /// Unregister a connection.
    pub fn unregister(&mut self, conn_id: ConnId) {
        self.entries.remove(&conn_id);
    }

    /// Record that a pong was received from a connection.
    pub fn pong_received(&mut self, conn_id: ConnId, now: u64) {
        if let Some(entry) = self.entries.get_mut(&conn_id) {
            entry.last_pong = now;
            entry.unanswered_pings = 0;
        }
    }

    /// Check which connections need a ping sent to them.
    /// Returns list of connection IDs that need pings.
    pub fn connections_needing_ping(&self, now: u64) -> Vec<ConnId> {
        self.entries
            .iter()
            .filter(|(_, entry)| {
                let elapsed = now.saturating_sub(entry.last_pong);
                let since_last_ping = now.saturating_sub(entry.last_ping);
                elapsed >= self.ping_interval_ms && since_last_ping >= self.ping_interval_ms
            })
            .map(|(&id, _)| id)
            .collect()
    }

    /// Record that a ping was sent to a connection.
    pub fn ping_sent(&mut self, conn_id: ConnId, now: u64) {
        if let Some(entry) = self.entries.get_mut(&conn_id) {
            entry.last_ping = now;
            entry.unanswered_pings += 1;
        }
    }

    /// Check which connections are considered dead (too many unanswered pings
    /// or pong timeout exceeded).
    pub fn dead_connections(&self, now: u64) -> Vec<ConnId> {
        self.entries
            .iter()
            .filter(|(_, entry)| {
                entry.unanswered_pings >= self.max_unanswered
                    || (entry.last_ping > 0
                        && now.saturating_sub(entry.last_pong) > self.pong_timeout_ms
                        && entry.unanswered_pings > 0)
            })
            .map(|(&id, _)| id)
            .collect()
    }

    /// Get the number of unanswered pings for a connection.
    pub fn unanswered_pings(&self, conn_id: ConnId) -> u32 {
        self.entries.get(&conn_id).map_or(0, |e| e.unanswered_pings)
    }

    /// Number of tracked connections.
    pub fn tracked_count(&self) -> usize {
        self.entries.len()
    }

    /// Get the last pong time for a connection.
    pub fn last_pong(&self, conn_id: ConnId) -> Option<u64> {
        self.entries.get(&conn_id).map(|e| e.last_pong)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_unregister() {
        let mut hb = HeartbeatManager::new(1000, 5000, 3);
        hb.register(1, 0);
        hb.register(2, 0);
        assert_eq!(hb.tracked_count(), 2);

        hb.unregister(1);
        assert_eq!(hb.tracked_count(), 1);
    }

    #[test]
    fn test_ping_needed_after_interval() {
        let hb = HeartbeatManager::new(1000, 5000, 3);
        // No connections yet
        assert!(hb.connections_needing_ping(0).is_empty());
    }

    #[test]
    fn test_ping_pong_cycle() {
        let mut hb = HeartbeatManager::new(1000, 5000, 3);
        hb.register(1, 0);

        // At time 0, no ping needed
        assert!(hb.connections_needing_ping(500).is_empty());

        // At time 1500, ping needed
        let need_ping = hb.connections_needing_ping(1500);
        assert_eq!(need_ping, vec![1]);

        // Send ping
        hb.ping_sent(1, 1500);
        assert_eq!(hb.unanswered_pings(1), 1);

        // Receive pong
        hb.pong_received(1, 1600);
        assert_eq!(hb.unanswered_pings(1), 0);
        assert_eq!(hb.last_pong(1), Some(1600));
    }

    #[test]
    fn test_dead_connection_unanswered() {
        let mut hb = HeartbeatManager::new(1000, 5000, 3);
        hb.register(1, 0);

        // Simulate 3 unanswered pings
        hb.ping_sent(1, 1000);
        hb.ping_sent(1, 2000);
        hb.ping_sent(1, 3000);

        let dead = hb.dead_connections(3000);
        assert_eq!(dead, vec![1]);
    }

    #[test]
    fn test_dead_connection_timeout() {
        let mut hb = HeartbeatManager::new(1000, 5000, 10);
        hb.register(1, 0);

        hb.ping_sent(1, 1000);
        // Pong timeout = 5000, last_pong=0, now=6000, unanswered=1
        let dead = hb.dead_connections(6000);
        assert_eq!(dead, vec![1]);
    }

    #[test]
    fn test_alive_connection() {
        let mut hb = HeartbeatManager::new(1000, 5000, 3);
        hb.register(1, 0);
        hb.ping_sent(1, 1000);
        hb.pong_received(1, 1100);

        let dead = hb.dead_connections(1200);
        assert!(dead.is_empty());
    }

    #[test]
    fn test_multiple_connections() {
        let mut hb = HeartbeatManager::new(1000, 5000, 3);
        hb.register(1, 0);
        hb.register(2, 0);
        hb.register(3, 0);

        // Only conn 1 gets pings without pong
        hb.ping_sent(1, 1000);
        hb.ping_sent(1, 2000);
        hb.ping_sent(1, 3000);

        hb.ping_sent(2, 1000);
        hb.pong_received(2, 1100);

        let dead = hb.dead_connections(3000);
        assert_eq!(dead, vec![1]);
    }
}
