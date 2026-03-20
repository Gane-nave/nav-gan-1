/// Road surface quality engine: roughness index, pothole detection, maintenance scoring.
#[derive(Debug, Clone, PartialEq)]
pub enum SurfaceCondition {
    Excellent,
    Good,
    Fair,
    Poor,
    VeryPoor,
    Impassable,
}

impl SurfaceCondition {
    pub fn from_iri(iri: f64) -> Self {
        match iri {
            i if i < 2.0 => SurfaceCondition::Excellent,
            i if i < 4.0 => SurfaceCondition::Good,
            i if i < 6.0 => SurfaceCondition::Fair,
            i if i < 8.0 => SurfaceCondition::Poor,
            i if i < 12.0 => SurfaceCondition::VeryPoor,
            _ => SurfaceCondition::Impassable,
        }
    }

    pub fn comfort_score(&self) -> f64 {
        match self {
            SurfaceCondition::Excellent => 1.0,
            SurfaceCondition::Good => 0.8,
            SurfaceCondition::Fair => 0.6,
            SurfaceCondition::Poor => 0.35,
            SurfaceCondition::VeryPoor => 0.15,
            SurfaceCondition::Impassable => 0.0,
        }
    }

    pub fn speed_reduction_pct(&self) -> f64 {
        match self {
            SurfaceCondition::Excellent => 0.0,
            SurfaceCondition::Good => 5.0,
            SurfaceCondition::Fair => 15.0,
            SurfaceCondition::Poor => 30.0,
            SurfaceCondition::VeryPoor => 50.0,
            SurfaceCondition::Impassable => 100.0,
        }
    }

    pub fn maintenance_urgency(&self) -> f64 {
        1.0 - self.comfort_score()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DefectType {
    Pothole,
    Crack,
    Rutting,
    Raveling,
    Bleeding,
    Bump,
    Depression,
}

impl DefectType {
    pub fn severity_weight(&self) -> f64 {
        match self {
            DefectType::Pothole => 1.0,
            DefectType::Crack => 0.3,
            DefectType::Rutting => 0.6,
            DefectType::Raveling => 0.4,
            DefectType::Bleeding => 0.2,
            DefectType::Bump => 0.7,
            DefectType::Depression => 0.5,
        }
    }

    pub fn is_hazardous(&self) -> bool {
        matches!(self, DefectType::Pothole | DefectType::Bump)
    }
}

#[derive(Debug, Clone)]
pub struct RoadDefect {
    pub defect_type: DefectType,
    pub position_m: f64,
    pub width_cm: f64,
    pub depth_cm: f64,
    pub confidence: f64,
}

impl RoadDefect {
    pub fn new(defect_type: DefectType, position_m: f64) -> Self {
        Self {
            defect_type,
            position_m,
            width_cm: 20.0,
            depth_cm: 5.0,
            confidence: 0.9,
        }
    }

    pub fn severity(&self) -> f64 {
        let size = (self.width_cm * self.depth_cm / 100.0).min(10.0) / 10.0;
        let weight = self.defect_type.severity_weight();
        (size * weight * self.confidence).clamp(0.0, 1.0)
    }

    pub fn requires_avoidance(&self) -> bool {
        self.severity() > 0.5 || (self.defect_type.is_hazardous() && self.depth_cm > 8.0)
    }
}

#[derive(Debug, Clone)]
pub struct RoadSection {
    pub start_m: f64,
    pub end_m: f64,
    pub iri: f64,
    pub defects: Vec<RoadDefect>,
    pub last_maintained_days: u32,
}

impl RoadSection {
    pub fn new(start_m: f64, end_m: f64, iri: f64) -> Self {
        Self {
            start_m,
            end_m,
            iri,
            defects: Vec::new(),
            last_maintained_days: 0,
        }
    }

    pub fn length(&self) -> f64 {
        (self.end_m - self.start_m).max(0.0)
    }

    pub fn condition(&self) -> SurfaceCondition {
        SurfaceCondition::from_iri(self.iri)
    }

    pub fn defect_density(&self) -> f64 {
        let len_km = self.length() / 1000.0;
        if len_km < f64::EPSILON {
            return 0.0;
        }
        self.defects.len() as f64 / len_km
    }

    pub fn hazard_count(&self) -> usize {
        self.defects
            .iter()
            .filter(|d| d.requires_avoidance())
            .count()
    }

    pub fn quality_score(&self) -> f64 {
        let condition = self.condition().comfort_score();
        let defect_penalty = (self.defect_density() / 50.0).min(0.5);
        let age_penalty = (self.last_maintained_days as f64 / 3650.0).min(0.3);
        (condition - defect_penalty - age_penalty).clamp(0.0, 1.0)
    }

    pub fn recommended_speed_kmh(&self, base_speed: f64) -> f64 {
        let reduction = self.condition().speed_reduction_pct() / 100.0;
        let hazard_reduction = (self.hazard_count() as f64 * 5.0 / base_speed).min(0.3);
        (base_speed * (1.0 - reduction - hazard_reduction)).max(5.0)
    }
}

#[derive(Debug, Clone)]
pub struct RoadSurvey {
    pub sections: Vec<RoadSection>,
}

impl Default for RoadSurvey {
    fn default() -> Self {
        Self::new()
    }
}

impl RoadSurvey {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
        }
    }

