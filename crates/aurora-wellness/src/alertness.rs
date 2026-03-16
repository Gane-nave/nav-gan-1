//! Alertness scoring — continuous driver attention assessment.

use std::time::Instant;

/// Alertness score (0.0 = unconscious, 1.0 = fully alert).
#[derive(Debug, Clone, Copy)]
pub struct AlertnessScore {
    /// Overall alertness (0.0 to 1.0).
    pub score: f64,
    /// Reaction time estimate (seconds).
    pub estimated_reaction_time: f64,
    /// Confidence in the assessment (0.0 to 1.0).
    pub confidence: f64,
}

impl AlertnessScore {
    /// Whether the driver is considered safe to drive.
    pub fn is_safe(&self) -> bool {
        self.score >= 0.5
    }

    /// Whether the driver needs a warning.
    pub fn needs_warning(&self) -> bool {
        self.score < 0.7 && self.score >= 0.4
    }

    /// Whether immediate intervention is needed.
    pub fn needs_intervention(&self) -> bool {
        self.score < 0.4
    }
}

/// Input signals for alertness assessment.
#[derive(Debug, Clone)]
pub struct AlertnessInput {
    /// Eye blink rate (blinks per minute), if available from camera.
    pub blink_rate: Option<f64>,
    /// Percentage of time eyes are closed (PERCLOS), if available.
    pub perclos: Option<f64>,
    /// Head pose deviation from forward-facing (degrees).
    pub head_deviation_deg: Option<f64>,
    /// Steering wheel grip pressure (0.0 to 1.0), if available.
    pub grip_pressure: Option<f64>,
    /// Time since last driver input (seconds).
    pub time_since_last_input: f64,
    /// Current driving duration (seconds).
    pub driving_duration_secs: f64,
}

/// Alertness scoring configuration.
#[derive(Debug, Clone)]
pub struct AlertnessConfig {
    /// Normal blink rate (blinks per minute).
    pub normal_blink_rate: f64,
    /// PERCLOS threshold for drowsiness (fraction).
    pub perclos_drowsy_threshold: f64,
    /// Head deviation threshold for distraction (degrees).
    pub head_deviation_threshold: f64,
    /// Weight for blink rate factor.
    pub blink_weight: f64,
    /// Weight for PERCLOS factor.
    pub perclos_weight: f64,
    /// Weight for head pose factor.
    pub head_weight: f64,
    /// Weight for grip pressure factor.
    pub grip_weight: f64,
    /// Weight for input activity factor.
    pub input_weight: f64,
    /// Weight for driving duration factor.
    pub duration_weight: f64,
    /// Driving duration after which fatigue penalty starts (seconds).
    pub fatigue_onset_secs: f64,
}

impl Default for AlertnessConfig {
    fn default() -> Self {
        Self {
            normal_blink_rate: 15.0,
            perclos_drowsy_threshold: 0.15,
            head_deviation_threshold: 30.0,
            blink_weight: 0.2,
            perclos_weight: 0.25,
            head_weight: 0.15,
            grip_weight: 0.1,
            input_weight: 0.15,
            duration_weight: 0.15,
            fatigue_onset_secs: 7200.0, // 2 hours
        }
    }
}

/// Alertness scoring engine.
pub struct AlertnessMonitor {
    config: AlertnessConfig,
    last_assessment: Option<Instant>,
    score_history: Vec<(Instant, f64)>,
    max_history: usize,
}

impl AlertnessMonitor {
    /// Create a new alertness monitor.
    pub fn new(config: AlertnessConfig) -> Self {
        Self {
            config,
            last_assessment: None,
            score_history: Vec::new(),
            max_history: 100,
        }
    }

