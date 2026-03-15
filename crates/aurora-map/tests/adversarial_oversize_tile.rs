//! Adversarial test: oversize tile rejected
//! BUG: When a single tile's size exceeded max_bytes, the eviction loop would
//! drain the cache and still insert the tile, leaving total_bytes > max_bytes permanently.
//! Fix: Early return if tile_size > self.max_bytes.

use aurora_core::map::MapTile;
use aurora_core::types::EntityId;
use aurora_map::tile_manager::*;
use chrono::Utc;

fn make_tile_with_data(zoom: u8, x: u32, y: u32, data_len: usize) -> MapTile {
    MapTile {
        id: EntityId::new(),
        zoom_level: zoom,
        tile_x: x,
        tile_y: y,
        version: 1,
        data_hash: "x".repeat(data_len),
        updated_at: Utc::now(),
    }
}

#[test]
fn oversize_tile_rejected_not_inserted() {
    // Budget = 200 bytes. tile_size = data_hash.len() + 128.
    // A tile with data_hash of 200 chars = 200 + 128 = 328 bytes > 200 budget.
    let mut mgr = TileManager::new(200);

    // First, store a small tile that fits
    let small = make_tile_with_data(14, 1, 1, 10); // 10 + 128 = 138 bytes, fits
    mgr.store_tile(small);
    assert_eq!(mgr.tile_count(), 1, "Small tile should be stored");
    assert_eq!(mgr.storage_used(), 138);

    // Now try to store an oversize tile (200 chars + 128 = 328 > 200 budget)
    let oversize = make_tile_with_data(14, 2, 2, 200);
    mgr.store_tile(oversize);

    // BUG FIX: oversize tile should be rejected, small tile should survive
    assert_eq!(
        mgr.tile_count(),
        1,
        "BUG FIX: Oversize tile rejected. Old code would drain cache and insert it anyway."
    );
    assert!(
        mgr.storage_used() <= 200,
        "BUG FIX: storage_used ({}) must not exceed budget (200). Old code would leave it at 328.",
        mgr.storage_used()
    );

    // Verify the small tile is still there (not evicted by the oversize attempt)
    let key = TileKey {
        zoom: 14,
        x: 1,
        y: 1,
    };
    assert!(
        mgr.get_tile(&key).is_some(),
        "Small tile should survive — oversize tile attempt should not evict it"
    );
}

#[test]
fn oversize_tile_into_empty_cache_stays_empty() {
    let mut mgr = TileManager::new(50);

    // Tile with 0 chars data_hash still costs 128 bytes > 50 budget
    let oversize = make_tile_with_data(14, 1, 1, 0); // 0 + 128 = 128 > 50
    mgr.store_tile(oversize);

    assert_eq!(
        mgr.tile_count(),
        0,
        "Empty cache should stay empty for oversize tile"
    );
    assert_eq!(mgr.storage_used(), 0, "No bytes used");
}
