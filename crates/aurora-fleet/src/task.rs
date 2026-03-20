//! Fleet task management — create, assign, track, and complete fleet tasks
//! with time windows, constraints, and proof of completion.

use aurora_core::types::{EntityId, GeoPosition};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::debug;

/// Task priority level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Urgent,
    Critical,
}

/// Current state of a fleet task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    /// Created but not yet assigned.
    Pending,
    /// Assigned to a driver.
    Assigned,
    /// Driver is en route to the task location.
    EnRoute,
    /// Driver has arrived at the task location.
    Arrived,
    /// Task is being performed.
    InProgress,
    /// Task completed successfully.
    Completed,
    /// Task failed or could not be completed.
    Failed,
    /// Task was cancelled.
    Cancelled,
}

/// Type of proof required or provided for task completion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProofType {
    /// Photo evidence of visit/delivery.
    Photo,
    /// Signature capture.
    Signature,
    /// GPS confirmation (arrived within geofence).
    GpsConfirmation,
    /// Barcode or QR scan.
    BarcodeScan,
    /// Manual confirmation by driver.
    DriverConfirmation,
    /// Recipient confirmation.
    RecipientConfirmation,
}

/// Proof of visit or delivery.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofOfCompletion {
    pub id: EntityId,
    pub task_id: EntityId,
    pub proof_type: ProofType,
    pub timestamp: DateTime<Utc>,
    pub position: GeoPosition,
    /// Distance from task location in metres.
    pub distance_from_target_m: f64,
    /// Optional notes from driver.
    pub notes: Option<String>,
    /// Whether the proof was validated.
    pub validated: bool,
}

/// Time window constraint for a task.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimeWindow {
    pub earliest: DateTime<Utc>,
    pub latest: DateTime<Utc>,
}

impl TimeWindow {
    pub fn contains(&self, time: &DateTime<Utc>) -> bool {
        (&self.earliest..=&self.latest).contains(&time)
    }

    pub fn duration_minutes(&self) -> f64 {
        (self.latest - self.earliest).num_seconds() as f64 / 60.0
    }
}

/// Task constraints (hard and soft).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskConstraints {
    /// Hard time window — must be met.
    pub time_window: Option<TimeWindow>,
    /// Required vehicle type.
    pub required_vehicle_type: Option<String>,
    /// Required driver skills.
    pub required_skills: Vec<String>,
    /// Maximum weight capacity needed (kg).
    pub weight_kg: Option<f64>,
    /// Maximum volume needed (m^3).
    pub volume_m3: Option<f64>,
    /// Precedence — this task must be completed before these tasks.
    pub must_precede: Vec<EntityId>,
    /// Precedence — this task must be completed after these tasks.
    pub must_follow: Vec<EntityId>,
}

/// A fleet task (delivery, pickup, visit, service, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FleetTask {
    pub id: EntityId,
    pub title: String,
    pub description: Option<String>,
    pub task_type: TaskType,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub location: GeoPosition,
    pub location_name: Option<String>,
    pub constraints: TaskConstraints,
    pub assigned_driver_id: Option<EntityId>,
    pub assigned_vehicle_id: Option<EntityId>,
    pub proofs: Vec<ProofOfCompletion>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub estimated_duration_min: f64,
    pub actual_duration_min: Option<f64>,
}

/// Type of fleet task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskType {
    Delivery,
    Pickup,
    Visit,
    Service,
    Inspection,
    Emergency,
}

/// Task manager — CRUD operations and lifecycle management for fleet tasks.
pub struct TaskManager {
    tasks: Vec<FleetTask>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    /// Create a new task.
    pub fn create_task(
        &mut self,
        title: &str,
        task_type: TaskType,
        priority: TaskPriority,
        location: GeoPosition,
        constraints: TaskConstraints,
        estimated_duration_min: f64,
    ) -> EntityId {
        let id = EntityId::new();
        let now = Utc::now();
        let task = FleetTask {
            id,
            title: title.to_string(),
            description: None,
            task_type,
            priority,
            status: TaskStatus::Pending,
            location,
            location_name: None,
            constraints,
            assigned_driver_id: None,
            assigned_vehicle_id: None,
            proofs: Vec::new(),
            created_at: now,
            updated_at: now,
            started_at: None,
            completed_at: None,
            estimated_duration_min,
            actual_duration_min: None,
        };
        debug!(task_id = %id, title, "Task created");
        self.tasks.push(task);
        id
    }

