//! Padstack geometry calculator.
//!
//! Implemented sub-calculators:
//! 1. Thru-hole pad sizing (pad and anti-pad from hole, annular ring, isolation)
//! 2. Corner-to-corner (diagonal) distance
//!
//! Saturn's BGA land size and conductor/pad routing sub-calculators are not
//! implemented.

use serde::{Deserialize, Serialize};

use crate::model::{ModelInfo, ModelStatus};
use crate::{CalcError, validate};

/// Model description.
pub const MODEL: ModelInfo = ModelInfo {
    name: "Padstack geometry (thru-hole pad sizing, diagonal distance)",
    status: ModelStatus::Validated,
    reference: "Exact geometry; Saturn PCB Toolkit help p.23",
    validity: "Exact; BGA land and routing sub-calculators are not implemented",
};

/// Input parameters for the thru-hole pad calculator.
///
/// All dimensions in mils.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThruHoleInput {
    /// Drilled hole diameter in mils.
    pub hole_diameter_mils: f64,
    /// Annular ring width (copper from hole edge to pad edge) in mils.
    pub annular_ring_mils: f64,
    /// Isolation width (clearance from pad edge to plane copper) in mils.
    pub isolation_width_mils: f64,
}

/// Computed pad sizes for a plated thru-hole.
///
/// All dimensions in mils.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThruHoleResult {
    /// Pad diameter on external (signal) layers in mils.
    ///
    /// `hole_diameter + 2 × annular_ring`
    pub pad_external_mils: f64,
    /// Pad diameter on internal signal layers in mils.
    ///
    /// Same as external for plated thru-hole.
    pub pad_internal_signal_mils: f64,
    /// Anti-pad (clearance opening) diameter on internal plane layers in mils.
    ///
    /// `pad_external + 2 × isolation_width`
    pub pad_internal_plane_mils: f64,
}

/// Calculate pad sizes for a plated thru-hole component pad.
///
/// # Arguments
/// - `input` — hole geometry and design rule parameters
///
/// # Errors
/// Returns [`CalcError::NegativeDimension`] for non-positive dimensions.
pub fn thru_hole(input: &ThruHoleInput) -> Result<ThruHoleResult, CalcError> {
    validate::positive("hole_diameter_mils", input.hole_diameter_mils)?;
    validate::non_negative("annular_ring_mils", input.annular_ring_mils)?;
    validate::non_negative("isolation_width_mils", input.isolation_width_mils)?;

    let pad_external_mils = input.hole_diameter_mils + 2.0 * input.annular_ring_mils;
    let pad_internal_signal_mils = pad_external_mils;
    let pad_internal_plane_mils = pad_external_mils + 2.0 * input.isolation_width_mils;

    Ok(ThruHoleResult {
        pad_external_mils: validate::finite_result("pad_external_mils", pad_external_mils)?,
        pad_internal_signal_mils,
        pad_internal_plane_mils: validate::finite_result(
            "pad_internal_plane_mils",
            pad_internal_plane_mils,
        )?,
    })
}

/// Calculate the corner-to-corner (diagonal) distance between two points.
///
/// # Arguments
/// - `a_mils` — horizontal span in mils (must be ≥ 0)
/// - `b_mils` — vertical span in mils (must be ≥ 0)
///
/// Returns the Euclidean distance `√(a² + b²)` in mils.
///
/// # Errors
/// Returns [`CalcError::NegativeDimension`] if either dimension is negative.
pub fn corner_to_corner(a_mils: f64, b_mils: f64) -> Result<f64, CalcError> {
    validate::non_negative("a_mils", a_mils)?;
    validate::non_negative("b_mils", b_mils)?;
    validate::finite_result("distance", a_mils.hypot(b_mils))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    // Saturn PCB Toolkit help PDF page 23:
    //   Thru-Hole Pad, hole=32 mils, annular ring=12 mils, isolation=12 mils
    //
    //   External layers        = 56.00 mils
    //   Internal signal layers = 56.00 mils
    //   Internal plane layers  = 80.00 mils
    #[test]
    fn saturn_page23_thru_hole_vector() {
        let input = ThruHoleInput {
            hole_diameter_mils: 32.0,
            annular_ring_mils: 12.0,
            isolation_width_mils: 12.0,
        };
        let result = thru_hole(&input).unwrap();
        assert_relative_eq!(result.pad_external_mils, 56.0, epsilon = 1e-10);
        assert_relative_eq!(result.pad_internal_signal_mils, 56.0, epsilon = 1e-10);
        assert_relative_eq!(result.pad_internal_plane_mils, 80.0, epsilon = 1e-10);
    }

    #[test]
    fn corner_to_corner_3_4_5() {
        let d = corner_to_corner(3.0, 4.0).unwrap();
        assert_relative_eq!(d, 5.0, epsilon = 1e-10);
    }

    #[test]
    fn corner_to_corner_zero() {
        let d = corner_to_corner(0.0, 0.0).unwrap();
        assert_relative_eq!(d, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn error_on_zero_hole() {
        let input = ThruHoleInput {
            hole_diameter_mils: 0.0,
            annular_ring_mils: 12.0,
            isolation_width_mils: 12.0,
        };
        assert!(thru_hole(&input).is_err());
    }

    #[test]
    fn error_on_negative_annular_ring() {
        let input = ThruHoleInput {
            hole_diameter_mils: 32.0,
            annular_ring_mils: -1.0,
            isolation_width_mils: 12.0,
        };
        assert!(thru_hole(&input).is_err());
    }

    #[test]
    fn error_on_negative_corner_dimension() {
        assert!(corner_to_corner(-1.0, 4.0).is_err());
        assert!(corner_to_corner(3.0, -1.0).is_err());
        assert!(corner_to_corner(f64::NAN, 1.0).is_err());
    }

    #[test]
    fn hypot_does_not_overflow_intermediate() {
        let d = corner_to_corner(1e200, 1e200).unwrap();
        assert!(d.is_finite());
        assert_relative_eq!(d, 1e200 * std::f64::consts::SQRT_2, max_relative = 1e-12);
    }
}
