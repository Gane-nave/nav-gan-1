//! Incident reporter — create, validate, and manage incident reports with evidence.

use aurora_core::incident::{
    Evidence, EvidenceType, Incident, IncidentSeverity, IncidentStatus, IncidentTimeline,
    IncidentType, TimelineEntry,
};
use aurora_core::types::{EntityId, GeoPosition};
use chrono::Utc;
use std::collections::{HashMap, HashSet};
use tracing::{debug, info};

/// Request to create a new incident report.
#[derive(Debug, Clone)]
pub struct IncidentReport {
    pub reporter_id: EntityId,
    pub position: GeoPosition,
    pub incident_type: IncidentType,
    pub severity: IncidentSeverity,
    pub description: Option<String>,
    /// Optional heading in degrees when the report was filed.
    pub heading_deg: Option<f64>,
}

/// Request to attach evidence to an incident.
#[derive(Debug, Clone)]
pub struct EvidenceAttachment {
    pub reporter_id: EntityId,
    pub evidence_type: EvidenceType,
    pub position: GeoPosition,
    pub heading_deg: Option<f64>,
    pub media_url: Option<String>,
    pub media_hash: Option<String>,
}

/// Manages incident lifecycle: creation, validation, evidence attachment, closure.
pub struct IncidentReporter {
    incidents: HashMap<EntityId, Incident>,
    timelines: HashMap<EntityId, IncidentTimeline>,
    evidence: HashMap<EntityId, Evidence>,
    /// Tracks which users have already validated each incident.
    validated_by: HashMap<EntityId, HashSet<EntityId>>,
    /// Minimum validations before an incident is promoted to Active.
    validation_threshold: u32,
}

impl IncidentReporter {
    pub fn new() -> Self {
        Self {
            incidents: HashMap::new(),
            timelines: HashMap::new(),
            evidence: HashMap::new(),
            validated_by: HashMap::new(),
            validation_threshold: 3,
        }
    }

    /// Set the number of validations required to promote an incident to Active.
    pub fn with_validation_threshold(mut self, threshold: u32) -> Self {
        self.validation_threshold = threshold;
        self
    }

    /// Create a new incident from a report.
    pub fn create_incident(&mut self, report: IncidentReport) -> EntityId {
        let now = Utc::now();
        let id = EntityId::new();

        let incident = Incident {
            id,
            reporter_id: report.reporter_id,
            position: report.position,
            incident_type: report.incident_type,
            severity: report.severity,
            status: IncidentStatus::Reported,
            description: report.description.clone(),
            evidence_ids: Vec::new(),
            trust_score: 0.5, // neutral initial trust
            validation_count: 0,
            created_at: now,
            updated_at: now,
            closed_at: None,
        };

        let timeline = IncidentTimeline {
            incident_id: id,
            entries: vec![TimelineEntry {
                timestamp: now,
                event: "Incident reported".into(),
                actor_id: Some(report.reporter_id),
                details: report.description,
            }],
        };

        info!(incident_id = %id, incident_type = ?report.incident_type, "incident created");

        self.incidents.insert(id, incident);
        self.timelines.insert(id, timeline);
        id
    }

    /// Attach evidence to an existing incident.
    pub fn attach_evidence(
        &mut self,
        incident_id: EntityId,
        attachment: EvidenceAttachment,
    ) -> Option<EntityId> {
        let incident = self.incidents.get_mut(&incident_id)?;
        let now = Utc::now();
        let evidence_id = EntityId::new();

        let evidence = Evidence {
            id: evidence_id,
            incident_id: Some(incident_id),
            reporter_id: attachment.reporter_id,
            evidence_type: attachment.evidence_type,
            position: attachment.position,
            heading_deg: attachment.heading_deg,
            timestamp: now,
            media_url: attachment.media_url,
            media_hash: attachment.media_hash,
            signed_metadata: None,
            privacy_processed: false,
            encrypted: false,
        };

        incident.evidence_ids.push(evidence_id);
        incident.updated_at = now;

        if let Some(timeline) = self.timelines.get_mut(&incident_id) {
            timeline.entries.push(TimelineEntry {
                timestamp: now,
                event: format!("Evidence attached: {:?}", attachment.evidence_type),
                actor_id: Some(attachment.reporter_id),
                details: None,
            });
        }

        debug!(incident_id = %incident_id, evidence_id = %evidence_id, "evidence attached");

        self.evidence.insert(evidence_id, evidence);
        Some(evidence_id)
    }

