//! Hierarchical timer wheel implementation.

use std::collections::HashMap;

/// A timer entry with an ID and expiration tick.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimerEntry {
    /// Unique timer ID.
    pub id: u64,
    /// The tick at which this timer expires.
    pub expires_at: u64,
    /// Optional label for the timer.
    pub label: String,
}

/// A timer wheel for efficiently managing timeouts.
///
/// Slots are arranged in a circular buffer. Each tick advances the
/// current position, firing all timers in the current slot.
#[derive(Debug)]
pub struct TimerWheel {
    /// Circular buffer of timer slots.
    slots: Vec<Vec<TimerEntry>>,
    /// Number of slots in the wheel.
    num_slots: usize,
    /// Current tick position.
    current_tick: u64,
    /// Map from timer ID to slot index for O(1) cancellation.
    timer_map: HashMap<u64, usize>,
    /// Next timer ID to assign.
    next_id: u64,
    /// Total timers scheduled.
    scheduled: u64,
    /// Total timers fired.
    fired: u64,
    /// Total timers cancelled.
    cancelled: u64,
}

impl TimerWheel {
    /// Create a new timer wheel with the given number of slots.
    pub fn new(num_slots: usize) -> Self {
        assert!(num_slots > 0, "num_slots must be > 0");
        Self {
            slots: (0..num_slots).map(|_| Vec::new()).collect(),
            num_slots,
            current_tick: 0,
            timer_map: HashMap::new(),
            next_id: 1,
            scheduled: 0,
            fired: 0,
            cancelled: 0,
        }
    }

    /// Schedule a timer to fire after `delay` ticks.
    /// A delay of 0 fires on the very next tick.
    /// Returns the timer ID.
    pub fn schedule(&mut self, delay: u64, label: &str) -> u64 {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        let effective_delay = delay.max(1);
        let expires_at = self.current_tick.saturating_add(effective_delay);
        let slot = (expires_at as usize) % self.num_slots;
        let entry = TimerEntry {
            id,
            expires_at,
            label: label.to_string(),
        };
        self.slots[slot].push(entry);
        self.timer_map.insert(id, slot);
        self.scheduled = self.scheduled.saturating_add(1);
        id
    }

    /// Cancel a timer by ID. Returns `true` if the timer was found and cancelled.
    pub fn cancel(&mut self, id: u64) -> bool {
        if let Some(slot) = self.timer_map.remove(&id) {
            self.slots[slot].retain(|e| e.id != id);
            self.cancelled = self.cancelled.saturating_add(1);
            true
        } else {
            false
        }
    }

    /// Advance the wheel by one tick and return all fired timers.
    pub fn tick(&mut self) -> Vec<TimerEntry> {
        self.current_tick = self.current_tick.saturating_add(1);
        let slot = (self.current_tick as usize) % self.num_slots;
        let mut fired = Vec::new();
        let mut remaining = Vec::new();

        for entry in self.slots[slot].drain(..) {
            if entry.expires_at <= self.current_tick {
                self.timer_map.remove(&entry.id);
                self.fired = self.fired.saturating_add(1);
                fired.push(entry);
            } else {
                remaining.push(entry);
            }
        }
        self.slots[slot] = remaining;
        fired
    }

    /// Advance the wheel by `n` ticks, collecting all fired timers.
    pub fn advance(&mut self, n: u64) -> Vec<TimerEntry> {
        let mut all_fired = Vec::new();
        for _ in 0..n {
            all_fired.extend(self.tick());
        }
        all_fired
    }

    /// Current tick position.
    pub fn current_tick(&self) -> u64 {
        self.current_tick
    }

    /// Number of pending timers.
    pub fn pending(&self) -> usize {
        self.timer_map.len()
    }

    /// Number of slots.
    pub fn num_slots(&self) -> usize {
        self.num_slots
    }

    /// Check if a timer exists.
    pub fn has_timer(&self, id: u64) -> bool {
        self.timer_map.contains_key(&id)
    }

    /// Total timers scheduled.
    pub fn total_scheduled(&self) -> u64 {
        self.scheduled
    }

    /// Total timers fired.
    pub fn total_fired(&self) -> u64 {
        self.fired
    }

    /// Total timers cancelled.
    pub fn total_cancelled(&self) -> u64 {
        self.cancelled
    }

    /// Check if the wheel has no pending timers.
    pub fn is_empty(&self) -> bool {
        self.timer_map.is_empty()
    }

