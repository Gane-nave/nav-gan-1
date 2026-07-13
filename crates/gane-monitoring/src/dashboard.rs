//! Dashboard configuration generation.
//!
//! Generates Grafana-compatible dashboard JSON configurations for
//! monitoring G.A.N.E NAV system metrics, SLOs, and alerts.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// A dashboard panel type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PanelType {
    Graph,
    Gauge,
    Stat,
    Table,
    Heatmap,
    AlertList,
    Text,
}

impl PanelType {
    pub fn as_str(&self) -> &'static str {
        match self {
            PanelType::Graph => "graph",
            PanelType::Gauge => "gauge",
            PanelType::Stat => "stat",
            PanelType::Table => "table",
            PanelType::Heatmap => "heatmap",
            PanelType::AlertList => "alertlist",
            PanelType::Text => "text",
        }
    }
}

/// A single dashboard panel definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelDefinition {
    /// Panel title.
    pub title: String,
    /// Panel type.
    pub panel_type: PanelType,
    /// Prometheus query expression.
    pub query: String,
    /// Grid position (x, y, w, h).
    pub grid_pos: (u32, u32, u32, u32),
    /// Optional unit for display.
    pub unit: Option<String>,
    /// Optional thresholds for color coding.
    pub thresholds: Vec<(f64, String)>,
}

/// A dashboard row (group of panels).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardRow {
    /// Row title.
    pub title: String,
    /// Panels in this row.
    pub panels: Vec<PanelDefinition>,
}

/// A complete dashboard definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardDefinition {
    /// Dashboard title.
    pub title: String,
    /// Dashboard description.
    pub description: String,
    /// Dashboard UID (unique identifier).
    pub uid: String,
    /// Rows of panels.
    pub rows: Vec<DashboardRow>,
    /// Auto-refresh interval.
    pub refresh_interval: String,
    /// Time range.
    pub time_range: String,
    /// Dashboard tags.
    pub tags: Vec<String>,
}

// ---------------------------------------------------------------------------
// Dashboard builder
// ---------------------------------------------------------------------------

/// Builder for constructing dashboard definitions.
pub struct DashboardBuilder {
    title: String,
    description: String,
    uid: String,
    rows: Vec<DashboardRow>,
    refresh: String,
    time_range: String,
    tags: Vec<String>,
}

impl DashboardBuilder {
    pub fn new(title: &str, uid: &str) -> Self {
        Self {
            title: title.to_string(),
            description: String::new(),
            uid: uid.to_string(),
            rows: Vec::new(),
            refresh: "30s".to_string(),
            time_range: "1h".to_string(),
            tags: Vec::new(),
        }
    }

    pub fn description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    pub fn refresh_interval(mut self, interval: &str) -> Self {
        self.refresh = interval.to_string();
        self
    }

    pub fn time_range(mut self, range: &str) -> Self {
        self.time_range = range.to_string();
        self
    }

    pub fn tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    pub fn add_row(mut self, row: DashboardRow) -> Self {
        self.rows.push(row);
        self
    }

    pub fn build(self) -> DashboardDefinition {
        DashboardDefinition {
            title: self.title,
            description: self.description,
            uid: self.uid,
            rows: self.rows,
            refresh_interval: self.refresh,
            time_range: self.time_range,
            tags: self.tags,
        }
    }
}

// ---------------------------------------------------------------------------
// Default dashboards
// ---------------------------------------------------------------------------

