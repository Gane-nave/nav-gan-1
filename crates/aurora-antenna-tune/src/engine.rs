/// Antenna tuning: signal optimization, multi-band reception, interference mitigation
/// Phase 145

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Band {
    AM,
    FM,
    DAB,
    Satellite,
    LTE,
    V2X,
    GPS,
}

impl Band {
    pub fn frequency_range_mhz(&self) -> (f64, f64) {
        match self {
            Band::AM => (0.535, 1.705),
            Band::FM => (87.5, 108.0),
            Band::DAB => (174.0, 240.0),
            Band::Satellite => (2320.0, 2345.0),
            Band::LTE => (700.0, 2600.0),
            Band::V2X => (5855.0, 5925.0),
            Band::GPS => (1575.0, 1575.5),
        }
    }

    pub fn is_navigation(&self) -> bool {
        matches!(self, Band::GPS | Band::V2X)
    }

    pub fn is_entertainment(&self) -> bool {
        matches!(self, Band::AM | Band::FM | Band::DAB | Band::Satellite)
    }
}

#[derive(Debug, Clone)]
pub struct AntennaSignal {
    pub band: Band,
    pub strength_dbm: f64,
    pub snr_db: f64,
    pub interference_level: f64,
}

impl AntennaSignal {
    pub fn new(band: Band, strength: f64, snr: f64) -> Self {
        Self {
            band,
            strength_dbm: strength,
            snr_db: snr,
            interference_level: 0.0,
        }
    }

    pub fn quality_pct(&self) -> f64 {
        let q: f64 = ((self.strength_dbm + 120.0) / 80.0 * 100.0).min(100.0);
        q.max(0.0)
    }

    pub fn is_usable(&self) -> bool {
        self.snr_db > 10.0 && self.strength_dbm > -100.0
    }

    pub fn has_interference(&self) -> bool {
        self.interference_level > 30.0
    }

    pub fn effective_snr(&self) -> f64 {
        (self.snr_db - self.interference_level * 0.5).max(0.0)
    }
}

#[derive(Debug, Clone)]
pub struct AntennaSystem {
    pub signals: Vec<AntennaSignal>,
    pub diversity_enabled: bool,
}

impl Default for AntennaSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl AntennaSystem {
    pub fn new() -> Self {
        Self {
            signals: Vec::new(),
            diversity_enabled: true,
        }
    }

    pub fn add_signal(&mut self, s: AntennaSignal) {
        self.signals.push(s);
    }

    pub fn best_signal(&self, band: Band) -> Option<&AntennaSignal> {
        self.signals
            .iter()
            .filter(|s| s.band == band)
            .max_by(|a, b| {
                a.strength_dbm
                    .partial_cmp(&b.strength_dbm)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    pub fn nav_signal_ok(&self) -> bool {
        self.signals
            .iter()
            .any(|s| s.band.is_navigation() && s.is_usable())
    }

    pub fn entertainment_available(&self) -> bool {
        self.signals
            .iter()
            .any(|s| s.band.is_entertainment() && s.is_usable())
    }

    pub fn total_bands_available(&self) -> usize {
        self.signals.iter().filter(|s| s.is_usable()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freq_range() {
        let (lo, hi) = Band::FM.frequency_range_mhz();
        assert!(lo < hi);
    }

    #[test]
    fn test_is_nav() {
        assert!(Band::GPS.is_navigation());
        assert!(!Band::FM.is_navigation());
    }

    #[test]
    fn test_quality() {
        let s = AntennaSignal::new(Band::FM, -40.0, 30.0);
        assert!(s.quality_pct() > 90.0);
    }

    #[test]
    fn test_usable() {
        let s = AntennaSignal::new(Band::GPS, -80.0, 20.0);
        assert!(s.is_usable());
    }

    #[test]
    fn test_not_usable() {
        let s = AntennaSignal::new(Band::GPS, -110.0, 5.0);
        assert!(!s.is_usable());
    }

    #[test]
    fn test_interference() {
        let mut s = AntennaSignal::new(Band::LTE, -60.0, 25.0);
        s.interference_level = 50.0;
        assert!(s.has_interference());
    }

    #[test]
    fn test_effective_snr() {
        let s = AntennaSignal::new(Band::FM, -50.0, 30.0);
        assert!(s.effective_snr() > 25.0);
    }

    #[test]
    fn test_best_signal() {
        let mut sys = AntennaSystem::new();
        sys.add_signal(AntennaSignal::new(Band::FM, -60.0, 20.0));
        sys.add_signal(AntennaSignal::new(Band::FM, -40.0, 30.0));
        assert!(sys.best_signal(Band::FM).unwrap().strength_dbm > -50.0);
    }

    #[test]
    fn test_nav_ok() {
        let mut sys = AntennaSystem::new();
        sys.add_signal(AntennaSignal::new(Band::GPS, -70.0, 25.0));
        assert!(sys.nav_signal_ok());
    }

    #[test]
    fn test_bands_count() {
        let mut sys = AntennaSystem::new();
        sys.add_signal(AntennaSignal::new(Band::FM, -50.0, 25.0));
        sys.add_signal(AntennaSignal::new(Band::GPS, -70.0, 20.0));
        assert_eq!(sys.total_bands_available(), 2);
    }
}
