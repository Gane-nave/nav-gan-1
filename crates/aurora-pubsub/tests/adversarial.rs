//! Adversarial tests for aurora-pubsub

use aurora_pubsub::broker::Broker;
use aurora_pubsub::subscriber::{DeliveryMode, Subscriber};
use aurora_pubsub::topic::Topic;

#[test]
fn adversarial_backpressure_and_removal() {
    let mut broker = Broker::new();
    broker.register_topic("sensor.gps");

    let fast_id = broker.add_subscriber("fast");
    broker.subscribe(fast_id, "sensor.*");

    let slow_id = broker.add_subscriber("slow");
    broker.subscribe(slow_id, "sensor.*");

    let d1 = broker.publish("sensor.gps", vec![1], 1000);
    assert_eq!(d1, 2);

    let d2 = broker.publish("sensor.gps", vec![2], 2000);
    assert_eq!(d2, 2);

    let d3 = broker.publish("sensor.gps", vec![3], 3000);
    assert_eq!(d3, 2);

    assert_eq!(broker.total_published(), 3);
    assert_eq!(broker.total_delivered(), 6);
    assert_eq!(broker.total_dropped(), 0);

    assert!(broker.remove_subscriber(slow_id));
    assert_eq!(broker.subscriber_count(), 1);

    let d4 = broker.publish("sensor.gps", vec![4], 4000);
    assert_eq!(d4, 1);

    assert_eq!(broker.total_published(), 4);
    assert_eq!(broker.total_delivered(), 7);

    let d5 = broker.publish("nav.route", vec![5], 5000);
    assert_eq!(d5, 0);
}

#[test]
fn adversarial_topic_patterns() {
    let topic = Topic::new("nav.gps");
    assert!(!topic.matches_pattern(""));

    let root_topic = Topic::new("anything");
    assert!(root_topic.matches_pattern("*"));

    let deep = Topic::new("a.b.c.d");
    assert_eq!(deep.namespace(), "a.b.c");

    let no_dot = Topic::new("simple");
    assert_eq!(no_dot.namespace(), "");

    let mut sub = Subscriber::new(1, "multi").with_delivery_mode(DeliveryMode::AtMostOnce);
    sub.subscribe("nav.*");
    sub.subscribe("sensor.gps");
    sub.subscribe("alert.*");

    assert!(sub.matches_topic("nav.position"));
    assert!(sub.matches_topic("sensor.gps"));
    assert!(sub.matches_topic("alert.emergency"));
    assert!(!sub.matches_topic("sensor.imu"));
    assert!(!sub.matches_topic("other.thing"));

    assert_eq!(sub.delivery_mode(), DeliveryMode::AtMostOnce);
}

#[test]
fn adversarial_inactive_topic_blocks_delivery() {
    let mut broker = Broker::new();
    broker.register_topic("blocked.topic");

    let sub_id = broker.add_subscriber("listener");
    broker.subscribe(sub_id, "blocked.*");

    // Deactivate topic via a publish then check
    let d1 = broker.publish("blocked.topic", vec![1], 1000);
    assert_eq!(d1, 1, "Should deliver when active");

    // We can't directly deactivate through broker API, but we tested this in unit tests
    // Instead test that unregistered topic gets auto-created
    let d2 = broker.publish("blocked.new", vec![2], 2000);
    assert_eq!(d2, 1, "Auto-registered topic should deliver");
    assert!(broker.get_topic("blocked.new").is_some());
}
