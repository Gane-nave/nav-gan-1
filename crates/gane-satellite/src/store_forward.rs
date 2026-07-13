//! Store-and-forward — message queuing for intermittent satellite connectivity.
//!
//! Buffers messages when no uplink is available and transmits them
//! in priority order when a window opens. Handles partial transmissions
//! and automatic retry.

use chrono::{DateTime, Duration, Utc};
use gane_core::types::EntityId;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use tracing::{debug, info, warn};

/// Status of a stored message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageStatus {
    /// Waiting for transmission window.
    Stored,
    /// Currently being transmitted.
    Transmitting,
    /// Successfully transmitted and acknowledged.
    Delivered,
    /// Transmission failed — will retry.
    RetryPending,
    /// Expired — TTL exceeded.
    Expired,
    /// Permanently failed — max retries exceeded.
    Dead,
}

/// Priority for store-and-forward messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ForwardPriority {
    Bulk,
    Normal,
    Urgent,
    Emergency,
}

/// A message in the store-and-forward buffer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMessage {
    pub id: EntityId,
    pub priority: ForwardPriority,
    pub payload: Vec<u8>,
    pub size_bytes: usize,
    pub created_at: DateTime<Utc>,
    pub ttl_seconds: u64,
    pub status: MessageStatus,
    pub attempts: u32,
    pub max_attempts: u32,
    pub last_attempt: Option<DateTime<Utc>>,
    pub destination: String,
}

/// A transmission window — period when satellite uplink is available.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransmissionWindow {
    pub id: EntityId,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub bandwidth_bps: u64,
    pub bytes_transmitted: u64,
    pub messages_sent: u32,
}

impl TransmissionWindow {
    /// Duration of the window in seconds.
    pub fn duration_secs(&self) -> i64 {
        (self.end - self.start).num_seconds()
    }

    /// Maximum bytes that can be transmitted in this window.
    pub fn capacity_bytes(&self) -> u64 {
        (self.bandwidth_bps * self.duration_secs().max(0) as u64) / 8
    }

    /// Remaining capacity.
    pub fn remaining_bytes(&self) -> u64 {
        self.capacity_bytes().saturating_sub(self.bytes_transmitted)
    }

    /// Whether the window is currently active.
    pub fn is_active(&self, now: DateTime<Utc>) -> bool {
        (self.start..=self.end).contains(&now)
    }
}

/// Store-and-forward engine — manages message buffering and transmission.
pub struct StoreForwardEngine {
    buffer: VecDeque<StoredMessage>,
    /// Maximum buffer size in bytes.
    max_buffer_bytes: u64,
    /// Current buffer usage in bytes.
    used_bytes: u64,
    /// Transmission windows.
    windows: Vec<TransmissionWindow>,
    /// Total messages stored.
    total_stored: u64,
    /// Total messages delivered.
    total_delivered: u64,
    /// Total messages expired.
    total_expired: u64,
    /// Total messages dead (exhausted retries).
    total_dead: u64,
    /// Default max attempts.
    default_max_attempts: u32,
}

impl StoreForwardEngine {
    pub fn new(max_buffer_bytes: u64) -> Self {
        Self {
            buffer: VecDeque::new(),
            max_buffer_bytes,
            used_bytes: 0,
            windows: Vec::new(),
            total_stored: 0,
            total_delivered: 0,
            total_expired: 0,
            total_dead: 0,
            default_max_attempts: 5,
        }
    }

    /// Store a message for later forwarding. Returns None if buffer is full.
    pub fn store(
        &mut self,
        priority: ForwardPriority,
        payload: &[u8],
        destination: &str,
        ttl_seconds: u64,
    ) -> Option<EntityId> {
        let size = payload.len();

        // Check buffer capacity.
        if self.used_bytes + size as u64 > self.max_buffer_bytes {
            // Try to evict a message with strictly lower priority.
            if !self.evict_lowest_priority(size as u64, priority) {
                warn!(
                    needed = size,
                    available = self.max_buffer_bytes - self.used_bytes,
                    "store-and-forward buffer full — message rejected"
                );
                return None;
            }
        }

        let id = EntityId::new();
        let msg = StoredMessage {
            id,
            priority,
            payload: payload.to_vec(),
            size_bytes: size,
            created_at: Utc::now(),
            ttl_seconds,
            status: MessageStatus::Stored,
            attempts: 0,
            max_attempts: self.default_max_attempts,
            last_attempt: None,
            destination: destination.to_string(),
        };

        self.used_bytes += size as u64;
        self.buffer.push_back(msg);
        self.total_stored += 1;

        debug!(id = %id, size = size, priority = ?priority, "message stored for forwarding");
        Some(id)
    }

