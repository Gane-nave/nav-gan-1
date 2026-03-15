//! Subscription management — plan definitions, subscription lifecycle,
//! upgrade/downgrade, trial periods, and cancellation.

use aurora_core::types::EntityId;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

// ---------------------------------------------------------------------------
// Plan types
// ---------------------------------------------------------------------------

/// A subscription plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub id: EntityId,
    pub name: String,
    pub tier: PlanTier,
    pub price_cents: u64,
    pub billing_interval: BillingInterval,
    pub features: Vec<String>,
    pub api_quota: u64,
    pub max_plugins: u32,
    pub active: bool,
}

/// Plan tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PlanTier {
    Free,
    Starter,
    Professional,
    Enterprise,
}

/// Billing interval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillingInterval {
    Monthly,
    Quarterly,
    Yearly,
}

impl BillingInterval {
    pub fn duration_days(&self) -> i64 {
        match self {
            Self::Monthly => 30,
            Self::Quarterly => 90,
            Self::Yearly => 365,
        }
    }
}

// ---------------------------------------------------------------------------
// Subscription types
// ---------------------------------------------------------------------------

/// A user's subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: EntityId,
    pub user_id: EntityId,
    pub plan_id: EntityId,
    pub plan_tier: PlanTier,
    pub status: SubscriptionStatus,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub trial_end: Option<DateTime<Utc>>,
    pub cancel_at_period_end: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Subscription {
    /// Whether the subscription is currently in a trial period.
    pub fn is_trial(&self) -> bool {
        if let Some(trial_end) = self.trial_end {
            Utc::now() < trial_end
        } else {
            false
        }
    }

    /// Days remaining in current period.
    pub fn days_remaining(&self) -> i64 {
        let now = Utc::now();
        if now >= self.current_period_end {
            0
        } else {
            (self.current_period_end - now).num_days()
        }
    }
}

/// Subscription status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscriptionStatus {
    /// Active and paid.
    Active,
    /// In trial period.
    Trialing,
    /// Payment failed, in grace period.
    PastDue,
    /// Cancelled but still active until period end.
    Cancelling,
    /// Fully cancelled and expired.
    Cancelled,
    /// Paused by user.
    Paused,
}

// ---------------------------------------------------------------------------
// Subscription manager
// ---------------------------------------------------------------------------

/// Manages plans and subscriptions.
pub struct SubscriptionManager {
    plans: HashMap<EntityId, Plan>,
    subscriptions: HashMap<EntityId, Subscription>,
    /// user_id → subscription_id
    user_subs: HashMap<EntityId, EntityId>,
}

impl SubscriptionManager {
    pub fn new() -> Self {
        Self {
            plans: HashMap::new(),
            subscriptions: HashMap::new(),
            user_subs: HashMap::new(),
        }
    }

    /// Register a plan.
    #[allow(clippy::too_many_arguments)]
    pub fn register_plan(
        &mut self,
        name: impl Into<String>,
        tier: PlanTier,
        price_cents: u64,
        interval: BillingInterval,
        features: Vec<String>,
        api_quota: u64,
        max_plugins: u32,
    ) -> Plan {
        let plan = Plan {
            id: EntityId::new(),
            name: name.into(),
            tier,
            price_cents,
            billing_interval: interval,
            features,
            api_quota,
            max_plugins,
            active: true,
        };

        let result = plan.clone();
        self.plans.insert(plan.id, plan);
        result
    }

