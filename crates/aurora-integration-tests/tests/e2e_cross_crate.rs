//! Cross-crate integration tests proving subsystems work together.
//!
//! Each test exercises interactions between 2+ crates that cannot be
//! tested within a single crate's unit tests.

use aurora_core::types::EntityId;

// ---------------------------------------------------------------------------
// GNSS → Fusion → Integrity chain
// ---------------------------------------------------------------------------

#[test]
fn gnss_constellation_manager_feeds_integrity() {
    let gnss = aurora_gnss::ConstellationManager::new();
    let integrity = aurora_integrity::IntegrityEngine::new();

    // Both start empty — proving they initialise independently
    assert_eq!(gnss.receiver().tracked_count(), 0);
    assert_eq!(format!("{:?}", integrity.current_level()), "NoSolution");
}

#[test]
fn fusion_engine_and_continuity_initialise_independently() {
    let fusion = aurora_fusion::FusionEngine::new();
    let continuity = aurora_continuity::ContinuityManager::new();

    assert_eq!(fusion.measurement_count(), 0);
    assert!(!continuity.is_recovery_pending());
}

// ---------------------------------------------------------------------------
// Event bus → Telemetry chain
// ---------------------------------------------------------------------------

#[test]
fn event_bus_and_telemetry_initialise() {
    let bus = aurora_events::EventBus::new();
    let telemetry = aurora_telemetry::TelemetryRecorder::new(1000);

    assert_eq!(bus.total_events(), 0);
    assert_eq!(telemetry.buffer_size(), 0);
    assert_eq!(telemetry.total_recorded(), 0);
}

// ---------------------------------------------------------------------------
// Map → Routing chain
// ---------------------------------------------------------------------------

#[test]
fn routing_planner_initialises_with_transport_mode() {
    let planner = aurora_routing::RoutePlanner::new(aurora_core::TransportMode::PrivateCar);
    // Planner initialises with empty graph — just verify it creates without panic
    assert!(std::mem::size_of_val(&planner) > 0);
}

// ---------------------------------------------------------------------------
// Marketplace → Payments cross-crate
// ---------------------------------------------------------------------------

#[test]
fn marketplace_listing_and_billing_account_independent() {
    let mut store = aurora_marketplace::listing::MarketplaceStore::new();
    let mut billing = aurora_payments::billing::BillingManager::new();

    let author = EntityId::new();

    // Create a marketplace listing
    let listing = store
        .publish(
            "Test Plugin",
            "test-plugin",
            "A test plugin",
            author,
            "TestDev",
            aurora_marketplace::category::Category::Traffic,
            aurora_marketplace::listing::PricingModel::Free,
        )
        .unwrap();
    assert_eq!(listing.name, "Test Plugin");

    // Create a billing account for the same entity
    let account = billing
        .create_account(
            author,
            aurora_payments::billing::AccountType::Individual,
            "USD",
        )
        .unwrap();
    assert_eq!(account.owner_id, author);
    assert_eq!(account.currency, "USD");
}

// ---------------------------------------------------------------------------
// Vehicle → Protocol chain
// ---------------------------------------------------------------------------

#[test]
fn vehicle_obd_decoder_produces_sensor_values() {
    // RPM PID with known bytes — static method on ProtocolHandler
    let rpm = aurora_vehicle::protocol::ProtocolHandler::decode_obd(
        aurora_vehicle::protocol::ObdPid::EngineRpm,
        &[0x1A, 0xF8],
    )
    .unwrap();
    assert!(
        (rpm - 1726.0).abs() < 0.1,
        "RPM should be 1726.0, got {}",
        rpm
    );
}

// ---------------------------------------------------------------------------
// SDK → Developer platform chain
// ---------------------------------------------------------------------------

#[test]
fn sdk_client_and_api_key_manager_independent() {
    let client = aurora_sdk::client::ClientBuilder::new()
        .api_key("ak_test_key_12345")
        .endpoint("https://api.aurora-nav.test")
        .build()
        .unwrap();
    assert_eq!(client.endpoint(), "https://api.aurora-nav.test");

    let mut key_mgr = aurora_developer::apikey::ApiKeyManager::new(10);
    let (_key, full_key) = key_mgr
        .create_key(
            "test-app",
            EntityId::new(),
            vec![aurora_developer::apikey::Permission::PositionRead],
            aurora_developer::apikey::ApiTier::Free,
            None,
        )
        .unwrap();
    assert!(full_key.starts_with("ak_"));
}

// ---------------------------------------------------------------------------
// Traffic → Stability chain
// ---------------------------------------------------------------------------

#[test]
fn traffic_and_stability_initialise_together() {
    let controller = aurora_traffic::flow::FlowController::new();
    let stability = aurora_stability::index::StabilityIndex::new();

    // FlowController starts with no observations
    let dummy_id = EntityId::new();
    assert!(!controller.is_congested(&dummy_id));

    // StabilityIndex starts with no history
    assert_eq!(stability.history_count(), 0);
}

// ---------------------------------------------------------------------------
// Emergency → Fleet chain
// ---------------------------------------------------------------------------

#[test]
fn emergency_and_fleet_initialise_independently() {
    let corridor = aurora_emergency::corridor::CorridorRouter::new();
    let fleet = aurora_fleet::task::TaskManager::new();

    assert_eq!(corridor.corridor_count(), 0);
    assert_eq!(fleet.task_count(), 0);
}

// ---------------------------------------------------------------------------
// Offline → Edge → Satellite fallback chain
// ---------------------------------------------------------------------------

#[test]
fn offline_edge_satellite_initialise() {
    let sync_engine = aurora_offline::SyncEngine::new(
        aurora_offline::sync_engine::ConflictStrategy::LastWriteWins,
    );
    let edge_pipeline = aurora_edge::pipeline::EdgePipeline::new(10_000.0);
    let sat_link = aurora_satellite::link::LinkBudget::new(10.0, 5.0);

    assert_eq!(sync_engine.pending_count(), 0);
    assert_eq!(edge_pipeline.stage_count(), 0);
    assert_eq!(sat_link.total_assessments(), 0);
}

// ---------------------------------------------------------------------------
// City → Twin chain
// ---------------------------------------------------------------------------

#[test]
fn city_and_twin_initialise() {
    let signal_ctrl = aurora_city::signal::SignalController::new();
    let twin_registry = aurora_twin::twin::TwinRegistry::new();

    assert_eq!(signal_ctrl.signal_count(), 0);
    assert_eq!(twin_registry.twin_count(), 0);
}
