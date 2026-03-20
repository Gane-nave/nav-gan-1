use aurora_interval::{Interval, IntervalTree};

#[test]
fn adversarial_interval_overlap_containment_removal() {
    let mut tree = IntervalTree::new();

    // Phase 1: Insert overlapping intervals (time ranges for events)
    // Simulate a schedule: meeting, lunch, call, break, review
    tree.insert(Interval::new(900, 1000, "meeting")); // 9:00-10:00
    tree.insert(Interval::new(1030, 1100, "standup")); // 10:30-11:00
    tree.insert(Interval::new(1100, 1200, "coding")); // 11:00-12:00
    tree.insert(Interval::new(1200, 1300, "lunch")); // 12:00-13:00
    tree.insert(Interval::new(1300, 1400, "call")); // 13:00-14:00
    tree.insert(Interval::new(1330, 1500, "review")); // 13:30-15:00 (overlaps with call)
    tree.insert(Interval::new(1500, 1600, "break")); // 15:00-16:00
    tree.insert(Interval::new(800, 1700, "workday")); // 8:00-17:00 (spans all)
    assert_eq!(tree.len(), 8);

    // Phase 2: Point queries
    // At 9:30 — should find meeting + workday
    let at_930 = tree.query_point(930);
    assert_eq!(at_930.len(), 2);
    let labels: Vec<&str> = at_930.iter().map(|i| i.label()).collect();
    assert!(labels.contains(&"meeting"));
    assert!(labels.contains(&"workday"));

    // At 13:45 — should find call + review + workday
    let at_1345 = tree.query_point(1345);
    assert_eq!(at_1345.len(), 3);
    let labels: Vec<&str> = at_1345.iter().map(|i| i.label()).collect();
    assert!(labels.contains(&"call"));
    assert!(labels.contains(&"review"));
    assert!(labels.contains(&"workday"));

    // At 7:00 — nothing
    assert!(tree.query_point(700).is_empty());

    // At 17:00 — just workday (boundary)
    let at_1700 = tree.query_point(1700);
    assert_eq!(at_1700.len(), 1);
    assert_eq!(at_1700[0].label(), "workday");

    // Phase 3: Overlap queries
    // What overlaps with 10:00-11:00?
    let overlaps = tree.query_overlap(1000, 1100);
    let labels: Vec<&str> = overlaps.iter().map(|i| i.label()).collect();
    assert!(labels.contains(&"meeting")); // ends at 1000, overlaps at boundary
    assert!(labels.contains(&"standup")); // 1030-1100
    assert!(labels.contains(&"coding")); // starts at 1100, boundary overlap
    assert!(labels.contains(&"workday")); // spans all

    // What overlaps with 18:00-19:00? Nothing
    assert!(tree.query_overlap(1800, 1900).is_empty());

    // Phase 4: Containment queries
    // Which intervals fully contain 13:30-14:00?
    let containers = tree.query_containing(1330, 1400);
    let labels: Vec<&str> = containers.iter().map(|i| i.label()).collect();
    assert!(labels.contains(&"review")); // 1330-1500 contains 1330-1400
    assert!(labels.contains(&"workday")); // 800-1700 contains 1330-1400
    assert!(labels.contains(&"call")); // 1300-1400 contains 1330-1400

    // Phase 5: Remove and re-query
    assert_eq!(tree.remove_by_label("lunch"), 1);
    assert_eq!(tree.len(), 7);
    assert!(tree.query_point(1230).iter().all(|i| i.label() != "lunch"));

    // Remove all items with label "workday"
    assert_eq!(tree.remove_by_label("workday"), 1);
    assert_eq!(tree.len(), 6);

    // Now at 9:30, only meeting remains
    let at_930 = tree.query_point(930);
    assert_eq!(at_930.len(), 1);
    assert_eq!(at_930[0].label(), "meeting");

    // Phase 6: Span
    let span = tree.span().unwrap();
    assert_eq!(span.0, 900); // earliest low
    assert_eq!(span.1, 1600); // latest high

    // Phase 7: Count overlaps
    assert_eq!(tree.count_overlaps(1300, 1500), 3); // call, review, break

    // Phase 8: Edge cases
    // Point interval
    tree.insert(Interval::new(1000, 1000, "instant"));
    assert!(tree.contains_point(1000));
    let at_1000 = tree.query_point(1000);
    let labels: Vec<&str> = at_1000.iter().map(|i| i.label()).collect();
    assert!(labels.contains(&"instant"));
    assert!(labels.contains(&"meeting")); // meeting ends at 1000

    // Phase 9: Clear and verify empty
    tree.clear();
    assert!(tree.is_empty());
    assert!(tree.span().is_none());
    assert!(tree.query_point(1000).is_empty());

    // Phase 10: Negative intervals
    let mut neg_tree = IntervalTree::new();
    neg_tree.insert(Interval::new(-100, -50, "past"));
    neg_tree.insert(Interval::new(-25, 25, "crossing"));
    neg_tree.insert(Interval::new(50, 100, "future"));

    assert_eq!(neg_tree.query_point(0).len(), 1);
    assert_eq!(neg_tree.query_point(0)[0].label(), "crossing");
    assert_eq!(neg_tree.query_point(-75).len(), 1);
    assert_eq!(neg_tree.query_point(-75)[0].label(), "past");

    assert!(neg_tree.queries() > 0);
}
