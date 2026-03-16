//! Cohort segmentation — groups users by shared characteristics
//! (signup date, region, device type, usage pattern) for comparative analysis.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Criterion for assigning users to a cohort.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CohortCriterion {
    /// Users who first appeared within a date range.
    FirstSeenRange {
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    },
    /// Users in a geographic region (bounding box).
    Region {
        min_lat: f64,
        max_lat: f64,
        min_lon: f64,
        max_lon: f64,
    },
    /// Users matching a specific property value.
    Property { key: String, value: String },
    /// Users with event count above a threshold.
    HighActivity { min_events: u64 },
}

/// A cohort definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortDefinition {
    pub id: Uuid,
    pub name: String,
    pub criterion: CohortCriterion,
}

/// A user record used for cohort assignment.
#[derive(Debug, Clone)]
pub struct UserRecord {
    pub user_id: Uuid,
    pub first_seen: DateTime<Utc>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub properties: HashMap<String, String>,
    pub event_count: u64,
}

/// Result of cohort analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortResult {
    pub cohort_id: Uuid,
    pub cohort_name: String,
    pub member_count: usize,
    pub members: Vec<Uuid>,
}

/// Cohort engine that assigns users to cohorts and provides comparison metrics.
pub struct CohortEngine {
    definitions: Vec<CohortDefinition>,
}

impl CohortEngine {
    /// Create a new engine with definitions.
    pub fn new(definitions: Vec<CohortDefinition>) -> Self {
        Self { definitions }
    }

    /// Add a cohort definition.
    pub fn add_cohort(&mut self, def: CohortDefinition) {
        self.definitions.push(def);
    }

    /// Check if a user matches a criterion.
    pub fn matches(criterion: &CohortCriterion, user: &UserRecord) -> bool {
        match criterion {
            CohortCriterion::FirstSeenRange { from, to } => {
                user.first_seen >= *from && user.first_seen <= *to
            }
            CohortCriterion::Region {
                min_lat,
                max_lat,
                min_lon,
                max_lon,
            } => {
                if let (Some(lat), Some(lon)) = (user.lat, user.lon) {
                    lat >= *min_lat && lat <= *max_lat && lon >= *min_lon && lon <= *max_lon
                } else {
                    false
                }
            }
            CohortCriterion::Property { key, value } => user.properties.get(key) == Some(value),
            CohortCriterion::HighActivity { min_events } => user.event_count >= *min_events,
        }
    }

    /// Assign users to a specific cohort.
    pub fn assign(&self, cohort_id: Uuid, users: &[UserRecord]) -> Option<CohortResult> {
        let def = self.definitions.iter().find(|d| d.id == cohort_id)?;
        let members: Vec<Uuid> = users
            .iter()
            .filter(|u| Self::matches(&def.criterion, u))
            .map(|u| u.user_id)
            .collect();

        Some(CohortResult {
            cohort_id,
            cohort_name: def.name.clone(),
            member_count: members.len(),
            members,
        })
    }

    /// Assign users to all cohorts, returning results for each.
    pub fn assign_all(&self, users: &[UserRecord]) -> Vec<CohortResult> {
        self.definitions
            .iter()
            .filter_map(|def| self.assign(def.id, users))
            .collect()
    }

    /// Compare two cohorts by a metric function.
    pub fn compare<F>(
        &self,
        cohort_a: Uuid,
        cohort_b: Uuid,
        users: &[UserRecord],
        metric_fn: F,
    ) -> Option<CohortComparison>
    where
        F: Fn(&[UserRecord]) -> f64,
    {
        let result_a = self.assign(cohort_a, users)?;
        let result_b = self.assign(cohort_b, users)?;

        let members_a: HashSet<Uuid> = result_a.members.iter().copied().collect();
        let members_b: HashSet<Uuid> = result_b.members.iter().copied().collect();

        let users_a: Vec<UserRecord> = users
            .iter()
            .filter(|u| members_a.contains(&u.user_id))
            .cloned()
            .collect();
        let users_b: Vec<UserRecord> = users
            .iter()
            .filter(|u| members_b.contains(&u.user_id))
            .cloned()
            .collect();

        let metric_a = metric_fn(&users_a);
        let metric_b = metric_fn(&users_b);

        Some(CohortComparison {
            cohort_a_name: result_a.cohort_name,
            cohort_b_name: result_b.cohort_name,
            cohort_a_count: result_a.member_count,
            cohort_b_count: result_b.member_count,
            metric_a,
            metric_b,
            difference: metric_a - metric_b,
        })
    }
}

