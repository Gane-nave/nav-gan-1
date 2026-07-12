//! Push notification system — manages notification channels, priorities, and delivery.

use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Notification priority level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Normal,
    High,
    Critical,
}

/// Notification channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Channel {
    InApp,
    Push,
    Sms,
    Email,
    Voice,
}

/// Delivery status of a notification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeliveryStatus {
    Pending,
    Sent,
    Delivered,
    Read,
    Failed,
    Expired,
}

/// A notification to be delivered.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub priority: Priority,
    pub channel: Channel,
    pub status: DeliveryStatus,
    pub created_at: DateTime<Utc>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub read_at: Option<DateTime<Utc>>,
    pub user_id: Uuid,
    pub metadata: HashMap<String, String>,
}

impl Notification {
    /// Create a new notification.
    pub fn new(
        title: &str,
        body: &str,
        priority: Priority,
        channel: Channel,
        user_id: Uuid,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.to_string(),
            body: body.to_string(),
            priority,
            channel,
            status: DeliveryStatus::Pending,
            created_at: Utc::now(),
            delivered_at: None,
            read_at: None,
            user_id,
            metadata: HashMap::new(),
        }
    }
}

/// User notification preferences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    pub user_id: Uuid,
    pub enabled_channels: Vec<Channel>,
    pub quiet_hours_start: Option<u8>,
    pub quiet_hours_end: Option<u8>,
    pub min_priority: Priority,
}

impl NotificationPreferences {
    /// Create default preferences (all channels, no quiet hours).
    pub fn default_for(user_id: Uuid) -> Self {
        Self {
            user_id,
            enabled_channels: vec![Channel::InApp, Channel::Push],
            quiet_hours_start: None,
            quiet_hours_end: None,
            min_priority: Priority::Low,
        }
    }

    /// Check if a notification should be delivered given preferences.
    pub fn should_deliver(&self, notification: &Notification, current_hour: u8) -> bool {
        if notification.priority < self.min_priority {
            return false;
        }
        if !self.enabled_channels.contains(&notification.channel) {
            return false;
        }
        if let (Some(start), Some(end)) = (self.quiet_hours_start, self.quiet_hours_end) {
            if start <= end {
                if current_hour >= start && current_hour < end {
                    return notification.priority >= Priority::Critical;
                }
            } else {
                // Wraps midnight (e.g., 22:00 - 06:00)
                if current_hour >= start || current_hour < end {
                    return notification.priority >= Priority::Critical;
                }
            }
        }
        true
    }
}

/// Notification dispatcher — manages delivery queue and user preferences.
pub struct NotificationDispatcher {
    queue: RwLock<Vec<Notification>>,
    delivered: RwLock<Vec<Notification>>,
    preferences: RwLock<HashMap<Uuid, NotificationPreferences>>,
    max_queue_size: usize,
}

impl NotificationDispatcher {
    /// Create a new dispatcher.
    pub fn new(max_queue_size: usize) -> Self {
        Self {
            queue: RwLock::new(Vec::new()),
            delivered: RwLock::new(Vec::new()),
            preferences: RwLock::new(HashMap::new()),
            max_queue_size,
        }
    }

    /// Set user preferences.
    pub fn set_preferences(&self, prefs: NotificationPreferences) {
        self.preferences.write().insert(prefs.user_id, prefs);
    }

    /// Enqueue a notification. Returns false if queue is full.
    pub fn enqueue(&self, notification: Notification) -> bool {
        let mut queue = self.queue.write();
        if queue.len() >= self.max_queue_size {
            // Evict lowest priority from queue
            if let Some(pos) = queue
                .iter()
                .position(|n| n.priority < notification.priority)
            {
                queue.remove(pos);
            } else {
                return false;
            }
        }
        queue.push(notification);
        true
    }

