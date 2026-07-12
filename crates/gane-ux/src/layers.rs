//! Layer switchboard — manages map overlay visibility, ordering, and opacity.

use chrono::{DateTime, Utc};
use gane_core::types::EntityId;
use serde::{Deserialize, Serialize};
use tracing::debug;

/// A map layer that can be toggled, reordered, and configured.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapLayer {
    pub id: EntityId,
    pub name: String,
    pub layer_type: LayerType,
    pub visible: bool,
    pub opacity: f64,
    pub z_order: i32,
    pub min_zoom: Option<f64>,
    pub max_zoom: Option<f64>,
}

/// Built-in layer types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayerType {
    BaseMap,
    Satellite,
    Traffic,
    Risk,
    Weather,
    Stability,
    Uncertainty,
    Infrastructure,
    Parking,
    Charging,
    Transit,
    Bicycle,
    Pedestrian,
    Indoor,
    Custom,
}

/// Layer switchboard — controls which layers are visible and their configuration.
pub struct LayerSwitchboard {
    layers: Vec<MapLayer>,
    active_preset: Option<String>,
    change_log: Vec<LayerChange>,
}

/// A record of a layer change.
#[derive(Debug, Clone)]
pub struct LayerChange {
    pub layer_id: EntityId,
    pub field: String,
    pub timestamp: DateTime<Utc>,
}

/// A named collection of layer visibility settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerPreset {
    pub name: String,
    /// Layer types to enable.
    pub enabled: Vec<LayerType>,
}

