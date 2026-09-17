//! Edge-coupled external (surface microstrip) differential pair.
//!
//! IPC-2141A / National Semiconductor AN-905 closed forms:
//!
//! ```text
//! Zo    = 87/√(εr + 1.41) · ln(5.98·H / (0.8·W + T))
//! Zodd  = Zo · (1 − 0.48·exp(−0.96·S/H))
//! Zeven = Zo² / Zodd
//! ```
//!
//! This is the formula pair Saturn PCB Toolkit uses for this topology; the
//! Saturn help PDF example (p. 11) is reproduced exactly. IPC-2141A states
//! the single-ended form is accurate to about ±5% for 0.1 < W/H < 2.0 and
//! 1 < εr < 15; the coupling term is an empirical fit whose stated range is
//! 0.2 ≤ S/H ≤ 3.0. `Zeven = Zo²/Zodd` is the weak-coupling approximation
//! `Zodd·Zeven ≈ Zo²`, not an exact relation.

use super::types::{self, DifferentialResult};
use crate::model::{ModelInfo, ModelStatus};
use crate::{CalcError, validate};

/// Model description.
pub const MODEL: ModelInfo = ModelInfo {
    name: "IPC-2141A surface microstrip differential pair",
    status: ModelStatus::Compatibility,
    reference: "IPC-2141A §4; National Semiconductor AN-905; Saturn PCB Toolkit help p.11",
    validity: "0.1 < W/H < 2.0, 1 < εr < 15, 0.2 ≤ S/H ≤ 3.0; single-ended ±5%, coupling term empirical (Zeven from Zo²/Zodd)",
};

/// Inputs for edge-coupled external (surface) differential pair.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdgeCoupledExternalInput {
    /// Conductor width (mils).
    pub width: f64,
    /// Gap between traces (mils).
    pub spacing: f64,
    /// Dielectric height to ground plane (mils).
    pub height: f64,
    /// Conductor thickness (mils).
    pub thickness: f64,
    /// Substrate relative permittivity.
    pub er: f64,
}

/// IPC-2141A single-ended surface microstrip impedance.
pub fn ipc2141_zo(width: f64, height: f64, thickness: f64, er: f64) -> Result<f64, CalcError> {
    validate::positive("width", width)?;
    validate::positive("height", height)?;
    validate::non_negative("thickness", thickness)?;
    validate::er(er)?;
    let zo = (87.0 / (er + 1.41_f64).sqrt()) * (5.98 * height / (0.8 * width + thickness)).ln();
    let zo = validate::finite_result("zo", zo)?;
    if zo <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "W/H",
            value: width / height,
            expected: "IPC-2141A form valid only while 5.98·H > 0.8·W + T (roughly W/H < 7)",
        });
    }
    Ok(zo)
}

/// IPC-2141A odd-mode reduction factor for edge-coupled microstrip.
pub fn ipc2141_odd_factor(spacing: f64, height: f64) -> f64 {
    1.0 - 0.48 * (-0.96 * spacing / height).exp()
}

/// Compute differential impedance for an edge-coupled external (surface) pair.
pub fn calculate(input: &EdgeCoupledExternalInput) -> Result<DifferentialResult, CalcError> {
    let EdgeCoupledExternalInput {
        width,
        spacing,
        height,
        thickness,
        er,
    } = *input;
    validate::positive("spacing", spacing)?;
    let zo = ipc2141_zo(width, height, thickness, er)?;
    let zodd = zo * ipc2141_odd_factor(spacing, height);
    let zeven = zo * zo / zodd;
    types::build(zo, zodd, zeven)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn input(
        width: f64,
        spacing: f64,
        height: f64,
        thickness: f64,
        er: f64,
    ) -> EdgeCoupledExternalInput {
        EdgeCoupledExternalInput {
            width,
            spacing,
            height,
            thickness,
            er,
        }
    }

    /// Saturn PDF page 11: W=10, S=5, H=15, Er=4.6, T=2.10
    #[test]
    fn saturn_pdf_page11() {
        let result = calculate(&input(10.0, 5.0, 15.0, 2.10, 4.6)).unwrap();
        assert_relative_eq!(result.zo, 77.504, max_relative = 0.002);
        assert_relative_eq!(result.zodd, 50.490, max_relative = 0.002);
        assert_relative_eq!(result.zeven, 118.971, max_relative = 0.002);
        assert_relative_eq!(result.zdiff, 100.979, max_relative = 0.002);
        assert_relative_eq!(result.kb, 0.4041, max_relative = 0.002);
        assert_relative_eq!(result.kb_db.unwrap(), -7.870, max_relative = 0.002);
        assert_relative_eq!(result.kb_term, 0.2111, max_relative = 0.005);
        assert_relative_eq!(result.kb_term_db.unwrap(), -13.512, max_relative = 0.005);
    }

    #[test]
    fn wider_spacing_reduces_coupling() {
        let close = calculate(&input(10.0, 5.0, 15.0, 2.10, 4.6)).unwrap();
        let far = calculate(&input(10.0, 20.0, 15.0, 2.10, 4.6)).unwrap();
        assert!(far.kb < close.kb);
    }

    #[test]
    fn higher_er_gives_lower_z0() {
        let low_er = calculate(&input(10.0, 5.0, 15.0, 2.10, 3.0)).unwrap();
        let high_er = calculate(&input(10.0, 5.0, 15.0, 2.10, 4.6)).unwrap();
        assert!(high_er.zo < low_er.zo);
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert!(calculate(&input(-1.0, 5.0, 15.0, 2.10, 4.6)).is_err());
        assert!(calculate(&input(10.0, 5.0, 15.0, 2.10, 0.5)).is_err());
        assert!(calculate(&input(f64::NAN, 5.0, 15.0, 2.10, 4.6)).is_err());
        // Very wide trace drives the log negative: rejected, not returned.
        assert!(calculate(&input(200.0, 5.0, 15.0, 2.10, 4.6)).is_err());
    }
}
