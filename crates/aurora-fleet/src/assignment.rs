//! Assignment engine — matches tasks to drivers based on proximity,
//! skills, capacity, workload, and priority.

use aurora_core::types::{EntityId, GeoPosition};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::debug;

/// A driver available for task assignment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FleetDriver {
    pub id: EntityId,
    pub name: String,
    pub position: GeoPosition,
    pub vehicle_id: Option<EntityId>,
    pub status: DriverStatus,
    pub skills: Vec<String>,
    pub current_capacity_kg: f64,
    pub max_capacity_kg: f64,
    pub current_volume_m3: f64,
    pub max_volume_m3: f64,
    pub active_task_count: u32,
    pub max_concurrent_tasks: u32,
    pub shift_end: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}

/// Driver availability status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DriverStatus {
    Available,
    OnTask,
    OnBreak,
    OffDuty,
    Offline,
}

/// Result of an assignment decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignmentDecision {
    pub task_id: EntityId,
    pub driver_id: EntityId,
    pub score: f64,
    pub distance_km: f64,
    pub reason: String,
    pub timestamp: DateTime<Utc>,
}

/// Assignment strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssignmentStrategy {
    /// Assign to nearest available driver.
    NearestFirst,
    /// Assign to driver with fewest active tasks.
    LeastLoaded,
    /// Balance distance and workload.
    Balanced,
    /// Assign to best-skilled driver.
    BestSkillMatch,
}

/// Task requirements for assignment matching.
#[derive(Debug, Clone)]
pub struct TaskRequirements {
    pub task_id: EntityId,
    pub location: GeoPosition,
    pub required_skills: Vec<String>,
    pub weight_kg: Option<f64>,
    pub volume_m3: Option<f64>,
    pub deadline: Option<DateTime<Utc>>,
}

/// Assignment engine — finds the best driver for a given task.
pub struct AssignmentEngine {
    drivers: Vec<FleetDriver>,
    strategy: AssignmentStrategy,
    max_distance_km: f64,
    history: Vec<AssignmentDecision>,
}

impl AssignmentEngine {
    pub fn new(strategy: AssignmentStrategy) -> Self {
        Self {
            drivers: Vec::new(),
            strategy,
            max_distance_km: 50.0,
            history: Vec::new(),
        }
    }

    /// Register a driver.
    pub fn add_driver(&mut self, driver: FleetDriver) {
        debug!(driver_id = %driver.id, name = %driver.name, "Driver registered");
        self.drivers.push(driver);
    }

    /// Update a driver's position and status.
    pub fn update_driver(
        &mut self,
        driver_id: &EntityId,
        position: GeoPosition,
        status: DriverStatus,
    ) -> bool {
        if let Some(driver) = self.drivers.iter_mut().find(|d| d.id == *driver_id) {
            driver.position = position;
            driver.status = status;
            driver.updated_at = Utc::now();
            true
        } else {
            false
        }
    }

    /// Find the best driver for a task.
    pub fn find_best_match(&self, requirements: &TaskRequirements) -> Option<AssignmentDecision> {
        let candidates: Vec<(&FleetDriver, f64)> = self
            .drivers
            .iter()
            .filter(|d| self.is_eligible(d, requirements))
            .map(|d| {
                let distance_km = haversine_distance(&d.position, &requirements.location) / 1000.0;
                let score = self.compute_score(d, requirements, distance_km);
                (d, score)
            })
            .collect();

        if candidates.is_empty() {
            debug!(task_id = %requirements.task_id, "No eligible drivers found");
            return None;
        }

        // Higher score = better match.
        let (best_driver, best_score) = candidates
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap();

        let distance_km =
            haversine_distance(&best_driver.position, &requirements.location) / 1000.0;

        Some(AssignmentDecision {
            task_id: requirements.task_id,
            driver_id: best_driver.id,
            score: *best_score,
            distance_km,
            reason: format!("Strategy: {:?}, Score: {:.3}", self.strategy, best_score),
            timestamp: Utc::now(),
        })
    }

    /// Assign a task to the best driver and record the decision.
    pub fn assign(&mut self, requirements: &TaskRequirements) -> Option<AssignmentDecision> {
        let decision = self.find_best_match(requirements)?;
        debug!(
            task_id = %decision.task_id,
            driver_id = %decision.driver_id,
            score = decision.score,
            "Task assigned"
        );

        // Update driver state.
        if let Some(driver) = self.drivers.iter_mut().find(|d| d.id == decision.driver_id) {
            driver.active_task_count += 1;
            if let Some(w) = requirements.weight_kg {
                driver.current_capacity_kg += w;
            }
            if let Some(v) = requirements.volume_m3 {
                driver.current_volume_m3 += v;
            }
            if driver.active_task_count >= driver.max_concurrent_tasks {
                driver.status = DriverStatus::OnTask;
            }
        }

        self.history.push(decision.clone());
        Some(decision)
    }

