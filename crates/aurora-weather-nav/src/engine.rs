/// Weather-aware navigation engine: visibility, precipitation, wind impact on routing.
#[derive(Debug, Clone, PartialEq)]
pub enum WeatherCondition {
    Clear,
    Cloudy,
    Rain,
    HeavyRain,
    Snow,
    HeavySnow,
    Fog,
    DenseFog,
    Hail,
    Thunderstorm,
    Sandstorm,
    IceStorm,
}

impl WeatherCondition {
    pub fn visibility_km(&self) -> f64 {
        match self {
            WeatherCondition::Clear => 20.0,
            WeatherCondition::Cloudy => 15.0,
            WeatherCondition::Rain => 5.0,
            WeatherCondition::HeavyRain => 2.0,
            WeatherCondition::Snow => 3.0,
            WeatherCondition::HeavySnow => 1.0,
            WeatherCondition::Fog => 0.5,
            WeatherCondition::DenseFog => 0.1,
            WeatherCondition::Hail => 2.0,
            WeatherCondition::Thunderstorm => 1.5,
            WeatherCondition::Sandstorm => 0.3,
            WeatherCondition::IceStorm => 0.8,
        }
    }

    pub fn speed_limit_factor(&self) -> f64 {
        match self {
            WeatherCondition::Clear | WeatherCondition::Cloudy => 1.0,
            WeatherCondition::Rain => 0.8,
            WeatherCondition::HeavyRain => 0.6,
            WeatherCondition::Snow => 0.6,
            WeatherCondition::HeavySnow => 0.4,
            WeatherCondition::Fog => 0.5,
            WeatherCondition::DenseFog => 0.3,
            WeatherCondition::Hail => 0.5,
            WeatherCondition::Thunderstorm => 0.4,
            WeatherCondition::Sandstorm => 0.3,
            WeatherCondition::IceStorm => 0.2,
        }
    }

    pub fn traction_penalty(&self) -> f64 {
        match self {
            WeatherCondition::Clear | WeatherCondition::Cloudy => 0.0,
            WeatherCondition::Rain => 0.15,
            WeatherCondition::HeavyRain => 0.30,
            WeatherCondition::Snow => 0.35,
            WeatherCondition::HeavySnow => 0.50,
            WeatherCondition::Fog => 0.05,
            WeatherCondition::DenseFog => 0.05,
            WeatherCondition::Hail => 0.25,
            WeatherCondition::Thunderstorm => 0.30,
            WeatherCondition::Sandstorm => 0.20,
            WeatherCondition::IceStorm => 0.60,
        }
    }

    pub fn is_severe(&self) -> bool {
        matches!(
            self,
            WeatherCondition::HeavySnow
                | WeatherCondition::DenseFog
                | WeatherCondition::Thunderstorm
                | WeatherCondition::Sandstorm
                | WeatherCondition::IceStorm
        )
    }

    pub fn risk_score(&self) -> f64 {
        let visibility_risk = (1.0 - self.visibility_km() / 20.0).clamp(0.0, 1.0);
        let traction_risk = self.traction_penalty();
        let speed_risk = 1.0 - self.speed_limit_factor();
        (visibility_risk * 0.3 + traction_risk * 0.4 + speed_risk * 0.3).clamp(0.0, 1.0)
    }
}

#[derive(Debug, Clone)]
pub struct WeatherSegment {
    pub condition: WeatherCondition,
    pub start_km: f64,
    pub end_km: f64,
    pub wind_speed_kmh: f64,
    pub wind_heading_deg: f64,
    pub temperature_c: f64,
}

impl WeatherSegment {
    pub fn new(condition: WeatherCondition, start_km: f64, end_km: f64) -> Self {
        Self {
            condition,
            start_km,
            end_km,
            wind_speed_kmh: 0.0,
            wind_heading_deg: 0.0,
            temperature_c: 20.0,
        }
    }

    pub fn length_km(&self) -> f64 {
        (self.end_km - self.start_km).max(0.0)
    }

    pub fn effective_speed_limit(&self, base_speed: f64) -> f64 {
        let weather_factor = self.condition.speed_limit_factor();
        let wind_factor = if self.wind_speed_kmh > 80.0 {
            0.6
        } else if self.wind_speed_kmh > 50.0 {
            0.8
        } else {
            1.0
        };
        (base_speed * weather_factor * wind_factor).max(10.0)
    }

    pub fn estimated_delay_min(&self, base_speed: f64) -> f64 {
        let normal_time = self.length_km() / base_speed * 60.0;
        let actual_time = self.length_km() / self.effective_speed_limit(base_speed) * 60.0;
        (actual_time - normal_time).max(0.0)
    }

    pub fn black_ice_risk(&self) -> f64 {
        if self.temperature_c > 4.0 || self.temperature_c < -10.0 {
            return 0.0;
        }
        let temp_risk = 1.0 - ((self.temperature_c - (-3.0)).abs() / 7.0).min(1.0);
        let moisture = match self.condition {
            WeatherCondition::Rain | WeatherCondition::HeavyRain => 0.9,
            WeatherCondition::Snow | WeatherCondition::HeavySnow => 0.7,
            WeatherCondition::IceStorm => 1.0,
            WeatherCondition::Fog | WeatherCondition::DenseFog => 0.5,
            _ => 0.1,
        };
        (temp_risk * moisture).clamp(0.0, 1.0)
    }
}

#[derive(Debug, Clone)]
pub struct WeatherRoute {
    pub segments: Vec<WeatherSegment>,
}

impl Default for WeatherRoute {
    fn default() -> Self {
        Self::new()
    }
}

impl WeatherRoute {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    pub fn add_segment(&mut self, seg: WeatherSegment) {
        self.segments.push(seg);
    }

