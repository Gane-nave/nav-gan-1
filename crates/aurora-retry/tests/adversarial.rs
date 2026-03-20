//! Adversarial tests for aurora-retry

use aurora_retry::backoff::{add_jitter, BackoffIter, BackoffStrategy};
use aurora_retry::budget::RetryBudget;
use aurora_retry::policy::{ErrorKind, RetryDecision, RetryPolicy, RetryPolicyConfig};

#[test]
fn adversarial_exhaustion_reset_reuse_cycle() {
    let mut policy = RetryPolicy::new(RetryPolicyConfig {
        max_attempts: 2,
        backoff: BackoffStrategy::exponential(100, 2.0, 10_000),
        retry_on_timeout: true,
        retry_on_transient: true,
    });

    let d1 = policy.should_retry(ErrorKind::Transient);
    assert_eq!(d1, RetryDecision::RetryAfter { delay_ms: 100 });

    let d2 = policy.should_retry(ErrorKind::Transient);
    assert_eq!(d2, RetryDecision::RetryAfter { delay_ms: 200 });

    let d3 = policy.should_retry(ErrorKind::Transient);
    assert_eq!(d3, RetryDecision::Exhausted);
    assert_eq!(policy.total_exhausted(), 1);
    assert_eq!(policy.current_attempt(), 0);

    let d4 = policy.should_retry(ErrorKind::Transient);
    assert_eq!(d4, RetryDecision::RetryAfter { delay_ms: 100 });

    let d5 = policy.should_retry(ErrorKind::Permanent);
    assert_eq!(d5, RetryDecision::NoRetry);

    assert_eq!(policy.total_retries(), 3);
}

#[test]
fn adversarial_timeout_disabled() {
    let mut no_timeout = RetryPolicy::new(RetryPolicyConfig {
        max_attempts: 5,
        backoff: BackoffStrategy::constant(100),
        retry_on_timeout: false,
        retry_on_transient: true,
    });
    let dt = no_timeout.should_retry(ErrorKind::Timeout);
    assert_eq!(dt, RetryDecision::NoRetry);
}

#[test]
fn adversarial_backoff_overflow() {
    let exp = BackoffStrategy::exponential(100, 2.0, 50_000);
    let delay = exp.delay_ms(u32::MAX);
    assert!(delay <= 50_000, "Got {delay}");

    let lin = BackoffStrategy::linear(u64::MAX - 10, 100, u64::MAX);
    let delay2 = lin.delay_ms(u32::MAX);
    assert_eq!(delay2, u64::MAX);

    let con = BackoffStrategy::constant(42);
    assert_eq!(con.delay_ms(0), 42);
    assert_eq!(con.delay_ms(u32::MAX), 42);

    let iter = BackoffIter::new(BackoffStrategy::constant(100), 0);
    let delays: Vec<u64> = iter.collect();
    assert!(delays.is_empty());

    let j1 = add_jitter(1000, 2.0, 500);
    assert!((500..=1500).contains(&j1), "Got {j1}");

    let j2 = add_jitter(1, 0.5, 0);
    assert!(j2 <= 2, "Got {j2}");
}

#[test]
fn adversarial_budget_edge_cases() {
    // Zero budget should reject immediately
    let mut zero_budget = RetryBudget::new(0, 1000);
    assert!(!zero_budget.try_retry(100));
    assert_eq!(zero_budget.remaining(100), 0);

    // Normal budget usage
    let mut budget = RetryBudget::new(5, 10_000);
    assert!(budget.try_retry(100));
    assert!(budget.try_retry(200));
    assert_eq!(budget.used(), 2);
    assert_eq!(budget.remaining(300), 3);

    // Time-based expiration: retries at t=100,200 expire after window_ms=10000
    // At t=10200, both should be expired (10200-100=10100 >= 10000)
    assert_eq!(budget.remaining(10200), 5);
    assert!(budget.try_retry(10200)); // old ones expire internally
    assert_eq!(budget.used(), 1); // old two expired, one new

    // Reset clears all
    budget.reset();
    assert_eq!(budget.used(), 0);
    assert_eq!(budget.remaining(11000), 5);
    assert!(budget.try_retry(11000));

    // Utilization
    let mut util_budget = RetryBudget::new(4, 50_000);
    util_budget.try_retry(100);
    util_budget.try_retry(200);
    let util = util_budget.utilization();
    assert!((util - 0.5).abs() < 0.01, "Got {util}");
}
