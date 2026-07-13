//! User profile management.
//!
//! Stores user identity, vehicle associations, saved locations,
//! and profile sharing/sync capabilities.

use std::collections::HashMap;

/// User profile.
#[derive(Debug, Clone)]
pub struct UserProfile {
    pub id: String,
    pub display_name: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub home: Option<SavedLocation>,
    pub work: Option<SavedLocation>,
    pub saved_locations: Vec<SavedLocation>,
    pub vehicles: Vec<VehicleProfile>,
    pub active_vehicle_idx: Option<usize>,
    pub metadata: HashMap<String, String>,
    pub created_at_epoch_s: u64,
}

impl UserProfile {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            display_name: name.into(),
            email: None,
            avatar_url: None,
            home: None,
            work: None,
            saved_locations: Vec::new(),
            vehicles: Vec::new(),
            active_vehicle_idx: None,
            metadata: HashMap::new(),
            created_at_epoch_s: 0,
        }
    }

    /// Add a saved location.
    pub fn add_location(&mut self, loc: SavedLocation) {
        // Prevent duplicates by label
        if !self.saved_locations.iter().any(|l| l.label == loc.label) {
            self.saved_locations.push(loc);
        }
    }

    /// Remove a saved location by label.
    pub fn remove_location(&mut self, label: &str) -> bool {
        let before = self.saved_locations.len();
        self.saved_locations.retain(|l| l.label != label);
        self.saved_locations.len() < before
    }

    /// Get the active vehicle profile.
    pub fn active_vehicle(&self) -> Option<&VehicleProfile> {
        self.active_vehicle_idx.and_then(|i| self.vehicles.get(i))
    }

    /// Set the active vehicle by index.
    pub fn set_active_vehicle(&mut self, idx: usize) -> bool {
        if idx < self.vehicles.len() {
            self.active_vehicle_idx = Some(idx);
            true
        } else {
            false
        }
    }

    /// Add a vehicle and set it as active.
    pub fn add_vehicle(&mut self, vehicle: VehicleProfile) {
        self.vehicles.push(vehicle);
        self.active_vehicle_idx = Some(self.vehicles.len() - 1);
    }

    /// Total saved locations including home/work.
    pub fn total_saved_locations(&self) -> usize {
        let mut count = self.saved_locations.len();
        if self.home.is_some() {
            count += 1;
        }
        if self.work.is_some() {
            count += 1;
        }
        count
    }
}

/// A saved/favourite location.
#[derive(Debug, Clone)]
pub struct SavedLocation {
    pub label: String,
    pub lat: f64,
    pub lon: f64,
    pub address: Option<String>,
    pub icon: Option<String>,
}

/// Vehicle profile for route optimisation.
#[derive(Debug, Clone)]
pub struct VehicleProfile {
    pub name: String,
    pub vehicle_type: VehicleType,
    pub fuel_type: FuelType,
    /// Height in metres (for bridge/tunnel clearance).
    pub height_m: Option<f64>,
    /// Weight in kg (for weight-restricted roads).
    pub weight_kg: Option<f64>,
    /// Length in metres (for parking).
    pub length_m: Option<f64>,
    /// Whether the vehicle has a toll transponder.
    pub has_toll_pass: bool,
}

/// Vehicle type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VehicleType {
    Car,
    SUV,
    Motorcycle,
    Van,
    Truck,
    Bus,
}

impl VehicleType {
    /// Whether the vehicle may have height/weight restrictions.
    pub fn has_restrictions(self) -> bool {
        matches!(self, Self::Van | Self::Truck | Self::Bus)
    }
}

/// Fuel/energy type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FuelType {
    Petrol,
    Diesel,
    Electric,
    Hybrid,
    Hydrogen,
    LPG,
}

impl FuelType {
    /// Whether the vehicle needs charging stations.
    pub fn needs_charging(self) -> bool {
        matches!(self, Self::Electric)
    }

    /// Whether the vehicle benefits from hybrid-optimised routes.
    pub fn is_eco(self) -> bool {
        matches!(self, Self::Electric | Self::Hybrid | Self::Hydrogen)
    }
}

/// Profile store for managing multiple user profiles.
#[derive(Debug)]
pub struct ProfileStore {
    profiles: HashMap<String, UserProfile>,
    active_id: Option<String>,
}

impl ProfileStore {
    pub fn new() -> Self {
        Self {
            profiles: HashMap::new(),
            active_id: None,
        }
    }

    /// Add or update a profile.
    pub fn upsert(&mut self, profile: UserProfile) {
        let id = profile.id.clone();
        self.profiles.insert(id.clone(), profile);
        if self.active_id.is_none() {
            self.active_id = Some(id);
        }
    }

    /// Get the active profile.
    pub fn active(&self) -> Option<&UserProfile> {
        self.active_id.as_ref().and_then(|id| self.profiles.get(id))
    }

