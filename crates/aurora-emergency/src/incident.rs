//! Mass incident management — coordinates multi-agency response, resource
//! allocation, incident zones, and communication for large-scale emergencies.

use aurora_core::types::{EntityId, GeoPosition};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::debug;

/// Incident severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum IncidentSeverity {
    /// Minor — single unit response.
    Minor,
    /// Moderate — multiple units, single agency.
    Moderate,
    /// Major — multi-agency response.
    Major,
    /// Critical — mass casualty / disaster.
    Critical,
}

/// Incident type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncidentType {
    TrafficAccident,
    Fire,
    MedicalEmergency,
    HazmatSpill,
    NaturalDisaster,
    StructuralCollapse,
    MassCasualty,
    SecurityIncident,
    InfrastructureFailure,
}

/// Incident zone classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZoneType {
    /// Hot zone — immediate danger, restricted access.
    Hot,
    /// Warm zone — decontamination/staging, limited access.
    Warm,
    /// Cold zone — command post and logistics, open access.
    Cold,
    /// Perimeter — public exclusion boundary.
    Perimeter,
}

/// An incident zone with geographic boundary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentZone {
    pub id: EntityId,
    pub zone_type: ZoneType,
    pub center: GeoPosition,
    pub radius_m: f64,
    pub description: Option<String>,
}

/// A resource deployed to an incident.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployedResource {
    pub id: EntityId,
    pub resource_type: ResourceType,
    pub unit_name: String,
    pub position: GeoPosition,
    pub status: ResourceStatus,
    pub assigned_zone: Option<EntityId>,
    pub deployed_at: DateTime<Utc>,
}

/// Type of emergency resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceType {
    Ambulance,
    FireEngine,
    PoliceCar,
    HazmatUnit,
    CommandPost,
    MedicalTeam,
    SearchAndRescue,
    Helicopter,
}

/// Resource deployment status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceStatus {
    Dispatched,
    EnRoute,
    OnScene,
    Operational,
    Returning,
    OutOfService,
}

/// Incident status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncidentStatus {
    Reported,
    Confirmed,
    Active,
    Contained,
    Resolved,
    Closed,
}

/// A mass incident record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassIncident {
    pub id: EntityId,
    pub incident_type: IncidentType,
    pub severity: IncidentSeverity,
    pub status: IncidentStatus,
    pub location: GeoPosition,
    pub description: String,
    pub zones: Vec<IncidentZone>,
    pub resources: Vec<DeployedResource>,
    pub affected_roads: Vec<EntityId>,
    pub reported_at: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub commander_id: Option<EntityId>,
    pub casualty_count: u32,
    pub evacuees_count: u32,
}

/// Incident manager — creates and coordinates mass incident responses.
pub struct IncidentManager {
    incidents: Vec<MassIncident>,
}

impl IncidentManager {
    pub fn new() -> Self {
        Self {
            incidents: Vec::new(),
        }
    }

    /// Report a new incident.
    pub fn report_incident(
        &mut self,
        incident_type: IncidentType,
        severity: IncidentSeverity,
        location: GeoPosition,
        description: &str,
    ) -> EntityId {
        let id = EntityId::new();
        let incident = MassIncident {
            id,
            incident_type,
            severity,
            status: IncidentStatus::Reported,
            location,
            description: description.to_string(),
            zones: Vec::new(),
            resources: Vec::new(),
            affected_roads: Vec::new(),
            reported_at: Utc::now(),
            confirmed_at: None,
            resolved_at: None,
            commander_id: None,
            casualty_count: 0,
            evacuees_count: 0,
        };
        debug!(
            incident_id = %id,
            incident_type = ?incident_type,
            severity = ?severity,
            "Incident reported"
        );
        self.incidents.push(incident);
        id
    }

