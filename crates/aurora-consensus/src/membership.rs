//! Cluster membership management — join, leave, and health tracking.

use std::collections::HashMap;

/// Member state in the cluster.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemberState {
    /// Joining the cluster.
    Joining,
    /// Active and healthy.
    Active,
    /// Suspected to be down (missed heartbeats).
    Suspect,
    /// Confirmed down.
    Down,
    /// Gracefully leaving.
    Leaving,
    /// Left the cluster.
    Left,
}

/// A cluster member.
#[derive(Debug, Clone)]
pub struct ClusterMember {
    /// Member ID.
    pub id: u64,
    /// Human-readable name.
    pub name: String,
    /// Address (e.g., IP:port).
    pub address: String,
    /// Current state.
    pub state: MemberState,
    /// When the member joined (epoch millis).
    pub joined_ms: u64,
    /// Last heartbeat received (epoch millis).
    pub last_heartbeat_ms: u64,
    /// Metadata.
    pub metadata: HashMap<String, String>,
    /// Number of missed heartbeats.
    pub missed_heartbeats: u32,
}

/// Cluster membership manager.
pub struct ClusterManager {
    /// All members.
    members: HashMap<u64, ClusterMember>,
    /// Local member ID.
    local_id: u64,
    /// Heartbeat interval in millis.
    heartbeat_interval_ms: u64,
    /// Number of missed heartbeats before suspect.
    suspect_threshold: u32,
    /// Number of missed heartbeats before down.
    down_threshold: u32,
    /// Generation counter (incremented on membership changes).
    generation: u64,
}

impl ClusterManager {
    /// Create a new cluster manager.
    pub fn new(
        local_id: u64,
        local_name: &str,
        local_address: &str,
        heartbeat_interval_ms: u64,
    ) -> Self {
        let mut members = HashMap::new();
        members.insert(
            local_id,
            ClusterMember {
                id: local_id,
                name: local_name.to_string(),
                address: local_address.to_string(),
                state: MemberState::Active,
                joined_ms: 0,
                last_heartbeat_ms: 0,
                metadata: HashMap::new(),
                missed_heartbeats: 0,
            },
        );

        Self {
            members,
            local_id,
            heartbeat_interval_ms,
            suspect_threshold: 3,
            down_threshold: 5,
            generation: 1,
        }
    }

    /// Add a member to the cluster.
    pub fn join(&mut self, id: u64, name: &str, address: &str, now_ms: u64) -> bool {
        if self.members.contains_key(&id) {
            return false;
        }
        self.members.insert(
            id,
            ClusterMember {
                id,
                name: name.to_string(),
                address: address.to_string(),
                state: MemberState::Joining,
                joined_ms: now_ms,
                last_heartbeat_ms: now_ms,
                metadata: HashMap::new(),
                missed_heartbeats: 0,
            },
        );
        self.generation += 1;
        true
    }

    /// Mark a member as active (after successful join handshake).
    pub fn activate(&mut self, id: u64) -> bool {
        if let Some(member) = self.members.get_mut(&id) {
            if member.state == MemberState::Joining {
                member.state = MemberState::Active;
                self.generation += 1;
                return true;
            }
        }
        false
    }

    /// Gracefully remove a member.
    pub fn leave(&mut self, id: u64) -> bool {
        if let Some(member) = self.members.get_mut(&id) {
            member.state = MemberState::Left;
            self.generation += 1;
            return true;
        }
        false
    }

    /// Record a heartbeat from a member.
    pub fn heartbeat(&mut self, id: u64, now_ms: u64) {
        if let Some(member) = self.members.get_mut(&id) {
            member.last_heartbeat_ms = now_ms;
            member.missed_heartbeats = 0;
            if member.state == MemberState::Suspect {
                member.state = MemberState::Active;
            }
        }
    }

    /// Check for failed members based on heartbeat timing.
    pub fn check_health(&mut self, now_ms: u64) {
        let interval = self.heartbeat_interval_ms;
        let suspect_thresh = self.suspect_threshold;
        let down_thresh = self.down_threshold;
        let local_id = self.local_id;
        let mut changed = false;

        for member in self.members.values_mut() {
            if member.id == local_id {
                continue;
            }
            if member.state == MemberState::Left || member.state == MemberState::Down {
                continue;
            }

            if member.last_heartbeat_ms > 0 {
                let elapsed = now_ms.saturating_sub(member.last_heartbeat_ms);
                let missed = (elapsed / interval.max(1)) as u32;
                member.missed_heartbeats = missed;

                if missed >= down_thresh && member.state != MemberState::Down {
                    member.state = MemberState::Down;
                    changed = true;
                } else if missed >= suspect_thresh && member.state == MemberState::Active {
                    member.state = MemberState::Suspect;
                    changed = true;
                }
            }
        }

        if changed {
            self.generation += 1;
        }
    }

