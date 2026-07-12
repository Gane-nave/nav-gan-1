//! Leader election — bully algorithm for coordinator selection.

use std::collections::HashMap;

/// Node identifier in the cluster.
pub type MemberId = u64;

/// Election state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElectionState {
    /// No election in progress; leader is known.
    Stable,
    /// Election is in progress.
    Electing,
    /// This node is the leader.
    Leader,
    /// No leader (cluster is forming).
    NoLeader,
}

/// A participant in leader election.
#[derive(Debug, Clone)]
pub struct ElectionNode {
    /// This node's ID.
    pub id: MemberId,
    /// Priority (higher = more likely to be leader).
    pub priority: u64,
    /// Whether the node is alive.
    pub alive: bool,
    /// Last heartbeat timestamp (epoch millis).
    pub last_heartbeat_ms: u64,
}

/// Leader election coordinator.
pub struct LeaderElection {
    /// Local node ID.
    local_id: MemberId,
    /// Known nodes.
    nodes: HashMap<MemberId, ElectionNode>,
    /// Current leader (if any).
    current_leader: Option<MemberId>,
    /// Current state.
    state: ElectionState,
    /// Heartbeat timeout in millis.
    heartbeat_timeout_ms: u64,
    /// Term/epoch counter.
    term: u64,
    /// Election count.
    elections_held: u64,
}

impl LeaderElection {
    /// Create a new election instance for a local node.
    pub fn new(local_id: MemberId, priority: u64, heartbeat_timeout_ms: u64) -> Self {
        let mut nodes = HashMap::new();
        nodes.insert(
            local_id,
            ElectionNode {
                id: local_id,
                priority,
                alive: true,
                last_heartbeat_ms: 0,
            },
        );

        Self {
            local_id,
            nodes,
            current_leader: None,
            state: ElectionState::NoLeader,
            heartbeat_timeout_ms,
            term: 0,
            elections_held: 0,
        }
    }

    /// Register a peer node.
    pub fn add_peer(&mut self, id: MemberId, priority: u64) {
        self.nodes.insert(
            id,
            ElectionNode {
                id,
                priority,
                alive: true,
                last_heartbeat_ms: 0,
            },
        );
    }

    /// Record a heartbeat from a node.
    pub fn heartbeat(&mut self, from: MemberId, now_ms: u64) {
        if let Some(node) = self.nodes.get_mut(&from) {
            node.alive = true;
            node.last_heartbeat_ms = now_ms;
        }
    }

    /// Check for expired heartbeats and trigger election if leader is down.
    pub fn check_health(&mut self, now_ms: u64) {
        // Mark nodes with expired heartbeats as dead
        for node in self.nodes.values_mut() {
            if node.id == self.local_id {
                continue; // Local node is always alive
            }
            if node.last_heartbeat_ms > 0
                && now_ms.saturating_sub(node.last_heartbeat_ms) > self.heartbeat_timeout_ms
            {
                node.alive = false;
            }
        }

        // If current leader is dead, start election
        if let Some(leader) = self.current_leader {
            if let Some(node) = self.nodes.get(&leader) {
                if !node.alive {
                    self.start_election();
                }
            }
        }
    }

    /// Start a new election.
    pub fn start_election(&mut self) {
        self.state = ElectionState::Electing;
        self.term += 1;
        self.elections_held += 1;

        // Bully algorithm: highest priority alive node wins
        let winner = self
            .nodes
            .values()
            .filter(|n| n.alive)
            .max_by_key(|n| (n.priority, n.id))
            .map(|n| n.id);

        if let Some(winner_id) = winner {
            self.current_leader = Some(winner_id);
            if winner_id == self.local_id {
                self.state = ElectionState::Leader;
            } else {
                self.state = ElectionState::Stable;
            }
        } else {
            self.current_leader = None;
            self.state = ElectionState::NoLeader;
        }
    }

    /// Get current leader.
    pub fn leader(&self) -> Option<MemberId> {
        self.current_leader
    }

    /// Check if the local node is leader.
    pub fn is_leader(&self) -> bool {
        self.current_leader == Some(self.local_id)
    }

    /// Current election state.
    pub fn state(&self) -> ElectionState {
        self.state
    }

    /// Current term.
    pub fn term(&self) -> u64 {
        self.term
    }

    /// Number of elections held.
    pub fn elections_held(&self) -> u64 {
        self.elections_held
    }

    /// Number of alive nodes.
    pub fn alive_count(&self) -> usize {
        self.nodes.values().filter(|n| n.alive).count()
    }

    /// Total known nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Remove a node from the cluster.
    pub fn remove_node(&mut self, id: MemberId) -> bool {
        let removed = self.nodes.remove(&id).is_some();
        if self.current_leader == Some(id) {
            self.current_leader = None;
            self.state = ElectionState::NoLeader;
        }
        removed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_election_single_node() {
        let mut election = LeaderElection::new(1, 100, 5000);
        election.start_election();
        assert!(election.is_leader());
        assert_eq!(election.leader(), Some(1));
        assert_eq!(election.state(), ElectionState::Leader);
    }

    #[test]
    fn test_election_highest_priority_wins() {
        let mut election = LeaderElection::new(1, 50, 5000);
        election.add_peer(2, 100);
        election.add_peer(3, 75);
        election.heartbeat(2, 1000);
        election.heartbeat(3, 1000);
        election.start_election();
        assert_eq!(election.leader(), Some(2));
        assert!(!election.is_leader());
    }

    #[test]
    fn test_heartbeat_timeout() {
        let mut election = LeaderElection::new(1, 50, 5000);
        election.add_peer(2, 100);
        election.heartbeat(2, 1000);
        election.start_election();
        assert_eq!(election.leader(), Some(2));

        // Node 2 heartbeat expires
        election.check_health(7000);
        // Leader should be gone, new election triggered
        assert_eq!(election.leader(), Some(1)); // Only alive node
    }

    #[test]
    fn test_term_increments() {
        let mut election = LeaderElection::new(1, 100, 5000);
        assert_eq!(election.term(), 0);
        election.start_election();
        assert_eq!(election.term(), 1);
        election.start_election();
        assert_eq!(election.term(), 2);
    }

    #[test]
    fn test_remove_leader_triggers_no_leader() {
        let mut election = LeaderElection::new(1, 50, 5000);
        election.add_peer(2, 100);
        election.heartbeat(2, 1000);
        election.start_election();
        assert_eq!(election.leader(), Some(2));

        election.remove_node(2);
        assert_eq!(election.state(), ElectionState::NoLeader);
        assert!(election.leader().is_none());
    }

    #[test]
    fn test_alive_count() {
        let mut election = LeaderElection::new(1, 100, 5000);
        election.add_peer(2, 50);
        election.add_peer(3, 75);
        election.heartbeat(2, 1000);
        election.heartbeat(3, 1000);
        assert_eq!(election.alive_count(), 3);

        election.check_health(7000); // 2 and 3 expire
        assert_eq!(election.alive_count(), 1); // only local
    }

    #[test]
    fn test_elections_held_counter() {
        let mut election = LeaderElection::new(1, 100, 5000);
        assert_eq!(election.elections_held(), 0);
        election.start_election();
        election.start_election();
        assert_eq!(election.elections_held(), 2);
    }

    #[test]
    fn test_peer_count() {
        let mut election = LeaderElection::new(1, 100, 5000);
        assert_eq!(election.node_count(), 1);
        election.add_peer(2, 50);
        assert_eq!(election.node_count(), 2);
    }
}
