//! Billing system — account billing management, payment methods,
//! balance tracking, and billing history.

use aurora_core::types::EntityId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

// ---------------------------------------------------------------------------
// Billing types
// ---------------------------------------------------------------------------

/// A billing account for a user or organisation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingAccount {
    pub id: EntityId,
    pub owner_id: EntityId,
    pub account_type: AccountType,
    pub balance_cents: i64,
    pub currency: String,
    pub payment_methods: Vec<PaymentMethod>,
    pub default_payment_method: Option<EntityId>,
    pub status: BillingStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Account type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountType {
    Individual,
    Organisation,
    Enterprise,
}

/// Billing account status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingStatus {
    Active,
    Suspended,
    Closed,
}

/// A stored payment method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub id: EntityId,
    pub method_type: PaymentMethodType,
    pub label: String,
    pub last_four: String,
    pub expiry_month: u8,
    pub expiry_year: u16,
    pub is_default: bool,
    pub added_at: DateTime<Utc>,
}

/// Payment method type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentMethodType {
    CreditCard,
    DebitCard,
    BankTransfer,
    PayPal,
    Crypto,
}

/// A billing transaction record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: EntityId,
    pub account_id: EntityId,
    pub amount_cents: i64,
    pub currency: String,
    pub transaction_type: TransactionType,
    pub description: String,
    pub status: TransactionStatus,
    pub created_at: DateTime<Utc>,
}

/// Transaction type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    Charge,
    Refund,
    Credit,
    Payout,
}

/// Transaction status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Reversed,
}

// ---------------------------------------------------------------------------
// Billing manager
// ---------------------------------------------------------------------------

/// Manages billing accounts and transactions.
pub struct BillingManager {
    accounts: HashMap<EntityId, BillingAccount>,
    /// owner_id → account_id
    owner_index: HashMap<EntityId, EntityId>,
    transactions: Vec<Transaction>,
}

