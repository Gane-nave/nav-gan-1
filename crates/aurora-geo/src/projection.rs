//! Map projections — convert between geographic coordinates and planar map coordinates.

use std::f64::consts::PI;

/// UTM zone information.
#[derive(Debug, Clone, Copy)]
pub struct UtmZone {
    /// Zone number (1-60).
    pub number: u8,
    /// Whether the zone is in the northern hemisphere.
    pub northern: bool,
}

impl UtmZone {
    /// Determine UTM zone from geographic coordinates.
    pub fn from_geo(lat: f64, lon: f64) -> Self {
        let number = ((lon + 180.0) / 6.0).floor() as u8 + 1;
        Self {
            number,
            northern: lat >= 0.0,
        }
    }

    /// Central meridian of this zone in degrees.
    pub fn central_meridian(&self) -> f64 {
        (self.number as f64 - 1.0) * 6.0 - 180.0 + 3.0
    }
}

/// Projected coordinates (easting, northing in meters).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Projected {
    /// Easting in meters.
    pub easting: f64,
    /// Northing in meters.
    pub northing: f64,
}

impl Projected {
    /// Create new projected coordinates.
    pub fn new(easting: f64, northing: f64) -> Self {
        Self { easting, northing }
    }

    /// Distance to another projected point in meters.
    pub fn distance_to(&self, other: &Projected) -> f64 {
        let de = self.easting - other.easting;
        let dn = self.northing - other.northing;
        (de * de + dn * dn).sqrt()
    }
}

/// WGS-84 constants.
const A: f64 = 6_378_137.0;
const F: f64 = 1.0 / 298.257_223_563;
const E2: f64 = 2.0 * F - F * F;
const E_PRIME2: f64 = E2 / (1.0 - E2);

/// UTM scale factor.
const K0: f64 = 0.9996;

/// Convert geographic to UTM projection.
pub fn geo_to_utm(lat: f64, lon: f64) -> (Projected, UtmZone) {
    let zone = UtmZone::from_geo(lat, lon);
    let cm = zone.central_meridian();

    let lat_r = lat.to_radians();
    let lon_r = (lon - cm).to_radians();

    let n = A / (1.0 - E2 * lat_r.sin().powi(2)).sqrt();
    let t = lat_r.tan().powi(2);
    let c = E_PRIME2 * lat_r.cos().powi(2);
    let a_val = lon_r * lat_r.cos();

    let m = A
        * ((1.0 - E2 / 4.0 - 3.0 * E2 * E2 / 64.0 - 5.0 * E2.powi(3) / 256.0) * lat_r
            - (3.0 * E2 / 8.0 + 3.0 * E2 * E2 / 32.0 + 45.0 * E2.powi(3) / 1024.0)
                * (2.0 * lat_r).sin()
            + (15.0 * E2 * E2 / 256.0 + 45.0 * E2.powi(3) / 1024.0) * (4.0 * lat_r).sin()
            - (35.0 * E2.powi(3) / 3072.0) * (6.0 * lat_r).sin());

    let easting = K0
        * n
        * (a_val
            + (1.0 - t + c) * a_val.powi(3) / 6.0
            + (5.0 - 18.0 * t + t * t + 72.0 * c - 58.0 * E_PRIME2) * a_val.powi(5) / 120.0)
        + 500_000.0;

    let northing_raw = K0
        * (m + n
            * lat_r.tan()
            * (a_val * a_val / 2.0
                + (5.0 - t + 9.0 * c + 4.0 * c * c) * a_val.powi(4) / 24.0
                + (61.0 - 58.0 * t + t * t + 600.0 * c - 330.0 * E_PRIME2) * a_val.powi(6)
                    / 720.0));

    let northing = if zone.northern {
        northing_raw
    } else {
        northing_raw + 10_000_000.0
    };

    (Projected::new(easting, northing), zone)
}

/// Mercator projection — convert lat/lon to x/y in meters.
/// Uses Web Mercator (EPSG:3857) projection.
pub fn geo_to_mercator(lat: f64, lon: f64) -> Projected {
    let x = A * lon.to_radians();
    let lat_r = lat.to_radians();
    let y = A * ((PI / 4.0 + lat_r / 2.0).tan()).ln();
    Projected::new(x, y)
}

/// Inverse Mercator — convert x/y back to lat/lon.
pub fn mercator_to_geo(proj: &Projected) -> (f64, f64) {
    let lon = (proj.easting / A).to_degrees();
    let lat = (2.0 * (proj.northing / A).exp().atan() - PI / 2.0).to_degrees();
    (lat, lon)
}

