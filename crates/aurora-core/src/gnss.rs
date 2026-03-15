//! GNSS domain types — constellations, measurements, satellite state.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{EcefPosition, EntityId, NavigationSource};

// ---------------------------------------------------------------------------
// Constellations & signals
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Constellation {
    Gps,
    Galileo,
    Glonass,
    BeiDou,
}

impl std::fmt::Display for Constellation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gps => write!(f, "GPS"),
            Self::Galileo => write!(f, "Galileo"),
            Self::Glonass => write!(f, "GLONASS"),
            Self::BeiDou => write!(f, "BeiDou"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FrequencyBand {
    L1,
    L2,
    L5,
    E1,
    E5a,
    E5b,
    B1,
    B2a,
    B2b,
    B3,
    G1,
    G2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SignalType {
    GpsL1CA,
    GpsL2C,
    GpsL5,
    GalileoE1,
    GalileoE5a,
    GalileoE5b,
    GlonassL1OF,
    GlonassL2OF,
    BeidouB1I,
    BeidouB1C,
    BeidouB2a,
    BeidouB2b,
    BeidouB3I,
}

impl SignalType {
    pub fn constellation(&self) -> Constellation {
        match self {
            Self::GpsL1CA | Self::GpsL2C | Self::GpsL5 => Constellation::Gps,
            Self::GalileoE1 | Self::GalileoE5a | Self::GalileoE5b => Constellation::Galileo,
            Self::GlonassL1OF | Self::GlonassL2OF => Constellation::Glonass,
            Self::BeidouB1I | Self::BeidouB1C | Self::BeidouB2a | Self::BeidouB2b | Self::BeidouB3I => {
                Constellation::BeiDou
            }
        }
    }

    pub fn frequency_band(&self) -> FrequencyBand {
        match self {
            Self::GpsL1CA => FrequencyBand::L1,
            Self::GpsL2C => FrequencyBand::L2,
            Self::GpsL5 => FrequencyBand::L5,
            Self::GalileoE1 => FrequencyBand::E1,
            Self::GalileoE5a => FrequencyBand::E5a,
            Self::GalileoE5b => FrequencyBand::E5b,
            Self::GlonassL1OF => FrequencyBand::G1,
            Self::GlonassL2OF => FrequencyBand::G2,
            Self::BeidouB1I | Self::BeidouB1C => FrequencyBand::B1,
            Self::BeidouB2a => FrequencyBand::B2a,
            Self::BeidouB2b => FrequencyBand::B2b,
            Self::BeidouB3I => FrequencyBand::B3,
        }
    }

    pub fn to_nav_source(&self) -> NavigationSource {
        match self {
            Self::GpsL1CA => NavigationSource::GpsL1,
            Self::GpsL2C => NavigationSource::GpsL1,
            Self::GpsL5 => NavigationSource::GpsL5,
            Self::GalileoE1 => NavigationSource::GalileoE1,
            Self::GalileoE5a | Self::GalileoE5b => NavigationSource::GalileoE5a,
            Self::GlonassL1OF => NavigationSource::GlonassL1,
            Self::GlonassL2OF => NavigationSource::GlonassL2,
            Self::BeidouB1I | Self::BeidouB1C => NavigationSource::BeidouB1,
            Self::BeidouB2a | Self::BeidouB2b | Self::BeidouB3I => NavigationSource::BeidouB2a,
        }
    }
}

// ---------------------------------------------------------------------------
// Satellite measurement (raw observables)
// ---------------------------------------------------------------------------

/// Unique identifier for a satellite within its constellation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SatelliteId {
    pub constellation: Constellation,
    /// PRN (GPS/BeiDou), SVID (Galileo), or slot number (GLONASS).
    pub prn: u8,
}

impl std::fmt::Display for SatelliteId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = match self.constellation {
            Constellation::Gps => "G",
            Constellation::Galileo => "E",
            Constellation::Glonass => "R",
            Constellation::BeiDou => "C",
        };
        write!(f, "{}{:02}", prefix, self.prn)
    }
}

