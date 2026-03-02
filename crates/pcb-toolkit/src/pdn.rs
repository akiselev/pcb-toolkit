//! PDN (Power Delivery Network) impedance calculator.
//!
//! Computes target PDN impedance, plane capacitance, and capacitive reactance.

use serde::{Deserialize, Serialize};

use crate::CalcError;

/// Inputs for a PDN impedance calculation.
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
    /// Frequency (MHz). 0 = DC only (skip Xc calculation).
    pub freq_mhz: f64,
}

/// Result of a PDN impedance calculation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PdnResult {
    /// Target PDN impedance (Ω).
    pub z_target_ohms: f64,
    /// Total plane capacitance (pF).
    pub c_plane_pf: f64,
    /// Capacitive reactance (Ω), None if DC.
    pub xc_ohms: Option<f64>,
}

/// Parallel-plate capacitance constant (ε₀ in pF, imperial units).
const EPSILON_0_IMPERIAL: f64 = 0.225;

/// Calculate PDN impedance.
pub fn calculate(input: &PdnInput) -> Result<PdnResult, CalcError> {
    if input.v_supply <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "v_supply",
            value: input.v_supply,
            expected: "> 0",
        });
    }
    if input.i_max <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "i_max",
            value: input.i_max,
            expected: "> 0",
        });
    }
    if input.i_step_pct <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "i_step_pct",
            value: input.i_step_pct,
            expected: "> 0",
        });
    }
    if input.v_ripple_pct <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "v_ripple_pct",
            value: input.v_ripple_pct,
            expected: "> 0",
        });
    }
    if input.area_sq_in <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "area_sq_in",
            value: input.area_sq_in,
            expected: "> 0",
        });
    }
    if input.er <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "er",
            value: input.er,
            expected: "> 0",
        });
    }
    if input.d_mils <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "d_mils",
            value: input.d_mils,
            expected: "> 0",
        });
    }

    // Z_target = (V * V_ripple%) / (I * I_step%)
    let z_target_ohms = (input.v_supply * input.v_ripple_pct / 100.0)
        / (input.i_max * input.i_step_pct / 100.0);

    // C_plane = 0.225 * Er * A / (d_mils/1000)
    let d_inches = input.d_mils / 1000.0;
    let c_plane_pf = EPSILON_0_IMPERIAL * input.er * input.area_sq_in / d_inches;

    // Xc = 1 / (2*pi*f*C) — skip if DC
    let xc_ohms = if input.freq_mhz > 0.0 {
        let f_hz = input.freq_mhz * 1e6;
        let c_farads = c_plane_pf * 1e-12;
        Some(1.0 / (2.0 * std::f64::consts::PI * f_hz * c_farads))
    } else {
        None
    };

    Ok(PdnResult { z_target_ohms, c_plane_pf, xc_ohms })
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn help_pdf_test_vector() {
        let result = calculate(&PdnInput {
            v_supply: 5.0,
            i_max: 2.0,
            i_step_pct: 50.0,
            v_ripple_pct: 5.0,
            area_sq_in: 5.0,
            er: 4.6,
            d_mils: 2.0,
            freq_mhz: 1.0,
        })
        .unwrap();

        assert_relative_eq!(result.z_target_ohms, 0.25, epsilon = 1e-10);
        assert_relative_eq!(result.c_plane_pf, 2587.5, epsilon = 1e-10);
        assert_relative_eq!(result.xc_ohms.unwrap(), 61.5092, epsilon = 0.001);
    }

    #[test]
    fn dc_mode_no_xc() {
        let result = calculate(&PdnInput {
            v_supply: 3.3,
            i_max: 1.0,
            i_step_pct: 100.0,
            v_ripple_pct: 10.0,
            area_sq_in: 2.0,
            er: 4.6,
            d_mils: 4.0,
            freq_mhz: 0.0,
        })
        .unwrap();

        assert_relative_eq!(result.z_target_ohms, 0.33, epsilon = 1e-10);
        assert!(result.xc_ohms.is_none());
    }

    #[test]
    fn invalid_inputs() {
        let base = PdnInput {
            v_supply: 5.0,
            i_max: 2.0,
            i_step_pct: 50.0,
            v_ripple_pct: 5.0,
            area_sq_in: 5.0,
            er: 4.6,
            d_mils: 2.0,
            freq_mhz: 1.0,
        };
        assert!(calculate(&PdnInput { v_supply: 0.0, ..base }).is_err());
        assert!(calculate(&PdnInput { i_step_pct: 0.0, ..base }).is_err());
        assert!(calculate(&PdnInput { d_mils: -1.0, ..base }).is_err());
    }
}
