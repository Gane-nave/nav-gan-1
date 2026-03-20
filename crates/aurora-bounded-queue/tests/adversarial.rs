//! Adversarial tests for aurora-bounded-queue.

use aurora_bounded_queue::BoundedQueue;

#[test]
fn adversarial_empty_operations() {
    let mut q: BoundedQueue<i32> = BoundedQueue::new(10);
    assert_eq!(q.pop(), None);
    assert_eq!(q.peek(), None);
    assert!(q.is_empty());
    assert!(!q.is_full());
}

#[test]
fn adversarial_capacity_one() {
    let mut q = BoundedQueue::new(1);
    assert_eq!(q.push(1), aurora_bounded_queue::PushResult::Ok);
    assert_eq!(q.push(2), aurora_bounded_queue::PushResult::Full);
    assert_eq!(q.pop(), Some(1));
    assert_eq!(q.push(3), aurora_bounded_queue::PushResult::Ok);
    assert_eq!(q.pop(), Some(3));
}

#[test]
fn adversarial_zero_capacity_clamps() {
    let q: BoundedQueue<i32> = BoundedQueue::new(0);
    assert_eq!(q.capacity(), 1); // clamped to 1
}

#[test]
fn adversarial_force_push_evicts_oldest() {
    let mut q = BoundedQueue::new(3);
    q.force_push(1);
    q.force_push(2);
    q.force_push(3);
    q.force_push(4); // evicts 1
    q.force_push(5); // evicts 2
    assert_eq!(q.pop(), Some(3));
    assert_eq!(q.pop(), Some(4));
    assert_eq!(q.pop(), Some(5));
}

#[test]
fn adversarial_rejected_count_accurate() {
    let mut q = BoundedQueue::new(2);
    q.push(1);
    q.push(2);
    for _ in 0..100 {
        q.push(99);
    }
    assert_eq!(q.rejected_count(), 100);
    assert_eq!(q.len(), 2);
}

#[test]
fn adversarial_drain_updates_dequeued() {
    let mut q = BoundedQueue::new(10);
    for i in 0..5 {
        q.push(i);
    }
    let items = q.drain();
    assert_eq!(items.len(), 5);
    assert_eq!(q.dequeued_count(), 5);
    assert!(q.is_empty());
}

#[test]
fn adversarial_load_factor_bounds() {
    let mut q = BoundedQueue::new(4);
    assert!((q.load_factor() - 0.0).abs() < f64::EPSILON);
    q.push(1);
    q.push(2);
    q.push(3);
    q.push(4);
    assert!((q.load_factor() - 1.0).abs() < f64::EPSILON);
}

#[test]
fn adversarial_clear_and_reuse() {
    let mut q = BoundedQueue::new(5);
    for i in 0..5 {
        q.push(i);
    }
    assert!(q.is_full());
    q.clear();
    assert!(q.is_empty());
    assert_eq!(q.remaining(), 5);
    q.push(42);
    assert_eq!(q.pop(), Some(42));
}