impl BillingManager {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            owner_index: HashMap::new(),
            transactions: Vec::new(),
        }
    }

    /// Create a billing account for an owner.
    pub fn create_account(
        &mut self,
        owner_id: EntityId,
        account_type: AccountType,
        currency: impl Into<String>,
    ) -> Result<BillingAccount, BillingError> {
        if self.owner_index.contains_key(&owner_id) {
            return Err(BillingError::AccountExists(owner_id));
        }

        let now = Utc::now();
        let account = BillingAccount {
            id: EntityId::new(),
            owner_id,
            account_type,
            balance_cents: 0,
            currency: currency.into(),
            payment_methods: Vec::new(),
            default_payment_method: None,
            status: BillingStatus::Active,
            created_at: now,
            updated_at: now,
        };

        info!(account_id = %account.id, owner = %owner_id, "billing account created");
        self.owner_index.insert(owner_id, account.id);
        let result = account.clone();
        self.accounts.insert(account.id, account);
        Ok(result)
    }

    /// Add a payment method to an account.
    pub fn add_payment_method(
        &mut self,
        account_id: &EntityId,
        method_type: PaymentMethodType,
        label: impl Into<String>,
        last_four: impl Into<String>,
        expiry_month: u8,
        expiry_year: u16,
    ) -> Result<EntityId, BillingError> {
        let account = self
            .accounts
            .get_mut(account_id)
            .ok_or(BillingError::AccountNotFound(*account_id))?;

        let is_first = account.payment_methods.is_empty();
        let method_id = EntityId::new();

        account.payment_methods.push(PaymentMethod {
            id: method_id,
            method_type,
            label: label.into(),
            last_four: last_four.into(),
            expiry_month,
            expiry_year,
            is_default: is_first,
            added_at: Utc::now(),
        });

        if is_first {
            account.default_payment_method = Some(method_id);
        }

        account.updated_at = Utc::now();
        Ok(method_id)
    }

    /// Charge an account (deduct from balance or create pending charge).
    pub fn charge(
        &mut self,
        account_id: &EntityId,
        amount_cents: u64,
        description: impl Into<String>,
    ) -> Result<Transaction, BillingError> {
        let account = self
            .accounts
            .get_mut(account_id)
            .ok_or(BillingError::AccountNotFound(*account_id))?;

        if account.status != BillingStatus::Active {
            return Err(BillingError::AccountNotActive);
        }

        account.balance_cents -= amount_cents as i64;
        account.updated_at = Utc::now();

        let tx = Transaction {
            id: EntityId::new(),
            account_id: *account_id,
            amount_cents: amount_cents as i64,
            currency: account.currency.clone(),
            transaction_type: TransactionType::Charge,
            description: description.into(),
            status: TransactionStatus::Completed,
            created_at: Utc::now(),
        };

        self.transactions.push(tx.clone());
        Ok(tx)
    }

    /// Add credit to an account.
    pub fn add_credit(
        &mut self,
        account_id: &EntityId,
        amount_cents: u64,
        description: impl Into<String>,
    ) -> Result<Transaction, BillingError> {
        let account = self
            .accounts
            .get_mut(account_id)
            .ok_or(BillingError::AccountNotFound(*account_id))?;

        account.balance_cents += amount_cents as i64;
        account.updated_at = Utc::now();

        let tx = Transaction {
            id: EntityId::new(),
            account_id: *account_id,
            amount_cents: amount_cents as i64,
            currency: account.currency.clone(),
            transaction_type: TransactionType::Credit,
            description: description.into(),
            status: TransactionStatus::Completed,
            created_at: Utc::now(),
        };

        self.transactions.push(tx.clone());
        Ok(tx)
    }

    /// Refund a charge.
    pub fn refund(
        &mut self,
        account_id: &EntityId,
        amount_cents: u64,
        description: impl Into<String>,
    ) -> Result<Transaction, BillingError> {
        let account = self
            .accounts
            .get_mut(account_id)
            .ok_or(BillingError::AccountNotFound(*account_id))?;

        account.balance_cents += amount_cents as i64;
        account.updated_at = Utc::now();

        let tx = Transaction {
            id: EntityId::new(),
            account_id: *account_id,
            amount_cents: amount_cents as i64,
            currency: account.currency.clone(),
            transaction_type: TransactionType::Refund,
            description: description.into(),
            status: TransactionStatus::Completed,
            created_at: Utc::now(),
        };

        self.transactions.push(tx.clone());
        Ok(tx)
    }

    /// Suspend a billing account.
    pub fn suspend(&mut self, account_id: &EntityId) -> Result<(), BillingError> {
        let account = self
            .accounts
            .get_mut(account_id)
            .ok_or(BillingError::AccountNotFound(*account_id))?;
        account.status = BillingStatus::Suspended;
        account.updated_at = Utc::now();
        Ok(())
    }

    /// Get account by owner ID.
    pub fn for_owner(&self, owner_id: &EntityId) -> Option<&BillingAccount> {
        self.owner_index
            .get(owner_id)
            .and_then(|id| self.accounts.get(id))
    }

    /// Get account by ID.
    pub fn get(&self, account_id: &EntityId) -> Option<&BillingAccount> {
        self.accounts.get(account_id)
    }

    /// Transaction history for an account.
    pub fn transactions_for(&self, account_id: &EntityId) -> Vec<&Transaction> {
        self.transactions
            .iter()
            .filter(|t| t.account_id == *account_id)
            .collect()
    }

    /// Total revenue across all accounts.
    pub fn total_revenue_cents(&self) -> i64 {
        self.transactions
            .iter()
            .filter(|t| {
                t.transaction_type == TransactionType::Charge
                    && t.status == TransactionStatus::Completed
            })
            .map(|t| t.amount_cents)
            .sum()
    }
}

impl Default for BillingManager {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum BillingError {
    #[error("account not found: {0}")]
    AccountNotFound(EntityId),
    #[error("account already exists for owner: {0}")]
    AccountExists(EntityId),
    #[error("account is not active")]
    AccountNotActive,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn test_mgr() -> BillingManager {
        BillingManager::new()
    }

