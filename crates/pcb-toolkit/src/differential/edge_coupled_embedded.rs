//! Edge-coupled embedded (buried microstrip) differential pair.
//!
//! The single-ended base is the same IPC-2141A surface expression used by
//! [`super::edge_coupled_external`], so a zero cover reproduces the external
//! result exactly. The cover is applied with the embedded-microstrip filling
//! model from [`crate::impedance::embedded`]: the effective permittivity moves
//! from the surface value towards εr as `1 − exp(−2·cover/H)` and the
//! impedance scales by `√(εeff,surface/εeff,embedded)`. The IPC-2141A
//! coupling factor is then applied to the buried single-ended impedance.

use super::edge_coupled_external::{ipc2141_odd_factor, ipc2141_zo};
use super::types::{self, DifferentialResult};
use crate::impedance::{common, embedded};
use crate::model::{ModelInfo, ModelStatus};
use crate::{CalcError, validate};

/// Model description.
pub const MODEL: ModelInfo = ModelInfo {
    name: "IPC-2141A surface pair with exponential cover filling (embedded microstrip)",
    status: ModelStatus::Compatibility,
    reference: "IPC-2141A; Wadell 1991 §3.5.4 cover model",
    validity: "Same range as the external pair; cover of the same εr as the substrate; reduces exactly to the external pair at zero cover",
};

/// Inputs for edge-coupled embedded (buried) differential pair.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdgeCoupledEmbeddedInput {
    /// Conductor width (mils).
    pub width: f64,
    /// Gap between traces (mils).
    pub spacing: f64,
    /// Dielectric height to ground plane (mils).
    pub height: f64,
    /// Conductor thickness (mils).
    pub thickness: f64,
    /// Substrate (and cover) relative permittivity.
    pub er: f64,
    /// Cover height — dielectric above the trace (mils). 0 = external pair.
    pub cover_height: f64,
}

/// Compute differential impedance for an edge-coupled embedded (buried) pair.
pub fn calculate(input: &EdgeCoupledEmbeddedInput) -> Result<DifferentialResult, CalcError> {
    let EdgeCoupledEmbeddedInput {
        width,
        spacing,
        height,
        thickness,
        er,
        cover_height,
    } = *input;
    validate::positive("spacing", spacing)?;
    validate::non_negative("cover_height", cover_height)?;

    let zo_surface = ipc2141_zo(width, height, thickness, er)?;
    // Surface effective permittivity (H-J) is only used for the filling ratio.
    let er_eff_surface = common::hj_er_eff(width / height, er);
    let er_eff = embedded::er_eff_embedded(er, er_eff_surface, cover_height, height);
    let zo = embedded::zo_embedded(zo_surface, er_eff_surface, er_eff);

    let zodd = zo * ipc2141_odd_factor(spacing, height);
    let zeven = zo * zo / zodd;
    types::build(zo, zodd, zeven)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::differential::edge_coupled_external::{self, EdgeCoupledExternalInput};
    use approx::assert_relative_eq;

    fn input(
        width: f64,
        spacing: f64,
        height: f64,
        thickness: f64,
        er: f64,
        cover_height: f64,
    ) -> EdgeCoupledEmbeddedInput {
        EdgeCoupledEmbeddedInput {
            width,
            spacing,
            height,
            thickness,
            er,
            cover_height,
        }
    }

    /// The boundary relationship is tested against the actual external calculator.
    #[test]
    fn zero_cover_matches_external() {
        let embedded = calculate(&input(10.0, 5.0, 15.0, 2.10, 4.6, 0.0)).unwrap();
        let external = edge_coupled_external::calculate(&EdgeCoupledExternalInput {
            width: 10.0,
            spacing: 5.0,
            height: 15.0,
            thickness: 2.10,
            er: 4.6,
        })
        .unwrap();
        assert_relative_eq!(embedded.zdiff, external.zdiff, max_relative = 1e-12);
        assert_relative_eq!(embedded.zo, external.zo, max_relative = 1e-12);
        assert_relative_eq!(embedded.kb, external.kb, max_relative = 1e-12);
    }

    #[test]
    fn continuous_at_zero_cover() {
        let a = calculate(&input(10.0, 5.0, 15.0, 2.10, 4.6, 0.0)).unwrap();
        let b = calculate(&input(10.0, 5.0, 15.0, 2.10, 4.6, 1e-6)).unwrap();
        assert_relative_eq!(a.zdiff, b.zdiff, max_relative = 1e-5);
    }

    #[test]
    fn deeper_burial_reduces_z0_monotonically() {
        let mut prev = calculate(&input(10.0, 5.0, 15.0, 2.10, 4.6, 0.0))
            .unwrap()
            .zo;
        for c in [0.5, 2.0, 5.0, 20.0] {
            let z = calculate(&input(10.0, 5.0, 15.0, 2.10, 4.6, c)).unwrap().zo;
            assert!(z < prev, "cover {c}");
            prev = z;
        }
    }

    #[test]
    fn air_cover_on_air_pair_changes_nothing() {
        let a = calculate(&input(10.0, 5.0, 15.0, 2.10, 1.0, 0.0)).unwrap();
        let b = calculate(&input(10.0, 5.0, 15.0, 2.10, 1.0, 5.0)).unwrap();
        assert_relative_eq!(a.zdiff, b.zdiff, max_relative = 1e-12);
    }

    #[test]
    fn wider_spacing_reduces_coupling() {
        let close = calculate(&input(10.0, 5.0, 15.0, 2.10, 4.6, 3.0)).unwrap();
        let far = calculate(&input(10.0, 20.0, 15.0, 2.10, 4.6, 3.0)).unwrap();
        assert!(far.kb < close.kb);
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert!(calculate(&input(-1.0, 5.0, 15.0, 2.10, 4.6, 0.0)).is_err());
        assert!(calculate(&input(10.0, 5.0, 15.0, 2.10, 4.6, -1.0)).is_err());
        assert!(calculate(&input(10.0, 5.0, 15.0, 2.10, 4.6, f64::NAN)).is_err());
    }
}