    /// Process the queue: deliver notifications respecting preferences.
    /// Returns count of delivered + skipped notifications.
    pub fn process(&self, current_hour: u8) -> (usize, usize) {
        let mut queue = self.queue.write();
        let prefs = self.preferences.read();
        let mut delivered_list = self.delivered.write();

        let mut delivered_count = 0;
        let mut skipped_count = 0;
        let mut remaining = Vec::new();

        for mut notif in queue.drain(..) {
            let user_prefs = prefs
                .get(&notif.user_id)
                .cloned()
                .unwrap_or_else(|| NotificationPreferences::default_for(notif.user_id));

            if user_prefs.should_deliver(&notif, current_hour) {
                notif.status = DeliveryStatus::Delivered;
                notif.delivered_at = Some(Utc::now());
                delivered_list.push(notif);
                delivered_count += 1;
            } else {
                notif.status = DeliveryStatus::Expired;
                skipped_count += 1;
                remaining.push(notif);
            }
        }

        *queue = remaining;
        (delivered_count, skipped_count)
    }

    /// Get delivered notifications for a user.
    pub fn delivered_for_user(&self, user_id: Uuid) -> Vec<Notification> {
        self.delivered
            .read()
            .iter()
            .filter(|n| n.user_id == user_id)
            .cloned()
            .collect()
    }

    /// Mark a notification as read.
    pub fn mark_read(&self, notification_id: Uuid) -> bool {
        let mut delivered = self.delivered.write();
        if let Some(notif) = delivered.iter_mut().find(|n| n.id == notification_id) {
            notif.status = DeliveryStatus::Read;
            notif.read_at = Some(Utc::now());
            return true;
        }
        false
    }

    /// Queue length.
    pub fn queue_len(&self) -> usize {
        self.queue.read().len()
    }

    /// Delivered count.
    pub fn delivered_count(&self) -> usize {
        self.delivered.read().len()
    }