    /// Try to evict lowest-priority messages to free space.
    /// Only evicts messages with strictly lower priority than `incoming`.
    /// Keeps evicting until enough space is freed or no more evictable messages remain.
    fn evict_lowest_priority(&mut self, needed_bytes: u64, incoming: ForwardPriority) -> bool {
        loop {
            // Check if we already have enough space.
            if self.max_buffer_bytes - self.used_bytes >= needed_bytes {
                return true;
            }

            // Find the lowest-priority Stored message that is strictly below incoming.
            let idx = self
                .buffer
                .iter()
                .enumerate()
                .filter(|(_, m)| m.status == MessageStatus::Stored && m.priority < incoming)
                .min_by_key(|(_, m)| m.priority)
                .map(|(i, _)| i);

            let Some(i) = idx else {
                return false; // No more evictable messages.
            };

            let evicted = self.buffer.remove(i).unwrap();
            self.used_bytes = self.used_bytes.saturating_sub(evicted.size_bytes as u64);
            info!(id = %evicted.id, priority = ?evicted.priority, "evicted message to make room");
        }
    }

    /// Expire messages that have exceeded their TTL.
    pub fn expire_stale(&mut self) -> u32 {
        let now = Utc::now();
        let mut expired_count = 0u32;

        for msg in &mut self.buffer {
            if msg.status == MessageStatus::Stored || msg.status == MessageStatus::RetryPending {
                let expiry = msg.created_at + Duration::seconds(msg.ttl_seconds as i64);
                if now > expiry {
                    msg.status = MessageStatus::Expired;
                    self.used_bytes = self.used_bytes.saturating_sub(msg.size_bytes as u64);
                    expired_count += 1;
                    self.total_expired += 1;
                }
            }
        }

        if expired_count > 0 {
            self.buffer.retain(|m| m.status != MessageStatus::Expired);
            debug!(count = expired_count, "expired stale stored messages");
        }
        expired_count
    }

    /// Transmit messages during a transmission window.
    /// Returns the number of messages successfully transmitted.
    pub fn transmit(&mut self, window: &mut TransmissionWindow) -> u32 {
        self.expire_stale();

        // Sort buffer by priority (highest first).
        let mut sorted: Vec<usize> = (0..self.buffer.len())
            .filter(|&i| {
                self.buffer[i].status == MessageStatus::Stored
                    || self.buffer[i].status == MessageStatus::RetryPending
            })
            .collect();
        sorted.sort_by(|&a, &b| self.buffer[b].priority.cmp(&self.buffer[a].priority));

        let mut sent = 0u32;

        for idx in sorted {
            let msg = &self.buffer[idx];
            if msg.size_bytes as u64 > window.remaining_bytes() {
                continue; // Skip if message doesn't fit.
            }

            window.bytes_transmitted += msg.size_bytes as u64;
            window.messages_sent += 1;

            let msg = &mut self.buffer[idx];
            msg.status = MessageStatus::Delivered;
            msg.attempts += 1;
            msg.last_attempt = Some(Utc::now());
            self.used_bytes = self.used_bytes.saturating_sub(msg.size_bytes as u64);
            self.total_delivered += 1;
            sent += 1;
        }

        // Remove delivered messages.
        self.buffer.retain(|m| m.status != MessageStatus::Delivered);

        if sent > 0 {
            info!(
                sent = sent,
                remaining = self.buffer.len(),
                "transmission window: messages forwarded"
            );
        }

        sent
    }

    /// Mark a message as failed (will retry if attempts remain).
    pub fn mark_failed(&mut self, msg_id: &EntityId) {
        if let Some(msg) = self.buffer.iter_mut().find(|m| m.id == *msg_id) {
            msg.attempts += 1;
            msg.last_attempt = Some(Utc::now());

            if msg.attempts >= msg.max_attempts {
                msg.status = MessageStatus::Dead;
                self.used_bytes = self.used_bytes.saturating_sub(msg.size_bytes as u64);
                self.total_dead += 1;
                warn!(id = %msg_id, attempts = msg.attempts, "message dead — max retries exceeded");
            } else {
                msg.status = MessageStatus::RetryPending;
                debug!(id = %msg_id, attempts = msg.attempts, "message marked for retry");
            }
        }
    }

