/// Live data network effect: fleet data, city sensors, data flywheel.
#[derive(Debug, Clone, PartialEq)]
pub enum DataSourceType {
    FleetVehicle,
    CitySensor,
    CrowdUser,
    OemVehicle,
    PublicTransit,
}
#[derive(Debug, Clone)]
pub struct LiveDataSource {
    pub source_type: DataSourceType,
    pub name: String,
    pub data_points_per_hour: u64,
    pub reliability: f64,
    pub exclusive: bool,
}
#[derive(Debug, Clone)]
pub struct DataNetwork {
    pub sources: Vec<LiveDataSource>,
}
impl Default for DataNetwork {
    fn default() -> Self {
        Self::new()
    }
}
impl DataNetwork {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
        }
    }
    pub fn add_source(&mut self, s: LiveDataSource) {
        self.sources.push(s);
    }
    pub fn total_throughput(&self) -> u64 {
        self.sources.iter().map(|s| s.data_points_per_hour).sum()
    }
    pub fn avg_reliability(&self) -> f64 {
        if self.sources.is_empty() {
            0.0
        } else {
            self.sources.iter().map(|s| s.reliability).sum::<f64>() / self.sources.len() as f64
        }
    }
    pub fn exclusive_sources(&self) -> usize {
        self.sources.iter().filter(|s| s.exclusive).count()
    }
    pub fn network_strength(&self) -> f64 {
        let throughput_score =
            ((self.total_throughput() as f64).ln().max(0.0) / 20.0).min(1.0) * 0.3;
        let reliability_score = self.avg_reliability() * 0.3;
        let exclusive_score = (self.exclusive_sources() as f64 / 10.0).min(1.0) * 0.2;
        let diversity_score = {
            let types: std::collections::HashSet<_> = self
                .sources
                .iter()
                .map(|s| std::mem::discriminant(&s.source_type))
                .collect();
            (types.len() as f64 / 5.0).min(1.0) * 0.2
        };
        (throughput_score + reliability_score + exclusive_score + diversity_score).clamp(0.0, 1.0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty() {
        let n = DataNetwork::new();
        assert_eq!(n.total_throughput(), 0);
        assert_eq!(n.avg_reliability(), 0.0);
    }
    #[test]
    fn test_add() {
        let mut n = DataNetwork::new();
        n.add_source(LiveDataSource {
            source_type: DataSourceType::FleetVehicle,
            name: "Fleet1".into(),
            data_points_per_hour: 10000,
            reliability: 0.95,
            exclusive: true,
        });
        assert_eq!(n.total_throughput(), 10000);
        assert_eq!(n.exclusive_sources(), 1);
    }
    #[test]
    fn test_strength() {
        let mut n = DataNetwork::new();
        n.add_source(LiveDataSource {
            source_type: DataSourceType::FleetVehicle,
            name: "F".into(),
            data_points_per_hour: 100000,
            reliability: 0.9,
            exclusive: true,
        });
        n.add_source(LiveDataSource {
            source_type: DataSourceType::CitySensor,
            name: "C".into(),
            data_points_per_hour: 50000,
            reliability: 0.95,
            exclusive: false,
        });
        assert!(n.network_strength() > 0.0);
    }
}
