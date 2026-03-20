/// Reinforcement learning: agent, env, reward, policy, explore
/// Phase 1019

#[derive(Debug, Clone)]
pub struct Reinforce {
    pub agent_ok: bool,
    pub env_ok: bool,
    pub reward_ok: bool,
    pub policy_ok: bool,
    pub explore_ok: bool,
}

impl Default for Reinforce {
    fn default() -> Self {
        Self::new()
    }
}

impl Reinforce {
    pub fn new() -> Self {
        Self {
            agent_ok: true,
            env_ok: true,
            reward_ok: true,
            policy_ok: true,
            explore_ok: true,
        }
    }

    pub fn learning_ok(&self) -> bool {
        self.agent_ok && self.env_ok && self.reward_ok
    }

    pub fn strategy_ok(&self) -> bool {
        self.policy_ok && self.explore_ok
    }

    pub fn all_ok(&self) -> bool {
        self.learning_ok() && self.strategy_ok()
    }

    pub fn needs_train(&self) -> bool {
        !self.agent_ok || !self.policy_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.agent_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_learning() {
        let c = Reinforce::new();
        assert!(c.learning_ok());
    }

    #[test]
    fn test_strategy() {
        let c = Reinforce::new();
        assert!(c.strategy_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Reinforce::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_train() {
        let c = Reinforce::new();
        assert!(!c.needs_train());
    }

    #[test]
    fn test_agent() {
        let mut c = Reinforce::new();
        c.agent_ok = false;
        assert!(c.needs_train());
    }

    #[test]
    fn test_health() {
        let c = Reinforce::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
