//! Standalone crosstalk calculator.
//!
//! Note: This calculator is marked "Unsupported" in the original Saturn PCB
//! Toolkit due to formula accuracy concerns. Included for completeness.
//!
//! Estimates backward crosstalk (NEXT) using the standard coupled-line model:
//!
//! Kb = 1 / (4 × (1 + (S/H)²))
//! Lsat = rise_time × v_prop / 2
//! NEXT = Kb × min(coupled_length / Lsat, 1.0)
//!
//! # Known Limitations
//! The standard Kb formula does not match Saturn's test vector (-2.23 dB / 3.87 V).
//! Saturn likely uses a different formula or additional correction factors.
//!
//! # TODO
//! - Match Saturn formula exactly
//! - Stripline variant
//! - Forward crosstalk (FEXT) estimation

use serde::{Deserialize, Serialize};

use crate::CalcError;
use crate::constants::SPEED_OF_LIGHT_IN_NS;
use crate::impedance::common;

/// Inputs for crosstalk estimation.
pub struct CrosstalkInput {
    /// Signal rise time (ns).
    pub rise_time_ns: f64,
    /// Signal voltage (V).
    pub voltage: f64,
    /// Coupled (parallel) trace length (mils).
    pub coupled_length_mils: f64,
    /// Edge-to-edge spacing between traces (mils).
    pub spacing_mils: f64,
    /// Dielectric height — trace to ground plane (mils).
    pub height_mils: f64,
    /// Substrate relative permittivity.
    pub er: f64,
    /// Trace width (mils). Used for Er_eff calculation.
    pub trace_width_mils: f64,
}

/// Result of a crosstalk estimation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrosstalkResult {
    /// Backward crosstalk coefficient Kb (dimensionless, 0–0.25).
    pub kb: f64,
    /// Crosstalk in dB (20 × log10(NEXT)).
    pub crosstalk_db: f64,
    /// Coupled voltage (V) = NEXT × voltage.
    pub coupled_voltage: f64,
    /// NEXT coefficient (dimensionless, 0–Kb).
    pub next_coefficient: f64,
    /// Saturation length (mils).
    pub lsat_mils: f64,
}

