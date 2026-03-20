use aurora_core::types::EntityId;

#[test]
fn adversarial_rotate_key_at_max_capacity() {
    use aurora_developer::apikey::*;

    let mut mgr = ApiKeyManager::new(2);
    let owner = EntityId::new();

    let (key1, secret1) = mgr
        .create_key(
            "key1",
            owner,
            vec![Permission::PositionRead],
            ApiTier::Developer,
            None,
        )
        .unwrap();
    let (_key2, secret2) = mgr
        .create_key(
            "key2",
            owner,
            vec![Permission::RouteRead],
            ApiTier::Developer,
            None,
        )
        .unwrap();
    assert_eq!(mgr.total_keys(), 2);

    // This is the critical test: rotate_key MUST succeed at max capacity
    let (_new_key, new_secret) = mgr.rotate_key(&key1.id).unwrap();
    assert_eq!(mgr.total_keys(), 3);

    let result = mgr.validate_key(&new_secret);
    assert!(result.valid, "rotated key must validate");

    let old_result = mgr.validate_key(&secret1);
    assert!(!old_result.valid, "revoked key must not validate");

    let k2_result = mgr.validate_key(&secret2);
    assert!(k2_result.valid, "key2 must still validate");
}

#[test]
fn adversarial_webhook_health_total_failure_count() {
    use aurora_developer::webhook::*;

    let mut mgr = WebhookManager::new(10);
    let owner = EntityId::new();

    let wh = mgr
        .register(
            owner,
            "test-hook",
            "https://example.com/hook",
            "secret",
            vec![WebhookEvent::PositionUpdate],
        )
        .unwrap();

    let ids1 = mgr.trigger(WebhookEvent::PositionUpdate, serde_json::json!({"test": 1}));
    assert_eq!(ids1.len(), 1);

    // Exhaust all 5 retry attempts
    for i in 0..5u32 {
        let status = mgr.process_delivery(&ids1[0], false, 500).unwrap();
        if i < 4 {
            assert_eq!(
                status,
                DeliveryStatus::Retrying,
                "attempt {} should retry",
                i + 1
            );
        } else {
            assert_eq!(
                status,
                DeliveryStatus::Failed,
                "attempt 5 should fail permanently"
            );
        }
    }

    let wh_after_fail = mgr.get(&wh.id).unwrap();
    assert_eq!(
        wh_after_fail.total_failure_count, 1,
        "total_failure_count must be 1"
    );
    assert_eq!(
        wh_after_fail.failure_count, 1,
        "consecutive failure_count must be 1"
    );

    // Now succeed once
    let ids2 = mgr.trigger(WebhookEvent::PositionUpdate, serde_json::json!({"test": 2}));
    let status = mgr.process_delivery(&ids2[0], true, 200).unwrap();
    assert_eq!(status, DeliveryStatus::Delivered);

    let wh_after_success = mgr.get(&wh.id).unwrap();
    assert_eq!(wh_after_success.success_count, 1, "success_count must be 1");
    assert_eq!(
        wh_after_success.failure_count, 0,
        "consecutive failure_count must be reset to 0"
    );
    assert_eq!(
        wh_after_success.total_failure_count, 1,
        "total_failure_count must still be 1"
    );

    // CRITICAL: health() must use total_failure_count
    let health = mgr.health(&wh.id).unwrap();
    assert_eq!(
        health.total_deliveries, 2,
        "total must be 2 (1 fail + 1 success)"
    );
    assert!(
        (health.success_rate - 0.5).abs() < 0.001,
        "success_rate must be 0.5, got {}",
        health.success_rate
    );
}

#[test]
fn adversarial_rate_limiting_token_bucket() {
    use aurora_developer::ratelimit::*;

    let mut limiter = RateLimiter::new(3, 1);
    let key = EntityId::new();

    assert!(
        limiter.try_acquire(&key).is_ok(),
        "request 1 must be allowed"
    );
    assert!(
        limiter.try_acquire(&key).is_ok(),
        "request 2 must be allowed"
    );
    assert!(
        limiter.try_acquire(&key).is_ok(),
        "request 3 must be allowed"
    );
    assert!(
        limiter.try_acquire(&key).is_err(),
        "request 4 must be denied (bucket empty)"
    );
}