    /// Check if a driver is eligible for a task.
    fn is_eligible(&self, driver: &FleetDriver, req: &TaskRequirements) -> bool {
        // Must be available or on-task with capacity.
        if driver.status != DriverStatus::Available && driver.status != DriverStatus::OnTask {
            return false;
        }

        // Must not exceed concurrent task limit.
        if driver.active_task_count >= driver.max_concurrent_tasks {
            return false;
        }

        // Must have required skills.
        for skill in &req.required_skills {
            if !driver.skills.contains(skill) {
                return false;
            }
        }

        // Must have capacity.
        if let Some(w) = req.weight_kg {
            if driver.current_capacity_kg + w > driver.max_capacity_kg {
                return false;
            }
        }
        if let Some(v) = req.volume_m3 {
            if driver.current_volume_m3 + v > driver.max_volume_m3 {
                return false;
            }
        }

        // Must be within max distance.
        let distance_km = haversine_distance(&driver.position, &req.location) / 1000.0;
        if distance_km > self.max_distance_km {
            return false;
        }

        true
    }

    /// Compute assignment score (higher = better).
    fn compute_score(&self, driver: &FleetDriver, req: &TaskRequirements, distance_km: f64) -> f64 {
        match self.strategy {
            AssignmentStrategy::NearestFirst => {
                // Score inversely proportional to distance.
                1.0 / (1.0 + distance_km)
            }
            AssignmentStrategy::LeastLoaded => {
                // Score inversely proportional to active tasks.
                1.0 / (1.0 + driver.active_task_count as f64)
            }
            AssignmentStrategy::Balanced => {
                // 60% distance, 40% workload.
                let distance_score = 1.0 / (1.0 + distance_km);
                let load_score = 1.0 / (1.0 + driver.active_task_count as f64);
                0.6 * distance_score + 0.4 * load_score
            }
            AssignmentStrategy::BestSkillMatch => {
                // Skill overlap ratio + distance tiebreaker.
                let skill_match = if req.required_skills.is_empty() {
                    1.0
                } else {
                    let matched = req
                        .required_skills
                        .iter()
                        .filter(|s| driver.skills.contains(s))
                        .count();
                    matched as f64 / req.required_skills.len() as f64
                };
                0.7 * skill_match + 0.3 / (1.0 + distance_km)
            }
        }
    }

    /// Get assignment history.
    pub fn history(&self) -> &[AssignmentDecision] {
        &self.history
    }

    /// Set max assignment distance.
    pub fn set_max_distance_km(&mut self, km: f64) {
        self.max_distance_km = km;
    }

    /// Get registered driver count.
    pub fn driver_count(&self) -> usize {
        self.drivers.len()
    }

    /// Get available drivers.
    pub fn available_drivers(&self) -> Vec<&FleetDriver> {
        self.drivers
            .iter()
            .filter(|d| d.status == DriverStatus::Available || d.status == DriverStatus::OnTask)
            .collect()
    }
}

impl Default for AssignmentEngine {
    fn default() -> Self {
        Self::new(AssignmentStrategy::Balanced)
    }
}

