/// Perceptual stability: zero jitter, smooth camera, stable rendering.
#[derive(Debug, Clone)]
pub struct PositionSample { pub x: f64, pub y: f64, pub timestamp_ms: u64 }
#[derive(Debug, Clone)]
pub struct StabilityAnalyzer { pub samples: Vec<PositionSample>, pub max_jitter_threshold: f64 }
impl StabilityAnalyzer {
    pub fn new(threshold: f64) -> Self { Self { samples: Vec::new(), max_jitter_threshold: threshold } }
    pub fn add_sample(&mut self, s: PositionSample) { self.samples.push(s); }
    pub fn jitter(&self) -> f64 {
        if self.samples.len() < 3 { return 0.0; }
        let mut total = 0.0;
        for w in self.samples.windows(3) {
            let mid_x = (w[0].x + w[2].x) / 2.0;
            let mid_y = (w[0].y + w[2].y) / 2.0;
            total += ((w[1].x - mid_x).powi(2) + (w[1].y - mid_y).powi(2)).sqrt();
        }
        total / (self.samples.len() - 2) as f64
    }
    pub fn is_stable(&self) -> bool { self.jitter() <= self.max_jitter_threshold }
    pub fn smooth_position(&self) -> Option<(f64, f64)> {
        if self.samples.is_empty() { return None; }
        let n = self.samples.len().min(5);
        let recent = &self.samples[self.samples.len()-n..];
        let x = recent.iter().map(|s| s.x).sum::<f64>() / n as f64;
        let y = recent.iter().map(|s| s.y).sum::<f64>() / n as f64;
        Some((x, y))
    }
    pub fn stability_score(&self) -> f64 { (1.0 - (self.jitter() / self.max_jitter_threshold).min(1.0)).clamp(0.0, 1.0) }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_stable() { let mut a = StabilityAnalyzer::new(1.0); for i in 0..5 { a.add_sample(PositionSample { x: i as f64, y: 0.0, timestamp_ms: i as u64 * 100 }); } assert!(a.is_stable()); }
    #[test] fn test_jittery() { let mut a = StabilityAnalyzer::new(0.1); a.add_sample(PositionSample { x: 0.0, y: 0.0, timestamp_ms: 0 }); a.add_sample(PositionSample { x: 5.0, y: 5.0, timestamp_ms: 100 }); a.add_sample(PositionSample { x: 0.0, y: 0.0, timestamp_ms: 200 }); assert!(!a.is_stable()); }
    #[test] fn test_smooth() { let mut a = StabilityAnalyzer::new(1.0); a.add_sample(PositionSample { x: 1.0, y: 2.0, timestamp_ms: 0 }); a.add_sample(PositionSample { x: 3.0, y: 4.0, timestamp_ms: 100 }); let (x, y) = a.smooth_position().unwrap(); assert!((x-2.0).abs()<0.01); assert!((y-3.0).abs()<0.01); }
    #[test] fn test_empty() { let a = StabilityAnalyzer::new(1.0); assert!(a.smooth_position().is_none()); assert_eq!(a.jitter(), 0.0); }
}
