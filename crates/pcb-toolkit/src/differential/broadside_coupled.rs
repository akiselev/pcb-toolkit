//! Broadside-coupled differential pair impedance calculator.
//!
//! Supports shielded (between two ground planes, Wadell stripline model) and
//! non-shielded (single ground plane below, microstrip-style model) configurations.
//!
//! # TODO
//! - Validate against Saturn when test vector available (confidence: LOW — no RE data)

use crate::CalcError;
use super::types::{DifferentialResult, kb_terminated};

/// Inputs for broadside-coupled differential pair.
pub struct BroadsideCoupledInput {
    /// Strip width (mils).
    pub width: f64,
    /// Vertical separation between the two strips (mils).
    pub separation: f64,
    /// Ground-to-ground spacing (mils). Only used in shielded mode.
    pub height_total: f64,
    /// Conductor thickness (mils).
    pub thickness: f64,
    /// Substrate relative permittivity.
    pub er: f64,
    /// true = shielded (between two ground planes), false = non-shielded
    pub shielded: bool,
}

/// Compute differential impedance for a broadside-coupled differential pair.
pub fn calculate(input: &BroadsideCoupledInput) -> Result<DifferentialResult, CalcError> {
    let BroadsideCoupledInput { width, separation, height_total, thickness, er, shielded } = *input;

    if width <= 0.0 {
        return Err(CalcError::NegativeDimension { name: "width", value: width });
    }
    if separation <= 0.0 {
        return Err(CalcError::NegativeDimension { name: "separation", value: separation });
    }
    if height_total <= 0.0 {
        return Err(CalcError::NegativeDimension { name: "height_total", value: height_total });
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

    let z0 = if shielded {
        (60.0 / er.sqrt())
            * (1.9 * height_total / (0.8 * width + thickness)).ln()
    } else {
        (87.0 / (er + 1.41_f64).sqrt())
            * (5.98 * (separation + height_total) / (0.8 * width + thickness)).ln()
    };

    let cf = 1.0 - 1.0 / (std::f64::consts::PI * width / (2.0 * separation)).cosh();

    let zodd  = z0 * (1.0 - cf);
    let zeven = z0 * (1.0 + cf);
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

    fn shielded(width: f64, separation: f64, height_total: f64, thickness: f64, er: f64) -> BroadsideCoupledInput {
        BroadsideCoupledInput { width, separation, height_total, thickness, er, shielded: true }
    }

    fn unshielded(width: f64, separation: f64, height_total: f64, thickness: f64, er: f64) -> BroadsideCoupledInput {
        BroadsideCoupledInput { width, separation, height_total, thickness, er, shielded: false }
    }

    /// Wider strip lowers Z0 (stripline: denominator 0.8*W+T grows).
    #[test]
    fn shielded_wider_strip_lowers_z0() {
        let narrow = calculate(&shielded(5.0,  5.0, 30.0, 2.0, 4.5)).unwrap();
        let wide   = calculate(&shielded(20.0, 5.0, 30.0, 2.0, 4.5)).unwrap();

        assert!(
            narrow.zo > wide.zo,
            "narrow Z0 {:.3} should exceed wide Z0 {:.3}",
            narrow.zo,
            wide.zo
        );
    }

    /// Larger separation reduces coupling coefficient Kb.
    #[test]
    fn shielded_larger_separation_reduces_coupling() {
        let close = calculate(&shielded(10.0, 5.0,  30.0, 2.0, 4.5)).unwrap();
        let far   = calculate(&shielded(10.0, 20.0, 30.0, 2.0, 4.5)).unwrap();

        assert!(
            far.kb < close.kb,
            "wider separation Kb {:.4} should be less than {:.4}",
            far.kb,
            close.kb
        );
    }

    /// Zdiff = 2 * Zodd is an exact identity.
    #[test]
    fn shielded_zdiff_equals_two_zodd() {
        let r = calculate(&shielded(10.0, 5.0, 30.0, 2.0, 4.5)).unwrap();
        assert_relative_eq!(r.zdiff, 2.0 * r.zodd, max_relative = 1e-12);
    }

    /// Non-shielded should produce a Z0 in the 20–200 Ohm range.
    #[test]
    fn unshielded_reasonable_impedance() {
        let r = calculate(&unshielded(10.0, 5.0, 15.0, 2.0, 4.5)).unwrap();

        assert!(
            r.zo >= 20.0 && r.zo <= 200.0,
            "Z0 {:.3} should be in [20, 200] Ohm range",
            r.zo
        );
    }

    #[test]
    fn rejects_negative_width() {
        let result = calculate(&shielded(-1.0, 5.0, 30.0, 2.0, 4.5));
        assert!(result.is_err());
    }

    #[test]
    fn rejects_er_below_one() {
        let result = calculate(&shielded(10.0, 5.0, 30.0, 2.0, 0.5));
        assert!(result.is_err());
    }
}
