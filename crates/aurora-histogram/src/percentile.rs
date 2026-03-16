//! Percentile calculation — compute p50, p90, p95, p99, etc. from sorted data.

/// Compute a percentile from a sorted slice of values.
/// Percentile should be in [0, 100].
/// Uses linear interpolation between nearest ranks.
pub fn percentile(sorted: &[f64], p: f64) -> Option<f64> {
    if sorted.is_empty() || !(0.0..=100.0).contains(&p) {
        return None;
    }
    if sorted.len() == 1 {
        return Some(sorted[0]);
    }

    let rank = (p / 100.0) * (sorted.len() - 1) as f64;
    let lower = rank.floor() as usize;
    let upper = rank.ceil() as usize;
    let frac = rank - lower as f64;

    if lower == upper || upper >= sorted.len() {
        Some(sorted[lower])
    } else {
        Some(sorted[lower] * (1.0 - frac) + sorted[upper] * frac)
    }
}

/// Compute the median (p50).
pub fn median(sorted: &[f64]) -> Option<f64> {
    percentile(sorted, 50.0)
}

/// Compute standard percentile set: p50, p75, p90, p95, p99.
pub struct PercentileSet {
    pub p50: f64,
    pub p75: f64,
    pub p90: f64,
    pub p95: f64,
    pub p99: f64,
}

impl PercentileSet {
    /// Compute from a sorted slice.
    pub fn from_sorted(sorted: &[f64]) -> Option<Self> {
        Some(Self {
            p50: percentile(sorted, 50.0)?,
            p75: percentile(sorted, 75.0)?,
            p90: percentile(sorted, 90.0)?,
            p95: percentile(sorted, 95.0)?,
            p99: percentile(sorted, 99.0)?,
        })
    }

    /// Inter-quartile range (p75 - p50).
    pub fn iqr(&self) -> f64 {
        self.p75 - self.p50
    }
}

/// Compute percentile from unsorted data (sorts a copy internally).
pub fn percentile_unsorted(data: &[f64], p: f64) -> Option<f64> {
    if data.is_empty() {
        return None;
    }
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    percentile(&sorted, p)
}

/// Compute variance from a slice.
pub fn variance(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let mean = data.iter().sum::<f64>() / data.len() as f64;
    let sq_diff: f64 = data.iter().map(|v| (v - mean).powi(2)).sum();
    sq_diff / data.len() as f64
}

/// Compute standard deviation from a slice.
pub fn std_dev(data: &[f64]) -> f64 {
    variance(data).sqrt()
}

/// Skewness (asymmetry measure).
pub fn skewness(data: &[f64]) -> f64 {
    if data.len() < 3 {
        return 0.0;
    }
    let n = data.len() as f64;
    let mean = data.iter().sum::<f64>() / n;
    let sd = std_dev(data);
    if sd < 1e-15 {
        return 0.0;
    }
    let m3: f64 = data.iter().map(|v| ((v - mean) / sd).powi(3)).sum();
    m3 / n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentile_basic() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert!((percentile(&data, 0.0).unwrap() - 1.0).abs() < 1e-10);
        assert!((percentile(&data, 50.0).unwrap() - 3.0).abs() < 1e-10);
        assert!((percentile(&data, 100.0).unwrap() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_percentile_interpolation() {
        let data = vec![10.0, 20.0, 30.0, 40.0];
        let p25 = percentile(&data, 25.0).unwrap();
        assert!((p25 - 17.5).abs() < 1e-10); // between 10 and 20
    }

    #[test]
    fn test_median() {
        let odd = vec![1.0, 3.0, 5.0];
        assert!((median(&odd).unwrap() - 3.0).abs() < 1e-10);

        let even = vec![1.0, 3.0, 5.0, 7.0];
        assert!((median(&even).unwrap() - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_percentile_set() {
        let data: Vec<f64> = (1..=100).map(|i| i as f64).collect();
        let ps = PercentileSet::from_sorted(&data).unwrap();
        assert!((ps.p50 - 50.5).abs() < 0.5);
        assert!((ps.p90 - 90.1).abs() < 1.0);
        assert!((ps.p99 - 99.01).abs() < 1.0);
        assert!(ps.iqr() > 0.0);
    }

    #[test]
    fn test_empty_data() {
        assert!(percentile(&[], 50.0).is_none());
        assert!(median(&[]).is_none());
    }

    #[test]
    fn test_single_value() {
        assert!((percentile(&[42.0], 50.0).unwrap() - 42.0).abs() < 1e-10);
    }

    #[test]
    fn test_percentile_unsorted() {
        let data = vec![5.0, 1.0, 3.0, 2.0, 4.0];
        let p50 = percentile_unsorted(&data, 50.0).unwrap();
        assert!((p50 - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_variance_and_std_dev() {
        let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let var = variance(&data);
        assert!((var - 4.0).abs() < 0.01);
        assert!((std_dev(&data) - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_skewness_symmetric() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let sk = skewness(&data);
        assert!(sk.abs() < 0.01); // symmetric data → near-zero skewness
    }

    #[test]
    fn test_skewness_right() {
        let data = vec![1.0, 1.0, 1.0, 1.0, 10.0];
        let sk = skewness(&data);
        assert!(sk > 0.0); // right-skewed
    }
}
