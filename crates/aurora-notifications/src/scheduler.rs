//! Scheduled alerts — time-based notification scheduling with recurrence.

use chrono::{DateTime, Datelike, Utc, Weekday};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Recurrence pattern for scheduled alerts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Recurrence {
    Once,
    Daily,
    Weekly(Vec<Weekday>),
    Monthly(Vec<u32>),
    Custom { interval_secs: u64 },
}

/// A scheduled alert.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledAlert {
    pub id: Uuid,
    pub title: String,
    pub message: String,
    pub next_fire: DateTime<Utc>,
    pub recurrence: Recurrence,
    pub active: bool,
    pub fire_count: u64,
    pub max_fires: Option<u64>,
}

impl ScheduledAlert {
    /// Create a new one-time scheduled alert.
    pub fn once(title: &str, message: &str, fire_at: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.to_string(),
            message: message.to_string(),
            next_fire: fire_at,
            recurrence: Recurrence::Once,
            active: true,
            fire_count: 0,
            max_fires: Some(1),
        }
    }

    /// Create a recurring alert.
    pub fn recurring(
        title: &str,
        message: &str,
        first_fire: DateTime<Utc>,
        recurrence: Recurrence,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.to_string(),
            message: message.to_string(),
            next_fire: first_fire,
            recurrence,
            active: true,
            fire_count: 0,
            max_fires: None,
        }
    }

    /// Check if this alert should fire at the given time.
    pub fn should_fire(&self, now: DateTime<Utc>) -> bool {
        if !self.active {
            return false;
        }
        if let Some(max) = self.max_fires {
            if self.fire_count >= max {
                return false;
            }
        }
        now >= self.next_fire
    }

    /// Advance to the next fire time based on recurrence.
    pub fn advance(&mut self) {
        self.fire_count += 1;
        if let Some(max) = self.max_fires {
            if self.fire_count >= max {
                self.active = false;
                return;
            }
        }
        match &self.recurrence {
            Recurrence::Once => {
                self.active = false;
            }
            Recurrence::Daily => {
                self.next_fire += chrono::Duration::days(1);
            }
            Recurrence::Weekly(days) => {
                if days.is_empty() {
                    self.active = false;
                    return;
                }
                let mut candidate = self.next_fire + chrono::Duration::days(1);
                for _ in 0..7 {
                    if days.contains(&candidate.weekday()) {
                        self.next_fire = candidate;
                        return;
                    }
                    candidate += chrono::Duration::days(1);
                }
                self.next_fire += chrono::Duration::days(7);
            }
            Recurrence::Monthly(days_of_month) => {
                if days_of_month.is_empty() {
                    self.active = false;
                    return;
                }
                // Find next matching day
                let mut candidate = self.next_fire + chrono::Duration::days(1);
                for _ in 0..62 {
                    if days_of_month.contains(&candidate.day()) {
                        self.next_fire = candidate;
                        return;
                    }
                    candidate += chrono::Duration::days(1);
                }
                self.active = false;
            }
            Recurrence::Custom { interval_secs } => {
                self.next_fire += chrono::Duration::seconds(*interval_secs as i64);
            }
        }
    }
}

/// Fired alert event.
#[derive(Debug, Clone)]
pub struct FiredAlert {
    pub alert_id: Uuid,
    pub title: String,
    pub message: String,
    pub fired_at: DateTime<Utc>,
    pub fire_number: u64,
}

/// Scheduler — manages and fires scheduled alerts.
pub struct AlertScheduler {
    alerts: RwLock<Vec<ScheduledAlert>>,
}

impl AlertScheduler {
    /// Create a new scheduler.
    pub fn new() -> Self {
        Self {
            alerts: RwLock::new(Vec::new()),
        }
    }

    /// Add a scheduled alert.
    pub fn schedule(&self, alert: ScheduledAlert) {
        self.alerts.write().push(alert);
    }

    /// Cancel a scheduled alert.
    pub fn cancel(&self, id: Uuid) -> bool {
        let mut alerts = self.alerts.write();
        if let Some(a) = alerts.iter_mut().find(|a| a.id == id) {
            a.active = false;
            return true;
        }
        false
    }

    /// Check and fire all due alerts. Returns fired events.
    pub fn tick(&self, now: DateTime<Utc>) -> Vec<FiredAlert> {
        let mut alerts = self.alerts.write();
        let mut fired = Vec::new();

        for alert in alerts.iter_mut() {
            if alert.should_fire(now) {
                fired.push(FiredAlert {
                    alert_id: alert.id,
                    title: alert.title.clone(),
                    message: alert.message.clone(),
                    fired_at: now,
                    fire_number: alert.fire_count + 1,
                });
                alert.advance();
            }
        }

        fired
    }

    /// Get all active alerts.
    pub fn active_alerts(&self) -> Vec<ScheduledAlert> {
        self.alerts
            .read()
            .iter()
            .filter(|a| a.active)
            .cloned()
            .collect()
    }

    /// Count active alerts.
    pub fn active_count(&self) -> usize {
        self.alerts.read().iter().filter(|a| a.active).count()
    }

