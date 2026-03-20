//! System introspection — provides runtime information about the system's internal state.

use std::collections::HashMap;
use std::time::Instant;

/// Component type in the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentType {
    /// Core processing component.
    Core,
    /// Sensor input component.
    Sensor,
    /// Data processing pipeline.
    Pipeline,
    /// Storage component.
    Storage,
    /// Network / communication.
    Network,
    /// User interface.
    Interface,
    /// External integration.
    Integration,
}

/// Runtime information about a component.
#[derive(Debug, Clone)]
pub struct ComponentInfo {
    /// Component name.
    pub name: String,
    /// Component type.
    pub component_type: ComponentType,
    /// Version string.
    pub version: String,
    /// Whether the component is currently active.
    pub active: bool,
    /// Uptime since last restart.
    pub started_at: Instant,
    /// Memory usage estimate (bytes).
    pub memory_bytes: u64,
    /// Current load (0.0 = idle, 1.0 = fully loaded).
    pub load: f64,
    /// Custom properties.
    pub properties: HashMap<String, String>,
}

/// A dependency between two components.
#[derive(Debug, Clone)]
pub struct Dependency {
    /// Source component.
    pub from: String,
    /// Target component (dependency).
    pub to: String,
    /// Whether this is a required dependency.
    pub required: bool,
}

/// System introspector — provides a view of the system's internal structure.
pub struct SystemIntrospector {
    components: HashMap<String, ComponentInfo>,
    dependencies: Vec<Dependency>,
}

impl SystemIntrospector {
    /// Create a new introspector.
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            dependencies: Vec::new(),
        }
    }

    /// Register a component.
    pub fn register_component(&mut self, info: ComponentInfo) {
        self.components.insert(info.name.clone(), info);
    }

    /// Add a dependency between components.
    pub fn add_dependency(&mut self, from: &str, to: &str, required: bool) {
        self.dependencies.push(Dependency {
            from: from.to_string(),
            to: to.to_string(),
            required,
        });
    }

    /// Get component count.
    pub fn component_count(&self) -> usize {
        self.components.len()
    }

    /// Get a component by name.
    pub fn get_component(&self, name: &str) -> Option<&ComponentInfo> {
        self.components.get(name)
    }

    /// Get all components of a given type.
    pub fn by_type(&self, component_type: ComponentType) -> Vec<&ComponentInfo> {
        self.components
            .values()
            .filter(|c| c.component_type == component_type)
            .collect()
    }

    /// Get all active components.
    pub fn active_components(&self) -> Vec<&ComponentInfo> {
        self.components.values().filter(|c| c.active).collect()
    }

    /// Get all inactive components.
    pub fn inactive_components(&self) -> Vec<&ComponentInfo> {
        self.components.values().filter(|c| !c.active).collect()
    }

    /// Get total memory usage.
    pub fn total_memory_bytes(&self) -> u64 {
        self.components.values().map(|c| c.memory_bytes).sum()
    }

    /// Get average load across all active components.
    pub fn average_load(&self) -> f64 {
        let active: Vec<_> = self.active_components();
        if active.is_empty() {
            return 0.0;
        }
        let total: f64 = active.iter().map(|c| c.load).sum();
        total / active.len() as f64
    }

    /// Get dependencies of a component.
    pub fn dependencies_of(&self, component: &str) -> Vec<&Dependency> {
        self.dependencies
            .iter()
            .filter(|d| d.from == component)
            .collect()
    }

    /// Get dependents of a component (what depends on it).
    pub fn dependents_of(&self, component: &str) -> Vec<&Dependency> {
        self.dependencies
            .iter()
            .filter(|d| d.to == component)
            .collect()
    }

    /// Check if all required dependencies of a component are active.
    pub fn all_required_deps_active(&self, component: &str) -> bool {
        let deps = self.dependencies_of(component);
        for dep in deps {
            if dep.required {
                match self.components.get(&dep.to) {
                    Some(c) if c.active => {}
                    _ => return false,
                }
            }
        }
        true
    }

    /// Get the dependency count.
    pub fn dependency_count(&self) -> usize {
        self.dependencies.len()
    }

    /// Update a component's load.
    pub fn update_load(&mut self, name: &str, load: f64) -> bool {
        if let Some(c) = self.components.get_mut(name) {
            c.load = load.clamp(0.0, 1.0);
            true
        } else {
            false
        }
    }

    /// Set component active status.
    pub fn set_active(&mut self, name: &str, active: bool) -> bool {
        if let Some(c) = self.components.get_mut(name) {
            c.active = active;
            true
        } else {
            false
        }
    }
}