    /// Get a task by ID.
    pub fn task(&self, id: &EntityId) -> Option<&FleetTask> {
        self.tasks.iter().find(|t| t.id == *id)
    }

    /// Get a mutable reference to a task by ID.
    pub fn task_mut(&mut self, id: &EntityId) -> Option<&mut FleetTask> {
        self.tasks.iter_mut().find(|t| t.id == *id)
    }

    /// Transition a task to a new status.
    pub fn transition(&mut self, task_id: &EntityId, new_status: TaskStatus) -> bool {
        let Some(task) = self.task_mut(task_id) else {
            return false;
        };

        // Validate transition.
        if !Self::is_valid_transition(task.status, new_status) {
            debug!(
                task_id = %task_id,
                from = ?task.status,
                to = ?new_status,
                "Invalid task status transition"
            );
            return false;
        }

        let now = Utc::now();
        task.status = new_status;
        task.updated_at = now;

        match new_status {
            TaskStatus::InProgress => {
                task.started_at = Some(now);
            }
            TaskStatus::Completed => {
                task.completed_at = Some(now);
                if let Some(started) = task.started_at {
                    task.actual_duration_min = Some((now - started).num_seconds() as f64 / 60.0);
                }
            }
            _ => {}
        }

        debug!(task_id = %task_id, status = ?new_status, "Task status changed");
        true
    }

    /// Check if a status transition is valid.
    fn is_valid_transition(from: TaskStatus, to: TaskStatus) -> bool {
        matches!(
            (from, to),
            (TaskStatus::Pending, TaskStatus::Assigned)
                | (TaskStatus::Pending, TaskStatus::Cancelled)
                | (TaskStatus::Assigned, TaskStatus::EnRoute)
                | (TaskStatus::Assigned, TaskStatus::Cancelled)
                | (TaskStatus::EnRoute, TaskStatus::Arrived)
                | (TaskStatus::EnRoute, TaskStatus::Cancelled)
                | (TaskStatus::Arrived, TaskStatus::InProgress)
                | (TaskStatus::InProgress, TaskStatus::Completed)
                | (TaskStatus::InProgress, TaskStatus::Failed)
        )
    }

    /// Add proof of completion to a task.
    pub fn add_proof(
        &mut self,
        task_id: &EntityId,
        proof_type: ProofType,
        position: GeoPosition,
    ) -> Option<EntityId> {
        let task = self.task_mut(task_id)?;
        let distance = haversine_distance(&task.location, &position);
        let proof_id = EntityId::new();
        let proof = ProofOfCompletion {
            id: proof_id,
            task_id: *task_id,
            proof_type,
            timestamp: Utc::now(),
            position,
            distance_from_target_m: distance,
            notes: None,
            validated: distance < 100.0, // Auto-validate if within 100m
        };
        debug!(
            task_id = %task_id,
            proof_type = ?proof_type,
            distance_m = distance,
            validated = proof.validated,
            "Proof of completion added"
        );
        task.proofs.push(proof);
        Some(proof_id)
    }

    /// Get tasks by status.
    pub fn tasks_by_status(&self, status: TaskStatus) -> Vec<&FleetTask> {
        self.tasks.iter().filter(|t| t.status == status).collect()
    }

    /// Get tasks assigned to a driver.
    pub fn tasks_for_driver(&self, driver_id: &EntityId) -> Vec<&FleetTask> {
        self.tasks
            .iter()
            .filter(|t| t.assigned_driver_id.as_ref() == Some(driver_id))
            .collect()
    }

    /// Get tasks by priority (sorted highest first).
    pub fn tasks_by_priority(&self) -> Vec<&FleetTask> {
        let mut tasks: Vec<&FleetTask> = self.tasks.iter().collect();
        tasks.sort_by(|a, b| b.priority.cmp(&a.priority));
        tasks
    }

    /// Get overdue tasks (past their time window latest).
    pub fn overdue_tasks(&self) -> Vec<&FleetTask> {
        let now = Utc::now();
        self.tasks
            .iter()
            .filter(|t| {
                t.status != TaskStatus::Completed
                    && t.status != TaskStatus::Cancelled
                    && t.status != TaskStatus::Failed
                    && t.constraints
                        .time_window
                        .as_ref()
                        .map_or(false, |tw| now > tw.latest)
            })
            .collect()
    }