/// Estimate backward crosstalk (NEXT) between parallel microstrip traces.
pub fn calculate(input: &CrosstalkInput) -> Result<CrosstalkResult, CalcError> {
    let CrosstalkInput {
        rise_time_ns,
        voltage,
        coupled_length_mils,
        spacing_mils,
        height_mils,
        er,
        trace_width_mils,
    } = *input;

    if rise_time_ns <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "rise_time_ns",
            value: rise_time_ns,
            expected: "> 0",
        });
    }
    if voltage <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "voltage",
            value: voltage,
            expected: "> 0",
        });
    }
    if coupled_length_mils <= 0.0 {
        return Err(CalcError::NegativeDimension {
            name: "coupled_length_mils",
            value: coupled_length_mils,
        });
    }
    if spacing_mils <= 0.0 {
        return Err(CalcError::NegativeDimension {
            name: "spacing_mils",
            value: spacing_mils,
        });
    }
    if height_mils <= 0.0 {
        return Err(CalcError::NegativeDimension {
            name: "height_mils",
            value: height_mils,
        });
    }
    if er < 1.0 {
        return Err(CalcError::OutOfRange {
            name: "er",
            value: er,
            expected: ">= 1.0",
        });
    }
    if trace_width_mils <= 0.0 {
        return Err(CalcError::NegativeDimension {
            name: "trace_width_mils",
            value: trace_width_mils,
        });
    }

    // Backward crosstalk coefficient
    let s_over_h = spacing_mils / height_mils;
    let kb = 1.0 / (4.0 * (1.0 + s_over_h * s_over_h));

    // Propagation velocity from Er_eff
    let u = trace_width_mils / height_mils;
    let er_eff = common::er_eff_static(u, er);
    // v_prop in in/ns
    let v_prop = SPEED_OF_LIGHT_IN_NS / er_eff.sqrt();
    // Convert to mils/ns: 1 in = 1000 mils
    let v_prop_mils_ns = v_prop * 1000.0;

    // Saturation length (mils)
    let lsat_mils = rise_time_ns * v_prop_mils_ns / 2.0;

    // NEXT coefficient (saturates at Kb)
    let length_ratio = coupled_length_mils / lsat_mils;
    let next_coefficient = kb * length_ratio.min(1.0);

    // Coupled voltage
    let coupled_voltage = next_coefficient * voltage;

    // Crosstalk in dB
    let crosstalk_db = 20.0 * next_coefficient.log10();

    Ok(CrosstalkResult {
        kb,
        crosstalk_db,
        coupled_voltage,
        next_coefficient,
        lsat_mils,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_crosstalk() {
        let result = calculate(&CrosstalkInput {
            rise_time_ns: 1.0,
            voltage: 5.0,
            coupled_length_mils: 1000.0,
            spacing_mils: 10.0,
            height_mils: 5.0,
            er: 4.6,
            trace_width_mils: 10.0,
        })
        .unwrap();

        // Kb should be in range (0, 0.25]
        assert!(result.kb > 0.0 && result.kb <= 0.25, "Kb = {}", result.kb);
        // NEXT should be in range (0, Kb]
        assert!(
            result.next_coefficient > 0.0 && result.next_coefficient <= result.kb,
            "NEXT = {}",
            result.next_coefficient
        );
        assert!(result.coupled_voltage > 0.0 && result.coupled_voltage < 5.0);
        assert!(result.crosstalk_db < 0.0); // dB is negative for NEXT < 1
        assert!(result.lsat_mils > 0.0);
    }

    #[test]
    fn wider_spacing_less_crosstalk() {
        let close = calculate(&CrosstalkInput {
            rise_time_ns: 1.0,
            voltage: 5.0,
            coupled_length_mils: 1000.0,
            spacing_mils: 5.0,
            height_mils: 5.0,
            er: 4.6,
            trace_width_mils: 10.0,
        })
        .unwrap();

        let far = calculate(&CrosstalkInput {
            rise_time_ns: 1.0,
            voltage: 5.0,
            coupled_length_mils: 1000.0,
            spacing_mils: 20.0,
            height_mils: 5.0,
            er: 4.6,
            trace_width_mils: 10.0,
        })
        .unwrap();

        assert!(
            close.kb > far.kb,
            "close Kb {} should be > far Kb {}",
            close.kb,
            far.kb
        );
    }

    #[test]
    fn kb_max_at_zero_spacing_limit() {
        // As S→0, Kb→0.25
        let result = calculate(&CrosstalkInput {
            rise_time_ns: 1.0,
            voltage: 5.0,
            coupled_length_mils: 1000.0,
            spacing_mils: 0.01, // very small spacing
            height_mils: 5.0,
            er: 4.6,
            trace_width_mils: 10.0,
        })
        .unwrap();

        assert!(result.kb > 0.24, "Kb at near-zero spacing = {}", result.kb);
    }

    #[test]
    fn short_coupled_length_reduces_next() {
        let long = calculate(&CrosstalkInput {
            rise_time_ns: 1.0,
            voltage: 5.0,
            coupled_length_mils: 10000.0,
            spacing_mils: 10.0,
            height_mils: 5.0,
            er: 4.6,
            trace_width_mils: 10.0,
        })
        .unwrap();

        let short = calculate(&CrosstalkInput {
            rise_time_ns: 1.0,
            voltage: 5.0,
            coupled_length_mils: 100.0,
            spacing_mils: 10.0,
            height_mils: 5.0,
            er: 4.6,
            trace_width_mils: 10.0,
        })
        .unwrap();

        assert!(
            short.next_coefficient <= long.next_coefficient,
            "short NEXT {} should be <= long NEXT {}",
            short.next_coefficient,
            long.next_coefficient
        );
    }

    #[test]
    fn rejects_negative_spacing() {
        let result = calculate(&CrosstalkInput {
            rise_time_ns: 1.0,
            voltage: 5.0,
            coupled_length_mils: 1000.0,
            spacing_mils: -1.0,
            height_mils: 5.0,
            er: 4.6,
            trace_width_mils: 10.0,
        });
        assert!(result.is_err());
    }

    #[test]
    fn rejects_zero_rise_time() {
        let result = calculate(&CrosstalkInput {
            rise_time_ns: 0.0,
            voltage: 5.0,
            coupled_length_mils: 1000.0,
            spacing_mils: 10.0,
            height_mils: 5.0,
            er: 4.6,
            trace_width_mils: 10.0,
        });
        assert!(result.is_err());
    }
}
