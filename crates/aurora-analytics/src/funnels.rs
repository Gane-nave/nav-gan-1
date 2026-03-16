//! Funnel analysis — tracks user progression through multi-step flows
//! (e.g. search → route → navigate → arrive) and identifies drop-off points.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Definition of a funnel — an ordered sequence of steps a user should complete.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunnelDefinition {
    pub id: Uuid,
    pub name: String,
    pub steps: Vec<FunnelStep>,
}

/// A single step in a funnel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunnelStep {
    pub name: String,
    /// Event name that triggers this step.
    pub event_name: String,
    /// Maximum seconds allowed between this step and the previous one.
    /// If exceeded, the user is considered to have dropped off.
    pub max_gap_s: u64,
}

/// Result of analyzing a funnel for a set of users.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunnelResult {
    pub funnel_id: Uuid,
    pub funnel_name: String,
    pub step_counts: Vec<StepCount>,
    pub overall_conversion_rate: f64,
}

/// Count and conversion for a single funnel step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepCount {
    pub step_name: String,
    pub entered: u64,
    pub conversion_rate: f64,
}

/// A timestamped event used for funnel analysis.
#[derive(Debug, Clone)]
pub struct FunnelEvent {
    pub user_id: Uuid,
    pub event_name: String,
    pub timestamp: DateTime<Utc>,
}

/// Funnel analyzer that processes events and computes conversion rates.
pub struct FunnelAnalyzer {
    definitions: Vec<FunnelDefinition>,
}

impl FunnelAnalyzer {
    /// Create a new analyzer with funnel definitions.
    pub fn new(definitions: Vec<FunnelDefinition>) -> Self {
        Self { definitions }
    }

    /// Add a funnel definition.
    pub fn add_funnel(&mut self, def: FunnelDefinition) {
        self.definitions.push(def);
    }

    /// Analyze a specific funnel against a set of events.
    pub fn analyze(&self, funnel_id: Uuid, events: &[FunnelEvent]) -> Option<FunnelResult> {
        let def = self.definitions.iter().find(|d| d.id == funnel_id)?;

        if def.steps.is_empty() {
            return Some(FunnelResult {
                funnel_id,
                funnel_name: def.name.clone(),
                step_counts: Vec::new(),
                overall_conversion_rate: 0.0,
            });
        }

        // Group events by user, sorted by timestamp
        let mut by_user: HashMap<Uuid, Vec<&FunnelEvent>> = HashMap::new();
        for e in events {
            by_user.entry(e.user_id).or_default().push(e);
        }
        for user_events in by_user.values_mut() {
            user_events.sort_by_key(|e| e.timestamp);
        }

        let total_users = by_user.len() as u64;
        let mut step_counts: Vec<u64> = vec![0; def.steps.len()];

        for user_events in by_user.values() {
            let mut step_idx = 0;
            let mut last_time: Option<DateTime<Utc>> = None;

            for event in user_events.iter() {
                if step_idx >= def.steps.len() {
                    break;
                }
                let step = &def.steps[step_idx];
                if event.event_name != step.event_name {
                    continue;
                }

                // Check gap constraint (skip for first step)
                if let Some(prev_time) = last_time {
                    let gap = (event.timestamp - prev_time).num_seconds().unsigned_abs();
                    if gap > step.max_gap_s {
                        break; // dropped off
                    }
                }

                step_counts[step_idx] += 1;
                last_time = Some(event.timestamp);
                step_idx += 1;
            }
        }

        let step_results: Vec<StepCount> = def
            .steps
            .iter()
            .enumerate()
            .map(|(i, step)| {
                let entered = step_counts[i];
                let base = if i == 0 {
                    total_users
                } else {
                    step_counts[i - 1]
                };
                let rate = if base > 0 {
                    entered as f64 / base as f64
                } else {
                    0.0
                };
                StepCount {
                    step_name: step.name.clone(),
                    entered,
                    conversion_rate: rate,
                }
            })
            .collect();

        let overall = if total_users > 0 {
            *step_counts.last().unwrap_or(&0) as f64 / total_users as f64
        } else {
            0.0
        };

        Some(FunnelResult {
            funnel_id,
            funnel_name: def.name.clone(),
            step_counts: step_results,
            overall_conversion_rate: overall,
        })
    }

