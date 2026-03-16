//! Payment processor — transaction processing, gateway abstraction,
//! retry logic, and settlement tracking.

use aurora_core::types::EntityId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

// ---------------------------------------------------------------------------
// Payment types
// ---------------------------------------------------------------------------

/// A payment transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: EntityId,
    pub account_id: EntityId,
    pub amount_cents: u64,
    pub currency: String,
    pub payment_method_id: EntityId,
    pub gateway: PaymentGateway,
    pub status: PaymentStatus,
    pub gateway_reference: Option<String>,
    pub attempts: u32,
    pub max_attempts: u32,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Supported payment gateways.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentGateway {
    Stripe,
    PayPal,
    BankTransfer,
    Crypto,
    Internal,
}

/// Payment status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentStatus {
    /// Waiting to be processed.
    Pending,
    /// Being processed by gateway.
    Processing,
    /// Successfully completed.
    Completed,
    /// Failed after all retries.
    Failed,
    /// Refunded.
    Refunded,
    /// Disputed by cardholder.
    Disputed,
}

/// Settlement batch for transferring funds to sellers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settlement {
    pub id: EntityId,
    pub recipient_id: EntityId,
    pub payment_ids: Vec<EntityId>,
    pub total_cents: u64,
    pub platform_fee_cents: u64,
    pub net_cents: u64,
    pub currency: String,
    pub status: SettlementStatus,
    pub created_at: DateTime<Utc>,
}

/// Settlement status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettlementStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

// ---------------------------------------------------------------------------
// Payment processor
// ---------------------------------------------------------------------------

/// Processes payments and manages settlements.
pub struct PaymentProcessor {
    payments: HashMap<EntityId, Payment>,
    settlements: Vec<Settlement>,
    platform_fee_rate: f64,
    default_max_attempts: u32,
}

impl PaymentProcessor {
    pub fn new(platform_fee_rate: f64, default_max_attempts: u32) -> Self {
        Self {
            payments: HashMap::new(),
            settlements: Vec::new(),
            platform_fee_rate,
            default_max_attempts,
        }
    }

    /// Create a payment.
    pub fn create_payment(
        &mut self,
        account_id: EntityId,
        amount_cents: u64,
        currency: impl Into<String>,
        payment_method_id: EntityId,
        gateway: PaymentGateway,
    ) -> Payment {
        let payment = Payment {
            id: EntityId::new(),
            account_id,
            amount_cents,
            currency: currency.into(),
            payment_method_id,
            gateway,
            status: PaymentStatus::Pending,
            gateway_reference: None,
            attempts: 0,
            max_attempts: self.default_max_attempts,
            error_message: None,
            created_at: Utc::now(),
            completed_at: None,
        };

        let result = payment.clone();
        self.payments.insert(payment.id, payment);
        result
    }

    /// Process a pending payment (simulate gateway interaction).
    pub fn process(
        &mut self,
        payment_id: &EntityId,
        success: bool,
        gateway_ref: Option<String>,
    ) -> Result<PaymentStatus, PaymentError> {
        let payment = self
            .payments
            .get_mut(payment_id)
            .ok_or(PaymentError::NotFound(*payment_id))?;

        if payment.status != PaymentStatus::Pending && payment.status != PaymentStatus::Processing {
            return Err(PaymentError::InvalidStatus(payment.status));
        }

        payment.attempts += 1;
        payment.status = PaymentStatus::Processing;

        if success {
            payment.status = PaymentStatus::Completed;
            payment.gateway_reference = gateway_ref;
            payment.completed_at = Some(Utc::now());
            info!(payment_id = %payment_id, amount = payment.amount_cents, "payment completed");
        } else if payment.attempts >= payment.max_attempts {
            payment.status = PaymentStatus::Failed;
            payment.error_message = Some("Max attempts exceeded".into());
        } else {
            // Back to pending for retry.
            payment.status = PaymentStatus::Pending;
            payment.error_message = Some("Payment declined, will retry".into());
        }

        Ok(payment.status)
    }

