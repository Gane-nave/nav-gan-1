use aurora_gnss_quality::scorer::*;
#[test]
fn range() {
    let mut s = QualityScorer::new();
    let m = s.evaluate(20, 18, &[45.0; 20], 0.8, 1.0, 20);
    assert!(m.overall_score >= 0.0 && m.overall_score <= 1.0);
}
#[test]
fn poor_geo() {
    let mut s = QualityScorer::new();
    let m = s.evaluate(4, 3, &[20.0; 3], 10.0, 15.0, 1000);
    assert!(m.overall_score < 0.5);
}
#[test]
fn excellent() {
    let mut s = QualityScorer::new();
    let m = s.evaluate(24, 20, &[48.0; 20], 0.5, 0.8, 10);
    assert!(m.overall_score > 0.7);
}
#[test]
fn stress() {
    let mut s = QualityScorer::new();
    for _ in 0..5000 {
        s.evaluate(12, 10, &[35.0, 40.0], 1.2, 1.8, 100);
    }
    assert_eq!(s.evaluations(), 5000);
}
#[test]
fn pdop() {
    let mut s = QualityScorer::new();
    let m = s.evaluate(10, 8, &[40.0], 3.0, 4.0, 50);
    assert!((m.pdop - 5.0).abs() < 0.01);
}
#[test]
fn acc() {
    let mut s = QualityScorer::new();
    let m = s.evaluate(10, 8, &[40.0], 2.0, 3.0, 50);
    assert!((m.horizontal_accuracy_m - 5.0).abs() < 0.01);
}
#[test]
fn ser() {
    let mut s = QualityScorer::new();
    let m = s.evaluate(10, 8, &[40.0], 1.0, 1.5, 50);
    assert!(serde_json::to_string(&m).unwrap().contains("overall_score"));
}
#[test]
fn thresh() {
    let mut s = QualityScorer::new();
    s.set_multipath_threshold(0.1);
    s.evaluate(10, 8, &[10.0, 50.0], 1.0, 1.5, 50);
}