/// Result of comparing two cohorts on a metric.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortComparison {
    pub cohort_a_name: String,
    pub cohort_b_name: String,
    pub cohort_a_count: usize,
    pub cohort_b_count: usize,
    pub metric_a: f64,
    pub metric_b: f64,
    pub difference: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn sample_users() -> Vec<UserRecord> {
        let t0 = Utc::now();
        vec![
            UserRecord {
                user_id: Uuid::new_v4(),
                first_seen: t0 - Duration::days(5),
                lat: Some(32.05),
                lon: Some(34.78),
                properties: [("device".into(), "ios".into())].into(),
                event_count: 100,
            },
            UserRecord {
                user_id: Uuid::new_v4(),
                first_seen: t0 - Duration::days(30),
                lat: Some(40.71),
                lon: Some(-74.01),
                properties: [("device".into(), "android".into())].into(),
                event_count: 50,
            },
            UserRecord {
                user_id: Uuid::new_v4(),
                first_seen: t0 - Duration::days(2),
                lat: Some(32.08),
                lon: Some(34.80),
                properties: [("device".into(), "ios".into())].into(),
                event_count: 10,
            },
        ]
    }

    #[test]
    fn test_region_cohort() {
        let users = sample_users();
        let cohort = CohortDefinition {
            id: Uuid::new_v4(),
            name: "Tel Aviv".into(),
            criterion: CohortCriterion::Region {
                min_lat: 32.0,
                max_lat: 32.2,
                min_lon: 34.7,
                max_lon: 34.9,
            },
        };
        let cid = cohort.id;
        let engine = CohortEngine::new(vec![cohort]);
        let result = engine.assign(cid, &users).unwrap();
        assert_eq!(result.member_count, 2); // users 0 and 2 are in TLV
    }

    #[test]
    fn test_property_cohort() {
        let users = sample_users();
        let cohort = CohortDefinition {
            id: Uuid::new_v4(),
            name: "iOS Users".into(),
            criterion: CohortCriterion::Property {
                key: "device".into(),
                value: "ios".into(),
            },
        };
        let cid = cohort.id;
        let engine = CohortEngine::new(vec![cohort]);
        let result = engine.assign(cid, &users).unwrap();
        assert_eq!(result.member_count, 2);
    }

    #[test]
    fn test_high_activity_cohort() {
        let users = sample_users();
        let cohort = CohortDefinition {
            id: Uuid::new_v4(),
            name: "Power Users".into(),
            criterion: CohortCriterion::HighActivity { min_events: 50 },
        };
        let cid = cohort.id;
        let engine = CohortEngine::new(vec![cohort]);
        let result = engine.assign(cid, &users).unwrap();
        assert_eq!(result.member_count, 2); // users with 100 and 50 events
    }

    #[test]
    fn test_first_seen_cohort() {
        let users = sample_users();
        let t0 = Utc::now();
        let cohort = CohortDefinition {
            id: Uuid::new_v4(),
            name: "New Users (< 7 days)".into(),
            criterion: CohortCriterion::FirstSeenRange {
                from: t0 - Duration::days(7),
                to: t0,
            },
        };
        let cid = cohort.id;
        let engine = CohortEngine::new(vec![cohort]);
        let result = engine.assign(cid, &users).unwrap();
        assert_eq!(result.member_count, 2); // users 0 and 2
    }

    #[test]
    fn test_compare_cohorts() {
        let users = sample_users();
        let ios = CohortDefinition {
            id: Uuid::new_v4(),
            name: "iOS".into(),
            criterion: CohortCriterion::Property {
                key: "device".into(),
                value: "ios".into(),
            },
        };
        let android = CohortDefinition {
            id: Uuid::new_v4(),
            name: "Android".into(),
            criterion: CohortCriterion::Property {
                key: "device".into(),
                value: "android".into(),
            },
        };
        let ios_id = ios.id;
        let android_id = android.id;
        let engine = CohortEngine::new(vec![ios, android]);

        let comparison = engine
            .compare(ios_id, android_id, &users, |members| {
                if members.is_empty() {
                    return 0.0;
                }
                members.iter().map(|u| u.event_count as f64).sum::<f64>() / members.len() as f64
            })
            .unwrap();

        assert_eq!(comparison.cohort_a_count, 2);
        assert_eq!(comparison.cohort_b_count, 1);
        assert!((comparison.metric_a - 55.0).abs() < f64::EPSILON); // avg(100, 10)
        assert!((comparison.metric_b - 50.0).abs() < f64::EPSILON); // avg(50)
        assert!((comparison.difference - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_assign_all() {
        let users = sample_users();
        let c1 = CohortDefinition {
            id: Uuid::new_v4(),
            name: "A".into(),
            criterion: CohortCriterion::HighActivity { min_events: 100 },
        };
        let c2 = CohortDefinition {
            id: Uuid::new_v4(),
            name: "B".into(),
            criterion: CohortCriterion::HighActivity { min_events: 10 },
        };
        let engine = CohortEngine::new(vec![c1, c2]);
        let results = engine.assign_all(&users);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].member_count, 1);
        assert_eq!(results[1].member_count, 3);
    }

    #[test]
    fn test_no_location_excluded_from_region() {
        let user = UserRecord {
            user_id: Uuid::new_v4(),
            first_seen: Utc::now(),
            lat: None,
            lon: None,
            properties: HashMap::new(),
            event_count: 0,
        };
        let criterion = CohortCriterion::Region {
            min_lat: 0.0,
            max_lat: 90.0,
            min_lon: -180.0,
            max_lon: 180.0,
        };
        assert!(!CohortEngine::matches(&criterion, &user));
    }
}