    /// Unread count for a user.
    pub fn unread_count(&self, user_id: Uuid) -> usize {
        self.delivered
            .read()
            .iter()
            .filter(|n| n.user_id == user_id && n.status != DeliveryStatus::Read)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_notif(priority: Priority, channel: Channel, user_id: Uuid) -> Notification {
        Notification::new("Test", "Body", priority, channel, user_id)
    }

    #[test]
    fn test_enqueue_and_process() {
        let dispatcher = NotificationDispatcher::new(100);
        let uid = Uuid::new_v4();
        dispatcher.enqueue(make_notif(Priority::Normal, Channel::InApp, uid));
        dispatcher.enqueue(make_notif(Priority::High, Channel::Push, uid));
        assert_eq!(dispatcher.queue_len(), 2);
        let (delivered, skipped) = dispatcher.process(12);
        assert_eq!(delivered, 2);
        assert_eq!(skipped, 0);
        assert_eq!(dispatcher.delivered_count(), 2);
    }

    #[test]
    fn test_quiet_hours_block() {
        let dispatcher = NotificationDispatcher::new(100);
        let uid = Uuid::new_v4();
        let mut prefs = NotificationPreferences::default_for(uid);
        prefs.quiet_hours_start = Some(22);
        prefs.quiet_hours_end = Some(6);
        dispatcher.set_preferences(prefs);

        dispatcher.enqueue(make_notif(Priority::Normal, Channel::InApp, uid));
        let (delivered, skipped) = dispatcher.process(23); // during quiet hours
        assert_eq!(delivered, 0);
        assert_eq!(skipped, 1);
    }

    #[test]
    fn test_critical_bypasses_quiet_hours() {
        let dispatcher = NotificationDispatcher::new(100);
        let uid = Uuid::new_v4();
        let mut prefs = NotificationPreferences::default_for(uid);
        prefs.quiet_hours_start = Some(22);
        prefs.quiet_hours_end = Some(6);
        dispatcher.set_preferences(prefs);

        dispatcher.enqueue(make_notif(Priority::Critical, Channel::InApp, uid));
        let (delivered, _) = dispatcher.process(23);
        assert_eq!(delivered, 1);
    }

    #[test]
    fn test_min_priority_filter() {
        let dispatcher = NotificationDispatcher::new(100);
        let uid = Uuid::new_v4();
        let mut prefs = NotificationPreferences::default_for(uid);
        prefs.min_priority = Priority::High;
        dispatcher.set_preferences(prefs);

        dispatcher.enqueue(make_notif(Priority::Low, Channel::InApp, uid));
        dispatcher.enqueue(make_notif(Priority::High, Channel::InApp, uid));
        let (delivered, skipped) = dispatcher.process(12);
        assert_eq!(delivered, 1);
        assert_eq!(skipped, 1);
    }

    #[test]
    fn test_channel_filter() {
        let dispatcher = NotificationDispatcher::new(100);
        let uid = Uuid::new_v4();
        let mut prefs = NotificationPreferences::default_for(uid);
        prefs.enabled_channels = vec![Channel::InApp]; // Push disabled
        dispatcher.set_preferences(prefs);

        dispatcher.enqueue(make_notif(Priority::Normal, Channel::Push, uid));
        let (delivered, skipped) = dispatcher.process(12);
        assert_eq!(delivered, 0);
        assert_eq!(skipped, 1);
    }

    #[test]
    fn test_queue_eviction() {
        let dispatcher = NotificationDispatcher::new(2);
        let uid = Uuid::new_v4();
        dispatcher.enqueue(make_notif(Priority::Low, Channel::InApp, uid));
        dispatcher.enqueue(make_notif(Priority::Normal, Channel::InApp, uid));
        // Queue full, high priority should evict low
        let ok = dispatcher.enqueue(make_notif(Priority::High, Channel::InApp, uid));
        assert!(ok);
        assert_eq!(dispatcher.queue_len(), 2);
    }

    #[test]
    fn test_queue_full_reject() {
        let dispatcher = NotificationDispatcher::new(2);
        let uid = Uuid::new_v4();
        dispatcher.enqueue(make_notif(Priority::High, Channel::InApp, uid));
        dispatcher.enqueue(make_notif(Priority::High, Channel::InApp, uid));
        // Cannot evict — same priority
        let ok = dispatcher.enqueue(make_notif(Priority::Low, Channel::InApp, uid));
        assert!(!ok);
    }

    #[test]
    fn test_mark_read() {
        let dispatcher = NotificationDispatcher::new(100);
        let uid = Uuid::new_v4();
        let notif = make_notif(Priority::Normal, Channel::InApp, uid);
        let nid = notif.id;
        dispatcher.enqueue(notif);
        dispatcher.process(12);
        assert_eq!(dispatcher.unread_count(uid), 1);
        assert!(dispatcher.mark_read(nid));
        assert_eq!(dispatcher.unread_count(uid), 0);
    }

    #[test]
    fn test_delivered_for_user() {
        let dispatcher = NotificationDispatcher::new(100);
        let u1 = Uuid::new_v4();
        let u2 = Uuid::new_v4();
        dispatcher.enqueue(make_notif(Priority::Normal, Channel::InApp, u1));
        dispatcher.enqueue(make_notif(Priority::Normal, Channel::InApp, u1));
        dispatcher.enqueue(make_notif(Priority::Normal, Channel::InApp, u2));
        dispatcher.process(12);
        assert_eq!(dispatcher.delivered_for_user(u1).len(), 2);
        assert_eq!(dispatcher.delivered_for_user(u2).len(), 1);
    }

    #[test]
    fn test_quiet_hours_same_day() {
        let dispatcher = NotificationDispatcher::new(100);
        let uid = Uuid::new_v4();
        let mut prefs = NotificationPreferences::default_for(uid);
        prefs.quiet_hours_start = Some(13);
        prefs.quiet_hours_end = Some(15);
        dispatcher.set_preferences(prefs);

        dispatcher.enqueue(make_notif(Priority::Normal, Channel::InApp, uid));
        let (delivered, _) = dispatcher.process(14); // during quiet hours
        assert_eq!(delivered, 0);
        assert_eq!(dispatcher.queue_len(), 1); // skipped notification stays in queue

        // Process again outside quiet hours — the retained notification is delivered
        dispatcher.enqueue(make_notif(Priority::Normal, Channel::InApp, uid));
        let (delivered, _) = dispatcher.process(16); // outside quiet hours
        assert_eq!(delivered, 2); // both the retained + new notification
    }
}
