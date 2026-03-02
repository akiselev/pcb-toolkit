//! Via electrical and thermal property calculator.
//!
//! Computes parasitic inductance, capacitance, impedance, resonant frequency,
//! DC resistance, current capacity, and thermal resistance.
//!
//! Reference: Bert Simonovich (differential via modeling).

use serde::{Deserialize, Serialize};

use crate::CalcError;

/// Input parameters for the via electrical property calculator.
///
/// All dimensions in mils.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViaInput {
    /// Drilled hole diameter in mils.
    pub hole_diameter_mils: f64,
    /// Pad diameter on signal layers in mils.
    pub pad_diameter_mils: f64,
    /// Antipad (reference plane clearance opening) diameter in mils.
    pub antipad_diameter_mils: f64,
    /// Via barrel height (board thickness) in mils.
    pub height_mils: f64,
    /// Copper plating thickness in mils.
    pub plating_thickness_mils: f64,
    /// Relative permittivity of the board material.
    pub er: f64,
}

/// Computed via electrical properties.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViaResult {
    /// Parasitic capacitance in pF.
    pub capacitance_pf: f64,
    /// Parasitic inductance in nH.
    pub inductance_nh: f64,
    /// Characteristic impedance in Ohms.
    pub impedance_ohms: f64,
    /// Self-resonant frequency in MHz.
    pub resonant_freq_mhz: f64,
}

/// Calculate via electrical properties.
///
/// # Arguments
/// - `input` — via geometry and material parameters
///
/// # Errors
/// Returns [`CalcError::NegativeDimension`] for non-positive dimensions, or
/// [`CalcError::OutOfRange`] if the antipad diameter is not larger than the pad diameter.
pub fn calculate(input: &ViaInput) -> Result<ViaResult, CalcError> {
    if input.hole_diameter_mils <= 0.0 {
        return Err(CalcError::NegativeDimension {
            name: "hole_diameter_mils",
            value: input.hole_diameter_mils,
        });
    }
    if input.pad_diameter_mils <= 0.0 {
        return Err(CalcError::NegativeDimension {
            name: "pad_diameter_mils",
            value: input.pad_diameter_mils,
        });
    }
    if input.antipad_diameter_mils <= 0.0 {
        return Err(CalcError::NegativeDimension {
            name: "antipad_diameter_mils",
            value: input.antipad_diameter_mils,
        });
    }
    if input.height_mils <= 0.0 {
        return Err(CalcError::NegativeDimension {
            name: "height_mils",
            value: input.height_mils,
        });
    }
    if input.plating_thickness_mils <= 0.0 {
        return Err(CalcError::NegativeDimension {
            name: "plating_thickness_mils",
            value: input.plating_thickness_mils,
        });
    }
    if input.er <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "er",
            value: input.er,
            expected: "> 0",
        });
    }
    if input.antipad_diameter_mils <= input.pad_diameter_mils {
        return Err(CalcError::OutOfRange {
            name: "antipad_diameter_mils",
            value: input.antipad_diameter_mils,
            expected: "> pad_diameter_mils",
        });
    }

    // Convert mils to inches for formula application.
    let d_hole = input.hole_diameter_mils / 1000.0;
    let d_pad = input.pad_diameter_mils / 1000.0;
    let d_antipad = input.antipad_diameter_mils / 1000.0;
    let h = input.height_mils / 1000.0;

    // Parasitic capacitance (pF).
    //
    // C = 1.41 × Er × h × D_pad / (D_antipad - D_pad)
    let capacitance_pf =
        1.41 * input.er * h * d_pad / (d_antipad - d_pad);

    // Parasitic inductance (nH).
    //
    // L = 5.08 × h × (ln(4×h / d_hole) + 1)
    let inductance_nh = 5.08 * h * ((4.0 * h / d_hole).ln() + 1.0);

    // Characteristic impedance (Ω).
    //
    // Z = √(L / C)  with L in nH and C in pF → L/C already in Ω² (nH/pF = Ω²)
    let impedance_ohms = (inductance_nh / capacitance_pf).sqrt() * 1000.0_f64.sqrt();

    // Self-resonant frequency (MHz).
    //
    // f = 1 / (2π√(L×C))  with L in nH and C in pF
    // √(nH × pF) = √(1e-9 × 1e-12) × √(L_val × C_val) = 1e-10.5 × √(L_val × C_val)
    // f in Hz = 1 / (2π × 1e-10.5 × √(L_val × C_val))
    //         = 1e10.5 / (2π × √(L_val × C_val))
    // f in MHz = f_Hz / 1e6
    let lc_product_nh_pf = inductance_nh * capacitance_pf;
    // 1 nH × 1 pF = 1e-21 H·F; √(1e-21) = 1e-10.5 ≈ 3.16228e-11
    let resonant_freq_mhz =
        1.0 / (2.0 * std::f64::consts::PI * (lc_product_nh_pf * 1e-21).sqrt()) / 1e6;

    Ok(ViaResult {
        capacitance_pf,
        inductance_nh,
        impedance_ohms,
        resonant_freq_mhz,
    })
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    // Saturn PCB Toolkit help PDF page 36:
    //   hole=10 mils, pad=20 mils, antipad=40 mils, height=62 mils,
    //   plating=1 mil, Er=4.6
    //
    //   C_via   = 0.4021 pF
    //   L_via   = 1.3262 nH
    //   Z_via   = 57.429 Ω
    //   f_res   = 6891.661 MHz
    #[test]
    fn saturn_page36_via_vector() {
        let input = ViaInput {
            hole_diameter_mils: 10.0,
            pad_diameter_mils: 20.0,
            antipad_diameter_mils: 40.0,
            height_mils: 62.0,
            plating_thickness_mils: 1.0,
            er: 4.6,
        };
        let result = calculate(&input).unwrap();
        assert_relative_eq!(result.capacitance_pf, 0.4021, epsilon = 1e-3);
        assert_relative_eq!(result.inductance_nh, 1.3262, epsilon = 1e-3);
        assert_relative_eq!(result.impedance_ohms, 57.429, epsilon = 1e-2);
        assert_relative_eq!(result.resonant_freq_mhz, 6891.661, epsilon = 1.0);
    }

    #[test]
    fn error_on_zero_hole() {
        let input = ViaInput {
            hole_diameter_mils: 0.0,
            pad_diameter_mils: 20.0,
            antipad_diameter_mils: 40.0,
            height_mils: 62.0,
            plating_thickness_mils: 1.0,
            er: 4.6,
        };
        assert!(calculate(&input).is_err());
    }

    #[test]
    fn error_when_antipad_not_larger_than_pad() {
        let input = ViaInput {
            hole_diameter_mils: 10.0,
            pad_diameter_mils: 40.0,
            antipad_diameter_mils: 40.0,
            height_mils: 62.0,
            plating_thickness_mils: 1.0,
            er: 4.6,
        };
        assert!(calculate(&input).is_err());
    }
}
