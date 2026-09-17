//! PDN (Power Delivery Network) target impedance and plane capacitance.
//!
//! Computes the target impedance `Z = ΔV/ΔI`, the parallel-plate
//! capacitance of a power/ground plane pair, and that capacitance's
//! reactance at one frequency. This is a first-order budget: it does not
//! include package or via inductance, capacitor ESR/ESL, plane resonances
//! or regulator response.

use serde::{Deserialize, Serialize};

use crate::model::{ModelInfo, ModelStatus};
use crate::{CalcError, validate};

/// Model description.
pub const MODEL: ModelInfo = ModelInfo {
    name: "PDN target impedance and parallel-plate plane capacitance",
    status: ModelStatus::Validated,
    reference: "Z_target = ΔV/ΔI; C = ε₀·εr·A/d with ε₀ = 0.225 pF/in; Saturn PCB Toolkit help",
    validity: "Exact relations; plane capacitance ignores fringing and via/decoupling inductance; single-frequency reactance only",
};

/// Inputs for a PDN impedance calculation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PdnInput {
    /// DC supply voltage (V).
    pub v_supply: f64,
    /// Maximum load current (A).
    pub i_max: f64,
    /// Transient current step as percentage of i_max (%).
    pub i_step_pct: f64,
    /// Allowable voltage ripple as percentage of v_supply (%).
    pub v_ripple_pct: f64,
    /// Area of power/ground plane (sq.in).
    pub area_sq_in: f64,
    /// Substrate relative permittivity.
    pub er: f64,
    /// Distance between power and ground planes (mils).
    pub d_mils: f64,
    /// Frequency (MHz). Exactly 0 = DC (reactance omitted); negative is an error.
    pub freq_mhz: f64,
}

/// Result of a PDN impedance calculation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PdnResult {
    /// Target PDN impedance (Ω).
    pub z_target_ohms: f64,
    /// Total plane capacitance (pF).
    pub c_plane_pf: f64,
    /// Capacitive reactance (Ω); `None` when the frequency is 0 (DC).
    pub xc_ohms: Option<f64>,
}

/// Parallel-plate capacitance constant, ε₀ in pF/in (0.2249 pF/in, rounded
/// as Saturn does; 0.05% effect).
const EPSILON_0_PF_PER_IN: f64 = 0.225;

/// Calculate PDN impedance.
pub fn calculate(input: &PdnInput) -> Result<PdnResult, CalcError> {
    validate::positive_quantity("v_supply", input.v_supply)?;
    validate::positive_quantity("i_max", input.i_max)?;
    validate::positive_quantity("i_step_pct", input.i_step_pct)?;
    validate::positive_quantity("v_ripple_pct", input.v_ripple_pct)?;
    validate::positive_quantity("area_sq_in", input.area_sq_in)?;
    validate::er(input.er)?;
    validate::positive("d_mils", input.d_mils)?;
    validate::non_negative("freq_mhz", input.freq_mhz)?;

    let z_target_ohms =
        (input.v_supply * input.v_ripple_pct / 100.0) / (input.i_max * input.i_step_pct / 100.0);
    let c_plane_pf = EPSILON_0_PF_PER_IN * input.er * input.area_sq_in / (input.d_mils / 1000.0);

    let xc_ohms = if input.freq_mhz > 0.0 {
        let f_hz = input.freq_mhz * 1e6;
        Some(validate::finite_result(
            "xc_ohms",
            1.0 / (2.0 * std::f64::consts::PI * f_hz * c_plane_pf * 1e-12),
        )?)
    } else {
        None
    };

    Ok(PdnResult {
        z_target_ohms: validate::finite_result("z_target_ohms", z_target_ohms)?,
        c_plane_pf: validate::finite_result("c_plane_pf", c_plane_pf)?,
        xc_ohms,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn base() -> PdnInput {
        PdnInput {
            v_supply: 5.0,
            i_max: 2.0,
            i_step_pct: 50.0,
            v_ripple_pct: 5.0,
            area_sq_in: 5.0,
            er: 4.6,
            d_mils: 2.0,
            freq_mhz: 1.0,
        }
    }

    #[test]
    fn help_pdf_test_vector() {
        let result = calculate(&base()).unwrap();
        assert_relative_eq!(result.z_target_ohms, 0.25, epsilon = 1e-10);
        assert_relative_eq!(result.c_plane_pf, 2587.5, epsilon = 1e-10);
        assert_relative_eq!(result.xc_ohms.unwrap(), 61.5092, epsilon = 0.001);
    }

    #[test]
    fn dc_mode_no_xc() {
        let result = calculate(&PdnInput {
            freq_mhz: 0.0,
            ..base()
        })
        .unwrap();
        assert!(result.xc_ohms.is_none());
    }

    #[test]
    fn invalid_inputs() {
        assert!(
            calculate(&PdnInput {
                v_supply: 0.0,
                ..base()
            })
            .is_err()
        );
        assert!(
            calculate(&PdnInput {
                i_step_pct: 0.0,
                ..base()
            })
            .is_err()
        );
        assert!(
            calculate(&PdnInput {
                d_mils: -1.0,
                ..base()
            })
            .is_err()
        );
        // Negative frequency is an error, not silently DC (audit A25).
        assert!(
            calculate(&PdnInput {
                freq_mhz: -1.0,
                ..base()
            })
            .is_err()
        );
        assert!(
            calculate(&PdnInput {
                freq_mhz: f64::NAN,
                ..base()
            })
            .is_err()
        );
        assert!(calculate(&PdnInput { er: 0.5, ..base() }).is_err());
    }
}
