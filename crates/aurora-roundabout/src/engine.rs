/// Roundabout navigation: entry timing, lane selection, exit guidance.
#[derive(Debug, Clone, PartialEq)]
pub enum RoundaboutSize {
    Mini,
    Single,
    Double,
    Turbo,
    Large,
}

impl RoundaboutSize {
    pub fn lanes(&self) -> u8 {
        match self {
            RoundaboutSize::Mini => 1,
            RoundaboutSize::Single => 1,
            RoundaboutSize::Double => 2,
            RoundaboutSize::Turbo => 2,
            RoundaboutSize::Large => 3,
        }
    }

    pub fn diameter_m(&self) -> f64 {
        match self {
            RoundaboutSize::Mini => 15.0,
            RoundaboutSize::Single => 30.0,
            RoundaboutSize::Double => 45.0,
            RoundaboutSize::Turbo => 50.0,
            RoundaboutSize::Large => 70.0,
        }
    }

    pub fn max_speed_kmh(&self) -> f64 {
        match self {
            RoundaboutSize::Mini => 20.0,
            RoundaboutSize::Single => 30.0,
            RoundaboutSize::Double => 35.0,
            RoundaboutSize::Turbo => 40.0,
            RoundaboutSize::Large => 45.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RoundaboutExit {
    pub angle_deg: f64,
    pub name: String,
    pub exit_number: u8,
}

impl RoundaboutExit {
    pub fn new(angle_deg: f64, name: &str, exit_number: u8) -> Self {
        Self {
            angle_deg,
            name: name.to_string(),
            exit_number,
        }
    }

    pub fn is_straight(&self) -> bool {
        (self.angle_deg - 180.0).abs() < 30.0
    }

    pub fn is_right_turn(&self) -> bool {
        self.angle_deg < 120.0
    }

    pub fn is_left_turn(&self) -> bool {
        self.angle_deg > 240.0
    }

    pub fn is_uturn(&self) -> bool {
        self.angle_deg > 330.0
    }
}

#[derive(Debug, Clone)]
pub struct Roundabout {
    pub size: RoundaboutSize,
    pub exits: Vec<RoundaboutExit>,
    pub traffic_volume: f64,
}

impl Roundabout {
    pub fn new(size: RoundaboutSize) -> Self {
        Self {
            size,
            exits: Vec::new(),
            traffic_volume: 0.5,
        }
    }

    pub fn add_exit(&mut self, exit: RoundaboutExit) {
        self.exits.push(exit);
    }

    pub fn num_exits(&self) -> usize {
        self.exits.len()
    }

    pub fn entry_speed_kmh(&self) -> f64 {
        self.size.max_speed_kmh() * (1.0 - self.traffic_volume * 0.5)
    }

    pub fn circulating_speed_kmh(&self) -> f64 {
        self.size.max_speed_kmh() * 0.8
    }

    pub fn estimated_traverse_sec(&self, target_exit: u8) -> f64 {
        let circumference = std::f64::consts::PI * self.size.diameter_m();
        let exit = self.exits.iter().find(|e| e.exit_number == target_exit);
        let fraction = match exit {
            Some(e) => e.angle_deg / 360.0,
            None => 0.5,
        };
        let distance = circumference * fraction;
        let speed_ms = self.circulating_speed_kmh() / 3.6;
        if speed_ms <= 0.0 {
            return f64::INFINITY;
        }
        distance / speed_ms + 3.0
    }

    pub fn recommended_lane(&self, target_exit: u8) -> u8 {
        if self.size.lanes() == 1 {
            return 1;
        }
        let exit = self.exits.iter().find(|e| e.exit_number == target_exit);
        match exit {
            Some(e) if e.is_right_turn() => self.size.lanes(),
            Some(e) if e.is_left_turn() || e.is_uturn() => 1,
            _ => 1,
        }
    }

    pub fn complexity(&self) -> f64 {
        let exit_factor = self.exits.len() as f64 * 10.0;
        let lane_factor = self.size.lanes() as f64 * 15.0;
        let traffic_factor = self.traffic_volume * 30.0;
        (exit_factor + lane_factor + traffic_factor).clamp(0.0, 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_lanes() {
        assert_eq!(RoundaboutSize::Mini.lanes(), 1);
        assert_eq!(RoundaboutSize::Large.lanes(), 3);
    }

    #[test]
    fn test_max_speed() {
        assert!(RoundaboutSize::Large.max_speed_kmh() > RoundaboutSize::Mini.max_speed_kmh());
    }

    #[test]
    fn test_exit_straight() {
        let e = RoundaboutExit::new(180.0, "Main St", 2);
        assert!(e.is_straight());
    }

    #[test]
    fn test_exit_right() {
        let e = RoundaboutExit::new(90.0, "First", 1);
        assert!(e.is_right_turn());
    }

    #[test]
    fn test_exit_left() {
        let e = RoundaboutExit::new(270.0, "Third", 3);
        assert!(e.is_left_turn());
    }

    #[test]
    fn test_exit_uturn() {
        let e = RoundaboutExit::new(350.0, "U-Turn", 4);
        assert!(e.is_uturn());
    }

    #[test]
    fn test_entry_speed() {
        let r = Roundabout::new(RoundaboutSize::Single);
        assert!(r.entry_speed_kmh() > 15.0);
    }

    #[test]
    fn test_traverse_time() {
        let mut r = Roundabout::new(RoundaboutSize::Single);
        r.add_exit(RoundaboutExit::new(90.0, "First", 1));
        let t = r.estimated_traverse_sec(1);
        assert!(t > 2.0 && t < 30.0);
    }

    #[test]
    fn test_recommended_lane_single() {
        let r = Roundabout::new(RoundaboutSize::Mini);
        assert_eq!(r.recommended_lane(1), 1);
    }

    #[test]
    fn test_recommended_lane_right() {
        let mut r = Roundabout::new(RoundaboutSize::Double);
        r.add_exit(RoundaboutExit::new(90.0, "First", 1));
        assert_eq!(r.recommended_lane(1), 2);
    }

    #[test]
    fn test_complexity() {
        let mut r = Roundabout::new(RoundaboutSize::Large);
        r.add_exit(RoundaboutExit::new(90.0, "A", 1));
        r.add_exit(RoundaboutExit::new(180.0, "B", 2));
        r.add_exit(RoundaboutExit::new(270.0, "C", 3));
        assert!(r.complexity() > 50.0);
    }

    #[test]
    fn test_num_exits() {
        let mut r = Roundabout::new(RoundaboutSize::Single);
        r.add_exit(RoundaboutExit::new(90.0, "A", 1));
        r.add_exit(RoundaboutExit::new(270.0, "B", 2));
        assert_eq!(r.num_exits(), 2);
    }
}
