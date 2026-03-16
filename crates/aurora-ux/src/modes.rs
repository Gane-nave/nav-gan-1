//! Driving mode manager — controls interface complexity, alert thresholds,
//! and rendering density based on context (speed, cognitive load, mode).

use aurora_core::types::{DayNightMode, EntityId, UsageMode};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::debug;

/// Interface complexity level — determines how much information is shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterfaceComplexity {
    /// Minimal: speed, next turn, distance only.
    Minimal,
    /// Standard: turn-by-turn, ETA, traffic summary.
    Standard,
    /// Detailed: lane guidance, risk overlay, traffic detail.
    Detailed,
    /// Professional: all layers, diagnostics, fleet info.
    Professional,
}

/// Alert priority level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AlertPriority {
    /// Informational — shown only if interface is not minimal.
    Info,
    /// Warning — shown in standard and above.
    Warning,
    /// Urgent — always shown, audio alert.
    Urgent,
    /// Critical — always shown, persistent audio, haptic.
    Critical,
}

/// Configuration for a driving mode preset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModePreset {
    pub name: String,
    pub complexity: InterfaceComplexity,
    pub min_alert_priority: AlertPriority,
    pub show_speed_limit: bool,
    pub show_traffic_overlay: bool,
    pub show_risk_overlay: bool,
    pub show_lane_guidance: bool,
    pub show_eta: bool,
    pub voice_guidance: bool,
    pub auto_zoom: bool,
    pub day_night: DayNightMode,
    /// Speed threshold (km/h) above which the interface auto-simplifies.
    pub auto_simplify_speed_kmh: Option<f64>,
}

impl ModePreset {
    /// Create the default "Highway" preset — simplified for high-speed driving.
    pub fn highway() -> Self {
        Self {
            name: "Highway".into(),
            complexity: InterfaceComplexity::Minimal,
            min_alert_priority: AlertPriority::Warning,
            show_speed_limit: true,
            show_traffic_overlay: false,
            show_risk_overlay: false,
            show_lane_guidance: true,
            show_eta: true,
            voice_guidance: true,
            auto_zoom: true,
            day_night: DayNightMode::Auto,
            auto_simplify_speed_kmh: None,
        }
    }

    /// Create the default "City" preset — standard detail for urban driving.
    pub fn city() -> Self {
        Self {
            name: "City".into(),
            complexity: InterfaceComplexity::Standard,
            min_alert_priority: AlertPriority::Info,
            show_speed_limit: true,
            show_traffic_overlay: true,
            show_risk_overlay: false,
            show_lane_guidance: true,
            show_eta: true,
            voice_guidance: true,
            auto_zoom: true,
            day_night: DayNightMode::Auto,
            auto_simplify_speed_kmh: Some(80.0),
        }
    }

    /// Create the "Night" preset — reduced brightness, minimal distractions.
    pub fn night() -> Self {
        Self {
            name: "Night".into(),
            complexity: InterfaceComplexity::Minimal,
            min_alert_priority: AlertPriority::Warning,
            show_speed_limit: true,
            show_traffic_overlay: false,
            show_risk_overlay: false,
            show_lane_guidance: true,
            show_eta: true,
            voice_guidance: true,
            auto_zoom: true,
            day_night: DayNightMode::Night,
            auto_simplify_speed_kmh: None,
        }
    }

    /// Create the "Professional" preset — full detail for fleet/operations.
    pub fn professional() -> Self {
        Self {
            name: "Professional".into(),
            complexity: InterfaceComplexity::Professional,
            min_alert_priority: AlertPriority::Info,
            show_speed_limit: true,
            show_traffic_overlay: true,
            show_risk_overlay: true,
            show_lane_guidance: true,
            show_eta: true,
            voice_guidance: false,
            auto_zoom: false,
            day_night: DayNightMode::Auto,
            auto_simplify_speed_kmh: None,
        }
    }

    /// Create the "Emergency" preset — critical info only, high contrast.
    pub fn emergency() -> Self {
        Self {
            name: "Emergency".into(),
            complexity: InterfaceComplexity::Minimal,
            min_alert_priority: AlertPriority::Urgent,
            show_speed_limit: false,
            show_traffic_overlay: false,
            show_risk_overlay: false,
            show_lane_guidance: false,
            show_eta: true,
            voice_guidance: true,
            auto_zoom: true,
            day_night: DayNightMode::Auto,
            auto_simplify_speed_kmh: None,
        }
    }
}

