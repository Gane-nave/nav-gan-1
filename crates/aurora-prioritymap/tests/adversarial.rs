//! Adversarial tests for aurora-prioritymap.

use aurora_prioritymap::PriorityMap;

#[test]
fn adversarial_insert_many() {
    let mut pm = PriorityMap::new();
    for i in 0..1000 {
        pm.insert(&format!("k{}", i), 1000 - i);
    }
    assert_eq!(pm.len(), 1000);
    // Min should be the last inserted (priority 1)
    let (key, pri) = pm.pop_min().unwrap();
    assert_eq!(pri, 1);
    assert_eq!(key, "k999");
}

#[test]
fn adversarial_update_to_min() {
    let mut pm = PriorityMap::new();
    pm.insert("a", 100);
    pm.insert("b", 50);
    pm.insert("c", 75);
    // Update a to lowest
    pm.insert("a", 1);
    assert_eq!(pm.peek_min().unwrap().0, "a");
}

#[test]
fn adversarial_remove_and_verify_heap() {
    let mut pm = PriorityMap::new();
    pm.insert("a", 10);
    pm.insert("b", 20);
    pm.insert("c", 5);
    pm.insert("d", 15);
    pm.remove("c"); // remove current min
    assert_eq!(pm.peek_min().unwrap().0, "a");
    assert_eq!(pm.peek_min().unwrap().1, 10);
}

#[test]
fn adversarial_decrease_priority_chain() {
    let mut pm = PriorityMap::new();
    pm.insert("x", 100);
    for i in (0..100).rev() {
        pm.decrease_priority("x", i);
    }
    assert_eq!(pm.get("x"), Some(0));
}

#[test]
fn adversarial_pop_all_sorted() {
    let mut pm = PriorityMap::new();
    pm.insert("c", 30);
    pm.insert("a", 10);
    pm.insert("d", 40);
    pm.insert("b", 20);
    let mut results = Vec::new();
    while let Some((key, pri)) = pm.pop_min() {
        results.push((key, pri));
    }
    assert_eq!(results.len(), 4);
    for i in 1..results.len() {
        assert!(
            results[i].1 >= results[i - 1].1,
            "should be sorted by priority"
        );
    }
}

#[test]
fn adversarial_negative_priorities() {
    let mut pm = PriorityMap::new();
    pm.insert("a", -100);
    pm.insert("b", -50);
    pm.insert("c", i64::MIN);
    assert_eq!(pm.peek_min().unwrap().0, "c");
    assert_eq!(pm.peek_min().unwrap().1, i64::MIN);
}

#[test]
fn adversarial_clear_and_reuse() {
    let mut pm = PriorityMap::new();
    for i in 0..100 {
        pm.insert(&format!("k{}", i), i);
    }
    pm.clear();
    assert!(pm.is_empty());
    pm.insert("new", 1);
    assert_eq!(pm.peek_min().unwrap().0, "new");
}

#[test]
fn adversarial_same_priority() {
    let mut pm = PriorityMap::new();
    for i in 0..50 {
        pm.insert(&format!("k{}", i), 42);
    }
    assert_eq!(pm.len(), 50);
    // All have same priority; pop should return them all
    let mut count = 0;
    while pm.pop_min().is_some() {
        count += 1;
    }
    assert_eq!(count, 50);
}
