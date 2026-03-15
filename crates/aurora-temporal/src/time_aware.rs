//! Time-aware routing — adjusts routing parameters based on time of day,
//! day of week, season, and temporal traffic patterns.

use chrono::{DateTime, Datelike, Timelike, Utc};
use serde::{Deserialize, Serialize};

/// Time period classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimePeriod {
    /// Early morning (5:00 - 6:59)
    EarlyMorning,
    /// Morning rush hour (7:00 - 9:29)
    MorningRush,
    /// Mid-morning (9:30 - 11:59)
    MidMorning,
    /// Lunch time (12:00 - 13:29)
    LunchTime,
    /// Afternoon (13:30 - 15:59)
    Afternoon,
    /// Evening rush hour (16:00 - 18:59)
    EveningRush,
    /// Evening (19:00 - 21:59)
    Evening,
    /// Night (22:00 - 4:59)
    Night,
}

impl TimePeriod {
    /// Classify a time into a period.
    pub fn from_hour(hour: u32) -> Self {
        match hour {
            5..=6 => Self::EarlyMorning,
            7..=9 => Self::MorningRush,
            10..=11 => Self::MidMorning,
            12..=13 => Self::LunchTime,
            14..=15 => Self::Afternoon,
            16..=18 => Self::EveningRush,
            19..=21 => Self::Evening,
            _ => Self::Night,
        }
    }

    /// Traffic density multiplier (1.0 = normal baseline).
    pub fn traffic_multiplier(&self) -> f64 {
        match self {
            Self::EarlyMorning => 0.4,
            Self::MorningRush => 1.8,
            Self::MidMorning => 1.0,
            Self::LunchTime => 1.2,
            Self::Afternoon => 1.1,
            Self::EveningRush => 1.9,
            Self::Evening => 0.7,
            Self::Night => 0.3,
        }
    }

    /// Whether headlights are likely needed.
    pub fn needs_headlights(&self) -> bool {
        matches!(self, Self::Night | Self::EarlyMorning | Self::Evening)
    }
}

/// Season classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

impl Season {
    /// Determine season from month (Northern Hemisphere).
    pub fn from_month_north(month: u32) -> Self {
        match month {
            3..=5 => Self::Spring,
            6..=8 => Self::Summer,
            9..=11 => Self::Autumn,
            _ => Self::Winter,
        }
    }

    /// Determine season from month (Southern Hemisphere).
    pub fn from_month_south(month: u32) -> Self {
        match month {
            3..=5 => Self::Autumn,
            6..=8 => Self::Winter,
            9..=11 => Self::Spring,
            _ => Self::Summer,
        }
    }

    /// Daylight hours estimate (approximate for mid-latitudes).
    pub fn approx_daylight_hours(&self) -> f64 {
        match self {
            Self::Spring => 13.0,
            Self::Summer => 15.0,
            Self::Autumn => 11.0,
            Self::Winter => 9.0,
        }
    }

    /// Road condition risk factor (0.0 = low, 1.0 = high).
    pub fn road_risk(&self) -> f64 {
        match self {
            Self::Spring => 0.2,
            Self::Summer => 0.1,
            Self::Autumn => 0.3,
            Self::Winter => 0.6,
        }
    }
}

/// Time-aware routing adjustment parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeAdjustment {
    /// Time period
    pub period: TimePeriod,
    /// Season
    pub season: Season,
    /// Traffic multiplier (combined period + day-of-week)
    pub traffic_factor: f64,
    /// Recommended speed adjustment (1.0 = no change)
    pub speed_factor: f64,
    /// Whether to prefer main roads (rush hour) or side streets
    pub prefer_main_roads: bool,
    /// Whether toll roads are likely faster (rush hour bypass)
    pub toll_advantage: bool,
    /// Whether to suggest departure time change
    pub suggest_delay: bool,
    /// Suggested delay minutes (if suggest_delay is true)
    pub delay_minutes: u32,
}

/// Time-aware routing engine.
#[derive(Debug)]
pub struct TimeAwareEngine {
    /// Whether the location is in the northern hemisphere
    northern_hemisphere: bool,
    /// Local UTC offset in hours
    utc_offset_hours: i32,
}

impl TimeAwareEngine {
    /// Create a new engine for the given location parameters.
    pub fn new(northern_hemisphere: bool, utc_offset_hours: i32) -> Self {
        Self {
            northern_hemisphere,
            utc_offset_hours,
        }
    }

    /// Get the current local hour (0-23).
    pub fn local_hour(&self, utc: DateTime<Utc>) -> u32 {
        let local_hour = utc.hour() as i32 + self.utc_offset_hours;
        ((local_hour % 24 + 24) % 24) as u32
    }

    /// Classify the current time period.
    pub fn current_period(&self, utc: DateTime<Utc>) -> TimePeriod {
        TimePeriod::from_hour(self.local_hour(utc))
    }

    /// Get the current season.
    pub fn current_season(&self, utc: DateTime<Utc>) -> Season {
        let month = utc.month();
        if self.northern_hemisphere {
            Season::from_month_north(month)
        } else {
            Season::from_month_south(month)
        }
    }

