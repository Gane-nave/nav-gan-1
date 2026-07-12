//! Augmented Reality navigation overlay with 3D waypoints and lane projection

use serde::{Deserialize, Serialize};

/// 3D point in AR space (metres relative to camera).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ArPoint3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ArPoint3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn distance_to(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }

    /// Project 3D point to 2D screen coordinates using simple pinhole camera model.
    pub fn project_to_screen(
        &self,
        focal_length: f64,
        screen_w: f64,
        screen_h: f64,
    ) -> Option<(f64, f64)> {
        if self.z <= 0.1 {
            return None; // behind camera
        }
        let sx = (self.x * focal_length / self.z) + screen_w / 2.0;
        let sy = (self.y * focal_length / self.z) + screen_h / 2.0;
        if (0.0..=screen_w).contains(&sx) && (0.0..=screen_h).contains(&sy) {
            Some((sx, sy))
        } else {
            None // off screen
        }
    }
}

/// AR overlay element type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArElementType {
    /// Navigation arrow on road surface
    NavArrow,
    /// Turn indicator
    TurnIndicator,
    /// Lane guidance highlight
    LaneHighlight,
    /// Speed limit sign
    SpeedSign,
    /// Point of interest marker
    PoiMarker,
    /// Distance marker
    DistanceMarker,
    /// Hazard warning
    HazardWarning,
    /// Parking spot indicator
    ParkingSpot,
}

/// An AR overlay element to render.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArElement {
    pub element_type: ArElementType,
    pub position: ArPoint3D,
    pub opacity: f64,
    pub scale: f64,
    pub label: Option<String>,
    pub color_rgba: [u8; 4],
}

impl ArElement {
    pub fn new(element_type: ArElementType, position: ArPoint3D) -> Self {
        Self {
            element_type,
            position,
            opacity: 1.0,
            scale: 1.0,
            label: None,
            color_rgba: [0, 200, 100, 255],
        }
    }

    /// Fade element based on distance -- farther elements are more transparent.
    pub fn apply_distance_fade(&mut self, max_distance: f64) {
        let dist = self.position.distance_to(&ArPoint3D::new(0.0, 0.0, 0.0));
        self.opacity = (1.0 - dist / max_distance).clamp(0.1, 1.0);
    }

    /// Scale element based on distance -- farther elements appear smaller.
    pub fn apply_distance_scale(&mut self, base_scale: f64, max_distance: f64) {
        let dist = self.position.z.max(0.1);
        self.scale = (base_scale * max_distance / dist).clamp(0.1, 5.0);
    }
}

/// Lane projection for AR lane guidance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaneProjection {
    pub left_points: Vec<ArPoint3D>,
    pub right_points: Vec<ArPoint3D>,
    pub center_points: Vec<ArPoint3D>,
    pub lane_width_m: f64,
    pub is_highlighted: bool,
}

impl LaneProjection {
    /// Generate a straight lane projection ahead of camera.
    pub fn generate_straight(lane_width_m: f64, distance_m: f64, num_points: usize) -> Self {
        let half = lane_width_m / 2.0;
        let step = distance_m / num_points as f64;
        let mut left = Vec::new();
        let mut right = Vec::new();
        let mut center = Vec::new();
        for i in 0..num_points {
            let z = (i as f64 + 1.0) * step;
            left.push(ArPoint3D::new(-half, 0.0, z));
            right.push(ArPoint3D::new(half, 0.0, z));
            center.push(ArPoint3D::new(0.0, 0.0, z));
        }
        Self {
            left_points: left,
            right_points: right,
            center_points: center,
            lane_width_m,
            is_highlighted: false,
        }
    }

    /// Generate a curved lane projection.
    pub fn generate_curve(
        lane_width_m: f64,
        radius_m: f64,
        angle_deg: f64,
        num_points: usize,
    ) -> Self {
        let half = lane_width_m / 2.0;
        let angle_rad = angle_deg.to_radians();
        let step = angle_rad / num_points as f64;
        let mut left = Vec::new();
        let mut right = Vec::new();
        let mut center = Vec::new();
        for i in 0..num_points {
            let theta = (i as f64 + 1.0) * step;
            let cx = radius_m * theta.sin();
            let cz = radius_m * (1.0 - theta.cos());
            let nx = theta.cos();
            let nz = theta.sin();
            center.push(ArPoint3D::new(cx, 0.0, cz));
            left.push(ArPoint3D::new(cx - half * nx, 0.0, cz - half * nz));
            right.push(ArPoint3D::new(cx + half * nx, 0.0, cz + half * nz));
        }
        Self {
            left_points: left,
            right_points: right,
            center_points: center,
            lane_width_m,
            is_highlighted: true,
        }
    }