    /// Validate an incident (another user confirms it).
    pub fn validate_incident(
        &mut self,
        incident_id: EntityId,
        validator_id: EntityId,
    ) -> Option<IncidentStatus> {
        let incident = self.incidents.get_mut(&incident_id)?;

        // Cannot validate your own incident.
        if incident.reporter_id == validator_id {
            return Some(incident.status);
        }

        // Reject duplicate validations from the same user.
        let validators = self.validated_by.entry(incident_id).or_default();
        if !validators.insert(validator_id) {
            return Some(incident.status);
        }

        incident.validation_count += 1;
        incident.updated_at = Utc::now();

        // Promote to Validated after reaching threshold.
        if incident.validation_count >= self.validation_threshold
            && incident.status == IncidentStatus::Reported
        {
            incident.status = IncidentStatus::Validated;
            incident.trust_score = (incident.trust_score + 0.1).min(1.0);

            if let Some(timeline) = self.timelines.get_mut(&incident_id) {
                timeline.entries.push(TimelineEntry {
                    timestamp: Utc::now(),
                    event: "Incident validated by community".into(),
                    actor_id: Some(validator_id),
                    details: Some(format!("Validation count: {}", incident.validation_count)),
                });
            }

            info!(incident_id = %incident_id, validations = incident.validation_count, "incident validated");
        }

        Some(incident.status)
    }

    /// Close an incident.
    pub fn close_incident(
        &mut self,
        incident_id: EntityId,
        actor_id: EntityId,
        reason: &str,
    ) -> bool {
        if let Some(incident) = self.incidents.get_mut(&incident_id) {
            let now = Utc::now();
            incident.status = IncidentStatus::Closed;
            incident.closed_at = Some(now);
            incident.updated_at = now;

            if let Some(timeline) = self.timelines.get_mut(&incident_id) {
                timeline.entries.push(TimelineEntry {
                    timestamp: now,
                    event: "Incident closed".into(),
                    actor_id: Some(actor_id),
                    details: Some(reason.into()),
                });
            }

            info!(incident_id = %incident_id, "incident closed");
            true
        } else {
            false
        }
    }

    /// Reject an incident (false report).
    pub fn reject_incident(&mut self, incident_id: EntityId, actor_id: EntityId) -> bool {
        if let Some(incident) = self.incidents.get_mut(&incident_id) {
            let now = Utc::now();
            incident.status = IncidentStatus::Rejected;
            incident.trust_score = (incident.trust_score - 0.2).max(0.0);
            incident.updated_at = now;

            if let Some(timeline) = self.timelines.get_mut(&incident_id) {
                timeline.entries.push(TimelineEntry {
                    timestamp: now,
                    event: "Incident rejected".into(),
                    actor_id: Some(actor_id),
                    details: None,
                });
            }

            info!(incident_id = %incident_id, "incident rejected");
            true
        } else {
            false
        }
    }

    /// Get an incident by ID.
    pub fn get_incident(&self, id: &EntityId) -> Option<&Incident> {
        self.incidents.get(id)
    }

    /// Get an incident timeline.
    pub fn get_timeline(&self, incident_id: &EntityId) -> Option<&IncidentTimeline> {
        self.timelines.get(incident_id)
    }

    /// Get evidence by ID.
    pub fn get_evidence(&self, id: &EntityId) -> Option<&Evidence> {
        self.evidence.get(id)
    }

    /// Count of active incidents.
    pub fn active_count(&self) -> usize {
        self.incidents
            .values()
            .filter(|i| i.status != IncidentStatus::Closed && i.status != IncidentStatus::Rejected)
            .count()
    }

    /// Get incidents near a position within a radius in metres.
    pub fn incidents_near(&self, position: &GeoPosition, radius_m: f64) -> Vec<&Incident> {
        self.incidents
            .values()
            .filter(|i| {
                let dist = haversine_m(position, &i.position);
                dist <= radius_m
                    && i.status != IncidentStatus::Closed
                    && i.status != IncidentStatus::Rejected
            })
            .collect()
    }
}

impl Default for IncidentReporter {
    fn default() -> Self {
        Self::new()
    }
}

