//! Adversarial tests for gane-lease

use gane_lease::manager::{LeaseManager, LeaseResult};

#[test]
fn adversarial_conflict_expiry_max_renewals() {
    let mut mgr = LeaseManager::new(2);

    // Acquire by node-a
    assert_eq!(
        mgr.acquire("gps", "node-a", 1000, 5000),
        LeaseResult::Granted { lease_id: 1 }
    );

    // Conflict: different holder on active lease
    assert_eq!(
        mgr.acquire("gps", "node-b", 2000, 5000),
        LeaseResult::Conflict {
            current_holder: "node-a".to_string()
        }
    );
    assert_eq!(mgr.total_conflicts(), 1);

    // Same holder renews via acquire
    assert_eq!(
        mgr.acquire("gps", "node-a", 3000, 5000),
        LeaseResult::Granted { lease_id: 1 }
    );

    // Renew by ID — renewal #2
    assert!(mgr.renew(1, 7000, 5000));

    // Renew by ID — should FAIL (max_renewals=2 reached)
    assert!(!mgr.renew(1, 8000, 5000));

    // After expiry (renewed at 7000 + 5000 = 12000), at t=13000 it's expired
    assert!(!mgr.is_leased("gps", 13000));

    // New holder can acquire after expiry
    assert_eq!(
        mgr.acquire("gps", "node-b", 14000, 5000),
        LeaseResult::Granted { lease_id: 2 }
    );
    assert_eq!(mgr.holder_of("gps", 15000), Some("node-b"));
}

#[test]
fn adversarial_revoke_and_cleanup() {
    let mut mgr = LeaseManager::new(0);

    mgr.acquire("r1", "h", 1000, 2000);
    mgr.acquire("r2", "h", 1000, 10000);
    mgr.acquire("r3", "h", 1000, 4000);

    assert_eq!(mgr.active_count(2000), 3);

    // Revoke r2
    assert!(mgr.revoke(2));
    assert_eq!(mgr.active_count(2000), 2);
    assert_eq!(mgr.total_revoked(), 1);

    // At t=5000: r1 expired (1000+2000=3000), r3 expired (1000+4000=5000), r2 revoked
    assert_eq!(mgr.active_count(5000), 0);

    // Cleanup removes all expired/revoked
    mgr.cleanup(5000);
    assert_eq!(mgr.total_leases(), 0);
}