    pub fn add_section(&mut self, section: RoadSection) {
        self.sections.push(section);
    }

    pub fn total_length(&self) -> f64 {
        self.sections.iter().map(|s| s.length()).sum()
    }

    pub fn average_iri(&self) -> f64 {
        if self.sections.is_empty() {
            return 0.0;
        }
        let total_len = self.total_length();
        if total_len < f64::EPSILON {
            return 0.0;
        }
        let weighted: f64 = self.sections.iter().map(|s| s.iri * s.length()).sum();
        weighted / total_len
    }

    pub fn overall_condition(&self) -> SurfaceCondition {
        SurfaceCondition::from_iri(self.average_iri())
    }

    pub fn total_defects(&self) -> usize {
        self.sections.iter().map(|s| s.defects.len()).sum()
    }

    pub fn total_hazards(&self) -> usize {
        self.sections.iter().map(|s| s.hazard_count()).sum()
    }

    pub fn worst_section(&self) -> Option<&RoadSection> {
        self.sections.iter().max_by(|a, b| {
            a.iri
                .partial_cmp(&b.iri)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    pub fn needs_maintenance_pct(&self) -> f64 {
        let total = self.total_length();
        if total < f64::EPSILON {
            return 0.0;
        }
        let poor: f64 = self
            .sections
            .iter()
            .filter(|s| s.condition().maintenance_urgency() > 0.5)
            .map(|s| s.length())
            .sum();
        (poor / total) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition_from_iri_excellent() {
        assert_eq!(SurfaceCondition::from_iri(1.5), SurfaceCondition::Excellent);
    }

    #[test]
    fn test_condition_from_iri_poor() {
        assert_eq!(SurfaceCondition::from_iri(7.0), SurfaceCondition::Poor);
    }

    #[test]
    fn test_condition_from_iri_impassable() {
        assert_eq!(
            SurfaceCondition::from_iri(15.0),
            SurfaceCondition::Impassable
        );
    }

    #[test]
    fn test_comfort_score() {
        assert!(
            SurfaceCondition::Excellent.comfort_score() > SurfaceCondition::Poor.comfort_score()
        );
    }

    #[test]
    fn test_speed_reduction() {
        assert!(
            SurfaceCondition::Impassable.speed_reduction_pct()
                > SurfaceCondition::Good.speed_reduction_pct()
        );
    }

    #[test]
    fn test_maintenance_urgency() {
        assert!(
            SurfaceCondition::VeryPoor.maintenance_urgency()
                > SurfaceCondition::Good.maintenance_urgency()
        );
    }

    #[test]
    fn test_defect_severity() {
        let d = RoadDefect::new(DefectType::Pothole, 100.0);
        assert!(d.severity() > 0.0);
    }

    #[test]
    fn test_pothole_hazardous() {
        assert!(DefectType::Pothole.is_hazardous());
        assert!(!DefectType::Crack.is_hazardous());
    }

    #[test]
    fn test_defect_avoidance() {
        let mut d = RoadDefect::new(DefectType::Pothole, 100.0);
        d.width_cm = 50.0;
        d.depth_cm = 15.0;
        assert!(d.requires_avoidance());
    }

    #[test]
    fn test_section_length() {
        let s = RoadSection::new(0.0, 1000.0, 3.0);
        assert!((s.length() - 1000.0).abs() < 0.01);
    }

    #[test]
    fn test_section_condition() {
        let s = RoadSection::new(0.0, 1000.0, 3.0);
        assert_eq!(s.condition(), SurfaceCondition::Good);
    }

    #[test]
    fn test_defect_density() {
        let mut s = RoadSection::new(0.0, 1000.0, 5.0);
        s.defects.push(RoadDefect::new(DefectType::Crack, 100.0));
        s.defects.push(RoadDefect::new(DefectType::Crack, 500.0));
        assert!((s.defect_density() - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_quality_score() {
        let good = RoadSection::new(0.0, 1000.0, 2.0);
        let poor = RoadSection::new(0.0, 1000.0, 8.0);
        assert!(good.quality_score() > poor.quality_score());
    }

    #[test]
    fn test_recommended_speed() {
        let good = RoadSection::new(0.0, 1000.0, 2.0);
        let poor = RoadSection::new(0.0, 1000.0, 8.0);
        assert!(good.recommended_speed_kmh(100.0) > poor.recommended_speed_kmh(100.0));
    }

    #[test]
    fn test_survey_total_length() {
        let mut survey = RoadSurvey::new();
        survey.add_section(RoadSection::new(0.0, 1000.0, 3.0));
        survey.add_section(RoadSection::new(1000.0, 3000.0, 5.0));
        assert!((survey.total_length() - 3000.0).abs() < 0.01);
    }

    #[test]
    fn test_survey_average_iri() {
        let mut survey = RoadSurvey::new();
        survey.add_section(RoadSection::new(0.0, 1000.0, 2.0));
        survey.add_section(RoadSection::new(1000.0, 2000.0, 6.0));
        let avg = survey.average_iri();
        assert!((avg - 4.0).abs() < 0.01);
    }

    #[test]
    fn test_empty_survey() {
        let survey = RoadSurvey::new();
        assert_eq!(survey.total_length(), 0.0);
        assert_eq!(survey.average_iri(), 0.0);
        assert_eq!(survey.total_defects(), 0);
    }

    #[test]
    fn test_worst_section() {
        let mut survey = RoadSurvey::new();
        survey.add_section(RoadSection::new(0.0, 1000.0, 2.0));
        survey.add_section(RoadSection::new(1000.0, 2000.0, 9.0));
        let worst = survey.worst_section().unwrap();
        assert!((worst.iri - 9.0).abs() < 0.01);
    }

    #[test]
    fn test_needs_maintenance() {
        let mut survey = RoadSurvey::new();
        survey.add_section(RoadSection::new(0.0, 500.0, 2.0));
        survey.add_section(RoadSection::new(500.0, 1000.0, 10.0));
        assert!(survey.needs_maintenance_pct() > 40.0);
    }

    #[test]
    fn test_total_hazards() {
        let mut section = RoadSection::new(0.0, 1000.0, 6.0);
        let mut big_pothole = RoadDefect::new(DefectType::Pothole, 300.0);
        big_pothole.width_cm = 40.0;
        big_pothole.depth_cm = 12.0;
        section.defects.push(big_pothole);
        section
            .defects
            .push(RoadDefect::new(DefectType::Crack, 600.0));
        let mut survey = RoadSurvey::new();
        survey.add_section(section);
        assert!(survey.total_hazards() >= 1);
    }
}
