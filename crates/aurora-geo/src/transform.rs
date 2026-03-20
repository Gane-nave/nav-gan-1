//! Coordinate transforms — conversions between geographic coordinate systems.

use std::f64::consts::PI;

/// WGS-84 ellipsoid semi-major axis (meters).
const WGS84_A: f64 = 6_378_137.0;
/// WGS-84 flattening.
const WGS84_F: f64 = 1.0 / 298.257_223_563;
/// WGS-84 semi-minor axis.
const WGS84_B: f64 = WGS84_A * (1.0 - WGS84_F);
/// WGS-84 first eccentricity squared.
const WGS84_E2: f64 = 1.0 - (WGS84_B * WGS84_B) / (WGS84_A * WGS84_A);

/// Geographic coordinates (latitude, longitude, altitude).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Geographic {
    /// Latitude in degrees (-90 to 90).
    pub lat: f64,
    /// Longitude in degrees (-180 to 180).
    pub lon: f64,
    /// Altitude above ellipsoid in meters.
    pub alt: f64,
}

impl Geographic {
    /// Create new geographic coordinates.
    pub fn new(lat: f64, lon: f64, alt: f64) -> Self {
        Self { lat, lon, alt }
    }
}

/// Earth-Centered, Earth-Fixed (ECEF) coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ecef {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Ecef {
    /// Create new ECEF coordinates.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

/// East-North-Up (ENU) local tangent plane coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Enu {
    /// East in meters.
    pub east: f64,
    /// North in meters.
    pub north: f64,
    /// Up in meters.
    pub up: f64,
}

impl Enu {
    /// Create new ENU coordinates.
    pub fn new(east: f64, north: f64, up: f64) -> Self {
        Self { east, north, up }
    }

    /// Horizontal distance from origin.
    pub fn horizontal_distance(&self) -> f64 {
        (self.east * self.east + self.north * self.north).sqrt()
    }

    /// Bearing from origin in degrees (0=North, 90=East).
    pub fn bearing(&self) -> f64 {
        let rad = self.east.atan2(self.north);
        let deg = rad * 180.0 / PI;
        if deg < 0.0 {
            deg + 360.0
        } else {
            deg
        }
    }
}

fn deg2rad(d: f64) -> f64 {
    d * PI / 180.0
}

fn rad2deg(r: f64) -> f64 {
    r * 180.0 / PI
}

/// Convert geographic coordinates to ECEF.
pub fn geo_to_ecef(geo: &Geographic) -> Ecef {
    let lat = deg2rad(geo.lat);
    let lon = deg2rad(geo.lon);
    let sin_lat = lat.sin();
    let cos_lat = lat.cos();
    let sin_lon = lon.sin();
    let cos_lon = lon.cos();

    let n = WGS84_A / (1.0 - WGS84_E2 * sin_lat * sin_lat).sqrt();
    let x = (n + geo.alt) * cos_lat * cos_lon;
    let y = (n + geo.alt) * cos_lat * sin_lon;
    let z = (n * (1.0 - WGS84_E2) + geo.alt) * sin_lat;

    Ecef::new(x, y, z)
}

/// Convert ECEF coordinates to geographic (iterative method).
pub fn ecef_to_geo(ecef: &Ecef) -> Geographic {
    let p = (ecef.x * ecef.x + ecef.y * ecef.y).sqrt();
    let lon = ecef.y.atan2(ecef.x);

    // Iterative approach for latitude
    let mut lat = (ecef.z / p).atan();
    for _ in 0..10 {
        let sin_lat = lat.sin();
        let n = WGS84_A / (1.0 - WGS84_E2 * sin_lat * sin_lat).sqrt();
        lat = (ecef.z + WGS84_E2 * n * sin_lat).atan2(p);
    }

    let sin_lat = lat.sin();
    let n = WGS84_A / (1.0 - WGS84_E2 * sin_lat * sin_lat).sqrt();
    let alt = p / lat.cos() - n;

    Geographic::new(rad2deg(lat), rad2deg(lon), alt)
}

/// Convert ECEF to ENU relative to a reference point.
pub fn ecef_to_enu(ecef: &Ecef, ref_geo: &Geographic) -> Enu {
    let ref_ecef = geo_to_ecef(ref_geo);
    let dx = ecef.x - ref_ecef.x;
    let dy = ecef.y - ref_ecef.y;
    let dz = ecef.z - ref_ecef.z;

    let lat = deg2rad(ref_geo.lat);
    let lon = deg2rad(ref_geo.lon);
    let sin_lat = lat.sin();
    let cos_lat = lat.cos();
    let sin_lon = lon.sin();
    let cos_lon = lon.cos();

    let east = -sin_lon * dx + cos_lon * dy;
    let north = -sin_lat * cos_lon * dx - sin_lat * sin_lon * dy + cos_lat * dz;
    let up = cos_lat * cos_lon * dx + cos_lat * sin_lon * dy + sin_lat * dz;

    Enu::new(east, north, up)
}

/// Convert ENU to ECEF relative to a reference point.
pub fn enu_to_ecef(enu: &Enu, ref_geo: &Geographic) -> Ecef {
    let ref_ecef = geo_to_ecef(ref_geo);

    let lat = deg2rad(ref_geo.lat);
    let lon = deg2rad(ref_geo.lon);
    let sin_lat = lat.sin();
    let cos_lat = lat.cos();
    let sin_lon = lon.sin();
    let cos_lon = lon.cos();

    let dx = -sin_lon * enu.east - sin_lat * cos_lon * enu.north + cos_lat * cos_lon * enu.up;
    let dy = cos_lon * enu.east - sin_lat * sin_lon * enu.north + cos_lat * sin_lon * enu.up;
    let dz = cos_lat * enu.north + sin_lat * enu.up;

    Ecef::new(ref_ecef.x + dx, ref_ecef.y + dy, ref_ecef.z + dz)
}