    fn create_test_account(mgr: &mut BillingManager) -> BillingAccount {
        mgr.create_account(EntityId::new(), AccountType::Individual, "USD")
            .unwrap()
    }

    #[test]
    fn create_account() {
        let mut mgr = test_mgr();
        let account = create_test_account(&mut mgr);
        assert_eq!(account.balance_cents, 0);
        assert_eq!(account.status, BillingStatus::Active);
    }

    #[test]
    fn duplicate_owner_rejected() {
        let mut mgr = test_mgr();
        let owner = EntityId::new();
        mgr.create_account(owner, AccountType::Individual, "USD")
            .unwrap();
        let result = mgr.create_account(owner, AccountType::Individual, "USD");
        assert!(matches!(
            result.unwrap_err(),
            BillingError::AccountExists(_)
        ));
    }

    #[test]
    fn add_payment_method_first_is_default() {
        let mut mgr = test_mgr();
        let account = create_test_account(&mut mgr);
        let pm_id = mgr
            .add_payment_method(
                &account.id,
                PaymentMethodType::CreditCard,
                "Visa",
                "4242",
                12,
                2028,
            )
            .unwrap();
        let acc = mgr.get(&account.id).unwrap();
        assert_eq!(acc.payment_methods.len(), 1);
        assert!(acc.payment_methods[0].is_default);
        assert_eq!(acc.default_payment_method, Some(pm_id));
    }

    #[test]
    fn charge_and_balance() {
        let mut mgr = test_mgr();
        let account = create_test_account(&mut mgr);
        mgr.add_credit(&account.id, 10000, "Top up").unwrap();
        mgr.charge(&account.id, 3500, "Plugin purchase").unwrap();
        let acc = mgr.get(&account.id).unwrap();
        assert_eq!(acc.balance_cents, 6500);
    }

    #[test]
    fn refund_restores_balance() {
        let mut mgr = test_mgr();
        let account = create_test_account(&mut mgr);
        mgr.add_credit(&account.id, 10000, "Top up").unwrap();
        mgr.charge(&account.id, 5000, "Purchase").unwrap();
        mgr.refund(&account.id, 5000, "Refund").unwrap();
        assert_eq!(mgr.get(&account.id).unwrap().balance_cents, 10000);
    }

    #[test]
    fn charge_suspended_fails() {
        let mut mgr = test_mgr();
        let account = create_test_account(&mut mgr);
        mgr.suspend(&account.id).unwrap();
        let result = mgr.charge(&account.id, 100, "test");
        assert!(matches!(
            result.unwrap_err(),
            BillingError::AccountNotActive
        ));
    }

    #[test]
    fn transaction_history() {
        let mut mgr = test_mgr();
        let account = create_test_account(&mut mgr);
        mgr.add_credit(&account.id, 1000, "c1").unwrap();
        mgr.charge(&account.id, 500, "ch1").unwrap();
        mgr.refund(&account.id, 200, "r1").unwrap();
        assert_eq!(mgr.transactions_for(&account.id).len(), 3);
    }

    #[test]
    fn total_revenue() {
        let mut mgr = test_mgr();
        let a1 = create_test_account(&mut mgr);
        let a2 = create_test_account(&mut mgr);
        mgr.add_credit(&a1.id, 10000, "t").unwrap();
        mgr.add_credit(&a2.id, 10000, "t").unwrap();
        mgr.charge(&a1.id, 3000, "p1").unwrap();
        mgr.charge(&a2.id, 7000, "p2").unwrap();
        assert_eq!(mgr.total_revenue_cents(), 10000);
    }

    #[test]
    fn for_owner() {
        let mut mgr = test_mgr();
        let owner = EntityId::new();
        mgr.create_account(owner, AccountType::Individual, "USD")
            .unwrap();
        assert!(mgr.for_owner(&owner).is_some());
        assert!(mgr.for_owner(&EntityId::new()).is_none());
    }
}
