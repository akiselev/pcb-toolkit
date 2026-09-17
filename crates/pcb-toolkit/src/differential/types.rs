use serde::{Deserialize, Serialize};

use crate::{CalcError, validate};

/// Differential pair protocol presets with target Zdiff.
///
/// These are the nominal targets Saturn PCB Toolkit lists. Real specifications
/// state tolerances and, for some protocols, platform-specific targets
/// (e.g. 85 Ω vs 100 Ω PCIe implementations); consult the governing
/// specification for the design at hand.
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
    /// Nominal target differential impedance (Ohms) for this protocol, as
    /// listed by Saturn PCB Toolkit.
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DifferentialResult {
    /// Differential impedance (Ohms). Zdiff = 2 × Zodd.
    pub zdiff: f64,
    /// Single-ended impedance of one trace with the other absent (Ohms).
    pub zo: f64,
    /// Odd-mode impedance (Ohms).
    pub zodd: f64,
    /// Even-mode impedance (Ohms).
    pub zeven: f64,
    /// Backward coupling coefficient Kb = (Zeven − Zodd)/(Zeven + Zodd).
    pub kb: f64,
    /// Kb in dB. `None` when the coupling is exactly zero (−∞ dB).
    pub kb_db: Option<f64>,
    /// Terminated backward coupling coefficient.
    pub kb_term: f64,
    /// Terminated coupling in dB. `None` when the coupling is exactly zero.
    pub kb_term_db: Option<f64>,
}

/// Compute the terminated backward coupling coefficient from the
/// unterminated Kb.
///
/// `Kb_term = (1 − √(1 − Kb²)) / Kb`, evaluated in the rationalised form
/// `Kb / (1 + √(1 − Kb²))` so that small couplings do not cancel to zero.
///
/// Saturn validation (PDF p.11): Kb = 0.4041 → Kb_term = 0.2111.
///
/// # Errors
/// Returns an error unless `0 ≤ Kb ≤ 1`.
pub fn kb_terminated(kb: f64) -> Result<f64, CalcError> {
    validate::in_range("kb", kb, 0.0, 1.0, "0 ..= 1")?;
    Ok(kb / (1.0 + (1.0 - kb * kb).sqrt()))
}

/// Coupling in dB; `None` for zero coupling.
fn to_db(k: f64) -> Option<f64> {
    if k > 0.0 {
        Some(20.0 * k.log10())
    } else {
        None
    }
}

/// Assemble a validated result from the single-ended, odd- and even-mode impedances.
pub(crate) fn build(zo: f64, zodd: f64, zeven: f64) -> Result<DifferentialResult, CalcError> {
    let zo = validate::finite_result("zo", zo)?;
    let zodd = validate::finite_result("zodd", zodd)?;
    let zeven = validate::finite_result("zeven", zeven)?;
    if zo <= 0.0 || zodd <= 0.0 || zeven <= 0.0 {
        return Err(CalcError::NonFiniteResult {
            name: "impedance (non-positive)",
        });
    }
    if zeven < zodd {
        return Err(CalcError::NonFiniteResult {
            name: "zeven < zodd (model out of domain)",
        });
    }
    let kb = (zeven - zodd) / (zeven + zodd);
    let kb_term = kb_terminated(kb)?;
    Ok(DifferentialResult {
        zdiff: 2.0 * zodd,
        zo,
        zodd,
        zeven,
        kb,
        kb_db: to_db(kb),
        kb_term,
        kb_term_db: to_db(kb_term),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn saturn_kb_term_vector() {
        assert_relative_eq!(kb_terminated(0.4041).unwrap(), 0.2111, max_relative = 2e-3);
    }

    #[test]
    fn weak_coupling_does_not_cancel() {
        assert_relative_eq!(kb_terminated(1e-9).unwrap(), 5e-10, max_relative = 1e-6);
        assert_eq!(kb_terminated(0.0).unwrap(), 0.0);
    }

    #[test]
    fn kb_domain() {
        assert!(kb_terminated(1.0).is_ok());
        assert!(kb_terminated(1.0001).is_err());
        assert!(kb_terminated(-0.1).is_err());
        assert!(kb_terminated(f64::NAN).is_err());
    }

    #[test]
    fn build_rejects_bad_modes() {
        assert!(build(50.0, 40.0, 30.0).is_err());
        assert!(build(50.0, f64::NAN, 60.0).is_err());
        let r = build(50.0, 50.0, 50.0).unwrap();
        assert_eq!(r.kb, 0.0);
        assert!(r.kb_db.is_none());
    }
}
