//! Hospital entrance navigation — routes emergency vehicles to the correct
//! hospital entrance (ER, trauma, helipad, ambulance bay) with real-time
//! capacity and diversion logic.

use chrono::{DateTime, Utc};
use gane_core::types::{EntityId, GeoPosition};
use serde::{Deserialize, Serialize};
use tracing::debug;

/// Type of hospital entrance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntranceType {
    /// Emergency room / A&E entrance.
    EmergencyRoom,
    /// Trauma centre entrance.
    TraumaCentre,
    /// Ambulance bay (vehicle drop-off).
    AmbulanceBay,
    /// Helipad.
    Helipad,
    /// Main entrance (walk-in).
    MainEntrance,
    /// Service / loading entrance.
    ServiceEntrance,
}

/// Hospital entrance with location and status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HospitalEntrance {
    pub id: EntityId,
    pub entrance_type: EntranceType,
    pub position: GeoPosition,
    pub name: Option<String>,
    pub is_accessible: bool,
    pub is_open: bool,
    pub notes: Option<String>,
}

/// Hospital capacity status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapacityStatus {
    /// Normal operations — accepting patients.
    Normal,
    /// Busy but accepting.
    Busy,
    /// Near capacity — may divert non-critical.
    NearCapacity,
    /// Full — diverting all except critical.
    Diverting,
    /// Closed to new patients.
    Closed,
}

/// A hospital record with entrances and capacity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hospital {
    pub id: EntityId,
    pub name: String,
    pub position: GeoPosition,
    pub entrances: Vec<HospitalEntrance>,
    pub capacity_status: CapacityStatus,
    pub has_trauma_centre: bool,
    pub has_helipad: bool,
    pub specialties: Vec<String>,
    pub updated_at: DateTime<Utc>,
}

/// A routing recommendation to a hospital entrance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HospitalRouteRecommendation {
    pub hospital_id: EntityId,
    pub hospital_name: String,
    pub entrance_id: EntityId,
    pub entrance_type: EntranceType,
    pub entrance_position: GeoPosition,
    pub distance_km: f64,
    pub estimated_time_min: f64,
    pub capacity_status: CapacityStatus,
    pub score: f64,
}

/// Patient acuity level — determines hospital and entrance selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatientAcuity {
    /// Minor injuries — any ER.
    Minor,
    /// Moderate — ER with capacity.
    Moderate,
    /// Severe — trauma centre preferred.
    Severe,
    /// Critical — nearest trauma centre, ambulance bay.
    Critical,
}

/// Hospital navigation engine — finds and routes to the best hospital entrance.
pub struct HospitalNavigator {
    hospitals: Vec<Hospital>,
    /// Maximum search radius in km.
    max_search_radius_km: f64,
    /// Emergency vehicle average speed in km/h.
    emergency_speed_kmh: f64,
}

impl HospitalNavigator {
    pub fn new() -> Self {
        Self {
            hospitals: Vec::new(),
            max_search_radius_km: 30.0,
            emergency_speed_kmh: 80.0,
        }
    }

    /// Register a hospital.
    pub fn add_hospital(&mut self, hospital: Hospital) {
        debug!(hospital_id = %hospital.id, name = %hospital.name, "Hospital registered");
        self.hospitals.push(hospital);
    }

    /// Update hospital capacity status.
    pub fn update_capacity(&mut self, hospital_id: &EntityId, status: CapacityStatus) -> bool {
        if let Some(h) = self.hospitals.iter_mut().find(|h| h.id == *hospital_id) {
            h.capacity_status = status;
            h.updated_at = Utc::now();
            true
        } else {
            false
        }
    }

    /// Update entrance availability.
    pub fn update_entrance(
        &mut self,
        hospital_id: &EntityId,
        entrance_id: &EntityId,
        is_open: bool,
    ) -> bool {
        let Some(hospital) = self.hospitals.iter_mut().find(|h| h.id == *hospital_id) else {
            return false;
        };
        let Some(entrance) = hospital.entrances.iter_mut().find(|e| e.id == *entrance_id) else {
            return false;
        };
        entrance.is_open = is_open;
        true
    }