/// Build the default G.A.N.E NAV overview dashboard.
pub fn gane_overview_dashboard() -> DashboardDefinition {
    DashboardBuilder::new("G.A.N.E NAV Overview", "gane-overview")
        .description("System-wide overview of G.A.N.E NAV")
        .refresh_interval("30s")
        .time_range("1h")
        .tag("gane")
        .tag("overview")
        .add_row(DashboardRow {
            title: "System Health".to_string(),
            panels: vec![
                PanelDefinition {
                    title: "API Request Rate".to_string(),
                    panel_type: PanelType::Graph,
                    query: "rate(gane_http_requests_total[5m])".to_string(),
                    grid_pos: (0, 0, 12, 8),
                    unit: Some("reqps".to_string()),
                    thresholds: vec![],
                },
                PanelDefinition {
                    title: "Error Rate".to_string(),
                    panel_type: PanelType::Gauge,
                    query: "rate(gane_http_errors_total[5m]) / rate(gane_http_requests_total[5m])"
                        .to_string(),
                    grid_pos: (12, 0, 6, 8),
                    unit: Some("percentunit".to_string()),
                    thresholds: vec![
                        (0.01, "green".to_string()),
                        (0.05, "yellow".to_string()),
                        (0.1, "red".to_string()),
                    ],
                },
                PanelDefinition {
                    title: "Active Sessions".to_string(),
                    panel_type: PanelType::Stat,
                    query: "gane_active_sessions".to_string(),
                    grid_pos: (18, 0, 6, 8),
                    unit: None,
                    thresholds: vec![],
                },
            ],
        })
        .add_row(DashboardRow {
            title: "Navigation".to_string(),
            panels: vec![
                PanelDefinition {
                    title: "GNSS Tracked Satellites".to_string(),
                    panel_type: PanelType::Graph,
                    query: "gane_gnss_tracked_satellites".to_string(),
                    grid_pos: (0, 8, 12, 8),
                    unit: None,
                    thresholds: vec![
                        (4.0, "red".to_string()),
                        (8.0, "yellow".to_string()),
                        (12.0, "green".to_string()),
                    ],
                },
                PanelDefinition {
                    title: "Position Accuracy (m)".to_string(),
                    panel_type: PanelType::Graph,
                    query: "gane_position_accuracy_meters".to_string(),
                    grid_pos: (12, 8, 12, 8),
                    unit: Some("m".to_string()),
                    thresholds: vec![
                        (1.0, "green".to_string()),
                        (5.0, "yellow".to_string()),
                        (10.0, "red".to_string()),
                    ],
                },
            ],
        })
        .add_row(DashboardRow {
            title: "SLO Compliance".to_string(),
            panels: vec![
                PanelDefinition {
                    title: "API Availability SLO".to_string(),
                    panel_type: PanelType::Gauge,
                    query: "gane_slo_api_availability_ratio".to_string(),
                    grid_pos: (0, 16, 8, 8),
                    unit: Some("percentunit".to_string()),
                    thresholds: vec![
                        (0.99, "red".to_string()),
                        (0.995, "yellow".to_string()),
                        (0.999, "green".to_string()),
                    ],
                },
                PanelDefinition {
                    title: "Error Budget Remaining".to_string(),
                    panel_type: PanelType::Gauge,
                    query: "gane_slo_error_budget_remaining".to_string(),
                    grid_pos: (8, 16, 8, 8),
                    unit: Some("percentunit".to_string()),
                    thresholds: vec![
                        (0.0, "red".to_string()),
                        (0.25, "yellow".to_string()),
                        (0.5, "green".to_string()),
                    ],
                },
                PanelDefinition {
                    title: "Burn Rate".to_string(),
                    panel_type: PanelType::Stat,
                    query: "gane_slo_burn_rate".to_string(),
                    grid_pos: (16, 16, 8, 8),
                    unit: Some("x".to_string()),
                    thresholds: vec![
                        (1.0, "green".to_string()),
                        (2.0, "yellow".to_string()),
                        (5.0, "red".to_string()),
                    ],
                },
            ],
        })
        .build()
}

