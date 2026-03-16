//! API and event system example for AURORA NAV.
#![allow(unknown_lints)]
#![allow(clippy::manual_is_multiple_of)]
//!
//! Demonstrates the event bus, EKF fusion engine, and GNSS receiver APIs.

use aurora_events::bus::EventBus;
use aurora_events::envelope::EventEnvelope;
use aurora_events::event_type::EventType;
use aurora_events::subscriber::EventHandler;
use aurora_fusion::ekf::NavigationEkf;
use aurora_gnss::GnssReceiver;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use uuid::Uuid;

struct LoggingHandler {
    count: AtomicU32,
}

impl LoggingHandler {
    fn new() -> Self {
        Self {
            count: AtomicU32::new(0),
        }
    }
}

impl EventHandler for LoggingHandler {
    fn handle(&self, event: &EventEnvelope) {
        let n = self.count.fetch_add(1, Ordering::Relaxed) + 1;
        println!("  Event #{}: {} from {}", n, event.event_type, event.origin);
    }

    fn subscribed_types(&self) -> Option<Vec<EventType>> {
        None // subscribe to all
    }
}

fn main() {
    println!("=== AURORA NAV API Example ===\n");

    // --- Event Bus ---
    println!("1. Event Bus");
    let bus = EventBus::new();
    let handler = Arc::new(LoggingHandler::new());
    let _sub = bus.subscribe(handler);

    bus.publish(&EventEnvelope::new(
        "gnss",
        "Measurement",
        Uuid::new_v4(),
        EventType::GnssMeasurementReceived,
        serde_json::json!({"satellites": 12}),
    ));
    bus.publish(&EventEnvelope::new(
        "fusion",
        "State",
        Uuid::new_v4(),
        EventType::FusionStateUpdated,
        serde_json::json!({"uncertainty_m": 2.5}),
    ));
    bus.publish(&EventEnvelope::new(
        "routing",
        "Route",
        Uuid::new_v4(),
        EventType::RouteCommitted,
        serde_json::json!({"segments": 15}),
    ));
    println!("  Total events published: {}", bus.total_events());
    println!("  Active subscribers: {}\n", bus.subscriber_count());

    // --- EKF Fusion ---
    println!("2. EKF Fusion Engine");
    let mut ekf = NavigationEkf::new();
    println!(
        "  Initial uncertainty: {:.1}m",
        ekf.position_uncertainty_m()
    );

    for epoch in 0..20 {
        ekf.predict(1.0);
        ekf.update_position(100.0, 200.0, 50.0, 3.0);
        if epoch % 5 == 0 {
            ekf.update_velocity(10.0, 5.0, 0.0, 1.0);
            ekf.update_heading(1.57, 0.1);
        }
    }

    let pos = ekf.position_enu();
    println!(
        "  Converged position: ({:.2}, {:.2}, {:.2})",
        pos.x, pos.y, pos.z
    );
    println!("  Final uncertainty: {:.2}m", ekf.position_uncertainty_m());
    println!(
        "  Vertical uncertainty: {:.2}m\n",
        ekf.vertical_uncertainty_m()
    );

    // --- GNSS Receiver ---
    println!("3. GNSS Receiver");
    let rx = GnssReceiver::new();
    println!("  Tracked satellites: {}", rx.tracked_count());
    println!("  Constellation states: {:?}", rx.constellation_states());

    println!("\n=== Example complete ===");
}