    /// Refund a completed payment.
    pub fn refund(&mut self, payment_id: &EntityId) -> Result<(), PaymentError> {
        let payment = self
            .payments
            .get_mut(payment_id)
            .ok_or(PaymentError::NotFound(*payment_id))?;

        if payment.status != PaymentStatus::Completed {
            return Err(PaymentError::NotCompleted);
        }

        payment.status = PaymentStatus::Refunded;
        info!(payment_id = %payment_id, "payment refunded");
        Ok(())
    }

    /// Create a settlement for a recipient from completed payments.
    pub fn settle(
        &mut self,
        recipient_id: EntityId,
        payment_ids: Vec<EntityId>,
        currency: impl Into<String>,
    ) -> Result<Settlement, PaymentError> {
        let mut total = 0u64;
        for pid in &payment_ids {
            let payment = self.payments.get(pid).ok_or(PaymentError::NotFound(*pid))?;

            if payment.status != PaymentStatus::Completed {
                return Err(PaymentError::NotCompleted);
            }
            total += payment.amount_cents;
        }

        let fee = (total as f64 * self.platform_fee_rate) as u64;
        let net = total - fee;

        let settlement = Settlement {
            id: EntityId::new(),
            recipient_id,
            payment_ids,
            total_cents: total,
            platform_fee_cents: fee,
            net_cents: net,
            currency: currency.into(),
            status: SettlementStatus::Pending,
            created_at: Utc::now(),
        };

        info!(settlement_id = %settlement.id, total, fee, net, "settlement created");
        let result = settlement.clone();
        self.settlements.push(settlement);
        Ok(result)
    }

    /// Get a payment by ID.
    pub fn get(&self, payment_id: &EntityId) -> Option<&Payment> {
        self.payments.get(payment_id)
    }

    /// Payments for an account.
    pub fn for_account(&self, account_id: &EntityId) -> Vec<&Payment> {
        self.payments
            .values()
            .filter(|p| p.account_id == *account_id)
            .collect()
    }

    /// Total processed (completed payments).
    pub fn total_processed_cents(&self) -> u64 {
        self.payments
            .values()
            .filter(|p| p.status == PaymentStatus::Completed)
            .map(|p| p.amount_cents)
            .sum()
    }

    /// Pending payments needing processing.
    pub fn pending(&self) -> Vec<&Payment> {
        self.payments
            .values()
            .filter(|p| p.status == PaymentStatus::Pending)
            .collect()
    }

    /// Total settlements.
    pub fn total_settlements(&self) -> usize {
        self.settlements.len()
    }