    /// Clear all pending timers.
    pub fn clear(&mut self) {
        for slot in &mut self.slots {
            slot.clear();
        }
        self.timer_map.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_wheel() {
        let w = TimerWheel::new(16);
        assert_eq!(w.num_slots(), 16);
        assert_eq!(w.current_tick(), 0);
        assert_eq!(w.pending(), 0);
        assert!(w.is_empty());
    }

    #[test]
    fn test_schedule_timer() {
        let mut w = TimerWheel::new(16);
        let id = w.schedule(5, "timeout");
        assert!(w.has_timer(id));
        assert_eq!(w.pending(), 1);
        assert_eq!(w.total_scheduled(), 1);
    }

    #[test]
    fn test_fire_timer() {
        let mut w = TimerWheel::new(16);
        let id = w.schedule(3, "test");
        // Advance 3 ticks
        let fired = w.advance(3);
        assert_eq!(fired.len(), 1);
        assert_eq!(fired[0].id, id);
        assert_eq!(fired[0].label, "test");
        assert!(!w.has_timer(id));
        assert_eq!(w.total_fired(), 1);
    }

    #[test]
    fn test_cancel_timer() {
        let mut w = TimerWheel::new(16);
        let id = w.schedule(5, "cancel-me");
        assert!(w.cancel(id));
        assert!(!w.has_timer(id));
        assert_eq!(w.total_cancelled(), 1);
        // Advancing should not fire it
        let fired = w.advance(5);
        assert!(fired.is_empty());
    }

    #[test]
    fn test_cancel_nonexistent() {
        let mut w = TimerWheel::new(16);
        assert!(!w.cancel(999));
    }

    #[test]
    fn test_multiple_timers_same_slot() {
        let mut w = TimerWheel::new(16);
        let id1 = w.schedule(3, "a");
        let id2 = w.schedule(3, "b");
        let fired = w.advance(3);
        assert_eq!(fired.len(), 2);
        let ids: Vec<u64> = fired.iter().map(|e| e.id).collect();
        assert!(ids.contains(&id1));
        assert!(ids.contains(&id2));
    }

    #[test]
    fn test_different_delays() {
        let mut w = TimerWheel::new(16);
        w.schedule(2, "fast");
        w.schedule(5, "slow");
        let fired2 = w.advance(2);
        assert_eq!(fired2.len(), 1);
        assert_eq!(fired2[0].label, "fast");
        let fired5 = w.advance(3);
        assert_eq!(fired5.len(), 1);
        assert_eq!(fired5[0].label, "slow");
    }

    #[test]
    fn test_wrap_around() {
        let mut w = TimerWheel::new(4);
        w.schedule(5, "wrap"); // wraps around the wheel
        let fired = w.advance(5);
        assert_eq!(fired.len(), 1);
        assert_eq!(fired[0].label, "wrap");
    }

    #[test]
    fn test_tick_no_timers() {
        let mut w = TimerWheel::new(16);
        let fired = w.tick();
        assert!(fired.is_empty());
    }

    #[test]
    fn test_clear() {
        let mut w = TimerWheel::new(16);
        w.schedule(1, "a");
        w.schedule(2, "b");
        w.clear();
        assert!(w.is_empty());
        assert_eq!(w.pending(), 0);
    }

    #[test]
    fn test_delay_zero() {
        let mut w = TimerWheel::new(16);
        w.schedule(0, "immediate");
        // delay=0 is treated as delay=1 (fires on very next tick)
        let fired = w.tick();
        assert_eq!(fired.len(), 1);
        assert_eq!(fired[0].label, "immediate");
    }

    #[test]
    fn test_sequential_ids() {
        let mut w = TimerWheel::new(16);
        let id1 = w.schedule(1, "a");
        let id2 = w.schedule(1, "b");
        let id3 = w.schedule(1, "c");
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(id3, 3);
    }

    #[test]
    fn test_stats() {
        let mut w = TimerWheel::new(16);
        w.schedule(1, "fire");
        w.schedule(10, "cancel");
        w.cancel(2);
        w.advance(1);
        assert_eq!(w.total_scheduled(), 2);
        assert_eq!(w.total_fired(), 1);
        assert_eq!(w.total_cancelled(), 1);
    }

    #[test]
    #[should_panic]
    fn test_zero_slots() {
        TimerWheel::new(0);
    }

    #[test]
    fn test_large_delay() {
        let mut w = TimerWheel::new(8);
        let id = w.schedule(100, "far");
        assert!(w.has_timer(id));
        let fired = w.advance(100);
        assert_eq!(fired.len(), 1);
    }
}
