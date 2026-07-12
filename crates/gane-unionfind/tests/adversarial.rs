//! Adversarial tests for gane-unionfind.

use gane_unionfind::UnionFind;

#[test]
fn adversarial_large_chain() {
    let mut uf = UnionFind::new(10_000);
    for i in 0..9_999 {
        uf.union(i, i + 1);
    }
    assert_eq!(uf.components(), 1);
    // All should be connected after path compression
    let root = uf.find(0);
    assert_eq!(uf.find(9_999), root);
    assert_eq!(uf.find(5_000), root);
}

#[test]
fn adversarial_union_self() {
    let mut uf = UnionFind::new(5);
    assert!(!uf.union(2, 2), "union with self should be no-op");
    assert_eq!(uf.components(), 5);
}

#[test]
fn adversarial_double_union() {
    let mut uf = UnionFind::new(5);
    assert!(uf.union(0, 1));
    assert!(!uf.union(0, 1)); // already connected
    assert!(!uf.union(1, 0)); // reverse, still same set
    assert_eq!(uf.components(), 4);
}

#[test]
fn adversarial_star_topology() {
    let mut uf = UnionFind::new(1000);
    // Connect all to element 0
    for i in 1..1000 {
        uf.union(0, i);
    }
    assert_eq!(uf.components(), 1);
    assert_eq!(uf.component_size(500), 1000);
}

#[test]
fn adversarial_two_large_components_merge() {
    let mut uf = UnionFind::new(200);
    // Component A: 0..100
    for i in 0..99 {
        uf.union(i, i + 1);
    }
    // Component B: 100..200
    for i in 100..199 {
        uf.union(i, i + 1);
    }
    assert_eq!(uf.components(), 2);
    // Merge them
    uf.union(50, 150);
    assert_eq!(uf.components(), 1);
    assert!(uf.connected(0, 199));
}

#[test]
fn adversarial_reset_and_rebuild() {
    let mut uf = UnionFind::new(10);
    for i in 0..9 {
        uf.union(i, i + 1);
    }
    assert_eq!(uf.components(), 1);
    uf.reset();
    assert_eq!(uf.components(), 10);
    assert!(!uf.connected(0, 1));
    // Rebuild different structure
    uf.union(0, 2);
    uf.union(2, 4);
    assert!(uf.connected(0, 4));
    assert!(!uf.connected(0, 1));
}

#[test]
#[should_panic]
fn adversarial_out_of_bounds() {
    let mut uf = UnionFind::new(5);
    uf.find(100);
}

#[test]
fn adversarial_single_element() {
    let mut uf = UnionFind::new(1);
    assert_eq!(uf.find(0), 0);
    assert_eq!(uf.components(), 1);
    assert_eq!(uf.component_size(0), 1);
}
