//! Cron-like scheduling — time-based job triggers with flexible patterns.

/// Day of week.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl DayOfWeek {
    /// Convert from numeric (0 = Monday .. 6 = Sunday).
    pub fn from_num(n: u8) -> Option<Self> {
        match n {
            0 => Some(Self::Monday),
            1 => Some(Self::Tuesday),
            2 => Some(Self::Wednesday),
            3 => Some(Self::Thursday),
            4 => Some(Self::Friday),
            5 => Some(Self::Saturday),
            6 => Some(Self::Sunday),
            _ => None,
        }
    }

    /// Convert to numeric.
    pub fn to_num(self) -> u8 {
        match self {
            Self::Monday => 0,
            Self::Tuesday => 1,
            Self::Wednesday => 2,
            Self::Thursday => 3,
            Self::Friday => 4,
            Self::Saturday => 5,
            Self::Sunday => 6,
        }
    }

    /// Check if this is a weekday.
    pub fn is_weekday(self) -> bool {
        !matches!(self, Self::Saturday | Self::Sunday)
    }
}

/// Schedule pattern — when to trigger a job.
#[derive(Debug, Clone)]
pub enum SchedulePattern {
    /// Run every N seconds.
    Interval { seconds: u64 },
    /// Run at specific times of day (hour, minute).
    DailyAt { hour: u8, minute: u8 },
    /// Run on specific days at a given time.
    Weekly {
        days: Vec<DayOfWeek>,
        hour: u8,
        minute: u8,
    },
    /// Run once at a specific timestamp.
    Once { timestamp_secs: u64 },
}

/// A scheduled trigger.
#[derive(Debug, Clone)]
pub struct CronTrigger {
    /// Trigger identifier.
    pub id: String,
    /// Job ID to trigger.
    pub job_id: String,
    /// Schedule pattern.
    pub pattern: SchedulePattern,
    /// Whether this trigger is active.
    pub active: bool,
    /// Last trigger time (epoch seconds).
    pub last_triggered: Option<u64>,
    /// Total number of times triggered.
    pub trigger_count: u64,
}

impl CronTrigger {
    /// Create a new trigger.
    pub fn new(id: &str, job_id: &str, pattern: SchedulePattern) -> Self {
        Self {
            id: id.to_string(),
            job_id: job_id.to_string(),
            pattern,
            active: true,
            last_triggered: None,
            trigger_count: 0,
        }
    }

    /// Check if the trigger should fire at the given time.
    pub fn should_trigger(&self, current_secs: u64) -> bool {
        if !self.active {
            return false;
        }
        match &self.pattern {
            SchedulePattern::Interval { seconds } => {
                if *seconds == 0 {
                    return false;
                }
                match self.last_triggered {
                    None => true,
                    Some(last) => current_secs >= last + seconds,
                }
            }
            SchedulePattern::Once { timestamp_secs } => {
                self.last_triggered.is_none() && current_secs >= *timestamp_secs
            }
            SchedulePattern::DailyAt { hour, minute } => {
                let secs_in_day = current_secs % 86400;
                let target_secs = *hour as u64 * 3600 + *minute as u64 * 60;
                let in_window = secs_in_day >= target_secs && secs_in_day < target_secs + 60;
                if !in_window {
                    return false;
                }
                match self.last_triggered {
                    None => true,
                    Some(last) => current_secs - last >= 86400, // at least 1 day gap
                }
            }
            SchedulePattern::Weekly { days, hour, minute } => {
                // Simplified: check if current day-of-week matches
                // In real code we'd need proper calendar; here we use epoch modular arithmetic
                let day_num = ((current_secs / 86400) + 3) % 7; // epoch was Thursday
                let current_day = match DayOfWeek::from_num(day_num as u8) {
                    Some(d) => d,
                    None => return false,
                };
                if !days.contains(&current_day) {
                    return false;
                }
                let secs_in_day = current_secs % 86400;
                let target_secs = *hour as u64 * 3600 + *minute as u64 * 60;
                let in_window = secs_in_day >= target_secs && secs_in_day < target_secs + 60;
                if !in_window {
                    return false;
                }
                match self.last_triggered {
                    None => true,
                    Some(last) => current_secs - last >= 86400,
                }
            }
        }
    }

