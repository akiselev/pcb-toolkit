//! Edge-coupled external (surface) differential pair impedance calculator.
//!
//! Computes odd-mode, even-mode, and differential impedance for a surface
//! microstrip differential pair using the IPC-2141 approximation formula.

use crate::CalcError;
use super::types::{DifferentialResult, kb_terminated};

/// Inputs for edge-coupled external (surface) differential pair.
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

/// Compute differential impedance for an edge-coupled external (surface) pair.
pub fn calculate(input: &EdgeCoupledExternalInput) -> Result<DifferentialResult, CalcError> {
    let EdgeCoupledExternalInput { width, spacing, height, thickness, er } = *input;

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

    let z0 = (87.0 / (er + 1.41_f64).sqrt())
        * (5.98 * height / (0.8 * width + thickness)).ln();

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

    fn input(width: f64, spacing: f64, height: f64, thickness: f64, er: f64) -> EdgeCoupledExternalInput {
        EdgeCoupledExternalInput { width, spacing, height, thickness, er }
    }

    /// Saturn PDF page 11: W=10, S=5, H=15, Er=4.6, T=2.10
    #[test]
    fn saturn_pdf_page11() {
        let result = calculate(&input(10.0, 5.0, 15.0, 2.10, 4.6)).unwrap();

        assert_relative_eq!(result.zo,    77.504,  max_relative = 0.002);
        assert_relative_eq!(result.zodd,  50.490,  max_relative = 0.002);
        assert_relative_eq!(result.zeven, 118.971, max_relative = 0.002);
        assert_relative_eq!(result.zdiff, 100.979, max_relative = 0.002);
        assert_relative_eq!(result.kb,      0.4041,  max_relative = 0.002);
        assert_relative_eq!(result.kb_db,   -7.870,  max_relative = 0.002);
        assert_relative_eq!(result.kb_term, 0.2111,  max_relative = 0.005);
        assert_relative_eq!(result.kb_term_db, -13.512, max_relative = 0.005);
    }

    /// Wider spacing reduces coupling magnitude.
    #[test]
    fn wider_spacing_reduces_coupling() {
        let close = calculate(&input(10.0, 5.0, 15.0, 2.10, 4.6)).unwrap();
        let far   = calculate(&input(10.0, 20.0, 15.0, 2.10, 4.6)).unwrap();

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
        let low_er  = calculate(&input(10.0, 5.0, 15.0, 2.10, 3.0)).unwrap();
        let high_er = calculate(&input(10.0, 5.0, 15.0, 2.10, 4.6)).unwrap();

        assert!(
            high_er.zo < low_er.zo,
            "higher Er Zo {:.3} should be less than {:.3}",
            high_er.zo,
            low_er.zo
        );
    }

    #[test]
    fn rejects_negative_width() {
        let result = calculate(&input(-1.0, 5.0, 15.0, 2.10, 4.6));
        assert!(result.is_err());
    }

    #[test]
    fn rejects_er_below_one() {
        let result = calculate(&input(10.0, 5.0, 15.0, 2.10, 0.5));
        assert!(result.is_err());
    }
}