impl Default for SystemIntrospector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_component(name: &str, ctype: ComponentType, active: bool) -> ComponentInfo {
        ComponentInfo {
            name: name.to_string(),
            component_type: ctype,
            version: "1.0.0".to_string(),
            active,
            started_at: Instant::now(),
            memory_bytes: 1024 * 1024, // 1 MB
            load: 0.5,
            properties: HashMap::new(),
        }
    }

    #[test]
    fn test_register_component() {
        let mut intro = SystemIntrospector::new();
        intro.register_component(make_component("gnss", ComponentType::Sensor, true));
        assert_eq!(intro.component_count(), 1);
    }

    #[test]
    fn test_get_component() {
        let mut intro = SystemIntrospector::new();
        intro.register_component(make_component("gnss", ComponentType::Sensor, true));
        assert!(intro.get_component("gnss").is_some());
        assert!(intro.get_component("nonexistent").is_none());
    }

    #[test]
    fn test_by_type() {
        let mut intro = SystemIntrospector::new();
        intro.register_component(make_component("gnss", ComponentType::Sensor, true));
        intro.register_component(make_component("imu", ComponentType::Sensor, true));
        intro.register_component(make_component("fusion", ComponentType::Pipeline, true));

        let sensors = intro.by_type(ComponentType::Sensor);
        assert_eq!(sensors.len(), 2);
        let pipelines = intro.by_type(ComponentType::Pipeline);
        assert_eq!(pipelines.len(), 1);
    }

    #[test]
    fn test_active_inactive() {
        let mut intro = SystemIntrospector::new();
        intro.register_component(make_component("gnss", ComponentType::Sensor, true));
        intro.register_component(make_component("lidar", ComponentType::Sensor, false));

        assert_eq!(intro.active_components().len(), 1);
        assert_eq!(intro.inactive_components().len(), 1);
    }

    #[test]
    fn test_total_memory() {
        let mut intro = SystemIntrospector::new();
        intro.register_component(make_component("gnss", ComponentType::Sensor, true));
        intro.register_component(make_component("imu", ComponentType::Sensor, true));
        assert_eq!(intro.total_memory_bytes(), 2 * 1024 * 1024);
    }

    #[test]
    fn test_average_load() {
        let mut intro = SystemIntrospector::new();
        let mut c1 = make_component("gnss", ComponentType::Sensor, true);
        c1.load = 0.8;
        let mut c2 = make_component("imu", ComponentType::Sensor, true);
        c2.load = 0.4;
        intro.register_component(c1);
        intro.register_component(c2);
        assert!((intro.average_load() - 0.6).abs() < 0.01);
    }

    #[test]
    fn test_average_load_empty() {
        let intro = SystemIntrospector::new();
        assert_eq!(intro.average_load(), 0.0);
    }

    #[test]
    fn test_dependencies() {
        let mut intro = SystemIntrospector::new();
        intro.register_component(make_component("fusion", ComponentType::Pipeline, true));
        intro.register_component(make_component("gnss", ComponentType::Sensor, true));
        intro.add_dependency("fusion", "gnss", true);

        assert_eq!(intro.dependency_count(), 1);
        assert_eq!(intro.dependencies_of("fusion").len(), 1);
        assert_eq!(intro.dependents_of("gnss").len(), 1);
    }

    #[test]
    fn test_required_deps_active() {
        let mut intro = SystemIntrospector::new();
        intro.register_component(make_component("fusion", ComponentType::Pipeline, true));
        intro.register_component(make_component("gnss", ComponentType::Sensor, true));
        intro.add_dependency("fusion", "gnss", true);

        assert!(intro.all_required_deps_active("fusion"));
    }

    #[test]
    fn test_required_deps_inactive() {
        let mut intro = SystemIntrospector::new();
        intro.register_component(make_component("fusion", ComponentType::Pipeline, true));
        intro.register_component(make_component("gnss", ComponentType::Sensor, false));
        intro.add_dependency("fusion", "gnss", true);

        assert!(!intro.all_required_deps_active("fusion"));
    }

    #[test]
    fn test_optional_deps_inactive_ok() {
        let mut intro = SystemIntrospector::new();
        intro.register_component(make_component("fusion", ComponentType::Pipeline, true));
        intro.register_component(make_component("lidar", ComponentType::Sensor, false));
        intro.add_dependency("fusion", "lidar", false); // Optional

        assert!(intro.all_required_deps_active("fusion"));
    }

    #[test]
    fn test_update_load() {
        let mut intro = SystemIntrospector::new();
        intro.register_component(make_component("gnss", ComponentType::Sensor, true));
        assert!(intro.update_load("gnss", 0.9));
        assert!((intro.get_component("gnss").unwrap().load - 0.9).abs() < 0.01);
        assert!(!intro.update_load("nonexistent", 0.5));
    }

    #[test]
    fn test_update_load_clamped() {
        let mut intro = SystemIntrospector::new();
        intro.register_component(make_component("gnss", ComponentType::Sensor, true));
        intro.update_load("gnss", 1.5); // Above max
        assert!((intro.get_component("gnss").unwrap().load - 1.0).abs() < 0.01);
        intro.update_load("gnss", -0.5); // Below min
        assert!((intro.get_component("gnss").unwrap().load - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_set_active() {
        let mut intro = SystemIntrospector::new();
        intro.register_component(make_component("gnss", ComponentType::Sensor, true));
        assert!(intro.set_active("gnss", false));
        assert!(!intro.get_component("gnss").unwrap().active);
        assert!(!intro.set_active("nonexistent", true));
    }
}