    /// Find the best hospital and entrance for a patient.
    pub fn find_best_hospital(
        &self,
        from: &GeoPosition,
        acuity: PatientAcuity,
    ) -> Vec<HospitalRouteRecommendation> {
        let mut recommendations: Vec<HospitalRouteRecommendation> = self
            .hospitals
            .iter()
            .filter(|h| self.is_hospital_suitable(h, acuity))
            .filter_map(|h| {
                let entrance = self.best_entrance(h, acuity)?;
                let distance_km = haversine_distance(from, &entrance.position) / 1000.0;
                if distance_km > self.max_search_radius_km {
                    return None;
                }
                let time_min = distance_km / self.emergency_speed_kmh * 60.0;
                let score = self.compute_score(h, &entrance, distance_km, acuity);

                Some(HospitalRouteRecommendation {
                    hospital_id: h.id,
                    hospital_name: h.name.clone(),
                    entrance_id: entrance.id,
                    entrance_type: entrance.entrance_type,
                    entrance_position: entrance.position,
                    distance_km,
                    estimated_time_min: time_min,
                    capacity_status: h.capacity_status,
                    score,
                })
            })
            .collect();

        // Sort by score descending (best first).
        recommendations.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        recommendations
    }

    /// Check if a hospital is suitable for the given acuity.
    fn is_hospital_suitable(&self, hospital: &Hospital, acuity: PatientAcuity) -> bool {
        // Closed hospitals reject everyone.
        if hospital.capacity_status == CapacityStatus::Closed {
            return false;
        }
        // Diverting hospitals only accept critical.
        if hospital.capacity_status == CapacityStatus::Diverting
            && acuity != PatientAcuity::Critical
        {
            return false;
        }
        // Severe/Critical need trauma if available in the region.
        if (acuity == PatientAcuity::Severe || acuity == PatientAcuity::Critical)
            && !hospital.has_trauma_centre
        {
            // Still consider non-trauma hospitals but with lower score.
            // Don't reject outright — may be the only option.
        }
        true
    }

    /// Select the best entrance for the given acuity.
    fn best_entrance(
        &self,
        hospital: &Hospital,
        acuity: PatientAcuity,
    ) -> Option<HospitalEntrance> {
        let preferred_types = match acuity {
            PatientAcuity::Critical => vec![
                EntranceType::TraumaCentre,
                EntranceType::AmbulanceBay,
                EntranceType::EmergencyRoom,
            ],
            PatientAcuity::Severe => vec![
                EntranceType::AmbulanceBay,
                EntranceType::TraumaCentre,
                EntranceType::EmergencyRoom,
            ],
            PatientAcuity::Moderate => {
                vec![EntranceType::EmergencyRoom, EntranceType::AmbulanceBay]
            }
            PatientAcuity::Minor => vec![EntranceType::EmergencyRoom, EntranceType::MainEntrance],
        };

        // Try preferred entrance types in order.
        for pref in &preferred_types {
            if let Some(entrance) = hospital
                .entrances
                .iter()
                .find(|e| e.entrance_type == *pref && e.is_open)
            {
                return Some(entrance.clone());
            }
        }

        // Fallback: any open entrance.
        hospital.entrances.iter().find(|e| e.is_open).cloned()
    }

    /// Compute routing score (higher = better).
    fn compute_score(
        &self,
        hospital: &Hospital,
        _entrance: &HospitalEntrance,
        distance_km: f64,
        acuity: PatientAcuity,
    ) -> f64 {
        let mut score = 0.0;

        // Distance score (closer = better). Max 40 points.
        score += 40.0 / (1.0 + distance_km);

        // Capacity score. Max 30 points.
        score += match hospital.capacity_status {
            CapacityStatus::Normal => 30.0,
            CapacityStatus::Busy => 20.0,
            CapacityStatus::NearCapacity => 10.0,
            CapacityStatus::Diverting => 2.0,
            CapacityStatus::Closed => 0.0,
        };

        // Trauma centre bonus for severe/critical. Max 20 points.
        if (acuity == PatientAcuity::Severe || acuity == PatientAcuity::Critical)
            && hospital.has_trauma_centre
        {
            score += 20.0;
        }

        // Helipad bonus for critical. Max 10 points.
        if acuity == PatientAcuity::Critical && hospital.has_helipad {
            score += 10.0;
        }

        score
    }

    /// Get a hospital by ID.
    pub fn hospital(&self, id: &EntityId) -> Option<&Hospital> {
        self.hospitals.iter().find(|h| h.id == *id)
    }

    /// Hospital count.
    pub fn hospital_count(&self) -> usize {
        self.hospitals.len()
    }

    /// Set search radius.
    pub fn set_search_radius_km(&mut self, km: f64) {
        self.max_search_radius_km = km;
    }
}