/// Simple equirectangular projection (for small areas / quick estimates).
pub fn geo_to_equirect(lat: f64, lon: f64, ref_lat: f64) -> Projected {
    let x = (lon.to_radians()) * ref_lat.to_radians().cos() * A;
    let y = lat.to_radians() * A;
    Projected::new(x, y)
}

/// Meters per degree of latitude at a given latitude.
pub fn meters_per_degree_lat(lat: f64) -> f64 {
    let lat_r = lat.to_radians();
    111_132.92 - 559.82 * (2.0 * lat_r).cos() + 1.175 * (4.0 * lat_r).cos()
        - 0.0023 * (6.0 * lat_r).cos()
}

/// Meters per degree of longitude at a given latitude.
pub fn meters_per_degree_lon(lat: f64) -> f64 {
    let lat_r = lat.to_radians();
    111_412.84 * lat_r.cos() - 93.5 * (3.0 * lat_r).cos() + 0.118 * (5.0 * lat_r).cos()
}

/// Scale factor at a given latitude for Mercator projection.
pub fn mercator_scale_factor(lat: f64) -> f64 {
    1.0 / lat.to_radians().cos()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utm_zone_detection() {
        let zone = UtmZone::from_geo(48.8566, 2.3522); // Paris
        assert_eq!(zone.number, 31);
        assert!(zone.northern);

        let zone = UtmZone::from_geo(-33.8688, 151.2093); // Sydney
        assert_eq!(zone.number, 56);
        assert!(!zone.northern);
    }

    #[test]
    fn test_central_meridian() {
        let zone = UtmZone {
            number: 31,
            northern: true,
        };
        assert!((zone.central_meridian() - 3.0).abs() < 0.01);
    }

    #[test]
    fn test_geo_to_utm() {
        let (proj, zone) = geo_to_utm(48.8566, 2.3522); // Paris
        assert_eq!(zone.number, 31);
        // UTM easting should be near 500000 (close to central meridian)
        assert!(proj.easting > 400_000.0 && proj.easting < 600_000.0);
        assert!(proj.northing > 5_000_000.0); // northern hemisphere
    }

    #[test]
    fn test_mercator_roundtrip() {
        let lat = 48.8566;
        let lon = 2.3522;
        let proj = geo_to_mercator(lat, lon);
        let (lat2, lon2) = mercator_to_geo(&proj);
        assert!((lat - lat2).abs() < 1e-6);
        assert!((lon - lon2).abs() < 1e-6);
    }

    #[test]
    fn test_mercator_equator() {
        let proj = geo_to_mercator(0.0, 0.0);
        assert!(proj.easting.abs() < 1.0);
        assert!(proj.northing.abs() < 1.0);
    }

    #[test]
    fn test_projected_distance() {
        let a = Projected::new(500_000.0, 5_000_000.0);
        let b = Projected::new(500_300.0, 5_000_400.0);
        let dist = a.distance_to(&b);
        assert!((dist - 500.0).abs() < 0.01); // 300^2 + 400^2 = 500^2
    }

    #[test]
    fn test_meters_per_degree() {
        let lat = 45.0;
        let m_lat = meters_per_degree_lat(lat);
        let m_lon = meters_per_degree_lon(lat);
        // At 45°, ~111km per degree lat, ~78km per degree lon
        assert!(m_lat > 110_000.0 && m_lat < 112_000.0);
        assert!(m_lon > 77_000.0 && m_lon < 80_000.0);
    }

    #[test]
    fn test_mercator_scale_factor() {
        let scale = mercator_scale_factor(0.0);
        assert!((scale - 1.0).abs() < 1e-10); // no distortion at equator

        let scale = mercator_scale_factor(60.0);
        assert!((scale - 2.0).abs() < 0.01); // 2x at 60°
    }

    #[test]
    fn test_equirect_projection() {
        let proj = geo_to_equirect(0.0, 0.0, 0.0);
        assert!(proj.easting.abs() < 1.0);
        assert!(proj.northing.abs() < 1.0);
    }

    #[test]
    fn test_utm_southern_hemisphere() {
        let (proj, zone) = geo_to_utm(-33.8688, 151.2093); // Sydney
        assert!(!zone.northern);
        assert!(proj.northing > 6_000_000.0); // offset for southern hemisphere
    }
}
