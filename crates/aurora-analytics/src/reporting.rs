//! Reporting — generates structured reports from analytics data,
//! including KPI dashboards, trend analysis, and export formats.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A key performance indicator value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kpi {
    pub name: String,
    pub value: f64,
    pub unit: String,
    pub trend: Trend,
    pub timestamp: DateTime<Utc>,
}

/// Trend direction of a KPI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Trend {
    Up,
    Down,
    Stable,
}

/// A data point in a time series.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesPoint {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
}

/// A time series with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeries {
    pub name: String,
    pub unit: String,
    pub points: Vec<TimeSeriesPoint>,
}

impl TimeSeries {
    /// Create a new time series.
    pub fn new(name: &str, unit: &str) -> Self {
        Self {
            name: name.to_string(),
            unit: unit.to_string(),
            points: Vec::new(),
        }
    }

    /// Add a data point.
    pub fn add_point(&mut self, timestamp: DateTime<Utc>, value: f64) {
        self.points.push(TimeSeriesPoint { timestamp, value });
    }

    /// Calculate the mean value.
    pub fn mean(&self) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.points.iter().map(|p| p.value).sum();
        sum / self.points.len() as f64
    }

    /// Calculate the min value.
    pub fn min(&self) -> f64 {
        self.points
            .iter()
            .map(|p| p.value)
            .fold(f64::INFINITY, f64::min)
    }

    /// Calculate the max value.
    pub fn max(&self) -> f64 {
        self.points
            .iter()
            .map(|p| p.value)
            .fold(f64::NEG_INFINITY, f64::max)
    }

    /// Compute trend from first half to second half.
    pub fn trend(&self) -> Trend {
        if self.points.len() < 2 {
            return Trend::Stable;
        }
        let mid = self.points.len() / 2;
        let first_half_avg: f64 =
            self.points[..mid].iter().map(|p| p.value).sum::<f64>() / mid as f64;
        let second_half_avg: f64 = self.points[mid..].iter().map(|p| p.value).sum::<f64>()
            / (self.points.len() - mid) as f64;

        let threshold = first_half_avg.abs() * 0.05; // 5% change threshold
        if second_half_avg > first_half_avg + threshold {
            Trend::Up
        } else if second_half_avg < first_half_avg - threshold {
            Trend::Down
        } else {
            Trend::Stable
        }
    }

    /// Compute a simple moving average with the given window size.
    pub fn moving_average(&self, window: usize) -> Vec<TimeSeriesPoint> {
        if window == 0 || self.points.is_empty() {
            return Vec::new();
        }
        self.points
            .windows(window)
            .map(|w| {
                let avg = w.iter().map(|p| p.value).sum::<f64>() / w.len() as f64;
                TimeSeriesPoint {
                    timestamp: w.last().unwrap().timestamp,
                    value: avg,
                }
            })
            .collect()
    }
}

/// Report format for export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportFormat {
    Json,
    Csv,
}

/// A complete analytics report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub title: String,
    pub generated_at: DateTime<Utc>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub kpis: Vec<Kpi>,
    pub series: Vec<TimeSeries>,
    pub metadata: HashMap<String, String>,
}

