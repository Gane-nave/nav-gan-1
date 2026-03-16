//! Contrast analysis — WCAG color contrast ratio calculation and validation.

use serde::{Deserialize, Serialize};

/// An sRGB color.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Create a color from RGB values.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Create a color from a hex string (e.g., "#FF0000" or "FF0000").
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Self { r, g, b })
    }

    /// Convert to hex string.
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// Calculate relative luminance per WCAG 2.0 formula.
    pub fn relative_luminance(&self) -> f64 {
        let r = srgb_to_linear(self.r as f64 / 255.0);
        let g = srgb_to_linear(self.g as f64 / 255.0);
        let b = srgb_to_linear(self.b as f64 / 255.0);
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }
}

/// Convert sRGB component to linear.
fn srgb_to_linear(c: f64) -> f64 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Calculate the contrast ratio between two colors per WCAG 2.0.
/// Returns a value between 1.0 (no contrast) and 21.0 (max contrast).
pub fn contrast_ratio(fg: &Color, bg: &Color) -> f64 {
    let l1 = fg.relative_luminance();
    let l2 = bg.relative_luminance();
    let lighter = l1.max(l2);
    let darker = l1.min(l2);
    (lighter + 0.05) / (darker + 0.05)
}

/// WCAG conformance level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WcagLevel {
    /// Does not pass any level.
    Fail,
    /// Passes AA for large text (ratio >= 3:1).
    AaLarge,
    /// Passes AA for normal text (ratio >= 4.5:1).
    Aa,
    /// Passes AAA for large text (ratio >= 4.5:1).
    AaaLarge,
    /// Passes AAA for normal text (ratio >= 7:1).
    Aaa,
}

/// Check WCAG conformance level for a contrast ratio.
pub fn wcag_level(ratio: f64) -> WcagLevel {
    if ratio >= 7.0 {
        WcagLevel::Aaa
    } else if ratio >= 4.5 {
        WcagLevel::Aa
    } else if ratio >= 3.0 {
        WcagLevel::AaLarge
    } else {
        WcagLevel::Fail
    }
}

/// A color pair with contrast analysis results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContrastResult {
    pub foreground: Color,
    pub background: Color,
    pub ratio: f64,
    pub level: WcagLevel,
    pub passes_aa_normal: bool,
    pub passes_aa_large: bool,
    pub passes_aaa_normal: bool,
    pub passes_aaa_large: bool,
}

/// Analyze contrast between two colors.
pub fn analyze_contrast(fg: &Color, bg: &Color) -> ContrastResult {
    let ratio = contrast_ratio(fg, bg);
    let level = wcag_level(ratio);
    ContrastResult {
        foreground: *fg,
        background: *bg,
        ratio,
        level,
        passes_aa_normal: ratio >= 4.5,
        passes_aa_large: ratio >= 3.0,
        passes_aaa_normal: ratio >= 7.0,
        passes_aaa_large: ratio >= 4.5,
    }
}

/// Suggest a lighter or darker version of a color to meet a target contrast ratio.
pub fn suggest_color_for_contrast(base: &Color, bg: &Color, target_ratio: f64) -> Option<Color> {
    let bg_lum = bg.relative_luminance();

    // Try darkening or lightening the base color
    for step in 0..=255 {
        // Try darker
        let darker = Color::new(
            base.r.saturating_sub(step),
            base.g.saturating_sub(step),
            base.b.saturating_sub(step),
        );
        if contrast_ratio(&darker, bg) >= target_ratio {
            return Some(darker);
        }

        // Try lighter
        let lighter = Color::new(
            base.r.saturating_add(step),
            base.g.saturating_add(step),
            base.b.saturating_add(step),
        );
        if contrast_ratio(&lighter, bg) >= target_ratio {
            return Some(lighter);
        }
    }

    // Fallback: use black or white
    let black = Color::new(0, 0, 0);
    let white = Color::new(255, 255, 255);
    if bg_lum > 0.5 {
        Some(black)
    } else {
        Some(white)
    }
}

