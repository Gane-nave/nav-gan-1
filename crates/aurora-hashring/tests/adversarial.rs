use aurora_hashring::HashRing;

#[test]
fn adversarial_consistent_hashing_rebalance_and_replication() {
    // Phase 1: Build ring with 5 nodes, 100 virtual nodes each
    let mut ring = HashRing::new(100);
    for i in 0..5 {
        assert!(ring.add_node(&format!("node-{}", i)));
    }
    assert_eq!(ring.node_count(), 5);
    assert_eq!(ring.virtual_node_count(), 500);

    // Phase 2: Assign 1000 keys and record distribution
    let mut assignments = Vec::new();
    for i in 0..1000 {
        let key = format!("user-session-{}", i);
        let node = ring.lookup(&key).unwrap().to_string();
        assignments.push((key, node));
    }

    // Verify all 5 nodes got some keys (with 100 vnodes, distribution should be reasonable)
    let dist = ring.distribution();
    assert_eq!(dist.len(), 5);
    for i in 0..5 {
        let name = format!("node-{}", i);
        assert!(
            dist.contains_key(name.as_str()),
            "node-{} missing from distribution",
            i
        );
    }

    // Phase 3: Remove one node and measure key movement
    ring.remove_node("node-2");
    assert_eq!(ring.node_count(), 4);
    assert!(!ring.contains("node-2"));

    let mut moved = 0;
    for (key, old_node) in &assignments {
        let new_node = ring.lookup(key).unwrap();
        if new_node != old_node {
            moved += 1;
        }
        // No key should map to the removed node
        assert_ne!(new_node, "node-2", "key still maps to removed node");
    }
    // With consistent hashing, roughly 1/5 of keys should move (the ones on removed node)
    // Allow generous bounds: at least some moved, but not all
    assert!(
        moved > 50,
        "too few keys moved after node removal: {}",
        moved
    );
    assert!(
        moved < 500,
        "too many keys moved after node removal: {}",
        moved
    );

    // Phase 4: Test replication with lookup_n
    ring.add_node("node-5"); // add replacement
    let replicas = ring.lookup_n("critical-data", 3);
    assert_eq!(replicas.len(), 3);
    // All replicas should be distinct
    assert_ne!(replicas[0], replicas[1]);
    assert_ne!(replicas[0], replicas[2]);
    assert_ne!(replicas[1], replicas[2]);

    // Phase 5: Request more replicas than nodes
    let all_replicas = ring.lookup_n("key-x", 100);
    assert_eq!(all_replicas.len(), 5); // capped at physical node count

    // Phase 6: Consistency after add — key that didn't move should still map same
    let stable_key = "stability-check";
    let before = ring.lookup(stable_key).unwrap().to_string();
    ring.add_node("node-6");
    let after = ring.lookup(stable_key).unwrap().to_string();
    // May or may not move, but should always resolve to a valid node
    assert!(ring.contains(&after));

    // Phase 7: Empty ring returns None
    let mut empty = HashRing::new(10);
    assert!(empty.lookup("anything").is_none());
    assert!(empty.lookup_n("anything", 3).is_empty());

    // Phase 8: Single node ring always returns that node
    empty.add_node("solo");
    for i in 0..100 {
        assert_eq!(empty.lookup(&format!("k{}", i)), Some("solo"));
    }

    // Verify lookup counter
    assert!(ring.lookups() > 0);
    let _ = before; // suppress unused warning
}
