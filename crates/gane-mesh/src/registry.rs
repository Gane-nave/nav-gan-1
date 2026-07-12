//! Service registry — dynamic service registration, discovery, and metadata.

use std::collections::HashMap;

/// Service health status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceStatus {
    /// Service is healthy and accepting traffic.
    Healthy,
    /// Service is degraded but partially functional.
    Degraded,
    /// Service is unhealthy and should not receive traffic.
    Unhealthy,
    /// Service is draining (shutting down gracefully).
    Draining,
    /// Service is unknown/unreachable.
    Unknown,
}

/// Service instance metadata.
#[derive(Debug, Clone)]
pub struct ServiceInstance {
    /// Instance identifier.
    pub id: String,
    /// Service name this instance belongs to.
    pub service_name: String,
    /// Host address.
    pub host: String,
    /// Port number.
    pub port: u16,
    /// Instance weight for load balancing (higher = more traffic).
    pub weight: u32,
    /// Current health status.
    pub status: ServiceStatus,
    /// Custom metadata tags.
    pub tags: HashMap<String, String>,
    /// Registration timestamp (epoch millis).
    pub registered_at_ms: u64,
    /// Last heartbeat timestamp (epoch millis).
    pub last_heartbeat_ms: u64,
}

impl ServiceInstance {
    /// Get the full address (host:port).
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// Whether this instance can receive traffic.
    pub fn is_routable(&self) -> bool {
        matches!(
            self.status,
            ServiceStatus::Healthy | ServiceStatus::Degraded
        )
    }
}

/// Service registry — manages service instances.
pub struct ServiceRegistry {
    /// service_name → Vec of [`ServiceInstance`]
    services: HashMap<String, Vec<ServiceInstance>>,
    /// Heartbeat timeout in milliseconds.
    heartbeat_timeout_ms: u64,
}

impl ServiceRegistry {
    /// Create a new service registry.
    pub fn new(heartbeat_timeout_ms: u64) -> Self {
        Self {
            services: HashMap::new(),
            heartbeat_timeout_ms,
        }
    }

    /// Register a service instance.
    pub fn register(&mut self, instance: ServiceInstance) -> Result<(), String> {
        let instances = self
            .services
            .entry(instance.service_name.clone())
            .or_default();

        // Check for duplicate instance ID within the same service
        if instances.iter().any(|i| i.id == instance.id) {
            return Err(format!(
                "Instance '{}' already registered for service '{}'",
                instance.id, instance.service_name
            ));
        }

        instances.push(instance);
        Ok(())
    }

    /// Deregister a service instance.
    pub fn deregister(&mut self, service_name: &str, instance_id: &str) -> Result<(), String> {
        let instances = self
            .services
            .get_mut(service_name)
            .ok_or_else(|| format!("Service '{service_name}' not found"))?;

        let before = instances.len();
        instances.retain(|i| i.id != instance_id);

        if instances.len() == before {
            return Err(format!(
                "Instance '{instance_id}' not found in service '{service_name}'"
            ));
        }

        // Clean up empty service entries
        if instances.is_empty() {
            self.services.remove(service_name);
        }

        Ok(())
    }

    /// Record a heartbeat for an instance.
    pub fn heartbeat(
        &mut self,
        service_name: &str,
        instance_id: &str,
        timestamp_ms: u64,
    ) -> Result<(), String> {
        let instances = self
            .services
            .get_mut(service_name)
            .ok_or_else(|| format!("Service '{service_name}' not found"))?;

        let instance = instances
            .iter_mut()
            .find(|i| i.id == instance_id)
            .ok_or_else(|| {
                format!("Instance '{instance_id}' not found in service '{service_name}'")
            })?;

        instance.last_heartbeat_ms = timestamp_ms;
        Ok(())
    }

    /// Mark stale instances (no heartbeat within timeout) as Unknown.
    pub fn mark_stale(&mut self, now_ms: u64) -> usize {
        let mut stale_count = 0;
        for instances in self.services.values_mut() {
            for instance in instances.iter_mut() {
                if instance.status != ServiceStatus::Unknown
                    && now_ms.saturating_sub(instance.last_heartbeat_ms) > self.heartbeat_timeout_ms
                {
                    instance.status = ServiceStatus::Unknown;
                    stale_count += 1;
                }
            }
        }
        stale_count
    }

    /// Discover healthy instances of a service.
    pub fn discover(&self, service_name: &str) -> Vec<&ServiceInstance> {
        self.services
            .get(service_name)
            .map(|instances| instances.iter().filter(|i| i.is_routable()).collect())
            .unwrap_or_default()
    }

    /// Discover instances by tag.
    pub fn discover_by_tag(&self, tag_key: &str, tag_value: &str) -> Vec<&ServiceInstance> {
        self.services
            .values()
            .flatten()
            .filter(|i| {
                i.is_routable() && i.tags.get(tag_key).map(|v| v.as_str()) == Some(tag_value)
            })
            .collect()
    }

