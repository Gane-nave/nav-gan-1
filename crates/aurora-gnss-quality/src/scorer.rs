use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub satellites_tracked: u32,
    pub satellites_used: u32,
    pub avg_snr_dbhz: f64,
    pub min_snr_dbhz: f64,
    pub max_snr_dbhz: f64,
    pub hdop: f64,
    pub vdop: f64,
    pub pdop: f64,
    pub horizontal_accuracy_m: f64,
    pub vertical_accuracy_m: f64,
    pub multipath_detected: bool,
    pub multipath_severity: f64,
    pub fix_latency_ms: u64,
    pub overall_score: f64,
    pub geometry_score: f64,
    pub signal_score: f64,
}

pub struct QualityScorer {
    history: Vec<QualityMetrics>,
    max_history: usize,
    multipath_threshold: f64,
    evaluations: u64,
}

impl QualityScorer {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            max_history: 1000,
            multipath_threshold: 0.3,
            evaluations: 0,
        }
    }
    pub fn evaluate(
        &mut self,
        tracked: u32,
        used: u32,
        snrs: &[f64],
        hdop: f64,
        vdop: f64,
        fix_latency_ms: u64,
    ) -> QualityMetrics {
        let avg_snr = if snrs.is_empty() {
            0.0
        } else {
            snrs.iter().sum::<f64>() / snrs.len() as f64
        };
        let min_snr = snrs.iter().copied().fold(f64::MAX, f64::min);
        let max_snr = snrs.iter().copied().fold(f64::MIN, f64::max);
        let pdop = (hdop * hdop + vdop * vdop).sqrt();
        let var = if snrs.len() > 1 {
            snrs.iter().map(|s| (s - avg_snr).powi(2)).sum::<f64>() / snrs.len() as f64
        } else {
            0.0
        };
        let multipath = var > self.multipath_threshold * 100.0;
        let signal_score = (avg_snr / 45.0).min(1.0);
        let geometry_score = (1.0 / pdop.max(0.1)).min(1.0);
        let overall = signal_score * 0.3
            + geometry_score * 0.3
            + (used as f64 / 12.0).min(1.0) * 0.25
            + (1.0 - fix_latency_ms as f64 / 2000.0).max(0.0) * 0.15;
        let m = QualityMetrics {
            satellites_tracked: tracked,
            satellites_used: used,
            avg_snr_dbhz: avg_snr,
            min_snr_dbhz: if snrs.is_empty() { 0.0 } else { min_snr },
            max_snr_dbhz: if snrs.is_empty() { 0.0 } else { max_snr },
            hdop,
            vdop,
            pdop,
            horizontal_accuracy_m: hdop * 2.5,
            vertical_accuracy_m: vdop * 3.0,
            multipath_detected: multipath,
            multipath_severity: (var / 200.0).min(1.0),
            fix_latency_ms,
            overall_score: overall,
            geometry_score,
            signal_score,
        };
        self.history.push(m.clone());
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
        self.evaluations += 1;
        m
    }
    pub fn latest(&self) -> Option<&QualityMetrics> {
        self.history.last()
    }
    pub fn history_len(&self) -> usize {
        self.history.len()
    }
    pub fn evaluations(&self) -> u64 {
        self.evaluations
    }
    pub fn avg_score(&self) -> f64 {
        if self.history.is_empty() {
            0.0
        } else {
            self.history.iter().map(|m| m.overall_score).sum::<f64>() / self.history.len() as f64
        }
    }
    pub fn set_multipath_threshold(&mut self, t: f64) {
        self.multipath_threshold = t.max(0.0);
    }
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

impl Default for QualityScorer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_s() {
        assert_eq!(QualityScorer::new().evaluations(), 0);
    }
    #[test]
    fn default_s() {
        assert_eq!(QualityScorer::default().history_len(), 0);
    }
    #[test]
    fn eval_basic() {
        let mut s = QualityScorer::new();
        let m = s.evaluate(12, 10, &[35.0, 40.0, 38.0], 1.2, 1.8, 100);
        assert!(m.overall_score > 0.0);
    }
    #[test]
    fn empty_snrs() {
        let mut s = QualityScorer::new();
        let m = s.evaluate(0, 0, &[], 99.0, 99.0, 5000);
        assert_eq!(m.avg_snr_dbhz, 0.0);
    }
    #[test]
    fn hist_cap() {
        let mut s = QualityScorer::new();
        for _ in 0..1100 {
            s.evaluate(10, 8, &[40.0], 1.0, 1.5, 50);
        }
        assert!(s.history_len() <= 1000);
    }
    #[test]
    fn avg_s() {
        let mut s = QualityScorer::new();
        s.evaluate(10, 8, &[40.0], 1.0, 1.5, 50);
        assert!(s.avg_score() > 0.0);
    }
    #[test]
    fn multipath() {
        let mut s = QualityScorer::new();
        let m = s.evaluate(10, 8, &[10.0, 50.0, 15.0, 48.0, 12.0, 45.0], 1.0, 1.5, 50);
        assert!(m.multipath_severity > 0.0);
    }
    #[test]
    fn clear_h() {
        let mut s = QualityScorer::new();
        s.evaluate(10, 8, &[40.0], 1.0, 1.5, 50);
        s.clear_history();
        assert_eq!(s.history_len(), 0);
    }
}
