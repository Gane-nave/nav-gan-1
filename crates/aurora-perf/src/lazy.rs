//! Lazy loading — deferred initialization, resource prefetching, priority loading.

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

/// Load state of a resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadState {
    /// Not yet requested.
    Unloaded,
    /// Queued for loading but not yet started.
    Queued,
    /// Loading in progress.
    Loading,
    /// Loaded successfully.
    Loaded,
    /// Failed to load.
    Failed,
    /// Evicted from memory.
    Evicted,
}

/// Priority level for loading.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum LoadPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// A lazily loaded resource descriptor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LazyResource {
    pub id: String,
    pub resource_type: ResourceType,
    pub state: LoadState,
    pub priority: LoadPriority,
    pub size_bytes: Option<usize>,
    pub dependencies: Vec<String>,
}

/// Type of loadable resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceType {
    MapTile,
    RouteData,
    PoiData,
    TextureAtlas,
    AudioClip,
    FontSet,
    TranslationBundle,
    ConfigData,
}

impl LazyResource {
    /// Create a new lazy resource.
    pub fn new(id: &str, resource_type: ResourceType) -> Self {
        Self {
            id: id.to_string(),
            resource_type,
            state: LoadState::Unloaded,
            priority: LoadPriority::Normal,
            size_bytes: None,
            dependencies: Vec::new(),
        }
    }

    /// Set priority.
    pub fn with_priority(mut self, priority: LoadPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Add a dependency.
    pub fn with_dependency(mut self, dep: &str) -> Self {
        self.dependencies.push(dep.to_string());
        self
    }

    /// Check if all dependencies are met.
    pub fn dependencies_met(&self, loaded: &[String]) -> bool {
        self.dependencies.iter().all(|d| loaded.contains(d))
    }
}

/// Lazy load manager — tracks resources, manages load queues, and handles prefetching.
pub struct LazyLoadManager {
    resources: RwLock<HashMap<String, LazyResource>>,
    load_queue: RwLock<VecDeque<String>>,
    loaded_ids: RwLock<Vec<String>>,
    max_concurrent: usize,
    total_loaded_bytes: RwLock<usize>,
    memory_budget: usize,
}

impl LazyLoadManager {
    /// Create a new lazy load manager.
    pub fn new(max_concurrent: usize, memory_budget: usize) -> Self {
        Self {
            resources: RwLock::new(HashMap::new()),
            load_queue: RwLock::new(VecDeque::new()),
            loaded_ids: RwLock::new(Vec::new()),
            max_concurrent,
            total_loaded_bytes: RwLock::new(0),
            memory_budget,
        }
    }

    /// Register a resource for lazy loading.
    pub fn register(&self, resource: LazyResource) {
        self.resources.write().insert(resource.id.clone(), resource);
    }

    /// Request a resource to be loaded.
    pub fn request_load(&self, id: &str) -> bool {
        let mut resources = self.resources.write();
        if let Some(res) = resources.get_mut(id) {
            if res.state == LoadState::Unloaded || res.state == LoadState::Evicted {
                res.state = LoadState::Queued;
                let mut queue = self.load_queue.write();
                if !queue.contains(&id.to_string()) {
                    queue.push_back(id.to_string());
                }
                return true;
            }
        }
        false
    }

    /// Mark a resource as loaded.
    pub fn mark_loaded(&self, id: &str, size_bytes: usize) {
        let mut resources = self.resources.write();
        if let Some(res) = resources.get_mut(id) {
            res.state = LoadState::Loaded;
            res.size_bytes = Some(size_bytes);
            self.loaded_ids.write().push(id.to_string());
            *self.total_loaded_bytes.write() += size_bytes;
        }
        self.load_queue.write().retain(|k| k != id);
    }

    /// Mark a resource as failed.
    pub fn mark_failed(&self, id: &str) {
        let mut resources = self.resources.write();
        if let Some(res) = resources.get_mut(id) {
            res.state = LoadState::Failed;
        }
        self.load_queue.write().retain(|k| k != id);
    }

