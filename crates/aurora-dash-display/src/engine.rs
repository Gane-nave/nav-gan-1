/// Dashboard display management: instrument cluster, HUD, info panels
/// Phase 146

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayMode {
    Normal,
    Sport,
    Eco,
    Navigation,
    Minimal,
    Night,
}

impl DisplayMode {
    pub fn brightness_pct(&self) -> f64 {
        match self {
            DisplayMode::Normal => 70.0,
            DisplayMode::Sport => 80.0,
            DisplayMode::Eco => 50.0,
            DisplayMode::Navigation => 75.0,
            DisplayMode::Minimal => 40.0,
            DisplayMode::Night => 20.0,
        }
    }

    pub fn info_density(&self) -> &'static str {
        match self {
            DisplayMode::Normal => "medium",
            DisplayMode::Sport => "high",
            DisplayMode::Eco => "low",
            DisplayMode::Navigation => "medium",
            DisplayMode::Minimal => "minimal",
            DisplayMode::Night => "low",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WarningLevel {
    Info,
    Caution,
    Warning,
    Critical,
}

impl WarningLevel {
    pub fn color(&self) -> &'static str {
        match self {
            WarningLevel::Info => "white",
            WarningLevel::Caution => "yellow",
            WarningLevel::Warning => "orange",
            WarningLevel::Critical => "red",
        }
    }

    pub fn requires_ack(&self) -> bool {
        matches!(self, WarningLevel::Warning | WarningLevel::Critical)
    }

    pub fn audible_alert(&self) -> bool {
        matches!(self, WarningLevel::Critical)
    }
}

#[derive(Debug, Clone)]
pub struct DashAlert {
    pub message: String,
    pub level: WarningLevel,
    pub acknowledged: bool,
}

impl DashAlert {
    pub fn new(msg: &str, level: WarningLevel) -> Self {
        Self {
            message: msg.to_string(),
            level,
            acknowledged: false,
        }
    }

    pub fn needs_attention(&self) -> bool {
        self.level.requires_ack() && !self.acknowledged
    }
}

#[derive(Debug, Clone)]
pub struct DashDisplay {
    pub mode: DisplayMode,
    pub brightness_override: Option<f64>,
    pub alerts: Vec<DashAlert>,
    pub speed_kmh: f64,
    pub rpm: f64,
    pub fuel_pct: f64,
}

impl DashDisplay {
    pub fn new() -> Self {
        Self {
            mode: DisplayMode::Normal,
            brightness_override: None,
            alerts: Vec::new(),
            speed_kmh: 0.0,
            rpm: 0.0,
            fuel_pct: 100.0,
        }
    }

    pub fn effective_brightness(&self) -> f64 {
        self.brightness_override
            .unwrap_or_else(|| self.mode.brightness_pct())
    }

    pub fn add_alert(&mut self, alert: DashAlert) {
        self.alerts.push(alert);
    }

    pub fn active_alerts(&self) -> usize {
        self.alerts.iter().filter(|a| a.needs_attention()).count()
    }

    pub fn critical_alerts(&self) -> usize {
        self.alerts
            .iter()
            .filter(|a| a.level == WarningLevel::Critical)
            .count()
    }

    pub fn has_critical(&self) -> bool {
        self.critical_alerts() > 0
    }

    pub fn power_draw_watts(&self) -> f64 {
        self.effective_brightness() * 0.5
    }
}

impl Default for DashDisplay {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_brightness() {
        assert!(DisplayMode::Sport.brightness_pct() > DisplayMode::Night.brightness_pct());
    }

    #[test]
    fn test_info_density() {
        assert_eq!(DisplayMode::Sport.info_density(), "high");
    }

    #[test]
    fn test_warning_color() {
        assert_eq!(WarningLevel::Critical.color(), "red");
    }

    #[test]
    fn test_requires_ack() {
        assert!(WarningLevel::Critical.requires_ack());
        assert!(!WarningLevel::Info.requires_ack());
    }

    #[test]
    fn test_alert_attention() {
        let a = DashAlert::new("Low fuel", WarningLevel::Warning);
        assert!(a.needs_attention());
    }

    #[test]
    fn test_alert_acked() {
        let mut a = DashAlert::new("Low fuel", WarningLevel::Warning);
        a.acknowledged = true;
        assert!(!a.needs_attention());
    }

    #[test]
    fn test_brightness_override() {
        let mut d = DashDisplay::new();
        d.brightness_override = Some(90.0);
        assert!((d.effective_brightness() - 90.0).abs() < 0.1);
    }

    #[test]
    fn test_active_alerts() {
        let mut d = DashDisplay::new();
        d.add_alert(DashAlert::new("Check engine", WarningLevel::Warning));
        d.add_alert(DashAlert::new("Info", WarningLevel::Info));
        assert_eq!(d.active_alerts(), 1);
    }

    #[test]
    fn test_critical() {
        let mut d = DashDisplay::new();
        d.add_alert(DashAlert::new("Brake failure", WarningLevel::Critical));
        assert!(d.has_critical());
    }

    #[test]
    fn test_power_draw() {
        let d = DashDisplay::new();
        assert!(d.power_draw_watts() > 0.0);
    }
}