    /// Get a member by ID.
    pub fn get_member(&self, id: u64) -> Option<&ClusterMember> {
        self.members.get(&id)
    }

    /// Get all active members.
    pub fn active_members(&self) -> Vec<&ClusterMember> {
        self.members
            .values()
            .filter(|m| m.state == MemberState::Active)
            .collect()
    }

    /// Total member count (all states).
    pub fn total_count(&self) -> usize {
        self.members.len()
    }

    /// Active member count.
    pub fn active_count(&self) -> usize {
        self.members
            .values()
            .filter(|m| m.state == MemberState::Active)
            .count()
    }

    /// Current generation.
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Remove members that have left or are down.
    pub fn prune_dead(&mut self) -> usize {
        let before = self.members.len();
        self.members.retain(|id, m| {
            *id == self.local_id || (m.state != MemberState::Left && m.state != MemberState::Down)
        });
        let removed = before - self.members.len();
        if removed > 0 {
            self.generation += 1;
        }
        removed
    }

    /// Get all member IDs.
    pub fn member_ids(&self) -> Vec<u64> {
        self.members.keys().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_and_activate() {
        let mut mgr = ClusterManager::new(1, "node-1", "10.0.0.1:8000", 1000);
        assert!(mgr.join(2, "node-2", "10.0.0.2:8000", 1000));
        assert_eq!(mgr.total_count(), 2);
        assert_eq!(mgr.active_count(), 1); // node-2 is still Joining

        mgr.activate(2);
        assert_eq!(mgr.active_count(), 2);
    }

    #[test]
    fn test_duplicate_join() {
        let mut mgr = ClusterManager::new(1, "node-1", "10.0.0.1:8000", 1000);
        assert!(mgr.join(2, "node-2", "10.0.0.2:8000", 1000));
        assert!(!mgr.join(2, "node-2", "10.0.0.2:8000", 2000)); // duplicate
    }

    #[test]
    fn test_leave() {
        let mut mgr = ClusterManager::new(1, "node-1", "10.0.0.1:8000", 1000);
        mgr.join(2, "node-2", "10.0.0.2:8000", 1000);
        mgr.activate(2);
        assert!(mgr.leave(2));
        assert_eq!(mgr.get_member(2).unwrap().state, MemberState::Left);
    }

    #[test]
    fn test_heartbeat_recovery() {
        let mut mgr = ClusterManager::new(1, "node-1", "10.0.0.1:8000", 1000);
        mgr.join(2, "node-2", "10.0.0.2:8000", 1000);
        mgr.activate(2);
        mgr.heartbeat(2, 1000);

        // Miss heartbeats -> Suspect
        mgr.check_health(5000); // 4 missed
        assert_eq!(mgr.get_member(2).unwrap().state, MemberState::Suspect);

        // Heartbeat recovers
        mgr.heartbeat(2, 5000);
        assert_eq!(mgr.get_member(2).unwrap().state, MemberState::Active);
    }

    #[test]
    fn test_down_detection() {
        let mut mgr = ClusterManager::new(1, "node-1", "10.0.0.1:8000", 1000);
        mgr.join(2, "node-2", "10.0.0.2:8000", 1000);
        mgr.activate(2);
        mgr.heartbeat(2, 1000);

        mgr.check_health(7000); // 6 missed, > down_threshold(5)
        assert_eq!(mgr.get_member(2).unwrap().state, MemberState::Down);
    }

    #[test]
    fn test_prune_dead() {
        let mut mgr = ClusterManager::new(1, "node-1", "10.0.0.1:8000", 1000);
        mgr.join(2, "node-2", "10.0.0.2:8000", 1000);
        mgr.activate(2);
        mgr.leave(2);

        let pruned = mgr.prune_dead();
        assert_eq!(pruned, 1);
        assert_eq!(mgr.total_count(), 1); // Only local node remains
    }

    #[test]
    fn test_generation_increments() {
        let mut mgr = ClusterManager::new(1, "node-1", "10.0.0.1:8000", 1000);
        let g1 = mgr.generation();
        mgr.join(2, "node-2", "10.0.0.2:8000", 1000);
        assert!(mgr.generation() > g1);
    }

    #[test]
    fn test_active_members_list() {
        let mut mgr = ClusterManager::new(1, "node-1", "10.0.0.1:8000", 1000);
        mgr.join(2, "node-2", "10.0.0.2:8000", 1000);
        mgr.activate(2);
        mgr.join(3, "node-3", "10.0.0.3:8000", 1000);
        // node-3 still Joining

        let active = mgr.active_members();
        assert_eq!(active.len(), 2); // node-1, node-2
    }

    #[test]
    fn test_local_node_not_pruned() {
        let mut mgr = ClusterManager::new(1, "node-1", "10.0.0.1:8000", 1000);
        mgr.prune_dead();
        assert_eq!(mgr.total_count(), 1); // Local node always stays
    }
}
