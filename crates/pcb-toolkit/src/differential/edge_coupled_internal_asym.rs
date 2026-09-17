//! Edge-coupled internal asymmetric (offset stripline) differential pair.
//!
//! The pair sits at gaps `H1` and `H2` from the two ground planes. Each
//! modal impedance is obtained by combining the two half-spaces: the
//! capacitance to each plane is taken as one half of the capacitance of a
//! centered coupled pair with twice that gap, so
//!
//! ```text
//! Z = 2·Z(2H1 + T)·Z(2H2 + T) / (Z(2H1 + T) + Z(2H2 + T))
//! ```
//!
//! for the single-ended, even- and odd-mode impedances separately, with the
//! centered values from [`super::edge_coupled_internal_sym`] (Cohn 1955).
//! The construction is exact when `H1 = H2`, symmetric under swapping the
//! planes, and responds to the offset at fixed total spacing. Fringing
//! coupling between the two half-spaces is neglected (Wadell §3.4.2 offset
//! stripline approximation).

use super::edge_coupled_internal_sym::coupled_modes;
use super::types::{self, DifferentialResult};
use crate::impedance::stripline::z0_offset;
use crate::model::{ModelInfo, ModelStatus};
use crate::{CalcError, validate};

/// Model description.
pub const MODEL: ModelInfo = ModelInfo {
    name: "Offset coupled stripline: half-space combination of Cohn 1955 modes",
    status: ModelStatus::Compatibility,
    reference: "Cohn 1955; Wadell 1991 §3.4.2 (offset strip half-space combination)",
    validity: "T < min(H1, H2); exact for H1 = H2; estimated ±5% for offsets up to H1/H2 = 4",
};

/// Inputs for edge-coupled internal asymmetric (offset) differential pair.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdgeCoupledInternalAsymInput {
    /// Conductor width (mils).
    pub width: f64,
    /// Gap between traces (mils).
    pub spacing: f64,
    /// Dielectric gap from the trace face to the top ground plane (mils).
    pub height1: f64,
    /// Dielectric gap from the trace face to the bottom ground plane (mils).
    pub height2: f64,
    /// Conductor thickness (mils).
    pub thickness: f64,
    /// Substrate relative permittivity.
    pub er: f64,
}

fn parallel(a: f64, b: f64) -> f64 {
    2.0 * a * b / (a + b)
}

/// Compute differential impedance for an edge-coupled offset stripline pair.
pub fn calculate(input: &EdgeCoupledInternalAsymInput) -> Result<DifferentialResult, CalcError> {
    let EdgeCoupledInternalAsymInput {
        width,
        spacing,
        height1,
        height2,
        thickness,
        er,
    } = *input;
    validate::positive("height1", height1)?;
    validate::positive("height2", height2)?;
    validate::non_negative("thickness", thickness)?;

    let zo = z0_offset(width, height1, height2, thickness, er)?;
    let (ze1, zo1) = coupled_modes(width, spacing, 2.0 * height1 + thickness, thickness, er)?;
    let (ze2, zo2) = coupled_modes(width, spacing, 2.0 * height2 + thickness, thickness, er)?;
    types::build(zo, parallel(zo1, zo2), parallel(ze1, ze2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::differential::edge_coupled_internal_sym::{self, EdgeCoupledInternalSymInput};
    use approx::assert_relative_eq;

    fn input(
        width: f64,
        spacing: f64,
        height1: f64,
        height2: f64,
        thickness: f64,
        er: f64,
    ) -> EdgeCoupledInternalAsymInput {
        EdgeCoupledInternalAsymInput {
            width,
            spacing,
            height1,
            height2,
            thickness,
            er,
        }
    }

    #[test]
    fn symmetric_case_matches_centered_calculator_exactly() {
        let asym = calculate(&input(10.0, 5.0, 10.0, 10.0, 1.4, 4.6)).unwrap();
        let sym = edge_coupled_internal_sym::calculate(&EdgeCoupledInternalSymInput {
            width: 10.0,
            spacing: 5.0,
            height: 10.0,
            thickness: 1.4,
            er: 4.6,
        })
        .unwrap();
        assert_relative_eq!(asym.zo, sym.zo, max_relative = 1e-12);
        assert_relative_eq!(asym.zodd, sym.zodd, max_relative = 1e-12);
        assert_relative_eq!(asym.zeven, sym.zeven, max_relative = 1e-12);
    }

    #[test]
    fn offset_at_fixed_total_spacing_changes_impedance() {
        let centered = calculate(&input(10.0, 5.0, 10.0, 10.0, 1.4, 4.6)).unwrap();
        let offset = calculate(&input(10.0, 5.0, 1.0, 19.0, 1.4, 4.6)).unwrap();
        assert!((centered.zdiff - offset.zdiff).abs() > 1.0);
        assert!(
            offset.zdiff < centered.zdiff,
            "closer plane must lower Zdiff"
        );
    }

    #[test]
    fn swapping_planes_is_a_symmetry() {
        let a = calculate(&input(10.0, 5.0, 1.0, 19.0, 1.4, 4.6)).unwrap();
        let b = calculate(&input(10.0, 5.0, 19.0, 1.0, 1.4, 4.6)).unwrap();
        assert_relative_eq!(a.zdiff, b.zdiff, max_relative = 1e-12);
        assert_relative_eq!(a.kb, b.kb, max_relative = 1e-12);
    }

    #[test]
    fn taller_span_raises_z0() {
        let baseline = calculate(&input(10.0, 5.0, 10.0, 10.0, 1.4, 4.6)).unwrap();
        let taller = calculate(&input(10.0, 5.0, 15.0, 10.0, 1.4, 4.6)).unwrap();
        assert!(taller.zo > baseline.zo);
    }

    #[test]
    fn wider_spacing_reduces_coupling() {
        let close = calculate(&input(10.0, 5.0, 10.0, 10.0, 1.4, 4.6)).unwrap();
        let far = calculate(&input(10.0, 20.0, 10.0, 10.0, 1.4, 4.6)).unwrap();
        assert!(far.kb < close.kb);
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert!(calculate(&input(10.0, 5.0, -1.0, 10.0, 1.4, 4.6)).is_err());
        assert!(calculate(&input(10.0, 5.0, 10.0, 10.0, 1.4, 0.5)).is_err());
        assert!(calculate(&input(10.0, 5.0, f64::NAN, 10.0, 1.4, 4.6)).is_err());
    }
}