/// Color palette accessibility checker — validates a set of color pairs.
pub struct PaletteChecker {
    pairs: Vec<(String, Color, Color)>,
}

impl PaletteChecker {
    /// Create a new palette checker.
    pub fn new() -> Self {
        Self { pairs: Vec::new() }
    }

    /// Add a color pair to check.
    pub fn add_pair(&mut self, name: &str, fg: Color, bg: Color) {
        self.pairs.push((name.to_string(), fg, bg));
    }

    /// Run contrast checks on all pairs.
    pub fn check_all(&self) -> Vec<(String, ContrastResult)> {
        self.pairs
            .iter()
            .map(|(name, fg, bg)| (name.clone(), analyze_contrast(fg, bg)))
            .collect()
    }

    /// Get pairs that fail AA normal text.
    pub fn failing_pairs(&self) -> Vec<(String, ContrastResult)> {
        self.check_all()
            .into_iter()
            .filter(|(_, r)| !r.passes_aa_normal)
            .collect()
    }

    /// Number of pairs.
    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
}

impl Default for PaletteChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_hex() {
        let c = Color::from_hex("#FF0000").unwrap();
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 0);
        assert_eq!(c.b, 0);
    }

    #[test]
    fn test_color_to_hex() {
        let c = Color::new(255, 128, 0);
        assert_eq!(c.to_hex(), "#FF8000");
    }

    #[test]
    fn test_color_from_hex_no_hash() {
        let c = Color::from_hex("00FF00").unwrap();
        assert_eq!(c.g, 255);
    }

    #[test]
    fn test_color_from_hex_invalid() {
        assert!(Color::from_hex("XYZ").is_none());
        assert!(Color::from_hex("#FFF").is_none());
    }

    #[test]
    fn test_relative_luminance() {
        let black = Color::new(0, 0, 0);
        let white = Color::new(255, 255, 255);
        assert!(black.relative_luminance() < 0.01);
        assert!((white.relative_luminance() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_contrast_ratio_black_white() {
        let black = Color::new(0, 0, 0);
        let white = Color::new(255, 255, 255);
        let ratio = contrast_ratio(&black, &white);
        assert!((ratio - 21.0).abs() < 0.1);
    }

    #[test]
    fn test_contrast_ratio_same_color() {
        let c = Color::new(128, 128, 128);
        let ratio = contrast_ratio(&c, &c);
        assert!((ratio - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_wcag_levels() {
        assert_eq!(wcag_level(21.0), WcagLevel::Aaa);
        assert_eq!(wcag_level(7.0), WcagLevel::Aaa);
        assert_eq!(wcag_level(5.0), WcagLevel::Aa);
        assert_eq!(wcag_level(4.5), WcagLevel::Aa);
        assert_eq!(wcag_level(3.5), WcagLevel::AaLarge);
        assert_eq!(wcag_level(2.0), WcagLevel::Fail);
    }

    #[test]
    fn test_analyze_contrast() {
        let result = analyze_contrast(&Color::new(0, 0, 0), &Color::new(255, 255, 255));
        assert!(result.passes_aa_normal);
        assert!(result.passes_aaa_normal);
        assert_eq!(result.level, WcagLevel::Aaa);
    }

    #[test]
    fn test_suggest_color_for_contrast() {
        let gray = Color::new(128, 128, 128);
        let white = Color::new(255, 255, 255);
        let suggested = suggest_color_for_contrast(&gray, &white, 4.5);
        assert!(suggested.is_some());
        let s = suggested.unwrap();
        assert!(contrast_ratio(&s, &white) >= 4.5);
    }

    #[test]
    fn test_palette_checker() {
        let mut checker = PaletteChecker::new();
        checker.add_pair("good", Color::new(0, 0, 0), Color::new(255, 255, 255));
        checker.add_pair("bad", Color::new(200, 200, 200), Color::new(255, 255, 255));

        let failing = checker.failing_pairs();
        assert_eq!(failing.len(), 1);
        assert_eq!(failing[0].0, "bad");
    }
}
