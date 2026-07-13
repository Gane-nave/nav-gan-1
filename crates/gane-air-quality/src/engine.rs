/// Air quality monitoring: AQI levels, pollution avoidance, ventilation control.
#[derive(Debug, Clone, PartialEq)]
pub enum Pollutant {
    PM25,
    PM10,
    Ozone,
    NitrogenDioxide,
    SulfurDioxide,
    CarbonMonoxide,
    VOC,
}

impl Pollutant {
    pub fn unit(&self) -> &str {
        match self {
            Pollutant::PM25 | Pollutant::PM10 => "µg/m³",
            Pollutant::CarbonMonoxide => "mg/m³",
            _ => "ppb",
        }
    }

    pub fn health_threshold(&self) -> f64 {
        match self {
            Pollutant::PM25 => 35.0,
            Pollutant::PM10 => 150.0,
            Pollutant::Ozone => 70.0,
            Pollutant::NitrogenDioxide => 100.0,
            Pollutant::SulfurDioxide => 75.0,
            Pollutant::CarbonMonoxide => 10.0,
            Pollutant::VOC => 500.0,
        }
    }

    pub fn respiratory_impact(&self) -> f64 {
        match self {
            Pollutant::PM25 => 0.95,
            Pollutant::Ozone => 0.8,
            Pollutant::NitrogenDioxide => 0.7,
            Pollutant::SulfurDioxide => 0.75,
            Pollutant::PM10 => 0.6,
            Pollutant::CarbonMonoxide => 0.5,
            Pollutant::VOC => 0.4,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AqiCategory {
    Good,
    Moderate,
    UnhealthySensitive,
    Unhealthy,
    VeryUnhealthy,
    Hazardous,
}

impl AqiCategory {
    pub fn from_aqi(aqi: f64) -> Self {
        match aqi {
            a if a <= 50.0 => AqiCategory::Good,
            a if a <= 100.0 => AqiCategory::Moderate,
            a if a <= 150.0 => AqiCategory::UnhealthySensitive,
            a if a <= 200.0 => AqiCategory::Unhealthy,
            a if a <= 300.0 => AqiCategory::VeryUnhealthy,
            _ => AqiCategory::Hazardous,
        }
    }

    pub fn outdoor_safe(&self) -> bool {
        matches!(self, AqiCategory::Good | AqiCategory::Moderate)
    }

    pub fn cabin_filter_recommended(&self) -> bool {
        !matches!(self, AqiCategory::Good)
    }

    pub fn window_advisory(&self) -> &str {
        match self {
            AqiCategory::Good => "Windows open OK",
            AqiCategory::Moderate => "Windows open with caution",
            _ => "Keep windows closed",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AirQualityReading {
    pub aqi: f64,
    pub dominant_pollutant: Pollutant,
    pub pm25: f64,
    pub pm10: f64,
}

impl AirQualityReading {
    pub fn new(aqi: f64, dominant: Pollutant) -> Self {
        Self {
            aqi,
            dominant_pollutant: dominant,
            pm25: 0.0,
            pm10: 0.0,
        }
    }

    pub fn category(&self) -> AqiCategory {
        AqiCategory::from_aqi(self.aqi)
    }

    pub fn is_healthy(&self) -> bool {
        self.aqi <= 100.0
    }

    pub fn recirculation_recommended(&self) -> bool {
        self.aqi > 100.0
    }

    pub fn exercise_safe(&self) -> bool {
        self.aqi <= 50.0
    }

    pub fn health_risk(&self) -> f64 {
        (self.aqi / 500.0).min(1.0)
    }
}

#[derive(Debug, Clone)]
pub struct AirQualityRoute {
    pub readings: Vec<AirQualityReading>,
}

impl Default for AirQualityRoute {
    fn default() -> Self {
        Self::new()
    }
}

impl AirQualityRoute {
    pub fn new() -> Self {
        Self {
            readings: Vec::new(),
        }
    }

    pub fn add(&mut self, r: AirQualityReading) {
        self.readings.push(r);
    }

    pub fn average_aqi(&self) -> f64 {
        if self.readings.is_empty() {
            return 0.0;
        }
        let total: f64 = self.readings.iter().map(|r| r.aqi).sum();
        total / self.readings.len() as f64
    }

    pub fn max_aqi(&self) -> f64 {
        self.readings.iter().map(|r| r.aqi).fold(0.0_f64, f64::max)
    }

    pub fn healthy_pct(&self) -> f64 {
        if self.readings.is_empty() {
            return 100.0;
        }
        let healthy = self.readings.iter().filter(|r| r.is_healthy()).count();
        (healthy as f64 / self.readings.len() as f64) * 100.0
    }

    pub fn needs_filter(&self) -> bool {
        self.readings.iter().any(|r| r.recirculation_recommended())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pollutant_unit() {
        assert_eq!(Pollutant::PM25.unit(), "µg/m³");
        assert_eq!(Pollutant::Ozone.unit(), "ppb");
    }

    #[test]
    fn test_respiratory_impact() {
        assert!(Pollutant::PM25.respiratory_impact() > Pollutant::VOC.respiratory_impact());
    }

    #[test]
    fn test_aqi_good() {
        assert_eq!(AqiCategory::from_aqi(30.0), AqiCategory::Good);
    }

    #[test]
    fn test_aqi_hazardous() {
        assert_eq!(AqiCategory::from_aqi(350.0), AqiCategory::Hazardous);
    }

    #[test]
    fn test_outdoor_safe() {
        assert!(AqiCategory::Good.outdoor_safe());
        assert!(!AqiCategory::Unhealthy.outdoor_safe());
    }

    #[test]
    fn test_cabin_filter() {
        assert!(!AqiCategory::Good.cabin_filter_recommended());
        assert!(AqiCategory::Moderate.cabin_filter_recommended());
    }

    #[test]
    fn test_window_advisory() {
        assert_eq!(
            AqiCategory::Hazardous.window_advisory(),
            "Keep windows closed"
        );
    }

    #[test]
    fn test_reading_healthy() {
        let r = AirQualityReading::new(45.0, Pollutant::PM25);
        assert!(r.is_healthy());
    }

    #[test]
    fn test_reading_unhealthy() {
        let r = AirQualityReading::new(150.0, Pollutant::PM25);
        assert!(!r.is_healthy());
    }

    #[test]
    fn test_recirculation() {
        let r = AirQualityReading::new(120.0, Pollutant::PM25);
        assert!(r.recirculation_recommended());
    }

    #[test]
    fn test_exercise_safe() {
        let r = AirQualityReading::new(40.0, Pollutant::Ozone);
        assert!(r.exercise_safe());
    }

    #[test]
    fn test_route_average() {
        let mut route = AirQualityRoute::new();
        route.add(AirQualityReading::new(50.0, Pollutant::PM25));
        route.add(AirQualityReading::new(100.0, Pollutant::PM10));
        assert!((route.average_aqi() - 75.0).abs() < 0.01);
    }

    #[test]
    fn test_route_max() {
        let mut route = AirQualityRoute::new();
        route.add(AirQualityReading::new(50.0, Pollutant::PM25));
        route.add(AirQualityReading::new(150.0, Pollutant::PM10));
        assert!((route.max_aqi() - 150.0).abs() < 0.01);
    }

    #[test]
    fn test_healthy_pct() {
        let mut route = AirQualityRoute::new();
        route.add(AirQualityReading::new(50.0, Pollutant::PM25));
        route.add(AirQualityReading::new(150.0, Pollutant::PM10));
        assert!((route.healthy_pct() - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_needs_filter() {
        let mut route = AirQualityRoute::new();
        route.add(AirQualityReading::new(120.0, Pollutant::PM25));
        assert!(route.needs_filter());
    }

    #[test]
    fn test_empty_route() {
        let route = AirQualityRoute::new();
        assert_eq!(route.average_aqi(), 0.0);
        assert!((route.healthy_pct() - 100.0).abs() < 0.01);
    }
}
