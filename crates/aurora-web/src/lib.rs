//! AURORA NAV — Visual UI Layer
//!
//! Provides a rich interactive web frontend with:
//! - Interactive map with OpenStreetMap tiles
//! - Real-time GPS tracking and position display
//! - Route visualization with turn-by-turn directions
//! - Traffic overlay with congestion heatmap
//! - Satellite constellation sky view
//! - Integrity and continuity status panels
//! - Emergency mode indicators
//! - Fleet tracking dashboard
//! - Smart city signal status
//! - Performance metrics graphs

pub mod assets;
pub mod dashboard;

pub use assets::MAIN_HTML;
pub use dashboard::DashboardData;
