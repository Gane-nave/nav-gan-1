use gane_nightmode::controller::*;
#[test]
fn stress_add() {
    let mut e = NightModeController::new();
    for i in 0..1000 {
        e.add(i as f64, "s");
    }
    assert_eq!(e.count(), 1000);
}
#[test]
fn remove_missing() {
    let mut e = NightModeController::new();
    assert!(!e.remove(999));
}
#[test]
fn get_missing() {
    let e = NightModeController::new();
    assert!(e.get(999).is_none());
}
#[test]
fn add_remove_add() {
    let mut e = NightModeController::new();
    let id = e.add(1.0, "a");
    e.remove(id);
    e.add(2.0, "b");
    assert_eq!(e.count(), 1);
}
#[test]
fn clear_then_add() {
    let mut e = NightModeController::new();
    e.add(1.0, "a");
    e.clear();
    e.add(2.0, "b");
    assert_eq!(e.count(), 1);
}
#[test]
fn total_after_remove() {
    let mut e = NightModeController::new();
    let id = e.add(10.0, "a");
    e.add(20.0, "b");
    e.remove(id);
    assert_eq!(e.total_value(), 20.0);
}
#[test]
fn entry_serializes() {
    let e = Entry {
        id: 1,
        value: 42.0,
        timestamp_ms: 0,
        label: "test".into(),
    };
    let j = serde_json::to_string(&e).unwrap();
    assert!(j.contains("42"));
}
#[test]
fn all_entries() {
    let mut e = NightModeController::new();
    e.add(1.0, "a");
    e.add(2.0, "b");
    assert_eq!(e.all().len(), 2);
}
