//! Route sharing — share routes, ETAs, and live locations with contacts.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Share visibility level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShareVisibility {
    /// Only visible to specific contacts.
    Private,
    /// Visible to friends/followers.
    Friends,
    /// Visible to anyone with the link.
    LinkOnly,
    /// Publicly visible.
    Public,
}

/// Type of shared content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShareType {
    /// Sharing a saved route.
    Route,
    /// Sharing live location.
    LiveLocation,
    /// Sharing ETA.
    Eta,
    /// Sharing a place/destination.
    Place,
}

/// A shared item.
#[derive(Debug, Clone)]
pub struct SharedItem {
    /// Unique share ID.
    pub id: u64,
    /// Share type.
    pub share_type: ShareType,
    /// Visibility.
    pub visibility: ShareVisibility,
    /// Sharer's user ID.
    pub owner_id: u64,
    /// View count.
    pub view_count: u32,
    /// Share link token.
    pub link_token: String,
    /// Created timestamp.
    pub created_at: Instant,
    /// Expiry duration.
    pub expires_after: Option<Duration>,
    /// Route data (serialised).
    pub data: String,
}

impl SharedItem {
    /// Check if the share has expired.
    pub fn is_expired(&self) -> bool {
        if let Some(ttl) = self.expires_after {
            self.created_at.elapsed() >= ttl
        } else {
            false
        }
    }
}

/// Share management engine.
pub struct ShareManager {
    shares: HashMap<u64, SharedItem>,
    token_index: HashMap<String, u64>,
    next_id: u64,
    token_counter: u64,
}

impl ShareManager {
    /// Create a new share manager.
    pub fn new() -> Self {
        Self {
            shares: HashMap::new(),
            token_index: HashMap::new(),
            next_id: 1,
            token_counter: 0,
        }
    }

    /// Create a new share.
    pub fn create_share(
        &mut self,
        share_type: ShareType,
        visibility: ShareVisibility,
        owner_id: u64,
        data: &str,
        expires_after: Option<Duration>,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.token_counter += 1;
        let token = format!("share_{}", self.token_counter);

        let item = SharedItem {
            id,
            share_type,
            visibility,
            owner_id,
            view_count: 0,
            link_token: token.clone(),
            created_at: Instant::now(),
            expires_after,
            data: data.to_string(),
        };

        self.token_index.insert(token, id);
        self.shares.insert(id, item);
        id
    }

    /// Access a share by token (increments view count).
    pub fn access_by_token(&mut self, token: &str) -> Option<&SharedItem> {
        let id = *self.token_index.get(token)?;
        let item = self.shares.get_mut(&id)?;

        if item.is_expired() {
            return None;
        }

        item.view_count += 1;
        self.shares.get(&id)
    }

    /// Get a share by ID without incrementing view count.
    pub fn get(&self, id: u64) -> Option<&SharedItem> {
        self.shares.get(&id)
    }

    /// Delete a share.
    pub fn delete(&mut self, id: u64, requester_id: u64) -> Result<(), ShareError> {
        let item = self.shares.get(&id).ok_or(ShareError::NotFound)?;
        if item.owner_id != requester_id {
            return Err(ShareError::NotOwner);
        }
        let token = item.link_token.clone();
        self.shares.remove(&id);
        self.token_index.remove(&token);
        Ok(())
    }

    /// List shares owned by a user.
    pub fn user_shares(&self, owner_id: u64) -> Vec<&SharedItem> {
        self.shares
            .values()
            .filter(|s| s.owner_id == owner_id)
            .collect()
    }

    /// Prune expired shares.
    pub fn prune_expired(&mut self) -> usize {
        let expired_ids: Vec<u64> = self
            .shares
            .values()
            .filter(|s| s.is_expired())
            .map(|s| s.id)
            .collect();
        let count = expired_ids.len();
        for id in expired_ids {
            if let Some(item) = self.shares.remove(&id) {
                self.token_index.remove(&item.link_token);
            }
        }
        count
    }