    /// Get all service names.
    pub fn service_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.services.keys().cloned().collect();
        names.sort();
        names
    }

    /// Get total instance count across all services.
    pub fn instance_count(&self) -> usize {
        self.services.values().map(|v| v.len()).sum()
    }

    /// Get healthy instance count for a service.
    pub fn healthy_count(&self, service_name: &str) -> usize {
        self.discover(service_name).len()
    }

    /// Set instance status.
    pub fn set_status(
        &mut self,
        service_name: &str,
        instance_id: &str,
        status: ServiceStatus,
    ) -> Result<(), String> {
        let instances = self
            .services
            .get_mut(service_name)
            .ok_or_else(|| format!("Service '{service_name}' not found"))?;

        let instance = instances
            .iter_mut()
            .find(|i| i.id == instance_id)
            .ok_or_else(|| {
                format!("Instance '{instance_id}' not found in service '{service_name}'")
            })?;

        instance.status = status;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_instance(id: &str, service: &str, port: u16) -> ServiceInstance {
        ServiceInstance {
            id: id.to_string(),
            service_name: service.to_string(),
            host: "127.0.0.1".to_string(),
            port,
            weight: 1,
            status: ServiceStatus::Healthy,
            tags: HashMap::new(),
            registered_at_ms: 1000,
            last_heartbeat_ms: 1000,
        }
    }

    #[test]
    fn test_register_and_discover() {
        let mut reg = ServiceRegistry::new(5000);
        reg.register(make_instance("i1", "routing", 8001)).unwrap();
        reg.register(make_instance("i2", "routing", 8002)).unwrap();
        let instances = reg.discover("routing");
        assert_eq!(instances.len(), 2);
    }

    #[test]
    fn test_register_duplicate_rejected() {
        let mut reg = ServiceRegistry::new(5000);
        reg.register(make_instance("i1", "routing", 8001)).unwrap();
        let err = reg
            .register(make_instance("i1", "routing", 8002))
            .unwrap_err();
        assert!(err.contains("already registered"));
    }

    #[test]
    fn test_deregister() {
        let mut reg = ServiceRegistry::new(5000);
        reg.register(make_instance("i1", "routing", 8001)).unwrap();
        reg.deregister("routing", "i1").unwrap();
        assert_eq!(reg.instance_count(), 0);
    }

    #[test]
    fn test_unhealthy_not_discovered() {
        let mut reg = ServiceRegistry::new(5000);
        reg.register(make_instance("i1", "routing", 8001)).unwrap();
        reg.set_status("routing", "i1", ServiceStatus::Unhealthy)
            .unwrap();
        assert_eq!(reg.discover("routing").len(), 0);
    }

    #[test]
    fn test_degraded_is_routable() {
        let mut reg = ServiceRegistry::new(5000);
        reg.register(make_instance("i1", "routing", 8001)).unwrap();
        reg.set_status("routing", "i1", ServiceStatus::Degraded)
            .unwrap();
        assert_eq!(reg.discover("routing").len(), 1);
    }

    #[test]
    fn test_heartbeat_and_stale() {
        let mut reg = ServiceRegistry::new(5000);
        reg.register(make_instance("i1", "routing", 8001)).unwrap();
        reg.register(make_instance("i2", "routing", 8002)).unwrap();

        // i1 sends heartbeat at 3000, i2 does not
        reg.heartbeat("routing", "i1", 3000).unwrap();

        // At 7000, i2 is stale (last_heartbeat=1000, timeout=5000, 7000-1000=6000>5000)
        // i1 is not stale (7000-3000=4000<5000)
        let stale = reg.mark_stale(7000);
        assert_eq!(stale, 1);
        assert_eq!(reg.healthy_count("routing"), 1);
    }

    #[test]
    fn test_discover_by_tag() {
        let mut reg = ServiceRegistry::new(5000);
        let mut i1 = make_instance("i1", "routing", 8001);
        i1.tags.insert("region".to_string(), "us-east".to_string());
        let mut i2 = make_instance("i2", "routing", 8002);
        i2.tags.insert("region".to_string(), "eu-west".to_string());
        reg.register(i1).unwrap();
        reg.register(i2).unwrap();

        let us_instances = reg.discover_by_tag("region", "us-east");
        assert_eq!(us_instances.len(), 1);
        assert_eq!(us_instances[0].id, "i1");
    }

    #[test]
    fn test_service_names() {
        let mut reg = ServiceRegistry::new(5000);
        reg.register(make_instance("i1", "routing", 8001)).unwrap();
        reg.register(make_instance("i1", "gnss", 9001)).unwrap();
        assert_eq!(reg.service_names(), vec!["gnss", "routing"]);
    }

    #[test]
    fn test_address() {
        let inst = make_instance("i1", "routing", 8080);
        assert_eq!(inst.address(), "127.0.0.1:8080");
    }
}
