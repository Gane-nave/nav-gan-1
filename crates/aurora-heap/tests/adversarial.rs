use aurora_heap::heap::{BinaryHeap, HeapMode};

#[test]
fn adversarial_min_max_heap_stable_ordering() {
    // === Min Heap Test ===
    let mut min_heap = BinaryHeap::min_heap();
    assert_eq!(min_heap.mode(), HeapMode::Min);

    // Push 10 items with mixed priorities, including duplicates
    min_heap.push(5, "five-a");
    min_heap.push(3, "three-a");
    min_heap.push(8, "eight");
    min_heap.push(1, "one");
    min_heap.push(5, "five-b");
    min_heap.push(3, "three-b");
    min_heap.push(10, "ten");
    min_heap.push(2, "two");
    min_heap.push(5, "five-c");
    min_heap.push(0, "zero");

    assert_eq!(min_heap.len(), 10);
    assert_eq!(min_heap.peak_size(), 10);
    assert_eq!(min_heap.total_pushes(), 10);

    // Peek should return lowest priority (0)
    assert_eq!(min_heap.peek(), Some((0, &"zero")));

    // Pop all — verify ascending priority order
    let items = min_heap.drain_sorted();
    assert_eq!(items.len(), 10);

    let mut prev_priority = i64::MIN;
    for (pri, _val) in &items {
        assert!(*pri >= prev_priority);
        prev_priority = *pri;
    }

    // Verify specific order: 0, 1, 2, 3a, 3b, 5a, 5b, 5c, 8, 10
    assert_eq!(items[0], (0, "zero"));
    assert_eq!(items[1], (1, "one"));
    assert_eq!(items[2], (2, "two"));
    assert_eq!(items[3], (3, "three-a"));
    assert_eq!(items[4], (3, "three-b"));
    assert_eq!(items[5], (5, "five-a"));
    assert_eq!(items[6], (5, "five-b"));
    assert_eq!(items[7], (5, "five-c"));
    assert_eq!(items[8], (8, "eight"));
    assert_eq!(items[9], (10, "ten"));

    assert!(min_heap.is_empty());
    assert_eq!(min_heap.total_pops(), 10);

    // === Max Heap Test ===
    let mut max_heap = BinaryHeap::max_heap();
    assert_eq!(max_heap.mode(), HeapMode::Max);

    max_heap.push(5, "five-a");
    max_heap.push(3, "three");
    max_heap.push(8, "eight-a");
    max_heap.push(1, "one");
    max_heap.push(8, "eight-b");
    max_heap.push(10, "ten");
    max_heap.push(2, "two");
    max_heap.push(-1, "neg-one");

    assert_eq!(max_heap.len(), 8);

    // Peek should return highest priority (10)
    assert_eq!(max_heap.peek(), Some((10, &"ten")));

    // Pop all — verify descending priority order
    let max_items = max_heap.drain_sorted();
    assert_eq!(max_items.len(), 8);

    let mut prev_max = i64::MAX;
    for (pri, _val) in &max_items {
        assert!(*pri <= prev_max);
        prev_max = *pri;
    }

    // Verify: 10, 8a, 8b, 5, 3, 2, 1, -1
    assert_eq!(max_items[0], (10, "ten"));
    assert_eq!(max_items[1], (8, "eight-a"));
    assert_eq!(max_items[2], (8, "eight-b"));
    assert_eq!(max_items[3], (5, "five-a"));
    assert_eq!(max_items[4], (3, "three"));
    assert_eq!(max_items[5], (2, "two"));
    assert_eq!(max_items[6], (1, "one"));
    assert_eq!(max_items[7], (-1, "neg-one"));

    // === Edge cases ===
    let mut edge_heap = BinaryHeap::min_heap();

    // Pop from empty
    assert!(edge_heap.pop().is_none());
    assert!(edge_heap.peek().is_none());

    // Single element
    edge_heap.push(42, "solo");
    assert_eq!(edge_heap.len(), 1);
    assert_eq!(edge_heap.pop(), Some((42, "solo")));
    assert!(edge_heap.is_empty());

    // Negative priorities
    edge_heap.push(-100, "very-low");
    edge_heap.push(-50, "low");
    edge_heap.push(0, "zero");
    assert_eq!(edge_heap.pop(), Some((-100, "very-low")));
    assert_eq!(edge_heap.pop(), Some((-50, "low")));
    assert_eq!(edge_heap.pop(), Some((0, "zero")));

    // Clear
    edge_heap.push(1, "a");
    edge_heap.push(2, "b");
    edge_heap.clear();
    assert!(edge_heap.is_empty());
    assert_eq!(edge_heap.len(), 0);
}
