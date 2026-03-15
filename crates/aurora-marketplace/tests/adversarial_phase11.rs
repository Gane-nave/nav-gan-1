//! Adversarial integration tests for Phase 11 marketplace features.

use aurora_core::types::EntityId;
use aurora_marketplace::category::Category;
use aurora_marketplace::listing::{
    ListingStatus, MarketplaceStore, PricingModel, SearchCriteria, VersionStability,
};

#[test]
fn adversarial_marketplace_full_lifecycle() {
    let mut store = MarketplaceStore::new();
    let author = EntityId::new();

    // 1. Publish → PendingReview
    let listing = store
        .publish(
            "Traffic Plugin",
            "traffic-plugin",
            "Real-time overlay",
            author,
            "TestAuthor",
            Category::Traffic,
            PricingModel::Free,
        )
        .unwrap();
    assert_eq!(
        listing.status,
        ListingStatus::PendingReview,
        "new listing must be PendingReview"
    );

    // 2. Approve → Published
    store.approve(&listing.id).unwrap();
    assert_eq!(
        store.get(&listing.id).unwrap().status,
        ListingStatus::Published
    );

    // 3. Search by keyword finds it
    let results = store.search(&SearchCriteria {
        query: Some("traffic".into()),
        ..Default::default()
    });
    assert_eq!(results.len(), 1, "search must find 1 result");
    assert_eq!(results[0].name, "Traffic Plugin");

    // 4. Add version (version, changelog, stability, min_sdk, size_bytes, checksum)
    store
        .add_version(
            &listing.id,
            "1.0.0",
            "Initial release",
            VersionStability::Stable,
            "0.1.0",
            1024,
            "sha256:abc",
        )
        .unwrap();
    assert_eq!(
        store.get(&listing.id).unwrap().versions.len(),
        1,
        "must have 1 version"
    );

    // 5. Search with no query still returns published listing
    let all = store.search(&SearchCriteria::default());
    assert!(!all.is_empty(), "default search returns published listings");
}