    /// Evict a resource to free memory.
    pub fn evict(&self, id: &str) -> usize {
        let mut resources = self.resources.write();
        if let Some(res) = resources.get_mut(id) {
            if res.state == LoadState::Loaded {
                let freed = res.size_bytes.unwrap_or(0);
                res.state = LoadState::Evicted;
                res.size_bytes = None;
                self.loaded_ids.write().retain(|k| k != id);
                let mut total = self.total_loaded_bytes.write();
                *total = total.saturating_sub(freed);
                return freed;
            }
        }
        0
    }

    /// Get the next batch of resources to load (respecting concurrency limit).
    /// Returns queued resource IDs that are ready to start loading, and
    /// promotes their state from Queued to Loading.
    pub fn next_batch(&self) -> Vec<String> {
        let mut resources = self.resources.write();
        let loaded_ids = self.loaded_ids.read();
        let queue = self.load_queue.read();
        let loaded_strings: Vec<String> = loaded_ids.clone();

        // Only count resources actually in Loading state (not Queued)
        let currently_loading = resources
            .values()
            .filter(|r| r.state == LoadState::Loading)
            .count();
        let slots = self.max_concurrent.saturating_sub(currently_loading);

        let batch: Vec<String> = queue
            .iter()
            .filter(|id| {
                resources
                    .get(id.as_str())
                    .map(|r| r.state == LoadState::Queued && r.dependencies_met(&loaded_strings))
                    .unwrap_or(false)
            })
            .take(slots)
            .cloned()
            .collect();

        // Promote selected items from Queued to Loading
        for id in &batch {
            if let Some(res) = resources.get_mut(id.as_str()) {
                res.state = LoadState::Loading;
            }
        }

        batch
    }

    /// Get the state of a resource.
    pub fn state(&self, id: &str) -> Option<LoadState> {
        self.resources.read().get(id).map(|r| r.state)
    }

    /// Total memory used by loaded resources.
    pub fn memory_used(&self) -> usize {
        *self.total_loaded_bytes.read()
    }

    /// Check if memory budget is exceeded.
    pub fn over_budget(&self) -> bool {
        self.memory_used() > self.memory_budget
    }

    /// Number of registered resources.
    pub fn resource_count(&self) -> usize {
        self.resources.read().len()
    }

    /// Number of loaded resources.
    pub fn loaded_count(&self) -> usize {
        self.loaded_ids.read().len()
    }