    /// Record that the trigger fired.
    pub fn record_trigger(&mut self, timestamp_secs: u64) {
        self.last_triggered = Some(timestamp_secs);
        self.trigger_count += 1;
        // Once triggers auto-deactivate
        if matches!(self.pattern, SchedulePattern::Once { .. }) {
            self.active = false;
        }
    }
}

/// Cron scheduler — manages multiple triggers.
pub struct CronScheduler {
    triggers: Vec<CronTrigger>,
}

impl CronScheduler {
    /// Create a new scheduler.
    pub fn new() -> Self {
        Self {
            triggers: Vec::new(),
        }
    }

    /// Add a trigger.
    pub fn add_trigger(&mut self, trigger: CronTrigger) -> Result<(), String> {
        if self.triggers.iter().any(|t| t.id == trigger.id) {
            return Err(format!("Trigger '{}' already exists", trigger.id));
        }
        self.triggers.push(trigger);
        Ok(())
    }

    /// Remove a trigger.
    pub fn remove_trigger(&mut self, id: &str) -> Result<(), String> {
        let before = self.triggers.len();
        self.triggers.retain(|t| t.id != id);
        if self.triggers.len() == before {
            return Err(format!("Trigger '{id}' not found"));
        }
        Ok(())
    }

    /// Get all triggers that should fire at the given time.
    pub fn pending_triggers(&self, current_secs: u64) -> Vec<&CronTrigger> {
        self.triggers
            .iter()
            .filter(|t| t.should_trigger(current_secs))
            .collect()
    }

    /// Fire all pending triggers and return job IDs.
    pub fn tick(&mut self, current_secs: u64) -> Vec<String> {
        let mut job_ids = Vec::new();
        for trigger in &mut self.triggers {
            if trigger.should_trigger(current_secs) {
                job_ids.push(trigger.job_id.clone());
                trigger.record_trigger(current_secs);
            }
        }
        job_ids
    }

    /// Get trigger count.
    pub fn trigger_count(&self) -> usize {
        self.triggers.len()
    }

    /// Get active trigger count.
    pub fn active_count(&self) -> usize {
        self.triggers.iter().filter(|t| t.active).count()
    }

    /// Get a trigger by ID.
    pub fn get_trigger(&self, id: &str) -> Option<&CronTrigger> {
        self.triggers.iter().find(|t| t.id == id)
    }
}

