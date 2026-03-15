//! Invoice system — generation, line items, tax calculation,
//! and payment status tracking.

use aurora_core::types::EntityId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

// ---------------------------------------------------------------------------
// Invoice types
// ---------------------------------------------------------------------------

/// An invoice for billing purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: EntityId,
    pub invoice_number: String,
    pub account_id: EntityId,
    pub line_items: Vec<LineItem>,
    pub subtotal_cents: i64,
    pub tax_cents: i64,
    pub total_cents: i64,
    pub currency: String,
    pub status: InvoiceStatus,
    pub due_date: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// A line item on an invoice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItem {
    pub description: String,
    pub quantity: u32,
    pub unit_price_cents: i64,
    pub total_cents: i64,
}

/// Invoice status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvoiceStatus {
    Draft,
    Issued,
    Paid,
    Overdue,
    Void,
}

// ---------------------------------------------------------------------------
// Invoice manager
// ---------------------------------------------------------------------------

/// Manages invoice generation and tracking.
pub struct InvoiceManager {
    invoices: HashMap<EntityId, Invoice>,
    next_number: u64,
    tax_rate: f64,
}

impl InvoiceManager {
    pub fn new(tax_rate: f64) -> Self {
        Self {
            invoices: HashMap::new(),
            next_number: 1000,
            tax_rate,
        }
    }

    /// Create a new draft invoice.
    pub fn create(
        &mut self,
        account_id: EntityId,
        currency: impl Into<String>,
        due_date: DateTime<Utc>,
    ) -> Invoice {
        let number = format!("INV-{:06}", self.next_number);
        self.next_number += 1;

        let invoice = Invoice {
            id: EntityId::new(),
            invoice_number: number,
            account_id,
            line_items: Vec::new(),
            subtotal_cents: 0,
            tax_cents: 0,
            total_cents: 0,
            currency: currency.into(),
            status: InvoiceStatus::Draft,
            due_date,
            paid_at: None,
            created_at: Utc::now(),
        };

        let result = invoice.clone();
        self.invoices.insert(invoice.id, invoice);
        result
    }

    /// Add a line item to a draft invoice.
    pub fn add_line_item(
        &mut self,
        invoice_id: &EntityId,
        description: impl Into<String>,
        quantity: u32,
        unit_price_cents: i64,
    ) -> Result<(), InvoiceError> {
        let invoice = self
            .invoices
            .get_mut(invoice_id)
            .ok_or(InvoiceError::NotFound(*invoice_id))?;

        if invoice.status != InvoiceStatus::Draft {
            return Err(InvoiceError::NotDraft(*invoice_id));
        }

        let line_total = unit_price_cents * quantity as i64;
        invoice.line_items.push(LineItem {
            description: description.into(),
            quantity,
            unit_price_cents,
            total_cents: line_total,
        });

        self.recalculate(invoice_id);
        Ok(())
    }

    /// Recalculate totals for an invoice.
    fn recalculate(&mut self, invoice_id: &EntityId) {
        if let Some(invoice) = self.invoices.get_mut(invoice_id) {
            invoice.subtotal_cents = invoice.line_items.iter().map(|li| li.total_cents).sum();
            invoice.tax_cents = (invoice.subtotal_cents as f64 * self.tax_rate) as i64;
            invoice.total_cents = invoice.subtotal_cents + invoice.tax_cents;
        }
    }

    /// Issue a draft invoice (make it payable).
    pub fn issue(&mut self, invoice_id: &EntityId) -> Result<(), InvoiceError> {
        let invoice = self
            .invoices
            .get_mut(invoice_id)
            .ok_or(InvoiceError::NotFound(*invoice_id))?;

        if invoice.status != InvoiceStatus::Draft {
            return Err(InvoiceError::NotDraft(*invoice_id));
        }

        if invoice.line_items.is_empty() {
            return Err(InvoiceError::NoLineItems);
        }

        invoice.status = InvoiceStatus::Issued;
        info!(invoice = %invoice.invoice_number, total = invoice.total_cents, "invoice issued");
        Ok(())
    }