/// Raw measurement from one satellite on one signal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SatelliteMeasurement {
    pub id: EntityId,
    pub satellite: SatelliteId,
    pub signal: SignalType,
    pub timestamp: DateTime<Utc>,
    /// Pseudorange in metres.
    pub pseudorange_m: f64,
    /// Carrier phase in cycles (if available).
    pub carrier_phase_cycles: Option<f64>,
    /// Doppler shift in Hz.
    pub doppler_hz: Option<f64>,
    /// Carrier-to-noise density in dB-Hz.
    pub cn0_dbhz: f64,
    /// Satellite health flag from navigation message.
    pub healthy: bool,
    /// Satellite ECEF position (from ephemeris).
    pub satellite_position: Option<EcefPosition>,
    /// Elevation angle in degrees.
    pub elevation_deg: Option<f64>,
    /// Azimuth angle in degrees.
    pub azimuth_deg: Option<f64>,
}

/// Aggregated GNSS measurement epoch — all satellites at one instant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GnssMeasurement {
    pub id: EntityId,
    pub timestamp: DateTime<Utc>,
    pub measurements: Vec<SatelliteMeasurement>,
    pub receiver_clock_bias_ns: Option<f64>,
    pub receiver_clock_drift_nps: Option<f64>,
}

// ---------------------------------------------------------------------------
// Constellation-level state
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstellationHealth {
    Nominal,
    Degraded,
    Unavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstellationState {
    pub constellation: Constellation,
    pub health: ConstellationHealth,
    pub visible_count: u32,
    pub used_count: u32,
    pub avg_cn0_dbhz: f64,
    pub pdop: Option<f64>,
    pub hdop: Option<f64>,
    pub vdop: Option<f64>,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// PVT solution (per-constellation or combined)
// ---------------------------------------------------------------------------

/// Position-Velocity-Time solution from one or more constellations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PvtSolution {
    pub timestamp: DateTime<Utc>,
    pub position: EcefPosition,
    pub velocity_ecef: [f64; 3],
    pub clock_bias_ns: f64,
    pub clock_drift_nps: f64,
    pub satellites_used: Vec<SatelliteId>,
    pub constellations: Vec<Constellation>,
    pub pdop: f64,
    pub hdop: f64,
    pub vdop: f64,
    pub residuals: Vec<f64>,
    pub quality: PvtQuality,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PvtQuality {
    /// Autonomous fix, no corrections.
    Autonomous,
    /// Differential (SBAS/DGNSS).
    Differential,
    /// RTK float.
    RtkFloat,
    /// RTK fixed.
    RtkFixed,
    /// PPP converging.
    PppConverging,
    /// PPP converged.
    PppConverged,
    /// Dead reckoning only.
    DeadReckoning,
    /// No fix.
    NoFix,
}

// ---------------------------------------------------------------------------
// Correction state
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorrectionSource {
    Sbas,
    Ppp,
    Rtk,
    Nrtk,
    InternetCorrection,
    CachedFallback,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrectionState {
    pub source: CorrectionSource,
    pub active: bool,
    pub age_seconds: f64,
    pub confidence: f64,
    pub validity_window_s: f64,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Spoofing / Jamming alerts
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatType {
    Spoofing,
    Jamming,
    Replay,
    Multipath,
    Nlos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatSeverity {
    Suspected,
    Confirmed,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAlert {
    pub id: EntityId,
    pub timestamp: DateTime<Utc>,
    pub threat_type: ThreatType,
    pub severity: ThreatSeverity,
    pub affected_constellation: Option<Constellation>,
    pub affected_satellites: Vec<SatelliteId>,
    pub reason: String,
    pub mitigated: bool,
}

// ---------------------------------------------------------------------------
// Source trust record
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceTrustRecord {
    pub source: NavigationSource,
    pub trust_score: f64,
    pub excluded: bool,
    pub exclusion_reason: Option<String>,
    pub last_validated_at: DateTime<Utc>,
    pub history: Vec<TrustHistoryEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustHistoryEntry {
    pub timestamp: DateTime<Utc>,
    pub score: f64,
    pub event: String,
}