    /// Switch active profile.
    pub fn set_active(&mut self, id: &str) -> bool {
        if self.profiles.contains_key(id) {
            self.active_id = Some(id.to_string());
            true
        } else {
            false
        }
    }

    /// Remove a profile.
    pub fn remove(&mut self, id: &str) -> bool {
        let removed = self.profiles.remove(id).is_some();
        if removed && self.active_id.as_deref() == Some(id) {
            self.active_id = self.profiles.keys().next().cloned();
        }
        removed
    }

    pub fn count(&self) -> usize {
        self.profiles.len()
    }

    /// List all profile IDs.
    pub fn list_ids(&self) -> Vec<&str> {
        self.profiles.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for ProfileStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_profile(id: &str) -> UserProfile {
        UserProfile::new(id, format!("User {id}"))
    }

    #[test]
    fn test_user_profile_creation() {
        let p = UserProfile::new("u1", "Alice");
        assert_eq!(p.id, "u1");
        assert_eq!(p.display_name, "Alice");
        assert!(p.vehicles.is_empty());
    }

    #[test]
    fn test_add_saved_location() {
        let mut p = test_profile("u1");
        let loc = SavedLocation {
            label: "Gym".into(),
            lat: 32.0,
            lon: 34.0,
            address: None,
            icon: None,
        };
        p.add_location(loc.clone());
        assert_eq!(p.saved_locations.len(), 1);
        // Duplicate label → not added
        p.add_location(loc);
        assert_eq!(p.saved_locations.len(), 1);
    }

    #[test]
    fn test_remove_location() {
        let mut p = test_profile("u1");
        p.add_location(SavedLocation {
            label: "Gym".into(),
            lat: 0.0,
            lon: 0.0,
            address: None,
            icon: None,
        });
        assert!(p.remove_location("Gym"));
        assert!(!p.remove_location("Gym")); // already removed
    }

    #[test]
    fn test_vehicle_management() {
        let mut p = test_profile("u1");
        p.add_vehicle(VehicleProfile {
            name: "My Car".into(),
            vehicle_type: VehicleType::Car,
            fuel_type: FuelType::Electric,
            height_m: None,
            weight_kg: None,
            length_m: None,
            has_toll_pass: true,
        });
        assert_eq!(p.active_vehicle_idx, Some(0));
        assert_eq!(p.active_vehicle().unwrap().name, "My Car");
    }

    #[test]
    fn test_set_active_vehicle_bounds() {
        let mut p = test_profile("u1");
        assert!(!p.set_active_vehicle(0)); // no vehicles
        p.add_vehicle(VehicleProfile {
            name: "V1".into(),
            vehicle_type: VehicleType::Car,
            fuel_type: FuelType::Petrol,
            height_m: None,
            weight_kg: None,
            length_m: None,
            has_toll_pass: false,
        });
        assert!(!p.set_active_vehicle(5)); // out of bounds
        assert!(p.set_active_vehicle(0));
    }

    #[test]
    fn test_total_saved_locations() {
        let mut p = test_profile("u1");
        p.home = Some(SavedLocation {
            label: "Home".into(),
            lat: 0.0,
            lon: 0.0,
            address: None,
            icon: None,
        });
        p.add_location(SavedLocation {
            label: "X".into(),
            lat: 0.0,
            lon: 0.0,
            address: None,
            icon: None,
        });
        assert_eq!(p.total_saved_locations(), 2); // home + 1
    }

    #[test]
    fn test_vehicle_type_restrictions() {
        assert!(VehicleType::Truck.has_restrictions());
        assert!(!VehicleType::Car.has_restrictions());
    }

    #[test]
    fn test_fuel_type() {
        assert!(FuelType::Electric.needs_charging());
        assert!(!FuelType::Petrol.needs_charging());
        assert!(FuelType::Hybrid.is_eco());
    }

    #[test]
    fn test_profile_store() {
        let mut store = ProfileStore::new();
        store.upsert(test_profile("a"));
        store.upsert(test_profile("b"));
        assert_eq!(store.count(), 2);
        assert_eq!(store.active().unwrap().id, "a"); // first added is active
    }

    #[test]
    fn test_profile_store_switch() {
        let mut store = ProfileStore::new();
        store.upsert(test_profile("a"));
        store.upsert(test_profile("b"));
        assert!(store.set_active("b"));
        assert_eq!(store.active().unwrap().id, "b");
        assert!(!store.set_active("nonexistent"));
    }

    #[test]
    fn test_profile_store_remove() {
        let mut store = ProfileStore::new();
        store.upsert(test_profile("a"));
        store.upsert(test_profile("b"));
        store.set_active("a");
        assert!(store.remove("a"));
        assert_eq!(store.count(), 1);
        // Active should auto-switch
        assert!(store.active().is_some());
    }
}
