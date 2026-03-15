//! Tile manager — manages map tile lifecycle, caching, and versioning.

use aurora_core::map::MapTile;
use std::collections::HashMap;
use tracing::{debug, info};

/// Key for a tile in the tile store.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileKey {
    pub zoom: u8,
    pub x: u32,
    pub y: u32,
}

/// Tile download/update status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileStatus {
    /// Tile is current.
    Current,
    /// Tile is stale (newer version available).
    Stale,
    /// Tile is not yet downloaded.
    Missing,
    /// Tile download is in progress.
    Downloading,
}

/// Manages map tile storage, versioning, and differential updates.
pub struct TileManager {
    /// In-memory tile store.
    tiles: HashMap<TileKey, MapTile>,
    /// Known latest versions per tile.
    latest_versions: HashMap<TileKey, u64>,
    /// Total bytes used (approximate).
    total_bytes: u64,
    /// Maximum storage budget in bytes.
    max_bytes: u64,
}

impl TileManager {
    pub fn new(max_bytes: u64) -> Self {
        Self {
            tiles: HashMap::new(),
            latest_versions: HashMap::new(),
            total_bytes: 0,
            max_bytes,
        }
    }

    /// Store a tile. Evicts oldest tiles if over budget.
    pub fn store_tile(&mut self, tile: MapTile) {
        let key = TileKey {
            zoom: tile.zoom_level,
            x: tile.tile_x,
            y: tile.tile_y,
        };

        let tile_size = tile.data_hash.len() as u64 + 128; // approximate

        // Subtract old tile's size BEFORE the eviction check so the budget
        // calculation reflects the space that will be freed by the replacement.
        if let Some(old_tile) = self.tiles.get(&key) {
            let old_size = old_tile.data_hash.len() as u64 + 128;
            self.total_bytes = self.total_bytes.saturating_sub(old_size);
        }

        // Evict if over budget.
        while self.total_bytes + tile_size > self.max_bytes && !self.tiles.is_empty() {
            self.evict_oldest();
        }

        debug!(zoom = key.zoom, x = key.x, y = key.y, "storing tile");
        self.total_bytes += tile_size;
        self.tiles.insert(key, tile);
    }

    /// Get a tile by key.
    pub fn get_tile(&self, key: &TileKey) -> Option<&MapTile> {
        self.tiles.get(key)
    }

    /// Get the status of a tile.
    pub fn tile_status(&self, key: &TileKey) -> TileStatus {
        match self.tiles.get(key) {
            None => TileStatus::Missing,
            Some(tile) => {
                if let Some(&latest) = self.latest_versions.get(key) {
                    if tile.version < latest {
                        TileStatus::Stale
                    } else {
                        TileStatus::Current
                    }
                } else {
                    TileStatus::Current
                }
            }
        }
    }

    /// Register the latest version for a tile (from server).
    pub fn register_latest_version(&mut self, key: TileKey, version: u64) {
        self.latest_versions.insert(key, version);
    }

    /// Get all tiles that need updating.
    pub fn stale_tiles(&self) -> Vec<TileKey> {
        self.tiles
            .keys()
            .filter(|k| self.tile_status(k) == TileStatus::Stale)
            .copied()
            .collect()
    }

    /// Get tiles needed for a region defined by bounding box at a given zoom level.
    pub fn tiles_for_region(
        &self,
        min_lat: f64,
        max_lat: f64,
        min_lon: f64,
        max_lon: f64,
        zoom: u8,
    ) -> Vec<TileKey> {
        let n = 1u32 << zoom;
        let min_x = lon_to_tile_x(min_lon, n);
        let max_x = lon_to_tile_x(max_lon, n);
        let min_y = lat_to_tile_y(max_lat, n); // note: y is inverted
        let max_y = lat_to_tile_y(min_lat, n);

        let mut keys = Vec::new();
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                keys.push(TileKey { zoom, x, y });
            }
        }
        keys
    }

    /// Number of tiles stored.
    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }

    /// Approximate storage used in bytes.
    pub fn storage_used(&self) -> u64 {
        self.total_bytes
    }

    /// Remove all tiles.
    pub fn clear(&mut self) {
        self.tiles.clear();
        self.total_bytes = 0;
        info!("tile cache cleared");
    }

    fn evict_oldest(&mut self) {
        // Find oldest tile by updated_at.
        let oldest = self
            .tiles
            .iter()
            .min_by_key(|(_, t)| t.updated_at)
            .map(|(k, _)| *k);

        if let Some(key) = oldest {
            if let Some(tile) = self.tiles.remove(&key) {
                let size = tile.data_hash.len() as u64 + 128;
                self.total_bytes = self.total_bytes.saturating_sub(size);
                debug!(zoom = key.zoom, x = key.x, y = key.y, "evicted tile");
            }
        }
    }
}

/// Convert longitude to tile X coordinate.
fn lon_to_tile_x(lon: f64, n: u32) -> u32 {
    ((lon + 180.0) / 360.0 * n as f64).floor() as u32
}

/// Convert latitude to tile Y coordinate (Mercator).
fn lat_to_tile_y(lat: f64, n: u32) -> u32 {
    let lat_rad = lat.to_radians();
    ((1.0 - lat_rad.tan().asinh() / std::f64::consts::PI) / 2.0 * n as f64).floor() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use aurora_core::types::EntityId;
    use chrono::Utc;

    fn make_tile(zoom: u8, x: u32, y: u32, version: u64) -> MapTile {
        MapTile {
            id: EntityId::new(),
            zoom_level: zoom,
            tile_x: x,
            tile_y: y,
            version,
            data_hash: "abc123".into(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn store_and_retrieve_tile() {
        let mut mgr = TileManager::new(1_000_000);
        let tile = make_tile(14, 100, 200, 1);
        mgr.store_tile(tile);
        assert_eq!(mgr.tile_count(), 1);

        let key = TileKey {
            zoom: 14,
            x: 100,
            y: 200,
        };
        assert!(mgr.get_tile(&key).is_some());
        assert_eq!(mgr.tile_status(&key), TileStatus::Current);
    }

    #[test]
    fn stale_detection() {
        let mut mgr = TileManager::new(1_000_000);
        let tile = make_tile(14, 100, 200, 1);
        mgr.store_tile(tile);

        let key = TileKey {
            zoom: 14,
            x: 100,
            y: 200,
        };
        mgr.register_latest_version(key, 2);
        assert_eq!(mgr.tile_status(&key), TileStatus::Stale);
        assert_eq!(mgr.stale_tiles().len(), 1);
    }

    #[test]
    fn eviction_on_budget_exceeded() {
        // Very small budget: only room for ~1 tile.
        let mut mgr = TileManager::new(200);
        mgr.store_tile(make_tile(14, 1, 1, 1));
        mgr.store_tile(make_tile(14, 2, 2, 1));
        // Second tile should evict the first.
        assert_eq!(mgr.tile_count(), 1);
    }

    #[test]
    fn tiles_for_region_returns_grid() {
        let mgr = TileManager::new(1_000_000);
        let tiles = mgr.tiles_for_region(32.0, 32.1, 34.7, 34.8, 14);
        assert!(!tiles.is_empty());
        for t in &tiles {
            assert_eq!(t.zoom, 14);
        }
    }
}
