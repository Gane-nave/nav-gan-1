//! Cross-crate integration — verifies that data types and interfaces work correctly
//! when passed between different Aurora crates.

use std::collections::HashMap;

/// Represents data flowing between crates.
#[derive(Debug, Clone)]
pub struct InterCrateMessage {
    /// Source crate name.
    pub source: String,
    /// Destination crate name.
    pub destination: String,
    /// Message type tag.
    pub msg_type: String,
    /// Payload size in bytes.
    pub payload_bytes: usize,
    /// Timestamp (epoch ms).
    pub timestamp_ms: u64,
}

/// Contract verification — ensures crate interfaces satisfy expected contracts.
#[derive(Debug, Clone)]
pub struct ContractCheck {
    /// Contract name.
    pub name: String,
    /// Whether the contract is satisfied.
    pub satisfied: bool,
    /// Description of what was checked.
    pub description: String,
}

/// Cross-crate integration verifier.
pub struct IntegrationVerifier {
    messages: Vec<InterCrateMessage>,
    contracts: Vec<ContractCheck>,
    crate_dependencies: HashMap<String, Vec<String>>,
}

impl IntegrationVerifier {
    /// Create a new verifier.
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            contracts: Vec::new(),
            crate_dependencies: HashMap::new(),
        }
    }

    /// Register a crate dependency.
    pub fn add_dependency(&mut self, crate_name: &str, depends_on: &str) {
        self.crate_dependencies
            .entry(crate_name.to_string())
            .or_default()
            .push(depends_on.to_string());
    }

    /// Record an inter-crate message.
    pub fn record_message(&mut self, msg: InterCrateMessage) {
        self.messages.push(msg);
    }

    /// Add a contract check result.
    pub fn check_contract(&mut self, name: &str, description: &str, satisfied: bool) {
        self.contracts.push(ContractCheck {
            name: name.to_string(),
            satisfied,
            description: description.to_string(),
        });
    }

    /// Get total message count.
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    /// Get messages between two specific crates.
    pub fn messages_between(&self, source: &str, dest: &str) -> Vec<&InterCrateMessage> {
        self.messages
            .iter()
            .filter(|m| m.source == source && m.destination == dest)
            .collect()
    }

    /// Get total data transferred in bytes.
    pub fn total_bytes_transferred(&self) -> usize {
        self.messages.iter().map(|m| m.payload_bytes).sum()
    }

    /// Check if all contracts are satisfied.
    pub fn all_contracts_satisfied(&self) -> bool {
        self.contracts.iter().all(|c| c.satisfied)
    }

    /// Get failing contracts.
    pub fn failing_contracts(&self) -> Vec<&ContractCheck> {
        self.contracts.iter().filter(|c| !c.satisfied).collect()
    }

    /// Get contract count.
    pub fn contract_count(&self) -> usize {
        self.contracts.len()
    }

    /// Get dependencies of a crate.
    pub fn dependencies_of(&self, crate_name: &str) -> Vec<&str> {
        self.crate_dependencies
            .get(crate_name)
            .map(|deps| deps.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Check for circular dependencies.
    pub fn has_circular_dependency(&self) -> bool {
        for start in self.crate_dependencies.keys() {
            let mut visited = std::collections::HashSet::new();
            let mut stack = vec![start.as_str()];
            while let Some(current) = stack.pop() {
                if !visited.insert(current) {
                    return true;
                }
                if let Some(deps) = self.crate_dependencies.get(current) {
                    for dep in deps {
                        stack.push(dep.as_str());
                    }
                }
            }
        }
        false
    }

    /// Get unique crates involved in messages.
    pub fn active_crates(&self) -> Vec<String> {
        let mut crates: std::collections::HashSet<String> = std::collections::HashSet::new();
        for msg in &self.messages {
            crates.insert(msg.source.clone());
            crates.insert(msg.destination.clone());
        }
        let mut sorted: Vec<String> = crates.into_iter().collect();
        sorted.sort();
        sorted
    }

    /// Generate a summary report.
    pub fn summary(&self) -> String {
        let passed = self.contracts.iter().filter(|c| c.satisfied).count();
        let total = self.contracts.len();
        format!(
            "Contracts: {passed}/{total} passed, Messages: {}, Bytes: {}",
            self.messages.len(),
            self.total_bytes_transferred()
        )
    }
}

impl Default for IntegrationVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_and_query_messages() {
        let mut v = IntegrationVerifier::new();
        v.record_message(InterCrateMessage {
            source: "gnss".to_string(),
            destination: "fusion".to_string(),
            msg_type: "position".to_string(),
            payload_bytes: 256,
            timestamp_ms: 1000,
        });
        v.record_message(InterCrateMessage {
            source: "fusion".to_string(),
            destination: "routing".to_string(),
            msg_type: "fused_state".to_string(),
            payload_bytes: 512,
            timestamp_ms: 1010,
        });
        assert_eq!(v.message_count(), 2);
        assert_eq!(v.total_bytes_transferred(), 768);
        assert_eq!(v.messages_between("gnss", "fusion").len(), 1);
        assert_eq!(v.messages_between("fusion", "gnss").len(), 0);
    }

    #[test]
    fn test_contract_checking() {
        let mut v = IntegrationVerifier::new();
        v.check_contract("gnss_output_format", "GNSS outputs WGS84 coords", true);
        v.check_contract("fusion_latency", "Fusion < 10ms", true);
        v.check_contract("routing_coverage", "Route covers all waypoints", false);
        assert!(!v.all_contracts_satisfied());
        assert_eq!(v.failing_contracts().len(), 1);
        assert_eq!(v.failing_contracts()[0].name, "routing_coverage");
        assert_eq!(v.contract_count(), 3);
    }

    #[test]
    fn test_all_contracts_pass() {
        let mut v = IntegrationVerifier::new();
        v.check_contract("c1", "desc1", true);
        v.check_contract("c2", "desc2", true);
        assert!(v.all_contracts_satisfied());
        assert!(v.failing_contracts().is_empty());
    }

    #[test]
    fn test_dependency_graph() {
        let mut v = IntegrationVerifier::new();
        v.add_dependency("fusion", "gnss");
        v.add_dependency("fusion", "sensors");
        v.add_dependency("routing", "fusion");
        let deps = v.dependencies_of("fusion");
        assert_eq!(deps.len(), 2);
        assert!(deps.contains(&"gnss"));
        assert!(deps.contains(&"sensors"));
        assert_eq!(v.dependencies_of("gnss").len(), 0);
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut v = IntegrationVerifier::new();
        v.add_dependency("a", "b");
        v.add_dependency("b", "c");
        assert!(!v.has_circular_dependency());

        v.add_dependency("c", "a");
        assert!(v.has_circular_dependency());
    }

    #[test]
    fn test_active_crates() {
        let mut v = IntegrationVerifier::new();
        v.record_message(InterCrateMessage {
            source: "gnss".to_string(),
            destination: "fusion".to_string(),
            msg_type: "pos".to_string(),
            payload_bytes: 100,
            timestamp_ms: 1000,
        });
        let active = v.active_crates();
        assert_eq!(active, vec!["fusion", "gnss"]); // sorted
    }

    #[test]
    fn test_summary() {
        let mut v = IntegrationVerifier::new();
        v.check_contract("c1", "d1", true);
        v.record_message(InterCrateMessage {
            source: "a".to_string(),
            destination: "b".to_string(),
            msg_type: "t".to_string(),
            payload_bytes: 100,
            timestamp_ms: 0,
        });
        let s = v.summary();
        assert!(s.contains("1/1 passed"));
        assert!(s.contains("Messages: 1"));
        assert!(s.contains("Bytes: 100"));
    }

    #[test]
    fn test_empty_verifier() {
        let v = IntegrationVerifier::new();
        assert!(v.all_contracts_satisfied());
        assert_eq!(v.message_count(), 0);
        assert_eq!(v.total_bytes_transferred(), 0);
        assert!(!v.has_circular_dependency());
    }
}
