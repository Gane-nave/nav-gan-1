use gane_dag::graph::Dag;
use gane_dag::sort::{critical_path_length, node_depths, topological_sort, TopoResult};

#[test]
fn adversarial_cycle_detection_topological_sort() {
    let mut dag = Dag::new();

    // Build diamond dependency graph: a→b, a→c, b→d, c→d
    assert!(dag.add_edge("a", "b"));
    assert!(dag.add_edge("a", "c"));
    assert!(dag.add_edge("b", "d"));
    assert!(dag.add_edge("c", "d"));

    assert_eq!(dag.node_count(), 4);
    assert_eq!(dag.edge_count(), 4);

    // Verify structure
    let a_succ = dag.successors("a");
    assert_eq!(a_succ.len(), 2);
    assert!(a_succ.contains(&"b".to_string()));
    assert!(a_succ.contains(&"c".to_string()));

    let d_pred = dag.predecessors("d");
    assert_eq!(d_pred.len(), 2);
    assert!(d_pred.contains(&"b".to_string()));
    assert!(d_pred.contains(&"c".to_string()));

    // Roots and leaves
    let roots = dag.roots();
    assert_eq!(roots.len(), 1);
    assert_eq!(roots[0], "a");
    let leaves = dag.leaves();
    assert_eq!(leaves.len(), 1);
    assert_eq!(leaves[0], "d");

    // Topological sort should respect all dependencies
    let result = topological_sort(&dag);
    if let TopoResult::Order(order) = &result {
        let pos_a = order.iter().position(|n| n == "a").unwrap();
        let pos_b = order.iter().position(|n| n == "b").unwrap();
        let pos_c = order.iter().position(|n| n == "c").unwrap();
        let pos_d = order.iter().position(|n| n == "d").unwrap();

        // a must come before b and c
        assert!(pos_a < pos_b);
        assert!(pos_a < pos_c);
        // b and c must come before d
        assert!(pos_b < pos_d);
        assert!(pos_c < pos_d);
    } else {
        panic!("Expected Order, got Cycle");
    }

    // Attempt to add cycle: d→a should be rejected
    assert!(!dag.add_edge("d", "a"));
    // Graph should be unchanged
    assert_eq!(dag.edge_count(), 4);

    // Attempt self-loop: a→a should be rejected
    assert!(!dag.add_edge("a", "a"));

    // Attempt indirect cycle: d→b should be rejected (b→d exists, so d→b would create cycle)
    assert!(!dag.add_edge("d", "b"));

    // Node depths
    let depths = node_depths(&dag);
    assert_eq!(*depths.get("a").unwrap(), 0);
    assert_eq!(*depths.get("b").unwrap(), 1);
    assert_eq!(*depths.get("c").unwrap(), 1);
    assert_eq!(*depths.get("d").unwrap(), 2);

    // Critical path length
    let cp = critical_path_length(&dag);
    assert_eq!(cp, 2); // a→b→d or a→c→d = depth 2

    // Reachability
    assert!(dag.can_reach("a", "d"));
    assert!(dag.can_reach("a", "b"));
    assert!(!dag.can_reach("d", "a"));
    assert!(!dag.can_reach("b", "c")); // b and c are siblings, no path

    // Add more edges: d→e, e→f (extend chain)
    assert!(dag.add_edge("d", "e"));
    assert!(dag.add_edge("e", "f"));
    assert_eq!(dag.node_count(), 6);

    let result2 = topological_sort(&dag);
    if let TopoResult::Order(order) = &result2 {
        assert_eq!(order.len(), 6);
        let pos_d = order.iter().position(|n| n == "d").unwrap();
        let pos_e = order.iter().position(|n| n == "e").unwrap();
        let pos_f = order.iter().position(|n| n == "f").unwrap();
        assert!(pos_d < pos_e);
        assert!(pos_e < pos_f);
    } else {
        panic!("Expected Order after extension");
    }

    // Updated critical path
    let cp2 = critical_path_length(&dag);
    assert_eq!(cp2, 4); // a→b→d→e→f = depth 4

    // Remove edge and verify
    dag.remove_edge("d", "e");
    assert!(!dag.can_reach("a", "f"));
    // e→f still exists but disconnected from main graph
    assert!(dag.can_reach("e", "f"));
}