    /// Total number of active shares.
    pub fn total_shares(&self) -> usize {
        self.shares.len()
    }
}

impl Default for ShareManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Share errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShareError {
    /// Share not found.
    NotFound,
    /// Requester is not the owner.
    NotOwner,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_share() {
        let mut mgr = ShareManager::new();
        let id = mgr.create_share(
            ShareType::Route,
            ShareVisibility::Private,
            1,
            "route data",
            None,
        );
        assert_eq!(id, 1);
        assert_eq!(mgr.total_shares(), 1);
    }

    #[test]
    fn test_access_by_token() {
        let mut mgr = ShareManager::new();
        mgr.create_share(ShareType::Route, ShareVisibility::LinkOnly, 1, "data", None);
        let item = mgr.access_by_token("share_1").unwrap();
        assert_eq!(item.view_count, 1);
        assert_eq!(item.data, "data");
    }

    #[test]
    fn test_view_count_increments() {
        let mut mgr = ShareManager::new();
        mgr.create_share(
            ShareType::LiveLocation,
            ShareVisibility::Friends,
            1,
            "loc",
            None,
        );
        mgr.access_by_token("share_1");
        mgr.access_by_token("share_1");
        let item = mgr.get(1).unwrap();
        assert_eq!(item.view_count, 2);
    }

    #[test]
    fn test_delete_share() {
        let mut mgr = ShareManager::new();
        let id = mgr.create_share(ShareType::Eta, ShareVisibility::Private, 1, "eta", None);
        mgr.delete(id, 1).unwrap();
        assert_eq!(mgr.total_shares(), 0);
    }

    #[test]
    fn test_delete_not_owner() {
        let mut mgr = ShareManager::new();
        let id = mgr.create_share(ShareType::Route, ShareVisibility::Private, 1, "data", None);
        let result = mgr.delete(id, 2); // Different user
        assert_eq!(result.unwrap_err(), ShareError::NotOwner);
    }

    #[test]
    fn test_expired_share_not_accessible() {
        let mut mgr = ShareManager::new();
        mgr.create_share(
            ShareType::LiveLocation,
            ShareVisibility::Friends,
            1,
            "loc",
            Some(Duration::from_millis(0)), // Immediate expiry
        );
        std::thread::sleep(Duration::from_millis(1));
        let result = mgr.access_by_token("share_1");
        assert!(result.is_none());
    }

    #[test]
    fn test_user_shares() {
        let mut mgr = ShareManager::new();
        mgr.create_share(ShareType::Route, ShareVisibility::Private, 1, "a", None);
        mgr.create_share(ShareType::Eta, ShareVisibility::Friends, 1, "b", None);
        mgr.create_share(ShareType::Place, ShareVisibility::Public, 2, "c", None);
        assert_eq!(mgr.user_shares(1).len(), 2);
        assert_eq!(mgr.user_shares(2).len(), 1);
    }

    #[test]
    fn test_prune_expired() {
        let mut mgr = ShareManager::new();
        mgr.create_share(
            ShareType::Route,
            ShareVisibility::Private,
            1,
            "a",
            Some(Duration::from_millis(0)),
        );
        mgr.create_share(ShareType::Eta, ShareVisibility::Friends, 1, "b", None); // No expiry
        std::thread::sleep(Duration::from_millis(1));
        let pruned = mgr.prune_expired();
        assert_eq!(pruned, 1);
        assert_eq!(mgr.total_shares(), 1);
    }

    #[test]
    fn test_invalid_token() {
        let mut mgr = ShareManager::new();
        let result = mgr.access_by_token("nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn test_share_not_expired_without_ttl() {
        let mut mgr = ShareManager::new();
        mgr.create_share(ShareType::Route, ShareVisibility::Private, 1, "data", None);
        let item = mgr.get(1).unwrap();
        assert!(!item.is_expired());
    }
}
