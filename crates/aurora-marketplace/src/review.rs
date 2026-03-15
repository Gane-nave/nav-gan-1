//! Review system — user reviews with ratings, helpfulness voting,
//! and moderation support.

use aurora_core::types::EntityId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

// ---------------------------------------------------------------------------
// Review types
// ---------------------------------------------------------------------------

/// A user review of a marketplace listing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub id: EntityId,
    pub listing_id: EntityId,
    pub reviewer_id: EntityId,
    pub reviewer_name: String,
    pub rating: u8,
    pub title: String,
    pub body: String,
    pub helpful_count: u64,
    pub not_helpful_count: u64,
    pub status: ReviewStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Review {
    /// Helpfulness score (helpful / total votes), or 0 if no votes.
    pub fn helpfulness_score(&self) -> f64 {
        let total = self.helpful_count + self.not_helpful_count;
        if total == 0 {
            0.0
        } else {
            self.helpful_count as f64 / total as f64
        }
    }
}

/// Review status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewStatus {
    /// Visible to all users.
    Published,
    /// Flagged for moderation.
    Flagged,
    /// Removed by moderator.
    Removed,
}

// ---------------------------------------------------------------------------
// Review manager
// ---------------------------------------------------------------------------

/// Manages reviews for marketplace listings.
pub struct ReviewManager {
    reviews: HashMap<EntityId, Review>,
    /// listing_id → [review_ids]
    listing_reviews: HashMap<EntityId, Vec<EntityId>>,
    /// reviewer_id → [review_ids] (to enforce one-review-per-listing).
    reviewer_reviews: HashMap<(EntityId, EntityId), EntityId>,
}

impl ReviewManager {
    pub fn new() -> Self {
        Self {
            reviews: HashMap::new(),
            listing_reviews: HashMap::new(),
            reviewer_reviews: HashMap::new(),
        }
    }

    /// Submit a review. Returns error if reviewer already reviewed this listing.
    pub fn submit(
        &mut self,
        listing_id: EntityId,
        reviewer_id: EntityId,
        reviewer_name: impl Into<String>,
        rating: u8,
        title: impl Into<String>,
        body: impl Into<String>,
    ) -> Result<Review, ReviewError> {
        if !(1..=5).contains(&rating) {
            return Err(ReviewError::InvalidRating(rating));
        }

        let key = (reviewer_id, listing_id);
        if self.reviewer_reviews.contains_key(&key) {
            return Err(ReviewError::AlreadyReviewed {
                reviewer_id,
                listing_id,
            });
        }

        let now = Utc::now();
        let review = Review {
            id: EntityId::new(),
            listing_id,
            reviewer_id,
            reviewer_name: reviewer_name.into(),
            rating,
            title: title.into(),
            body: body.into(),
            helpful_count: 0,
            not_helpful_count: 0,
            status: ReviewStatus::Published,
            created_at: now,
            updated_at: now,
        };

        info!(review_id = %review.id, listing_id = %listing_id, rating, "review submitted");

        self.reviewer_reviews.insert(key, review.id);
        self.listing_reviews
            .entry(listing_id)
            .or_default()
            .push(review.id);
        let result = review.clone();
        self.reviews.insert(review.id, review);
        Ok(result)
    }

    /// Vote a review as helpful or not.
    pub fn vote(&mut self, review_id: &EntityId, helpful: bool) -> Result<(), ReviewError> {
        let review = self
            .reviews
            .get_mut(review_id)
            .ok_or(ReviewError::NotFound(*review_id))?;

        if helpful {
            review.helpful_count += 1;
        } else {
            review.not_helpful_count += 1;
        }
        Ok(())
    }

    /// Flag a review for moderation.
    pub fn flag(&mut self, review_id: &EntityId) -> Result<(), ReviewError> {
        let review = self
            .reviews
            .get_mut(review_id)
            .ok_or(ReviewError::NotFound(*review_id))?;
        review.status = ReviewStatus::Flagged;
        Ok(())
    }

    /// Remove a flagged review (moderator action).
    pub fn remove(&mut self, review_id: &EntityId) -> Result<(), ReviewError> {
        let review = self
            .reviews
            .get_mut(review_id)
            .ok_or(ReviewError::NotFound(*review_id))?;
        review.status = ReviewStatus::Removed;
        Ok(())
    }

