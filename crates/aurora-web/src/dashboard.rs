//! Dashboard data model for the web UI.

use serde::{Deserialize, Serialize};

/// Complete dashboard data sent to the frontend via JSON API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    /// Current position.
    pub position: PositionData,
    /// Active route information.
    pub route: RouteData,
    /// Satellite constellation status.
    pub satellites: SatelliteData,
    /// Integrity and continuity status.
    pub integrity: IntegrityData,
    /// Traffic overlay data.
    pub traffic: TrafficData,
    /// Fleet tracking data.
    pub fleet: FleetData,
    /// Emergency status.
    pub emergency: EmergencyData,
    /// Smart city signals.
    pub city: CityData,
    /// Performance metrics.
    pub metrics: MetricsData,
    /// System health.
    pub health: HealthData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionData {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude_m: f64,
    pub speed_kmh: f64,
    pub heading_deg: f64,
    pub accuracy_m: f64,
    pub fix_type: String,
    pub timestamp_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteData {
    pub active: bool,
    pub origin: [f64; 2],
    pub destination: [f64; 2],
    pub waypoints: Vec<[f64; 2]>,
    pub distance_km: f64,
    pub eta_minutes: f64,
    pub current_step: String,
    pub next_turn: String,
    pub next_turn_distance_m: f64,
    pub traffic_delay_minutes: f64,
    pub alternative_routes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SatelliteData {
    pub tracked: u32,
    pub used_in_fix: u32,
    pub gps_count: u32,
    pub galileo_count: u32,
    pub glonass_count: u32,
    pub beidou_count: u32,
    pub hdop: f64,
    pub vdop: f64,
    pub pdop: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityData {
    pub level: String,
    pub continuity_mode: String,
    pub protection_level_m: f64,
    pub jamming_detected: bool,
    pub spoofing_detected: bool,
    pub correction_age_s: f64,
    pub raim_available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficData {
    pub congestion_level: String,
    pub incidents_nearby: u32,
    pub average_speed_kmh: f64,
    pub segments: Vec<TrafficSegment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficSegment {
    pub start: [f64; 2],
    pub end: [f64; 2],
    pub speed_ratio: f64,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FleetData {
    pub enabled: bool,
    pub vehicles_tracked: u32,
    pub nearby_vehicles: Vec<VehicleInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleInfo {
    pub id: String,
    pub lat: f64,
    pub lon: f64,
    pub speed_kmh: f64,
    pub heading_deg: f64,
    pub vehicle_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyData {
    pub active: bool,
    pub nearest_hospital_km: f64,
    pub nearest_police_km: f64,
    pub nearest_fire_km: f64,
    pub corridor_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CityData {
    pub connected: bool,
    pub traffic_lights_ahead: u32,
    pub green_wave_active: bool,
    pub smart_parking_spots: u32,
    pub ev_chargers_nearby: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsData {
    pub pipeline_latency_ms: f64,
    pub position_update_hz: f64,
    pub cache_hit_rate: f64,
    pub circuit_breaker_open: bool,
    pub pending_requests: u32,
    pub uptime_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthData {
    pub overall: String,
    pub gnss: String,
    pub fusion: String,
    pub integrity: String,
    pub routing: String,
    pub traffic: String,
    pub api: String,
}

impl Default for DashboardData {
    fn default() -> Self {
        Self {
            position: PositionData {
                latitude: 32.0853,
                longitude: 34.7818,
                altitude_m: 25.0,
                speed_kmh: 0.0,
                heading_deg: 0.0,
                accuracy_m: 2.5,
                fix_type: "RTK_FIXED".into(),
                timestamp_ms: 0,
            },
            route: RouteData {
                active: false,
                origin: [32.0853, 34.7818],
                destination: [31.7683, 35.2137],
                waypoints: vec![],
                distance_km: 0.0,
                eta_minutes: 0.0,
                current_step: "Waiting for route".into(),
                next_turn: "—".into(),
                next_turn_distance_m: 0.0,
                traffic_delay_minutes: 0.0,
                alternative_routes: 0,
            },
            satellites: SatelliteData {
                tracked: 0,
                used_in_fix: 0,
                gps_count: 0,
                galileo_count: 0,
                glonass_count: 0,
                beidou_count: 0,
                hdop: 99.0,
                vdop: 99.0,
                pdop: 99.0,
            },
            integrity: IntegrityData {
                level: "Unknown".into(),
                continuity_mode: "Normal".into(),
                protection_level_m: 0.0,
                jamming_detected: false,
                spoofing_detected: false,
                correction_age_s: 0.0,
                raim_available: false,
            },
            traffic: TrafficData {
                congestion_level: "Unknown".into(),
                incidents_nearby: 0,
                average_speed_kmh: 0.0,
                segments: vec![],
            },
            fleet: FleetData {
                enabled: false,
                vehicles_tracked: 0,
                nearby_vehicles: vec![],
            },
            emergency: EmergencyData {
                active: false,
                nearest_hospital_km: 0.0,
                nearest_police_km: 0.0,
                nearest_fire_km: 0.0,
                corridor_active: false,
            },
            city: CityData {
                connected: false,
                traffic_lights_ahead: 0,
                green_wave_active: false,
                smart_parking_spots: 0,
                ev_chargers_nearby: 0,
            },
            metrics: MetricsData {
                pipeline_latency_ms: 0.0,
                position_update_hz: 0.0,
                cache_hit_rate: 0.0,
                circuit_breaker_open: false,
                pending_requests: 0,
                uptime_seconds: 0,
            },
            health: HealthData {
                overall: "Initializing".into(),
                gnss: "Initializing".into(),
                fusion: "Initializing".into(),
                integrity: "Initializing".into(),
                routing: "Initializing".into(),
                traffic: "Initializing".into(),
                api: "Initializing".into(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_dashboard_serializes() {
        let data = DashboardData::default();
        let json = serde_json::to_string(&data).unwrap();
        assert!(json.contains("latitude"));
        assert!(json.contains("RTK_FIXED"));
    }

    #[test]
    fn default_position_is_tel_aviv() {
        let data = DashboardData::default();
        assert!((data.position.latitude - 32.0853).abs() < 0.001);
        assert!((data.position.longitude - 34.7818).abs() < 0.001);
    }

    #[test]
    fn default_route_inactive() {
        let data = DashboardData::default();
        assert!(!data.route.active);
    }

    #[test]
    fn default_satellites_zero() {
        let data = DashboardData::default();
        assert_eq!(data.satellites.tracked, 0);
    }

    #[test]
    fn default_health_initializing() {
        let data = DashboardData::default();
        assert_eq!(data.health.overall, "Initializing");
    }

    #[test]
    fn dashboard_roundtrip_json() {
        let data = DashboardData::default();
        let json = serde_json::to_string(&data).unwrap();
        let parsed: DashboardData = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.position.latitude, data.position.latitude);
        assert_eq!(parsed.satellites.tracked, data.satellites.tracked);
    }

    #[test]
    fn traffic_segment_serializes() {
        let seg = TrafficSegment {
            start: [32.0, 34.0],
            end: [32.1, 34.1],
            speed_ratio: 0.7,
            color: "#FFA500".into(),
        };
        let json = serde_json::to_string(&seg).unwrap();
        assert!(json.contains("speed_ratio"));
        assert!(json.contains("#FFA500"));
    }

    #[test]
    fn vehicle_info_serializes() {
        let v = VehicleInfo {
            id: "V001".into(),
            lat: 32.0,
            lon: 34.0,
            speed_kmh: 60.0,
            heading_deg: 90.0,
            vehicle_type: "sedan".into(),
        };
        let json = serde_json::to_string(&v).unwrap();
        assert!(json.contains("V001"));
    }
}
