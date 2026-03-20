//! Map matching engine — snaps raw GPS coordinates to road network segments.
//!
//! Uses Hidden Markov Model (HMM) approach with transition probabilities
//! based on road network topology and emission probabilities based on
//! GPS measurement noise.

pub mod matcher;
pub use matcher::MapMatcher;
