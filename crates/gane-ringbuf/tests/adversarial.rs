//! Adversarial tests for gane-ringbuf

use gane_ringbuf::buffer::RingBuffer;
use gane_ringbuf::watermark::{WatermarkMonitor, WatermarkState};

#[test]
fn adversarial_wrap_around_pop_watermark() {
    // Ring buffer with capacity 3
    let mut rb = RingBuffer::new(3);

    // Push 5 items — overwrites 1 and 2
    rb.push(1u32);
    rb.push(2);
    rb.push(3);
    rb.push(4); // overwrites 1
    rb.push(5); // overwrites 2

    // Should contain [3, 4, 5] in order
    assert_eq!(rb.items(), vec![3, 4, 5]);
    assert_eq!(rb.total_writes(), 5);
    assert_eq!(rb.total_overwrites(), 2);
    assert!((rb.overwrite_ratio() - 0.4).abs() < f64::EPSILON);
    assert!(rb.is_full());

    // Pop oldest (3)
    let popped = rb.pop_oldest();
    assert_eq!(popped, Some(3));
    assert_eq!(rb.len(), 2);
    assert_eq!(rb.items(), vec![4, 5]);

    // Push 6 — buffer should contain [4, 5, 6]
    rb.push(6);
    assert_eq!(rb.items(), vec![4, 5, 6]);
    assert!(rb.is_full());

    // Peek latest and oldest
    assert_eq!(rb.peek_latest(), Some(&6));
    assert_eq!(rb.peek_oldest(), Some(&4));

    // Watermark monitoring
    let mut wm = WatermarkMonitor::new(0.8, 0.2);

    // Sample at different fill levels
    assert_eq!(wm.sample(0.5), WatermarkState::Normal);
    assert_eq!(wm.high_breaches(), 0);

    assert_eq!(wm.sample(0.9), WatermarkState::High);
    assert_eq!(wm.high_breaches(), 1);

    // Staying high doesn't increment breach count
    assert_eq!(wm.sample(0.85), WatermarkState::High);
    assert_eq!(wm.high_breaches(), 1);

    // Drop to normal
    assert_eq!(wm.sample(0.5), WatermarkState::Normal);

    // Go high again — new breach
    assert_eq!(wm.sample(0.9), WatermarkState::High);
    assert_eq!(wm.high_breaches(), 2);

    // Drop to low
    assert_eq!(wm.sample(0.1), WatermarkState::Low);
    assert_eq!(wm.low_breaches(), 1);

    // Verify peak and avg
    assert!((wm.peak_fill() - 0.9).abs() < f64::EPSILON);
    assert!(wm.avg_fill() > 0.0);
    assert_eq!(wm.samples(), 6);
}