    /// Total task count.
    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Haversine distance between two positions in metres.
fn haversine_distance(a: &GeoPosition, b: &GeoPosition) -> f64 {
    let r = 6_371_000.0; // Earth radius in metres
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

    #[test]
    fn create_and_query_task() {
        let mut mgr = TaskManager::new();
        let id = mgr.create_task(
            "Deliver package",
            TaskType::Delivery,
            TaskPriority::Normal,
            pos(32.0, 34.0),
            TaskConstraints::default(),
            30.0,
        );
        assert_eq!(mgr.task_count(), 1);
        let task = mgr.task(&id).unwrap();
        assert_eq!(task.title, "Deliver package");
        assert_eq!(task.status, TaskStatus::Pending);
        assert_eq!(task.priority, TaskPriority::Normal);
    }

    #[test]
    fn valid_task_transitions() {
        let mut mgr = TaskManager::new();
        let id = mgr.create_task(
            "Visit client",
            TaskType::Visit,
            TaskPriority::High,
            pos(32.0, 34.0),
            TaskConstraints::default(),
            15.0,
        );
        assert!(mgr.transition(&id, TaskStatus::Assigned));
        assert!(mgr.transition(&id, TaskStatus::EnRoute));
        assert!(mgr.transition(&id, TaskStatus::Arrived));
        assert!(mgr.transition(&id, TaskStatus::InProgress));
        assert!(mgr.transition(&id, TaskStatus::Completed));
        assert_eq!(mgr.task(&id).unwrap().status, TaskStatus::Completed);
        assert!(mgr.task(&id).unwrap().completed_at.is_some());
    }

    #[test]
    fn invalid_task_transition_rejected() {
        let mut mgr = TaskManager::new();
        let id = mgr.create_task(
            "Test task",
            TaskType::Service,
            TaskPriority::Low,
            pos(32.0, 34.0),
            TaskConstraints::default(),
            10.0,
        );
        // Cannot go directly from Pending to Completed.
        assert!(!mgr.transition(&id, TaskStatus::Completed));
        assert_eq!(mgr.task(&id).unwrap().status, TaskStatus::Pending);
    }

    #[test]
    fn cancel_pending_task() {
        let mut mgr = TaskManager::new();
        let id = mgr.create_task(
            "Cancelled task",
            TaskType::Pickup,
            TaskPriority::Normal,
            pos(32.0, 34.0),
            TaskConstraints::default(),
            20.0,
        );
        assert!(mgr.transition(&id, TaskStatus::Cancelled));
        assert_eq!(mgr.task(&id).unwrap().status, TaskStatus::Cancelled);
    }

    #[test]
    fn proof_of_completion_auto_validates_nearby() {
        let mut mgr = TaskManager::new();
        let task_pos = pos(32.0853, 34.7818); // Tel Aviv
        let id = mgr.create_task(
            "Delivery",
            TaskType::Delivery,
            TaskPriority::Normal,
            task_pos,
            TaskConstraints::default(),
            30.0,
        );
        // Proof from very close (within 100m).
        let proof_id = mgr.add_proof(&id, ProofType::GpsConfirmation, pos(32.0854, 34.7819));
        assert!(proof_id.is_some());
        let task = mgr.task(&id).unwrap();
        assert!(task.proofs[0].validated);
        assert!(task.proofs[0].distance_from_target_m < 100.0);
    }

    #[test]
    fn proof_of_completion_rejects_far_away() {
        let mut mgr = TaskManager::new();
        let id = mgr.create_task(
            "Delivery",
            TaskType::Delivery,
            TaskPriority::Normal,
            pos(32.0853, 34.7818),
            TaskConstraints::default(),
            30.0,
        );
        // Proof from far away (> 100m).
        let proof_id = mgr.add_proof(&id, ProofType::GpsConfirmation, pos(32.10, 34.80));
        assert!(proof_id.is_some());
        let task = mgr.task(&id).unwrap();
        assert!(!task.proofs[0].validated);
    }

    #[test]
    fn tasks_by_status_filter() {
        let mut mgr = TaskManager::new();
        let id1 = mgr.create_task(
            "Task 1",
            TaskType::Delivery,
            TaskPriority::Normal,
            pos(32.0, 34.0),
            TaskConstraints::default(),
            30.0,
        );
        mgr.create_task(
            "Task 2",
            TaskType::Pickup,
            TaskPriority::High,
            pos(32.1, 34.1),
            TaskConstraints::default(),
            20.0,
        );
        mgr.transition(&id1, TaskStatus::Assigned);

        assert_eq!(mgr.tasks_by_status(TaskStatus::Pending).len(), 1);
        assert_eq!(mgr.tasks_by_status(TaskStatus::Assigned).len(), 1);
    }

    #[test]
    fn tasks_by_priority_sorted() {
        let mut mgr = TaskManager::new();
        mgr.create_task(
            "Low",
            TaskType::Visit,
            TaskPriority::Low,
            pos(32.0, 34.0),
            TaskConstraints::default(),
            10.0,
        );
        mgr.create_task(
            "Critical",
            TaskType::Emergency,
            TaskPriority::Critical,
            pos(32.1, 34.1),
            TaskConstraints::default(),
            5.0,
        );
        mgr.create_task(
            "Normal",
            TaskType::Service,
            TaskPriority::Normal,
            pos(32.2, 34.2),
            TaskConstraints::default(),
            15.0,
        );

        let sorted = mgr.tasks_by_priority();
        assert_eq!(sorted[0].title, "Critical");
        assert_eq!(sorted[1].title, "Normal");
        assert_eq!(sorted[2].title, "Low");
    }

    #[test]
    fn tasks_for_driver_filter() {
        let mut mgr = TaskManager::new();
        let driver_id = EntityId::new();
        let id1 = mgr.create_task(
            "Driver A task",
            TaskType::Delivery,
            TaskPriority::Normal,
            pos(32.0, 34.0),
            TaskConstraints::default(),
            30.0,
        );
        mgr.create_task(
            "Unassigned",
            TaskType::Pickup,
            TaskPriority::Normal,
            pos(32.1, 34.1),
            TaskConstraints::default(),
            20.0,
        );
        mgr.task_mut(&id1).unwrap().assigned_driver_id = Some(driver_id);

        let driver_tasks = mgr.tasks_for_driver(&driver_id);
        assert_eq!(driver_tasks.len(), 1);
        assert_eq!(driver_tasks[0].title, "Driver A task");
    }

    #[test]
    fn time_window_contains() {
        let now = Utc::now();
        let tw = TimeWindow {
            earliest: now - chrono::Duration::hours(1),
            latest: now + chrono::Duration::hours(1),
        };
        assert!(tw.contains(&now));
        let past = now - chrono::Duration::hours(2);
        assert!(!tw.contains(&past));
    }

    #[test]
    fn overdue_task_detection() {
        let mut mgr = TaskManager::new();
        let past_window = TimeWindow {
            earliest: Utc::now() - chrono::Duration::hours(3),
            latest: Utc::now() - chrono::Duration::hours(1),
        };
        let constraints = TaskConstraints {
            time_window: Some(past_window),
            ..TaskConstraints::default()
        };
        mgr.create_task(
            "Overdue task",
            TaskType::Delivery,
            TaskPriority::High,
            pos(32.0, 34.0),
            constraints,
            30.0,
        );

        let overdue = mgr.overdue_tasks();
        assert_eq!(overdue.len(), 1);
        assert_eq!(overdue[0].title, "Overdue task");
    }

    #[test]
    fn failed_task_transition() {
        let mut mgr = TaskManager::new();
        let id = mgr.create_task(
            "Failing task",
            TaskType::Service,
            TaskPriority::Normal,
            pos(32.0, 34.0),
            TaskConstraints::default(),
            10.0,
        );
        mgr.transition(&id, TaskStatus::Assigned);
        mgr.transition(&id, TaskStatus::EnRoute);
        mgr.transition(&id, TaskStatus::Arrived);
        mgr.transition(&id, TaskStatus::InProgress);
        assert!(mgr.transition(&id, TaskStatus::Failed));
        assert_eq!(mgr.task(&id).unwrap().status, TaskStatus::Failed);
    }
}