/// Haversine distance in metres.
fn haversine_m(a: &GeoPosition, b: &GeoPosition) -> f64 {
    let r = 6_371_000.0;
    let d_lat = (b.latitude_deg - a.latitude_deg).to_radians();
    let d_lon = (b.longitude_deg - a.longitude_deg).to_radians();
    let lat1 = a.latitude_deg.to_radians();
    let lat2 = b.latitude_deg.to_radians();

    let a_val = (d_lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (d_lon / 2.0).sin().powi(2);
    let c = 2.0 * a_val.sqrt().asin();
    r * c
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

    fn report(reporter: EntityId) -> IncidentReport {
        IncidentReport {
            reporter_id: reporter,
            position: pos(32.08, 34.78),
            incident_type: IncidentType::Accident,
            severity: IncidentSeverity::Medium,
            description: Some("Two-car collision".into()),
            heading_deg: Some(90.0),
        }
    }

    #[test]
    fn create_and_retrieve_incident() {
        let mut reporter = IncidentReporter::new();
        let user = EntityId::new();
        let id = reporter.create_incident(report(user));

        let incident = reporter.get_incident(&id).unwrap();
        assert_eq!(incident.status, IncidentStatus::Reported);
        assert_eq!(incident.incident_type, IncidentType::Accident);
        assert_eq!(incident.validation_count, 0);
    }

    #[test]
    fn attach_evidence_to_incident() {
        let mut reporter = IncidentReporter::new();
        let user = EntityId::new();
        let id = reporter.create_incident(report(user));

        let ev_id = reporter
            .attach_evidence(
                id,
                EvidenceAttachment {
                    reporter_id: user,
                    evidence_type: EvidenceType::Photo,
                    position: pos(32.08, 34.78),
                    heading_deg: Some(90.0),
                    media_url: Some("https://evidence.example.com/photo1.jpg".into()),
                    media_hash: Some("sha256:abc123".into()),
                },
            )
            .unwrap();

        let incident = reporter.get_incident(&id).unwrap();
        assert_eq!(incident.evidence_ids.len(), 1);

        let evidence = reporter.get_evidence(&ev_id).unwrap();
        assert_eq!(evidence.evidence_type, EvidenceType::Photo);
    }

    #[test]
    fn validation_promotes_status() {
        let mut reporter = IncidentReporter::new().with_validation_threshold(2);
        let user = EntityId::new();
        let id = reporter.create_incident(report(user));

        let v1 = EntityId::new();
        let v2 = EntityId::new();

        reporter.validate_incident(id, v1);
        let incident = reporter.get_incident(&id).unwrap();
        assert_eq!(incident.status, IncidentStatus::Reported);

        reporter.validate_incident(id, v2);
        let incident = reporter.get_incident(&id).unwrap();
        assert_eq!(incident.status, IncidentStatus::Validated);
    }

    #[test]
    fn duplicate_validation_rejected() {
        let mut reporter = IncidentReporter::new().with_validation_threshold(2);
        let user = EntityId::new();
        let id = reporter.create_incident(report(user));

        let validator = EntityId::new();

        // First validation counts.
        reporter.validate_incident(id, validator);
        assert_eq!(reporter.get_incident(&id).unwrap().validation_count, 1);

        // Same validator again — should be rejected (no increment).
        reporter.validate_incident(id, validator);
        assert_eq!(reporter.get_incident(&id).unwrap().validation_count, 1);
        assert_eq!(
            reporter.get_incident(&id).unwrap().status,
            IncidentStatus::Reported
        ); // not promoted

        // Different validator promotes.
        let v2 = EntityId::new();
        reporter.validate_incident(id, v2);
        assert_eq!(reporter.get_incident(&id).unwrap().validation_count, 2);
        assert_eq!(
            reporter.get_incident(&id).unwrap().status,
            IncidentStatus::Validated
        );
    }

    #[test]
    fn self_validation_rejected() {
        let mut reporter = IncidentReporter::new().with_validation_threshold(1);
        let user = EntityId::new();
        let id = reporter.create_incident(report(user));

        reporter.validate_incident(id, user); // self-validation
        let incident = reporter.get_incident(&id).unwrap();
        assert_eq!(incident.status, IncidentStatus::Reported); // not promoted
    }

    #[test]
    fn close_and_reject_incidents() {
        let mut reporter = IncidentReporter::new();
        let user = EntityId::new();
        let admin = EntityId::new();

        let id1 = reporter.create_incident(report(user));
        let id2 = reporter.create_incident(report(user));

        reporter.close_incident(id1, admin, "Resolved");
        reporter.reject_incident(id2, admin);

        assert_eq!(
            reporter.get_incident(&id1).unwrap().status,
            IncidentStatus::Closed
        );
        assert_eq!(
            reporter.get_incident(&id2).unwrap().status,
            IncidentStatus::Rejected
        );
        assert_eq!(reporter.active_count(), 0);
    }

    #[test]
    fn timeline_tracks_events() {
        let mut reporter = IncidentReporter::new().with_validation_threshold(1);
        let user = EntityId::new();
        let id = reporter.create_incident(report(user));

        let v = EntityId::new();
        reporter.validate_incident(id, v);
        reporter.close_incident(id, v, "All clear");

        let timeline = reporter.get_timeline(&id).unwrap();
        assert_eq!(timeline.entries.len(), 3); // created + validated + closed
    }

    #[test]
    fn incidents_near_filters_by_distance() {
        let mut reporter = IncidentReporter::new();
        let user = EntityId::new();

        let mut r1 = report(user);
        r1.position = pos(32.080, 34.780);
        reporter.create_incident(r1);

        let mut r2 = report(user);
        r2.position = pos(33.000, 35.000); // ~100km away
        reporter.create_incident(r2);

        let nearby = reporter.incidents_near(&pos(32.080, 34.780), 1000.0);
        assert_eq!(nearby.len(), 1);
    }
}