    pub fn total_length_km(&self) -> f64 {
        self.segments.iter().map(|s| s.length_km()).sum()
    }

    pub fn total_delay_min(&self, base_speed: f64) -> f64 {
        self.segments
            .iter()
            .map(|s| s.estimated_delay_min(base_speed))
            .sum()
    }

    pub fn worst_visibility_km(&self) -> f64 {
        self.segments
            .iter()
            .map(|s| s.condition.visibility_km())
            .fold(f64::INFINITY, f64::min)
    }

    pub fn has_severe_weather(&self) -> bool {
        self.segments.iter().any(|s| s.condition.is_severe())
    }

    pub fn max_risk_score(&self) -> f64 {
        self.segments
            .iter()
            .map(|s| s.condition.risk_score())
            .fold(0.0_f64, f64::max)
    }

    pub fn route_advisable(&self) -> bool {
        !self.segments.iter().any(|s| {
            s.condition.risk_score() > 0.7 || s.wind_speed_kmh > 100.0 || s.black_ice_risk() > 0.8
        })
    }

    pub fn severe_segment_count(&self) -> usize {
        self.segments
            .iter()
            .filter(|s| s.condition.is_severe())
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_visibility() {
        assert!((WeatherCondition::Clear.visibility_km() - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_dense_fog_visibility() {
        assert!((WeatherCondition::DenseFog.visibility_km() - 0.1).abs() < 0.01);
    }

    #[test]
    fn test_clear_no_speed_reduction() {
        assert!((WeatherCondition::Clear.speed_limit_factor() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_ice_storm_severe() {
        assert!(WeatherCondition::IceStorm.is_severe());
    }

    #[test]
    fn test_rain_not_severe() {
        assert!(!WeatherCondition::Rain.is_severe());
    }

    #[test]
    fn test_risk_score_range() {
        let score = WeatherCondition::IceStorm.risk_score();
        assert!(score > 0.0 && score <= 1.0);
    }

    #[test]
    fn test_clear_low_risk() {
        assert!(WeatherCondition::Clear.risk_score() < 0.1);
    }

    #[test]
    fn test_segment_length() {
        let s = WeatherSegment::new(WeatherCondition::Rain, 10.0, 25.0);
        assert!((s.length_km() - 15.0).abs() < 0.01);
    }

    #[test]
    fn test_effective_speed_rain() {
        let s = WeatherSegment::new(WeatherCondition::Rain, 0.0, 10.0);
        let limit = s.effective_speed_limit(100.0);
        assert!((limit - 80.0).abs() < 0.01);
    }

    #[test]
    fn test_high_wind_reduces_speed() {
        let mut s = WeatherSegment::new(WeatherCondition::Clear, 0.0, 10.0);
        s.wind_speed_kmh = 90.0;
        assert!(s.effective_speed_limit(100.0) < 100.0);
    }

    #[test]
    fn test_delay_positive_in_bad_weather() {
        let s = WeatherSegment::new(WeatherCondition::HeavySnow, 0.0, 50.0);
        assert!(s.estimated_delay_min(100.0) > 0.0);
    }

    #[test]
    fn test_no_delay_clear() {
        let s = WeatherSegment::new(WeatherCondition::Clear, 0.0, 50.0);
        assert!(s.estimated_delay_min(100.0).abs() < 0.01);
    }

    #[test]
    fn test_black_ice_risk_freezing() {
        let mut s = WeatherSegment::new(WeatherCondition::Rain, 0.0, 10.0);
        s.temperature_c = -1.0;
        assert!(s.black_ice_risk() > 0.3);
    }

    #[test]
    fn test_black_ice_risk_warm() {
        let mut s = WeatherSegment::new(WeatherCondition::Rain, 0.0, 10.0);
        s.temperature_c = 25.0;
        assert!(s.black_ice_risk() < 0.01);
    }

    #[test]
    fn test_route_total_length() {
        let mut r = WeatherRoute::new();
        r.add_segment(WeatherSegment::new(WeatherCondition::Clear, 0.0, 30.0));
        r.add_segment(WeatherSegment::new(WeatherCondition::Rain, 30.0, 50.0));
        assert!((r.total_length_km() - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_has_severe_weather() {
        let mut r = WeatherRoute::new();
        r.add_segment(WeatherSegment::new(WeatherCondition::Clear, 0.0, 30.0));
        r.add_segment(WeatherSegment::new(
            WeatherCondition::Thunderstorm,
            30.0,
            50.0,
        ));
        assert!(r.has_severe_weather());
    }

    #[test]
    fn test_no_severe_weather() {
        let mut r = WeatherRoute::new();
        r.add_segment(WeatherSegment::new(WeatherCondition::Cloudy, 0.0, 50.0));
        assert!(!r.has_severe_weather());
    }

    #[test]
    fn test_route_advisable_clear() {
        let mut r = WeatherRoute::new();
        r.add_segment(WeatherSegment::new(WeatherCondition::Clear, 0.0, 50.0));
        assert!(r.route_advisable());
    }

    #[test]
    fn test_empty_route() {
        let r = WeatherRoute::new();
        assert_eq!(r.total_length_km(), 0.0);
        assert!(!r.has_severe_weather());
        assert_eq!(r.severe_segment_count(), 0);
    }

    #[test]
    fn test_worst_visibility() {
        let mut r = WeatherRoute::new();
        r.add_segment(WeatherSegment::new(WeatherCondition::Clear, 0.0, 30.0));
        r.add_segment(WeatherSegment::new(WeatherCondition::DenseFog, 30.0, 40.0));
        assert!((r.worst_visibility_km() - 0.1).abs() < 0.01);
    }
}