    /// Confirm an incident and optionally upgrade severity.
    pub fn confirm_incident(
        &mut self,
        incident_id: &EntityId,
        severity: Option<IncidentSeverity>,
    ) -> bool {
        let Some(incident) = self.incidents.iter_mut().find(|i| i.id == *incident_id) else {
            return false;
        };
        if incident.status != IncidentStatus::Reported {
            return false;
        }
        incident.status = IncidentStatus::Confirmed;
        incident.confirmed_at = Some(Utc::now());
        if let Some(s) = severity {
            incident.severity = s;
        }
        debug!(incident_id = %incident_id, severity = ?incident.severity, "Incident confirmed");
        true
    }

    /// Activate incident response.
    pub fn activate_incident(&mut self, incident_id: &EntityId) -> bool {
        let Some(incident) = self.incidents.iter_mut().find(|i| i.id == *incident_id) else {
            return false;
        };
        if incident.status != IncidentStatus::Confirmed {
            return false;
        }
        incident.status = IncidentStatus::Active;
        true
    }

    /// Add a zone to an incident.
    pub fn add_zone(
        &mut self,
        incident_id: &EntityId,
        zone_type: ZoneType,
        center: GeoPosition,
        radius_m: f64,
    ) -> Option<EntityId> {
        let incident = self.incidents.iter_mut().find(|i| i.id == *incident_id)?;
        let zone_id = EntityId::new();
        incident.zones.push(IncidentZone {
            id: zone_id,
            zone_type,
            center,
            radius_m,
            description: None,
        });
        debug!(
            incident_id = %incident_id,
            zone_type = ?zone_type,
            radius_m,
            "Zone added to incident"
        );
        Some(zone_id)
    }

    /// Deploy a resource to an incident.
    pub fn deploy_resource(
        &mut self,
        incident_id: &EntityId,
        resource_type: ResourceType,
        unit_name: &str,
        position: GeoPosition,
    ) -> Option<EntityId> {
        let incident = self.incidents.iter_mut().find(|i| i.id == *incident_id)?;
        let res_id = EntityId::new();
        incident.resources.push(DeployedResource {
            id: res_id,
            resource_type,
            unit_name: unit_name.to_string(),
            position,
            status: ResourceStatus::Dispatched,
            assigned_zone: None,
            deployed_at: Utc::now(),
        });
        debug!(
            incident_id = %incident_id,
            resource = ?resource_type,
            unit = unit_name,
            "Resource deployed"
        );
        Some(res_id)
    }

    /// Update resource status.
    pub fn update_resource_status(
        &mut self,
        incident_id: &EntityId,
        resource_id: &EntityId,
        status: ResourceStatus,
    ) -> bool {
        let Some(incident) = self.incidents.iter_mut().find(|i| i.id == *incident_id) else {
            return false;
        };
        let Some(resource) = incident.resources.iter_mut().find(|r| r.id == *resource_id) else {
            return false;
        };
        resource.status = status;
        true
    }

    /// Assign a resource to a zone.
    pub fn assign_resource_to_zone(
        &mut self,
        incident_id: &EntityId,
        resource_id: &EntityId,
        zone_id: EntityId,
    ) -> bool {
        let Some(incident) = self.incidents.iter_mut().find(|i| i.id == *incident_id) else {
            return false;
        };
        // Verify zone exists.
        if !incident.zones.iter().any(|z| z.id == zone_id) {
            return false;
        }
        let Some(resource) = incident.resources.iter_mut().find(|r| r.id == *resource_id) else {
            return false;
        };
        resource.assigned_zone = Some(zone_id);
        true
    }

    /// Update casualty count.
    pub fn update_casualties(
        &mut self,
        incident_id: &EntityId,
        casualties: u32,
        evacuees: u32,
    ) -> bool {
        let Some(incident) = self.incidents.iter_mut().find(|i| i.id == *incident_id) else {
            return false;
        };
        incident.casualty_count = casualties;
        incident.evacuees_count = evacuees;
        true
    }

    /// Contain an incident.
    pub fn contain_incident(&mut self, incident_id: &EntityId) -> bool {
        let Some(incident) = self.incidents.iter_mut().find(|i| i.id == *incident_id) else {
            return false;
        };
        if incident.status != IncidentStatus::Active {
            return false;
        }
        incident.status = IncidentStatus::Contained;
        true
    }

