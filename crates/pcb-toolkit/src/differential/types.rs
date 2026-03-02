use serde::{Deserialize, Serialize};

/// Differential pair protocol presets with target Zdiff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiffProtocol {
    Ddr2ClkDqs,
    Ddr3ClkDqs,
    Ddr4ClkDqs,
    Usb2x,
    Usb3x,
    Lvds,
    Hdmi,
    Sata,
    Ethernet,
    DisplayPort,
    DisplayPortEaglelake,
    DisplayPortCalpella,
    PcieGen1,
    PcieGen2,
    PcieGen3,
    PcieGen4,
    SsrxSstx,
    Custom,
}

impl DiffProtocol {
    /// Target differential impedance (Ohms) for this protocol.
    pub fn target_zdiff(self) -> Option<f64> {
        match self {
            Self::Ddr2ClkDqs | Self::Ddr3ClkDqs => Some(100.0),
            Self::Ddr4ClkDqs => Some(80.0),
            Self::Usb2x => Some(90.0),
            Self::Usb3x => Some(90.0),
            Self::Lvds | Self::Hdmi | Self::Sata | Self::Ethernet => Some(100.0),
            Self::DisplayPort => Some(100.0),
            Self::DisplayPortEaglelake | Self::DisplayPortCalpella => Some(85.0),
            Self::PcieGen1 => Some(100.0),
            Self::PcieGen2 | Self::PcieGen3 | Self::PcieGen4 => Some(85.0),
            Self::SsrxSstx => Some(85.0),
            Self::Custom => None,
        }
    }
}

/// Result of a differential impedance calculation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifferentialResult {
    /// Differential impedance (Ohms). Zdiff = 2 × Zodd.
    pub zdiff: f64,
    /// Single-ended impedance (Ohms).
    pub zo: f64,
    /// Odd-mode impedance (Ohms).
    pub zodd: f64,
    /// Even-mode impedance (Ohms).
    pub zeven: f64,
    /// Backward coupling coefficient Kb (unterminated).
    pub kb: f64,
    /// Unterminated coupling coefficient in dB.
    pub kb_db: f64,
    /// Terminated backward coupling coefficient.
    pub kb_term: f64,
    /// Terminated coupling coefficient in dB.
    pub kb_term_db: f64,
}

/// Compute terminated backward coupling coefficient from unterminated Kb.
///
/// Formula: `Kb_term = (1 − √(1 − Kb²)) / Kb`
///
/// Saturn validation (PDF p.11): Kb=0.4041 → Kb_term=0.2111
pub fn kb_terminated(kb: f64) -> f64 {
    if kb.abs() < 1e-15 {
        return 0.0;
    }
    (1.0 - (1.0 - kb * kb).sqrt()) / kb
}
