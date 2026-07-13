//! 3D scene management — camera, viewport, and scene-graph primitives
//! for rendering immersive navigation views.

use serde::{Deserialize, Serialize};

/// 3D vector.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len < 1e-12 {
            return Self::zero();
        }
        Self::new(self.x / len, self.y / len, self.z / len)
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

/// Camera projection type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Projection {
    /// Perspective projection (realistic depth)
    Perspective,
    /// Orthographic projection (no depth scaling)
    Orthographic,
}

/// Virtual camera for 3D scene rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera {
    /// Camera position in world space
    pub position: Vec3,
    /// Look-at target
    pub target: Vec3,
    /// Up direction
    pub up: Vec3,
    /// Field of view in degrees (perspective only)
    pub fov_deg: f64,
    /// Near clipping plane distance
    pub near: f64,
    /// Far clipping plane distance
    pub far: f64,
    /// Projection type
    pub projection: Projection,
}

impl Camera {
    /// Create a default perspective camera.
    pub fn perspective(position: Vec3, target: Vec3, fov_deg: f64) -> Self {
        Self {
            position,
            target,
            up: Vec3::new(0.0, 1.0, 0.0),
            fov_deg,
            near: 0.1,
            far: 10_000.0,
            projection: Projection::Perspective,
        }
    }

    /// Compute the forward direction vector.
    pub fn forward(&self) -> Vec3 {
        let dir = Vec3::new(
            self.target.x - self.position.x,
            self.target.y - self.position.y,
            self.target.z - self.position.z,
        );
        dir.normalize()
    }

    /// Compute the right direction vector.
    pub fn right(&self) -> Vec3 {
        self.forward().cross(&self.up).normalize()
    }
}

/// Render quality level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RenderQuality {
    /// Minimal rendering for low-power devices
    Low,
    /// Balanced quality/performance
    Medium,
    /// High quality rendering
    High,
    /// Ultra quality (desktop/high-end devices)
    Ultra,
}

/// Scene configuration for the 3D navigation view.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneConfig {
    /// Render quality level
    pub quality: RenderQuality,
    /// Viewport width in pixels
    pub viewport_width: u32,
    /// Viewport height in pixels
    pub viewport_height: u32,
    /// Enable shadows
    pub shadows: bool,
    /// Enable anti-aliasing
    pub anti_aliasing: bool,
    /// Enable fog/haze for depth perception
    pub fog: bool,
    /// Fog start distance (meters)
    pub fog_start: f64,
    /// Fog end distance (meters)
    pub fog_end: f64,
    /// Target frames per second
    pub target_fps: u32,
}

impl Default for SceneConfig {
    fn default() -> Self {
        Self {
            quality: RenderQuality::Medium,
            viewport_width: 1920,
            viewport_height: 1080,
            shadows: true,
            anti_aliasing: true,
            fog: true,
            fog_start: 500.0,
            fog_end: 5000.0,
            target_fps: 60,
        }
    }
}

/// A renderable object in the scene graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneObject {
    /// Unique identifier
    pub id: String,
    /// Object type
    pub object_type: SceneObjectType,
    /// Position in world space
    pub position: Vec3,
    /// Scale factors
    pub scale: Vec3,
    /// Rotation in euler angles (degrees)
    pub rotation_deg: Vec3,
    /// Whether the object is visible
    pub visible: bool,
    /// Opacity (0.0 = transparent, 1.0 = opaque)
    pub opacity: f32,
}

/// Type of scene object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SceneObjectType {
    /// Road surface
    Road,
    /// Building footprint/3D model
    Building,
    /// Navigation arrow/chevron
    NavArrow,
    /// Turn indicator
    TurnIndicator,
    /// Lane marking
    LaneMarking,
    /// Traffic sign
    TrafficSign,
    /// Point of interest marker
    PoiMarker,
    /// Vehicle model
    Vehicle,
    /// Terrain
    Terrain,
    /// Custom model
    Custom(String),
}

/// Scene graph managing all renderable objects.
#[derive(Debug)]
pub struct SceneGraph {
    objects: Vec<SceneObject>,
    camera: Camera,
    config: SceneConfig,
}

