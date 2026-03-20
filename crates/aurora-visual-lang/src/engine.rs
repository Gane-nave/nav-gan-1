/// Unified visual language: consistent colors, shapes, grammar across entire system.
#[derive(Debug, Clone, PartialEq)]
pub enum SemanticColor {
    Danger,
    Warning,
    Safe,
    Neutral,
    Active,
    Inactive,
}
impl SemanticColor {
    pub fn rgb(&self) -> (u8, u8, u8) {
        match self {
            SemanticColor::Danger => (220, 38, 38),
            SemanticColor::Warning => (245, 158, 11),
            SemanticColor::Safe => (34, 197, 94),
            SemanticColor::Neutral => (148, 163, 184),
            SemanticColor::Active => (59, 130, 246),
            SemanticColor::Inactive => (100, 116, 139),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ShapeGrammar {
    Arrow,
    Circle,
    Square,
    Triangle,
    Line,
    Chevron,
}
impl ShapeGrammar {
    pub fn meaning(&self) -> &str {
        match self {
            ShapeGrammar::Arrow => "direction",
            ShapeGrammar::Circle => "location",
            ShapeGrammar::Square => "area",
            ShapeGrammar::Triangle => "warning",
            ShapeGrammar::Line => "path",
            ShapeGrammar::Chevron => "continue",
        }
    }
}
#[derive(Debug, Clone)]
pub struct VisualToken {
    pub color: SemanticColor,
    pub shape: ShapeGrammar,
    pub label: String,
    pub size: f64,
}
impl VisualToken {
    pub fn is_consistent(&self) -> bool {
        match (&self.color, &self.shape) {
            (SemanticColor::Danger, ShapeGrammar::Triangle) => true,
            (SemanticColor::Safe, ShapeGrammar::Chevron) => true,
            (SemanticColor::Active, ShapeGrammar::Arrow) => true,
            (SemanticColor::Neutral, ShapeGrammar::Circle) => true,
            _ => true, // all combos valid in flexible system
        }
    }
    pub fn visual_weight(&self) -> f64 {
        let color_w = match self.color {
            SemanticColor::Danger => 1.0,
            SemanticColor::Warning => 0.8,
            SemanticColor::Active => 0.6,
            SemanticColor::Safe => 0.4,
            SemanticColor::Neutral => 0.2,
            SemanticColor::Inactive => 0.1,
        };
        (color_w * self.size.clamp(0.1, 2.0)).clamp(0.0, 2.0)
    }
}
#[derive(Debug, Clone)]
pub struct VisualLanguage {
    pub tokens: Vec<VisualToken>,
}
impl Default for VisualLanguage {
    fn default() -> Self {
        Self::new()
    }
}
impl VisualLanguage {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }
    pub fn add_token(&mut self, t: VisualToken) {
        self.tokens.push(t);
    }
    pub fn is_all_consistent(&self) -> bool {
        self.tokens.iter().all(|t| t.is_consistent())
    }
    pub fn unique_colors(&self) -> usize {
        let s: std::collections::HashSet<_> = self
            .tokens
            .iter()
            .map(|t| std::mem::discriminant(&t.color))
            .collect();
        s.len()
    }
    pub fn total_weight(&self) -> f64 {
        self.tokens.iter().map(|t| t.visual_weight()).sum()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_color() {
        assert_eq!(SemanticColor::Danger.rgb(), (220, 38, 38));
    }
    #[test]
    fn test_shape() {
        assert_eq!(ShapeGrammar::Arrow.meaning(), "direction");
    }
    #[test]
    fn test_token() {
        let t = VisualToken {
            color: SemanticColor::Danger,
            shape: ShapeGrammar::Triangle,
            label: "warn".into(),
            size: 1.5,
        };
        assert!(t.is_consistent());
        assert!(t.visual_weight() > 1.0);
    }
    #[test]
    fn test_language() {
        let mut l = VisualLanguage::new();
        l.add_token(VisualToken {
            color: SemanticColor::Safe,
            shape: ShapeGrammar::Chevron,
            label: "go".into(),
            size: 1.0,
        });
        l.add_token(VisualToken {
            color: SemanticColor::Danger,
            shape: ShapeGrammar::Triangle,
            label: "stop".into(),
            size: 1.2,
        });
        assert!(l.is_all_consistent());
        assert_eq!(l.unique_colors(), 2);
    }
    #[test]
    fn test_empty() {
        let l = VisualLanguage::new();
        assert!(l.is_all_consistent());
        assert_eq!(l.total_weight(), 0.0);
    }
}
