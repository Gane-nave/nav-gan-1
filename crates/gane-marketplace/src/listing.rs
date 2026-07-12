//! Marketplace listings — plugin/extension entries with metadata,
//! versioning, pricing, and discovery support.

use chrono::{DateTime, Utc};
use gane_core::types::EntityId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

use crate::category::Category;

// ---------------------------------------------------------------------------
// Listing types
// ---------------------------------------------------------------------------

/// A marketplace listing for a plugin or extension.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    pub id: EntityId,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub author_id: EntityId,
    pub author_name: String,
    pub category: Category,
    pub tags: Vec<String>,
    pub versions: Vec<VersionEntry>,
    pub pricing: PricingModel,
    pub status: ListingStatus,
    pub download_count: u64,
    pub rating_sum: f64,
    pub rating_count: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub featured: bool,
}

impl Listing {
    /// Average rating (0.0 if no ratings).
    pub fn average_rating(&self) -> f64 {
        if self.rating_count == 0 {
            0.0
        } else {
            self.rating_sum / self.rating_count as f64
        }
    }

    /// Latest stable version, if any.
    pub fn latest_stable(&self) -> Option<&VersionEntry> {
        self.versions
            .iter()
            .rev()
            .find(|v| v.stability == VersionStability::Stable)
    }

    /// Latest version of any stability.
    pub fn latest_version(&self) -> Option<&VersionEntry> {
        self.versions.last()
    }
}

/// A version entry for a listing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionEntry {
    pub version: String,
    pub changelog: String,
    pub stability: VersionStability,
    pub min_sdk_version: String,
    pub size_bytes: u64,
    pub checksum: String,
    pub published_at: DateTime<Utc>,
}

/// Version stability level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VersionStability {
    /// Production-ready.
    Stable,
    /// Feature-complete, testing in progress.
    Beta,
    /// Early development, may have breaking changes.
    Alpha,
    /// Development snapshot.
    Dev,
}

/// Pricing model for a listing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PricingModel {
    /// Completely free.
    Free,
    /// One-time purchase.
    Paid { price_cents: u64, currency: String },
    /// Recurring subscription.
    Subscription {
        monthly_cents: u64,
        currency: String,
    },
    /// Free with premium features.
    Freemium {
        base_free: bool,
        premium_monthly_cents: u64,
        currency: String,
    },
}

/// Listing status in the marketplace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ListingStatus {
    /// Under review by marketplace team.
    PendingReview,
    /// Published and visible.
    Published,
    /// Temporarily hidden by author.
    Unlisted,
    /// Removed by marketplace for policy violation.
    Suspended,
    /// Archived by author.
    Archived,
}

/// Search/sort criteria for marketplace queries.
#[derive(Debug, Clone)]
pub struct SearchCriteria {
    pub query: Option<String>,
    pub category: Option<Category>,
    pub pricing: Option<PricingFilter>,
    pub min_rating: Option<f64>,
    pub sort_by: SortOrder,
    pub limit: usize,
}

/// Filter by pricing type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PricingFilter {
    Free,
    Paid,
    Any,
}

/// Sort order for search results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Popularity,
    Rating,
    Newest,
    Name,
}

impl Default for SearchCriteria {
    fn default() -> Self {
        Self {
            query: None,
            category: None,
            pricing: None,
            min_rating: None,
            sort_by: SortOrder::Popularity,
            limit: 50,
        }
    }
}

// ---------------------------------------------------------------------------
// Marketplace store
// ---------------------------------------------------------------------------

/// Manages marketplace listings.
pub struct MarketplaceStore {
    listings: HashMap<EntityId, Listing>,
    slug_index: HashMap<String, EntityId>,
}

impl MarketplaceStore {
    pub fn new() -> Self {
        Self {
            listings: HashMap::new(),
            slug_index: HashMap::new(),
        }
    }