    /// Subscribe a user to a plan.
    pub fn subscribe(
        &mut self,
        user_id: EntityId,
        plan_id: EntityId,
        trial_days: Option<u32>,
    ) -> Result<Subscription, SubscriptionError> {
        // Check if user already has an active subscription.
        if let Some(sub_id) = self.user_subs.get(&user_id) {
            let existing = &self.subscriptions[sub_id];
            if existing.status != SubscriptionStatus::Cancelled {
                return Err(SubscriptionError::AlreadySubscribed(user_id));
            }
        }

        let plan = self
            .plans
            .get(&plan_id)
            .ok_or(SubscriptionError::PlanNotFound(plan_id))?;

        if !plan.active {
            return Err(SubscriptionError::PlanInactive(plan_id));
        }

        let now = Utc::now();
        let period_end = now + Duration::days(plan.billing_interval.duration_days());
        let trial_end = trial_days.map(|d| now + Duration::days(d as i64));
        let status = if trial_end.is_some() {
            SubscriptionStatus::Trialing
        } else {
            SubscriptionStatus::Active
        };

        let sub = Subscription {
            id: EntityId::new(),
            user_id,
            plan_id,
            plan_tier: plan.tier,
            status,
            current_period_start: now,
            current_period_end: period_end,
            trial_end,
            cancel_at_period_end: false,
            created_at: now,
            updated_at: now,
        };

        info!(sub_id = %sub.id, user = %user_id, plan = %plan_id, "subscription created");
        self.user_subs.insert(user_id, sub.id);
        let result = sub.clone();
        self.subscriptions.insert(sub.id, sub);
        Ok(result)
    }

    /// Cancel a subscription (at period end).
    pub fn cancel(&mut self, sub_id: &EntityId) -> Result<(), SubscriptionError> {
        let sub = self
            .subscriptions
            .get_mut(sub_id)
            .ok_or(SubscriptionError::NotFound(*sub_id))?;

        if sub.status == SubscriptionStatus::Cancelled {
            return Err(SubscriptionError::AlreadyCancelled);
        }

        sub.cancel_at_period_end = true;
        sub.status = SubscriptionStatus::Cancelling;
        sub.updated_at = Utc::now();
        info!(sub_id = %sub_id, "subscription cancelled");
        Ok(())
    }

    /// Immediately cancel (no grace period).
    pub fn cancel_immediately(&mut self, sub_id: &EntityId) -> Result<(), SubscriptionError> {
        let sub = self
            .subscriptions
            .get_mut(sub_id)
            .ok_or(SubscriptionError::NotFound(*sub_id))?;

        sub.status = SubscriptionStatus::Cancelled;
        sub.cancel_at_period_end = true;
        sub.updated_at = Utc::now();

        // Remove from user index so they can re-subscribe.
        self.user_subs.remove(&sub.user_id);
        Ok(())
    }

    /// Change plan (upgrade or downgrade).
    pub fn change_plan(
        &mut self,
        sub_id: &EntityId,
        new_plan_id: EntityId,
    ) -> Result<(), SubscriptionError> {
        let new_plan = self
            .plans
            .get(&new_plan_id)
            .ok_or(SubscriptionError::PlanNotFound(new_plan_id))?;

        if !new_plan.active {
            return Err(SubscriptionError::PlanInactive(new_plan_id));
        }

        let new_tier = new_plan.tier;
        let new_interval = new_plan.billing_interval;

        let sub = self
            .subscriptions
            .get_mut(sub_id)
            .ok_or(SubscriptionError::NotFound(*sub_id))?;

        if sub.status == SubscriptionStatus::Cancelled {
            return Err(SubscriptionError::AlreadyCancelled);
        }

        sub.plan_id = new_plan_id;
        sub.plan_tier = new_tier;
        let now = Utc::now();
        sub.current_period_start = now;
        sub.current_period_end = now + Duration::days(new_interval.duration_days());
        sub.updated_at = now;
        info!(sub_id = %sub_id, new_plan = %new_plan_id, "plan changed");
        Ok(())
    }

    /// Pause a subscription.
    pub fn pause(&mut self, sub_id: &EntityId) -> Result<(), SubscriptionError> {
        let sub = self
            .subscriptions
            .get_mut(sub_id)
            .ok_or(SubscriptionError::NotFound(*sub_id))?;

        if sub.status != SubscriptionStatus::Active {
            return Err(SubscriptionError::NotActive);
        }

        sub.status = SubscriptionStatus::Paused;
        sub.updated_at = Utc::now();
        Ok(())
    }

