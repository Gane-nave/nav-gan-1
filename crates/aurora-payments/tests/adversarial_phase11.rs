//! Adversarial integration tests for Phase 11 payment features.

use aurora_core::types::EntityId;
use aurora_payments::billing::{AccountType, BillingManager, PaymentMethodType};

#[test]
fn adversarial_billing_full_lifecycle() {
    let mut mgr = BillingManager::new();
    let owner = EntityId::new();

    // 1. Create account → balance 0
    let account = mgr
        .create_account(owner, AccountType::Organisation, "USD")
        .unwrap();
    assert_eq!(account.balance_cents, 0, "new account balance must be 0");

    // 2. Add payment method → first is default
    mgr.add_payment_method(
        &account.id,
        PaymentMethodType::CreditCard,
        "Visa",
        "4242",
        12,
        2028,
    )
    .unwrap();
    let acc = mgr.get(&account.id).unwrap();
    assert!(
        acc.default_payment_method.is_some(),
        "first payment method must be default"
    );

    // 3. Add credit → balance increases
    mgr.add_credit(&account.id, 10000, "Top up").unwrap();
    assert_eq!(
        mgr.get(&account.id).unwrap().balance_cents,
        10000,
        "balance after credit"
    );

    // 4. Charge → balance decreases
    mgr.charge(&account.id, 3500, "Plugin purchase").unwrap();
    assert_eq!(
        mgr.get(&account.id).unwrap().balance_cents,
        6500,
        "balance after charge: 10000-3500=6500"
    );

    // 5. Refund → balance increases
    mgr.refund(&account.id, 1500, "Partial refund").unwrap();
    assert_eq!(
        mgr.get(&account.id).unwrap().balance_cents,
        8000,
        "balance after refund: 6500+1500=8000"
    );

    // 6. Transaction history = 3 (credit, charge, refund)
    assert_eq!(
        mgr.transactions_for(&account.id).len(),
        3,
        "must have 3 transactions"
    );

    // 7. Total revenue = charge amount only
    assert_eq!(
        mgr.total_revenue_cents(),
        3500,
        "revenue = charges only = 3500"
    );
}