    /// Publish a new listing.
    #[allow(clippy::too_many_arguments)]
    pub fn publish(
        &mut self,
        name: impl Into<String>,
        slug: impl Into<String>,
        description: impl Into<String>,
        author_id: EntityId,
        author_name: impl Into<String>,
        category: Category,
        pricing: PricingModel,
    ) -> Result<Listing, MarketplaceError> {
        let slug_str = slug.into();

        if self.slug_index.contains_key(&slug_str) {
            return Err(MarketplaceError::SlugTaken(slug_str));
        }

        let now = Utc::now();
        let listing = Listing {
            id: EntityId::new(),
            name: name.into(),
            slug: slug_str.clone(),
            description: description.into(),
            author_id,
            author_name: author_name.into(),
            category,
            tags: Vec::new(),
            versions: Vec::new(),
            pricing,
            status: ListingStatus::PendingReview,
            download_count: 0,
            rating_sum: 0.0,
            rating_count: 0,
            created_at: now,
            updated_at: now,
            featured: false,
        };

        info!(listing_id = %listing.id, slug = %slug_str, "listing published");
        self.slug_index.insert(slug_str, listing.id);
        let result = listing.clone();
        self.listings.insert(listing.id, listing);
        Ok(result)
    }

    /// Add a version to a listing.
    #[allow(clippy::too_many_arguments)]
    pub fn add_version(
        &mut self,
        listing_id: &EntityId,
        version: impl Into<String>,
        changelog: impl Into<String>,
        stability: VersionStability,
        min_sdk: impl Into<String>,
        size_bytes: u64,
        checksum: impl Into<String>,
    ) -> Result<(), MarketplaceError> {
        let listing = self
            .listings
            .get_mut(listing_id)
            .ok_or(MarketplaceError::NotFound(*listing_id))?;

        let ver_str = version.into();

        // Check for duplicate version.
        if listing.versions.iter().any(|v| v.version == ver_str) {
            return Err(MarketplaceError::VersionExists(ver_str));
        }

        listing.versions.push(VersionEntry {
            version: ver_str,
            changelog: changelog.into(),
            stability,
            min_sdk_version: min_sdk.into(),
            size_bytes,
            checksum: checksum.into(),
            published_at: Utc::now(),
        });
        listing.updated_at = Utc::now();

        Ok(())
    }

    /// Approve a listing (move from PendingReview to Published).
    pub fn approve(&mut self, listing_id: &EntityId) -> Result<(), MarketplaceError> {
        let listing = self
            .listings
            .get_mut(listing_id)
            .ok_or(MarketplaceError::NotFound(*listing_id))?;

        if listing.status != ListingStatus::PendingReview {
            return Err(MarketplaceError::InvalidTransition {
                from: listing.status,
                to: ListingStatus::Published,
            });
        }

        listing.status = ListingStatus::Published;
        listing.updated_at = Utc::now();
        info!(listing_id = %listing_id, "listing approved");
        Ok(())
    }

    /// Suspend a listing.
    pub fn suspend(&mut self, listing_id: &EntityId) -> Result<(), MarketplaceError> {
        let listing = self
            .listings
            .get_mut(listing_id)
            .ok_or(MarketplaceError::NotFound(*listing_id))?;

        listing.status = ListingStatus::Suspended;
        listing.updated_at = Utc::now();
        Ok(())
    }

    /// Record a download.
    pub fn record_download(&mut self, listing_id: &EntityId) -> Result<u64, MarketplaceError> {
        let listing = self
            .listings
            .get_mut(listing_id)
            .ok_or(MarketplaceError::NotFound(*listing_id))?;

        listing.download_count += 1;
        Ok(listing.download_count)
    }

    /// Add a rating to a listing.
    pub fn add_rating(
        &mut self,
        listing_id: &EntityId,
        rating: f64,
    ) -> Result<f64, MarketplaceError> {
        if !(1.0..=5.0).contains(&rating) {
            return Err(MarketplaceError::InvalidRating(rating));
        }

        let listing = self
            .listings
            .get_mut(listing_id)
            .ok_or(MarketplaceError::NotFound(*listing_id))?;

        listing.rating_sum += rating;
        listing.rating_count += 1;
        Ok(listing.average_rating())
    }