    /// Get all published reviews for a listing.
    pub fn for_listing(&self, listing_id: &EntityId) -> Vec<&Review> {
        self.listing_reviews
            .get(listing_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.reviews.get(id))
                    .filter(|r| r.status == ReviewStatus::Published)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Average rating for a listing (from published reviews only).
    pub fn average_rating(&self, listing_id: &EntityId) -> f64 {
        let reviews = self.for_listing(listing_id);
        if reviews.is_empty() {
            return 0.0;
        }
        let sum: u64 = reviews.iter().map(|r| r.rating as u64).sum();
        sum as f64 / reviews.len() as f64
    }

    /// Get a review by ID.
    pub fn get(&self, review_id: &EntityId) -> Option<&Review> {
        self.reviews.get(review_id)
    }

    /// Total review count (all statuses).
    pub fn total_reviews(&self) -> usize {
        self.reviews.len()
    }

    /// Get flagged reviews for moderation.
    pub fn flagged_reviews(&self) -> Vec<&Review> {
        self.reviews
            .values()
            .filter(|r| r.status == ReviewStatus::Flagged)
            .collect()
    }
}

impl Default for ReviewManager {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum ReviewError {
    #[error("review not found: {0}")]
    NotFound(EntityId),
    #[error("invalid rating: {0} (must be 1–5)")]
    InvalidRating(u8),
    #[error("reviewer {reviewer_id} already reviewed listing {listing_id}")]
    AlreadyReviewed {
        reviewer_id: EntityId,
        listing_id: EntityId,
    },
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn test_mgr() -> ReviewManager {
        ReviewManager::new()
    }

    fn submit_test_review(mgr: &mut ReviewManager) -> (EntityId, Review) {
        let listing_id = EntityId::new();
        let review = mgr
            .submit(
                listing_id,
                EntityId::new(),
                "Tester",
                4,
                "Good",
                "Works well",
            )
            .unwrap();
        (listing_id, review)
    }

    #[test]
    fn submit_and_retrieve() {
        let mut mgr = test_mgr();
        let (listing_id, review) = submit_test_review(&mut mgr);
        assert_eq!(review.rating, 4);
        assert_eq!(review.status, ReviewStatus::Published);
        assert_eq!(mgr.for_listing(&listing_id).len(), 1);
    }

    #[test]
    fn duplicate_review_rejected() {
        let mut mgr = test_mgr();
        let listing_id = EntityId::new();
        let reviewer_id = EntityId::new();
        mgr.submit(listing_id, reviewer_id, "Tester", 4, "Good", "desc")
            .unwrap();
        let result = mgr.submit(listing_id, reviewer_id, "Tester", 5, "Again", "desc");
        assert!(matches!(
            result.unwrap_err(),
            ReviewError::AlreadyReviewed { .. }
        ));
    }

    #[test]
    fn invalid_rating() {
        let mut mgr = test_mgr();
        assert!(mgr
            .submit(EntityId::new(), EntityId::new(), "T", 0, "t", "b")
            .is_err());
        assert!(mgr
            .submit(EntityId::new(), EntityId::new(), "T", 6, "t", "b")
            .is_err());
    }

    #[test]
    fn voting() {
        let mut mgr = test_mgr();
        let (_, review) = submit_test_review(&mut mgr);
        mgr.vote(&review.id, true).unwrap();
        mgr.vote(&review.id, true).unwrap();
        mgr.vote(&review.id, false).unwrap();
        let r = mgr.get(&review.id).unwrap();
        assert_eq!(r.helpful_count, 2);
        assert_eq!(r.not_helpful_count, 1);
        assert!((r.helpfulness_score() - 2.0 / 3.0).abs() < 0.001);
    }

    #[test]
    fn flag_and_remove() {
        let mut mgr = test_mgr();
        let (listing_id, review) = submit_test_review(&mut mgr);
        mgr.flag(&review.id).unwrap();
        assert_eq!(mgr.flagged_reviews().len(), 1);
        assert_eq!(mgr.for_listing(&listing_id).len(), 0); // flagged = hidden
        mgr.remove(&review.id).unwrap();
        assert_eq!(mgr.flagged_reviews().len(), 0);
    }

    #[test]
    fn average_rating() {
        let mut mgr = test_mgr();
        let listing_id = EntityId::new();
        mgr.submit(listing_id, EntityId::new(), "A", 5, "t", "b")
            .unwrap();
        mgr.submit(listing_id, EntityId::new(), "B", 3, "t", "b")
            .unwrap();
        mgr.submit(listing_id, EntityId::new(), "C", 4, "t", "b")
            .unwrap();
        assert!((mgr.average_rating(&listing_id) - 4.0).abs() < 0.001);
    }

    #[test]
    fn removed_review_excluded_from_average() {
        let mut mgr = test_mgr();
        let listing_id = EntityId::new();
        mgr.submit(listing_id, EntityId::new(), "A", 5, "t", "b")
            .unwrap();
        let bad = mgr
            .submit(listing_id, EntityId::new(), "B", 1, "t", "b")
            .unwrap();
        mgr.remove(&bad.id).unwrap();
        assert!((mgr.average_rating(&listing_id) - 5.0).abs() < 0.001);
    }
}
