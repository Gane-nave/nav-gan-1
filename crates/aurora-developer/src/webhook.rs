//! Webhook system — registration, delivery, retry, and management
//! of outbound webhooks for notifying developers of platform events.

use aurora_core::types::EntityId;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};

// ---------------------------------------------------------------------------
// Webhook types
// ---------------------------------------------------------------------------

/// A registered webhook endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webhook {
    pub id: EntityId,
    pub owner_id: EntityId,
    pub name: String,
    pub url: String,
    pub secret: String,
    pub events: Vec<WebhookEvent>,
    pub status: WebhookStatus,
    pub created_at: DateTime<Utc>,
    pub last_triggered_at: Option<DateTime<Utc>>,
    pub success_count: u64,
    pub failure_count: u64,
}

/// Webhook status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WebhookStatus {
    /// Active and receiving events.
    Active,
    /// Paused by the owner.
    Paused,
    /// Disabled due to repeated failures.
    Disabled,
    /// Deleted (soft delete).
    Deleted,
}

/// Events that can trigger webhooks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WebhookEvent {
    /// Position update available.
    PositionUpdate,
    /// Route calculation completed.
    RouteCompleted,
    /// Integrity alert triggered.
    IntegrityAlert,
    /// SLA violation detected.
    SlaViolation,
    /// Fleet task status changed.
    TaskStatusChanged,
    /// Emergency corridor activated.
    EmergencyActivated,
    /// Infrastructure issue reported.
    InfrastructureIssue,
    /// API key rotated or revoked.
    ApiKeyChanged,
    /// Usage quota approaching limit.
    QuotaWarning,
    /// Traffic incident detected.
    TrafficIncident,
}

/// A queued webhook delivery attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookDelivery {
    pub id: EntityId,
    pub webhook_id: EntityId,
    pub event: WebhookEvent,
    pub payload: serde_json::Value,
    pub status: DeliveryStatus,
    pub attempts: u32,
    pub max_attempts: u32,
    pub next_retry_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub response_code: Option<u16>,
}

/// Delivery status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeliveryStatus {
    /// Queued for delivery.
    Pending,
    /// Successfully delivered.
    Delivered,
    /// Delivery failed, will retry.
    Retrying,
    /// All retry attempts exhausted.
    Failed,
    /// Cancelled (webhook deleted or disabled).
    Cancelled,
}

/// Webhook health summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookHealth {
    pub webhook_id: EntityId,
    pub name: String,
    pub status: WebhookStatus,
    pub success_rate: f64,
    pub total_deliveries: u64,
    pub pending_deliveries: usize,
    pub last_triggered_at: Option<DateTime<Utc>>,
}

// ---------------------------------------------------------------------------
// Webhook manager
// ---------------------------------------------------------------------------

/// Manages webhook registrations and delivery.
pub struct WebhookManager {
    webhooks: HashMap<EntityId, Webhook>,
    deliveries: Vec<WebhookDelivery>,
    max_webhooks_per_owner: usize,
    max_retry_attempts: u32,
    consecutive_failure_threshold: u64,
}

impl WebhookManager {
    /// Create a new webhook manager.
    pub fn new(max_webhooks_per_owner: usize) -> Self {
        Self {
            webhooks: HashMap::new(),
            deliveries: Vec::new(),
            max_webhooks_per_owner,
            max_retry_attempts: 5,
            consecutive_failure_threshold: 10,
        }
    }

