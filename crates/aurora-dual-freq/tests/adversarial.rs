use aurora_dual_freq::engine::*;
#[test]
fn stress() {
    let mut e = DualFreqEngine::new();
    for i in 0..1000 {
        e.add(i as f64, "s");
    }
    assert_eq!(e.count(), 1000);
}
#[test]
fn rm_miss() {
    assert!(!DualFreqEngine::new().remove(999));
}
#[test]
fn get_miss() {
    assert!(DualFreqEngine::new().get(999).is_none());
}
#[test]
fn add_rm_add() {
    let mut e = DualFreqEngine::new();
    let id = e.add(1.0, "a");
    e.remove(id);
    e.add(2.0, "b");
    assert_eq!(e.count(), 1);
}
#[test]
fn clear_add() {
    let mut e = DualFreqEngine::new();
    e.add(1.0, "a");
    e.clear();
    e.add(2.0, "b");
    assert_eq!(e.count(), 1);
}
#[test]
fn total_rm() {
    let mut e = DualFreqEngine::new();
    let id = e.add(10.0, "a");
    e.add(20.0, "b");
    e.remove(id);
    assert_eq!(e.total_value(), 20.0);
}
#[test]
fn entry_ser() {
    let e = Entry {
        id: 1,
        value: 42.0,
        timestamp_ms: 0,
        label: "test".into(),
    };
    assert!(serde_json::to_string(&e).unwrap().contains("42"));
}
#[test]
fn all_ent() {
    let mut e = DualFreqEngine::new();
    e.add(1.0, "a");
    e.add(2.0, "b");
    assert_eq!(e.all().len(), 2);
}
