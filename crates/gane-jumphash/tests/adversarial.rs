//! Adversarial tests for gane-jumphash.

use gane_jumphash::{jump_hash, JumpHasher};

#[test]
fn adversarial_zero_buckets() {
    assert_eq!(jump_hash(42, 0), 0);
}

#[test]
fn adversarial_single_bucket() {
    for k in 0..1000 {
        assert_eq!(jump_hash(k, 1), 0);
    }
}

#[test]
fn adversarial_max_key() {
    let bucket = jump_hash(u64::MAX, 10);
    assert!(bucket < 10);
}

#[test]
fn adversarial_large_bucket_count() {
    let bucket = jump_hash(42, u32::MAX);
    assert!(bucket < u32::MAX);
}

#[test]
fn adversarial_distribution() {
    let h = JumpHasher::new(8);
    let dist = h.distribution(80000);
    for &count in &dist {
        assert!(count > 5000, "bucket count {} too low", count);
        assert!(count < 15000, "bucket count {} too high", count);
    }
}

#[test]
fn adversarial_resize_min() {
    let mut h = JumpHasher::new(10);
    h.resize(0);
    assert_eq!(h.num_buckets(), 1);
}

#[test]
fn adversarial_monotonicity_property() {
    for key in 0..5000_u64 {
        let old = jump_hash(key, 5);
        let new = jump_hash(key, 6);
        if old != new {
            assert_eq!(
                new, 5,
                "key {} moved from {} to {} instead of 5",
                key, old, new
            );
        }
    }
}

#[test]
fn adversarial_deterministic() {
    let h = JumpHasher::new(100);
    let b1 = h.hash_key(&"test_key");
    let b2 = h.hash_key(&"test_key");
    assert_eq!(b1, b2);
}
