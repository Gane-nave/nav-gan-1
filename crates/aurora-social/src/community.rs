//! Community features — groups, challenges, leaderboards, and collaborative map editing.

use std::collections::{HashMap, HashSet};

/// Community group type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupType {
    /// Local neighbourhood group.
    Neighbourhood,
    /// Commuter group (shared routes).
    Commuter,
    /// Fleet or organisation.
    Fleet,
    /// Interest-based (cycling, EV owners, etc.).
    Interest,
    /// Event-based (temporary).
    Event,
}

/// A community group.
#[derive(Debug, Clone)]
pub struct CommunityGroup {
    /// Group ID.
    pub id: u64,
    /// Group name.
    pub name: String,
    /// Group type.
    pub group_type: GroupType,
    /// Creator user ID.
    pub creator_id: u64,
    /// Member user IDs.
    pub members: HashSet<u64>,
    /// Maximum members allowed.
    pub max_members: usize,
    /// Whether the group is public.
    pub is_public: bool,
}

impl CommunityGroup {
    /// Check if the group is full.
    pub fn is_full(&self) -> bool {
        self.members.len() >= self.max_members
    }

    /// Get the member count.
    pub fn member_count(&self) -> usize {
        self.members.len()
    }
}

/// A community challenge.
#[derive(Debug, Clone)]
pub struct Challenge {
    /// Challenge ID.
    pub id: u64,
    /// Challenge name.
    pub name: String,
    /// Description.
    pub description: String,
    /// Target metric value.
    pub target_value: f64,
    /// Current participants and their progress.
    pub participants: HashMap<u64, f64>,
}

impl Challenge {
    /// Add a participant.
    pub fn join(&mut self, user_id: u64) {
        self.participants.entry(user_id).or_insert(0.0);
    }

    /// Update progress for a participant.
    pub fn update_progress(&mut self, user_id: u64, value: f64) -> Option<f64> {
        let progress = self.participants.get_mut(&user_id)?;
        *progress = value;
        Some(*progress)
    }

    /// Check if a participant has completed the challenge.
    pub fn is_completed(&self, user_id: u64) -> bool {
        self.participants
            .get(&user_id)
            .is_some_and(|&p| p >= self.target_value)
    }

    /// Get the leaderboard (sorted by progress descending).
    pub fn leaderboard(&self) -> Vec<(u64, f64)> {
        let mut entries: Vec<(u64, f64)> =
            self.participants.iter().map(|(&k, &v)| (k, v)).collect();
        entries.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        entries
    }

    /// Get participant count.
    pub fn participant_count(&self) -> usize {
        self.participants.len()
    }
}

/// Community management engine.
pub struct CommunityManager {
    groups: HashMap<u64, CommunityGroup>,
    challenges: HashMap<u64, Challenge>,
    next_group_id: u64,
    next_challenge_id: u64,
}

impl CommunityManager {
    /// Create a new community manager.
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
            challenges: HashMap::new(),
            next_group_id: 1,
            next_challenge_id: 1,
        }
    }

    /// Create a new group.
    pub fn create_group(
        &mut self,
        name: &str,
        group_type: GroupType,
        creator_id: u64,
        max_members: usize,
        is_public: bool,
    ) -> u64 {
        let id = self.next_group_id;
        self.next_group_id += 1;

        let mut members = HashSet::new();
        members.insert(creator_id);

        self.groups.insert(
            id,
            CommunityGroup {
                id,
                name: name.to_string(),
                group_type,
                creator_id,
                members,
                max_members,
                is_public,
            },
        );
        id
    }

    /// Join a group.
    pub fn join_group(&mut self, group_id: u64, user_id: u64) -> Result<(), CommunityError> {
        let group = self
            .groups
            .get_mut(&group_id)
            .ok_or(CommunityError::NotFound)?;

        if !group.is_public {
            return Err(CommunityError::PrivateGroup);
        }

        if group.is_full() {
            return Err(CommunityError::GroupFull);
        }

        if !group.members.insert(user_id) {
            return Err(CommunityError::AlreadyMember);
        }

        Ok(())
    }

    /// Leave a group.
    pub fn leave_group(&mut self, group_id: u64, user_id: u64) -> Result<(), CommunityError> {
        let group = self
            .groups
            .get_mut(&group_id)
            .ok_or(CommunityError::NotFound)?;

        if user_id == group.creator_id {
            return Err(CommunityError::CannotLeaveAsCreator);
        }

        if !group.members.remove(&user_id) {
            return Err(CommunityError::NotMember);
        }

        Ok(())
    }

    /// Get a group.
    pub fn get_group(&self, group_id: u64) -> Option<&CommunityGroup> {
        self.groups.get(&group_id)
    }

    /// Get total number of groups.
    pub fn group_count(&self) -> usize {
        self.groups.len()
    }

    /// Create a new challenge.
    pub fn create_challenge(&mut self, name: &str, description: &str, target: f64) -> u64 {
        let id = self.next_challenge_id;
        self.next_challenge_id += 1;

        self.challenges.insert(
            id,
            Challenge {
                id,
                name: name.to_string(),
                description: description.to_string(),
                target_value: target,
                participants: HashMap::new(),
            },
        );
        id
    }

    /// Get a challenge.
    pub fn get_challenge(&self, challenge_id: u64) -> Option<&Challenge> {
        self.challenges.get(&challenge_id)
    }

    /// Get a mutable reference to a challenge.
    pub fn get_challenge_mut(&mut self, challenge_id: u64) -> Option<&mut Challenge> {
        self.challenges.get_mut(&challenge_id)
    }

    /// Get total number of challenges.
    pub fn challenge_count(&self) -> usize {
        self.challenges.len()
    }
}

