//! Adversarial tests for aurora-semaphore

use aurora_semaphore::counter::{AcquireResult, CountingSemaphore};
use aurora_semaphore::fair::FairSemaphore;

#[test]
fn adversarial_ttl_expiry_closed_reopen() {
    let mut sem = CountingSemaphore::new(2, 3000);

    // Fill to capacity
    assert_eq!(
        sem.try_acquire(1000),
        AcquireResult::Acquired { permit_id: 1 }
    );
    assert_eq!(
        sem.try_acquire(1000),
        AcquireResult::Acquired { permit_id: 2 }
    );

    // At capacity — should be unavailable
    assert_eq!(sem.try_acquire(2000), AcquireResult::Unavailable);
    assert_eq!(sem.total_rejected(), 1);

    // After TTL expiry (1000 + 3000 = 4000), both expired at t=5000
    assert_eq!(
        sem.try_acquire(5000),
        AcquireResult::Acquired { permit_id: 3 }
    );
    assert_eq!(sem.active_count(), 1); // only the new one

    // Close blocks all
    sem.close();
    assert_eq!(sem.try_acquire(6000), AcquireResult::Closed);

    // Reopen allows again
    sem.reopen();
    assert_eq!(
        sem.try_acquire(7000),
        AcquireResult::Acquired { permit_id: 4 }
    );

    assert_eq!(sem.total_acquired(), 4);
    assert_eq!(sem.total_rejected(), 1);
}

#[test]
fn adversarial_fair_priority_ordering() {
    let mut fs = FairSemaphore::new(1);

    // Enqueue with different priorities
    let _a = fs.enqueue_with_priority(100, 5); // low priority
    let _b = fs.enqueue_with_priority(200, 1); // high priority
    let _c = fs.enqueue_with_priority(300, 1); // high priority, later

    // Grant order should be B (priority 1, earliest) → C (priority 1, later) → A (priority 5)
    assert_eq!(fs.try_grant_next(), Some(2)); // B
    fs.release_one();
    assert_eq!(fs.try_grant_next(), Some(3)); // C
    fs.release_one();
    assert_eq!(fs.try_grant_next(), Some(1)); // A
    fs.release_one();

    assert_eq!(fs.total_granted(), 3);
    assert_eq!(fs.queue_len(), 0);
    assert_eq!(fs.held_count(), 0);
}
