//! Adversarial tests for aurora-worksteal.

use aurora_worksteal::WorkStealDeque;

#[test]
fn adversarial_empty_operations() {
    let mut d: WorkStealDeque<i32> = WorkStealDeque::new();
    assert_eq!(d.pop(), None);
    assert_eq!(d.steal(), None);
    assert!(d.steal_batch(10).is_empty());
}

#[test]
fn adversarial_push_pop_lifo() {
    let mut d = WorkStealDeque::new();
    for i in 0..100 {
        d.push(i);
    }
    for i in (0..100).rev() {
        assert_eq!(d.pop(), Some(i));
    }
}

#[test]
fn adversarial_steal_fifo() {
    let mut d = WorkStealDeque::new();
    for i in 0..100 {
        d.push(i);
    }
    for i in 0..100 {
        assert_eq!(d.steal(), Some(i));
    }
}

#[test]
fn adversarial_steal_batch_zero() {
    let mut d = WorkStealDeque::new();
    d.push(1);
    let batch = d.steal_batch(0);
    assert!(batch.is_empty());
    assert_eq!(d.len(), 1);
}

#[test]
fn adversarial_counts_accurate() {
    let mut d = WorkStealDeque::new();
    for i in 0..50 {
        d.push(i);
    }
    for _ in 0..20 {
        d.pop();
    }
    for _ in 0..10 {
        d.steal();
    }
    assert_eq!(d.pushed_count(), 50);
    assert_eq!(d.popped_count(), 20);
    assert_eq!(d.stolen_count(), 10);
    assert_eq!(d.len(), 20);
}

#[test]
fn adversarial_drain_all() {
    let mut d = WorkStealDeque::new();
    for i in 0..100 {
        d.push(i);
    }
    let items = d.drain();
    assert_eq!(items.len(), 100);
    assert!(d.is_empty());
}

#[test]
fn adversarial_clear_and_reuse() {
    let mut d = WorkStealDeque::new();
    for i in 0..50 {
        d.push(i);
    }
    d.clear();
    assert!(d.is_empty());
    d.push(999);
    assert_eq!(d.pop(), Some(999));
}

#[test]
fn adversarial_mixed_operations() {
    let mut d = WorkStealDeque::new();
    d.push(1);
    d.push(2);
    d.push(3);
    assert_eq!(d.steal(), Some(1)); // front
    d.push(4);
    assert_eq!(d.pop(), Some(4)); // back
    assert_eq!(d.len(), 2); // 2, 3 remain
}
