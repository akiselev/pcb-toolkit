use serde::{Deserialize, Serialize};

/// Differential pair protocol presets with target Zdiff.
///
/// Data extracted from Saturn PCB Toolkit binary at offset 0x783121.
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
    /// Backward coupling coefficient Kb.
    pub kb: f64,
    /// Coupling coefficient in dB.
    pub kb_db: f64,
}