    /// Search listings.
    pub fn search(&self, criteria: &SearchCriteria) -> Vec<&Listing> {
        let mut results: Vec<&Listing> = self
            .listings
            .values()
            .filter(|l| l.status == ListingStatus::Published)
            .filter(|l| {
                if let Some(ref q) = criteria.query {
                    let q_lower = q.to_lowercase();
                    l.name.to_lowercase().contains(&q_lower)
                        || l.description.to_lowercase().contains(&q_lower)
                        || l.tags.iter().any(|t| t.to_lowercase().contains(&q_lower))
                } else {
                    true
                }
            })
            .filter(|l| {
                if let Some(cat) = criteria.category {
                    l.category == cat
                } else {
                    true
                }
            })
            .filter(|l| {
                if let Some(filter) = criteria.pricing {
                    match filter {
                        PricingFilter::Free => matches!(l.pricing, PricingModel::Free),
                        PricingFilter::Paid => !matches!(l.pricing, PricingModel::Free),
                        PricingFilter::Any => true,
                    }
                } else {
                    true
                }
            })
            .filter(|l| {
                if let Some(min) = criteria.min_rating {
                    l.average_rating() >= min
                } else {
                    true
                }
            })
            .collect();

        match criteria.sort_by {
            SortOrder::Popularity => results.sort_by_key(|l| std::cmp::Reverse(l.download_count)),
            SortOrder::Rating => results.sort_by(|a, b| {
                b.average_rating()
                    .partial_cmp(&a.average_rating())
                    .unwrap_or(std::cmp::Ordering::Equal)
            }),
            SortOrder::Newest => results.sort_by_key(|l| std::cmp::Reverse(l.created_at)),
            SortOrder::Name => results.sort_by(|a, b| a.name.cmp(&b.name)),
        }

        results.truncate(criteria.limit);
        results
    }

    /// Get a listing by ID.
    pub fn get(&self, listing_id: &EntityId) -> Option<&Listing> {
        self.listings.get(listing_id)
    }

    /// Get a listing by slug.
    pub fn get_by_slug(&self, slug: &str) -> Option<&Listing> {
        self.slug_index
            .get(slug)
            .and_then(|id| self.listings.get(id))
    }

    /// List all listings by an author.
    pub fn by_author(&self, author_id: &EntityId) -> Vec<&Listing> {
        self.listings
            .values()
            .filter(|l| l.author_id == *author_id)
            .collect()
    }

    /// Total published listings.
    pub fn total_published(&self) -> usize {
        self.listings
            .values()
            .filter(|l| l.status == ListingStatus::Published)
            .count()
    }

    /// Set featured status.
    pub fn set_featured(
        &mut self,
        listing_id: &EntityId,
        featured: bool,
    ) -> Result<(), MarketplaceError> {
        let listing = self
            .listings
            .get_mut(listing_id)
            .ok_or(MarketplaceError::NotFound(*listing_id))?;
        listing.featured = featured;
        Ok(())
    }

    /// Get all featured listings.
    pub fn featured(&self) -> Vec<&Listing> {
        self.listings
            .values()
            .filter(|l| l.featured && l.status == ListingStatus::Published)
            .collect()
    }

    /// Add tags to a listing.
    pub fn add_tags(
        &mut self,
        listing_id: &EntityId,
        tags: Vec<String>,
    ) -> Result<(), MarketplaceError> {
        let listing = self
            .listings
            .get_mut(listing_id)
            .ok_or(MarketplaceError::NotFound(*listing_id))?;
        for tag in tags {
            if !listing.tags.contains(&tag) {
                listing.tags.push(tag);
            }
        }
        Ok(())
    }
}

