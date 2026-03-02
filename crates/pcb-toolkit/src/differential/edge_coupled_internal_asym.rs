//! Edge-coupled internal asymmetric (offset) differential pair impedance calculator.
//!
//! Computes odd-mode, even-mode, and differential impedance for an offset stripline
//! differential pair using the Wadell offset stripline formula with coupling correction.

use crate::CalcError;
use super::types::{DifferentialResult, kb_terminated};

/// Inputs for edge-coupled internal asymmetric (offset) differential pair.
pub struct EdgeCoupledInternalAsymInput {
    /// Conductor width (mils).
    pub width: f64,
    /// Gap between traces (mils).
    pub spacing: f64,
    /// Dielectric height from trace to top ground plane (mils).
    pub height1: f64,
    /// Dielectric height from trace to bottom ground plane (mils).
    pub height2: f64,
    /// Conductor thickness (mils).
    pub thickness: f64,
    /// Substrate relative permittivity.
    pub er: f64,
}

/// Compute differential impedance for an edge-coupled internal asymmetric (offset) pair.
pub fn calculate(input: &EdgeCoupledInternalAsymInput) -> Result<DifferentialResult, CalcError> {
    let EdgeCoupledInternalAsymInput { width, spacing, height1, height2, thickness, er } = *input;

    if width <= 0.0 {
        return Err(CalcError::NegativeDimension { name: "width", value: width });
    }
    if spacing <= 0.0 {
        return Err(CalcError::NegativeDimension { name: "spacing", value: spacing });
    }
    if height1 <= 0.0 {
        return Err(CalcError::NegativeDimension { name: "height1", value: height1 });
    }
    if height2 <= 0.0 {
        return Err(CalcError::NegativeDimension { name: "height2", value: height2 });
    }
    if thickness <= 0.0 {
        return Err(CalcError::NegativeDimension { name: "thickness", value: thickness });
    }
    if er < 1.0 {
        return Err(CalcError::OutOfRange {
            name: "er",
            value: er,
            expected: ">= 1.0",
        });
    }

    let z0 = (60.0 / er.sqrt())
        * (1.9 * (height1 + height2 + thickness) / (0.8 * width + thickness)).ln();

    let h_ref = (height1 + height2) / 2.0;
    let zodd = z0 * (1.0 - 0.48 * (-0.96 * spacing / h_ref).exp());
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
        height1: f64,
        height2: f64,
        thickness: f64,
        er: f64,
    ) -> EdgeCoupledInternalAsymInput {
        EdgeCoupledInternalAsymInput { width, spacing, height1, height2, thickness, er }
    }

    /// When H1 == H2 the asymmetric formula collapses to the symmetric offset stripline.
    /// Z0 = (60/√Er) × ln(1.9 × (2H + T) / (0.8W + T))
    #[test]
    fn symmetric_case_matches_formula() {
        let w = 10.0;
        let s = 5.0;
        let h = 10.0;
        let t = 1.4;
        let er = 4.6_f64;

        let result = calculate(&input(w, s, h, h, t, er)).unwrap();

        let z0_expected = (60.0 / er.sqrt()) * (1.9 * (2.0 * h + t) / (0.8 * w + t)).ln();
        assert_relative_eq!(result.zo, z0_expected, max_relative = 1e-10);

        let h_ref = h;
        let zodd_expected = z0_expected * (1.0 - 0.48 * (-0.96 * s / h_ref).exp());
        assert_relative_eq!(result.zodd, zodd_expected, max_relative = 1e-10);
        assert_relative_eq!(result.zdiff, 2.0 * zodd_expected, max_relative = 1e-10);
    }

    /// Changing H1 alters the total dielectric span (H1+H2) and thus Z0.
    #[test]
    fn asymmetric_heights_change_z0() {
        let baseline = calculate(&input(10.0, 5.0, 10.0, 10.0, 1.4, 4.6)).unwrap();
        let taller   = calculate(&input(10.0, 5.0, 15.0, 10.0, 1.4, 4.6)).unwrap();

        assert!(
            taller.zo > baseline.zo,
            "taller dielectric span Z0 {:.4} should exceed baseline Z0 {:.4}",
            taller.zo,
            baseline.zo
        );
    }

    /// Wider spacing reduces inter-trace coupling.
    #[test]
    fn wider_spacing_reduces_coupling() {
        let close = calculate(&input(10.0,  5.0, 10.0, 10.0, 1.4, 4.6)).unwrap();
        let far   = calculate(&input(10.0, 20.0, 10.0, 10.0, 1.4, 4.6)).unwrap();

        assert!(
            far.kb.abs() < close.kb.abs(),
            "wider spacing Kb {:.4} should be smaller than {:.4}",
            far.kb,
            close.kb
        );
    }

    /// Higher substrate permittivity lowers single-ended impedance.
    #[test]
    fn higher_er_gives_lower_z0() {
        let low_er  = calculate(&input(10.0, 5.0, 10.0, 10.0, 1.4, 2.2)).unwrap();
        let high_er = calculate(&input(10.0, 5.0, 10.0, 10.0, 1.4, 4.6)).unwrap();

        assert!(
            high_er.zo < low_er.zo,
            "higher Er Z0 {:.3} should be less than lower Er Z0 {:.3}",
            high_er.zo,
            low_er.zo
        );
    }

    #[test]
    fn rejects_negative_height1() {
        let result = calculate(&input(10.0, 5.0, -1.0, 10.0, 1.4, 4.6));
        assert!(result.is_err());
    }

    #[test]
    fn rejects_er_below_one() {
        let result = calculate(&input(10.0, 5.0, 10.0, 10.0, 1.4, 0.5));
        assert!(result.is_err());
    }
}