    /// Resolve an incident.
    pub fn resolve_incident(&mut self, incident_id: &EntityId) -> bool {
        let Some(incident) = self.incidents.iter_mut().find(|i| i.id == *incident_id) else {
            return false;
        };
        if incident.status != IncidentStatus::Contained && incident.status != IncidentStatus::Active
        {
            return false;
        }
        incident.status = IncidentStatus::Resolved;
        incident.resolved_at = Some(Utc::now());
        debug!(incident_id = %incident_id, "Incident resolved");
        true
    }

    /// Get an incident by ID.
    pub fn incident(&self, id: &EntityId) -> Option<&MassIncident> {
        self.incidents.iter().find(|i| i.id == *id)
    }

    /// Get active incidents.
    pub fn active_incidents(&self) -> Vec<&MassIncident> {
        self.incidents
            .iter()
            .filter(|i| i.status == IncidentStatus::Active || i.status == IncidentStatus::Confirmed)
            .collect()
    }

    /// Resources on scene for an incident.
    pub fn resources_on_scene(&self, incident_id: &EntityId) -> usize {
        self.incident(incident_id).map_or(0, |i| {
            i.resources
                .iter()
                .filter(|r| {
                    r.status == ResourceStatus::OnScene || r.status == ResourceStatus::Operational
                })
                .count()
        })
    }

    /// Total incident count.
    pub fn incident_count(&self) -> usize {
        self.incidents.len()
    }
}

impl Default for IncidentManager {
    fn default() -> Self {
        Self::new()
    }
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
    fn report_and_confirm_incident() {
        let mut mgr = IncidentManager::new();
        let id = mgr.report_incident(
            IncidentType::TrafficAccident,
            IncidentSeverity::Moderate,
            pos(32.0, 34.0),
            "Multi-vehicle collision on Highway 1",
        );
        assert_eq!(mgr.incident(&id).unwrap().status, IncidentStatus::Reported);

        assert!(mgr.confirm_incident(&id, Some(IncidentSeverity::Major)));
        let inc = mgr.incident(&id).unwrap();
        assert_eq!(inc.status, IncidentStatus::Confirmed);
        assert_eq!(inc.severity, IncidentSeverity::Major);
    }

    #[test]
    fn full_incident_lifecycle() {
        let mut mgr = IncidentManager::new();
        let id = mgr.report_incident(
            IncidentType::Fire,
            IncidentSeverity::Critical,
            pos(32.0, 34.0),
            "Industrial fire",
        );
        mgr.confirm_incident(&id, None);
        mgr.activate_incident(&id);
        mgr.contain_incident(&id);
        mgr.resolve_incident(&id);

        let inc = mgr.incident(&id).unwrap();
        assert_eq!(inc.status, IncidentStatus::Resolved);
        assert!(inc.resolved_at.is_some());
    }

    #[test]
    fn add_zones_to_incident() {
        let mut mgr = IncidentManager::new();
        let id = mgr.report_incident(
            IncidentType::HazmatSpill,
            IncidentSeverity::Major,
            pos(32.0, 34.0),
            "Chemical spill",
        );

        let hot_zone = mgr.add_zone(&id, ZoneType::Hot, pos(32.0, 34.0), 100.0);
        let warm_zone = mgr.add_zone(&id, ZoneType::Warm, pos(32.0, 34.0), 300.0);
        let cold_zone = mgr.add_zone(&id, ZoneType::Cold, pos(32.0, 34.0), 500.0);

        assert!(hot_zone.is_some());
        assert!(warm_zone.is_some());
        assert!(cold_zone.is_some());
        assert_eq!(mgr.incident(&id).unwrap().zones.len(), 3);
    }

