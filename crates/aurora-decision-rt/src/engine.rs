/// Real-time decision engine: network optimization, load balancing.
#[derive(Debug, Clone)]
pub struct RoadSegment { pub id: u64, pub capacity: f64, pub current_load: f64, pub travel_time_s: f64 }
impl RoadSegment {
    pub fn utilization(&self) -> f64 { if self.capacity <= 0.0 { 1.0 } else { (self.current_load / self.capacity).clamp(0.0, 1.0) } }
    pub fn is_congested(&self) -> bool { self.utilization() > 0.8 }
    pub fn adjusted_travel_time(&self) -> f64 { self.travel_time_s * (1.0 + self.utilization().powi(3) * 4.0) }
}
#[derive(Debug, Clone)]
pub struct NetworkOptimizer { pub segments: Vec<RoadSegment> }
impl Default for NetworkOptimizer {
    fn default() -> Self { Self::new() }
}
impl NetworkOptimizer {
    pub fn new() -> Self { Self { segments: Vec::new() } }
    pub fn add_segment(&mut self, s: RoadSegment) { self.segments.push(s); }
    pub fn avg_utilization(&self) -> f64 { if self.segments.is_empty() { 0.0 } else { self.segments.iter().map(|s| s.utilization()).sum::<f64>() / self.segments.len() as f64 } }
    pub fn congested_segments(&self) -> usize { self.segments.iter().filter(|s| s.is_congested()).count() }
    pub fn balance_score(&self) -> f64 {
        if self.segments.len() < 2 { return 1.0; }
        let avg = self.avg_utilization();
        let variance = self.segments.iter().map(|s| (s.utilization() - avg).powi(2)).sum::<f64>() / self.segments.len() as f64;
        (1.0 - variance.sqrt() * 2.0).clamp(0.0, 1.0)
    }
    pub fn recommend_redistribution(&self) -> Vec<(u64, f64)> {
        let avg = self.avg_utilization();
        self.segments.iter().filter(|s| s.utilization() > avg + 0.2).map(|s| (s.id, s.utilization() - avg)).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_util() { let s = RoadSegment { id: 1, capacity: 100.0, current_load: 80.0, travel_time_s: 60.0 }; assert!((s.utilization()-0.8).abs()<0.01); }
    #[test] fn test_congested() { let s = RoadSegment { id: 1, capacity: 100.0, current_load: 90.0, travel_time_s: 60.0 }; assert!(s.is_congested()); }
    #[test] fn test_balance() { let mut o = NetworkOptimizer::new(); o.add_segment(RoadSegment { id: 1, capacity: 100.0, current_load: 50.0, travel_time_s: 60.0 }); o.add_segment(RoadSegment { id: 2, capacity: 100.0, current_load: 50.0, travel_time_s: 60.0 }); assert!(o.balance_score() > 0.9); }
    #[test] fn test_empty() { let o = NetworkOptimizer::new(); assert_eq!(o.avg_utilization(), 0.0); }
}
