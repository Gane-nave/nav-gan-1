use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TileCoord {
    pub z: u8,
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorTile {
    pub coord: TileCoord,
    pub layers: Vec<TileLayer>,
    pub size_bytes: usize,
    pub loaded_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileLayer {
    pub name: String,
    pub feature_count: u32,
}

pub struct TileEngine {
    cache: HashMap<TileCoord, VectorTile>,
    max_cached: usize,
    loads: u64,
    cache_hits: u64,
}

impl TileEngine {
    pub fn new(max_cached: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_cached,
            loads: 0,
            cache_hits: 0,
        }
    }
    pub fn load_tile(&mut self, coord: TileCoord, tile: VectorTile) {
        self.loads += 1;
        if self.cache.len() >= self.max_cached {
            if let Some(&k) = self.cache.keys().next() {
                self.cache.remove(&k);
            }
        }
        self.cache.insert(coord, tile);
    }
    pub fn get_tile(&mut self, coord: &TileCoord) -> Option<&VectorTile> {
        if self.cache.contains_key(coord) {
            self.cache_hits += 1;
        }
        self.cache.get(coord)
    }
    pub fn cached_count(&self) -> usize {
        self.cache.len()
    }
    pub fn total_loads(&self) -> u64 {
        self.loads
    }
    pub fn hit_rate(&self) -> f64 {
        if self.loads == 0 {
            0.0
        } else {
            self.cache_hits as f64 / self.loads as f64
        }
    }
    pub fn evict(&mut self, coord: &TileCoord) -> bool {
        self.cache.remove(coord).is_some()
    }
    pub fn clear(&mut self) {
        self.cache.clear();
    }
    pub fn total_bytes(&self) -> usize {
        self.cache.values().map(|t| t.size_bytes).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn tc(z: u8, x: u32, y: u32) -> TileCoord {
        TileCoord { z, x, y }
    }
    fn tile(c: TileCoord) -> VectorTile {
        VectorTile {
            coord: c,
            layers: vec![TileLayer {
                name: "roads".into(),
                feature_count: 100,
            }],
            size_bytes: 4096,
            loaded_ms: 50,
        }
    }
    #[test]
    fn new_empty() {
        let e = TileEngine::new(10);
        assert_eq!(e.cached_count(), 0);
    }
    #[test]
    fn load_and_get() {
        let mut e = TileEngine::new(10);
        let c = tc(14, 8192, 5120);
        e.load_tile(c, tile(c));
        assert!(e.get_tile(&c).is_some());
    }
    #[test]
    fn eviction() {
        let mut e = TileEngine::new(2);
        for i in 0..5 {
            let c = tc(14, i, 0);
            e.load_tile(c, tile(c));
        }
        assert!(e.cached_count() <= 2);
    }
    #[test]
    fn evict_specific() {
        let mut e = TileEngine::new(10);
        let c = tc(14, 0, 0);
        e.load_tile(c, tile(c));
        assert!(e.evict(&c));
        assert_eq!(e.cached_count(), 0);
    }
    #[test]
    fn clear_all() {
        let mut e = TileEngine::new(10);
        for i in 0..5 {
            let c = tc(14, i, 0);
            e.load_tile(c, tile(c));
        }
        e.clear();
        assert_eq!(e.cached_count(), 0);
    }
    #[test]
    fn total_bytes() {
        let mut e = TileEngine::new(10);
        let c = tc(14, 0, 0);
        e.load_tile(c, tile(c));
        assert_eq!(e.total_bytes(), 4096);
    }
    #[test]
    fn hit_rate_zero() {
        let e = TileEngine::new(10);
        assert_eq!(e.hit_rate(), 0.0);
    }
    #[test]
    fn total_loads() {
        let mut e = TileEngine::new(10);
        for i in 0..3 {
            let c = tc(14, i, 0);
            e.load_tile(c, tile(c));
        }
        assert_eq!(e.total_loads(), 3);
    }
}