/// Cognitive load assessment for UI adaptation.
#[derive(Debug, Clone, Copy)]
pub struct CognitiveContext {
    /// Current speed in km/h.
    pub speed_kmh: f64,
    /// Driver cognitive load [0, 1].
    pub cognitive_load: f64,
    /// Whether it's currently dark outside.
    pub is_night: bool,
    /// Whether it's raining.
    pub is_raining: bool,
    /// Current usage mode.
    pub usage_mode: UsageMode,
}

/// A decision about whether an alert should be shown.
#[derive(Debug, Clone)]
pub struct AlertDecision {
    pub should_show: bool,
    pub should_play_audio: bool,
    pub should_haptic: bool,
    pub effective_priority: AlertPriority,
}

/// Mode manager — adapts UI complexity based on driving context.
pub struct ModeManager {
    active_preset: ModePreset,
    effective_complexity: InterfaceComplexity,
    presets: Vec<ModePreset>,
    history: Vec<ModeTransition>,
}

/// Record of a mode transition.
#[derive(Debug, Clone)]
pub struct ModeTransition {
    pub from: String,
    pub to: String,
    pub reason: String,
    pub timestamp: DateTime<Utc>,
}

impl ModeManager {
    pub fn new() -> Self {
        Self {
            active_preset: ModePreset::city(),
            effective_complexity: InterfaceComplexity::Standard,
            presets: vec![
                ModePreset::highway(),
                ModePreset::city(),
                ModePreset::night(),
                ModePreset::professional(),
                ModePreset::emergency(),
            ],
            history: Vec::new(),
        }
    }

    /// Switch to a named preset.
    pub fn switch_preset(&mut self, name: &str) -> bool {
        if let Some(preset) = self.presets.iter().find(|p| p.name == name).cloned() {
            let old_name = self.active_preset.name.clone();
            self.effective_complexity = preset.complexity;
            self.active_preset = preset;

            self.history.push(ModeTransition {
                from: old_name,
                to: self.active_preset.name.clone(),
                reason: "manual switch".into(),
                timestamp: Utc::now(),
            });

            debug!(preset = %self.active_preset.name, "mode preset switched");
            true
        } else {
            false
        }
    }

    /// Add a custom preset.
    pub fn add_preset(&mut self, preset: ModePreset) {
        self.presets.push(preset);
    }

    /// Adapt the interface based on cognitive context.
    ///
    /// Returns true if the complexity was changed.
    pub fn adapt(&mut self, ctx: &CognitiveContext) -> bool {
        let new_complexity = self.compute_effective_complexity(ctx);

        if new_complexity != self.effective_complexity {
            debug!(
                from = ?self.effective_complexity,
                to = ?new_complexity,
                speed = ctx.speed_kmh,
                cognitive_load = ctx.cognitive_load,
                "interface complexity adapted"
            );

            self.history.push(ModeTransition {
                from: format!("{:?}", self.effective_complexity),
                to: format!("{:?}", new_complexity),
                reason: format!(
                    "auto-adapt: speed={:.0} load={:.2}",
                    ctx.speed_kmh, ctx.cognitive_load
                ),
                timestamp: Utc::now(),
            });

            self.effective_complexity = new_complexity;
            true
        } else {
            false
        }
    }

    /// Compute the effective complexity given context.
    fn compute_effective_complexity(&self, ctx: &CognitiveContext) -> InterfaceComplexity {
        // Check auto-simplify speed threshold.
        if let Some(threshold) = self.active_preset.auto_simplify_speed_kmh {
            if ctx.speed_kmh > threshold {
                return InterfaceComplexity::Minimal;
            }
        }

        // Emergency mode always stays minimal — checked before cognitive load.
        if ctx.usage_mode == UsageMode::EmergencyResponse {
            return InterfaceComplexity::Minimal;
        }

        // High cognitive load → simplify.
        if ctx.cognitive_load > 0.8 {
            return InterfaceComplexity::Minimal;
        }

        if ctx.cognitive_load > 0.6 {
            return match self.active_preset.complexity {
                InterfaceComplexity::Professional => InterfaceComplexity::Detailed,
                InterfaceComplexity::Detailed => InterfaceComplexity::Standard,
                other => other,
            };
        }

        self.active_preset.complexity
    }

