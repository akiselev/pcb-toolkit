//! Via lumped-element property calculator.
//!
//! Computes the lumped parasitic capacitance, inductance, the impedance
//! `√(L/C)`, the LC natural frequency, and the DC barrel resistance.
//!
//! * Capacitance: `C = 1.41·εr·h·D_pad / (D_antipad − D_pad)` pF with h and
//!   diameters in inches (Howard Johnson, *High-Speed Digital Design*, §7.6).
//! * Inductance: `L = 5.08·h·(ln(4h/d) + 1)` nH, h and d in inches
//!   (Johnson §7.6; the partial inductance of a slender wire).
//! * Resistance: `R = ρ·h / A_barrel` with the plated annulus
//!   `A = π/4·(d² − (d − 2t)²)`.
//!
//! `√(L/C)` and `1/(2π√(LC))` are lumped-element figures of merit, not a
//! frequency-dependent via transition model or a stub-resonance model.

use serde::{Deserialize, Serialize};

use crate::model::{ModelInfo, ModelStatus};
use crate::{CalcError, constants, validate};

/// Model description.
pub const MODEL: ModelInfo = ModelInfo {
    name: "Johnson lumped via C/L with plated-barrel DC resistance",
    status: ModelStatus::Compatibility,
    reference: "Johnson & Graham, High-Speed Digital Design (1993) §7.6; Saturn PCB Toolkit help p.36",
    validity: "Slender barrel h ≥ d_hole; D_antipad > D_pad > d_hole; 2·plating < d_hole; lumped (electrically short) via only",
};

/// Input parameters for the via property calculator. All dimensions in mils.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViaInput {
    /// Drilled hole diameter in mils (outer diameter of the plated barrel).
    pub hole_diameter_mils: f64,
    /// Pad diameter on signal layers in mils.
    pub pad_diameter_mils: f64,
    /// Antipad (reference plane clearance opening) diameter in mils.
    pub antipad_diameter_mils: f64,
    /// Via barrel height (board thickness) in mils.
    pub height_mils: f64,
    /// Copper plating thickness in mils (used for the barrel resistance).
    pub plating_thickness_mils: f64,
    /// Relative permittivity of the board material.
    pub er: f64,
}

/// Computed via properties.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViaResult {
    /// Parasitic capacitance in pF.
    pub capacitance_pf: f64,
    /// Parasitic inductance in nH.
    pub inductance_nh: f64,
    /// Lumped impedance √(L/C) in Ohms.
    pub impedance_ohms: f64,
    /// LC natural frequency 1/(2π√(LC)) in MHz.
    pub resonant_freq_mhz: f64,
    /// DC resistance of the plated barrel in milliohms (copper at 20 °C).
    pub resistance_mohm: f64,
}