    /// Compute routing adjustments for the current time.
    pub fn compute_adjustment(&self, utc: DateTime<Utc>) -> TimeAdjustment {
        let period = self.current_period(utc);
        let season = self.current_season(utc);
        let weekday = utc.weekday();

        // Weekend factor
        let is_weekend = matches!(weekday, chrono::Weekday::Sat | chrono::Weekday::Sun);
        let day_factor = if is_weekend { 0.6 } else { 1.0 };

        let traffic_factor = period.traffic_multiplier() * day_factor;
        let speed_factor = if traffic_factor > 1.5 { 0.8 } else { 1.0 };

        let prefer_main = matches!(period, TimePeriod::Night | TimePeriod::EarlyMorning);
        let toll_advantage =
            matches!(period, TimePeriod::MorningRush | TimePeriod::EveningRush) && !is_weekend;

        // Suggest delay if currently in peak rush
        let (suggest_delay, delay_minutes) = if traffic_factor > 1.7 && !is_weekend {
            (true, 30)
        } else {
            (false, 0)
        };

        TimeAdjustment {
            period,
            season,
            traffic_factor,
            speed_factor,
            prefer_main_roads: prefer_main,
            toll_advantage,
            suggest_delay,
            delay_minutes,
        }
    }

    /// Estimate arrival time given departure time and base travel time.
    pub fn estimated_arrival(
        &self,
        departure: DateTime<Utc>,
        base_travel_seconds: u64,
    ) -> DateTime<Utc> {
        let adj = self.compute_adjustment(departure);
        let adjusted_seconds = (base_travel_seconds as f64 * adj.traffic_factor) as i64;
        departure + chrono::Duration::seconds(adjusted_seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_time_period_classification() {
        assert_eq!(TimePeriod::from_hour(8), TimePeriod::MorningRush);
        assert_eq!(TimePeriod::from_hour(17), TimePeriod::EveningRush);
        assert_eq!(TimePeriod::from_hour(2), TimePeriod::Night);
        assert_eq!(TimePeriod::from_hour(14), TimePeriod::Afternoon);
    }

    #[test]
    fn test_time_period_traffic_multiplier() {
        assert!(TimePeriod::MorningRush.traffic_multiplier() > 1.5);
        assert!(TimePeriod::EveningRush.traffic_multiplier() > 1.5);
        assert!(TimePeriod::Night.traffic_multiplier() < 0.5);
    }

    #[test]
    fn test_season_north() {
        assert_eq!(Season::from_month_north(1), Season::Winter);
        assert_eq!(Season::from_month_north(4), Season::Spring);
        assert_eq!(Season::from_month_north(7), Season::Summer);
        assert_eq!(Season::from_month_north(10), Season::Autumn);
    }

    #[test]
    fn test_season_south() {
        assert_eq!(Season::from_month_south(1), Season::Summer);
        assert_eq!(Season::from_month_south(7), Season::Winter);
    }

    #[test]
    fn test_time_aware_engine() {
        let engine = TimeAwareEngine::new(true, 2); // Israel UTC+2
        let morning_rush = Utc.with_ymd_and_hms(2026, 3, 15, 6, 0, 0).unwrap(); // 8am local
        let period = engine.current_period(morning_rush);
        assert_eq!(period, TimePeriod::MorningRush);
    }

    #[test]
    fn test_time_aware_adjustment() {
        let engine = TimeAwareEngine::new(true, 2);
        let rush_hour = Utc.with_ymd_and_hms(2026, 3, 16, 15, 0, 0).unwrap(); // Monday 5pm local
        let adj = engine.compute_adjustment(rush_hour);
        assert!(adj.traffic_factor > 1.0);
        assert_eq!(adj.period, TimePeriod::EveningRush);
    }

    #[test]
    fn test_weekend_factor() {
        let engine = TimeAwareEngine::new(true, 0);
        // Saturday at rush hour
        let sat_rush = Utc.with_ymd_and_hms(2026, 3, 14, 8, 0, 0).unwrap(); // Saturday
        let adj = engine.compute_adjustment(sat_rush);
        // Weekend should reduce traffic factor
        assert!(adj.traffic_factor < TimePeriod::MorningRush.traffic_multiplier());
    }

    #[test]
    fn test_estimated_arrival() {
        let engine = TimeAwareEngine::new(true, 0);
        let departure = Utc.with_ymd_and_hms(2026, 3, 15, 2, 0, 0).unwrap(); // Night
        let arrival = engine.estimated_arrival(departure, 3600); // 1 hour base
                                                                 // Night traffic is low, so should be less than 1 hour adjusted
        let diff = arrival.signed_duration_since(departure).num_seconds();
        assert!(diff < 3600); // Should be faster at night
    }

    #[test]
    fn test_headlights_needed() {
        assert!(TimePeriod::Night.needs_headlights());
        assert!(TimePeriod::EarlyMorning.needs_headlights());
        assert!(!TimePeriod::MidMorning.needs_headlights());
    }

    #[test]
    fn test_local_hour_wrap() {
        let engine = TimeAwareEngine::new(true, -5); // EST
        let utc_3am = Utc.with_ymd_and_hms(2026, 3, 15, 3, 0, 0).unwrap();
        let local = engine.local_hour(utc_3am);
        assert_eq!(local, 22); // 3am UTC - 5 = 10pm previous day
    }
}