    /// Decide whether an alert should be shown given current mode.
    pub fn filter_alert(&self, priority: AlertPriority) -> AlertDecision {
        let min = self.active_preset.min_alert_priority;

        if priority >= min {
            AlertDecision {
                should_show: true,
                should_play_audio: priority >= AlertPriority::Urgent
                    && self.active_preset.voice_guidance,
                should_haptic: priority >= AlertPriority::Critical,
                effective_priority: priority,
            }
        } else {
            AlertDecision {
                should_show: false,
                should_play_audio: false,
                should_haptic: false,
                effective_priority: priority,
            }
        }
    }

    /// Get the currently effective interface complexity.
    pub fn effective_complexity(&self) -> InterfaceComplexity {
        self.effective_complexity
    }

    /// Get the active preset name.
    pub fn active_preset_name(&self) -> &str {
        &self.active_preset.name
    }

    /// Get the active preset.
    pub fn active_preset(&self) -> &ModePreset {
        &self.active_preset
    }

    /// Get mode transition history.
    pub fn history(&self) -> &[ModeTransition] {
        &self.history
    }

    /// Get the recommended day/night mode.
    pub fn day_night_mode(&self, is_night: bool) -> DayNightMode {
        match self.active_preset.day_night {
            DayNightMode::Auto => {
                if is_night {
                    DayNightMode::Night
                } else {
                    DayNightMode::Day
                }
            }
            explicit => explicit,
        }
    }

    /// Create a UI visibility struct for the current state.
    pub fn visibility(&self) -> UiVisibility {
        UiVisibility {
            complexity: self.effective_complexity,
            show_speed_limit: self.active_preset.show_speed_limit,
            show_traffic: self.active_preset.show_traffic_overlay,
            show_risk: self.active_preset.show_risk_overlay,
            show_lanes: self.active_preset.show_lane_guidance,
            show_eta: self.active_preset.show_eta,
            voice_enabled: self.active_preset.voice_guidance,
            auto_zoom: self.active_preset.auto_zoom,
        }
    }
}

impl Default for ModeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Snapshot of what UI elements should be visible.
#[derive(Debug, Clone)]
pub struct UiVisibility {
    pub complexity: InterfaceComplexity,
    pub show_speed_limit: bool,
    pub show_traffic: bool,
    pub show_risk: bool,
    pub show_lanes: bool,
    pub show_eta: bool,
    pub voice_enabled: bool,
    pub auto_zoom: bool,
}

// Suppress unused warning on EntityId import — used for consistency with other crates.
const _: () = {
    let _ = std::mem::size_of::<EntityId>();
};

#[cfg(test)]
mod tests {
    use super::*;

    fn driving_ctx(speed: f64, load: f64) -> CognitiveContext {
        CognitiveContext {
            speed_kmh: speed,
            cognitive_load: load,
            is_night: false,
            is_raining: false,
            usage_mode: UsageMode::ActiveNavigation,
        }
    }

    #[test]
    fn default_mode_is_city() {
        let mgr = ModeManager::new();
        assert_eq!(mgr.active_preset_name(), "City");
        assert_eq!(mgr.effective_complexity(), InterfaceComplexity::Standard);
    }

    #[test]
    fn switch_to_highway_preset() {
        let mut mgr = ModeManager::new();
        assert!(mgr.switch_preset("Highway"));
        assert_eq!(mgr.active_preset_name(), "Highway");
        assert_eq!(mgr.effective_complexity(), InterfaceComplexity::Minimal);
    }

    #[test]
    fn switch_to_nonexistent_preset_fails() {
        let mut mgr = ModeManager::new();
        assert!(!mgr.switch_preset("NonExistent"));
        assert_eq!(mgr.active_preset_name(), "City");
    }

    #[test]
    fn high_speed_auto_simplifies() {
        let mut mgr = ModeManager::new();
        // City preset has auto_simplify at 80 km/h.
        let ctx = driving_ctx(100.0, 0.3);
        assert!(mgr.adapt(&ctx));
        assert_eq!(mgr.effective_complexity(), InterfaceComplexity::Minimal);
    }