    /// Number of points in the projection.
    pub fn point_count(&self) -> usize {
        self.center_points.len()
    }
}

/// AR Navigation engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArNavEngine {
    elements: Vec<ArElement>,
    lane_projection: Option<LaneProjection>,
    focal_length: f64,
    screen_width: f64,
    screen_height: f64,
    max_render_distance: f64,
    elements_rendered: u64,
    is_active: bool,
}

impl Default for ArNavEngine {
    fn default() -> Self {
        Self {
            elements: Vec::new(),
            lane_projection: None,
            focal_length: 500.0,
            screen_width: 1920.0,
            screen_height: 1080.0,
            max_render_distance: 200.0,
            elements_rendered: 0,
            is_active: false,
        }
    }
}

impl ArNavEngine {
    pub fn new(focal_length: f64, screen_width: f64, screen_height: f64) -> Self {
        Self {
            focal_length,
            screen_width,
            screen_height,
            is_active: true,
            ..Default::default()
        }
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn element_count(&self) -> usize {
        self.elements.len()
    }

    pub fn elements_rendered(&self) -> u64 {
        self.elements_rendered
    }

    /// Add an AR element.
    pub fn add_element(&mut self, element: ArElement) {
        self.elements.push(element);
    }

    /// Clear all elements.
    pub fn clear_elements(&mut self) {
        self.elements.clear();
    }

    /// Set lane projection.
    pub fn set_lane_projection(&mut self, projection: LaneProjection) {
        self.lane_projection = Some(projection);
    }

    /// Get lane projection.
    pub fn lane_projection(&self) -> Option<&LaneProjection> {
        self.lane_projection.as_ref()
    }

    /// Add a navigation arrow at a 3D position.
    pub fn add_nav_arrow(&mut self, x: f64, y: f64, z: f64) {
        self.add_element(ArElement::new(
            ArElementType::NavArrow,
            ArPoint3D::new(x, y, z),
        ));
    }

    /// Add a turn indicator with label.
    pub fn add_turn_indicator(&mut self, x: f64, y: f64, z: f64, label: &str) {
        let mut e = ArElement::new(ArElementType::TurnIndicator, ArPoint3D::new(x, y, z));
        e.label = Some(label.to_string());
        e.color_rgba = [255, 200, 0, 255];
        self.add_element(e);
    }

    /// Render pass -- project all elements, apply fading, return screen coordinates.
    pub fn render(&mut self) -> Vec<(ArElementType, f64, f64, f64)> {
        if !self.is_active {
            return Vec::new();
        }

        let mut result = Vec::new();
        for elem in &mut self.elements {
            elem.apply_distance_fade(self.max_render_distance);
            elem.apply_distance_scale(1.0, self.max_render_distance);
            if let Some((sx, sy)) = elem.position.project_to_screen(
                self.focal_length,
                self.screen_width,
                self.screen_height,
            ) {
                result.push((elem.element_type, sx, sy, elem.opacity));
                self.elements_rendered += 1;
            }
        }
        result
    }

    /// Sort elements by depth (back-to-front) for correct rendering order.
    pub fn sort_by_depth(&mut self) {
        self.elements.sort_by(|a, b| {
            b.position
                .z
                .partial_cmp(&a.position.z)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_distance() {
        let a = ArPoint3D::new(0.0, 0.0, 0.0);
        let b = ArPoint3D::new(3.0, 4.0, 0.0);
        assert!((a.distance_to(&b) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_project_behind_camera() {
        let p = ArPoint3D::new(0.0, 0.0, -1.0);
        assert!(p.project_to_screen(500.0, 1920.0, 1080.0).is_none());
    }

    #[test]
    fn test_project_center() {
        let p = ArPoint3D::new(0.0, 0.0, 10.0);
        let result = p.project_to_screen(500.0, 1920.0, 1080.0);
        assert!(result.is_some());
        let (sx, sy) = result.unwrap();
        assert!(
            (sx - 960.0).abs() < 1.0,
            "Center should project to screen center x"
        );
        assert!(
            (sy - 540.0).abs() < 1.0,
            "Center should project to screen center y"
        );
    }

    #[test]
    fn test_element_fade() {
        let mut e = ArElement::new(ArElementType::NavArrow, ArPoint3D::new(0.0, 0.0, 100.0));
        e.apply_distance_fade(200.0);
        assert!(
            e.opacity < 1.0 && e.opacity > 0.0,
            "Should fade at 100m of 200m max"
        );
    }

    #[test]
    fn test_element_scale() {
        let mut e = ArElement::new(ArElementType::NavArrow, ArPoint3D::new(0.0, 0.0, 50.0));
        e.apply_distance_scale(1.0, 200.0);
        assert!(e.scale > 1.0, "Close elements should scale up: {}", e.scale);
    }

    #[test]
    fn test_lane_straight() {
        let lane = LaneProjection::generate_straight(3.5, 100.0, 20);
        assert_eq!(lane.point_count(), 20);
        assert_eq!(lane.left_points.len(), 20);
        assert_eq!(lane.right_points.len(), 20);
        assert!((lane.left_points[0].x - (-1.75)).abs() < 0.01);
        assert!((lane.right_points[0].x - 1.75).abs() < 0.01);
    }

    #[test]
    fn test_lane_curve() {
        let lane = LaneProjection::generate_curve(3.5, 50.0, 45.0, 10);
        assert_eq!(lane.point_count(), 10);
        assert!(lane.is_highlighted);
        assert!(lane.center_points.last().unwrap().x > 0.0);
    }

    #[test]
    fn test_engine_default() {
        let e = ArNavEngine::default();
        assert!(!e.is_active());
        assert_eq!(e.element_count(), 0);
    }

    #[test]
    fn test_engine_add_elements() {
        let mut e = ArNavEngine::new(500.0, 1920.0, 1080.0);
        e.add_nav_arrow(0.0, 0.0, 30.0);
        e.add_turn_indicator(2.0, 0.0, 50.0, "Turn Right");
        assert_eq!(e.element_count(), 2);
    }

    #[test]
    fn test_render_inactive() {
        let mut e = ArNavEngine::default();
        e.add_nav_arrow(0.0, 0.0, 30.0);
        let rendered = e.render();
        assert!(rendered.is_empty(), "Inactive engine should render nothing");
    }

    #[test]
    fn test_render_active() {
        let mut e = ArNavEngine::new(500.0, 1920.0, 1080.0);
        e.add_nav_arrow(0.0, 0.0, 30.0);
        e.add_nav_arrow(0.0, 0.0, 60.0);
        let rendered = e.render();
        assert_eq!(rendered.len(), 2);
        assert_eq!(e.elements_rendered(), 2);
    }

    #[test]
    fn test_sort_by_depth() {
        let mut e = ArNavEngine::new(500.0, 1920.0, 1080.0);
        e.add_nav_arrow(0.0, 0.0, 10.0);
        e.add_nav_arrow(0.0, 0.0, 50.0);
        e.add_nav_arrow(0.0, 0.0, 30.0);
        e.sort_by_depth();
        assert!(e.elements[0].position.z > e.elements[1].position.z);
        assert!(e.elements[1].position.z > e.elements[2].position.z);
    }

    #[test]
    fn test_clear_elements() {
        let mut e = ArNavEngine::new(500.0, 1920.0, 1080.0);
        e.add_nav_arrow(0.0, 0.0, 10.0);
        e.add_nav_arrow(0.0, 0.0, 20.0);
        assert_eq!(e.element_count(), 2);
        e.clear_elements();
        assert_eq!(e.element_count(), 0);
    }

    #[test]
    fn test_lane_projection_set() {
        let mut e = ArNavEngine::new(500.0, 1920.0, 1080.0);
        assert!(e.lane_projection().is_none());
        e.set_lane_projection(LaneProjection::generate_straight(3.5, 100.0, 20));
        assert!(e.lane_projection().is_some());
        assert_eq!(e.lane_projection().unwrap().point_count(), 20);
    }
}