    /// Remove inactive (completed/cancelled) alerts.
    pub fn cleanup(&self) -> usize {
        let mut alerts = self.alerts.write();
        let before = alerts.len();
        alerts.retain(|a| a.active);
        before - alerts.len()
    }
}

impl Default for AlertScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_once_fires_and_deactivates() {
        let scheduler = AlertScheduler::new();
        let now = Utc::now();
        let alert = ScheduledAlert::once("Test", "Message", now - Duration::seconds(1));
        scheduler.schedule(alert);
        let fired = scheduler.tick(now);
        assert_eq!(fired.len(), 1);
        assert_eq!(fired[0].title, "Test");
        // Should not fire again
        let fired2 = scheduler.tick(now);
        assert!(fired2.is_empty());
        assert_eq!(scheduler.active_count(), 0);
    }

    #[test]
    fn test_daily_recurrence() {
        let scheduler = AlertScheduler::new();
        let now = Utc::now();
        let alert = ScheduledAlert::recurring("Daily", "Msg", now, Recurrence::Daily);
        scheduler.schedule(alert);
        let f1 = scheduler.tick(now);
        assert_eq!(f1.len(), 1);
        // Not due yet
        let f2 = scheduler.tick(now + Duration::hours(12));
        assert!(f2.is_empty());
        // Due after 24h
        let f3 = scheduler.tick(now + Duration::hours(25));
        assert_eq!(f3.len(), 1);
    }

    #[test]
    fn test_custom_interval() {
        let scheduler = AlertScheduler::new();
        let now = Utc::now();
        let alert = ScheduledAlert::recurring(
            "Every 5 min",
            "Check",
            now,
            Recurrence::Custom { interval_secs: 300 },
        );
        scheduler.schedule(alert);
        scheduler.tick(now);
        let f = scheduler.tick(now + Duration::seconds(301));
        assert_eq!(f.len(), 1);
    }

    #[test]
    fn test_max_fires() {
        let scheduler = AlertScheduler::new();
        let now = Utc::now();
        let mut alert = ScheduledAlert::recurring(
            "Limited",
            "Msg",
            now,
            Recurrence::Custom { interval_secs: 1 },
        );
        alert.max_fires = Some(3);
        scheduler.schedule(alert);

        for i in 0..5 {
            scheduler.tick(now + Duration::seconds(i * 2));
        }
        assert_eq!(scheduler.active_count(), 0);
    }

    #[test]
    fn test_cancel() {
        let scheduler = AlertScheduler::new();
        let alert = ScheduledAlert::once("Cancel me", "Msg", Utc::now());
        let id = alert.id;
        scheduler.schedule(alert);
        assert!(scheduler.cancel(id));
        assert_eq!(scheduler.active_count(), 0);
        let fired = scheduler.tick(Utc::now());
        assert!(fired.is_empty());
    }

    #[test]
    fn test_future_alert_not_fired() {
        let scheduler = AlertScheduler::new();
        let future = Utc::now() + Duration::hours(1);
        let alert = ScheduledAlert::once("Future", "Msg", future);
        scheduler.schedule(alert);
        let fired = scheduler.tick(Utc::now());
        assert!(fired.is_empty());
        assert_eq!(scheduler.active_count(), 1);
    }

    #[test]
    fn test_cleanup() {
        let scheduler = AlertScheduler::new();
        let now = Utc::now();
        scheduler.schedule(ScheduledAlert::once(
            "Done",
            "Msg",
            now - Duration::seconds(1),
        ));
        scheduler.schedule(ScheduledAlert::once(
            "Active",
            "Msg",
            now + Duration::hours(1),
        ));
        scheduler.tick(now);
        let cleaned = scheduler.cleanup();
        assert_eq!(cleaned, 1);
        assert_eq!(scheduler.active_count(), 1);
    }

    #[test]
    fn test_weekly_recurrence() {
        let scheduler = AlertScheduler::new();
        let now = Utc::now();
        let alert = ScheduledAlert::recurring(
            "Weekly",
            "Msg",
            now,
            Recurrence::Weekly(vec![now.weekday()]),
        );
        scheduler.schedule(alert);
        scheduler.tick(now);
        // Next fire should be 7 days later
        let alerts = scheduler.active_alerts();
        assert_eq!(alerts.len(), 1);
        let diff = alerts[0].next_fire - now;
        assert!(diff.num_days() >= 6 && diff.num_days() <= 7);
    }

    #[test]
    fn test_fire_number_increments() {
        let scheduler = AlertScheduler::new();
        let now = Utc::now();
        let alert = ScheduledAlert::recurring(
            "Counter",
            "Msg",
            now,
            Recurrence::Custom { interval_secs: 1 },
        );
        scheduler.schedule(alert);
        let f1 = scheduler.tick(now);
        assert_eq!(f1[0].fire_number, 1);
        let f2 = scheduler.tick(now + Duration::seconds(2));
        assert_eq!(f2[0].fire_number, 2);
    }
}