impl Default for HospitalNavigator {
    fn default() -> Self {
        Self::new()
    }
}

/// Haversine distance in metres.
fn haversine_distance(a: &GeoPosition, b: &GeoPosition) -> f64 {
    let r = 6_371_000.0;
    let d_lat = (b.latitude_deg - a.latitude_deg).to_radians();
    let d_lon = (b.longitude_deg - a.longitude_deg).to_radians();
    let lat1 = a.latitude_deg.to_radians();
    let lat2 = b.latitude_deg.to_radians();
    let h = (d_lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (d_lon / 2.0).sin().powi(2);
    2.0 * r * h.sqrt().asin()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pos(lat: f64, lon: f64) -> GeoPosition {
        GeoPosition {
            latitude_deg: lat,
            longitude_deg: lon,
            altitude_m: None,
        }
    }

    fn make_hospital(
        name: &str,
        lat: f64,
        lon: f64,
        has_trauma: bool,
        capacity: CapacityStatus,
    ) -> Hospital {
        let er_pos = pos(lat + 0.001, lon);
        let amb_pos = pos(lat, lon + 0.001);
        let mut entrances = vec![
            HospitalEntrance {
                id: EntityId::new(),
                entrance_type: EntranceType::EmergencyRoom,
                position: er_pos,
                name: Some("ER".into()),
                is_accessible: true,
                is_open: true,
                notes: None,
            },
            HospitalEntrance {
                id: EntityId::new(),
                entrance_type: EntranceType::AmbulanceBay,
                position: amb_pos,
                name: Some("Ambulance Bay".into()),
                is_accessible: true,
                is_open: true,
                notes: None,
            },
            HospitalEntrance {
                id: EntityId::new(),
                entrance_type: EntranceType::MainEntrance,
                position: pos(lat - 0.001, lon),
                name: Some("Main".into()),
                is_accessible: true,
                is_open: true,
                notes: None,
            },
        ];
        if has_trauma {
            entrances.push(HospitalEntrance {
                id: EntityId::new(),
                entrance_type: EntranceType::TraumaCentre,
                position: pos(lat + 0.002, lon),
                name: Some("Trauma".into()),
                is_accessible: true,
                is_open: true,
                notes: None,
            });
        }
        Hospital {
            id: EntityId::new(),
            name: name.to_string(),
            position: pos(lat, lon),
            entrances,
            capacity_status: capacity,
            has_trauma_centre: has_trauma,
            has_helipad: has_trauma, // Trauma centres usually have helipads.
            specialties: vec!["Emergency Medicine".into()],
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn find_nearest_hospital() {
        let mut nav = HospitalNavigator::new();
        nav.add_hospital(make_hospital(
            "Far Hospital",
            32.1,
            34.1,
            false,
            CapacityStatus::Normal,
        ));
        nav.add_hospital(make_hospital(
            "Near Hospital",
            32.001,
            34.001,
            false,
            CapacityStatus::Normal,
        ));

        let recs = nav.find_best_hospital(&pos(32.0, 34.0), PatientAcuity::Minor);
        assert!(!recs.is_empty());
        assert_eq!(recs[0].hospital_name, "Near Hospital");
    }

    #[test]
    fn trauma_centre_preferred_for_critical() {
        let mut nav = HospitalNavigator::new();
        // Close non-trauma hospital.
        nav.add_hospital(make_hospital(
            "Close Basic",
            32.001,
            34.001,
            false,
            CapacityStatus::Normal,
        ));
        // Slightly further trauma centre.
        nav.add_hospital(make_hospital(
            "Trauma Centre",
            32.01,
            34.01,
            true,
            CapacityStatus::Normal,
        ));

        let recs = nav.find_best_hospital(&pos(32.0, 34.0), PatientAcuity::Critical);
        assert!(!recs.is_empty());
        // Trauma centre should rank first despite being further.
        assert_eq!(recs[0].hospital_name, "Trauma Centre");
    }

    #[test]
    fn critical_patient_routes_to_trauma_entrance() {
        let mut nav = HospitalNavigator::new();
        nav.add_hospital(make_hospital(
            "Hospital A",
            32.01,
            34.01,
            true,
            CapacityStatus::Normal,
        ));

        let recs = nav.find_best_hospital(&pos(32.0, 34.0), PatientAcuity::Critical);
        assert!(!recs.is_empty());
        assert_eq!(recs[0].entrance_type, EntranceType::TraumaCentre);
    }

    #[test]
    fn minor_patient_routes_to_er() {
        let mut nav = HospitalNavigator::new();
        nav.add_hospital(make_hospital(
            "Hospital A",
            32.01,
            34.01,
            true,
            CapacityStatus::Normal,
        ));

        let recs = nav.find_best_hospital(&pos(32.0, 34.0), PatientAcuity::Minor);
        assert!(!recs.is_empty());
        assert_eq!(recs[0].entrance_type, EntranceType::EmergencyRoom);
    }

    #[test]
    fn diverting_hospital_rejected_for_minor() {
        let mut nav = HospitalNavigator::new();
        nav.add_hospital(make_hospital(
            "Diverting",
            32.001,
            34.001,
            false,
            CapacityStatus::Diverting,
        ));
        nav.add_hospital(make_hospital(
            "Normal",
            32.01,
            34.01,
            false,
            CapacityStatus::Normal,
        ));

        let recs = nav.find_best_hospital(&pos(32.0, 34.0), PatientAcuity::Minor);
        assert!(!recs.is_empty());
        assert_eq!(recs[0].hospital_name, "Normal");
    }

    #[test]
    fn diverting_hospital_accepted_for_critical() {
        let mut nav = HospitalNavigator::new();
        nav.add_hospital(make_hospital(
            "Diverting Trauma",
            32.001,
            34.001,
            true,
            CapacityStatus::Diverting,
        ));

        let recs = nav.find_best_hospital(&pos(32.0, 34.0), PatientAcuity::Critical);
        assert!(!recs.is_empty());
        assert_eq!(recs[0].hospital_name, "Diverting Trauma");
    }

    #[test]
    fn closed_hospital_excluded() {
        let mut nav = HospitalNavigator::new();
        nav.add_hospital(make_hospital(
            "Closed",
            32.001,
            34.001,
            true,
            CapacityStatus::Closed,
        ));

        let recs = nav.find_best_hospital(&pos(32.0, 34.0), PatientAcuity::Critical);
        assert!(recs.is_empty());
    }

    #[test]
    fn update_capacity_and_entrance() {
        let mut nav = HospitalNavigator::new();
        let hospital = make_hospital("Hospital", 32.0, 34.0, false, CapacityStatus::Normal);
        let h_id = hospital.id;
        let e_id = hospital.entrances[0].id;
        nav.add_hospital(hospital);

        assert!(nav.update_capacity(&h_id, CapacityStatus::Busy));
        assert_eq!(
            nav.hospital(&h_id).unwrap().capacity_status,
            CapacityStatus::Busy
        );

        assert!(nav.update_entrance(&h_id, &e_id, false));
        assert!(!nav.hospital(&h_id).unwrap().entrances[0].is_open);
    }

    #[test]
    fn out_of_range_excluded() {
        let mut nav = HospitalNavigator::new();
        nav.set_search_radius_km(5.0);
        nav.add_hospital(make_hospital(
            "Far Away",
            33.0,
            35.0,
            true,
            CapacityStatus::Normal,
        ));

        let recs = nav.find_best_hospital(&pos(32.0, 34.0), PatientAcuity::Minor);
        assert!(recs.is_empty());
    }

    #[test]
    fn capacity_affects_ranking() {
        let mut nav = HospitalNavigator::new();
        // Same distance, different capacity.
        nav.add_hospital(make_hospital(
            "Busy",
            32.01,
            34.0,
            false,
            CapacityStatus::NearCapacity,
        ));
        nav.add_hospital(make_hospital(
            "Available",
            32.0,
            34.01,
            false,
            CapacityStatus::Normal,
        ));

        let recs = nav.find_best_hospital(&pos(32.0, 34.0), PatientAcuity::Moderate);
        assert!(recs.len() >= 2);
        // Available should rank higher due to capacity score.
        assert_eq!(recs[0].hospital_name, "Available");
    }

    #[test]
    fn closed_entrance_skipped() {
        let mut nav = HospitalNavigator::new();
        let mut hospital = make_hospital("Hospital", 32.01, 34.01, false, CapacityStatus::Normal);
        // Close the ER entrance.
        hospital.entrances[0].is_open = false;
        nav.add_hospital(hospital);

        let recs = nav.find_best_hospital(&pos(32.0, 34.0), PatientAcuity::Minor);
        assert!(!recs.is_empty());
        // Should route to next available entrance (ambulance bay or main).
        assert_ne!(recs[0].entrance_type, EntranceType::EmergencyRoom);
    }
}