    /// Register a new webhook.
    pub fn register(
        &mut self,
        owner_id: EntityId,
        name: impl Into<String>,
        url: impl Into<String>,
        secret: impl Into<String>,
        events: Vec<WebhookEvent>,
    ) -> Result<Webhook, WebhookError> {
        let owner_count = self
            .webhooks
            .values()
            .filter(|w| w.owner_id == owner_id && w.status != WebhookStatus::Deleted)
            .count();

        if owner_count >= self.max_webhooks_per_owner {
            return Err(WebhookError::TooManyWebhooks {
                max: self.max_webhooks_per_owner,
            });
        }

        let url_str = url.into();
        if !url_str.starts_with("https://") && !url_str.starts_with("http://localhost") {
            return Err(WebhookError::InsecureUrl);
        }

        if events.is_empty() {
            return Err(WebhookError::NoEvents);
        }

        let webhook = Webhook {
            id: EntityId::new(),
            owner_id,
            name: name.into(),
            url: url_str,
            secret: secret.into(),
            events,
            status: WebhookStatus::Active,
            created_at: Utc::now(),
            last_triggered_at: None,
            success_count: 0,
            failure_count: 0,
        };

        info!(webhook_id = %webhook.id, name = %webhook.name, "webhook registered");
        let result = webhook.clone();
        self.webhooks.insert(webhook.id, webhook);
        Ok(result)
    }

    /// Trigger an event — queues deliveries for all matching webhooks.
    pub fn trigger(&mut self, event: WebhookEvent, payload: serde_json::Value) -> Vec<EntityId> {
        let matching_ids: Vec<EntityId> = self
            .webhooks
            .values()
            .filter(|w| w.status == WebhookStatus::Active && w.events.contains(&event))
            .map(|w| w.id)
            .collect();

        let mut delivery_ids = Vec::new();

        for webhook_id in &matching_ids {
            let delivery = WebhookDelivery {
                id: EntityId::new(),
                webhook_id: *webhook_id,
                event,
                payload: payload.clone(),
                status: DeliveryStatus::Pending,
                attempts: 0,
                max_attempts: self.max_retry_attempts,
                next_retry_at: None,
                created_at: Utc::now(),
                completed_at: None,
                response_code: None,
            };
            delivery_ids.push(delivery.id);
            self.deliveries.push(delivery);

            if let Some(wh) = self.webhooks.get_mut(webhook_id) {
                wh.last_triggered_at = Some(Utc::now());
            }
        }

        debug!(event = ?event, deliveries = delivery_ids.len(), "webhook event triggered");
        delivery_ids
    }

    /// Simulate delivery of a webhook. In production this would be an HTTP POST.
    pub fn process_delivery(
        &mut self,
        delivery_id: &EntityId,
        success: bool,
        response_code: u16,
    ) -> Result<DeliveryStatus, WebhookError> {
        let delivery = self
            .deliveries
            .iter_mut()
            .find(|d| d.id == *delivery_id)
            .ok_or(WebhookError::DeliveryNotFound(*delivery_id))?;

        delivery.attempts += 1;
        delivery.response_code = Some(response_code);

        if success {
            delivery.status = DeliveryStatus::Delivered;
            delivery.completed_at = Some(Utc::now());

            if let Some(wh) = self.webhooks.get_mut(&delivery.webhook_id) {
                wh.success_count += 1;
                // Reset failure count on success.
                wh.failure_count = 0;
            }

            debug!(delivery_id = %delivery_id, "webhook delivered");
            Ok(DeliveryStatus::Delivered)
        } else {
            let webhook_id = delivery.webhook_id;

            if delivery.attempts >= delivery.max_attempts {
                delivery.status = DeliveryStatus::Failed;
                delivery.completed_at = Some(Utc::now());

                if let Some(wh) = self.webhooks.get_mut(&webhook_id) {
                    wh.failure_count += 1;
                    // Auto-disable after too many consecutive failures.
                    if wh.failure_count >= self.consecutive_failure_threshold {
                        wh.status = WebhookStatus::Disabled;
                        warn!(webhook_id = %webhook_id, "webhook auto-disabled after repeated failures");
                    }
                }

                Ok(DeliveryStatus::Failed)
            } else {
                // Schedule retry with exponential backoff.
                let delay_secs = 2i64.pow(delivery.attempts);
                delivery.status = DeliveryStatus::Retrying;
                delivery.next_retry_at = Some(Utc::now() + Duration::seconds(delay_secs));

                debug!(
                    delivery_id = %delivery_id,
                    attempt = delivery.attempts,
                    next_retry_secs = delay_secs,
                    "webhook delivery retrying"
                );
                Ok(DeliveryStatus::Retrying)
            }
        }
    }

