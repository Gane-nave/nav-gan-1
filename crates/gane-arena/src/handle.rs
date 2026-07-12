/// A handle to an object in the arena, identified by index and generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArenaHandle {
    index: usize,
    generation: u64,
}

impl ArenaHandle {
    /// Create a new arena handle.
    pub fn new(index: usize, generation: u64) -> Self {
        Self { index, generation }
    }

    /// Get the index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Get the generation.
    pub fn generation(&self) -> u64 {
        self.generation
    }
}

/// Status of a slot in the arena.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlotStatus {
    /// Slot is free and available for allocation.
    Free,
    /// Slot is occupied with a live object.
    Occupied,
    /// Slot was freed and is in the recycling pool.
    Recycled,
}

/// A slot in the arena that can hold an object.
#[derive(Debug, Clone)]
pub struct ArenaSlot<T: Clone> {
    value: Option<T>,
    generation: u64,
    status: SlotStatus,
    alloc_time_ms: u64,
    free_time_ms: u64,
}

impl<T: Clone> ArenaSlot<T> {
    /// Create a new free slot.
    pub fn new() -> Self {
        Self {
            value: None,
            generation: 0,
            status: SlotStatus::Free,
            alloc_time_ms: 0,
            free_time_ms: 0,
        }
    }

    /// Allocate this slot with a value.
    pub fn allocate(&mut self, value: T, now_ms: u64) -> ArenaHandle {
        self.generation = self.generation.saturating_add(1);
        self.value = Some(value);
        self.status = SlotStatus::Occupied;
        self.alloc_time_ms = now_ms;
        self.free_time_ms = 0;
        ArenaHandle::new(0, self.generation) // index set by pool
    }

    /// Free this slot, returning the value if it was occupied.
    pub fn free(&mut self, now_ms: u64) -> Option<T> {
        if self.status != SlotStatus::Occupied {
            return None;
        }
        self.status = SlotStatus::Recycled;
        self.free_time_ms = now_ms;
        self.value.take()
    }

    /// Get the value if occupied.
    pub fn get(&self) -> Option<&T> {
        if self.status == SlotStatus::Occupied {
            self.value.as_ref()
        } else {
            None
        }
    }

    /// Get a mutable reference to the value if occupied.
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.status == SlotStatus::Occupied {
            self.value.as_mut()
        } else {
            None
        }
    }

    /// Get the current generation.
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Get the slot status.
    pub fn status(&self) -> &SlotStatus {
        &self.status
    }

    /// Get allocation time.
    pub fn alloc_time_ms(&self) -> u64 {
        self.alloc_time_ms
    }

    /// Get free time.
    pub fn free_time_ms(&self) -> u64 {
        self.free_time_ms
    }

    /// Mark as free (for initial setup).
    pub fn mark_free(&mut self) {
        self.status = SlotStatus::Free;
        self.value = None;
    }
}

impl<T: Clone> Default for ArenaSlot<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_creation() {
        let h = ArenaHandle::new(5, 3);
        assert_eq!(h.index(), 5);
        assert_eq!(h.generation(), 3);
    }

    #[test]
    fn test_handle_equality() {
        let h1 = ArenaHandle::new(1, 1);
        let h2 = ArenaHandle::new(1, 1);
        let h3 = ArenaHandle::new(1, 2);
        assert_eq!(h1, h2);
        assert_ne!(h1, h3);
    }

    #[test]
    fn test_slot_new() {
        let slot: ArenaSlot<u32> = ArenaSlot::new();
        assert_eq!(*slot.status(), SlotStatus::Free);
        assert_eq!(slot.get(), None);
        assert_eq!(slot.generation(), 0);
    }

    #[test]
    fn test_slot_allocate() {
        let mut slot: ArenaSlot<u32> = ArenaSlot::new();
        let handle = slot.allocate(42, 1000);
        assert_eq!(*slot.status(), SlotStatus::Occupied);
        assert_eq!(slot.get(), Some(&42));
        assert_eq!(handle.generation(), 1);
        assert_eq!(slot.alloc_time_ms(), 1000);
    }

    #[test]
    fn test_slot_free() {
        let mut slot: ArenaSlot<String> = ArenaSlot::new();
        slot.allocate("hello".to_string(), 100);
        let freed = slot.free(200);
        assert_eq!(freed, Some("hello".to_string()));
        assert_eq!(*slot.status(), SlotStatus::Recycled);
        assert_eq!(slot.free_time_ms(), 200);
        assert_eq!(slot.get(), None);
    }

    #[test]
    fn test_slot_double_free() {
        let mut slot: ArenaSlot<u32> = ArenaSlot::new();
        slot.allocate(10, 100);
        assert!(slot.free(200).is_some());
        assert!(slot.free(300).is_none()); // already freed
    }

    #[test]
    fn test_slot_get_mut() {
        let mut slot: ArenaSlot<u32> = ArenaSlot::new();
        slot.allocate(10, 100);
        if let Some(val) = slot.get_mut() {
            *val = 20;
        }
        assert_eq!(slot.get(), Some(&20));
    }

    #[test]
    fn test_slot_generation_increments() {
        let mut slot: ArenaSlot<u32> = ArenaSlot::new();
        let h1 = slot.allocate(1, 100);
        slot.free(200);
        slot.mark_free();
        let h2 = slot.allocate(2, 300);
        assert_eq!(h1.generation(), 1);
        assert_eq!(h2.generation(), 2);
    }
}
