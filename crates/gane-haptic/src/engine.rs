/// Haptic feedback engine: vibration patterns for navigation events.
#[derive(Debug, Clone, PartialEq)]
pub enum HapticPattern {
    TurnLeft,
    TurnRight,
    UTurn,
    Arrival,
    SpeedWarning,
    LaneDeparture,
    CollisionAlert,
    Confirmation,
    Heartbeat,
    Custom(Vec<HapticPulse>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct HapticPulse {
    pub intensity: f64,
    pub duration_ms: u32,
    pub pause_ms: u32,
}

impl HapticPulse {
    pub fn new(intensity: f64, duration_ms: u32, pause_ms: u32) -> Self {
        Self {
            intensity: intensity.clamp(0.0, 1.0),
            duration_ms,
            pause_ms,
        }
    }

    pub fn energy(&self) -> f64 {
        self.intensity * (self.duration_ms as f64 / 1000.0)
    }
}

#[derive(Debug, Clone)]
pub struct HapticSequence {
    pub pattern: HapticPattern,
    pub repeat_count: u32,
    pub urgency: f64,
}

impl HapticSequence {
    pub fn new(pattern: HapticPattern, repeat_count: u32, urgency: f64) -> Self {
        Self {
            pattern,
            repeat_count,
            urgency: urgency.clamp(0.0, 1.0),
        }
    }

    pub fn pulses(&self) -> Vec<HapticPulse> {
        let base = match &self.pattern {
            HapticPattern::TurnLeft => vec![
                HapticPulse::new(0.6, 200, 100),
                HapticPulse::new(0.8, 300, 0),
            ],
            HapticPattern::TurnRight => vec![
                HapticPulse::new(0.8, 300, 100),
                HapticPulse::new(0.6, 200, 0),
            ],
            HapticPattern::UTurn => vec![
                HapticPulse::new(0.7, 150, 50),
                HapticPulse::new(0.7, 150, 50),
                HapticPulse::new(0.9, 400, 0),
            ],
            HapticPattern::Arrival => vec![
                HapticPulse::new(0.5, 100, 80),
                HapticPulse::new(0.6, 100, 80),
                HapticPulse::new(0.7, 100, 80),
                HapticPulse::new(0.8, 200, 0),
            ],
            HapticPattern::SpeedWarning => vec![
                HapticPulse::new(1.0, 100, 50),
                HapticPulse::new(1.0, 100, 50),
                HapticPulse::new(1.0, 100, 0),
            ],
            HapticPattern::LaneDeparture => vec![
                HapticPulse::new(0.9, 80, 40),
                HapticPulse::new(0.9, 80, 40),
                HapticPulse::new(0.9, 80, 40),
                HapticPulse::new(0.9, 80, 0),
            ],
            HapticPattern::CollisionAlert => vec![
                HapticPulse::new(1.0, 50, 20),
                HapticPulse::new(1.0, 50, 20),
                HapticPulse::new(1.0, 50, 20),
                HapticPulse::new(1.0, 50, 20),
                HapticPulse::new(1.0, 200, 0),
            ],
            HapticPattern::Confirmation => vec![HapticPulse::new(0.4, 150, 0)],
            HapticPattern::Heartbeat => vec![
                HapticPulse::new(0.6, 80, 60),
                HapticPulse::new(0.3, 80, 400),
            ],
            HapticPattern::Custom(pulses) => pulses.clone(),
        };
        let mut result = Vec::new();
        for _ in 0..self.repeat_count.max(1) {
            result.extend(base.clone());
        }
        result
    }

    pub fn total_duration_ms(&self) -> u32 {
        self.pulses()
            .iter()
            .map(|p| p.duration_ms + p.pause_ms)
            .sum()
    }

    pub fn total_energy(&self) -> f64 {
        self.pulses().iter().map(|p| p.energy()).sum()
    }

    pub fn scale_by_urgency(&self) -> Vec<HapticPulse> {
        self.pulses()
            .into_iter()
            .map(|p| HapticPulse {
                intensity: (p.intensity * (0.5 + 0.5 * self.urgency)).clamp(0.0, 1.0),
                duration_ms: ((p.duration_ms as f64) * (0.7 + 0.3 * self.urgency)) as u32,
                pause_ms: ((p.pause_ms as f64) * (1.3 - 0.3 * self.urgency)) as u32,
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum HapticChannel {
    SteeringWheel,
    Seat,
    Pedal,
    Wristband,
    Phone,
}

#[derive(Debug, Clone)]
pub struct HapticDevice {
    pub channel: HapticChannel,
    pub max_intensity: f64,
    pub latency_ms: u32,
    pub enabled: bool,
}

impl Default for HapticDevice {
    fn default() -> Self {
        Self {
            channel: HapticChannel::SteeringWheel,
            max_intensity: 1.0,
            latency_ms: 5,
            enabled: true,
        }
    }
}

impl HapticDevice {
    pub fn new(channel: HapticChannel) -> Self {
        Self {
            channel,
            ..Self::default()
        }
    }

    pub fn apply_pulse(&self, pulse: &HapticPulse) -> Option<HapticPulse> {
        if !self.enabled {
            return None;
        }
        Some(HapticPulse {
            intensity: (pulse.intensity * self.max_intensity).clamp(0.0, 1.0),
            duration_ms: pulse.duration_ms,
            pause_ms: pulse.pause_ms + self.latency_ms,
        })
    }
}

#[derive(Debug, Clone)]
pub struct HapticRouter {
    pub devices: Vec<HapticDevice>,
    pub priority_threshold: f64,
}

impl Default for HapticRouter {
    fn default() -> Self {
        Self {
            devices: vec![HapticDevice::default()],
            priority_threshold: 0.3,
        }
    }
}

impl HapticRouter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_device(&mut self, device: HapticDevice) {
        self.devices.push(device);
    }

    pub fn active_devices(&self) -> Vec<&HapticDevice> {
        self.devices.iter().filter(|d| d.enabled).collect()
    }

    pub fn route_sequence(&self, seq: &HapticSequence) -> Vec<(HapticChannel, Vec<HapticPulse>)> {
        let scaled = seq.scale_by_urgency();
        self.devices
            .iter()
            .filter(|d| d.enabled)
            .filter(|d| {
                seq.urgency >= self.priority_threshold || d.channel == HapticChannel::SteeringWheel
            })
            .map(|d| {
                let pulses = scaled.iter().filter_map(|p| d.apply_pulse(p)).collect();
                (d.channel.clone(), pulses)
            })
            .collect()
    }

    pub fn should_alert(&self, seq: &HapticSequence) -> bool {
        seq.urgency >= self.priority_threshold && !self.active_devices().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pulse_energy() {
        let p = HapticPulse::new(0.5, 200, 100);
        assert!((p.energy() - 0.1).abs() < 0.001);
    }

    #[test]
    fn test_pulse_clamp() {
        let p = HapticPulse::new(1.5, 100, 0);
        assert!((p.intensity - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_sequence_turn_left() {
        let seq = HapticSequence::new(HapticPattern::TurnLeft, 1, 0.5);
        let pulses = seq.pulses();
        assert_eq!(pulses.len(), 2);
        assert!(seq.total_duration_ms() > 0);
    }

    #[test]
    fn test_sequence_repeat() {
        let seq = HapticSequence::new(HapticPattern::Confirmation, 3, 0.5);
        let pulses = seq.pulses();
        assert_eq!(pulses.len(), 3);
    }

    #[test]
    fn test_collision_alert_high_energy() {
        let seq = HapticSequence::new(HapticPattern::CollisionAlert, 1, 1.0);
        assert!(seq.total_energy() > 0.2);
    }

    #[test]
    fn test_scale_by_urgency_low() {
        let seq = HapticSequence::new(HapticPattern::TurnRight, 1, 0.0);
        let scaled = seq.scale_by_urgency();
        assert!(scaled[0].intensity <= 0.5);
    }

    #[test]
    fn test_scale_by_urgency_high() {
        let seq = HapticSequence::new(HapticPattern::TurnRight, 1, 1.0);
        let scaled = seq.scale_by_urgency();
        assert!(scaled[0].intensity >= 0.7);
    }

    #[test]
    fn test_device_default() {
        let d = HapticDevice::default();
        assert!(d.enabled);
        assert_eq!(d.channel, HapticChannel::SteeringWheel);
    }

    #[test]
    fn test_device_apply_pulse() {
        let d = HapticDevice::new(HapticChannel::Seat);
        let p = HapticPulse::new(0.8, 100, 50);
        let result = d.apply_pulse(&p).unwrap();
        assert!((result.intensity - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_device_disabled() {
        let mut d = HapticDevice::new(HapticChannel::Pedal);
        d.enabled = false;
        let p = HapticPulse::new(0.5, 100, 0);
        assert!(d.apply_pulse(&p).is_none());
    }

    #[test]
    fn test_router_default() {
        let r = HapticRouter::new();
        assert_eq!(r.active_devices().len(), 1);
    }

    #[test]
    fn test_router_add_device() {
        let mut r = HapticRouter::new();
        r.add_device(HapticDevice::new(HapticChannel::Seat));
        assert_eq!(r.active_devices().len(), 2);
    }

    #[test]
    fn test_router_route_sequence() {
        let r = HapticRouter::new();
        let seq = HapticSequence::new(HapticPattern::SpeedWarning, 1, 0.8);
        let routes = r.route_sequence(&seq);
        assert!(!routes.is_empty());
    }

    #[test]
    fn test_router_low_urgency_only_steering() {
        let mut r = HapticRouter::new();
        r.add_device(HapticDevice::new(HapticChannel::Seat));
        r.priority_threshold = 0.5;
        let seq = HapticSequence::new(HapticPattern::Confirmation, 1, 0.2);
        let routes = r.route_sequence(&seq);
        assert_eq!(routes.len(), 1);
        assert_eq!(routes[0].0, HapticChannel::SteeringWheel);
    }

    #[test]
    fn test_router_should_alert() {
        let r = HapticRouter::new();
        let high = HapticSequence::new(HapticPattern::CollisionAlert, 1, 0.9);
        let low = HapticSequence::new(HapticPattern::Confirmation, 1, 0.1);
        assert!(r.should_alert(&high));
        assert!(!r.should_alert(&low));
    }

    #[test]
    fn test_custom_pattern() {
        let custom = HapticPattern::Custom(vec![
            HapticPulse::new(0.3, 50, 20),
            HapticPulse::new(0.6, 100, 0),
        ]);
        let seq = HapticSequence::new(custom, 2, 0.5);
        assert_eq!(seq.pulses().len(), 4);
    }

    #[test]
    fn test_heartbeat_pattern() {
        let seq = HapticSequence::new(HapticPattern::Heartbeat, 1, 0.5);
        let pulses = seq.pulses();
        assert_eq!(pulses.len(), 2);
        assert!(pulses[0].intensity > pulses[1].intensity);
    }

    #[test]
    fn test_lane_departure_rapid() {
        let seq = HapticSequence::new(HapticPattern::LaneDeparture, 1, 1.0);
        let pulses = seq.pulses();
        assert_eq!(pulses.len(), 4);
        assert!(pulses.iter().all(|p| p.intensity >= 0.8));
    }

    #[test]
    fn test_arrival_crescendo() {
        let seq = HapticSequence::new(HapticPattern::Arrival, 1, 0.5);
        let pulses = seq.pulses();
        assert!(pulses.len() >= 3);
        assert!(pulses.last().unwrap().intensity > pulses.first().unwrap().intensity);
    }
}
