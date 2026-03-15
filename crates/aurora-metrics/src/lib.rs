//! AURORA NAV — Metrics Collection & Prometheus Export
//!
//! Lightweight, zero-dependency Prometheus-compatible metrics for the
//! navigation pipeline. Counters, gauges, and histograms are collected
//! in-process and exported via a `/metrics` endpoint in Prometheus
//! text exposition format.

pub mod collector;
pub mod export;
pub mod registry;
