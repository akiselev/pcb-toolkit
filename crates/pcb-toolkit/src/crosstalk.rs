//! Backward crosstalk (NEXT) **estimate** for parallel microstrip traces.
//!
//! This calculator is marked "unsupported" in the original Saturn PCB
//! Toolkit and its output here does not reproduce Saturn's example
//! (−2.23 dB / 3.87 V). It is a textbook rule-of-thumb, exposed as an
//! order-of-magnitude estimate only:
//!
//! ```text
//! Kb   = 1 / (4 · (1 + (S/H)²))          (saturated backward coupling)
//! Lsat = t_rise · v_prop / 2             (saturation length)
//! NEXT = Kb · min(L_coupled / Lsat, 1)
//! ```
//!
//! The coupling heuristic depends only on S/H: trace width, thickness,
//! dielectric constant (beyond the velocity) and termination are not
//! represented in `Kb`. For a quantitative answer use the coupled-line
//! `Kb` from the differential-pair calculators or a field solver.

use serde::{Deserialize, Serialize};

use crate::constants::SPEED_OF_LIGHT_IN_NS;
use crate::impedance::common;
use crate::model::{ModelInfo, ModelStatus};
use crate::{CalcError, validate};

/// Model description.
pub const MODEL: ModelInfo = ModelInfo {
    name: "NEXT rule-of-thumb Kb = 1/(4(1 + (S/H)²)) with saturation length",
    status: ModelStatus::Experimental,
    reference: "Johnson & Graham, High-Speed Digital Design §5; marked unsupported in Saturn PCB Toolkit",
    validity: "Order-of-magnitude estimate only; Kb ignores W, T, εr and terminations; no stated error bound",
};

/// Inputs for crosstalk estimation.
#[derive(Debug, Clone, Copy, PartialEq)]
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
    /// Trace width (mils). Used only for the Er_eff / velocity estimate.
    pub trace_width_mils: f64,
}

/// Result of a crosstalk estimation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrosstalkResult {
    /// Saturated backward crosstalk coefficient Kb (dimensionless, 0–0.25).
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

    validate::positive_quantity("rise_time_ns", rise_time_ns)?;
    validate::positive_quantity("voltage", voltage)?;
    validate::positive("coupled_length_mils", coupled_length_mils)?;
    validate::positive("spacing_mils", spacing_mils)?;
    validate::positive("height_mils", height_mils)?;
    validate::er(er)?;
    validate::positive("trace_width_mils", trace_width_mils)?;

    let s_over_h = spacing_mils / height_mils;
    let kb = 1.0 / (4.0 * (1.0 + s_over_h * s_over_h));

    let er_eff = common::er_eff_static(trace_width_mils / height_mils, er);
    let v_prop_mils_ns = SPEED_OF_LIGHT_IN_NS / er_eff.sqrt() * 1000.0;
    let lsat_mils = rise_time_ns * v_prop_mils_ns / 2.0;

    let next_coefficient = kb * (coupled_length_mils / lsat_mils).min(1.0);
    let coupled_voltage = next_coefficient * voltage;
    let crosstalk_db = 20.0 * next_coefficient.log10();

    Ok(CrosstalkResult {
        kb: validate::finite_result("kb", kb)?,
        crosstalk_db: validate::finite_result("crosstalk_db", crosstalk_db)?,
        coupled_voltage: validate::finite_result("coupled_voltage", coupled_voltage)?,
        next_coefficient: validate::finite_result("next_coefficient", next_coefficient)?,
        lsat_mils: validate::finite_result("lsat_mils", lsat_mils)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> CrosstalkInput {
        CrosstalkInput {
            rise_time_ns: 1.0,
            voltage: 5.0,
            coupled_length_mils: 1000.0,
            spacing_mils: 10.0,
            height_mils: 5.0,
            er: 4.6,
            trace_width_mils: 10.0,
        }
    }

    #[test]
    fn basic_crosstalk() {
        let result = calculate(&base()).unwrap();
        assert!(result.kb > 0.0 && result.kb <= 0.25);
        assert!(result.next_coefficient > 0.0 && result.next_coefficient <= result.kb);
        assert!(result.coupled_voltage > 0.0 && result.coupled_voltage < 5.0);
        assert!(result.crosstalk_db < 0.0);
        assert!(result.lsat_mils > 0.0);
    }

    #[test]
    fn wider_spacing_less_crosstalk() {
        let close = calculate(&CrosstalkInput {
            spacing_mils: 5.0,
            ..base()
        })
        .unwrap();
        let far = calculate(&CrosstalkInput {
            spacing_mils: 20.0,
            ..base()
        })
        .unwrap();
        assert!(close.kb > far.kb);
    }

    #[test]
    fn kb_max_at_zero_spacing_limit() {
        let result = calculate(&CrosstalkInput {
            spacing_mils: 0.01,
            ..base()
        })
        .unwrap();
        assert!(result.kb > 0.24);
    }

    #[test]
    fn short_coupled_length_reduces_next() {
        let long = calculate(&CrosstalkInput {
            coupled_length_mils: 10000.0,
            ..base()
        })
        .unwrap();
        let short = calculate(&CrosstalkInput {
            coupled_length_mils: 100.0,
            ..base()
        })
        .unwrap();
        assert!(short.next_coefficient <= long.next_coefficient);
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert!(
            calculate(&CrosstalkInput {
                spacing_mils: -1.0,
                ..base()
            })
            .is_err()
        );
        assert!(
            calculate(&CrosstalkInput {
                rise_time_ns: 0.0,
                ..base()
            })
            .is_err()
        );
        assert!(
            calculate(&CrosstalkInput {
                voltage: f64::NAN,
                ..base()
            })
            .is_err()
        );
    }
}