    /// Create with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(AlertnessConfig::default())
    }

    /// Assess alertness from input signals.
    pub fn assess(&mut self, input: &AlertnessInput) -> AlertnessScore {
        let mut total_weight = 0.0;
        let mut weighted_score = 0.0;
        let mut available_signals = 0u32;

        // Blink rate factor
        if let Some(blink_rate) = input.blink_rate {
            let blink_score = self.blink_rate_score(blink_rate);
            weighted_score += blink_score * self.config.blink_weight;
            total_weight += self.config.blink_weight;
            available_signals += 1;
        }

        // PERCLOS factor
        if let Some(perclos) = input.perclos {
            let perclos_score = self.perclos_score(perclos);
            weighted_score += perclos_score * self.config.perclos_weight;
            total_weight += self.config.perclos_weight;
            available_signals += 1;
        }

        // Head deviation factor
        if let Some(head_dev) = input.head_deviation_deg {
            let head_score = self.head_deviation_score(head_dev);
            weighted_score += head_score * self.config.head_weight;
            total_weight += self.config.head_weight;
            available_signals += 1;
        }

        // Grip pressure factor
        if let Some(grip) = input.grip_pressure {
            weighted_score += grip * self.config.grip_weight;
            total_weight += self.config.grip_weight;
            available_signals += 1;
        }

        // Input activity factor (always available)
        let input_score = self.input_activity_score(input.time_since_last_input);
        weighted_score += input_score * self.config.input_weight;
        total_weight += self.config.input_weight;
        available_signals += 1;

        // Duration factor (always available)
        let duration_score = self.duration_score(input.driving_duration_secs);
        weighted_score += duration_score * self.config.duration_weight;
        total_weight += self.config.duration_weight;
        available_signals += 1;

        let score = if total_weight > 0.0 {
            (weighted_score / total_weight).clamp(0.0, 1.0)
        } else {
            0.5 // No data — assume moderate
        };

        let confidence = (available_signals as f64 / 6.0).clamp(0.0, 1.0);
        let estimated_reaction_time = Self::estimate_reaction_time(score);

        let now = Instant::now();
        self.last_assessment = Some(now);
        self.score_history.push((now, score));
        if self.score_history.len() > self.max_history {
            self.score_history.remove(0);
        }

        AlertnessScore {
            score,
            estimated_reaction_time,
            confidence,
        }
    }

    /// Get the trend of alertness over recent history.
    /// Returns the slope: positive = improving, negative = degrading.
    pub fn trend(&self) -> f64 {
        if self.score_history.len() < 2 {
            return 0.0;
        }

        let n = self.score_history.len();
        let recent_half = &self.score_history[n / 2..];
        let early_half = &self.score_history[..n / 2];

        let recent_avg: f64 =
            recent_half.iter().map(|(_, s)| s).sum::<f64>() / recent_half.len() as f64;
        let early_avg: f64 =
            early_half.iter().map(|(_, s)| s).sum::<f64>() / early_half.len() as f64;

        recent_avg - early_avg
    }

    /// Get the number of assessments recorded.
    pub fn assessment_count(&self) -> usize {
        self.score_history.len()
    }

    fn blink_rate_score(&self, rate: f64) -> f64 {
        // Normal: ~15 blinks/min. Too fast (>25) or too slow (<5) indicates fatigue/distraction.
        let normal = self.config.normal_blink_rate;
        let deviation = (rate - normal).abs() / normal;
        (1.0 - deviation).clamp(0.0, 1.0)
    }

    fn perclos_score(&self, perclos: f64) -> f64 {
        // PERCLOS: fraction of time eyes are >80% closed over 1 minute.
        // Normal: <0.08, drowsy: >0.15
        let threshold = self.config.perclos_drowsy_threshold;
        if perclos <= 0.0 {
            1.0
        } else if perclos >= threshold {
            0.0
        } else {
            1.0 - (perclos / threshold)
        }
    }

    fn head_deviation_score(&self, deviation_deg: f64) -> f64 {
        let threshold = self.config.head_deviation_threshold;
        let abs_dev = deviation_deg.abs();
        if abs_dev >= threshold {
            0.0
        } else {
            1.0 - (abs_dev / threshold)
        }
    }

    fn input_activity_score(&self, secs_since_input: f64) -> f64 {
        // No input for >10s is concerning, >30s is critical.
        if secs_since_input <= 2.0 {
            1.0
        } else if secs_since_input >= 30.0 {
            0.0
        } else {
            1.0 - ((secs_since_input - 2.0) / 28.0)
        }
    }

    fn duration_score(&self, driving_secs: f64) -> f64 {
        let onset = self.config.fatigue_onset_secs;
        if driving_secs <= onset {
            1.0
        } else {
            // Linearly decrease to 0.2 over next 4 hours
            let overtime = driving_secs - onset;
            let max_overtime = 4.0 * 3600.0;
            (1.0 - 0.8 * (overtime / max_overtime)).clamp(0.2, 1.0)
        }
    }

    fn estimate_reaction_time(alertness: f64) -> f64 {
        // Baseline reaction time ~0.7s when alert, up to ~3.0s when very drowsy.
        let baseline = 0.7;
        let max_rt = 3.0;
        baseline + (1.0 - alertness) * (max_rt - baseline)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn alert_input() -> AlertnessInput {
        AlertnessInput {
            blink_rate: Some(15.0),
            perclos: Some(0.02),
            head_deviation_deg: Some(5.0),
            grip_pressure: Some(0.8),
            time_since_last_input: 1.0,
            driving_duration_secs: 1800.0, // 30 min
        }
    }

    fn drowsy_input() -> AlertnessInput {
        AlertnessInput {
            blink_rate: Some(30.0),         // High blink rate
            perclos: Some(0.20),            // Eyes closing frequently
            head_deviation_deg: Some(40.0), // Head nodding
            grip_pressure: Some(0.2),       // Loose grip
            time_since_last_input: 20.0,    // Long idle
            driving_duration_secs: 14400.0, // 4 hours
        }
    }

    #[test]
    fn test_alert_driver_scores_high() {
        let mut monitor = AlertnessMonitor::with_defaults();
        let result = monitor.assess(&alert_input());
        assert!(
            result.score > 0.7,
            "Alert driver should score > 0.7, got {}",
            result.score
        );
        assert!(result.is_safe());
        assert!(!result.needs_warning());
        assert!(!result.needs_intervention());
    }

    #[test]
    fn test_drowsy_driver_scores_low() {
        let mut monitor = AlertnessMonitor::with_defaults();
        let result = monitor.assess(&drowsy_input());
        assert!(
            result.score < 0.4,
            "Drowsy driver should score < 0.4, got {}",
            result.score
        );
        assert!(!result.is_safe());
        assert!(result.needs_intervention());
    }

    #[test]
    fn test_confidence_with_all_signals() {
        let mut monitor = AlertnessMonitor::with_defaults();
        let result = monitor.assess(&alert_input());
        assert_eq!(result.confidence, 1.0);
    }

    #[test]
    fn test_confidence_with_minimal_signals() {
        let mut monitor = AlertnessMonitor::with_defaults();
        let input = AlertnessInput {
            blink_rate: None,
            perclos: None,
            head_deviation_deg: None,
            grip_pressure: None,
            time_since_last_input: 1.0,
            driving_duration_secs: 1800.0,
        };
        let result = monitor.assess(&input);
        // Only 2 signals available (input activity + duration) out of 6
        assert!((result.confidence - 2.0 / 6.0).abs() < 0.01);
    }

    #[test]
    fn test_reaction_time_increases_with_drowsiness() {
        let mut monitor = AlertnessMonitor::with_defaults();
        let alert_rt = monitor.assess(&alert_input()).estimated_reaction_time;
        let drowsy_rt = monitor.assess(&drowsy_input()).estimated_reaction_time;
        assert!(
            drowsy_rt > alert_rt,
            "Drowsy RT ({drowsy_rt}) should be greater than alert RT ({alert_rt})"
        );
    }

    #[test]
    fn test_trend_detection() {
        let mut monitor = AlertnessMonitor::with_defaults();
        // First few: alert
        for _ in 0..5 {
            monitor.assess(&alert_input());
        }
        // Then: drowsy
        for _ in 0..5 {
            monitor.assess(&drowsy_input());
        }
        let trend = monitor.trend();
        assert!(
            trend < 0.0,
            "Degrading alertness should produce negative trend, got {trend}"
        );
    }

    #[test]
    fn test_assessment_count() {
        let mut monitor = AlertnessMonitor::with_defaults();
        for _ in 0..7 {
            monitor.assess(&alert_input());
        }
        assert_eq!(monitor.assessment_count(), 7);
    }

    #[test]
    fn test_perclos_boundary() {
        let mut monitor = AlertnessMonitor::with_defaults();
        // PERCLOS at exactly threshold (0.15) should produce score 0
        let input = AlertnessInput {
            blink_rate: None,
            perclos: Some(0.15),
            head_deviation_deg: None,
            grip_pressure: None,
            time_since_last_input: 1.0,
            driving_duration_secs: 0.0,
        };
        let result = monitor.assess(&input);
        // PERCLOS at threshold → 0 contribution from perclos, but other signals still contribute
        assert!(result.score < 0.9);
    }

    #[test]
    fn test_long_driving_duration_penalty() {
        let mut monitor = AlertnessMonitor::with_defaults();
        let short_drive = AlertnessInput {
            blink_rate: Some(15.0),
            perclos: Some(0.02),
            head_deviation_deg: Some(5.0),
            grip_pressure: Some(0.8),
            time_since_last_input: 1.0,
            driving_duration_secs: 1800.0, // 30 min
        };
        let long_drive = AlertnessInput {
            driving_duration_secs: 18000.0, // 5 hours
            ..short_drive.clone()
        };
        let short_score = monitor.assess(&short_drive).score;
        let long_score = monitor.assess(&long_drive).score;
        assert!(
            long_score < short_score,
            "Long drive ({long_score}) should score lower than short drive ({short_score})"
        );
    }
}