impl Default for CronScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_trigger() {
        let mut trigger = CronTrigger::new("t1", "j1", SchedulePattern::Interval { seconds: 60 });
        assert!(trigger.should_trigger(100)); // never triggered before
        trigger.record_trigger(100);
        assert!(!trigger.should_trigger(150)); // only 50s passed
        assert!(trigger.should_trigger(160)); // 60s passed
    }

    #[test]
    fn test_zero_interval_never_triggers() {
        let trigger = CronTrigger::new("t1", "j1", SchedulePattern::Interval { seconds: 0 });
        assert!(!trigger.should_trigger(100));
    }

    #[test]
    fn test_once_trigger() {
        let mut trigger = CronTrigger::new(
            "t1",
            "j1",
            SchedulePattern::Once {
                timestamp_secs: 1000,
            },
        );
        assert!(!trigger.should_trigger(500)); // too early
        assert!(trigger.should_trigger(1000)); // exact time
        trigger.record_trigger(1000);
        assert!(!trigger.active); // auto-deactivated
        assert!(!trigger.should_trigger(1500)); // won't fire again
    }

    #[test]
    fn test_inactive_trigger() {
        let mut trigger = CronTrigger::new("t1", "j1", SchedulePattern::Interval { seconds: 10 });
        trigger.active = false;
        assert!(!trigger.should_trigger(100)); // inactive
    }

    #[test]
    fn test_cron_scheduler_tick() {
        let mut sched = CronScheduler::new();
        sched
            .add_trigger(CronTrigger::new(
                "t1",
                "j1",
                SchedulePattern::Interval { seconds: 10 },
            ))
            .unwrap();
        sched
            .add_trigger(CronTrigger::new(
                "t2",
                "j2",
                SchedulePattern::Interval { seconds: 20 },
            ))
            .unwrap();

        let jobs = sched.tick(100);
        assert_eq!(jobs.len(), 2); // both trigger first time

        let jobs = sched.tick(110);
        assert_eq!(jobs.len(), 1); // only t1 (10s interval)
        assert_eq!(jobs[0], "j1");

        let jobs = sched.tick(120);
        assert_eq!(jobs.len(), 2); // both again
    }

    #[test]
    fn test_duplicate_trigger_rejected() {
        let mut sched = CronScheduler::new();
        sched
            .add_trigger(CronTrigger::new(
                "t1",
                "j1",
                SchedulePattern::Interval { seconds: 10 },
            ))
            .unwrap();
        let err = sched
            .add_trigger(CronTrigger::new(
                "t1",
                "j2",
                SchedulePattern::Interval { seconds: 20 },
            ))
            .unwrap_err();
        assert!(err.contains("already exists"));
    }

    #[test]
    fn test_remove_trigger() {
        let mut sched = CronScheduler::new();
        sched
            .add_trigger(CronTrigger::new(
                "t1",
                "j1",
                SchedulePattern::Interval { seconds: 10 },
            ))
            .unwrap();
        sched.remove_trigger("t1").unwrap();
        assert_eq!(sched.trigger_count(), 0);
    }

    #[test]
    fn test_trigger_count() {
        let mut trigger = CronTrigger::new("t1", "j1", SchedulePattern::Interval { seconds: 10 });
        assert_eq!(trigger.trigger_count, 0);
        trigger.record_trigger(100);
        assert_eq!(trigger.trigger_count, 1);
        trigger.record_trigger(110);
        assert_eq!(trigger.trigger_count, 2);
    }

    #[test]
    fn test_day_of_week() {
        assert!(DayOfWeek::Monday.is_weekday());
        assert!(DayOfWeek::Friday.is_weekday());
        assert!(!DayOfWeek::Saturday.is_weekday());
        assert!(!DayOfWeek::Sunday.is_weekday());
        assert_eq!(DayOfWeek::from_num(0), Some(DayOfWeek::Monday));
        assert_eq!(DayOfWeek::from_num(6), Some(DayOfWeek::Sunday));
        assert_eq!(DayOfWeek::from_num(7), None);
        assert_eq!(DayOfWeek::Monday.to_num(), 0);
    }

    #[test]
    fn test_active_count() {
        let mut sched = CronScheduler::new();
        sched
            .add_trigger(CronTrigger::new(
                "t1",
                "j1",
                SchedulePattern::Interval { seconds: 10 },
            ))
            .unwrap();
        let mut t2 = CronTrigger::new("t2", "j2", SchedulePattern::Interval { seconds: 20 });
        t2.active = false;
        sched.add_trigger(t2).unwrap();
        assert_eq!(sched.trigger_count(), 2);
        assert_eq!(sched.active_count(), 1);
    }

    #[test]
    fn test_get_trigger() {
        let mut sched = CronScheduler::new();
        sched
            .add_trigger(CronTrigger::new(
                "t1",
                "j1",
                SchedulePattern::Interval { seconds: 10 },
            ))
            .unwrap();
        assert!(sched.get_trigger("t1").is_some());
        assert!(sched.get_trigger("missing").is_none());
    }
}
