/// Highway merge assistance: acceleration lane timing, gap acceptance, ramp metering.
#[derive(Debug, Clone, PartialEq)]
pub enum MergeType {
    OnRamp,
    OffRamp,
    LaneReduction,
    Construction,
    WeaveZone,
    Collector,
}

impl MergeType {
    pub fn typical_length_m(&self) -> f64 {
        match self {
            MergeType::OnRamp => 300.0,
            MergeType::OffRamp => 250.0,
            MergeType::LaneReduction => 200.0,
            MergeType::Construction => 150.0,
            MergeType::WeaveZone => 400.0,
            MergeType::Collector => 350.0,
        }
    }

    pub fn difficulty(&self) -> f64 {
        match self {
            MergeType::WeaveZone => 0.9,
            MergeType::Construction => 0.8,
            MergeType::OnRamp => 0.6,
            MergeType::LaneReduction => 0.5,
            MergeType::Collector => 0.4,
            MergeType::OffRamp => 0.3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MergeZone {
    pub merge_type: MergeType,
    pub acceleration_lane_m: f64,
    pub mainline_speed_kmh: f64,
    pub ramp_speed_kmh: f64,
    pub mainline_volume: f64,
    pub has_ramp_meter: bool,
}

impl MergeZone {
    pub fn new(merge_type: MergeType, mainline_speed: f64) -> Self {
        Self {
            acceleration_lane_m: merge_type.typical_length_m(),
            merge_type,
            mainline_speed_kmh: mainline_speed,
            ramp_speed_kmh: mainline_speed * 0.6,
            mainline_volume: 0.5,
            has_ramp_meter: false,
        }
    }

    pub fn speed_differential(&self) -> f64 {
        (self.mainline_speed_kmh - self.ramp_speed_kmh).abs()
    }

    pub fn required_acceleration(&self) -> f64 {
        let diff_ms = self.speed_differential() / 3.6;
        let dist = self.acceleration_lane_m;
        if dist <= 0.0 {
            return f64::INFINITY;
        }
        (diff_ms * diff_ms) / (2.0 * dist)
    }

    pub fn merge_difficulty(&self) -> f64 {
        let type_diff = self.merge_type.difficulty() * 40.0;
        let volume_diff = self.mainline_volume * 30.0;
        let speed_diff = (self.speed_differential() / 100.0) * 30.0;
        (type_diff + volume_diff + speed_diff).clamp(0.0, 100.0)
    }

    pub fn recommended_entry_speed(&self) -> f64 {
        self.mainline_speed_kmh * 0.85
    }

    pub fn gap_acceptance_sec(&self) -> f64 {
        let base = 4.0;
        let volume_adj = self.mainline_volume * 3.0;
        base + volume_adj
    }

    pub fn is_safe_merge(&self, current_speed: f64, gap_sec: f64) -> bool {
        let speed_ok = current_speed >= self.recommended_entry_speed() * 0.8;
        let gap_ok = gap_sec >= self.gap_acceptance_sec();
        speed_ok && gap_ok
    }

    pub fn estimated_delay_sec(&self) -> f64 {
        let meter_delay = if self.has_ramp_meter { 15.0 } else { 0.0 };
        let volume_delay = self.mainline_volume * 20.0;
        meter_delay + volume_delay
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_length() {
        assert!(
            MergeType::WeaveZone.typical_length_m() > MergeType::Construction.typical_length_m()
        );
    }

    #[test]
    fn test_difficulty() {
        assert!(MergeType::WeaveZone.difficulty() > MergeType::OffRamp.difficulty());
    }

    #[test]
    fn test_speed_differential() {
        let mz = MergeZone::new(MergeType::OnRamp, 100.0);
        assert!((mz.speed_differential() - 40.0).abs() < 0.01);
    }

    #[test]
    fn test_required_accel() {
        let mz = MergeZone::new(MergeType::OnRamp, 100.0);
        assert!(mz.required_acceleration() > 0.0);
    }

    #[test]
    fn test_merge_difficulty_range() {
        let mz = MergeZone::new(MergeType::OnRamp, 100.0);
        let d = mz.merge_difficulty();
        assert!(d >= 0.0 && d <= 100.0);
    }

    #[test]
    fn test_recommended_speed() {
        let mz = MergeZone::new(MergeType::OnRamp, 100.0);
        assert!((mz.recommended_entry_speed() - 85.0).abs() < 0.01);
    }

    #[test]
    fn test_gap_acceptance() {
        let mz = MergeZone::new(MergeType::OnRamp, 100.0);
        assert!(mz.gap_acceptance_sec() > 3.0);
    }

    #[test]
    fn test_safe_merge() {
        let mz = MergeZone::new(MergeType::OnRamp, 100.0);
        assert!(mz.is_safe_merge(85.0, 10.0));
        assert!(!mz.is_safe_merge(30.0, 1.0));
    }

    #[test]
    fn test_ramp_meter_delay() {
        let mut mz = MergeZone::new(MergeType::OnRamp, 100.0);
        let d1 = mz.estimated_delay_sec();
        mz.has_ramp_meter = true;
        assert!(mz.estimated_delay_sec() > d1);
    }

    #[test]
    fn test_high_volume_difficulty() {
        let mut mz = MergeZone::new(MergeType::OnRamp, 100.0);
        let d1 = mz.merge_difficulty();
        mz.mainline_volume = 0.9;
        assert!(mz.merge_difficulty() > d1);
    }
}
