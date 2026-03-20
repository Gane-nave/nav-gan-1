use aurora_tunnel::engine::*;
#[test]
fn no_tunnel() {
    let mut t = TunnelMode::new();
    t.propagate(10.0);
    assert_eq!(t.current_position().distance_in_tunnel_m, 0.0);
}
#[test]
fn long_tunnel() {
    let mut t = TunnelMode::new();
    t.gnss_lost(32.0, 34.0, 0.0, 30.0, 0);
    for _ in 0..300 {
        t.propagate(1.0);
    }
    assert!(t.current_position().distance_in_tunnel_m > 8000.0);
    assert!(t.current_position().confidence >= 0.1);
}
#[test]
fn zero_speed() {
    let mut t = TunnelMode::new();
    t.gnss_lost(32.0, 34.0, 0.0, 0.0, 0);
    t.propagate(10.0);
    assert_eq!(t.current_position().distance_in_tunnel_m, 0.0);
}
#[test]
fn recover_conf() {
    let mut t = TunnelMode::new();
    t.gnss_lost(32.0, 34.0, 0.0, 20.0, 0);
    t.propagate(30.0);
    t.gnss_recovered(32.01, 34.0, 0.0, 20.0);
    t.reacquisition_complete();
    assert_eq!(t.current_position().confidence, 1.0);
}
#[test]
fn multi_acc() {
    let mut t = TunnelMode::new();
    for i in 0..5u64 {
        t.gnss_lost(32.0 + i as f64 * 0.01, 34.0, 0.0, 20.0, i * 10000);
        t.propagate(5.0);
        t.gnss_recovered(32.0, 34.0, 0.0, 20.0);
        t.reacquisition_complete();
    }
    assert!(t.total_tunnel_distance() > 0.0);
    assert_eq!(t.tunnel_count(), 5);
}
#[test]
fn pos_ser() {
    let p = TunnelPosition {
        lat: 32.0,
        lon: 34.0,
        heading_deg: 90.0,
        speed_mps: 20.0,
        confidence: 0.8,
        distance_in_tunnel_m: 500.0,
        state: TunnelState::InTunnel,
    };
    assert!(serde_json::to_string(&p)
        .unwrap()
        .contains("distance_in_tunnel_m"));
}
#[test]
fn set_decay() {
    let mut t = TunnelMode::new();
    t.set_confidence_decay(0.1);
    t.gnss_lost(32.0, 34.0, 0.0, 20.0, 0);
    t.propagate(5.0);
    assert!(t.current_position().confidence < 0.95);
}
#[test]
fn reacq_no_tunnel() {
    let mut t = TunnelMode::new();
    t.gnss_recovered(32.0, 34.0, 0.0, 20.0);
    assert_eq!(t.state(), TunnelState::Reacquiring);
}
