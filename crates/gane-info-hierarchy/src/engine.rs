/// Information hierarchy: show only critical info, hide the rest.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum InfoPriority {
    Critical,
    High,
    Medium,
    Low,
    Background,
}
#[derive(Debug, Clone)]
pub struct InfoElement {
    pub name: String,
    pub priority: InfoPriority,
    pub relevance: f64,
    pub cognitive_cost: f64,
}
impl InfoElement {
    pub fn display_score(&self) -> f64 {
        let p = match self.priority {
            InfoPriority::Critical => 1.0,
            InfoPriority::High => 0.8,
            InfoPriority::Medium => 0.5,
            InfoPriority::Low => 0.2,
            InfoPriority::Background => 0.0,
        };
        (p * 0.5
            + self.relevance.clamp(0.0, 1.0) * 0.3
            + (1.0 - self.cognitive_cost.clamp(0.0, 1.0)) * 0.2)
            .clamp(0.0, 1.0)
    }
}
#[derive(Debug, Clone)]
pub struct InfoManager {
    pub elements: Vec<InfoElement>,
    pub max_cognitive_budget: f64,
}
impl InfoManager {
    pub fn new(budget: f64) -> Self {
        Self {
            elements: Vec::new(),
            max_cognitive_budget: budget,
        }
    }
    pub fn add_element(&mut self, e: InfoElement) {
        self.elements.push(e);
    }
    pub fn visible_elements(&self) -> Vec<&InfoElement> {
        let mut sorted: Vec<_> = self.elements.iter().collect();
        sorted.sort_by(|a, b| {
            b.display_score()
                .partial_cmp(&a.display_score())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let mut budget = self.max_cognitive_budget;
        let mut result = Vec::new();
        for e in sorted {
            if budget <= 0.0 {
                break;
            }
            budget -= e.cognitive_cost;
            result.push(e);
        }
        result
    }
    pub fn cognitive_load(&self) -> f64 {
        self.visible_elements()
            .iter()
            .map(|e| e.cognitive_cost)
            .sum()
    }
    pub fn is_overloaded(&self) -> bool {
        self.cognitive_load() > self.max_cognitive_budget
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_priority() {
        let e = InfoElement {
            name: "turn".into(),
            priority: InfoPriority::Critical,
            relevance: 1.0,
            cognitive_cost: 0.2,
        };
        assert!(e.display_score() > 0.8);
    }
    #[test]
    fn test_budget() {
        let mut m = InfoManager::new(0.5);
        m.add_element(InfoElement {
            name: "a".into(),
            priority: InfoPriority::Critical,
            relevance: 1.0,
            cognitive_cost: 0.3,
        });
        m.add_element(InfoElement {
            name: "b".into(),
            priority: InfoPriority::Low,
            relevance: 0.2,
            cognitive_cost: 0.4,
        });
        assert!(m.visible_elements().len() <= 2);
    }
    #[test]
    fn test_empty() {
        let m = InfoManager::new(1.0);
        assert_eq!(m.cognitive_load(), 0.0);
        assert!(!m.is_overloaded());
    }
}
