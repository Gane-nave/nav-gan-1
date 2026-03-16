//! Voice guidance generator.
//!
//! Converts navigation manoeuvres into natural-language voice
//! instructions with distance callouts and contextual phrasing.

/// Manoeuvre type for voice instruction generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Manoeuvre {
    Continue,
    TurnLeft,
    TurnRight,
    SlightLeft,
    SlightRight,
    SharpLeft,
    SharpRight,
    UTurn,
    MergeLeft,
    MergeRight,
    ExitLeft,
    ExitRight,
    Roundabout { exit_number: u8 },
    Arrive,
    Depart,
    Ferry,
}

/// Distance threshold for advance warnings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalloutTiming {
    /// First warning (~1 km ahead on highways, ~300 m in cities).
    Prepare,
    /// Second warning (~500 m / ~100 m).
    Approach,
    /// Final instruction (~100 m / ~30 m).
    Now,
}

/// A voice guidance instruction.
#[derive(Debug, Clone)]
pub struct GuidanceInstruction {
    pub manoeuvre: Manoeuvre,
    pub timing: CalloutTiming,
    pub distance_m: f64,
    pub road_name: Option<String>,
    pub exit_name: Option<String>,
}

/// Language for instruction generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuidanceLanguage {
    English,
    Hebrew,
    Arabic,
    German,
    French,
    Spanish,
    Japanese,
}

impl GuidanceLanguage {
    fn direction_word(self, manoeuvre: Manoeuvre) -> &'static str {
        match self {
            Self::English => match manoeuvre {
                Manoeuvre::TurnLeft => "turn left",
                Manoeuvre::TurnRight => "turn right",
                Manoeuvre::SlightLeft => "keep left",
                Manoeuvre::SlightRight => "keep right",
                Manoeuvre::SharpLeft => "sharp left",
                Manoeuvre::SharpRight => "sharp right",
                Manoeuvre::UTurn => "make a U-turn",
                Manoeuvre::MergeLeft => "merge left",
                Manoeuvre::MergeRight => "merge right",
                Manoeuvre::ExitLeft => "exit left",
                Manoeuvre::ExitRight => "exit right",
                Manoeuvre::Continue => "continue straight",
                Manoeuvre::Arrive => "you have arrived",
                Manoeuvre::Depart => "depart",
                Manoeuvre::Ferry => "board the ferry",
                Manoeuvre::Roundabout { exit_number } => match exit_number {
                    1 => "take the first exit",
                    2 => "take the second exit",
                    3 => "take the third exit",
                    _ => "exit the roundabout",
                },
            },
            Self::Hebrew => match manoeuvre {
                Manoeuvre::TurnLeft => "פנה שמאלה",
                Manoeuvre::TurnRight => "פנה ימינה",
                Manoeuvre::UTurn => "בצע פניית פרסה",
                Manoeuvre::Continue => "המשך ישר",
                Manoeuvre::Arrive => "הגעת ליעד",
                _ => "המשך",
            },
            _ => match manoeuvre {
                Manoeuvre::TurnLeft => "turn left",
                Manoeuvre::TurnRight => "turn right",
                _ => "continue",
            },
        }
    }
}

/// Voice guidance generator.
#[derive(Debug)]
pub struct GuidanceGenerator {
    language: GuidanceLanguage,
    /// Whether to include road names in instructions.
    include_road_names: bool,
    /// Whether to use metric distances.
    metric: bool,
}

impl GuidanceGenerator {
    pub fn new(language: GuidanceLanguage) -> Self {
        Self {
            language,
            include_road_names: true,
            metric: true,
        }
    }

    pub fn set_include_road_names(&mut self, v: bool) {
        self.include_road_names = v;
    }

    pub fn set_metric(&mut self, v: bool) {
        self.metric = v;
    }

    /// Generate a text instruction from a guidance event.
    pub fn generate(&self, inst: &GuidanceInstruction) -> String {
        let direction = self.language.direction_word(inst.manoeuvre);
        let distance = self.format_distance(inst.distance_m);
        let road = if self.include_road_names {
            inst.road_name.as_deref().map(|r| format!(" onto {r}"))
        } else {
            None
        };
        let road_str = road.as_deref().unwrap_or("");

        match inst.timing {
            CalloutTiming::Prepare => {
                format!("In {distance}, {direction}{road_str}")
            }
            CalloutTiming::Approach => {
                format!("{direction}{road_str} in {distance}")
            }
            CalloutTiming::Now => {
                if inst.manoeuvre == Manoeuvre::Arrive {
                    direction.to_string()
                } else {
                    format!("{direction}{road_str} now")
                }
            }
        }
    }

