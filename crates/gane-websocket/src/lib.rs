//! WebSocket & Real-Time Communication engine for G.A.N.E NAV.
//!
//! Provides connection management, message framing, channel/room abstractions,
//! and heartbeat-based liveness detection.

pub mod channel;
pub mod connection;
pub mod frame;
pub mod heartbeat;