impl LayerSwitchboard {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            active_preset: None,
            change_log: Vec::new(),
        }
    }

    /// Add a layer to the switchboard.
    pub fn add_layer(&mut self, layer: MapLayer) {
        debug!(name = %layer.name, layer_type = ?layer.layer_type, "layer added");
        self.layers.push(layer);
    }

    /// Toggle a layer's visibility by ID.
    pub fn toggle(&mut self, layer_id: &EntityId) -> Option<bool> {
        if let Some(layer) = self.layers.iter_mut().find(|l| l.id == *layer_id) {
            layer.visible = !layer.visible;
            self.change_log.push(LayerChange {
                layer_id: *layer_id,
                field: "visible".into(),
                timestamp: Utc::now(),
            });
            debug!(name = %layer.name, visible = layer.visible, "layer toggled");
            Some(layer.visible)
        } else {
            None
        }
    }

    /// Set a layer's opacity [0, 1].
    pub fn set_opacity(&mut self, layer_id: &EntityId, opacity: f64) -> bool {
        let opacity = opacity.clamp(0.0, 1.0);
        if let Some(layer) = self.layers.iter_mut().find(|l| l.id == *layer_id) {
            layer.opacity = opacity;
            self.change_log.push(LayerChange {
                layer_id: *layer_id,
                field: "opacity".into(),
                timestamp: Utc::now(),
            });
            true
        } else {
            false
        }
    }

    /// Reorder a layer to a new z-order position.
    pub fn set_z_order(&mut self, layer_id: &EntityId, z_order: i32) -> bool {
        if let Some(layer) = self.layers.iter_mut().find(|l| l.id == *layer_id) {
            layer.z_order = z_order;
            true
        } else {
            false
        }
    }

    /// Get all visible layers sorted by z-order.
    pub fn visible_layers(&self) -> Vec<&MapLayer> {
        let mut visible: Vec<&MapLayer> = self.layers.iter().filter(|l| l.visible).collect();
        visible.sort_by_key(|l| l.z_order);
        visible
    }

    /// Get all layers.
    pub fn all_layers(&self) -> &[MapLayer] {
        &self.layers
    }

    /// Get layers visible at a specific zoom level.
    pub fn layers_at_zoom(&self, zoom: f64) -> Vec<&MapLayer> {
        self.layers
            .iter()
            .filter(|l| {
                if !l.visible {
                    return false;
                }
                if let Some(min) = l.min_zoom {
                    if zoom < min {
                        return false;
                    }
                }
                if let Some(max) = l.max_zoom {
                    if zoom > max {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    /// Apply a layer preset — enables only the specified layer types.
    pub fn apply_preset(&mut self, preset: &LayerPreset) {
        for layer in &mut self.layers {
            layer.visible = preset.enabled.contains(&layer.layer_type);
        }
        self.active_preset = Some(preset.name.clone());
        debug!(preset = %preset.name, "layer preset applied");
    }

    /// Get the active preset name.
    pub fn active_preset(&self) -> Option<&str> {
        self.active_preset.as_deref()
    }

    /// Get the number of change events.
    pub fn change_count(&self) -> usize {
        self.change_log.len()
    }

    /// Find a layer by type.
    pub fn find_by_type(&self, layer_type: LayerType) -> Option<&MapLayer> {
        self.layers.iter().find(|l| l.layer_type == layer_type)
    }
}

impl Default for LayerSwitchboard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_layer(name: &str, layer_type: LayerType, z_order: i32) -> MapLayer {
        MapLayer {
            id: EntityId::new(),
            name: name.into(),
            layer_type,
            visible: true,
            opacity: 1.0,
            z_order,
            min_zoom: None,
            max_zoom: None,
        }
    }

    #[test]
    fn add_and_list_layers() {
        let mut sb = LayerSwitchboard::new();
        sb.add_layer(make_layer("Base", LayerType::BaseMap, 0));
        sb.add_layer(make_layer("Traffic", LayerType::Traffic, 10));
        assert_eq!(sb.all_layers().len(), 2);
    }

    #[test]
    fn toggle_visibility() {
        let mut sb = LayerSwitchboard::new();
        let layer = make_layer("Traffic", LayerType::Traffic, 10);
        let id = layer.id;
        sb.add_layer(layer);

        assert_eq!(sb.visible_layers().len(), 1);
        sb.toggle(&id);
        assert_eq!(sb.visible_layers().len(), 0);
        sb.toggle(&id);
        assert_eq!(sb.visible_layers().len(), 1);
    }

    #[test]
    fn toggle_nonexistent_returns_none() {
        let mut sb = LayerSwitchboard::new();
        assert!(sb.toggle(&EntityId::new()).is_none());
    }

    #[test]
    fn opacity_clamped() {
        let mut sb = LayerSwitchboard::new();
        let layer = make_layer("Risk", LayerType::Risk, 5);
        let id = layer.id;
        sb.add_layer(layer);

        sb.set_opacity(&id, 1.5);
        assert!((sb.all_layers()[0].opacity - 1.0).abs() < f64::EPSILON);

        sb.set_opacity(&id, -0.5);
        assert!(sb.all_layers()[0].opacity.abs() < f64::EPSILON);
    }

    #[test]
    fn visible_layers_sorted_by_z_order() {
        let mut sb = LayerSwitchboard::new();
        sb.add_layer(make_layer("Top", LayerType::Risk, 20));
        sb.add_layer(make_layer("Base", LayerType::BaseMap, 0));
        sb.add_layer(make_layer("Mid", LayerType::Traffic, 10));

        let visible = sb.visible_layers();
        assert_eq!(visible[0].name, "Base");
        assert_eq!(visible[1].name, "Mid");
        assert_eq!(visible[2].name, "Top");
    }

    #[test]
    fn zoom_filtering() {
        let mut sb = LayerSwitchboard::new();
        let mut layer = make_layer("Detail", LayerType::Indoor, 5);
        layer.min_zoom = Some(15.0);
        layer.max_zoom = Some(20.0);
        sb.add_layer(layer);

        assert_eq!(sb.layers_at_zoom(10.0).len(), 0);
        assert_eq!(sb.layers_at_zoom(17.0).len(), 1);
        assert_eq!(sb.layers_at_zoom(25.0).len(), 0);
    }

    #[test]
    fn preset_application() {
        let mut sb = LayerSwitchboard::new();
        sb.add_layer(make_layer("Base", LayerType::BaseMap, 0));
        sb.add_layer(make_layer("Traffic", LayerType::Traffic, 10));
        sb.add_layer(make_layer("Risk", LayerType::Risk, 20));

        let preset = LayerPreset {
            name: "Highway".into(),
            enabled: vec![LayerType::BaseMap],
        };

        sb.apply_preset(&preset);
        assert_eq!(sb.visible_layers().len(), 1);
        assert_eq!(sb.visible_layers()[0].layer_type, LayerType::BaseMap);
        assert_eq!(sb.active_preset(), Some("Highway"));
    }

    #[test]
    fn find_by_type() {
        let mut sb = LayerSwitchboard::new();
        sb.add_layer(make_layer("Traffic", LayerType::Traffic, 10));
        assert!(sb.find_by_type(LayerType::Traffic).is_some());
        assert!(sb.find_by_type(LayerType::Risk).is_none());
    }

    #[test]
    fn change_log_tracked() {
        let mut sb = LayerSwitchboard::new();
        let layer = make_layer("Risk", LayerType::Risk, 5);
        let id = layer.id;
        sb.add_layer(layer);

        sb.toggle(&id);
        sb.set_opacity(&id, 0.5);
        assert_eq!(sb.change_count(), 2);
    }
}