    /// Total platform fees earned.
    pub fn total_fees_cents(&self) -> u64 {
        self.settlements.iter().map(|s| s.platform_fee_cents).sum()
    }
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum PaymentError {
    #[error("payment not found: {0}")]
    NotFound(EntityId),
    #[error("invalid payment status: {0:?}")]
    InvalidStatus(PaymentStatus),
    #[error("payment not completed")]
    NotCompleted,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn test_processor() -> PaymentProcessor {
        PaymentProcessor::new(0.15, 3) // 15% fee, 3 attempts
    }

    #[test]
    fn create_and_process_payment() {
        let mut proc = test_processor();
        let p = proc.create_payment(
            EntityId::new(),
            5000,
            "USD",
            EntityId::new(),
            PaymentGateway::Stripe,
        );
        assert_eq!(p.status, PaymentStatus::Pending);

        let status = proc.process(&p.id, true, Some("ch_123".into())).unwrap();
        assert_eq!(status, PaymentStatus::Completed);
        let payment = proc.get(&p.id).unwrap();
        assert_eq!(payment.gateway_reference, Some("ch_123".into()));
        assert_eq!(payment.attempts, 1);
    }

    #[test]
    fn retry_on_failure() {
        let mut proc = test_processor();
        let p = proc.create_payment(
            EntityId::new(),
            1000,
            "USD",
            EntityId::new(),
            PaymentGateway::Stripe,
        );

        // First attempt fails.
        let s1 = proc.process(&p.id, false, None).unwrap();
        assert_eq!(s1, PaymentStatus::Pending); // retryable

        // Second attempt fails.
        let s2 = proc.process(&p.id, false, None).unwrap();
        assert_eq!(s2, PaymentStatus::Pending); // retryable

        // Third attempt fails (max_attempts=3).
        let s3 = proc.process(&p.id, false, None).unwrap();
        assert_eq!(s3, PaymentStatus::Failed); // final failure
    }

    #[test]
    fn refund() {
        let mut proc = test_processor();
        let p = proc.create_payment(
            EntityId::new(),
            1000,
            "USD",
            EntityId::new(),
            PaymentGateway::Stripe,
        );
        proc.process(&p.id, true, None).unwrap();
        proc.refund(&p.id).unwrap();
        assert_eq!(proc.get(&p.id).unwrap().status, PaymentStatus::Refunded);
    }

    #[test]
    fn refund_pending_fails() {
        let mut proc = test_processor();
        let p = proc.create_payment(
            EntityId::new(),
            1000,
            "USD",
            EntityId::new(),
            PaymentGateway::Stripe,
        );
        assert!(proc.refund(&p.id).is_err());
    }

    #[test]
    fn settlement() {
        let mut proc = test_processor();
        let p1 = proc.create_payment(
            EntityId::new(),
            10000,
            "USD",
            EntityId::new(),
            PaymentGateway::Stripe,
        );
        let p2 = proc.create_payment(
            EntityId::new(),
            5000,
            "USD",
            EntityId::new(),
            PaymentGateway::Stripe,
        );
        proc.process(&p1.id, true, None).unwrap();
        proc.process(&p2.id, true, None).unwrap();

        let recipient = EntityId::new();
        let settlement = proc.settle(recipient, vec![p1.id, p2.id], "USD").unwrap();
        assert_eq!(settlement.total_cents, 15000);
        assert_eq!(settlement.platform_fee_cents, 2250); // 15%
        assert_eq!(settlement.net_cents, 12750);
    }

    #[test]
    fn settle_incomplete_payment_fails() {
        let mut proc = test_processor();
        let p = proc.create_payment(
            EntityId::new(),
            1000,
            "USD",
            EntityId::new(),
            PaymentGateway::Stripe,
        );
        let result = proc.settle(EntityId::new(), vec![p.id], "USD");
        assert!(result.is_err());
    }

    #[test]
    fn total_processed() {
        let mut proc = test_processor();
        let p1 = proc.create_payment(
            EntityId::new(),
            1000,
            "USD",
            EntityId::new(),
            PaymentGateway::Stripe,
        );
        let p2 = proc.create_payment(
            EntityId::new(),
            2000,
            "USD",
            EntityId::new(),
            PaymentGateway::PayPal,
        );
        proc.process(&p1.id, true, None).unwrap();
        proc.process(&p2.id, true, None).unwrap();
        assert_eq!(proc.total_processed_cents(), 3000);
    }

    #[test]
    fn pending_list() {
        let mut proc = test_processor();
        proc.create_payment(
            EntityId::new(),
            100,
            "USD",
            EntityId::new(),
            PaymentGateway::Stripe,
        );
        proc.create_payment(
            EntityId::new(),
            200,
            "USD",
            EntityId::new(),
            PaymentGateway::Stripe,
        );
        assert_eq!(proc.pending().len(), 2);
    }

    #[test]
    fn platform_fees() {
        let mut proc = test_processor();
        let p = proc.create_payment(
            EntityId::new(),
            10000,
            "USD",
            EntityId::new(),
            PaymentGateway::Stripe,
        );
        proc.process(&p.id, true, None).unwrap();
        proc.settle(EntityId::new(), vec![p.id], "USD").unwrap();
        assert_eq!(proc.total_fees_cents(), 1500);
    }
}