/// Haversine distance between two geographic points in meters.
pub fn haversine_distance(a: &Geographic, b: &Geographic) -> f64 {
    let lat1 = deg2rad(a.lat);
    let lat2 = deg2rad(b.lat);
    let dlat = deg2rad(b.lat - a.lat);
    let dlon = deg2rad(b.lon - a.lon);

    let h = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * h.sqrt().asin();
    WGS84_A * c
}

/// Bearing from point A to point B in degrees (0=North, 90=East).
pub fn bearing(a: &Geographic, b: &Geographic) -> f64 {
    let lat1 = deg2rad(a.lat);
    let lat2 = deg2rad(b.lat);
    let dlon = deg2rad(b.lon - a.lon);

    let y = dlon.sin() * lat2.cos();
    let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * dlon.cos();
    let brng = y.atan2(x);
    let deg = rad2deg(brng);
    if deg < 0.0 {
        deg + 360.0
    } else {
        deg
    }
}

/// Destination point given start, bearing, and distance.
pub fn destination(start: &Geographic, bearing_deg: f64, distance_m: f64) -> Geographic {
    let lat1 = deg2rad(start.lat);
    let lon1 = deg2rad(start.lon);
    let brng = deg2rad(bearing_deg);
    let d = distance_m / WGS84_A;

    let lat2 = (lat1.sin() * d.cos() + lat1.cos() * d.sin() * brng.cos()).asin();
    let lon2 = lon1 + (brng.sin() * d.sin() * lat1.cos()).atan2(d.cos() - lat1.sin() * lat2.sin());

    Geographic::new(rad2deg(lat2), rad2deg(lon2), start.alt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geo_ecef_roundtrip() {
        let geo = Geographic::new(48.8566, 2.3522, 35.0); // Paris
        let ecef = geo_to_ecef(&geo);
        let back = ecef_to_geo(&ecef);
        assert!((back.lat - geo.lat).abs() < 1e-8);
        assert!((back.lon - geo.lon).abs() < 1e-8);
        assert!((back.alt - geo.alt).abs() < 0.01);
    }

    #[test]
    fn test_ecef_origin() {
        let geo = Geographic::new(0.0, 0.0, 0.0);
        let ecef = geo_to_ecef(&geo);
        assert!((ecef.x - WGS84_A).abs() < 1.0);
        assert!(ecef.y.abs() < 1.0);
        assert!(ecef.z.abs() < 1.0);
    }

    #[test]
    fn test_enu_at_origin() {
        let ref_geo = Geographic::new(0.0, 0.0, 0.0);
        let ref_ecef = geo_to_ecef(&ref_geo);
        let enu = ecef_to_enu(&ref_ecef, &ref_geo);
        assert!(enu.east.abs() < 1e-6);
        assert!(enu.north.abs() < 1e-6);
        assert!(enu.up.abs() < 1e-6);
    }

    #[test]
    fn test_enu_ecef_roundtrip() {
        let ref_geo = Geographic::new(37.7749, -122.4194, 0.0); // San Francisco
        let enu = Enu::new(100.0, 200.0, 50.0);
        let ecef = enu_to_ecef(&enu, &ref_geo);
        let back = ecef_to_enu(&ecef, &ref_geo);
        assert!((back.east - enu.east).abs() < 0.01);
        assert!((back.north - enu.north).abs() < 0.01);
        assert!((back.up - enu.up).abs() < 0.01);
    }

    #[test]
    fn test_haversine_known() {
        let london = Geographic::new(51.5074, -0.1278, 0.0);
        let paris = Geographic::new(48.8566, 2.3522, 0.0);
        let dist = haversine_distance(&london, &paris);
        // Known: ~343 km
        assert!((dist - 343_000.0).abs() < 5000.0);
    }

    #[test]
    fn test_haversine_same_point() {
        let p = Geographic::new(40.0, -74.0, 0.0);
        assert!(haversine_distance(&p, &p) < 0.001);
    }

    #[test]
    fn test_bearing_north() {
        let a = Geographic::new(0.0, 0.0, 0.0);
        let b = Geographic::new(1.0, 0.0, 0.0); // due north
        let brng = bearing(&a, &b);
        assert!((brng - 0.0).abs() < 0.1);
    }

    #[test]
    fn test_bearing_east() {
        let a = Geographic::new(0.0, 0.0, 0.0);
        let b = Geographic::new(0.0, 1.0, 0.0); // due east
        let brng = bearing(&a, &b);
        assert!((brng - 90.0).abs() < 0.1);
    }

    #[test]
    fn test_destination_roundtrip() {
        let start = Geographic::new(48.8566, 2.3522, 0.0);
        let brng = 45.0;
        let dist = 10000.0; // 10 km
        let end = destination(&start, brng, dist);
        let actual_dist = haversine_distance(&start, &end);
        assert!((actual_dist - dist).abs() < 1.0);
    }

    #[test]
    fn test_enu_bearing_and_distance() {
        let enu = Enu::new(100.0, 0.0, 0.0); // due east
        assert!((enu.bearing() - 90.0).abs() < 0.1);
        assert!((enu.horizontal_distance() - 100.0).abs() < 0.01);
    }
}
