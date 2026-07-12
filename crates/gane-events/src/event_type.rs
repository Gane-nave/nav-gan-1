//! All event types in the G.A.N.E NAV system (Section 7 of the spec).

use serde::{Deserialize, Serialize};

/// Discriminated union of all event types in the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventType {
    // -- Position & Navigation --
    PositionUpdate,
    DriverStateUpdate,
    VehicleStateUpdate,

    // -- GNSS Pipeline --
    GnssMeasurementReceived,
    CorrectionReceived,
    FusionStateUpdated,
    IntegrityStateChanged,
    ContinuityModeChanged,

    // -- Map Discovery --
    RoadDiscovered,
    LaneDiscovered,
    MapUpdated,

    // -- Incidents --
    IncidentCreated,
    IncidentUpdated,
    IncidentClosed,
    EvidenceCaptured,

    // -- Scoring --
    TrustScoreUpdated,
    RiskScoreUpdated,
    StabilityIndexUpdated,

    // -- Routing --
    RouteProposed,
    RouteCommitted,
    RerouteRequested,
    RerouteApplied,

    // -- Traffic Flow Control --
    FlowControlUpdated,

    // -- Policy & Alerts --
    PolicyChanged,
    AlertTriggered,

    // -- UI --
    UIStateChanged,

    // -- Sync --
    OfflineQueueFlushed,
    SyncConflictResolved,

    // -- Satellite --
    SatellitePacketSent,
    SatellitePacketReceived,

    // -- Fleet --
    DispatchIssued,
    TaskAssigned,
    TaskCompleted,

    // -- Communication --
    MessageCreated,
    VoiceSessionStarted,

    // -- ML Models --
    ModelUpdated,

    // -- Infrastructure --
    InfrastructureAlert,
    TrafficSignalChange,
    RoadConditionDetected,

    // -- Security --
    SpoofingDetected,
    JammingDetected,
    SourceExcluded,
    SourceReinstated,

    // -- Replay --
    ReplayStarted,
    ReplayCompleted,
}

impl EventType {
    /// Returns a stable string key for serialisation / routing.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PositionUpdate => "position.update",
            Self::DriverStateUpdate => "driver.state.update",
            Self::VehicleStateUpdate => "vehicle.state.update",
            Self::GnssMeasurementReceived => "gnss.measurement.received",
            Self::CorrectionReceived => "correction.received",
            Self::FusionStateUpdated => "fusion.state.updated",
            Self::IntegrityStateChanged => "integrity.state.changed",
            Self::ContinuityModeChanged => "continuity.mode.changed",
            Self::RoadDiscovered => "map.road.discovered",
            Self::LaneDiscovered => "map.lane.discovered",
            Self::MapUpdated => "map.updated",
            Self::IncidentCreated => "incident.created",
            Self::IncidentUpdated => "incident.updated",
            Self::IncidentClosed => "incident.closed",
            Self::EvidenceCaptured => "evidence.captured",
            Self::TrustScoreUpdated => "trust.score.updated",
            Self::RiskScoreUpdated => "risk.score.updated",
            Self::StabilityIndexUpdated => "stability.index.updated",
            Self::RouteProposed => "route.proposed",
            Self::RouteCommitted => "route.committed",
            Self::RerouteRequested => "route.reroute.requested",
            Self::RerouteApplied => "route.reroute.applied",
            Self::FlowControlUpdated => "flow.control.updated",
            Self::PolicyChanged => "policy.changed",
            Self::AlertTriggered => "alert.triggered",
            Self::UIStateChanged => "ui.state.changed",
            Self::OfflineQueueFlushed => "sync.offline.flushed",
            Self::SyncConflictResolved => "sync.conflict.resolved",
            Self::SatellitePacketSent => "satellite.packet.sent",
            Self::SatellitePacketReceived => "satellite.packet.received",
            Self::DispatchIssued => "fleet.dispatch.issued",
            Self::TaskAssigned => "fleet.task.assigned",
            Self::TaskCompleted => "fleet.task.completed",
            Self::MessageCreated => "comm.message.created",
            Self::VoiceSessionStarted => "comm.voice.started",
            Self::ModelUpdated => "ml.model.updated",
            Self::InfrastructureAlert => "infra.alert",
            Self::TrafficSignalChange => "infra.signal.change",
            Self::RoadConditionDetected => "infra.road.condition",
            Self::SpoofingDetected => "security.spoofing.detected",
            Self::JammingDetected => "security.jamming.detected",
            Self::SourceExcluded => "integrity.source.excluded",
            Self::SourceReinstated => "integrity.source.reinstated",
            Self::ReplayStarted => "replay.started",
            Self::ReplayCompleted => "replay.completed",
        }
    }
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
