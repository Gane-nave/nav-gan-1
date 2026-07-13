//! AURORA Probabilistic Routing — multi-objective optimization with uncertainty propagation.
//!
//! Provides probabilistic route ranking that considers time, distance, risk,
//! network stability, cognitive load, energy, and emissions as objectives,
//! propagating uncertainty through the routing pipeline.

pub mod explanation;
pub mod optimizer;

pub use explanation::ExplanationGraph;
pub use optimizer::ProbabilisticRouter;
