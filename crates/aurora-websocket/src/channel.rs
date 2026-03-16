//! Channel & room management — pub/sub rooms for broadcasting messages.

use std::collections::{HashMap, HashSet};

use crate::connection::ConnId;

/// A named channel that connections can join.
#[derive(Debug)]
pub struct Channel {
    /// Channel name.
    pub name: String,
    /// Connected members.
    members: HashSet<ConnId>,
    /// Maximum members allowed (0 = unlimited).
    max_members: usize,
    /// Whether the channel is private.
    pub is_private: bool,
    /// Creation timestamp (epoch ms).
    pub created_at: u64,
    /// Message history (payload bytes).
    history: Vec<Vec<u8>>,
    /// Maximum history size.
    max_history: usize,
}

impl Channel {
    /// Create a new channel.
    pub fn new(name: String, created_at: u64) -> Self {
        Self {
            name,
            members: HashSet::new(),
            max_members: 0,
            is_private: false,
            created_at,
            history: Vec::new(),
            max_history: 100,
        }
    }

    /// Set the maximum number of members.
    pub fn with_max_members(mut self, max: usize) -> Self {
        self.max_members = max;
        self
    }

    /// Set the channel as private.
    pub fn with_private(mut self, private: bool) -> Self {
        self.is_private = private;
        self
    }

    /// Set max history size.
    pub fn with_max_history(mut self, max: usize) -> Self {
        self.max_history = max;
        self
    }

    /// Join a connection to this channel.
    pub fn join(&mut self, conn_id: ConnId) -> bool {
        if self.max_members > 0 && self.members.len() >= self.max_members {
            return false;
        }
        self.members.insert(conn_id)
    }

    /// Remove a connection from this channel.
    pub fn leave(&mut self, conn_id: ConnId) -> bool {
        self.members.remove(&conn_id)
    }

    /// Check if a connection is a member.
    pub fn has_member(&self, conn_id: ConnId) -> bool {
        self.members.contains(&conn_id)
    }

    /// Get all member IDs.
    pub fn members(&self) -> Vec<ConnId> {
        self.members.iter().copied().collect()
    }

    /// Number of members.
    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    /// Broadcast a message to all members. Returns list of recipient IDs
    /// (excluding the sender if provided).
    pub fn broadcast(&mut self, payload: Vec<u8>, sender: Option<ConnId>) -> Vec<ConnId> {
        // Store in history
        if self.history.len() >= self.max_history {
            self.history.remove(0);
        }
        self.history.push(payload);

        self.members
            .iter()
            .copied()
            .filter(|&id| sender.map_or(true, |s| id != s))
            .collect()
    }

    /// Get recent message history.
    pub fn history(&self, count: usize) -> &[Vec<u8>] {
        let start = self.history.len().saturating_sub(count);
        &self.history[start..]
    }

    /// Clear all history.
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Check if channel is empty.
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }
}

/// Manages multiple channels.
pub struct ChannelManager {
    channels: HashMap<String, Channel>,
    /// Connection-to-channels mapping for fast lookup.
    conn_channels: HashMap<ConnId, HashSet<String>>,
}

impl ChannelManager {
    /// Create a new channel manager.
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
            conn_channels: HashMap::new(),
        }
    }

    /// Create a new channel. Returns false if it already exists.
    pub fn create(&mut self, name: String, created_at: u64) -> bool {
        if self.channels.contains_key(&name) {
            return false;
        }
        self.channels
            .insert(name.clone(), Channel::new(name, created_at));
        true
    }

    /// Create a channel with options.
    pub fn create_with(
        &mut self,
        name: String,
        created_at: u64,
        max_members: usize,
        private: bool,
    ) -> bool {
        if self.channels.contains_key(&name) {
            return false;
        }
        let ch = Channel::new(name.clone(), created_at)
            .with_max_members(max_members)
            .with_private(private);
        self.channels.insert(name, ch);
        true
    }

    /// Delete a channel. Returns the number of members that were in it.
    pub fn delete(&mut self, name: &str) -> usize {
        if let Some(ch) = self.channels.remove(name) {
            let count = ch.member_count();
            for &member in &ch.members() {
                if let Some(set) = self.conn_channels.get_mut(&member) {
                    set.remove(name);
                }
            }
            count
        } else {
            0
        }
    }

    /// Join a connection to a channel.
    pub fn join(&mut self, channel: &str, conn_id: ConnId) -> bool {
        if let Some(ch) = self.channels.get_mut(channel) {
            if ch.join(conn_id) {
                self.conn_channels
                    .entry(conn_id)
                    .or_default()
                    .insert(channel.to_string());
                return true;
            }
        }
        false
    }

    /// Leave a channel.
    pub fn leave(&mut self, channel: &str, conn_id: ConnId) -> bool {
        if let Some(ch) = self.channels.get_mut(channel) {
            if ch.leave(conn_id) {
                if let Some(set) = self.conn_channels.get_mut(&conn_id) {
                    set.remove(channel);
                }
                return true;
            }
        }
        false
    }

    /// Remove a connection from all channels (on disconnect).
    pub fn disconnect(&mut self, conn_id: ConnId) -> Vec<String> {
        let channels = self
            .conn_channels
            .remove(&conn_id)
            .unwrap_or_default()
            .into_iter()
            .collect::<Vec<_>>();

        for ch_name in &channels {
            if let Some(ch) = self.channels.get_mut(ch_name.as_str()) {
                ch.leave(conn_id);
            }
        }
        channels
    }

    /// Broadcast to a channel. Returns recipient list.
    pub fn broadcast(
        &mut self,
        channel: &str,
        payload: Vec<u8>,
        sender: Option<ConnId>,
    ) -> Vec<ConnId> {
        if let Some(ch) = self.channels.get_mut(channel) {
            ch.broadcast(payload, sender)
        } else {
            Vec::new()
        }
    }

    /// Get channel info.
    pub fn get(&self, name: &str) -> Option<&Channel> {
        self.channels.get(name)
    }

    /// List all channel names.
    pub fn list(&self) -> Vec<String> {
        self.channels.keys().cloned().collect()
    }

    /// Channels a connection belongs to.
    pub fn connection_channels(&self, conn_id: ConnId) -> Vec<String> {
        self.conn_channels
            .get(&conn_id)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default()
    }
}