impl Default for MarketplaceStore {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum MarketplaceError {
    #[error("listing not found: {0}")]
    NotFound(EntityId),
    #[error("slug already taken: {0}")]
    SlugTaken(String),
    #[error("version already exists: {0}")]
    VersionExists(String),
    #[error("invalid state transition from {from:?} to {to:?}")]
    InvalidTransition {
        from: ListingStatus,
        to: ListingStatus,
    },
    #[error("invalid rating: {0} (must be 1.0–5.0)")]
    InvalidRating(f64),
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn test_store() -> MarketplaceStore {
        MarketplaceStore::new()
    }

    fn publish_test_listing(store: &mut MarketplaceStore) -> Listing {
        store
            .publish(
                "Nav Plugin",
                "nav-plugin",
                "A great navigation plugin",
                EntityId::new(),
                "Test Author",
                Category::Navigation,
                PricingModel::Free,
            )
            .unwrap()
    }

    #[test]
    fn publish_and_retrieve() {
        let mut store = test_store();
        let listing = publish_test_listing(&mut store);
        assert_eq!(listing.status, ListingStatus::PendingReview);
        assert_eq!(listing.download_count, 0);

        let retrieved = store.get(&listing.id).unwrap();
        assert_eq!(retrieved.name, "Nav Plugin");
        assert_eq!(retrieved.slug, "nav-plugin");
    }

    #[test]
    fn duplicate_slug_rejected() {
        let mut store = test_store();
        publish_test_listing(&mut store);
        let result = store.publish(
            "Another",
            "nav-plugin",
            "desc",
            EntityId::new(),
            "Author",
            Category::Navigation,
            PricingModel::Free,
        );
        assert!(matches!(
            result.unwrap_err(),
            MarketplaceError::SlugTaken(_)
        ));
    }

    #[test]
    fn approve_listing() {
        let mut store = test_store();
        let listing = publish_test_listing(&mut store);
        store.approve(&listing.id).unwrap();
        assert_eq!(
            store.get(&listing.id).unwrap().status,
            ListingStatus::Published
        );
    }

    #[test]
    fn approve_non_pending_fails() {
        let mut store = test_store();
        let listing = publish_test_listing(&mut store);
        store.approve(&listing.id).unwrap();
        let result = store.approve(&listing.id);
        assert!(result.is_err());
    }

    #[test]
    fn add_version() {
        let mut store = test_store();
        let listing = publish_test_listing(&mut store);
        store
            .add_version(
                &listing.id,
                "1.0.0",
                "Initial release",
                VersionStability::Stable,
                "0.1.0",
                1024,
                "abc123",
            )
            .unwrap();

        let l = store.get(&listing.id).unwrap();
        assert_eq!(l.versions.len(), 1);
        assert_eq!(l.versions[0].version, "1.0.0");
    }

    #[test]
    fn duplicate_version_rejected() {
        let mut store = test_store();
        let listing = publish_test_listing(&mut store);
        store
            .add_version(
                &listing.id,
                "1.0.0",
                "v1",
                VersionStability::Stable,
                "0.1.0",
                100,
                "a",
            )
            .unwrap();
        let result = store.add_version(
            &listing.id,
            "1.0.0",
            "v1 dup",
            VersionStability::Stable,
            "0.1.0",
            100,
            "b",
        );
        assert!(matches!(
            result.unwrap_err(),
            MarketplaceError::VersionExists(_)
        ));
    }

    #[test]
    fn latest_stable_version() {
        let mut store = test_store();
        let listing = publish_test_listing(&mut store);
        store
            .add_version(
                &listing.id,
                "0.9.0",
                "beta",
                VersionStability::Beta,
                "0.1.0",
                100,
                "a",
            )
            .unwrap();
        store
            .add_version(
                &listing.id,
                "1.0.0",
                "stable",
                VersionStability::Stable,
                "0.1.0",
                200,
                "b",
            )
            .unwrap();
        store
            .add_version(
                &listing.id,
                "1.1.0-beta",
                "next beta",
                VersionStability::Beta,
                "0.1.0",
                300,
                "c",
            )
            .unwrap();

        let l = store.get(&listing.id).unwrap();
        assert_eq!(l.latest_stable().unwrap().version, "1.0.0");
        assert_eq!(l.latest_version().unwrap().version, "1.1.0-beta");
    }