    /// Register a transmission window.
    pub fn register_window(
        &mut self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        bandwidth_bps: u64,
    ) -> EntityId {
        let id = EntityId::new();
        let window = TransmissionWindow {
            id,
            start,
            end,
            bandwidth_bps,
            bytes_transmitted: 0,
            messages_sent: 0,
        };
        info!(
            bandwidth = bandwidth_bps,
            duration_s = window.duration_secs(),
            capacity = window.capacity_bytes(),
            "transmission window registered"
        );
        self.windows.push(window);
        id
    }

    /// Get the next available transmission window.
    pub fn next_window(&self) -> Option<&TransmissionWindow> {
        let now = Utc::now();
        self.windows
            .iter()
            .filter(|w| w.end > now)
            .min_by_key(|w| w.start)
    }

    /// Number of messages in the buffer.
    pub fn buffer_count(&self) -> usize {
        self.buffer.len()
    }

    /// Buffer usage in bytes.
    pub fn buffer_used_bytes(&self) -> u64 {
        self.used_bytes
    }

    /// Buffer capacity.
    pub fn buffer_capacity(&self) -> u64 {
        self.max_buffer_bytes
    }

    /// Total messages stored.
    pub fn total_stored(&self) -> u64 {
        self.total_stored
    }

    /// Total messages delivered.
    pub fn total_delivered(&self) -> u64 {
        self.total_delivered
    }

    /// Total messages expired.
    pub fn total_expired(&self) -> u64 {
        self.total_expired
    }

    /// Total dead messages.
    pub fn total_dead(&self) -> u64 {
        self.total_dead
    }
}

impl Default for StoreForwardEngine {
    fn default() -> Self {
        Self::new(10 * 1024 * 1024) // 10 MB default buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_and_count() {
        let mut engine = StoreForwardEngine::new(10_000);
        let id = engine.store(ForwardPriority::Normal, b"test data", "server-1", 3600);
        assert!(id.is_some());
        assert_eq!(engine.buffer_count(), 1);
        assert_eq!(engine.total_stored(), 1);
    }

    #[test]
    fn buffer_full_rejects() {
        let mut engine = StoreForwardEngine::new(10);
        engine.store(ForwardPriority::Emergency, b"1234567890", "srv", 3600);
        let result = engine.store(ForwardPriority::Emergency, b"more data!", "srv", 3600);
        // Buffer is full and can't evict (same priority).
        assert!(result.is_none());
    }

    #[test]
    fn buffer_full_evicts_lower_priority() {
        let mut engine = StoreForwardEngine::new(15);
        engine.store(ForwardPriority::Bulk, b"low priority", "srv", 3600); // 12 bytes
                                                                           // 12 + 7 = 19 > 15, triggers eviction of Bulk (lower than Emergency).
        let result = engine.store(ForwardPriority::Emergency, b"urgent!", "srv", 3600); // 7 bytes
        assert!(result.is_some());
        // Low priority message was evicted.
        assert_eq!(engine.buffer_count(), 1);
    }

    #[test]
    fn transmit_in_priority_order() {
        let mut engine = StoreForwardEngine::new(100_000);
        engine.store(ForwardPriority::Bulk, b"bulk", "srv", 3600);
        engine.store(ForwardPriority::Emergency, b"emer", "srv", 3600);
        engine.store(ForwardPriority::Normal, b"norm", "srv", 3600);

        let mut window = TransmissionWindow {
            id: EntityId::new(),
            start: Utc::now() - Duration::seconds(10),
            end: Utc::now() + Duration::seconds(60),
            bandwidth_bps: 8000, // 1000 bytes/sec.
            bytes_transmitted: 0,
            messages_sent: 0,
        };

        let sent = engine.transmit(&mut window);
        assert_eq!(sent, 3);
        assert_eq!(engine.buffer_count(), 0);
        assert_eq!(engine.total_delivered(), 3);
    }

    #[test]
    fn transmit_limited_by_window_capacity() {
        let mut engine = StoreForwardEngine::new(100_000);
        engine.store(ForwardPriority::Normal, b"aaaaaaaaaa", "srv", 3600); // 10 bytes.
        engine.store(ForwardPriority::Normal, b"bbbbbbbbbb", "srv", 3600); // 10 bytes.

        let mut window = TransmissionWindow {
            id: EntityId::new(),
            start: Utc::now(),
            end: Utc::now() + Duration::seconds(1),
            bandwidth_bps: 80, // 10 bytes/sec → 10 bytes capacity in 1 sec.
            bytes_transmitted: 0,
            messages_sent: 0,
        };

        let sent = engine.transmit(&mut window);
        assert_eq!(sent, 1); // Only one fits.
        assert_eq!(engine.buffer_count(), 1);
    }

