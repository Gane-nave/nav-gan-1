//! Vehicle envelope — physical and regulatory profile used for
//! vehicle-aware routing (mirrors the product's vehicle profile catalog).

use serde::{Deserialize, Serialize};

use crate::map::{RoadClass, RoadSegment};

/// The eleven canonical vehicle classes of G.A.N.E NAV.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VehicleClass {
    Car,
    Motorcycle,
    Van,
    LightTruck,
    MediumTruck,
    HeavyTruck,
    Bus,
    Emergency,
    Military,
    Bicycle,
    Pedestrian,
}

impl VehicleClass {
    /// Whether this class is a motorized road vehicle.
    pub fn is_motorized(self) -> bool {
        !matches!(self, VehicleClass::Bicycle | VehicleClass::Pedestrian)
    }
}

/// Physical + regulatory envelope of a vehicle, used to filter road segments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VehicleEnvelope {
    pub class: VehicleClass,
    pub height_m: f64,
    pub width_m: f64,
    pub length_m: f64,
    /// Gross (loaded) weight in kilograms.
    pub weight_kg: f64,
    pub axle_count: u8,
    /// Carrying hazardous materials.
    pub hazmat: bool,
}

impl VehicleEnvelope {
    /// A default passenger-car envelope.
    pub fn car() -> Self {
        Self {
            class: VehicleClass::Car,
            height_m: 1.6,
            width_m: 1.8,
            length_m: 4.5,
            weight_kg: 1_800.0,
            axle_count: 2,
            hazmat: false,
        }
    }

    /// A default heavy-truck envelope (4.2 m tall, 26 t).
    pub fn heavy_truck() -> Self {
        Self {
            class: VehicleClass::HeavyTruck,
            height_m: 4.2,
            width_m: 2.55,
            length_m: 16.5,
            weight_kg: 26_000.0,
            axle_count: 5,
            hazmat: false,
        }
    }
}

/// Why a segment is not traversable by a given envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestrictionViolation {
    HeightLimit,
    WeightLimit,
    HazmatRestricted,
    ClassForbidden,
}

impl VehicleEnvelope {
    /// Check whether this vehicle may legally and physically traverse `seg`.
    ///
    /// Hard constraints only — soft preferences (grade, comfort) belong in
    /// cost functions, not here.
    pub fn permits(&self, seg: &RoadSegment) -> Result<(), RestrictionViolation> {
        if let Some(limit) = seg.height_limit_m {
            if self.height_m > limit {
                return Err(RestrictionViolation::HeightLimit);
            }
        }
        if let Some(limit) = seg.weight_limit_kg {
            if self.weight_kg > limit {
                return Err(RestrictionViolation::WeightLimit);
            }
        }
        if self.hazmat && (seg.hazmat_restricted || seg.tunnel) {
            return Err(RestrictionViolation::HazmatRestricted);
        }
        if !self.class_allowed_on(seg.road_class) {
            return Err(RestrictionViolation::ClassForbidden);
        }
        Ok(())
    }

    fn class_allowed_on(&self, road: RoadClass) -> bool {
        match road {
            RoadClass::Pedestrian => {
                matches!(
                    self.class,
                    VehicleClass::Pedestrian | VehicleClass::Bicycle | VehicleClass::Emergency
                )
            }
            RoadClass::Cycleway => {
                matches!(self.class, VehicleClass::Bicycle | VehicleClass::Pedestrian)
            }
            RoadClass::Motorway | RoadClass::Trunk => self.class.is_motorized(),
            RoadClass::Track => self.class != VehicleClass::HeavyTruck,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::{RoadClass, RoadSegment, SurfaceType};
    use crate::types::EntityId;

    fn seg(height: Option<f64>, weight: Option<f64>, hazmat_restricted: bool) -> RoadSegment {
        RoadSegment {
            id: EntityId::new(),
            from_node: EntityId::new(),
            to_node: EntityId::new(),
            geometry: vec![],
            road_class: RoadClass::Primary,
            one_way: false,
            speed_limit_kmh: Some(50.0),
            lane_count: Some(2),
            surface_type: SurfaceType::Asphalt,
            bridge: false,
            tunnel: false,
            toll: false,
            weight_limit_kg: weight,
            height_limit_m: height,
            hazmat_restricted,
            length_m: 100.0,
            travel_time_s: Some(10.0),
        }
    }

    #[test]
    fn car_passes_unrestricted_segment() {
        assert!(VehicleEnvelope::car()
            .permits(&seg(None, None, false))
            .is_ok());
    }

    #[test]
    fn truck_blocked_by_low_bridge() {
        let truck = VehicleEnvelope::heavy_truck();
        assert_eq!(
            truck.permits(&seg(Some(4.0), None, false)),
            Err(RestrictionViolation::HeightLimit)
        );
    }

    #[test]
    fn car_passes_low_bridge() {
        assert!(VehicleEnvelope::car()
            .permits(&seg(Some(4.0), None, false))
            .is_ok());
    }

    #[test]
    fn truck_blocked_by_weight_limit() {
        let truck = VehicleEnvelope::heavy_truck();
        assert_eq!(
            truck.permits(&seg(None, Some(10_000.0), false)),
            Err(RestrictionViolation::WeightLimit)
        );
    }

    #[test]
    fn hazmat_blocked_in_tunnel() {
        let mut truck = VehicleEnvelope::heavy_truck();
        truck.hazmat = true;
        let mut s = seg(None, None, false);
        s.tunnel = true;
        assert_eq!(
            truck.permits(&s),
            Err(RestrictionViolation::HazmatRestricted)
        );
    }

    #[test]
    fn hazmat_blocked_on_restricted_segment() {
        let mut truck = VehicleEnvelope::heavy_truck();
        truck.hazmat = true;
        assert_eq!(
            truck.permits(&seg(None, None, true)),
            Err(RestrictionViolation::HazmatRestricted)
        );
    }

    #[test]
    fn car_forbidden_on_pedestrian_way() {
        let mut s = seg(None, None, false);
        s.road_class = RoadClass::Pedestrian;
        assert_eq!(
            VehicleEnvelope::car().permits(&s),
            Err(RestrictionViolation::ClassForbidden)
        );
    }

    #[test]
    fn emergency_allowed_on_pedestrian_way() {
        let mut s = seg(None, None, false);
        s.road_class = RoadClass::Pedestrian;
        let mut ambulance = VehicleEnvelope::car();
        ambulance.class = VehicleClass::Emergency;
        assert!(ambulance.permits(&s).is_ok());
    }
}