/// Calculate via properties.
///
/// # Errors
/// Returns an error for non-finite or non-positive dimensions, an antipad
/// not larger than the pad, a pad not larger than the hole, plating that
/// would close the hole, or a barrel shorter than the hole diameter (outside
/// the slender-wire inductance model).
pub fn calculate(input: &ViaInput) -> Result<ViaResult, CalcError> {
    let d_hole = validate::positive("hole_diameter_mils", input.hole_diameter_mils)?;
    let d_pad = validate::positive("pad_diameter_mils", input.pad_diameter_mils)?;
    let d_antipad = validate::positive("antipad_diameter_mils", input.antipad_diameter_mils)?;
    let h = validate::positive("height_mils", input.height_mils)?;
    let t = validate::positive("plating_thickness_mils", input.plating_thickness_mils)?;
    let er = validate::er(input.er)?;

    if d_pad <= d_hole {
        return Err(CalcError::OutOfRange {
            name: "pad_diameter_mils",
            value: d_pad,
            expected: "> hole_diameter_mils",
        });
    }
    if d_antipad <= d_pad {
        return Err(CalcError::OutOfRange {
            name: "antipad_diameter_mils",
            value: d_antipad,
            expected: "> pad_diameter_mils",
        });
    }
    if 2.0 * t >= d_hole {
        return Err(CalcError::OutOfRange {
            name: "plating_thickness_mils",
            value: t,
            expected: "< hole_diameter_mils / 2",
        });
    }
    if h < d_hole {
        return Err(CalcError::OutOfRange {
            name: "height_mils",
            value: h,
            expected: ">= hole_diameter_mils (slender-barrel inductance model)",
        });
    }

    // Inches for the Johnson expressions.
    let (d_hole_in, d_pad_in, d_antipad_in, h_in) = (
        d_hole / 1000.0,
        d_pad / 1000.0,
        d_antipad / 1000.0,
        h / 1000.0,
    );

    let capacitance_pf = 1.41 * er * h_in * d_pad_in / (d_antipad_in - d_pad_in);
    let inductance_nh = 5.08 * h_in * ((4.0 * h_in / d_hole_in).ln() + 1.0);

    // nH/pF = 1e3 Ω².
    let impedance_ohms = (inductance_nh / capacitance_pf * 1e3).sqrt();
    // 1 nH × 1 pF = 1e-21 H·F.
    let resonant_freq_mhz =
        1.0 / (2.0 * std::f64::consts::PI * (inductance_nh * capacitance_pf * 1e-21).sqrt()) / 1e6;

    let barrel_area_sq_mils =
        std::f64::consts::FRAC_PI_4 * (d_hole * d_hole - (d_hole - 2.0 * t).powi(2));
    let resistance_mohm = constants::COPPER_RESISTIVITY_OHM_MIL * h / barrel_area_sq_mils * 1e3;

    Ok(ViaResult {
        capacitance_pf: validate::finite_result("capacitance_pf", capacitance_pf)?,
        inductance_nh: validate::finite_result("inductance_nh", inductance_nh)?,
        impedance_ohms: validate::finite_result("impedance_ohms", impedance_ohms)?,
        resonant_freq_mhz: validate::finite_result("resonant_freq_mhz", resonant_freq_mhz)?,
        resistance_mohm: validate::finite_result("resistance_mohm", resistance_mohm)?,
    })
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    fn saturn() -> ViaInput {
        ViaInput {
            hole_diameter_mils: 10.0,
            pad_diameter_mils: 20.0,
            antipad_diameter_mils: 40.0,
            height_mils: 62.0,
            plating_thickness_mils: 1.0,
            er: 4.6,
        }
    }

    // Saturn PCB Toolkit help PDF page 36.
    #[test]
    fn saturn_page36_via_vector() {
        let result = calculate(&saturn()).unwrap();
        assert_relative_eq!(result.capacitance_pf, 0.4021, epsilon = 1e-3);
        assert_relative_eq!(result.inductance_nh, 1.3262, epsilon = 1e-3);
        assert_relative_eq!(result.impedance_ohms, 57.429, epsilon = 1e-2);
        assert_relative_eq!(result.resonant_freq_mhz, 6891.661, epsilon = 1.0);
    }

    #[test]
    fn plating_affects_barrel_resistance() {
        let thin = calculate(&saturn()).unwrap();
        let thick = calculate(&ViaInput {
            plating_thickness_mils: 2.0,
            ..saturn()
        })
        .unwrap();
        assert!(thick.resistance_mohm < thin.resistance_mohm);
        // ρ·h/A: 6.787e-4 × 62 / (π/4 (100 − 64)) Ω = 1.488 mΩ.
        assert_relative_eq!(thin.resistance_mohm, 1.4883, max_relative = 2e-3);
    }

    #[test]
    fn short_barrel_is_rejected_not_negative() {
        let r = calculate(&ViaInput {
            height_mils: 0.5,
            ..saturn()
        });
        assert!(r.is_err());
        let ok = calculate(&ViaInput {
            height_mils: 10.0,
            ..saturn()
        })
        .unwrap();
        assert!(ok.inductance_nh > 0.0 && ok.impedance_ohms > 0.0);
    }

    #[test]
    fn geometry_checks() {
        assert!(
            calculate(&ViaInput {
                hole_diameter_mils: 0.0,
                ..saturn()
            })
            .is_err()
        );
        assert!(
            calculate(&ViaInput {
                pad_diameter_mils: 40.0,
                ..saturn()
            })
            .is_err()
        );
        assert!(
            calculate(&ViaInput {
                pad_diameter_mils: 10.0,
                ..saturn()
            })
            .is_err()
        );
        assert!(
            calculate(&ViaInput {
                plating_thickness_mils: 5.0,
                ..saturn()
            })
            .is_err()
        );
        assert!(
            calculate(&ViaInput {
                er: f64::NAN,
                ..saturn()
            })
            .is_err()
        );
        assert!(
            calculate(&ViaInput {
                er: 0.5,
                ..saturn()
            })
            .is_err()
        );
    }
}
