//! Link budget — satellite link quality estimation and management.
//!
//! Calculates link margins, estimates data rates, and recommends
//! transmission parameters based on current satellite visibility.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

/// Satellite link type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkType {
    /// Iridium (LEO, global coverage).
    Iridium,
    /// Globalstar (LEO).
    Globalstar,
    /// Inmarsat (GEO, high latitude gaps).
    Inmarsat,
    /// Thuraya (GEO, regional).
    Thuraya,
    /// Starlink (LEO, broadband).
    Starlink,
    /// Generic LEO.
    GenericLeo,
    /// Generic GEO.
    GenericGeo,
}

/// Quality of the satellite link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LinkQuality {
    /// No link available.
    NoLink,
    /// Marginal — high error rate expected.
    Marginal,
    /// Usable — moderate error rate.
    Usable,
    /// Good — low error rate.
    Good,
    /// Excellent — optimal conditions.
    Excellent,
}

/// Satellite link parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkParameters {
    pub link_type: LinkType,
    pub elevation_deg: f64,
    pub azimuth_deg: f64,
    /// Signal-to-noise ratio in dB.
    pub snr_db: f64,
    /// Free-space path loss in dB.
    pub path_loss_db: f64,
    /// Estimated data rate in bits per second.
    pub estimated_bps: u64,
    /// Link margin in dB (positive = good).
    pub link_margin_db: f64,
    pub quality: LinkQuality,
    pub timestamp: DateTime<Utc>,
}

/// Link budget calculator and manager.
pub struct LinkBudget {
    /// Current link parameters.
    current: Option<LinkParameters>,
    /// Link history.
    history: Vec<LinkParameters>,
    max_history: usize,
    /// Minimum elevation angle for usable link (degrees).
    min_elevation_deg: f64,
    /// Minimum SNR for usable link (dB).
    min_snr_db: f64,
    /// Total link assessments.
    total_assessments: u64,
}

impl LinkBudget {
    pub fn new(min_elevation_deg: f64, min_snr_db: f64) -> Self {
        Self {
            current: None,
            history: Vec::new(),
            max_history: 100,
            min_elevation_deg,
            min_snr_db,
            total_assessments: 0,
        }
    }

    /// Assess the current link quality given satellite parameters.
    pub fn assess(
        &mut self,
        link_type: LinkType,
        elevation_deg: f64,
        azimuth_deg: f64,
        snr_db: f64,
    ) -> LinkParameters {
        self.total_assessments += 1;

        // Calculate free-space path loss (simplified).
        let path_loss = self.estimate_path_loss(link_type, elevation_deg);

        // Estimate data rate based on link type and SNR.
        let base_rate = self.base_rate(link_type);
        let snr_factor = if snr_db > 20.0 {
            1.0
        } else if snr_db > 10.0 {
            0.5
        } else if snr_db > 5.0 {
            0.2
        } else {
            0.05
        };
        let estimated_bps = (base_rate as f64 * snr_factor) as u64;

        // Link margin = SNR - minimum required SNR.
        let link_margin = snr_db - self.min_snr_db;

        // Determine quality.
        let quality = if elevation_deg < self.min_elevation_deg || snr_db < self.min_snr_db {
            LinkQuality::NoLink
        } else if link_margin < 3.0 {
            LinkQuality::Marginal
        } else if link_margin < 6.0 {
            LinkQuality::Usable
        } else if link_margin < 12.0 {
            LinkQuality::Good
        } else {
            LinkQuality::Excellent
        };

        let params = LinkParameters {
            link_type,
            elevation_deg,
            azimuth_deg,
            snr_db,
            path_loss_db: path_loss,
            estimated_bps,
            link_margin_db: link_margin,
            quality,
            timestamp: Utc::now(),
        };

        match quality {
            LinkQuality::NoLink => warn!(
                elevation = elevation_deg,
                snr = snr_db,
                "no satellite link available"
            ),
            LinkQuality::Marginal => debug!("marginal satellite link"),
            _ => debug!(quality = ?quality, bps = estimated_bps, "satellite link assessed"),
        }

        if self.history.len() >= self.max_history {
            self.history.remove(0);
        }
        self.history.push(params.clone());
        self.current = Some(params.clone());

        params
    }

    fn estimate_path_loss(&self, link_type: LinkType, elevation_deg: f64) -> f64 {
        // Simplified path loss model.
        let altitude_km: f64 = match link_type {
            LinkType::Iridium
            | LinkType::Globalstar
            | LinkType::Starlink
            | LinkType::GenericLeo => 780.0,
            LinkType::Inmarsat | LinkType::Thuraya | LinkType::GenericGeo => 35_786.0,
        };

        // Slant range based on elevation.
        let elev_rad = elevation_deg.to_radians();
        let earth_r: f64 = 6371.0;
        let slant_range = ((earth_r + altitude_km).powi(2) - (earth_r * elev_rad.cos()).powi(2))
            .sqrt()
            - earth_r * elev_rad.sin();

        // Free-space path loss at 1.6 GHz (L-band).
        let freq_ghz: f64 = 1.6;
        20.0 * (slant_range.log10()) + 20.0 * (freq_ghz.log10()) + 92.45
    }