/// Build the G.A.N.E NAV navigation detail dashboard.
pub fn gane_navigation_dashboard() -> DashboardDefinition {
    DashboardBuilder::new("G.A.N.E NAV Navigation", "gane-navigation")
        .description("Detailed navigation metrics for G.A.N.E NAV")
        .refresh_interval("10s")
        .time_range("30m")
        .tag("gane")
        .tag("navigation")
        .add_row(DashboardRow {
            title: "GNSS & Fusion".to_string(),
            panels: vec![
                PanelDefinition {
                    title: "Constellation Health".to_string(),
                    panel_type: PanelType::Table,
                    query: "gane_gnss_constellation_status".to_string(),
                    grid_pos: (0, 0, 24, 8),
                    unit: None,
                    thresholds: vec![],
                },
                PanelDefinition {
                    title: "Fusion Measurement Rate".to_string(),
                    panel_type: PanelType::Graph,
                    query: "rate(gane_fusion_measurements_total[1m])".to_string(),
                    grid_pos: (0, 8, 12, 8),
                    unit: Some("ops".to_string()),
                    thresholds: vec![],
                },
                PanelDefinition {
                    title: "Integrity Score".to_string(),
                    panel_type: PanelType::Graph,
                    query: "gane_integrity_score".to_string(),
                    grid_pos: (12, 8, 12, 8),
                    unit: None,
                    thresholds: vec![
                        (0.8, "red".to_string()),
                        (0.9, "yellow".to_string()),
                        (0.95, "green".to_string()),
                    ],
                },
            ],
        })
        .build()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_produces_valid_dashboard() {
        let dash = DashboardBuilder::new("Test", "test-uid")
            .description("Test dashboard")
            .refresh_interval("10s")
            .time_range("6h")
            .tag("test")
            .build();

        assert_eq!(dash.title, "Test");
        assert_eq!(dash.uid, "test-uid");
        assert_eq!(dash.refresh_interval, "10s");
        assert_eq!(dash.time_range, "6h");
        assert_eq!(dash.tags, vec!["test"]);
    }

    #[test]
    fn builder_with_rows() {
        let dash = DashboardBuilder::new("Test", "uid")
            .add_row(DashboardRow {
                title: "Row 1".to_string(),
                panels: vec![PanelDefinition {
                    title: "Panel 1".to_string(),
                    panel_type: PanelType::Graph,
                    query: "up".to_string(),
                    grid_pos: (0, 0, 24, 8),
                    unit: None,
                    thresholds: vec![],
                }],
            })
            .build();

        assert_eq!(dash.rows.len(), 1);
        assert_eq!(dash.rows[0].panels.len(), 1);
        assert_eq!(dash.rows[0].panels[0].title, "Panel 1");
    }

    #[test]
    fn overview_dashboard_structure() {
        let dash = gane_overview_dashboard();
        assert_eq!(dash.uid, "gane-overview");
        assert_eq!(dash.rows.len(), 3);
        assert!(dash.tags.contains(&"gane".to_string()));
    }

    #[test]
    fn navigation_dashboard_structure() {
        let dash = gane_navigation_dashboard();
        assert_eq!(dash.uid, "gane-navigation");
        assert!(!dash.rows.is_empty());
        assert!(dash.tags.contains(&"navigation".to_string()));
    }

    #[test]
    fn panel_type_as_str() {
        assert_eq!(PanelType::Graph.as_str(), "graph");
        assert_eq!(PanelType::Gauge.as_str(), "gauge");
        assert_eq!(PanelType::Stat.as_str(), "stat");
        assert_eq!(PanelType::Table.as_str(), "table");
        assert_eq!(PanelType::Heatmap.as_str(), "heatmap");
        assert_eq!(PanelType::AlertList.as_str(), "alertlist");
        assert_eq!(PanelType::Text.as_str(), "text");
    }

    #[test]
    fn dashboard_serializable() {
        let dash = gane_overview_dashboard();
        let json = serde_json::to_string(&dash).unwrap();
        assert!(json.contains("gane-overview"));
        assert!(json.contains("gane_http_requests_total"));
    }

    #[test]
    fn panel_thresholds() {
        let dash = gane_overview_dashboard();
        let error_panel = &dash.rows[0].panels[1];
        assert!(!error_panel.thresholds.is_empty());
        assert_eq!(error_panel.thresholds[0].1, "green");
    }

    #[test]
    fn multiple_tags() {
        let dash = DashboardBuilder::new("T", "u")
            .tag("a")
            .tag("b")
            .tag("c")
            .build();
        assert_eq!(dash.tags.len(), 3);
    }

    #[test]
    fn default_refresh_interval() {
        let dash = DashboardBuilder::new("T", "u").build();
        assert_eq!(dash.refresh_interval, "30s");
    }
}
