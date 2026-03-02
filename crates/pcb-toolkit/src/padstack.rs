//! Padstack calculator — pad sizing for TH, BGA, and routing.
//!
//! 7 sub-calculators:
//! 1. Thru-Hole Pad
//! 2. BGA Land Size (IPC-7351A)
//! 3. Conductor/Pad TH
//! 4. Conductor/Pad BGA
//! 5. 2 Conductors/Pad TH
//! 6. 2 Conductors/Pad BGA
//! 7. Corner to Corner

use serde::{Deserialize, Serialize};

use crate::CalcError;

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
    if input.hole_diameter_mils <= 0.0 {
        return Err(CalcError::NegativeDimension {
            name: "hole_diameter_mils",
            value: input.hole_diameter_mils,
        });
    }
    if input.annular_ring_mils < 0.0 {
        return Err(CalcError::NegativeDimension {
            name: "annular_ring_mils",
            value: input.annular_ring_mils,
        });
    }
    if input.isolation_width_mils < 0.0 {
        return Err(CalcError::NegativeDimension {
            name: "isolation_width_mils",
            value: input.isolation_width_mils,
        });
    }

    let pad_external_mils =
        input.hole_diameter_mils + 2.0 * input.annular_ring_mils;
    let pad_internal_signal_mils = pad_external_mils;
    let pad_internal_plane_mils =
        pad_external_mils + 2.0 * input.isolation_width_mils;

    Ok(ThruHoleResult {
        pad_external_mils,
        pad_internal_signal_mils,
        pad_internal_plane_mils,
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
    if a_mils < 0.0 {
        return Err(CalcError::NegativeDimension {
            name: "a_mils",
            value: a_mils,
        });
    }
    if b_mils < 0.0 {
        return Err(CalcError::NegativeDimension {
            name: "b_mils",
            value: b_mils,
        });
    }

    Ok((a_mils * a_mils + b_mils * b_mils).sqrt())
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
    }
}