    #[test]
    fn normal_speed_keeps_standard() {
        let mut mgr = ModeManager::new();
        let ctx = driving_ctx(50.0, 0.3);
        assert!(!mgr.adapt(&ctx));
        assert_eq!(mgr.effective_complexity(), InterfaceComplexity::Standard);
    }

    #[test]
    fn high_cognitive_load_simplifies() {
        let mut mgr = ModeManager::new();
        let ctx = driving_ctx(30.0, 0.9);
        assert!(mgr.adapt(&ctx));
        assert_eq!(mgr.effective_complexity(), InterfaceComplexity::Minimal);
    }

    #[test]
    fn moderate_cognitive_load_steps_down_professional() {
        let mut mgr = ModeManager::new();
        mgr.switch_preset("Professional");
        let ctx = driving_ctx(30.0, 0.7);
        assert!(mgr.adapt(&ctx));
        assert_eq!(mgr.effective_complexity(), InterfaceComplexity::Detailed);
    }

    #[test]
    fn alert_filtering_by_priority() {
        let mgr = ModeManager::new(); // City: min = Info
        let info = mgr.filter_alert(AlertPriority::Info);
        assert!(info.should_show);
        assert!(!info.should_play_audio);

        let critical = mgr.filter_alert(AlertPriority::Critical);
        assert!(critical.should_show);
        assert!(critical.should_play_audio);
        assert!(critical.should_haptic);
    }

    #[test]
    fn highway_preset_filters_info_alerts() {
        let mut mgr = ModeManager::new();
        mgr.switch_preset("Highway");
        let info = mgr.filter_alert(AlertPriority::Info);
        assert!(!info.should_show);
        let warning = mgr.filter_alert(AlertPriority::Warning);
        assert!(warning.should_show);
    }

    #[test]
    fn emergency_mode_forces_minimal() {
        let mut mgr = ModeManager::new();
        let ctx = CognitiveContext {
            speed_kmh: 30.0,
            cognitive_load: 0.3,
            is_night: false,
            is_raining: false,
            usage_mode: UsageMode::EmergencyResponse,
        };
        assert!(mgr.adapt(&ctx));
        assert_eq!(mgr.effective_complexity(), InterfaceComplexity::Minimal);
    }

    #[test]
    fn adversarial_emergency_at_moderate_cognitive_load() {
        // This test WOULD FAIL with the old code where emergency check was after
        // moderate cognitive load (0.6-0.8) check. At load=0.7, the moderate check
        // would fire first and return Standard/Detailed instead of Minimal.
        let mut mgr = ModeManager::new();
        let ctx = CognitiveContext {
            speed_kmh: 30.0,
            cognitive_load: 0.7, // The adversarial value — 0.6-0.8 range was broken
            is_night: false,
            is_raining: false,
            usage_mode: UsageMode::EmergencyResponse,
        };
        assert!(mgr.adapt(&ctx));
        assert_eq!(
            mgr.effective_complexity(),
            InterfaceComplexity::Minimal,
            "BUG: Emergency at load=0.7 must be Minimal, not {:?}",
            mgr.effective_complexity()
        );
    }

    #[test]
    fn day_night_auto_resolves() {
        let mgr = ModeManager::new();
        assert_eq!(mgr.day_night_mode(false), DayNightMode::Day);
        assert_eq!(mgr.day_night_mode(true), DayNightMode::Night);
    }

    #[test]
    fn visibility_reflects_preset() {
        let mut mgr = ModeManager::new();
        mgr.switch_preset("Professional");
        let vis = mgr.visibility();
        assert!(vis.show_risk);
        assert!(vis.show_traffic);
        assert!(!vis.voice_enabled);
    }

    #[test]
    fn mode_transitions_tracked() {
        let mut mgr = ModeManager::new();
        mgr.switch_preset("Highway");
        mgr.switch_preset("Night");
        assert_eq!(mgr.history().len(), 2);
        assert_eq!(mgr.history()[0].from, "City");
        assert_eq!(mgr.history()[0].to, "Highway");
    }
}
