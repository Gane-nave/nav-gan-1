/// Hazardous materials routing: restricted zones, emergency protocols, compliance.
#[derive(Debug, Clone, PartialEq)]
pub enum HazmatClass {
    Explosives,
    Gases,
    FlammableLiquids,
    FlammableSolids,
    Oxidizers,
    Toxics,
    Radioactive,
    Corrosives,
    Miscellaneous,
}

impl HazmatClass {
    pub fn un_class(&self) -> u8 {
        match self {
            HazmatClass::Explosives => 1,
            HazmatClass::Gases => 2,
            HazmatClass::FlammableLiquids => 3,
            HazmatClass::FlammableSolids => 4,
            HazmatClass::Oxidizers => 5,
            HazmatClass::Toxics => 6,
            HazmatClass::Radioactive => 7,
            HazmatClass::Corrosives => 8,
            HazmatClass::Miscellaneous => 9,
        }
    }

    pub fn risk_level(&self) -> f64 {
        match self {
            HazmatClass::Explosives => 1.0,
            HazmatClass::Radioactive => 0.95,
            HazmatClass::Toxics => 0.85,
            HazmatClass::FlammableLiquids => 0.7,
            HazmatClass::Gases => 0.7,
            HazmatClass::Corrosives => 0.6,
            HazmatClass::FlammableSolids => 0.55,
            HazmatClass::Oxidizers => 0.5,
            HazmatClass::Miscellaneous => 0.2,
        }
    }

    pub fn tunnel_restricted(&self) -> bool {
        matches!(
            self,
            HazmatClass::Explosives | HazmatClass::Radioactive | HazmatClass::Gases
        )
    }

    pub fn requires_placard(&self) -> bool {
        !matches!(self, HazmatClass::Miscellaneous)
    }

    pub fn evacuation_radius_m(&self) -> f64 {
        match self {
            HazmatClass::Explosives => 1500.0,
            HazmatClass::Radioactive => 1000.0,
            HazmatClass::Toxics => 800.0,
            HazmatClass::Gases => 500.0,
            _ => 300.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HazmatShipment {
    pub hazmat_class: HazmatClass,
    pub quantity_kg: f64,
    pub proper_name: String,
    pub un_number: u32,
}

impl HazmatShipment {
    pub fn new(class: HazmatClass, quantity_kg: f64, name: &str, un_number: u32) -> Self {
        Self {
            hazmat_class: class,
            quantity_kg,
            proper_name: name.to_string(),
            un_number,
        }
    }

    pub fn max_speed_kmh(&self) -> f64 {
        let base = 80.0;
        let risk_reduction = self.hazmat_class.risk_level() * 20.0;
        base - risk_reduction
    }

    pub fn requires_escort(&self) -> bool {
        self.hazmat_class.risk_level() > 0.9 || self.quantity_kg > 5000.0
    }

    pub fn rest_stop_interval_hr(&self) -> f64 {
        if self.hazmat_class.risk_level() > 0.8 {
            2.0
        } else {
            4.0
        }
    }

    pub fn can_use_tunnel(&self) -> bool {
        !self.hazmat_class.tunnel_restricted()
    }

    pub fn emergency_response_level(&self) -> &str {
        let risk = self.hazmat_class.risk_level();
        if risk > 0.9 {
            "Critical"
        } else if risk > 0.7 {
            "High"
        } else if risk > 0.4 {
            "Medium"
        } else {
            "Low"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_un_class() {
        assert_eq!(HazmatClass::Explosives.un_class(), 1);
        assert_eq!(HazmatClass::Miscellaneous.un_class(), 9);
    }

    #[test]
    fn test_risk_explosives() {
        assert!((HazmatClass::Explosives.risk_level() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_tunnel_restricted() {
        assert!(HazmatClass::Explosives.tunnel_restricted());
        assert!(!HazmatClass::Corrosives.tunnel_restricted());
    }

    #[test]
    fn test_placard() {
        assert!(HazmatClass::FlammableLiquids.requires_placard());
        assert!(!HazmatClass::Miscellaneous.requires_placard());
    }

    #[test]
    fn test_evac_radius() {
        assert!(
            HazmatClass::Explosives.evacuation_radius_m()
                > HazmatClass::Corrosives.evacuation_radius_m()
        );
    }

    #[test]
    fn test_max_speed() {
        let s = HazmatShipment::new(HazmatClass::Explosives, 100.0, "TNT", 81);
        assert!(s.max_speed_kmh() < 65.0);
    }

    #[test]
    fn test_escort_required() {
        let s = HazmatShipment::new(HazmatClass::Explosives, 100.0, "TNT", 81);
        assert!(s.requires_escort());
    }

    #[test]
    fn test_no_escort() {
        let s = HazmatShipment::new(HazmatClass::Miscellaneous, 50.0, "Dry Ice", 1845);
        assert!(!s.requires_escort());
    }

    #[test]
    fn test_tunnel_use() {
        let s = HazmatShipment::new(HazmatClass::Corrosives, 100.0, "Acid", 1830);
        assert!(s.can_use_tunnel());
    }

    #[test]
    fn test_no_tunnel() {
        let s = HazmatShipment::new(HazmatClass::Radioactive, 50.0, "Uranium", 2977);
        assert!(!s.can_use_tunnel());
    }

    #[test]
    fn test_rest_interval() {
        let high = HazmatShipment::new(HazmatClass::Explosives, 100.0, "TNT", 81);
        let low = HazmatShipment::new(HazmatClass::Miscellaneous, 50.0, "Dry Ice", 1845);
        assert!(high.rest_stop_interval_hr() < low.rest_stop_interval_hr());
    }

    #[test]
    fn test_emergency_level() {
        let s = HazmatShipment::new(HazmatClass::Explosives, 100.0, "TNT", 81);
        assert_eq!(s.emergency_response_level(), "Critical");
    }
}