impl SceneGraph {
    /// Create a new scene graph with default settings.
    pub fn new(camera: Camera) -> Self {
        Self {
            objects: Vec::new(),
            camera,
            config: SceneConfig::default(),
        }
    }

    /// Add an object to the scene.
    pub fn add_object(&mut self, obj: SceneObject) {
        self.objects.push(obj);
    }

    /// Remove an object by ID.
    pub fn remove_object(&mut self, id: &str) -> bool {
        let len_before = self.objects.len();
        self.objects.retain(|o| o.id != id);
        self.objects.len() < len_before
    }

    /// Get an object by ID.
    pub fn get_object(&self, id: &str) -> Option<&SceneObject> {
        self.objects.iter().find(|o| o.id == id)
    }

    /// Get a mutable object by ID.
    pub fn get_object_mut(&mut self, id: &str) -> Option<&mut SceneObject> {
        self.objects.iter_mut().find(|o| o.id == id)
    }

    /// Update the camera.
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    /// Get the current camera.
    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    /// Update scene configuration.
    pub fn set_config(&mut self, config: SceneConfig) {
        self.config = config;
    }

    /// Get the current configuration.
    pub fn config(&self) -> &SceneConfig {
        &self.config
    }

    /// Get all visible objects.
    pub fn visible_objects(&self) -> Vec<&SceneObject> {
        self.objects.iter().filter(|o| o.visible).collect()
    }

    /// Total object count.
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    /// Clear all objects from the scene.
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_operations() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        assert!((a.length() - 1.0).abs() < 1e-10);
        assert!((a.dot(&b)).abs() < 1e-10);

        let cross = a.cross(&b);
        assert!((cross.z - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_vec3_normalize() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        let n = v.normalize();
        assert!((n.length() - 1.0).abs() < 1e-10);

        let zero = Vec3::zero();
        let nz = zero.normalize();
        assert!((nz.length()).abs() < 1e-10);
    }

    #[test]
    fn test_camera_directions() {
        let cam = Camera::perspective(Vec3::new(0.0, 0.0, 10.0), Vec3::new(0.0, 0.0, 0.0), 60.0);
        let fwd = cam.forward();
        assert!((fwd.z - (-1.0)).abs() < 1e-10);

        let right = cam.right();
        assert!(right.length() > 0.9);
    }

    #[test]
    fn test_scene_graph_add_remove() {
        let cam = Camera::perspective(Vec3::zero(), Vec3::new(0.0, 0.0, -1.0), 60.0);
        let mut scene = SceneGraph::new(cam);

        let obj = SceneObject {
            id: "road_1".to_string(),
            object_type: SceneObjectType::Road,
            position: Vec3::zero(),
            scale: Vec3::new(1.0, 1.0, 1.0),
            rotation_deg: Vec3::zero(),
            visible: true,
            opacity: 1.0,
        };
        scene.add_object(obj);
        assert_eq!(scene.object_count(), 1);

        assert!(scene.remove_object("road_1"));
        assert_eq!(scene.object_count(), 0);
    }

    #[test]
    fn test_scene_visible_objects() {
        let cam = Camera::perspective(Vec3::zero(), Vec3::new(0.0, 0.0, -1.0), 60.0);
        let mut scene = SceneGraph::new(cam);

        scene.add_object(SceneObject {
            id: "visible".to_string(),
            object_type: SceneObjectType::NavArrow,
            position: Vec3::zero(),
            scale: Vec3::new(1.0, 1.0, 1.0),
            rotation_deg: Vec3::zero(),
            visible: true,
            opacity: 1.0,
        });
        scene.add_object(SceneObject {
            id: "hidden".to_string(),
            object_type: SceneObjectType::Building,
            position: Vec3::zero(),
            scale: Vec3::new(1.0, 1.0, 1.0),
            rotation_deg: Vec3::zero(),
            visible: false,
            opacity: 1.0,
        });

        assert_eq!(scene.visible_objects().len(), 1);
        assert_eq!(scene.object_count(), 2);
    }

    #[test]
    fn test_render_quality_ordering() {
        assert!(RenderQuality::Ultra > RenderQuality::High);
        assert!(RenderQuality::High > RenderQuality::Medium);
        assert!(RenderQuality::Medium > RenderQuality::Low);
    }
}