impl Report {
    /// Create a new report.
    pub fn new(title: &str, period_start: DateTime<Utc>, period_end: DateTime<Utc>) -> Self {
        Self {
            title: title.to_string(),
            generated_at: Utc::now(),
            period_start,
            period_end,
            kpis: Vec::new(),
            series: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add a KPI to the report.
    pub fn add_kpi(&mut self, kpi: Kpi) {
        self.kpis.push(kpi);
    }

    /// Add a time series to the report.
    pub fn add_series(&mut self, series: TimeSeries) {
        self.series.push(series);
    }

    /// Export the report to JSON string.
    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Export the report as CSV lines.
    pub fn export_csv(&self) -> String {
        let mut lines = Vec::new();

        // KPIs section
        lines.push("section,name,value,unit,trend".to_string());
        for kpi in &self.kpis {
            lines.push(format!(
                "kpi,{},{},{},{:?}",
                kpi.name, kpi.value, kpi.unit, kpi.trend
            ));
        }

        // Time series section
        lines.push(String::new());
        lines.push("series,timestamp,value".to_string());
        for ts in &self.series {
            for p in &ts.points {
                lines.push(format!(
                    "{},{},{}",
                    ts.name,
                    p.timestamp.to_rfc3339(),
                    p.value
                ));
            }
        }

        lines.join("\n")
    }
}

/// Report builder with fluent API.
pub struct ReportBuilder {
    report: Report,
}

impl ReportBuilder {
    /// Start building a report.
    pub fn new(title: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self {
            report: Report::new(title, start, end),
        }
    }

    /// Add a KPI.
    pub fn kpi(mut self, name: &str, value: f64, unit: &str, trend: Trend) -> Self {
        self.report.add_kpi(Kpi {
            name: name.to_string(),
            value,
            unit: unit.to_string(),
            trend,
            timestamp: Utc::now(),
        });
        self
    }

    /// Add a time series.
    pub fn series(mut self, series: TimeSeries) -> Self {
        self.report.add_series(series);
        self
    }

    /// Add metadata.
    pub fn metadata(mut self, key: &str, value: &str) -> Self {
        self.report
            .metadata
            .insert(key.to_string(), value.to_string());
        self
    }

    /// Build the report.
    pub fn build(self) -> Report {
        self.report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_time_series_stats() {
        let mut ts = TimeSeries::new("latency", "ms");
        let t0 = Utc::now();
        for i in 0..10 {
            ts.add_point(t0 + Duration::seconds(i), (i * 10) as f64);
        }
        assert!((ts.mean() - 45.0).abs() < f64::EPSILON);
        assert!((ts.min() - 0.0).abs() < f64::EPSILON);
        assert!((ts.max() - 90.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_trend_detection() {
        let t0 = Utc::now();

        // Upward trend
        let mut up = TimeSeries::new("usage", "count");
        for i in 0..10 {
            up.add_point(t0 + Duration::seconds(i), (i * 10) as f64);
        }
        assert_eq!(up.trend(), Trend::Up);

        // Downward trend
        let mut down = TimeSeries::new("errors", "count");
        for i in 0..10 {
            down.add_point(t0 + Duration::seconds(i), (100 - i * 10) as f64);
        }
        assert_eq!(down.trend(), Trend::Down);

        // Stable
        let mut stable = TimeSeries::new("const", "count");
        for i in 0..10 {
            stable.add_point(t0 + Duration::seconds(i), 50.0);
        }
        assert_eq!(stable.trend(), Trend::Stable);
    }

    #[test]
    fn test_moving_average() {
        let mut ts = TimeSeries::new("speed", "km/h");
        let t0 = Utc::now();
        let values = [10.0, 20.0, 30.0, 40.0, 50.0];
        for (i, v) in values.iter().enumerate() {
            ts.add_point(t0 + Duration::seconds(i as i64), *v);
        }
        let ma = ts.moving_average(3);
        assert_eq!(ma.len(), 3); // 5 - 3 + 1
        assert!((ma[0].value - 20.0).abs() < f64::EPSILON); // avg(10,20,30)
        assert!((ma[1].value - 30.0).abs() < f64::EPSILON); // avg(20,30,40)
        assert!((ma[2].value - 40.0).abs() < f64::EPSILON); // avg(30,40,50)
    }

    #[test]
    fn test_empty_series() {
        let ts = TimeSeries::new("empty", "n/a");
        assert!((ts.mean() - 0.0).abs() < f64::EPSILON);
        assert_eq!(ts.trend(), Trend::Stable);
        assert!(ts.moving_average(3).is_empty());
    }

    #[test]
    fn test_report_builder() {
        let t0 = Utc::now();
        let report = ReportBuilder::new("Weekly Report", t0 - Duration::days(7), t0)
            .kpi("Active Users", 1500.0, "users", Trend::Up)
            .kpi("Avg Trip Duration", 22.5, "min", Trend::Stable)
            .metadata("region", "US-East")
            .build();

        assert_eq!(report.title, "Weekly Report");
        assert_eq!(report.kpis.len(), 2);
        assert_eq!(report.kpis[0].name, "Active Users");
        assert_eq!(report.metadata["region"], "US-East");
    }

    #[test]
    fn test_export_json() {
        let t0 = Utc::now();
        let report = ReportBuilder::new("Test", t0, t0)
            .kpi("Users", 100.0, "count", Trend::Up)
            .build();
        let json = report.export_json().unwrap();
        assert!(json.contains("\"Users\""));
        assert!(json.contains("100.0"));
    }

    #[test]
    fn test_export_csv() {
        let t0 = Utc::now();
        let mut ts = TimeSeries::new("speed", "km/h");
        ts.add_point(t0, 60.0);
        let report = ReportBuilder::new("Test", t0, t0)
            .kpi("Users", 100.0, "count", Trend::Up)
            .series(ts)
            .build();
        let csv = report.export_csv();
        assert!(csv.contains("kpi,Users,100,count,Up"));
        assert!(csv.contains("speed,"));
        assert!(csv.contains(",60"));
    }

    #[test]
    fn test_serialization_roundtrip() {
        let t0 = Utc::now();
        let report = ReportBuilder::new("Test", t0, t0)
            .kpi("K1", 42.0, "n", Trend::Down)
            .build();
        let json = serde_json::to_string(&report).unwrap();
        let de: Report = serde_json::from_str(&json).unwrap();
        assert_eq!(de.title, "Test");
        assert_eq!(de.kpis[0].value, 42.0);
    }
}