    /// Resume a paused subscription.
    pub fn resume(&mut self, sub_id: &EntityId) -> Result<(), SubscriptionError> {
        let sub = self
            .subscriptions
            .get_mut(sub_id)
            .ok_or(SubscriptionError::NotFound(*sub_id))?;

        if sub.status != SubscriptionStatus::Paused {
            return Err(SubscriptionError::NotPaused);
        }

        sub.status = SubscriptionStatus::Active;
        sub.updated_at = Utc::now();
        Ok(())
    }

    /// Get a plan by ID.
    pub fn get_plan(&self, plan_id: &EntityId) -> Option<&Plan> {
        self.plans.get(plan_id)
    }

    /// Get all active plans.
    pub fn active_plans(&self) -> Vec<&Plan> {
        self.plans.values().filter(|p| p.active).collect()
    }

    /// Get a subscription by ID.
    pub fn get(&self, sub_id: &EntityId) -> Option<&Subscription> {
        self.subscriptions.get(sub_id)
    }

    /// Get subscription for a user.
    pub fn for_user(&self, user_id: &EntityId) -> Option<&Subscription> {
        self.user_subs
            .get(user_id)
            .and_then(|id| self.subscriptions.get(id))
    }

    /// Total active subscribers.
    pub fn total_active(&self) -> usize {
        self.subscriptions
            .values()
            .filter(|s| {
                s.status == SubscriptionStatus::Active || s.status == SubscriptionStatus::Trialing
            })
            .count()
    }
}

impl Default for SubscriptionManager {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum SubscriptionError {
    #[error("subscription not found: {0}")]
    NotFound(EntityId),
    #[error("plan not found: {0}")]
    PlanNotFound(EntityId),
    #[error("plan is inactive: {0}")]
    PlanInactive(EntityId),
    #[error("user already subscribed: {0}")]
    AlreadySubscribed(EntityId),
    #[error("subscription already cancelled")]
    AlreadyCancelled,
    #[error("subscription is not active")]
    NotActive,
    #[error("subscription is not paused")]
    NotPaused,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn test_mgr_with_plan() -> (SubscriptionManager, Plan) {
        let mut mgr = SubscriptionManager::new();
        let plan = mgr.register_plan(
            "Pro",
            PlanTier::Professional,
            4999,
            BillingInterval::Monthly,
            vec!["routing".into(), "fleet".into()],
            100_000,
            50,
        );
        (mgr, plan)
    }

    #[test]
    fn register_and_list_plans() {
        let (mgr, plan) = test_mgr_with_plan();
        assert_eq!(mgr.active_plans().len(), 1);
        assert_eq!(plan.tier, PlanTier::Professional);
        assert_eq!(plan.price_cents, 4999);
    }

    #[test]
    fn subscribe() {
        let (mut mgr, plan) = test_mgr_with_plan();
        let user = EntityId::new();
        let sub = mgr.subscribe(user, plan.id, None).unwrap();
        assert_eq!(sub.status, SubscriptionStatus::Active);
        assert!(!sub.cancel_at_period_end);
        assert!(sub.days_remaining() > 0);
    }

    #[test]
    fn subscribe_with_trial() {
        let (mut mgr, plan) = test_mgr_with_plan();
        let sub = mgr.subscribe(EntityId::new(), plan.id, Some(14)).unwrap();
        assert_eq!(sub.status, SubscriptionStatus::Trialing);
        assert!(sub.trial_end.is_some());
        assert!(sub.is_trial());
    }

    #[test]
    fn duplicate_subscription_rejected() {
        let (mut mgr, plan) = test_mgr_with_plan();
        let user = EntityId::new();
        mgr.subscribe(user, plan.id, None).unwrap();
        let result = mgr.subscribe(user, plan.id, None);
        assert!(matches!(
            result.unwrap_err(),
            SubscriptionError::AlreadySubscribed(_)
        ));
    }