impl Default for CommunityManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Community errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommunityError {
    /// Group or challenge not found.
    NotFound,
    /// Group is private.
    PrivateGroup,
    /// Group is full.
    GroupFull,
    /// User is already a member.
    AlreadyMember,
    /// User is not a member.
    NotMember,
    /// Creator cannot leave their own group.
    CannotLeaveAsCreator,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_group() {
        let mut mgr = CommunityManager::new();
        let id = mgr.create_group("Tel Aviv Commuters", GroupType::Commuter, 1, 100, true);
        assert_eq!(id, 1);
        let group = mgr.get_group(id).unwrap();
        assert_eq!(group.member_count(), 1); // Creator auto-joined
        assert_eq!(group.creator_id, 1);
    }

    #[test]
    fn test_join_group() {
        let mut mgr = CommunityManager::new();
        let id = mgr.create_group("Cyclists", GroupType::Interest, 1, 50, true);
        mgr.join_group(id, 2).unwrap();
        assert_eq!(mgr.get_group(id).unwrap().member_count(), 2);
    }

    #[test]
    fn test_join_private_group_fails() {
        let mut mgr = CommunityManager::new();
        let id = mgr.create_group("Private Fleet", GroupType::Fleet, 1, 50, false);
        let result = mgr.join_group(id, 2);
        assert_eq!(result.unwrap_err(), CommunityError::PrivateGroup);
    }

    #[test]
    fn test_join_full_group_fails() {
        let mut mgr = CommunityManager::new();
        let id = mgr.create_group("Tiny Group", GroupType::Event, 1, 2, true);
        mgr.join_group(id, 2).unwrap(); // Now full (creator + 1)
        let result = mgr.join_group(id, 3);
        assert_eq!(result.unwrap_err(), CommunityError::GroupFull);
    }

    #[test]
    fn test_duplicate_join_fails() {
        let mut mgr = CommunityManager::new();
        let id = mgr.create_group("Group", GroupType::Neighbourhood, 1, 100, true);
        mgr.join_group(id, 2).unwrap();
        let result = mgr.join_group(id, 2);
        assert_eq!(result.unwrap_err(), CommunityError::AlreadyMember);
    }

    #[test]
    fn test_leave_group() {
        let mut mgr = CommunityManager::new();
        let id = mgr.create_group("Group", GroupType::Commuter, 1, 100, true);
        mgr.join_group(id, 2).unwrap();
        mgr.leave_group(id, 2).unwrap();
        assert_eq!(mgr.get_group(id).unwrap().member_count(), 1);
    }

    #[test]
    fn test_creator_cannot_leave() {
        let mut mgr = CommunityManager::new();
        let id = mgr.create_group("Group", GroupType::Commuter, 1, 100, true);
        let result = mgr.leave_group(id, 1);
        assert_eq!(result.unwrap_err(), CommunityError::CannotLeaveAsCreator);
    }

    #[test]
    fn test_challenge_lifecycle() {
        let mut mgr = CommunityManager::new();
        let id = mgr.create_challenge("Drive 100km", "Total driving distance", 100.0);
        let challenge = mgr.get_challenge_mut(id).unwrap();

        challenge.join(1);
        challenge.join(2);
        assert_eq!(challenge.participant_count(), 2);

        challenge.update_progress(1, 50.0);
        assert!(!challenge.is_completed(1));

        challenge.update_progress(1, 100.0);
        assert!(challenge.is_completed(1));
        assert!(!challenge.is_completed(2));
    }

    #[test]
    fn test_challenge_leaderboard() {
        let mut mgr = CommunityManager::new();
        let id = mgr.create_challenge("Eco Drive", "Fuel efficiency score", 100.0);
        let challenge = mgr.get_challenge_mut(id).unwrap();

        challenge.join(1);
        challenge.join(2);
        challenge.join(3);
        challenge.update_progress(1, 30.0);
        challenge.update_progress(2, 80.0);
        challenge.update_progress(3, 50.0);

        let lb = challenge.leaderboard();
        assert_eq!(lb[0].0, 2); // User 2 is first with 80
        assert_eq!(lb[1].0, 3); // User 3 second with 50
        assert_eq!(lb[2].0, 1); // User 1 third with 30
    }

    #[test]
    fn test_group_count() {
        let mut mgr = CommunityManager::new();
        mgr.create_group("A", GroupType::Commuter, 1, 100, true);
        mgr.create_group("B", GroupType::Interest, 2, 100, true);
        assert_eq!(mgr.group_count(), 2);
    }

    #[test]
    fn test_not_found_error() {
        let mut mgr = CommunityManager::new();
        let result = mgr.join_group(999, 1);
        assert_eq!(result.unwrap_err(), CommunityError::NotFound);
    }
}