    /// Mark an invoice as paid.
    pub fn mark_paid(&mut self, invoice_id: &EntityId) -> Result<i64, InvoiceError> {
        let invoice = self
            .invoices
            .get_mut(invoice_id)
            .ok_or(InvoiceError::NotFound(*invoice_id))?;

        if invoice.status != InvoiceStatus::Issued && invoice.status != InvoiceStatus::Overdue {
            return Err(InvoiceError::CannotPay(invoice.status));
        }

        invoice.status = InvoiceStatus::Paid;
        invoice.paid_at = Some(Utc::now());
        Ok(invoice.total_cents)
    }

    /// Void an invoice.
    pub fn void(&mut self, invoice_id: &EntityId) -> Result<(), InvoiceError> {
        let invoice = self
            .invoices
            .get_mut(invoice_id)
            .ok_or(InvoiceError::NotFound(*invoice_id))?;

        if invoice.status == InvoiceStatus::Paid {
            return Err(InvoiceError::AlreadyPaid);
        }

        invoice.status = InvoiceStatus::Void;
        Ok(())
    }

    /// Mark overdue invoices.
    pub fn mark_overdue(&mut self) -> usize {
        let now = Utc::now();
        let overdue_ids: Vec<EntityId> = self
            .invoices
            .values()
            .filter(|inv| inv.status == InvoiceStatus::Issued && inv.due_date < now)
            .map(|inv| inv.id)
            .collect();

        let count = overdue_ids.len();
        for id in overdue_ids {
            if let Some(inv) = self.invoices.get_mut(&id) {
                inv.status = InvoiceStatus::Overdue;
            }
        }
        count
    }

    /// Get an invoice.
    pub fn get(&self, invoice_id: &EntityId) -> Option<&Invoice> {
        self.invoices.get(invoice_id)
    }

    /// List invoices for an account.
    pub fn for_account(&self, account_id: &EntityId) -> Vec<&Invoice> {
        self.invoices
            .values()
            .filter(|inv| inv.account_id == *account_id)
            .collect()
    }

    /// Total outstanding (issued + overdue).
    pub fn total_outstanding_cents(&self) -> i64 {
        self.invoices
            .values()
            .filter(|inv| {
                inv.status == InvoiceStatus::Issued || inv.status == InvoiceStatus::Overdue
            })
            .map(|inv| inv.total_cents)
            .sum()
    }

