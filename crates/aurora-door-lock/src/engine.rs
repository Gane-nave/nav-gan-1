/// Door lock management: keyless entry, child locks, auto-lock, theft prevention
/// Phase 142

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DoorPosition {
    FrontLeft,
    FrontRight,
    RearLeft,
    RearRight,
    Trunk,
    FuelCap,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LockState {
    Locked,
    Unlocked,
    ChildLocked,
    DeadLocked,
}

impl LockState {
    pub fn is_secure(&self) -> bool {
        matches!(
            self,
            LockState::Locked | LockState::ChildLocked | LockState::DeadLocked
        )
    }
    pub fn can_open_inside(&self) -> bool {
        matches!(self, LockState::Unlocked)
    }
}

#[derive(Debug, Clone)]
pub struct Door {
    pub position: DoorPosition,
    pub state: LockState,
    pub is_open: bool,
    pub ajar_seconds: f64,
}

impl Door {
    pub fn new(pos: DoorPosition) -> Self {
        Self {
            position: pos,
            state: LockState::Locked,
            is_open: false,
            ajar_seconds: 0.0,
        }
    }
    pub fn is_secure(&self) -> bool {
        self.state.is_secure() && !self.is_open
    }
    pub fn ajar_warning(&self) -> bool {
        self.is_open && self.ajar_seconds > 30.0
    }
    pub fn is_child_safe(&self) -> bool {
        match self.position {
            DoorPosition::RearLeft | DoorPosition::RearRight => {
                self.state == LockState::ChildLocked
            }
            _ => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LockSystem {
    pub doors: Vec<Door>,
    pub auto_lock_speed_kmh: f64,
    pub keyless_range_m: f64,
}

impl Default for LockSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl LockSystem {
    pub fn new() -> Self {
        Self {
            doors: vec![
                Door::new(DoorPosition::FrontLeft),
                Door::new(DoorPosition::FrontRight),
                Door::new(DoorPosition::RearLeft),
                Door::new(DoorPosition::RearRight),
                Door::new(DoorPosition::Trunk),
            ],
            auto_lock_speed_kmh: 15.0,
            keyless_range_m: 3.0,
        }
    }
    pub fn all_secure(&self) -> bool {
        self.doors.iter().all(|d| d.is_secure())
    }
    pub fn any_open(&self) -> bool {
        self.doors.iter().any(|d| d.is_open)
    }
    pub fn any_ajar_warning(&self) -> bool {
        self.doors.iter().any(|d| d.ajar_warning())
    }
    pub fn should_auto_lock(&self, speed_kmh: f64) -> bool {
        speed_kmh >= self.auto_lock_speed_kmh && !self.all_secure()
    }
    pub fn open_door_count(&self) -> usize {
        self.doors.iter().filter(|d| d.is_open).count()
    }
    pub fn child_safe(&self) -> bool {
        self.doors.iter().all(|d| d.is_child_safe())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lock_secure() {
        assert!(LockState::Locked.is_secure());
        assert!(!LockState::Unlocked.is_secure());
    }
    #[test]
    fn test_door_secure() {
        let d = Door::new(DoorPosition::FrontLeft);
        assert!(d.is_secure());
    }
    #[test]
    fn test_door_open() {
        let mut d = Door::new(DoorPosition::FrontLeft);
        d.is_open = true;
        assert!(!d.is_secure());
    }
    #[test]
    fn test_ajar_warning() {
        let mut d = Door::new(DoorPosition::FrontLeft);
        d.is_open = true;
        d.ajar_seconds = 60.0;
        assert!(d.ajar_warning());
    }
    #[test]
    fn test_child_safe() {
        let mut d = Door::new(DoorPosition::RearLeft);
        d.state = LockState::ChildLocked;
        assert!(d.is_child_safe());
    }
    #[test]
    fn test_all_secure() {
        let s = LockSystem::new();
        assert!(s.all_secure());
    }
    #[test]
    fn test_any_open() {
        let mut s = LockSystem::new();
        s.doors[0].is_open = true;
        assert!(s.any_open());
    }
    #[test]
    fn test_auto_lock() {
        let mut s = LockSystem::new();
        s.doors[0].state = LockState::Unlocked;
        assert!(s.should_auto_lock(20.0));
    }
    #[test]
    fn test_no_auto_lock() {
        let s = LockSystem::new();
        assert!(!s.should_auto_lock(20.0));
    }
    #[test]
    fn test_open_count() {
        let mut s = LockSystem::new();
        s.doors[0].is_open = true;
        s.doors[1].is_open = true;
        assert_eq!(s.open_door_count(), 2);
    }
}
