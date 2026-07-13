//! Adversarial test: sampling_rate==0 no panic
//! BUG: time_sampling() used `self.sample_counter % self.config.sampling_rate as u64`
//! without guarding against zero — division by zero panic.
//! Fix: `let rate = (self.config.sampling_rate as u64).max(1);`

use chrono::Utc;
use gane_core::types::EntityId;
use gane_edge::reduction::*;

fn make_sample(value: f64) -> DataSample {
    DataSample {
        id: EntityId::new(),
        timestamp: Utc::now(),
        value,
        latitude: 32.08,
        longitude: 34.78,
        source: "test".into(),
    }
}

#[test]
fn sampling_rate_zero_does_not_panic() {
    let config = ReductionConfig {
        strategy: ReductionStrategy::TimeSampling,
        sampling_rate: 0, // THE BUG TRIGGER
        change_threshold: 0.0,
        spatial_radius_m: 0.0,
        window_size: 0,
    };
    let mut reducer = DataReducer::new(config);

    // Process 10 samples — old code would panic on first sample (division by zero)
    let mut emitted = 0;
    for i in 0..10 {
        if reducer.process(&make_sample(i as f64)).is_some() {
            emitted += 1;
        }
    }

    // rate clamped to 1 = every sample emitted
    assert_eq!(
        emitted, 10,
        "BUG FIX: sampling_rate=0 clamped to 1, all 10 samples emitted. Old code would panic."
    );
    assert_eq!(reducer.total_received(), 10);
    assert_eq!(reducer.total_emitted(), 10);
}