impl Default for ChannelManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_join_leave() {
        let mut ch = Channel::new("room1".into(), 1000);
        assert!(ch.join(1));
        assert!(ch.join(2));
        assert!(ch.has_member(1));
        assert_eq!(ch.member_count(), 2);

        assert!(ch.leave(1));
        assert!(!ch.has_member(1));
        assert_eq!(ch.member_count(), 1);
    }

    #[test]
    fn test_channel_max_members() {
        let mut ch = Channel::new("small".into(), 1000).with_max_members(2);
        assert!(ch.join(1));
        assert!(ch.join(2));
        assert!(!ch.join(3)); // full
        assert_eq!(ch.member_count(), 2);
    }

    #[test]
    fn test_channel_broadcast() {
        let mut ch = Channel::new("room".into(), 1000);
        ch.join(1);
        ch.join(2);
        ch.join(3);

        let recipients = ch.broadcast(b"hello".to_vec(), Some(1));
        assert!(!recipients.contains(&1)); // sender excluded
        assert_eq!(recipients.len(), 2);
    }

    #[test]
    fn test_channel_history() {
        let mut ch = Channel::new("room".into(), 1000).with_max_history(3);
        ch.join(1);
        ch.broadcast(b"a".to_vec(), None);
        ch.broadcast(b"b".to_vec(), None);
        ch.broadcast(b"c".to_vec(), None);
        ch.broadcast(b"d".to_vec(), None); // evicts "a"

        let hist = ch.history(10);
        assert_eq!(hist.len(), 3);
        assert_eq!(hist[0], b"b");
        assert_eq!(hist[2], b"d");
    }

    #[test]
    fn test_manager_create_join_broadcast() {
        let mut mgr = ChannelManager::new();
        assert!(mgr.create("lobby".into(), 1000));
        assert!(!mgr.create("lobby".into(), 2000)); // duplicate

        assert!(mgr.join("lobby", 1));
        assert!(mgr.join("lobby", 2));

        let recipients = mgr.broadcast("lobby", b"msg".to_vec(), Some(1));
        assert_eq!(recipients, vec![2]);
    }

    #[test]
    fn test_manager_disconnect() {
        let mut mgr = ChannelManager::new();
        mgr.create("a".into(), 1000);
        mgr.create("b".into(), 2000);
        mgr.join("a", 1);
        mgr.join("b", 1);
        mgr.join("a", 2);

        let left = mgr.disconnect(1);
        assert_eq!(left.len(), 2);
        assert_eq!(mgr.get("a").unwrap().member_count(), 1);
        assert_eq!(mgr.get("b").unwrap().member_count(), 0);
    }

    #[test]
    fn test_manager_delete_channel() {
        let mut mgr = ChannelManager::new();
        mgr.create("room".into(), 1000);
        mgr.join("room", 1);
        mgr.join("room", 2);

        let evicted = mgr.delete("room");
        assert_eq!(evicted, 2);
        assert!(mgr.get("room").is_none());
        assert!(mgr.connection_channels(1).is_empty());
    }

    #[test]
    fn test_manager_private_channel() {
        let mut mgr = ChannelManager::new();
        mgr.create_with("secret".into(), 1000, 5, true);
        assert!(mgr.get("secret").unwrap().is_private);
    }
}
