//! Resource management — monitors and manages device resources at the edge.
//!
//! Tracks CPU, memory, battery, thermal state, and network connectivity
//! to inform edge processing decisions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

/// Current state of a device resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceHealth {
    /// Resource is within normal parameters.
    Normal,
    /// Resource is under moderate pressure.
    Warning,
    /// Resource is critically constrained.
    Critical,
    /// Resource is unavailable.
    Unavailable,
}

/// Network connectivity state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectivityState {
    /// Full internet connectivity.
    Online,
    /// Limited connectivity (high latency, low bandwidth).
    Limited,
    /// Peer-to-peer / mesh only.
    MeshOnly,
    /// No connectivity.
    Offline,
}

/// Snapshot of device resource state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSnapshot {
    pub timestamp: DateTime<Utc>,
    pub cpu_usage_pct: f64,
    pub memory_usage_pct: f64,
    pub battery_pct: f64,
    pub temperature_celsius: f64,
    pub storage_free_bytes: u64,
    pub connectivity: ConnectivityState,
    pub overall_health: ResourceHealth,
}

/// Resource manager — tracks device state and provides recommendations.
pub struct ResourceManager {
    snapshots: Vec<ResourceSnapshot>,
    max_snapshots: usize,
    /// Thresholds.
    cpu_warning_pct: f64,
    cpu_critical_pct: f64,
    memory_warning_pct: f64,
    memory_critical_pct: f64,
    battery_warning_pct: f64,
    battery_critical_pct: f64,
    thermal_warning_c: f64,
    thermal_critical_c: f64,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            max_snapshots: 100,
            cpu_warning_pct: 70.0,
            cpu_critical_pct: 90.0,
            memory_warning_pct: 75.0,
            memory_critical_pct: 90.0,
            battery_warning_pct: 20.0,
            battery_critical_pct: 5.0,
            thermal_warning_c: 70.0,
            thermal_critical_c: 85.0,
        }
    }

    /// Record a resource snapshot.
    pub fn record_snapshot(
        &mut self,
        cpu_pct: f64,
        memory_pct: f64,
        battery_pct: f64,
        temperature_c: f64,
        storage_free: u64,
        connectivity: ConnectivityState,
    ) -> ResourceSnapshot {
        let overall = self.compute_health(cpu_pct, memory_pct, battery_pct, temperature_c);

        let snapshot = ResourceSnapshot {
            timestamp: Utc::now(),
            cpu_usage_pct: cpu_pct,
            memory_usage_pct: memory_pct,
            battery_pct,
            temperature_celsius: temperature_c,
            storage_free_bytes: storage_free,
            connectivity,
            overall_health: overall,
        };

        if overall == ResourceHealth::Critical {
            warn!(
                cpu = cpu_pct,
                memory = memory_pct,
                battery = battery_pct,
                temp = temperature_c,
                "CRITICAL resource state"
            );
        } else if overall == ResourceHealth::Warning {
            debug!("resource warning state recorded");
        }

        if self.snapshots.len() >= self.max_snapshots {
            self.snapshots.remove(0);
        }
        self.snapshots.push(snapshot.clone());
        snapshot
    }

    fn compute_health(&self, cpu: f64, memory: f64, battery: f64, temp: f64) -> ResourceHealth {
        // Any critical condition → Critical.
        if cpu > self.cpu_critical_pct
            || memory > self.memory_critical_pct
            || battery < self.battery_critical_pct
            || temp > self.thermal_critical_c
        {
            return ResourceHealth::Critical;
        }

        // Any warning condition → Warning.
        if cpu > self.cpu_warning_pct
            || memory > self.memory_warning_pct
            || battery < self.battery_warning_pct
            || temp > self.thermal_warning_c
        {
            return ResourceHealth::Warning;
        }

        ResourceHealth::Normal
    }

    /// Get the latest snapshot.
    pub fn latest(&self) -> Option<&ResourceSnapshot> {
        self.snapshots.last()
    }

    /// Get the current overall health.
    pub fn current_health(&self) -> ResourceHealth {
        self.snapshots
            .last()
            .map(|s| s.overall_health)
            .unwrap_or(ResourceHealth::Unavailable)
    }

    /// Get the current connectivity state.
    pub fn connectivity(&self) -> ConnectivityState {
        self.snapshots
            .last()
            .map(|s| s.connectivity)
            .unwrap_or(ConnectivityState::Offline)
    }

    /// Check if device is online (full or limited connectivity).
    pub fn is_online(&self) -> bool {
        matches!(
            self.connectivity(),
            ConnectivityState::Online | ConnectivityState::Limited
        )
    }

    /// Get average CPU usage over recent snapshots.
    pub fn avg_cpu(&self) -> f64 {
        if self.snapshots.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.snapshots.iter().map(|s| s.cpu_usage_pct).sum();
        sum / self.snapshots.len() as f64
    }

    /// Get average battery level.
    pub fn avg_battery(&self) -> f64 {
        if self.snapshots.is_empty() {
            return 100.0;
        }
        let sum: f64 = self.snapshots.iter().map(|s| s.battery_pct).sum();
        sum / self.snapshots.len() as f64
    }

    /// Should processing be throttled based on current resource state?
    pub fn should_throttle(&self) -> bool {
        matches!(
            self.current_health(),
            ResourceHealth::Critical | ResourceHealth::Warning
        )
    }

    /// Recommend a processing scale factor (0.0–1.0) based on resources.
    /// 1.0 = full processing, 0.0 = minimal processing.
    pub fn recommended_scale(&self) -> f64 {
        match self.current_health() {
            ResourceHealth::Normal => 1.0,
            ResourceHealth::Warning => 0.5,
            ResourceHealth::Critical => 0.1,
            ResourceHealth::Unavailable => 0.0,
        }
    }

    /// Get snapshot history.
    pub fn history(&self) -> &[ResourceSnapshot] {
        &self.snapshots
    }

    /// Number of snapshots recorded.
    pub fn snapshot_count(&self) -> usize {
        self.snapshots.len()
    }

    /// Set thermal thresholds.
    pub fn set_thermal_thresholds(&mut self, warning_c: f64, critical_c: f64) {
        self.thermal_warning_c = warning_c;
        self.thermal_critical_c = critical_c;
        info!(
            warning = warning_c,
            critical = critical_c,
            "thermal thresholds updated"
        );
    }

    /// Set battery thresholds.
    pub fn set_battery_thresholds(&mut self, warning_pct: f64, critical_pct: f64) {
        self.battery_warning_pct = warning_pct;
        self.battery_critical_pct = critical_pct;
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_health() {
        let mut mgr = ResourceManager::new();
        let snap = mgr.record_snapshot(
            30.0,
            40.0,
            80.0,
            50.0,
            1_000_000_000,
            ConnectivityState::Online,
        );
        assert_eq!(snap.overall_health, ResourceHealth::Normal);
        assert_eq!(mgr.current_health(), ResourceHealth::Normal);
    }

    #[test]
    fn warning_on_high_cpu() {
        let mut mgr = ResourceManager::new();
        let snap = mgr.record_snapshot(
            75.0,
            40.0,
            80.0,
            50.0,
            1_000_000_000,
            ConnectivityState::Online,
        );
        assert_eq!(snap.overall_health, ResourceHealth::Warning);
    }

    #[test]
    fn critical_on_low_battery() {
        let mut mgr = ResourceManager::new();
        let snap = mgr.record_snapshot(
            30.0,
            40.0,
            3.0,
            50.0,
            1_000_000_000,
            ConnectivityState::Online,
        );
        assert_eq!(snap.overall_health, ResourceHealth::Critical);
    }

    #[test]
    fn critical_on_high_temp() {
        let mut mgr = ResourceManager::new();
        let snap = mgr.record_snapshot(
            30.0,
            40.0,
            80.0,
            90.0,
            1_000_000_000,
            ConnectivityState::Online,
        );
        assert_eq!(snap.overall_health, ResourceHealth::Critical);
    }

    #[test]
    fn connectivity_tracking() {
        let mut mgr = ResourceManager::new();
        mgr.record_snapshot(
            30.0,
            40.0,
            80.0,
            50.0,
            1_000_000_000,
            ConnectivityState::Offline,
        );
        assert_eq!(mgr.connectivity(), ConnectivityState::Offline);
        assert!(!mgr.is_online());

        mgr.record_snapshot(
            30.0,
            40.0,
            80.0,
            50.0,
            1_000_000_000,
            ConnectivityState::Online,
        );
        assert!(mgr.is_online());
    }

    #[test]
    fn avg_cpu_calculation() {
        let mut mgr = ResourceManager::new();
        mgr.record_snapshot(
            20.0,
            40.0,
            80.0,
            50.0,
            1_000_000_000,
            ConnectivityState::Online,
        );
        mgr.record_snapshot(
            40.0,
            40.0,
            80.0,
            50.0,
            1_000_000_000,
            ConnectivityState::Online,
        );
        assert!((mgr.avg_cpu() - 30.0).abs() < f64::EPSILON);
    }

    #[test]
    fn recommended_scale() {
        let mut mgr = ResourceManager::new();
        assert!((mgr.recommended_scale() - 0.0).abs() < f64::EPSILON); // No data = Unavailable.

        mgr.record_snapshot(
            30.0,
            40.0,
            80.0,
            50.0,
            1_000_000_000,
            ConnectivityState::Online,
        );
        assert!((mgr.recommended_scale() - 1.0).abs() < f64::EPSILON);

        mgr.record_snapshot(
            75.0,
            40.0,
            80.0,
            50.0,
            1_000_000_000,
            ConnectivityState::Online,
        );
        assert!((mgr.recommended_scale() - 0.5).abs() < f64::EPSILON);

        mgr.record_snapshot(
            95.0,
            40.0,
            80.0,
            50.0,
            1_000_000_000,
            ConnectivityState::Online,
        );
        assert!((mgr.recommended_scale() - 0.1).abs() < f64::EPSILON);
    }

    #[test]
    fn should_throttle() {
        let mut mgr = ResourceManager::new();
        mgr.record_snapshot(
            30.0,
            40.0,
            80.0,
            50.0,
            1_000_000_000,
            ConnectivityState::Online,
        );
        assert!(!mgr.should_throttle());

        mgr.record_snapshot(
            75.0,
            40.0,
            80.0,
            50.0,
            1_000_000_000,
            ConnectivityState::Online,
        );
        assert!(mgr.should_throttle());
    }

    #[test]
    fn snapshot_history_capped() {
        let mut mgr = ResourceManager::new();
        mgr.max_snapshots = 3;
        for _ in 0..5 {
            mgr.record_snapshot(
                30.0,
                40.0,
                80.0,
                50.0,
                1_000_000_000,
                ConnectivityState::Online,
            );
        }
        assert_eq!(mgr.snapshot_count(), 3);
    }

    #[test]
    fn no_data_defaults() {
        let mgr = ResourceManager::new();
        assert_eq!(mgr.current_health(), ResourceHealth::Unavailable);
        assert_eq!(mgr.connectivity(), ConnectivityState::Offline);
        assert!((mgr.avg_cpu() - 0.0).abs() < f64::EPSILON);
        assert!((mgr.avg_battery() - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn custom_thermal_thresholds() {
        let mut mgr = ResourceManager::new();
        mgr.set_thermal_thresholds(50.0, 60.0);

        // 55°C should now be Warning (not Normal).
        let snap = mgr.record_snapshot(
            30.0,
            40.0,
            80.0,
            55.0,
            1_000_000_000,
            ConnectivityState::Online,
        );
        assert_eq!(snap.overall_health, ResourceHealth::Warning);
    }
}