    #[test]
    fn cancel_at_period_end() {
        let (mut mgr, plan) = test_mgr_with_plan();
        let sub = mgr.subscribe(EntityId::new(), plan.id, None).unwrap();
        mgr.cancel(&sub.id).unwrap();
        let s = mgr.get(&sub.id).unwrap();
        assert_eq!(s.status, SubscriptionStatus::Cancelling);
        assert!(s.cancel_at_period_end);
    }

    #[test]
    fn cancel_immediately_allows_resubscribe() {
        let (mut mgr, plan) = test_mgr_with_plan();
        let user = EntityId::new();
        let sub = mgr.subscribe(user, plan.id, None).unwrap();
        mgr.cancel_immediately(&sub.id).unwrap();
        // Can re-subscribe after immediate cancel.
        let sub2 = mgr.subscribe(user, plan.id, None).unwrap();
        assert_eq!(sub2.status, SubscriptionStatus::Active);
    }

    #[test]
    fn change_plan() {
        let (mut mgr, plan) = test_mgr_with_plan();
        let enterprise = mgr.register_plan(
            "Enterprise",
            PlanTier::Enterprise,
            19999,
            BillingInterval::Yearly,
            vec!["all".into()],
            1_000_000,
            200,
        );

        let sub = mgr.subscribe(EntityId::new(), plan.id, None).unwrap();
        mgr.change_plan(&sub.id, enterprise.id).unwrap();
        let s = mgr.get(&sub.id).unwrap();
        assert_eq!(s.plan_tier, PlanTier::Enterprise);
        assert_eq!(s.plan_id, enterprise.id);
    }

    #[test]
    fn pause_and_resume() {
        let (mut mgr, plan) = test_mgr_with_plan();
        let sub = mgr.subscribe(EntityId::new(), plan.id, None).unwrap();
        mgr.pause(&sub.id).unwrap();
        assert_eq!(mgr.get(&sub.id).unwrap().status, SubscriptionStatus::Paused);
        mgr.resume(&sub.id).unwrap();
        assert_eq!(mgr.get(&sub.id).unwrap().status, SubscriptionStatus::Active);
    }

    #[test]
    fn pause_non_active_fails() {
        let (mut mgr, plan) = test_mgr_with_plan();
        let sub = mgr.subscribe(EntityId::new(), plan.id, None).unwrap();
        mgr.cancel(&sub.id).unwrap();
        assert!(mgr.pause(&sub.id).is_err());
    }

    #[test]
    fn total_active() {
        let (mut mgr, plan) = test_mgr_with_plan();
        mgr.subscribe(EntityId::new(), plan.id, None).unwrap();
        let s2 = mgr.subscribe(EntityId::new(), plan.id, Some(14)).unwrap();
        assert_eq!(mgr.total_active(), 2); // Active + Trialing
        mgr.cancel_immediately(&s2.id).unwrap();
        assert_eq!(mgr.total_active(), 1);
    }

    #[test]
    fn for_user() {
        let (mut mgr, plan) = test_mgr_with_plan();
        let user = EntityId::new();
        mgr.subscribe(user, plan.id, None).unwrap();
        assert!(mgr.for_user(&user).is_some());
        assert!(mgr.for_user(&EntityId::new()).is_none());
    }

    #[test]
    fn inactive_plan_rejected() {
        let mut mgr = SubscriptionManager::new();
        let plan = mgr.register_plan(
            "Old",
            PlanTier::Starter,
            999,
            BillingInterval::Monthly,
            vec![],
            1000,
            5,
        );
        // Deactivate the plan.
        mgr.plans.get_mut(&plan.id).unwrap().active = false;
        let result = mgr.subscribe(EntityId::new(), plan.id, None);
        assert!(matches!(
            result.unwrap_err(),
            SubscriptionError::PlanInactive(_)
        ));
    }
}