    /// Total invoices count.
    pub fn total_invoices(&self) -> usize {
        self.invoices.len()
    }
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum InvoiceError {
    #[error("invoice not found: {0}")]
    NotFound(EntityId),
    #[error("invoice {0} is not in draft state")]
    NotDraft(EntityId),
    #[error("cannot add line items: invoice has no items")]
    NoLineItems,
    #[error("cannot pay invoice in status {0:?}")]
    CannotPay(InvoiceStatus),
    #[error("invoice already paid")]
    AlreadyPaid,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn test_mgr() -> InvoiceManager {
        InvoiceManager::new(0.10) // 10% tax
    }

    #[test]
    fn create_and_add_items() {
        let mut mgr = test_mgr();
        let due = Utc::now() + Duration::days(30);
        let inv = mgr.create(EntityId::new(), "USD", due);
        assert_eq!(inv.status, InvoiceStatus::Draft);
        assert!(inv.invoice_number.starts_with("INV-"));

        mgr.add_line_item(&inv.id, "Pro Plan - Monthly", 1, 4999)
            .unwrap();
        mgr.add_line_item(&inv.id, "Extra API Calls", 1000, 1)
            .unwrap();

        let i = mgr.get(&inv.id).unwrap();
        assert_eq!(i.subtotal_cents, 5999); // 4999 + 1000
        assert_eq!(i.tax_cents, 599); // 10% of 5999
        assert_eq!(i.total_cents, 6598);
    }

    #[test]
    fn issue_invoice() {
        let mut mgr = test_mgr();
        let due = Utc::now() + Duration::days(30);
        let inv = mgr.create(EntityId::new(), "USD", due);
        mgr.add_line_item(&inv.id, "Plan", 1, 1000).unwrap();
        mgr.issue(&inv.id).unwrap();
        assert_eq!(mgr.get(&inv.id).unwrap().status, InvoiceStatus::Issued);
    }

    #[test]
    fn issue_empty_invoice_fails() {
        let mut mgr = test_mgr();
        let inv = mgr.create(EntityId::new(), "USD", Utc::now());
        let result = mgr.issue(&inv.id);
        assert!(matches!(result.unwrap_err(), InvoiceError::NoLineItems));
    }

    #[test]
    fn pay_invoice() {
        let mut mgr = test_mgr();
        let inv = mgr.create(EntityId::new(), "USD", Utc::now() + Duration::days(30));
        mgr.add_line_item(&inv.id, "Plan", 1, 5000).unwrap();
        mgr.issue(&inv.id).unwrap();
        let total = mgr.mark_paid(&inv.id).unwrap();
        assert_eq!(total, 5500); // 5000 + 10% tax
        assert_eq!(mgr.get(&inv.id).unwrap().status, InvoiceStatus::Paid);
        assert!(mgr.get(&inv.id).unwrap().paid_at.is_some());
    }

    #[test]
    fn pay_draft_fails() {
        let mut mgr = test_mgr();
        let inv = mgr.create(EntityId::new(), "USD", Utc::now());
        mgr.add_line_item(&inv.id, "x", 1, 100).unwrap();
        let result = mgr.mark_paid(&inv.id);
        assert!(result.is_err());
    }

    #[test]
    fn void_invoice() {
        let mut mgr = test_mgr();
        let inv = mgr.create(EntityId::new(), "USD", Utc::now());
        mgr.add_line_item(&inv.id, "x", 1, 100).unwrap();
        mgr.issue(&inv.id).unwrap();
        mgr.void(&inv.id).unwrap();
        assert_eq!(mgr.get(&inv.id).unwrap().status, InvoiceStatus::Void);
    }

    #[test]
    fn void_paid_fails() {
        let mut mgr = test_mgr();
        let inv = mgr.create(EntityId::new(), "USD", Utc::now() + Duration::days(30));
        mgr.add_line_item(&inv.id, "x", 1, 100).unwrap();
        mgr.issue(&inv.id).unwrap();
        mgr.mark_paid(&inv.id).unwrap();
        assert!(mgr.void(&inv.id).is_err());
    }

    #[test]
    fn overdue_detection() {
        let mut mgr = test_mgr();
        let past = Utc::now() - Duration::days(1);
        let inv = mgr.create(EntityId::new(), "USD", past);
        mgr.add_line_item(&inv.id, "x", 1, 100).unwrap();
        mgr.issue(&inv.id).unwrap();
        let count = mgr.mark_overdue();
        assert_eq!(count, 1);
        assert_eq!(mgr.get(&inv.id).unwrap().status, InvoiceStatus::Overdue);
    }

    #[test]
    fn total_outstanding() {
        let mut mgr = test_mgr();
        let due = Utc::now() + Duration::days(30);
        let i1 = mgr.create(EntityId::new(), "USD", due);
        mgr.add_line_item(&i1.id, "a", 1, 1000).unwrap();
        mgr.issue(&i1.id).unwrap();

        let i2 = mgr.create(EntityId::new(), "USD", due);
        mgr.add_line_item(&i2.id, "b", 1, 2000).unwrap();
        mgr.issue(&i2.id).unwrap();

        // 1000*1.1 + 2000*1.1 = 1100 + 2200 = 3300
        assert_eq!(mgr.total_outstanding_cents(), 3300);
    }

    #[test]
    fn invoice_number_sequential() {
        let mut mgr = test_mgr();
        let i1 = mgr.create(EntityId::new(), "USD", Utc::now());
        let i2 = mgr.create(EntityId::new(), "USD", Utc::now());
        assert_eq!(i1.invoice_number, "INV-001000");
        assert_eq!(i2.invoice_number, "INV-001001");
    }

    #[test]
    fn add_item_to_issued_fails() {
        let mut mgr = test_mgr();
        let inv = mgr.create(EntityId::new(), "USD", Utc::now() + Duration::days(30));
        mgr.add_line_item(&inv.id, "x", 1, 100).unwrap();
        mgr.issue(&inv.id).unwrap();
        let result = mgr.add_line_item(&inv.id, "y", 1, 200);
        assert!(matches!(result.unwrap_err(), InvoiceError::NotDraft(_)));
    }
}
