//! Edge-coupled internal symmetric (centered stripline) differential pair impedance calculator.
//!
//! Computes odd-mode, even-mode, and differential impedance for a symmetric stripline
//! differential pair using the Cohn stripline Z0 formula with exponential coupling correction.

use crate::CalcError;
use super::types::{DifferentialResult, kb_terminated};

/// Inputs for edge-coupled internal symmetric (centered stripline) differential pair.
pub struct EdgeCoupledInternalSymInput {
    /// Conductor width (mils).
    pub width: f64,
    /// Gap between traces (mils).
    pub spacing: f64,
    /// Dielectric height — distance from trace to each ground plane (mils).
    /// For centered stripline, total dielectric thickness = 2 × height.
    pub height: f64,
    /// Conductor thickness (mils).
    pub thickness: f64,
    /// Substrate relative permittivity.
    pub er: f64,
}

/// Compute differential impedance for an edge-coupled internal symmetric (centered stripline) pair.
pub fn calculate(input: &EdgeCoupledInternalSymInput) -> Result<DifferentialResult, CalcError> {
    let EdgeCoupledInternalSymInput { width, spacing, height, thickness, er } = *input;

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
    if er < 1.0 {
        return Err(CalcError::OutOfRange {
            name: "er",
            value: er,
            expected: ">= 1.0",
        });
    }

    let z0 = (60.0 / er.sqrt()) * (1.9 * (2.0 * height + thickness) / (0.8 * width + thickness)).ln();

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

    fn input(width: f64, spacing: f64, height: f64, thickness: f64, er: f64) -> EdgeCoupledInternalSymInput {
        EdgeCoupledInternalSymInput { width, spacing, height, thickness, er }
    }

    /// Web reference vector: W=10, S=63, H=63, T=1.2, Er=4.
    /// S/H=1, weak coupling; Zdiff should be in the range 140–170 Ω.
    #[test]
    fn web_reference_weak_coupling() {
        let result = calculate(&input(10.0, 63.0, 63.0, 1.2, 4.0)).unwrap();

        assert!(
            result.zdiff >= 140.0 && result.zdiff <= 170.0,
            "Zdiff {:.3} should be in range 140–170 Ω",
            result.zdiff
        );
    }

    /// Narrower spacing increases coupling magnitude (higher Kb).
    #[test]
    fn wider_spacing_reduces_coupling() {
        let close = calculate(&input(10.0, 5.0, 63.0, 1.2, 4.0)).unwrap();
        let far   = calculate(&input(10.0, 20.0, 63.0, 1.2, 4.0)).unwrap();

        assert!(
            far.kb.abs() < close.kb.abs(),
            "wider spacing Kb {:.4} should be smaller than {:.4}",
            far.kb,
            close.kb
        );
    }

    /// Higher Er gives lower Z0.
    #[test]
    fn higher_er_gives_lower_z0() {
        let low_er  = calculate(&input(10.0, 10.0, 63.0, 1.2, 2.2)).unwrap();
        let high_er = calculate(&input(10.0, 10.0, 63.0, 1.2, 4.6)).unwrap();

        assert!(
            high_er.zo < low_er.zo,
            "higher Er Z0 {:.3} should be less than {:.3}",
            high_er.zo,
            low_er.zo
        );
    }

    /// For a fully embedded stripline, Er_eff equals Er exactly.
    #[test]
    fn er_eff_equals_er() {
        let er = 4.6_f64;
        let result = calculate(&input(10.0, 10.0, 63.0, 1.2, er)).unwrap();

        // Z0 = (60/sqrt(Er)) * ln(...); verify by back-computing Er from Z0 and the log term.
        let ln_term = (1.9_f64 * (2.0 * 63.0 + 1.2) / (0.8 * 10.0 + 1.2)).ln();
        let er_back = (60.0 * ln_term / result.zo).powi(2);
        assert_relative_eq!(er_back, er, max_relative = 1e-10);
    }

    #[test]
    fn rejects_negative_width() {
        let result = calculate(&input(-1.0, 10.0, 63.0, 1.2, 4.0));
        assert!(result.is_err());
    }

    #[test]
    fn rejects_er_below_one() {
        let result = calculate(&input(10.0, 10.0, 63.0, 1.2, 0.5));
        assert!(result.is_err());
    }
}