    /// Pause a webhook.
    pub fn pause(&mut self, webhook_id: &EntityId) -> Result<(), WebhookError> {
        let wh = self
            .webhooks
            .get_mut(webhook_id)
            .ok_or(WebhookError::WebhookNotFound(*webhook_id))?;
        wh.status = WebhookStatus::Paused;
        info!(webhook_id = %webhook_id, "webhook paused");
        Ok(())
    }

    /// Resume a paused or disabled webhook.
    pub fn resume(&mut self, webhook_id: &EntityId) -> Result<(), WebhookError> {
        let wh = self
            .webhooks
            .get_mut(webhook_id)
            .ok_or(WebhookError::WebhookNotFound(*webhook_id))?;

        if wh.status == WebhookStatus::Deleted {
            return Err(WebhookError::WebhookDeleted);
        }

        wh.status = WebhookStatus::Active;
        wh.failure_count = 0;
        info!(webhook_id = %webhook_id, "webhook resumed");
        Ok(())
    }

    /// Soft-delete a webhook and cancel pending deliveries.
    pub fn delete(&mut self, webhook_id: &EntityId) -> Result<usize, WebhookError> {
        let wh = self
            .webhooks
            .get_mut(webhook_id)
            .ok_or(WebhookError::WebhookNotFound(*webhook_id))?;
        wh.status = WebhookStatus::Deleted;

        // Cancel pending deliveries.
        let mut cancelled = 0;
        for d in &mut self.deliveries {
            if d.webhook_id == *webhook_id
                && (d.status == DeliveryStatus::Pending || d.status == DeliveryStatus::Retrying)
            {
                d.status = DeliveryStatus::Cancelled;
                d.completed_at = Some(Utc::now());
                cancelled += 1;
            }
        }

        info!(webhook_id = %webhook_id, cancelled, "webhook deleted");
        Ok(cancelled)
    }

    /// Update which events a webhook subscribes to.
    pub fn update_events(
        &mut self,
        webhook_id: &EntityId,
        events: Vec<WebhookEvent>,
    ) -> Result<(), WebhookError> {
        if events.is_empty() {
            return Err(WebhookError::NoEvents);
        }
        let wh = self
            .webhooks
            .get_mut(webhook_id)
            .ok_or(WebhookError::WebhookNotFound(*webhook_id))?;
        wh.events = events;
        Ok(())
    }

    /// Get webhook health status.
    pub fn health(&self, webhook_id: &EntityId) -> Option<WebhookHealth> {
        let wh = self.webhooks.get(webhook_id)?;
        let total = wh.success_count + wh.failure_count;
        let success_rate = if total > 0 {
            wh.success_count as f64 / total as f64
        } else {
            1.0
        };
        let pending = self
            .deliveries
            .iter()
            .filter(|d| {
                d.webhook_id == *webhook_id
                    && (d.status == DeliveryStatus::Pending || d.status == DeliveryStatus::Retrying)
            })
            .count();

        Some(WebhookHealth {
            webhook_id: wh.id,
            name: wh.name.clone(),
            status: wh.status,
            success_rate,
            total_deliveries: total,
            pending_deliveries: pending,
            last_triggered_at: wh.last_triggered_at,
        })
    }

    /// Get a webhook by ID.
    pub fn get(&self, webhook_id: &EntityId) -> Option<&Webhook> {
        self.webhooks.get(webhook_id)
    }

    /// List all webhooks for an owner.
    pub fn list_for_owner(&self, owner_id: &EntityId) -> Vec<&Webhook> {
        self.webhooks
            .values()
            .filter(|w| w.owner_id == *owner_id && w.status != WebhookStatus::Deleted)
            .collect()
    }