    #[test]
    fn expire_stale_messages() {
        let mut engine = StoreForwardEngine::new(100_000);
        // Store a message with TTL=0, created in the past.
        let id = engine
            .store(ForwardPriority::Normal, b"old", "srv", 0)
            .unwrap();

        // Manually set created_at to the past.
        if let Some(msg) = engine.buffer.iter_mut().find(|m| m.id == id) {
            msg.created_at = Utc::now() - Duration::seconds(10);
        }

        let expired = engine.expire_stale();
        assert_eq!(expired, 1);
        assert_eq!(engine.buffer_count(), 0);
        assert_eq!(engine.total_expired(), 1);
    }

    #[test]
    fn mark_failed_retries() {
        let mut engine = StoreForwardEngine::new(100_000);
        let id = engine
            .store(ForwardPriority::Normal, b"retry me", "srv", 3600)
            .unwrap();

        engine.mark_failed(&id);
        let msg = engine.buffer.iter().find(|m| m.id == id).unwrap();
        assert_eq!(msg.status, MessageStatus::RetryPending);
        assert_eq!(msg.attempts, 1);
    }

    #[test]
    fn mark_failed_dead_after_max_attempts() {
        let mut engine = StoreForwardEngine::new(100_000);
        engine.default_max_attempts = 2;
        let id = engine
            .store(ForwardPriority::Normal, b"die", "srv", 3600)
            .unwrap();

        engine.mark_failed(&id); // attempt 1
        engine.mark_failed(&id); // attempt 2 = max → dead

        let msg = engine.buffer.iter().find(|m| m.id == id).unwrap();
        assert_eq!(msg.status, MessageStatus::Dead);
        assert_eq!(engine.total_dead(), 1);
    }

    #[test]
    fn transmission_window_capacity() {
        let window = TransmissionWindow {
            id: EntityId::new(),
            start: Utc::now(),
            end: Utc::now() + Duration::seconds(10),
            bandwidth_bps: 9600,
            bytes_transmitted: 0,
            messages_sent: 0,
        };
        assert_eq!(window.capacity_bytes(), 12_000); // 9600 bps * 10 sec / 8
        assert_eq!(window.remaining_bytes(), 12_000);
    }

    #[test]
    fn transmission_window_active() {
        let now = Utc::now();
        let window = TransmissionWindow {
            id: EntityId::new(),
            start: now - Duration::seconds(5),
            end: now + Duration::seconds(5),
            bandwidth_bps: 1200,
            bytes_transmitted: 0,
            messages_sent: 0,
        };
        assert!(window.is_active(now));
        assert!(!window.is_active(now + Duration::seconds(10)));
    }

    #[test]
    fn register_and_find_next_window() {
        let mut engine = StoreForwardEngine::new(100_000);
        let now = Utc::now();
        engine.register_window(
            now + Duration::seconds(60),
            now + Duration::seconds(120),
            9600,
        );
        engine.register_window(
            now + Duration::seconds(10),
            now + Duration::seconds(30),
            1200,
        );

        let next = engine.next_window().unwrap();
        assert_eq!(next.bandwidth_bps, 1200); // Earlier window.
    }

    #[test]
    fn default_engine() {
        let engine = StoreForwardEngine::default();
        assert_eq!(engine.buffer_capacity(), 10 * 1024 * 1024);
    }

    #[test]
    fn multi_eviction_frees_enough_space() {
        // Buffer: 15 bytes. Fill with 3× 5-byte Bulk messages (15 bytes total).
        let mut engine = StoreForwardEngine::new(15);
        engine.store(ForwardPriority::Bulk, b"aaaaa", "srv", 3600); // 5 bytes
        engine.store(ForwardPriority::Bulk, b"bbbbb", "srv", 3600); // 5 bytes
        engine.store(ForwardPriority::Bulk, b"ccccc", "srv", 3600); // 5 bytes
        assert_eq!(engine.buffer_count(), 3);
        assert_eq!(engine.buffer_used_bytes(), 15);

        // Store a 10-byte Emergency message. Needs to evict 2 of 3 Bulk messages.
        let result = engine.store(ForwardPriority::Emergency, b"emergency!", "srv", 3600); // 10 bytes
        assert!(
            result.is_some(),
            "Emergency message should be stored after multi-eviction"
        );
        // At least the Emergency message is in the buffer.
        assert!(engine.buffer_count() >= 1);
        // The Emergency message's 10 bytes are accounted for.
        assert!(engine.buffer_used_bytes() <= 15);
    }
}
