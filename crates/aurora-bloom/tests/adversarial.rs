//! Adversarial tests for aurora-bloom

use aurora_bloom::filter::BloomFilter;

#[test]
fn adversarial_false_positive_rate_and_clear() {
    // Auto-size for 100 items at 1% FP rate
    let mut bf = BloomFilter::with_rate(100, 0.01);
    assert!(bf.num_bits() > 500, "Should have enough bits for 1% FP");
    assert!(bf.num_hashes() >= 3, "Should have multiple hash functions");

    // Insert 50 items
    for i in 0..50u32 {
        let key = format!("item-{}", i);
        bf.insert(key.as_bytes());
    }
    assert_eq!(bf.inserted(), 50);

    // All inserted items MUST be found (no false negatives)
    for i in 0..50u32 {
        let key = format!("item-{}", i);
        assert!(
            bf.might_contain(key.as_bytes()),
            "Inserted item {} must be found",
            i
        );
    }

    // Query 1000 non-inserted items, count false positives
    let mut false_positives = 0u32;
    for i in 1000..2000u32 {
        let key = format!("nonexistent-{}", i);
        if bf.might_contain(key.as_bytes()) {
            false_positives += 1;
        }
    }
    let fp_rate = false_positives as f64 / 1000.0;
    assert!(
        fp_rate < 0.05,
        "FP rate {} should be < 5% (target was 1%)",
        fp_rate
    );

    // Fill ratio should be between 0 and 1
    assert!(bf.fill_ratio() > 0.0);
    assert!(bf.fill_ratio() < 1.0);

    // Estimated FP rate should be > 0 (some bits set)
    assert!(bf.estimated_fp_rate() > 0.0);

    // Clear resets everything
    bf.clear();
    assert_eq!(bf.inserted(), 0);
    assert_eq!(bf.bits_set(), 0);
    // Previously inserted items should NOT be found after clear
    assert!(!bf.might_contain(b"item-0"));
    assert!(!bf.might_contain(b"item-25"));
}
