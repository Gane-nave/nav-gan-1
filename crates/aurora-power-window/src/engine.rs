/// Power window control: position tracking, anti-pinch, auto up/down
/// Phase 175

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowPosition {
    FrontLeft,
    FrontRight,
    RearLeft,
    RearRight,
}

#[derive(Debug, Clone)]
pub struct PowerWindow {
    pub position: WindowPosition,
    pub open_pct: f64,
    pub auto_enabled: bool,
    pub anti_pinch: bool,
    pub motor_current_a: f64,
}

impl PowerWindow {
    pub fn new(position: WindowPosition) -> Self {
        Self {
            position,
            open_pct: 0.0,
            auto_enabled: true,
            anti_pinch: true,
            motor_current_a: 0.0,
        }
    }

    pub fn is_closed(&self) -> bool {
        self.open_pct < 1.0
    }

    pub fn is_fully_open(&self) -> bool {
        self.open_pct > 99.0
    }

    pub fn obstruction_detected(&self) -> bool {
        self.anti_pinch && self.motor_current_a > 8.0
    }

    pub fn rain_warning(&self) -> bool {
        self.open_pct > 10.0
    }
}

#[derive(Debug, Clone)]
pub struct WindowSystem {
    pub windows: Vec<PowerWindow>,
}

impl Default for WindowSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowSystem {
    pub fn new() -> Self {
        Self {
            windows: vec![
                PowerWindow::new(WindowPosition::FrontLeft),
                PowerWindow::new(WindowPosition::FrontRight),
                PowerWindow::new(WindowPosition::RearLeft),
                PowerWindow::new(WindowPosition::RearRight),
            ],
        }
    }

    pub fn all_closed(&self) -> bool {
        self.windows.iter().all(|w| w.is_closed())
    }

    pub fn any_open(&self) -> bool {
        self.windows.iter().any(|w| !w.is_closed())
    }

    pub fn rain_risk(&self) -> bool {
        self.windows.iter().any(|w| w.rain_warning())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closed() {
        let w = PowerWindow::new(WindowPosition::FrontLeft);
        assert!(w.is_closed());
    }

    #[test]
    fn test_open() {
        let mut w = PowerWindow::new(WindowPosition::FrontLeft);
        w.open_pct = 100.0;
        assert!(w.is_fully_open());
    }

    #[test]
    fn test_obstruction() {
        let mut w = PowerWindow::new(WindowPosition::RearLeft);
        w.motor_current_a = 10.0;
        assert!(w.obstruction_detected());
    }

    #[test]
    fn test_system_closed() {
        let s = WindowSystem::new();
        assert!(s.all_closed());
    }

    #[test]
    fn test_system_open() {
        let mut s = WindowSystem::new();
        s.windows[0].open_pct = 50.0;
        assert!(s.any_open());
    }

    #[test]
    fn test_rain_risk() {
        let mut s = WindowSystem::new();
        s.windows[1].open_pct = 30.0;
        assert!(s.rain_risk());
    }

    #[test]
    fn test_no_rain_risk() {
        let s = WindowSystem::new();
        assert!(!s.rain_risk());
    }
}