    /// Format distance as human-readable string.
    pub fn format_distance(&self, metres: f64) -> String {
        if self.metric {
            if metres >= 1000.0 {
                let km = metres / 1000.0;
                if (km - km.round()).abs() < 0.05 {
                    format!("{} km", km.round() as u32)
                } else {
                    format!("{km:.1} km")
                }
            } else {
                let rounded = (metres / 50.0).round() * 50.0;
                format!("{} metres", rounded as u32)
            }
        } else {
            let feet = metres * 3.28084;
            if feet >= 5280.0 {
                let miles = feet / 5280.0;
                format!("{miles:.1} miles")
            } else {
                let rounded = (feet / 100.0).round() * 100.0;
                format!("{} feet", rounded as u32)
            }
        }
    }

    /// Generate a sequence of callouts for a manoeuvre at different distances.
    pub fn callout_sequence(&self, manoeuvre: Manoeuvre, road: Option<&str>) -> Vec<String> {
        let distances = [
            (CalloutTiming::Prepare, 500.0),
            (CalloutTiming::Approach, 200.0),
            (CalloutTiming::Now, 20.0),
        ];
        distances
            .iter()
            .map(|&(timing, dist)| {
                self.generate(&GuidanceInstruction {
                    manoeuvre,
                    timing,
                    distance_m: dist,
                    road_name: road.map(|s| s.to_string()),
                    exit_name: None,
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_instruction_english() {
        let gen = GuidanceGenerator::new(GuidanceLanguage::English);
        let inst = GuidanceInstruction {
            manoeuvre: Manoeuvre::TurnLeft,
            timing: CalloutTiming::Prepare,
            distance_m: 500.0,
            road_name: Some("King Street".into()),
            exit_name: None,
        };
        let text = gen.generate(&inst);
        assert!(text.contains("turn left"));
        assert!(text.contains("King Street"));
        assert!(text.contains("500"));
    }

    #[test]
    fn test_arrive_instruction() {
        let gen = GuidanceGenerator::new(GuidanceLanguage::English);
        let inst = GuidanceInstruction {
            manoeuvre: Manoeuvre::Arrive,
            timing: CalloutTiming::Now,
            distance_m: 0.0,
            road_name: None,
            exit_name: None,
        };
        assert_eq!(gen.generate(&inst), "you have arrived");
    }

    #[test]
    fn test_hebrew_direction() {
        let gen = GuidanceGenerator::new(GuidanceLanguage::Hebrew);
        let inst = GuidanceInstruction {
            manoeuvre: Manoeuvre::TurnRight,
            timing: CalloutTiming::Now,
            distance_m: 0.0,
            road_name: None,
            exit_name: None,
        };
        let text = gen.generate(&inst);
        assert!(text.contains("ימינה"));
    }

    #[test]
    fn test_distance_format_metric() {
        let gen = GuidanceGenerator::new(GuidanceLanguage::English);
        assert_eq!(gen.format_distance(1000.0), "1 km");
        assert_eq!(gen.format_distance(1500.0), "1.5 km");
        assert_eq!(gen.format_distance(250.0), "250 metres");
        assert_eq!(gen.format_distance(120.0), "100 metres"); // rounds to nearest 50
    }

    #[test]
    fn test_distance_format_imperial() {
        let mut gen = GuidanceGenerator::new(GuidanceLanguage::English);
        gen.set_metric(false);
        let text = gen.format_distance(1610.0); // ~1 mile
        assert!(text.contains("mile"));
    }

    #[test]
    fn test_callout_sequence() {
        let gen = GuidanceGenerator::new(GuidanceLanguage::English);
        let seq = gen.callout_sequence(Manoeuvre::TurnRight, Some("Main St"));
        assert_eq!(seq.len(), 3);
        assert!(seq[0].contains("500")); // prepare
        assert!(seq[1].contains("200")); // approach
        assert!(seq[2].contains("now")); // now
    }

    #[test]
    fn test_roundabout_exit() {
        let gen = GuidanceGenerator::new(GuidanceLanguage::English);
        let inst = GuidanceInstruction {
            manoeuvre: Manoeuvre::Roundabout { exit_number: 2 },
            timing: CalloutTiming::Prepare,
            distance_m: 200.0,
            road_name: None,
            exit_name: None,
        };
        let text = gen.generate(&inst);
        assert!(text.contains("second exit"));
    }

    #[test]
    fn test_no_road_name() {
        let mut gen = GuidanceGenerator::new(GuidanceLanguage::English);
        gen.set_include_road_names(false);
        let inst = GuidanceInstruction {
            manoeuvre: Manoeuvre::TurnLeft,
            timing: CalloutTiming::Now,
            distance_m: 0.0,
            road_name: Some("Hidden St".into()),
            exit_name: None,
        };
        let text = gen.generate(&inst);
        assert!(!text.contains("Hidden"));
    }
}
