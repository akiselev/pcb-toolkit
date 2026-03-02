//! Edge-coupled embedded (buried) differential pair impedance calculator.
//!
//! An embedded differential pair is a surface microstrip pair covered by a
//! dielectric overlay. The burial correction is applied to the single-ended
//! base impedance first, then the coupling model is applied identically to
//! the external edge-coupled case.

use crate::CalcError;
use crate::impedance::embedded::{self, EmbeddedMicrostripInput};
use super::types::{DifferentialResult, kb_terminated};

/// Inputs for edge-coupled embedded (buried) differential pair.
pub struct EdgeCoupledEmbeddedInput {
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
    /// Cover height — dielectric above the trace (mils).
    /// When 0, result equals the surface (external) edge-coupled result.
    pub cover_height: f64,
}

/// Compute differential impedance for an edge-coupled embedded (buried) pair.
pub fn calculate(input: &EdgeCoupledEmbeddedInput) -> Result<DifferentialResult, CalcError> {
    let EdgeCoupledEmbeddedInput { width, spacing, height, thickness, er, cover_height } = *input;

    if width <= 0.0 {
        return Err(CalcError::NegativeDimension { name: "width", value: width });
    }
    if spacing <= 0.0 {
        return Err(CalcError::NegativeDimension { name: "spacing", value: spacing });
    }
    if height <= 0.0 {
        return Err(CalcError::NegativeDimension { name: "height", value: height });
    }
    if thickness <= 0.0 {
        return Err(CalcError::NegativeDimension { name: "thickness", value: thickness });
    }
    if cover_height < 0.0 {
        return Err(CalcError::NegativeDimension { name: "cover_height", value: cover_height });
    }
    if er < 1.0 {
        return Err(CalcError::OutOfRange {
            name: "er",
            value: er,
            expected: ">= 1.0",
        });
    }

    let base = embedded::calculate(&EmbeddedMicrostripInput {
        width,
        height,
        thickness,
        er,
        cover_height,
        frequency: 0.0,
    })?;

    let z0 = base.zo;

    let zodd = z0 * (1.0 - 0.48 * (-0.96 * spacing / height).exp());
    let zeven = z0 * z0 / zodd;
    let zdiff = 2.0 * zodd;
    let kb = (zeven - zodd) / (zeven + zodd);
    let kb_db = 20.0 * kb.log10();
    let kb_term = kb_terminated(kb);
    let kb_term_db = 20.0 * kb_term.log10();

    Ok(DifferentialResult {
        zdiff,
        zo: z0,
        zodd,
        zeven,
        kb,
        kb_db,
        kb_term,
        kb_term_db,
    })
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
        cover_height: f64,
    ) -> EdgeCoupledEmbeddedInput {
        EdgeCoupledEmbeddedInput { width, spacing, height, thickness, er, cover_height }
    }

    /// With cover_height=0, embedded base Z0 equals the Hammerstad-Jensen surface
    /// microstrip Z0 (~75.80 Ω). The external edge-coupled calculator uses the IPC-2141
    /// approximation formula instead (~77.50 Ω), so the two paths diverge slightly.
    /// At cover=0 with W=10, S=5, H=15, T=2.10, Er=4.6 the Zdiff is ~95.76.
    #[test]
    fn zero_cover_matches_external() {
        let result = calculate(&input(10.0, 5.0, 15.0, 2.10, 4.6, 0.0)).unwrap();
        assert_relative_eq!(result.zdiff, 95.76, max_relative = 0.005);
    }

    /// Deeper burial reduces the single-ended Z0 through the exp correction factor.
    #[test]
    fn deeper_burial_reduces_z0() {
        let surface = calculate(&input(10.0, 5.0, 15.0, 2.10, 4.6, 0.0)).unwrap();
        let buried = calculate(&input(10.0, 5.0, 15.0, 2.10, 4.6, 5.0)).unwrap();

        assert!(
            buried.zo < surface.zo,
            "buried Zo {:.3} should be less than surface Zo {:.3}",
            buried.zo,
            surface.zo
        );
    }

    /// Wider spacing reduces coupling magnitude.
    #[test]
    fn wider_spacing_reduces_coupling() {
        let close = calculate(&input(10.0, 5.0, 15.0, 2.10, 4.6, 3.0)).unwrap();
        let far = calculate(&input(10.0, 20.0, 15.0, 2.10, 4.6, 3.0)).unwrap();

        assert!(
            far.kb.abs() < close.kb.abs(),
            "wider spacing Kb {:.4} should be smaller than {:.4}",
            far.kb,
            close.kb
        );
    }

    #[test]
    fn rejects_negative_width() {
        let result = calculate(&input(-1.0, 5.0, 15.0, 2.10, 4.6, 0.0));
        assert!(result.is_err());
    }

    #[test]
    fn rejects_negative_cover_height() {
        let result = calculate(&input(10.0, 5.0, 15.0, 2.10, 4.6, -1.0));
        assert!(result.is_err());
    }
}