    #[test]
    fn deploy_and_track_resources() {
        let mut mgr = IncidentManager::new();
        let id = mgr.report_incident(
            IncidentType::MassCasualty,
            IncidentSeverity::Critical,
            pos(32.0, 34.0),
            "Mass casualty event",
        );
        mgr.confirm_incident(&id, None);
        mgr.activate_incident(&id);

        let res1 = mgr
            .deploy_resource(&id, ResourceType::Ambulance, "MADA-1", pos(32.01, 34.01))
            .unwrap();
        let res2 = mgr
            .deploy_resource(&id, ResourceType::FireEngine, "FIRE-3", pos(32.02, 34.02))
            .unwrap();

        assert_eq!(mgr.incident(&id).unwrap().resources.len(), 2);

        // Update to OnScene.
        mgr.update_resource_status(&id, &res1, ResourceStatus::OnScene);
        mgr.update_resource_status(&id, &res2, ResourceStatus::OnScene);
        assert_eq!(mgr.resources_on_scene(&id), 2);
    }

    #[test]
    fn assign_resource_to_zone() {
        let mut mgr = IncidentManager::new();
        let id = mgr.report_incident(
            IncidentType::Fire,
            IncidentSeverity::Major,
            pos(32.0, 34.0),
            "Building fire",
        );
        let zone_id = mgr
            .add_zone(&id, ZoneType::Hot, pos(32.0, 34.0), 50.0)
            .unwrap();
        let res_id = mgr
            .deploy_resource(&id, ResourceType::FireEngine, "FIRE-1", pos(32.01, 34.01))
            .unwrap();

        assert!(mgr.assign_resource_to_zone(&id, &res_id, zone_id));
        let resource = &mgr.incident(&id).unwrap().resources[0];
        assert_eq!(resource.assigned_zone, Some(zone_id));
    }

    #[test]
    fn invalid_zone_assignment_rejected() {
        let mut mgr = IncidentManager::new();
        let id = mgr.report_incident(
            IncidentType::Fire,
            IncidentSeverity::Minor,
            pos(32.0, 34.0),
            "Small fire",
        );
        let res_id = mgr
            .deploy_resource(&id, ResourceType::FireEngine, "FIRE-1", pos(32.0, 34.0))
            .unwrap();
        let fake_zone = EntityId::new();
        assert!(!mgr.assign_resource_to_zone(&id, &res_id, fake_zone));
    }

    #[test]
    fn update_casualties() {
        let mut mgr = IncidentManager::new();
        let id = mgr.report_incident(
            IncidentType::MassCasualty,
            IncidentSeverity::Critical,
            pos(32.0, 34.0),
            "Event",
        );
        mgr.update_casualties(&id, 15, 200);
        let inc = mgr.incident(&id).unwrap();
        assert_eq!(inc.casualty_count, 15);
        assert_eq!(inc.evacuees_count, 200);
    }

    #[test]
    fn active_incidents_filter() {
        let mut mgr = IncidentManager::new();
        let id1 = mgr.report_incident(
            IncidentType::TrafficAccident,
            IncidentSeverity::Minor,
            pos(32.0, 34.0),
            "Accident 1",
        );
        let id2 = mgr.report_incident(
            IncidentType::Fire,
            IncidentSeverity::Major,
            pos(32.1, 34.1),
            "Fire",
        );
        mgr.confirm_incident(&id1, None);
        mgr.activate_incident(&id1);
        mgr.confirm_incident(&id2, None);

        // id1 = Active, id2 = Confirmed → both in active_incidents.
        assert_eq!(mgr.active_incidents().len(), 2);
    }

    #[test]
    fn cannot_resolve_unreported_incident() {
        let mut mgr = IncidentManager::new();
        let id = mgr.report_incident(
            IncidentType::Fire,
            IncidentSeverity::Minor,
            pos(32.0, 34.0),
            "Small fire",
        );
        // Cannot resolve directly from Reported.
        assert!(!mgr.resolve_incident(&id));
    }

    #[test]
    fn severity_ordering() {
        assert!(IncidentSeverity::Critical > IncidentSeverity::Major);
        assert!(IncidentSeverity::Major > IncidentSeverity::Moderate);
        assert!(IncidentSeverity::Moderate > IncidentSeverity::Minor);
    }
}