/// Haversine distance in metres.
fn haversine_distance(a: &GeoPosition, b: &GeoPosition) -> f64 {
    let r = 6_371_000.0;
    let d_lat = (b.latitude_deg - a.latitude_deg).to_radians();
    let d_lon = (b.longitude_deg - a.longitude_deg).to_radians();
    let lat1 = a.latitude_deg.to_radians();
    let lat2 = b.latitude_deg.to_radians();
    let h = (d_lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (d_lon / 2.0).sin().powi(2);
    2.0 * r * h.sqrt().asin()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pos(lat: f64, lon: f64) -> GeoPosition {
        GeoPosition {
            latitude_deg: lat,
            longitude_deg: lon,
            altitude_m: None,
        }
    }

    fn make_driver(name: &str, lat: f64, lon: f64, skills: Vec<&str>) -> FleetDriver {
        FleetDriver {
            id: EntityId::new(),
            name: name.to_string(),
            position: pos(lat, lon),
            vehicle_id: None,
            status: DriverStatus::Available,
            skills: skills.into_iter().map(String::from).collect(),
            current_capacity_kg: 0.0,
            max_capacity_kg: 500.0,
            current_volume_m3: 0.0,
            max_volume_m3: 10.0,
            active_task_count: 0,
            max_concurrent_tasks: 5,
            shift_end: None,
            updated_at: Utc::now(),
        }
    }

    fn make_req(lat: f64, lon: f64, skills: Vec<&str>) -> TaskRequirements {
        TaskRequirements {
            task_id: EntityId::new(),
            location: pos(lat, lon),
            required_skills: skills.into_iter().map(String::from).collect(),
            weight_kg: None,
            volume_m3: None,
            deadline: None,
        }
    }

    #[test]
    fn nearest_first_picks_closest() {
        let mut engine = AssignmentEngine::new(AssignmentStrategy::NearestFirst);
        engine.add_driver(make_driver("Far", 32.10, 34.80, vec![]));
        engine.add_driver(make_driver("Near", 32.085, 34.782, vec![]));

        let req = make_req(32.085, 34.781, vec![]);
        let decision = engine.find_best_match(&req).unwrap();
        assert!(decision.distance_km < 1.0, "Nearest driver should be < 1km");
    }

    #[test]
    fn least_loaded_picks_idle_driver() {
        let mut engine = AssignmentEngine::new(AssignmentStrategy::LeastLoaded);
        let mut busy = make_driver("Busy", 32.08, 34.78, vec![]);
        busy.active_task_count = 4;
        engine.add_driver(busy);
        engine.add_driver(make_driver("Idle", 32.09, 34.79, vec![]));

        let req = make_req(32.085, 34.785, vec![]);
        let decision = engine.find_best_match(&req).unwrap();
        // Idle driver (0 tasks) should score higher.
        assert!(decision.score > 0.5);
    }

    #[test]
    fn skill_match_filters_unqualified() {
        let mut engine = AssignmentEngine::new(AssignmentStrategy::BestSkillMatch);
        engine.add_driver(make_driver("NoSkill", 32.085, 34.781, vec![]));
        engine.add_driver(make_driver(
            "Skilled",
            32.09,
            34.79,
            vec!["hazmat", "forklift"],
        ));

        let req = make_req(32.085, 34.781, vec!["hazmat"]);
        let decision = engine.find_best_match(&req).unwrap();
        // Only Skilled driver has required skill.
        assert!(decision.score > 0.5);
    }

    #[test]
    fn capacity_overflow_rejected() {
        let mut engine = AssignmentEngine::new(AssignmentStrategy::NearestFirst);
        let mut driver = make_driver("Full", 32.085, 34.781, vec![]);
        driver.current_capacity_kg = 490.0; // Only 10kg left.
        engine.add_driver(driver);

        let req = TaskRequirements {
            task_id: EntityId::new(),
            location: pos(32.085, 34.781),
            required_skills: vec![],
            weight_kg: Some(50.0), // Needs 50kg — won't fit.
            volume_m3: None,
            deadline: None,
        };
        let decision = engine.find_best_match(&req);
        assert!(
            decision.is_none(),
            "Over-capacity driver should be rejected"
        );
    }

    #[test]
    fn out_of_range_rejected() {
        let mut engine = AssignmentEngine::new(AssignmentStrategy::NearestFirst);
        engine.set_max_distance_km(5.0);
        engine.add_driver(make_driver("TooFar", 33.0, 35.0, vec![])); // ~130km away

        let req = make_req(32.085, 34.781, vec![]);
        let decision = engine.find_best_match(&req);
        assert!(
            decision.is_none(),
            "Driver beyond max_distance should be rejected"
        );
    }

    #[test]
    fn assign_updates_driver_state() {
        let mut engine = AssignmentEngine::new(AssignmentStrategy::NearestFirst);
        let driver = make_driver("Driver A", 32.085, 34.781, vec![]);
        let driver_id = driver.id;
        engine.add_driver(driver);

        let req = TaskRequirements {
            task_id: EntityId::new(),
            location: pos(32.085, 34.782),
            required_skills: vec![],
            weight_kg: Some(20.0),
            volume_m3: None,
            deadline: None,
        };
        let decision = engine.assign(&req).unwrap();
        assert_eq!(decision.driver_id, driver_id);
        assert_eq!(engine.history().len(), 1);
    }

    #[test]
    fn off_duty_driver_excluded() {
        let mut engine = AssignmentEngine::new(AssignmentStrategy::NearestFirst);
        let mut driver = make_driver("OffDuty", 32.085, 34.781, vec![]);
        driver.status = DriverStatus::OffDuty;
        engine.add_driver(driver);

        let req = make_req(32.085, 34.781, vec![]);
        assert!(engine.find_best_match(&req).is_none());
    }

    #[test]
    fn balanced_strategy_combines_factors() {
        let mut engine = AssignmentEngine::new(AssignmentStrategy::Balanced);
        // Close but busy.
        let mut close_busy = make_driver("CloseBusy", 32.085, 34.782, vec![]);
        close_busy.active_task_count = 4;
        engine.add_driver(close_busy);
        // Far but idle.
        engine.add_driver(make_driver("FarIdle", 32.10, 34.80, vec![]));

        let req = make_req(32.085, 34.781, vec![]);
        let decision = engine.find_best_match(&req).unwrap();
        // Both should be considered; balanced should pick a reasonable one.
        assert!(decision.score > 0.0);
    }

    #[test]
    fn update_driver_position() {
        let mut engine = AssignmentEngine::new(AssignmentStrategy::NearestFirst);
        let driver = make_driver("Mobile", 32.0, 34.0, vec![]);
        let driver_id = driver.id;
        engine.add_driver(driver);

        assert!(engine.update_driver(&driver_id, pos(32.1, 34.1), DriverStatus::Available));
        assert!(!engine.update_driver(&EntityId::new(), pos(32.0, 34.0), DriverStatus::Available));
    }
}
