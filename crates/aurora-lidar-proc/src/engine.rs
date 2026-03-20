/// LiDAR processing: point cloud, object detection, 3D mapping
/// Phase 182

#[derive(Debug, Clone)]
pub struct LidarPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub intensity: f64,
}

impl LidarPoint {
    pub fn distance(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

#[derive(Debug, Clone)]
pub struct LidarSystem {
    pub active: bool,
    pub range_m: f64,
    pub points_per_sec: u64,
    pub channels: u32,
    pub rotation_hz: f64,
    pub detected_objects: u32,
}

impl Default for LidarSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl LidarSystem {
    pub fn new() -> Self {
        Self {
            active: true,
            range_m: 200.0,
            points_per_sec: 300_000,
            channels: 64,
            rotation_hz: 20.0,
            detected_objects: 0,
        }
    }

    pub fn resolution_score(&self) -> f64 {
        let ch_s = (self.channels as f64 / 128.0).min(1.0) * 50.0;
        let rate_s = (self.points_per_sec as f64 / 1_000_000.0).min(1.0) * 50.0;
        ch_s + rate_s
    }

    pub fn effective_range_m(&self) -> f64 {
        if self.active {
            self.range_m
        } else {
            0.0
        }
    }

    pub fn has_detections(&self) -> bool {
        self.detected_objects > 0
    }

    pub fn points_per_rotation(&self) -> u64 {
        if self.rotation_hz > 0.0 {
            (self.points_per_sec as f64 / self.rotation_hz) as u64
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_distance() {
        let p = LidarPoint {
            x: 3.0,
            y: 4.0,
            z: 0.0,
            intensity: 100.0,
        };
        assert!((p.distance() - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_active() {
        let s = LidarSystem::new();
        assert!(s.active);
    }

    #[test]
    fn test_range() {
        let s = LidarSystem::new();
        assert!((s.effective_range_m() - 200.0).abs() < 0.1);
    }

    #[test]
    fn test_inactive_range() {
        let mut s = LidarSystem::new();
        s.active = false;
        assert!((s.effective_range_m() - 0.0).abs() < 0.1);
    }

    #[test]
    fn test_resolution() {
        let s = LidarSystem::new();
        assert!(s.resolution_score() > 30.0);
    }

    #[test]
    fn test_no_detections() {
        let s = LidarSystem::new();
        assert!(!s.has_detections());
    }

    #[test]
    fn test_points_per_rotation() {
        let s = LidarSystem::new();
        assert!(s.points_per_rotation() > 10000);
    }
}