    fn base_rate(&self, link_type: LinkType) -> u64 {
        match link_type {
            LinkType::Iridium => 2400,         // 2.4 kbps
            LinkType::Globalstar => 9600,      // 9.6 kbps
            LinkType::Inmarsat => 64_000,      // 64 kbps
            LinkType::Thuraya => 15_000,       // 15 kbps
            LinkType::Starlink => 100_000_000, // 100 Mbps
            LinkType::GenericLeo => 9600,
            LinkType::GenericGeo => 64_000,
        }
    }

    /// Get current link quality.
    pub fn current_quality(&self) -> LinkQuality {
        self.current
            .as_ref()
            .map(|p| p.quality)
            .unwrap_or(LinkQuality::NoLink)
    }

    /// Get current estimated data rate.
    pub fn current_bps(&self) -> u64 {
        self.current.as_ref().map(|p| p.estimated_bps).unwrap_or(0)
    }

    /// Get current link parameters.
    pub fn current(&self) -> Option<&LinkParameters> {
        self.current.as_ref()
    }

    /// Is the link currently usable?
    pub fn is_usable(&self) -> bool {
        self.current_quality() >= LinkQuality::Usable
    }

    /// Get link history.
    pub fn history(&self) -> &[LinkParameters] {
        &self.history
    }

    /// Average SNR over recent history.
    pub fn avg_snr(&self) -> f64 {
        if self.history.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.history.iter().map(|h| h.snr_db).sum();
        sum / self.history.len() as f64
    }

    /// Total assessments performed.
    pub fn total_assessments(&self) -> u64 {
        self.total_assessments
    }

    /// Get the recommended transmission mode based on current quality.
    pub fn recommended_mode(&self) -> &str {
        match self.current_quality() {
            LinkQuality::NoLink => "store",
            LinkQuality::Marginal => "minimal",
            LinkQuality::Usable => "compressed",
            LinkQuality::Good => "standard",
            LinkQuality::Excellent => "full",
        }
    }
}

impl Default for LinkBudget {
    fn default() -> Self {
        Self::new(10.0, 5.0) // 10° min elevation, 5 dB min SNR
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assess_good_link() {
        let mut link = LinkBudget::default();
        let params = link.assess(LinkType::Iridium, 45.0, 180.0, 15.0);

        assert!(params.quality >= LinkQuality::Good);
        assert!(params.estimated_bps > 0);
        assert!(params.link_margin_db > 0.0);
    }

    #[test]
    fn assess_no_link_low_elevation() {
        let mut link = LinkBudget::new(10.0, 5.0);
        let params = link.assess(LinkType::Iridium, 5.0, 90.0, 15.0);
        assert_eq!(params.quality, LinkQuality::NoLink);
    }

    #[test]
    fn assess_no_link_low_snr() {
        let mut link = LinkBudget::new(10.0, 5.0);
        let params = link.assess(LinkType::Iridium, 45.0, 90.0, 3.0);
        assert_eq!(params.quality, LinkQuality::NoLink);
    }

    #[test]
    fn marginal_link_low_margin() {
        let mut link = LinkBudget::new(10.0, 5.0);
        let params = link.assess(LinkType::Iridium, 15.0, 90.0, 7.0);
        // Link margin = 7.0 - 5.0 = 2.0 dB < 3.0 → Marginal.
        assert_eq!(params.quality, LinkQuality::Marginal);
    }

    #[test]
    fn starlink_high_rate() {
        let mut link = LinkBudget::default();
        let params = link.assess(LinkType::Starlink, 60.0, 0.0, 25.0);
        assert!(params.estimated_bps > 1_000_000); // Should be very high.
        assert_eq!(params.quality, LinkQuality::Excellent);
    }

    #[test]
    fn is_usable() {
        let mut link = LinkBudget::default();
        assert!(!link.is_usable()); // No assessment yet.

        link.assess(LinkType::Iridium, 45.0, 180.0, 15.0);
        assert!(link.is_usable());
    }

    #[test]
    fn avg_snr_calculation() {
        let mut link = LinkBudget::default();
        link.assess(LinkType::Iridium, 45.0, 0.0, 10.0);
        link.assess(LinkType::Iridium, 45.0, 0.0, 20.0);
        assert!((link.avg_snr() - 15.0).abs() < 0.01);
    }

    #[test]
    fn recommended_mode_based_on_quality() {
        let mut link = LinkBudget::default();
        assert_eq!(link.recommended_mode(), "store");

        link.assess(LinkType::Iridium, 45.0, 0.0, 15.0);
        assert_ne!(link.recommended_mode(), "store");
    }

    #[test]
    fn history_capped() {
        let mut link = LinkBudget::default();
        link.max_history = 5;
        for i in 0..10 {
            link.assess(LinkType::Iridium, 45.0, 0.0, 10.0 + i as f64);
        }
        assert_eq!(link.history().len(), 5);
    }

    #[test]
    fn geo_vs_leo_path_loss() {
        let mut link = LinkBudget::default();
        let leo = link.assess(LinkType::Iridium, 45.0, 0.0, 15.0);
        let geo = link.assess(LinkType::Inmarsat, 45.0, 0.0, 15.0);
        // GEO should have higher path loss.
        assert!(geo.path_loss_db > leo.path_loss_db);
    }

    #[test]
    fn current_bps_no_assessment() {
        let link = LinkBudget::default();
        assert_eq!(link.current_bps(), 0);
    }
}