    /// Total registered webhooks (excluding deleted).
    pub fn total_active(&self) -> usize {
        self.webhooks
            .values()
            .filter(|w| w.status != WebhookStatus::Deleted)
            .count()
    }

    /// Count pending deliveries.
    pub fn pending_deliveries(&self) -> usize {
        self.deliveries
            .iter()
            .filter(|d| d.status == DeliveryStatus::Pending || d.status == DeliveryStatus::Retrying)
            .count()
    }
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum WebhookError {
    #[error("webhook not found: {0}")]
    WebhookNotFound(EntityId),
    #[error("delivery not found: {0}")]
    DeliveryNotFound(EntityId),
    #[error("too many webhooks (max {max})")]
    TooManyWebhooks { max: usize },
    #[error("webhook URL must use HTTPS (except localhost)")]
    InsecureUrl,
    #[error("at least one event is required")]
    NoEvents,
    #[error("webhook has been deleted")]
    WebhookDeleted,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn test_manager() -> WebhookManager {
        WebhookManager::new(10)
    }

    #[test]
    fn register_webhook() {
        let mut mgr = test_manager();
        let owner = EntityId::new();
        let wh = mgr
            .register(
                owner,
                "test-hook",
                "https://example.com/webhook",
                "secret123",
                vec![WebhookEvent::PositionUpdate],
            )
            .unwrap();

        assert_eq!(wh.status, WebhookStatus::Active);
        assert_eq!(wh.events.len(), 1);
        assert_eq!(mgr.total_active(), 1);
    }

    #[test]
    fn reject_insecure_url() {
        let mut mgr = test_manager();
        let result = mgr.register(
            EntityId::new(),
            "bad",
            "http://example.com/webhook",
            "s",
            vec![WebhookEvent::PositionUpdate],
        );
        assert!(matches!(result.unwrap_err(), WebhookError::InsecureUrl));
    }

    #[test]
    fn allow_localhost_http() {
        let mut mgr = test_manager();
        let result = mgr.register(
            EntityId::new(),
            "local",
            "http://localhost:3000/hook",
            "s",
            vec![WebhookEvent::PositionUpdate],
        );
        assert!(result.is_ok());
    }

    #[test]
    fn reject_empty_events() {
        let mut mgr = test_manager();
        let result = mgr.register(
            EntityId::new(),
            "no-events",
            "https://example.com/hook",
            "s",
            vec![],
        );
        assert!(matches!(result.unwrap_err(), WebhookError::NoEvents));
    }

    #[test]
    fn max_webhooks_enforced() {
        let mut mgr = WebhookManager::new(1);
        let owner = EntityId::new();
        mgr.register(
            owner,
            "a",
            "https://a.com/hook",
            "s",
            vec![WebhookEvent::PositionUpdate],
        )
        .unwrap();
        let result = mgr.register(
            owner,
            "b",
            "https://b.com/hook",
            "s",
            vec![WebhookEvent::PositionUpdate],
        );
        assert!(matches!(
            result.unwrap_err(),
            WebhookError::TooManyWebhooks { max: 1 }
        ));
    }

    #[test]
    fn trigger_and_deliver() {
        let mut mgr = test_manager();
        let owner = EntityId::new();
        mgr.register(
            owner,
            "hook",
            "https://ex.com/h",
            "s",
            vec![WebhookEvent::IntegrityAlert, WebhookEvent::SlaViolation],
        )
        .unwrap();

        // Trigger matching event.
        let ids = mgr.trigger(
            WebhookEvent::IntegrityAlert,
            serde_json::json!({"level": "warning"}),
        );
        assert_eq!(ids.len(), 1);

        // Trigger non-matching event.
        let ids2 = mgr.trigger(WebhookEvent::RouteCompleted, serde_json::json!({}));
        assert_eq!(ids2.len(), 0);

        // Deliver.
        let delivery_id = ids[0];
        let status = mgr.process_delivery(&delivery_id, true, 200).unwrap();
        assert_eq!(status, DeliveryStatus::Delivered);
    }

    #[test]
    fn retry_on_failure() {
        let mut mgr = test_manager();
        let owner = EntityId::new();
        mgr.register(
            owner,
            "hook",
            "https://ex.com/h",
            "s",
            vec![WebhookEvent::PositionUpdate],
        )
        .unwrap();

        let ids = mgr.trigger(WebhookEvent::PositionUpdate, serde_json::json!({}));
        let delivery_id = ids[0];

        // First failure → retrying.
        let status = mgr.process_delivery(&delivery_id, false, 500).unwrap();
        assert_eq!(status, DeliveryStatus::Retrying);
        assert_eq!(mgr.pending_deliveries(), 1);
    }

    #[test]
    fn exhausted_retries_marks_failed() {
        let mut mgr = test_manager();
        mgr.max_retry_attempts = 2;
        let owner = EntityId::new();
        mgr.register(
            owner,
            "hook",
            "https://ex.com/h",
            "s",
            vec![WebhookEvent::PositionUpdate],
        )
        .unwrap();

        let ids = mgr.trigger(WebhookEvent::PositionUpdate, serde_json::json!({}));
        let delivery_id = ids[0];

        mgr.process_delivery(&delivery_id, false, 500).unwrap(); // attempt 1 → retrying
        let status = mgr.process_delivery(&delivery_id, false, 500).unwrap(); // attempt 2 → failed
        assert_eq!(status, DeliveryStatus::Failed);
    }

    #[test]
    fn auto_disable_after_threshold() {
        let mut mgr = test_manager();
        mgr.max_retry_attempts = 1;
        mgr.consecutive_failure_threshold = 2;
        let owner = EntityId::new();
        let wh = mgr
            .register(
                owner,
                "fragile",
                "https://ex.com/h",
                "s",
                vec![WebhookEvent::PositionUpdate],
            )
            .unwrap();

        // Trigger and fail twice.
        for _ in 0..2 {
            let ids = mgr.trigger(WebhookEvent::PositionUpdate, serde_json::json!({}));
            mgr.process_delivery(&ids[0], false, 500).unwrap();
        }

        let webhook = mgr.get(&wh.id).unwrap();
        assert_eq!(webhook.status, WebhookStatus::Disabled);
    }

    #[test]
    fn pause_and_resume() {
        let mut mgr = test_manager();
        let owner = EntityId::new();
        let wh = mgr
            .register(
                owner,
                "hook",
                "https://ex.com/h",
                "s",
                vec![WebhookEvent::PositionUpdate],
            )
            .unwrap();

        mgr.pause(&wh.id).unwrap();
        assert_eq!(mgr.get(&wh.id).unwrap().status, WebhookStatus::Paused);

        // Paused webhook doesn't receive events.
        let ids = mgr.trigger(WebhookEvent::PositionUpdate, serde_json::json!({}));
        assert_eq!(ids.len(), 0);

        mgr.resume(&wh.id).unwrap();
        assert_eq!(mgr.get(&wh.id).unwrap().status, WebhookStatus::Active);
    }

    #[test]
    fn delete_cancels_pending() {
        let mut mgr = test_manager();
        let owner = EntityId::new();
        let wh = mgr
            .register(
                owner,
                "hook",
                "https://ex.com/h",
                "s",
                vec![WebhookEvent::PositionUpdate],
            )
            .unwrap();

        mgr.trigger(WebhookEvent::PositionUpdate, serde_json::json!({}));
        mgr.trigger(WebhookEvent::PositionUpdate, serde_json::json!({}));
        assert_eq!(mgr.pending_deliveries(), 2);

        let cancelled = mgr.delete(&wh.id).unwrap();
        assert_eq!(cancelled, 2);
        assert_eq!(mgr.pending_deliveries(), 0);
        assert_eq!(mgr.total_active(), 0);
    }

    #[test]
    fn resume_deleted_webhook_fails() {
        let mut mgr = test_manager();
        let owner = EntityId::new();
        let wh = mgr
            .register(
                owner,
                "hook",
                "https://ex.com/h",
                "s",
                vec![WebhookEvent::PositionUpdate],
            )
            .unwrap();

        mgr.delete(&wh.id).unwrap();
        let result = mgr.resume(&wh.id);
        assert!(matches!(result.unwrap_err(), WebhookError::WebhookDeleted));
    }

    #[test]
    fn webhook_health() {
        let mut mgr = test_manager();
        let owner = EntityId::new();
        let wh = mgr
            .register(
                owner,
                "health-check",
                "https://ex.com/h",
                "s",
                vec![WebhookEvent::PositionUpdate],
            )
            .unwrap();

        // Trigger and deliver 2 events.
        for _ in 0..2 {
            let ids = mgr.trigger(WebhookEvent::PositionUpdate, serde_json::json!({}));
            mgr.process_delivery(&ids[0], true, 200).unwrap();
        }

        let health = mgr.health(&wh.id).unwrap();
        assert_eq!(health.total_deliveries, 2);
        assert!((health.success_rate - 1.0).abs() < 0.01);
        assert_eq!(health.pending_deliveries, 0);
    }

    #[test]
    fn update_events() {
        let mut mgr = test_manager();
        let owner = EntityId::new();
        let wh = mgr
            .register(
                owner,
                "hook",
                "https://ex.com/h",
                "s",
                vec![WebhookEvent::PositionUpdate],
            )
            .unwrap();

        mgr.update_events(
            &wh.id,
            vec![WebhookEvent::SlaViolation, WebhookEvent::TrafficIncident],
        )
        .unwrap();

        let updated = mgr.get(&wh.id).unwrap();
        assert_eq!(updated.events.len(), 2);
        assert!(updated.events.contains(&WebhookEvent::SlaViolation));
    }

    #[test]
    fn list_for_owner() {
        let mut mgr = test_manager();
        let owner_a = EntityId::new();
        let owner_b = EntityId::new();

        mgr.register(
            owner_a,
            "a1",
            "https://a.com/h",
            "s",
            vec![WebhookEvent::PositionUpdate],
        )
        .unwrap();
        mgr.register(
            owner_a,
            "a2",
            "https://a.com/h2",
            "s",
            vec![WebhookEvent::RouteCompleted],
        )
        .unwrap();
        mgr.register(
            owner_b,
            "b1",
            "https://b.com/h",
            "s",
            vec![WebhookEvent::PositionUpdate],
        )
        .unwrap();

        assert_eq!(mgr.list_for_owner(&owner_a).len(), 2);
        assert_eq!(mgr.list_for_owner(&owner_b).len(), 1);
    }

    #[test]
    fn success_resets_failure_count() {
        let mut mgr = test_manager();
        mgr.max_retry_attempts = 1;
        let owner = EntityId::new();
        let wh = mgr
            .register(
                owner,
                "hook",
                "https://ex.com/h",
                "s",
                vec![WebhookEvent::PositionUpdate],
            )
            .unwrap();

        // Fail once.
        let ids = mgr.trigger(WebhookEvent::PositionUpdate, serde_json::json!({}));
        mgr.process_delivery(&ids[0], false, 500).unwrap();
        assert_eq!(mgr.get(&wh.id).unwrap().failure_count, 1);

        // Succeed — should reset failure count.
        let ids = mgr.trigger(WebhookEvent::PositionUpdate, serde_json::json!({}));
        mgr.process_delivery(&ids[0], true, 200).unwrap();
        assert_eq!(mgr.get(&wh.id).unwrap().failure_count, 0);
    }
}
