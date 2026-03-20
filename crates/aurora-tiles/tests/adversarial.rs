use aurora_tiles::engine::*;
fn tc(z: u8, x: u32, y: u32) -> TileCoord {
    TileCoord { z, x, y }
}
fn tile(c: TileCoord) -> VectorTile {
    VectorTile {
        coord: c,
        layers: vec![],
        size_bytes: 1024,
        loaded_ms: 10,
    }
}
#[test]
fn stress_load() {
    let mut e = TileEngine::new(100);
    for i in 0..1000 {
        let c = tc(14, i, 0);
        e.load_tile(c, tile(c));
    }
    assert!(e.cached_count() <= 100);
}
#[test]
fn get_missing() {
    let mut e = TileEngine::new(10);
    assert!(e.get_tile(&tc(0, 0, 0)).is_none());
}
#[test]
fn evict_missing() {
    let mut e = TileEngine::new(10);
    assert!(!e.evict(&tc(0, 0, 0)));
}
#[test]
fn duplicate_load() {
    let mut e = TileEngine::new(10);
    let c = tc(14, 0, 0);
    e.load_tile(c, tile(c));
    e.load_tile(c, tile(c));
    assert_eq!(e.cached_count(), 1);
}
#[test]
fn hit_rate_after_hits() {
    let mut e = TileEngine::new(10);
    let c = tc(14, 0, 0);
    e.load_tile(c, tile(c));
    e.get_tile(&c);
    assert!(e.hit_rate() > 0.0);
}
#[test]
fn zero_capacity() {
    let mut e = TileEngine::new(0);
    let c = tc(14, 0, 0);
    e.load_tile(c, tile(c));
    assert!(e.cached_count() <= 1);
}
#[test]
fn tile_serializes() {
    let t = tile(tc(14, 0, 0));
    let j = serde_json::to_string(&t).unwrap();
    assert!(j.contains("size_bytes"));
}
#[test]
fn coord_hash_eq() {
    let a = tc(14, 100, 200);
    let b = tc(14, 100, 200);
    assert_eq!(a, b);
}
