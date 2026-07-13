//! Offline cache — manages offline map regions, routing data, and risk caches.

use chrono::{DateTime, Utc};
use gane_core::types::{EntityId, GeoPosition};
use gane_map::tile_manager::{TileKey, TileManager};
use std::collections::HashMap;
use tracing::info;

/// An offline region that the user has downloaded.
#[derive(Debug, Clone)]
pub struct OfflineRegion {
    pub id: EntityId,
    pub name: String,
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_lon: f64,
    pub max_lon: f64,
    pub zoom_levels: Vec<u8>,
    pub downloaded_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub tile_count: usize,
    pub size_bytes: u64,
    pub status: RegionStatus,
}

/// Status of an offline region.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionStatus {
    /// Fully downloaded and current.
    Ready,
    /// Download in progress.
    Downloading,
    /// Update available.
    UpdateAvailable,
    /// Download failed — partial data.
    Failed,
}

/// Manages offline map caching with region-based downloads.
pub struct OfflineCache {
    /// Underlying tile manager.
    tile_manager: TileManager,
    /// Downloaded regions.
    regions: HashMap<EntityId, OfflineRegion>,
    /// Maximum storage budget (bytes).
    max_storage_bytes: u64,
}

impl OfflineCache {
    pub fn new(max_storage_bytes: u64) -> Self {
        Self {
            tile_manager: TileManager::new(max_storage_bytes),
            regions: HashMap::new(),
            max_storage_bytes,
        }
    }

    /// Create a new offline region for download.
    pub fn create_region(
        &mut self,
        name: String,
        min_lat: f64,
        max_lat: f64,
        min_lon: f64,
        max_lon: f64,
        zoom_levels: Vec<u8>,
    ) -> EntityId {
        let id = EntityId::new();

        // Count tiles needed.
        let mut tile_count = 0;
        for &zoom in &zoom_levels {
            let tiles = self
                .tile_manager
                .tiles_for_region(min_lat, max_lat, min_lon, max_lon, zoom);
            tile_count += tiles.len();
        }

        let region = OfflineRegion {
            id,
            name: name.clone(),
            min_lat,
            max_lat,
            min_lon,
            max_lon,
            zoom_levels,
            downloaded_at: Utc::now(),
            last_updated: Utc::now(),
            tile_count,
            size_bytes: 0,
            status: RegionStatus::Downloading,
        };

        info!(
            region = %name,
            tiles = tile_count,
            "creating offline region"
        );

        self.regions.insert(id, region);
        id
    }

    /// Mark a region as fully downloaded.
    pub fn mark_region_ready(&mut self, region_id: EntityId) {
        if let Some(region) = self.regions.get_mut(&region_id) {
            region.status = RegionStatus::Ready;
            region.size_bytes = self.tile_manager.storage_used();
            info!(region = %region.name, "offline region ready");
        }
    }

    /// Mark a region as having an update available.
    pub fn mark_update_available(&mut self, region_id: EntityId) {
        if let Some(region) = self.regions.get_mut(&region_id) {
            region.status = RegionStatus::UpdateAvailable;
        }
    }

    /// Check if a position is covered by any offline region.
    pub fn is_covered(&self, pos: &GeoPosition) -> bool {
        self.regions.values().any(|r| {
            r.status == RegionStatus::Ready
                && pos.latitude_deg >= r.min_lat
                && pos.latitude_deg <= r.max_lat
                && pos.longitude_deg >= r.min_lon
                && pos.longitude_deg <= r.max_lon
        })
    }

    /// Get tiles needed for a region that are not yet cached.
    pub fn missing_tiles(&self, region_id: EntityId) -> Vec<TileKey> {
        let region = match self.regions.get(&region_id) {
            Some(r) => r,
            None => return Vec::new(),
        };

        let mut missing = Vec::new();
        for &zoom in &region.zoom_levels {
            let tiles = self.tile_manager.tiles_for_region(
                region.min_lat,
                region.max_lat,
                region.min_lon,
                region.max_lon,
                zoom,
            );
            for key in tiles {
                if self.tile_manager.get_tile(&key).is_none() {
                    missing.push(key);
                }
            }
        }
        missing
    }

    /// Get all regions.
    pub fn regions(&self) -> Vec<&OfflineRegion> {
        self.regions.values().collect()
    }

    /// Delete a region and its tiles.
    pub fn delete_region(&mut self, region_id: EntityId) -> bool {
        if self.regions.remove(&region_id).is_some() {
            info!("deleted offline region");
            true
        } else {
            false
        }
    }

    /// Get total storage used.
    pub fn storage_used(&self) -> u64 {
        self.tile_manager.storage_used()
    }

    /// Get storage budget.
    pub fn storage_budget(&self) -> u64 {
        self.max_storage_bytes
    }

    /// Access the underlying tile manager.
    pub fn tile_manager(&self) -> &TileManager {
        &self.tile_manager
    }

    /// Mutable access to the tile manager.
    pub fn tile_manager_mut(&mut self) -> &mut TileManager {
        &mut self.tile_manager
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_query_region() {
        let mut cache = OfflineCache::new(100_000_000);
        let id = cache.create_region("Tel Aviv".into(), 32.0, 32.1, 34.7, 34.8, vec![14, 15]);

        assert_eq!(cache.regions().len(), 1);

        // Not ready yet.
        let pos = GeoPosition {
            latitude_deg: 32.05,
            longitude_deg: 34.75,
            altitude_m: None,
        };
        assert!(!cache.is_covered(&pos));

        cache.mark_region_ready(id);
        assert!(cache.is_covered(&pos));
    }

    #[test]
    fn position_outside_region_not_covered() {
        let mut cache = OfflineCache::new(100_000_000);
        let id = cache.create_region("Tel Aviv".into(), 32.0, 32.1, 34.7, 34.8, vec![14]);
        cache.mark_region_ready(id);

        let outside = GeoPosition {
            latitude_deg: 33.0,
            longitude_deg: 35.0,
            altitude_m: None,
        };
        assert!(!cache.is_covered(&outside));
    }

    #[test]
    fn delete_region_works() {
        let mut cache = OfflineCache::new(100_000_000);
        let id = cache.create_region("Test".into(), 32.0, 32.1, 34.7, 34.8, vec![14]);
        assert_eq!(cache.regions().len(), 1);
        assert!(cache.delete_region(id));
        assert_eq!(cache.regions().len(), 0);
    }

    #[test]
    fn missing_tiles_calculated() {
        let mut cache = OfflineCache::new(100_000_000);
        let id = cache.create_region(
            "Small Area".into(),
            32.085,
            32.086,
            34.781,
            34.782,
            vec![14],
        );
        let missing = cache.missing_tiles(id);
        assert!(!missing.is_empty()); // All tiles should be missing.
    }
}