    /// Get all funnel definitions.
    pub fn definitions(&self) -> &[FunnelDefinition] {
        &self.definitions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn nav_funnel() -> FunnelDefinition {
        FunnelDefinition {
            id: Uuid::new_v4(),
            name: "Navigation Flow".to_string(),
            steps: vec![
                FunnelStep {
                    name: "Search".to_string(),
                    event_name: "search".to_string(),
                    max_gap_s: 3600,
                },
                FunnelStep {
                    name: "Route Preview".to_string(),
                    event_name: "route_preview".to_string(),
                    max_gap_s: 300,
                },
                FunnelStep {
                    name: "Start Navigation".to_string(),
                    event_name: "nav_start".to_string(),
                    max_gap_s: 120,
                },
                FunnelStep {
                    name: "Arrive".to_string(),
                    event_name: "arrive".to_string(),
                    max_gap_s: 7200,
                },
            ],
        }
    }

    #[test]
    fn test_full_conversion() {
        let funnel = nav_funnel();
        let fid = funnel.id;
        let analyzer = FunnelAnalyzer::new(vec![funnel]);
        let user = Uuid::new_v4();
        let t0 = Utc::now();

        let events = vec![
            FunnelEvent {
                user_id: user,
                event_name: "search".into(),
                timestamp: t0,
            },
            FunnelEvent {
                user_id: user,
                event_name: "route_preview".into(),
                timestamp: t0 + Duration::seconds(10),
            },
            FunnelEvent {
                user_id: user,
                event_name: "nav_start".into(),
                timestamp: t0 + Duration::seconds(20),
            },
            FunnelEvent {
                user_id: user,
                event_name: "arrive".into(),
                timestamp: t0 + Duration::seconds(600),
            },
        ];

        let result = analyzer.analyze(fid, &events).unwrap();
        assert_eq!(result.step_counts.len(), 4);
        for sc in &result.step_counts {
            assert_eq!(sc.entered, 1);
        }
        assert!((result.overall_conversion_rate - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_drop_off_at_step_3() {
        let funnel = nav_funnel();
        let fid = funnel.id;
        let analyzer = FunnelAnalyzer::new(vec![funnel]);
        let user = Uuid::new_v4();
        let t0 = Utc::now();

        // User searches and previews but never starts navigation
        let events = vec![
            FunnelEvent {
                user_id: user,
                event_name: "search".into(),
                timestamp: t0,
            },
            FunnelEvent {
                user_id: user,
                event_name: "route_preview".into(),
                timestamp: t0 + Duration::seconds(10),
            },
        ];

        let result = analyzer.analyze(fid, &events).unwrap();
        assert_eq!(result.step_counts[0].entered, 1);
        assert_eq!(result.step_counts[1].entered, 1);
        assert_eq!(result.step_counts[2].entered, 0); // dropped off
        assert_eq!(result.step_counts[3].entered, 0);
        assert!((result.overall_conversion_rate - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_gap_timeout_causes_dropout() {
        let funnel = nav_funnel();
        let fid = funnel.id;
        let analyzer = FunnelAnalyzer::new(vec![funnel]);
        let user = Uuid::new_v4();
        let t0 = Utc::now();

        // Gap between route_preview and nav_start exceeds 120s
        let events = vec![
            FunnelEvent {
                user_id: user,
                event_name: "search".into(),
                timestamp: t0,
            },
            FunnelEvent {
                user_id: user,
                event_name: "route_preview".into(),
                timestamp: t0 + Duration::seconds(10),
            },
            FunnelEvent {
                user_id: user,
                event_name: "nav_start".into(),
                timestamp: t0 + Duration::seconds(500),
            }, // >120s gap
            FunnelEvent {
                user_id: user,
                event_name: "arrive".into(),
                timestamp: t0 + Duration::seconds(600),
            },
        ];

        let result = analyzer.analyze(fid, &events).unwrap();
        assert_eq!(result.step_counts[0].entered, 1);
        assert_eq!(result.step_counts[1].entered, 1);
        assert_eq!(result.step_counts[2].entered, 0); // timed out
        assert_eq!(result.step_counts[3].entered, 0);
    }

    #[test]
    fn test_multiple_users_partial_conversion() {
        let funnel = nav_funnel();
        let fid = funnel.id;
        let analyzer = FunnelAnalyzer::new(vec![funnel]);
        let u1 = Uuid::new_v4();
        let u2 = Uuid::new_v4();
        let t0 = Utc::now();

        let events = vec![
            // User 1: full funnel
            FunnelEvent {
                user_id: u1,
                event_name: "search".into(),
                timestamp: t0,
            },
            FunnelEvent {
                user_id: u1,
                event_name: "route_preview".into(),
                timestamp: t0 + Duration::seconds(10),
            },
            FunnelEvent {
                user_id: u1,
                event_name: "nav_start".into(),
                timestamp: t0 + Duration::seconds(20),
            },
            FunnelEvent {
                user_id: u1,
                event_name: "arrive".into(),
                timestamp: t0 + Duration::seconds(600),
            },
            // User 2: only search
            FunnelEvent {
                user_id: u2,
                event_name: "search".into(),
                timestamp: t0,
            },
        ];

        let result = analyzer.analyze(fid, &events).unwrap();
        assert_eq!(result.step_counts[0].entered, 2); // both searched
        assert_eq!(result.step_counts[1].entered, 1); // only u1
        assert_eq!(result.step_counts[3].entered, 1); // only u1 arrived
        assert!((result.overall_conversion_rate - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_empty_funnel() {
        let funnel = FunnelDefinition {
            id: Uuid::new_v4(),
            name: "Empty".into(),
            steps: vec![],
        };
        let fid = funnel.id;
        let analyzer = FunnelAnalyzer::new(vec![funnel]);
        let result = analyzer.analyze(fid, &[]).unwrap();
        assert!(result.step_counts.is_empty());
    }

    #[test]
    fn test_unknown_funnel() {
        let analyzer = FunnelAnalyzer::new(vec![]);
        assert!(analyzer.analyze(Uuid::new_v4(), &[]).is_none());
    }

    #[test]
    fn test_serialization() {
        let result = FunnelResult {
            funnel_id: Uuid::new_v4(),
            funnel_name: "Test".into(),
            step_counts: vec![StepCount {
                step_name: "Step 1".into(),
                entered: 100,
                conversion_rate: 0.85,
            }],
            overall_conversion_rate: 0.85,
        };
        let json = serde_json::to_string(&result).unwrap();
        let de: FunnelResult = serde_json::from_str(&json).unwrap();
        assert_eq!(de.funnel_name, "Test");
        assert_eq!(de.step_counts[0].entered, 100);
    }
}