    /// Prefetch resources by type — requests loading for all matching unloaded resources.
    pub fn prefetch_by_type(&self, resource_type: ResourceType) -> usize {
        let ids: Vec<String> = self
            .resources
            .read()
            .values()
            .filter(|r| {
                r.resource_type == resource_type
                    && (r.state == LoadState::Unloaded || r.state == LoadState::Evicted)
            })
            .map(|r| r.id.clone())
            .collect();

        let mut count = 0;
        for id in ids {
            if self.request_load(&id) {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lazy_resource_creation() {
        let res =
            LazyResource::new("tile_0_0", ResourceType::MapTile).with_priority(LoadPriority::High);
        assert_eq!(res.state, LoadState::Unloaded);
        assert_eq!(res.priority, LoadPriority::High);
    }

    #[test]
    fn test_dependencies_met() {
        let res =
            LazyResource::new("route_data", ResourceType::RouteData).with_dependency("map_data");

        assert!(!res.dependencies_met(&[]));
        assert!(res.dependencies_met(&["map_data".to_string()]));
    }

    #[test]
    fn test_load_manager_basic() {
        let mgr = LazyLoadManager::new(4, 1024 * 1024);
        mgr.register(LazyResource::new("tile_a", ResourceType::MapTile));

        assert!(mgr.request_load("tile_a"));
        assert_eq!(mgr.state("tile_a"), Some(LoadState::Queued));

        mgr.mark_loaded("tile_a", 1024);
        assert_eq!(mgr.state("tile_a"), Some(LoadState::Loaded));
        assert_eq!(mgr.memory_used(), 1024);
    }

    #[test]
    fn test_load_manager_evict() {
        let mgr = LazyLoadManager::new(4, 2048);
        mgr.register(LazyResource::new("tile_a", ResourceType::MapTile));
        mgr.request_load("tile_a");
        mgr.mark_loaded("tile_a", 1024);

        let freed = mgr.evict("tile_a");
        assert_eq!(freed, 1024);
        assert_eq!(mgr.memory_used(), 0);
        assert_eq!(mgr.state("tile_a"), Some(LoadState::Evicted));
    }

    #[test]
    fn test_load_manager_failed() {
        let mgr = LazyLoadManager::new(4, 1024);
        mgr.register(LazyResource::new("bad_tile", ResourceType::MapTile));
        mgr.request_load("bad_tile");
        mgr.mark_failed("bad_tile");
        assert_eq!(mgr.state("bad_tile"), Some(LoadState::Failed));
    }

    #[test]
    fn test_next_batch_respects_concurrency() {
        let mgr = LazyLoadManager::new(2, 1024 * 1024);
        for i in 0..5 {
            let id = format!("tile_{}", i);
            mgr.register(LazyResource::new(&id, ResourceType::MapTile));
            mgr.request_load(&id);
        }

        // First batch: exactly 2 items promoted from Queued to Loading
        let batch1 = mgr.next_batch();
        assert_eq!(batch1.len(), 2);

        // Second batch: still 2 Loading, so 0 slots available
        let batch2 = mgr.next_batch();
        assert_eq!(batch2.len(), 0);

        // Complete one item — frees a slot
        mgr.mark_loaded(&batch1[0], 100);
        let batch3 = mgr.next_batch();
        assert_eq!(batch3.len(), 1);
    }

    #[test]
    fn test_next_batch_respects_dependencies() {
        let mgr = LazyLoadManager::new(4, 1024 * 1024);
        mgr.register(LazyResource::new("base", ResourceType::ConfigData));
        mgr.register(
            LazyResource::new("dependent", ResourceType::RouteData).with_dependency("base"),
        );

        mgr.request_load("base");
        mgr.request_load("dependent");

        let batch = mgr.next_batch();
        // "dependent" should not be in batch since "base" is not loaded yet
        assert!(!batch.contains(&"dependent".to_string()));
    }

    #[test]
    fn test_prefetch_by_type() {
        let mgr = LazyLoadManager::new(4, 1024 * 1024);
        mgr.register(LazyResource::new("tile_1", ResourceType::MapTile));
        mgr.register(LazyResource::new("tile_2", ResourceType::MapTile));
        mgr.register(LazyResource::new("route", ResourceType::RouteData));

        let count = mgr.prefetch_by_type(ResourceType::MapTile);
        assert_eq!(count, 2);
        assert_eq!(mgr.state("tile_1"), Some(LoadState::Queued));
        assert_eq!(mgr.state("route"), Some(LoadState::Unloaded));
    }

    #[test]
    fn test_over_budget() {
        let mgr = LazyLoadManager::new(4, 100);
        mgr.register(LazyResource::new("big", ResourceType::MapTile));
        mgr.request_load("big");
        mgr.mark_loaded("big", 200);
        assert!(mgr.over_budget());
    }

    #[test]
    fn test_reload_evicted() {
        let mgr = LazyLoadManager::new(4, 1024);
        mgr.register(LazyResource::new("tile", ResourceType::MapTile));
        mgr.request_load("tile");
        mgr.mark_loaded("tile", 100);
        mgr.evict("tile");
        assert_eq!(mgr.state("tile"), Some(LoadState::Evicted));

        // Re-request load
        assert!(mgr.request_load("tile"));
        assert_eq!(mgr.state("tile"), Some(LoadState::Queued));
    }
}
