//! OSM tag classification — highway classes, speeds, and vehicle restrictions.

use gane_core::map::RoadClass;
use std::collections::HashMap;

/// Map an OSM `highway=*` value to a G.A.N.E road class.
/// Returns `None` for non-routable values (construction, proposed, bus stops…).
pub fn road_class(highway: &str) -> Option<RoadClass> {
    match highway {
        "motorway" | "motorway_link" => Some(RoadClass::Motorway),
        "trunk" | "trunk_link" => Some(RoadClass::Trunk),
        "primary" | "primary_link" => Some(RoadClass::Primary),
        "secondary" | "secondary_link" => Some(RoadClass::Secondary),
        "tertiary" | "tertiary_link" => Some(RoadClass::Tertiary),
        "residential" | "living_street" => Some(RoadClass::Residential),
        "service" => Some(RoadClass::Service),
        "unclassified" | "road" => Some(RoadClass::Unclassified),
        "pedestrian" | "footway" | "path" | "steps" => Some(RoadClass::Pedestrian),
        "cycleway" => Some(RoadClass::Cycleway),
        "track" => Some(RoadClass::Track),
        _ => None,
    }
}

/// Default speed (km/h) when the way carries no `maxspeed` tag.
pub fn default_speed_kmh(class: RoadClass) -> f64 {
    match class {
        RoadClass::Motorway => 110.0,
        RoadClass::Trunk => 90.0,
        RoadClass::Primary => 80.0,
        RoadClass::Secondary => 70.0,
        RoadClass::Tertiary => 50.0,
        RoadClass::Residential => 30.0,
        RoadClass::Service => 20.0,
        RoadClass::Unclassified => 40.0,
        RoadClass::Pedestrian => 5.0,
        RoadClass::Cycleway => 15.0,
        RoadClass::Track => 20.0,
    }
}

/// Parse `maxspeed` values: `"50"`, `"50 km/h"`, `"30 mph"`.
pub fn parse_maxspeed_kmh(value: &str) -> Option<f64> {
    let v = value.trim().to_lowercase();
    if let Some(mph) = v.strip_suffix("mph") {
        return mph.trim().parse::<f64>().ok().map(|x| x * 1.609_344);
    }
    let num = v.strip_suffix("km/h").unwrap_or(&v).trim();
    num.parse::<f64>().ok()
}

/// Parse metric dimensions: `"4.2"`, `"4.2 m"` (feet-inch forms are rejected).
pub fn parse_length_m(value: &str) -> Option<f64> {
    let v = value.trim().to_lowercase();
    if v.contains('\'') || v.contains('"') {
        return None; // imperial not supported yet — safer to skip than mis-parse
    }
    let num = v.strip_suffix('m').unwrap_or(&v).trim();
    num.parse::<f64>().ok()
}

/// Parse weights into kilograms. Bare numbers are tonnes per OSM convention:
/// `"3.5"` → 3 500 kg, `"3.5 t"` → 3 500 kg, `"3500 kg"` → 3 500 kg.
pub fn parse_weight_kg(value: &str) -> Option<f64> {
    let v = value.trim().to_lowercase();
    if let Some(kg) = v.strip_suffix("kg") {
        return kg.trim().parse::<f64>().ok();
    }
    let num = v.strip_suffix('t').unwrap_or(&v).trim();
    num.parse::<f64>().ok().map(|x| x * 1000.0)
}

/// One-way handling: `yes`/`true`/`1` forward, `-1` reversed (caller flips refs).
pub enum OneWay {
    No,
    Forward,
    Reverse,
}

pub fn parse_oneway(tags: &HashMap<String, String>, class: RoadClass) -> OneWay {
    match tags.get("oneway").map(String::as_str) {
        Some("yes") | Some("true") | Some("1") => OneWay::Forward,
        Some("-1") => OneWay::Reverse,
        Some(_) => OneWay::No,
        // Motorways are implicitly one-way in OSM.
        None if class == RoadClass::Motorway => OneWay::Forward,
        None => OneWay::No,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_highways() {
        assert_eq!(road_class("motorway"), Some(RoadClass::Motorway));
        assert_eq!(road_class("residential"), Some(RoadClass::Residential));
        assert_eq!(road_class("construction"), None);
    }

    #[test]
    fn parses_maxspeed_variants() {
        assert_eq!(parse_maxspeed_kmh("50"), Some(50.0));
        assert_eq!(parse_maxspeed_kmh("50 km/h"), Some(50.0));
        let mph = parse_maxspeed_kmh("30 mph").unwrap();
        assert!((mph - 48.28).abs() < 0.1);
    }

    #[test]
    fn parses_dimensions_and_weights() {
        assert_eq!(parse_length_m("4.2"), Some(4.2));
        assert_eq!(parse_length_m("4.2 m"), Some(4.2));
        assert_eq!(parse_length_m("13'6\""), None);
        assert_eq!(parse_weight_kg("3.5"), Some(3500.0));
        assert_eq!(parse_weight_kg("3.5 t"), Some(3500.0));
        assert_eq!(parse_weight_kg("3500 kg"), Some(3500.0));
    }
}