    #[test]
    fn download_count() {
        let mut store = test_store();
        let listing = publish_test_listing(&mut store);
        assert_eq!(store.record_download(&listing.id).unwrap(), 1);
        assert_eq!(store.record_download(&listing.id).unwrap(), 2);
        assert_eq!(store.get(&listing.id).unwrap().download_count, 2);
    }

    #[test]
    fn rating_system() {
        let mut store = test_store();
        let listing = publish_test_listing(&mut store);
        store.add_rating(&listing.id, 5.0).unwrap();
        store.add_rating(&listing.id, 3.0).unwrap();
        let avg = store.add_rating(&listing.id, 4.0).unwrap();
        assert!((avg - 4.0).abs() < 0.001);
    }

    #[test]
    fn invalid_rating_rejected() {
        let mut store = test_store();
        let listing = publish_test_listing(&mut store);
        assert!(store.add_rating(&listing.id, 0.5).is_err());
        assert!(store.add_rating(&listing.id, 5.1).is_err());
    }

    #[test]
    fn search_by_query() {
        let mut store = test_store();
        let l = publish_test_listing(&mut store);
        store.approve(&l.id).unwrap();

        let l2 = store
            .publish(
                "Traffic Monitor",
                "traffic-monitor",
                "Monitors traffic",
                EntityId::new(),
                "Auth",
                Category::Traffic,
                PricingModel::Free,
            )
            .unwrap();
        store.approve(&l2.id).unwrap();

        let criteria = SearchCriteria {
            query: Some("nav".into()),
            ..Default::default()
        };
        let results = store.search(&criteria);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Nav Plugin");
    }

    #[test]
    fn search_sort_by_popularity() {
        let mut store = test_store();
        let l1 = publish_test_listing(&mut store);
        store.approve(&l1.id).unwrap();
        store.record_download(&l1.id).unwrap();

        let l2 = store
            .publish(
                "Popular Plugin",
                "popular",
                "desc",
                EntityId::new(),
                "Auth",
                Category::Navigation,
                PricingModel::Free,
            )
            .unwrap();
        store.approve(&l2.id).unwrap();
        store.record_download(&l2.id).unwrap();
        store.record_download(&l2.id).unwrap();
        store.record_download(&l2.id).unwrap();

        let criteria = SearchCriteria {
            sort_by: SortOrder::Popularity,
            ..Default::default()
        };
        let results = store.search(&criteria);
        assert_eq!(results[0].name, "Popular Plugin");
    }

    #[test]
    fn get_by_slug() {
        let mut store = test_store();
        publish_test_listing(&mut store);
        let l = store.get_by_slug("nav-plugin").unwrap();
        assert_eq!(l.name, "Nav Plugin");
        assert!(store.get_by_slug("nonexistent").is_none());
    }

    #[test]
    fn featured_listings() {
        let mut store = test_store();
        let l = publish_test_listing(&mut store);
        store.approve(&l.id).unwrap();
        store.set_featured(&l.id, true).unwrap();

        assert_eq!(store.featured().len(), 1);
        store.set_featured(&l.id, false).unwrap();
        assert_eq!(store.featured().len(), 0);
    }

    #[test]
    fn suspend_listing() {
        let mut store = test_store();
        let l = publish_test_listing(&mut store);
        store.approve(&l.id).unwrap();
        store.suspend(&l.id).unwrap();
        assert_eq!(store.get(&l.id).unwrap().status, ListingStatus::Suspended);
        assert_eq!(store.total_published(), 0);
    }

    #[test]
    fn tags() {
        let mut store = test_store();
        let l = publish_test_listing(&mut store);
        store
            .add_tags(&l.id, vec!["gnss".into(), "navigation".into()])
            .unwrap();
        store
            .add_tags(&l.id, vec!["gnss".into(), "routing".into()])
            .unwrap(); // gnss dedup
        assert_eq!(store.get(&l.id).unwrap().tags.len(), 3);
    }
}
