//! Resource limits — define maximum allowable usage.

/// Resource type being limited.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    /// CPU usage (percentage 0-100).
    Cpu,
    /// Memory usage in bytes.
    Memory,
    /// Disk I/O in bytes per second.
    DiskIo,
    /// Network bandwidth in bytes per second.
    Network,
    /// Number of concurrent connections.
    Connections,
    /// Number of requests per interval.
    Requests,
    /// Custom named resource.
    Custom,
}

impl ResourceType {
    /// Display name.
    pub fn as_str(&self) -> &'static str {
        match self {
            ResourceType::Cpu => "cpu",
            ResourceType::Memory => "memory",
            ResourceType::DiskIo => "disk_io",
            ResourceType::Network => "network",
            ResourceType::Connections => "connections",
            ResourceType::Requests => "requests",
            ResourceType::Custom => "custom",
        }
    }
}

/// Action to take when a quota is exceeded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExceedAction {
    /// Reject the request.
    Reject,
    /// Throttle (slow down) the request.
    Throttle,
    /// Log a warning but allow.
    Warn,
    /// Queue the request for later.
    Queue,
}

/// A resource limit definition.
#[derive(Debug, Clone)]
pub struct ResourceLimit {
    /// Name of this limit.
    pub name: String,
    /// Type of resource.
    pub resource_type: ResourceType,
    /// Maximum value (hard limit).
    pub hard_limit: u64,
    /// Warning threshold (soft limit).
    pub soft_limit: u64,
    /// Action when hard limit is exceeded.
    pub exceed_action: ExceedAction,
    /// Whether this limit is enabled.
    pub enabled: bool,
}

impl ResourceLimit {
    /// Create a new resource limit.
    pub fn new(name: &str, resource_type: ResourceType, hard_limit: u64) -> Self {
        Self {
            name: name.to_string(),
            resource_type,
            hard_limit,
            soft_limit: (hard_limit as f64 * 0.8) as u64,
            exceed_action: ExceedAction::Reject,
            enabled: true,
        }
    }

    /// Set the soft limit.
    pub fn with_soft_limit(mut self, soft_limit: u64) -> Self {
        self.soft_limit = soft_limit;
        self
    }

    /// Set the exceed action.
    pub fn with_exceed_action(mut self, action: ExceedAction) -> Self {
        self.exceed_action = action;
        self
    }

    /// Check whether a value is within the limit.
    pub fn check(&self, value: u64) -> LimitStatus {
        if !self.enabled {
            return LimitStatus::Ok;
        }
        if value > self.hard_limit {
            LimitStatus::Exceeded
        } else if value > self.soft_limit {
            LimitStatus::Warning
        } else {
            LimitStatus::Ok
        }
    }

    /// Remaining capacity before hitting the hard limit.
    pub fn remaining(&self, current: u64) -> u64 {
        self.hard_limit.saturating_sub(current)
    }

    /// Usage ratio (0.0 – 1.0).
    pub fn usage_ratio(&self, current: u64) -> f64 {
        if self.hard_limit == 0 {
            return if current > 0 { 1.0 } else { 0.0 };
        }
        (current as f64 / self.hard_limit as f64).min(1.0)
    }
}

/// Status of a limit check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitStatus {
    /// Usage is within normal range.
    Ok,
    /// Usage is above the soft limit (warning).
    Warning,
    /// Usage exceeds the hard limit.
    Exceeded,
}

impl LimitStatus {
    /// Display name.
    pub fn as_str(&self) -> &'static str {
        match self {
            LimitStatus::Ok => "ok",
            LimitStatus::Warning => "warning",
            LimitStatus::Exceeded => "exceeded",
        }
    }

    /// Whether the status indicates a problem.
    pub fn is_problem(&self) -> bool {
        *self != LimitStatus::Ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limit_creation() {
        let limit = ResourceLimit::new("mem", ResourceType::Memory, 1000);
        assert_eq!(limit.name, "mem");
        assert_eq!(limit.hard_limit, 1000);
        assert_eq!(limit.soft_limit, 800); // 80% of 1000
        assert!(limit.enabled);
    }

    #[test]
    fn test_limit_check_ok() {
        let limit = ResourceLimit::new("mem", ResourceType::Memory, 1000);
        assert_eq!(limit.check(500), LimitStatus::Ok);
    }

    #[test]
    fn test_limit_check_warning() {
        let limit = ResourceLimit::new("mem", ResourceType::Memory, 1000);
        assert_eq!(limit.check(850), LimitStatus::Warning);
    }

    #[test]
    fn test_limit_check_exceeded() {
        let limit = ResourceLimit::new("mem", ResourceType::Memory, 1000);
        assert_eq!(limit.check(1100), LimitStatus::Exceeded);
    }

    #[test]
    fn test_limit_disabled() {
        let mut limit = ResourceLimit::new("mem", ResourceType::Memory, 1000);
        limit.enabled = false;
        assert_eq!(limit.check(9999), LimitStatus::Ok);
    }

    #[test]
    fn test_remaining() {
        let limit = ResourceLimit::new("mem", ResourceType::Memory, 1000);
        assert_eq!(limit.remaining(700), 300);
        assert_eq!(limit.remaining(1200), 0); // saturating sub
    }

    #[test]
    fn test_usage_ratio() {
        let limit = ResourceLimit::new("mem", ResourceType::Memory, 1000);
        assert!((limit.usage_ratio(500) - 0.5).abs() < f64::EPSILON);
        assert!((limit.usage_ratio(1500) - 1.0).abs() < f64::EPSILON); // capped at 1.0
    }

    #[test]
    fn test_builder_pattern() {
        let limit = ResourceLimit::new("net", ResourceType::Network, 5000)
            .with_soft_limit(4000)
            .with_exceed_action(ExceedAction::Throttle);

        assert_eq!(limit.soft_limit, 4000);
        assert_eq!(limit.exceed_action, ExceedAction::Throttle);
    }

    #[test]
    fn test_resource_type_display() {
        assert_eq!(ResourceType::Cpu.as_str(), "cpu");
        assert_eq!(ResourceType::Memory.as_str(), "memory");
        assert_eq!(ResourceType::Custom.as_str(), "custom");
    }

    #[test]
    fn test_limit_status_is_problem() {
        assert!(!LimitStatus::Ok.is_problem());
        assert!(LimitStatus::Warning.is_problem());
        assert!(LimitStatus::Exceeded.is_problem());
    }
}
