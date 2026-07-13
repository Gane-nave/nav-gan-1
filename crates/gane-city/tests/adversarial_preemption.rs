//! Adversarial test: preemption priority promotion
//! BUG: release_preemption used FIFO (position()) instead of max_by_key on priority.
//! If broken: Transit (lower) would be promoted instead of Emergency (higher).

use chrono::{Duration, Utc};
use gane_city::signal::*;
use gane_core::infrastructure::{TrafficPhase, TrafficSignalState};
use gane_core::types::{EntityId, GeoPosition};

fn make_signal(id: EntityId) -> TrafficSignalState {
    TrafficSignalState {
        id,
        position: GeoPosition {
            latitude_deg: 32.0,
            longitude_deg: 34.0,
            altitude_m: Some(0.0),
        },
        current_phase: TrafficPhase::Green,
        time_to_change_s: Some(20.0),
        cycle_duration_s: 90.0,
        spat_available: true,
        updated_at: Utc::now(),
    }
}

fn make_req(signal_id: EntityId, priority: PreemptionPriority) -> PreemptionRequest {
    PreemptionRequest {
        id: EntityId::new(),
        signal_id,
        priority,
        requested_phase: TrafficPhase::Green,
        vehicle_id: EntityId::new(),
        requested_at: Utc::now(),
        expires_at: Utc::now() + Duration::minutes(10),
    }
}

#[test]
fn release_promotes_highest_priority_not_fifo() {
    let mut ctrl = SignalController::new();
    let sig = EntityId::new();
    ctrl.update_signal(make_signal(sig));

    // Grant Evacuation as active
    assert_eq!(
        ctrl.request_preemption(make_req(sig, PreemptionPriority::Evacuation)),
        PreemptionOutcome::Granted
    );

    // Queue Transit FIRST (lowest)
    assert_eq!(
        ctrl.request_preemption(make_req(sig, PreemptionPriority::Transit)),
        PreemptionOutcome::Queued
    );

    // Queue Emergency SECOND (higher than Transit)
    assert_eq!(
        ctrl.request_preemption(make_req(sig, PreemptionPriority::Emergency)),
        PreemptionOutcome::Queued
    );

    assert_eq!(ctrl.queued_preemption_count(), 2);

    // Release Evacuation — should promote Emergency (highest queued), NOT Transit (FIFO)
    assert!(ctrl.release_preemption(&sig));

    let active = ctrl.active_preemption(&sig).expect("should have promoted");
    assert_eq!(
        active.priority,
        PreemptionPriority::Emergency,
        "BUG FIX: Emergency promoted over Transit. Old FIFO code would pick Transit."
    );
    assert_eq!(ctrl.queued_preemption_count(), 1, "Transit still queued");
}
