//! Marketplace categories — hierarchical categorisation of plugins
//! and extensions for discovery and browsing.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Category taxonomy
// ---------------------------------------------------------------------------

/// Top-level marketplace category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Category {
    /// Navigation and routing.
    Navigation,
    /// Traffic and road conditions.
    Traffic,
    /// Mapping and cartography.
    Mapping,
    /// Fleet management.
    Fleet,
    /// Safety and integrity.
    Safety,
    /// Parking and charging.
    ParkingCharging,
    /// Vehicle integrations.
    Vehicle,
    /// Weather and environment.
    Weather,
    /// Points of interest.
    PointsOfInterest,
    /// Analytics and reporting.
    Analytics,
    /// Social and community.
    Social,
    /// Accessibility.
    Accessibility,
    /// Developer tools.
    DeveloperTools,
    /// UI themes and customisation.
    Themes,
    /// Other / miscellaneous.
    Other,
}

impl Category {
    /// Human-readable display name.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Navigation => "Navigation & Routing",
            Self::Traffic => "Traffic & Road Conditions",
            Self::Mapping => "Mapping & Cartography",
            Self::Fleet => "Fleet Management",
            Self::Safety => "Safety & Integrity",
            Self::ParkingCharging => "Parking & Charging",
            Self::Vehicle => "Vehicle Integrations",
            Self::Weather => "Weather & Environment",
            Self::PointsOfInterest => "Points of Interest",
            Self::Analytics => "Analytics & Reporting",
            Self::Social => "Social & Community",
            Self::Accessibility => "Accessibility",
            Self::DeveloperTools => "Developer Tools",
            Self::Themes => "UI Themes & Customisation",
            Self::Other => "Other",
        }
    }

    /// All categories (for iteration / UI rendering).
    pub fn all() -> &'static [Category] {
        &[
            Self::Navigation,
            Self::Traffic,
            Self::Mapping,
            Self::Fleet,
            Self::Safety,
            Self::ParkingCharging,
            Self::Vehicle,
            Self::Weather,
            Self::PointsOfInterest,
            Self::Analytics,
            Self::Social,
            Self::Accessibility,
            Self::DeveloperTools,
            Self::Themes,
            Self::Other,
        ]
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_categories_count() {
        assert_eq!(Category::all().len(), 15);
    }

    #[test]
    fn display_names() {
        assert_eq!(Category::Navigation.display_name(), "Navigation & Routing");
        assert_eq!(Category::Other.display_name(), "Other");
    }

    #[test]
    fn category_equality() {
        assert_eq!(Category::Traffic, Category::Traffic);
        assert_ne!(Category::Traffic, Category::Safety);
    }

    #[test]
    fn serialization_roundtrip() {
        let cat = Category::Fleet;
        let json = serde_json::to_string(&cat).unwrap();
        let back: Category = serde_json::from_str(&json).unwrap();
        assert_eq!(cat, back);
    }
}
