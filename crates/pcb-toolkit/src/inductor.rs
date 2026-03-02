//! Planar spiral inductor calculator (Mohan/Wheeler modified).
//!
//! Reference: Mohan, Hershenson, Boyd, Lee — "Simple Accurate Expressions
//! for Planar Spiral Inductances", IEEE JSSC, October 1999.
//!
//! Supports square, hexagonal, octagonal, and circular geometries.

use serde::{Deserialize, Serialize};

use crate::CalcError;

/// Permeability of free space (H/m).
const MU_0: f64 = 4.0 * std::f64::consts::PI * 1e-7;

/// Mils to meters conversion factor.
const MILS_TO_METERS: f64 = 25.4e-6;

/// Spiral geometry shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpiralShape {
    Square,
    Hexagonal,
    Octagonal,
    Circle,
}

impl SpiralShape {
    /// Mohan coefficients (K1, K2) for each geometry.
    fn coefficients(self) -> (f64, f64) {
        match self {
            Self::Square => (2.34, 2.75),
            Self::Hexagonal => (2.33, 3.82),
            Self::Octagonal => (2.25, 3.55),
            Self::Circle => (2.23, 3.45),
        }
    }
}

/// Result of a planar spiral inductor calculation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InductorResult {
    /// Inner diameter of the spiral in mils.
    pub din_mils: f64,
    /// Fill factor ρ = (dout − din) / (dout + din).
    pub rho: f64,
    /// Average diameter d_avg = (dout + din) / 2, in mils.
    pub d_avg_mils: f64,
    /// Calculated inductance in nanohenries.
    pub inductance_nh: f64,
}

/// Calculate planar spiral inductor using the modified Wheeler / Mohan formula.
///
/// # Arguments
/// - `n_turns` — number of turns (must be ≥ 1)
/// - `width_mils` — trace width in mils (must be > 0)
/// - `spacing_mils` — inter-turn spacing in mils (must be > 0)
/// - `dout_mils` — outer diameter in mils (must be > 0)
/// - `shape` — spiral geometry
///
/// The inner diameter is derived as:
/// `din = dout − 2×n×(w+s) + 2×s`
///
/// # Errors
/// Returns [`CalcError::OutOfRange`] or [`CalcError::NegativeDimension`] if inputs are invalid.
pub fn planar_spiral(
    n_turns: u32,
    width_mils: f64,
    spacing_mils: f64,
    dout_mils: f64,
    shape: SpiralShape,
) -> Result<InductorResult, CalcError> {
    if n_turns == 0 {
        return Err(CalcError::OutOfRange {
            name: "n_turns",
            value: n_turns as f64,
            expected: ">= 1",
        });
    }
    if width_mils <= 0.0 {
        return Err(CalcError::NegativeDimension {
            name: "width_mils",
            value: width_mils,
        });
    }
    if spacing_mils <= 0.0 {
        return Err(CalcError::NegativeDimension {
            name: "spacing_mils",
            value: spacing_mils,
        });
    }
    if dout_mils <= 0.0 {
        return Err(CalcError::NegativeDimension {
            name: "dout_mils",
            value: dout_mils,
        });
    }

    let n = n_turns as f64;
    let din_mils = dout_mils - 2.0 * n * (width_mils + spacing_mils) + 2.0 * spacing_mils;

    if din_mils <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "din_mils (derived)",
            value: din_mils,
            expected: "> 0 — reduce n_turns, width, or spacing, or increase dout",
        });
    }

    let rho = (dout_mils - din_mils) / (dout_mils + din_mils);
    let d_avg_mils = (dout_mils + din_mils) / 2.0;
    let d_avg_m = d_avg_mils * MILS_TO_METERS;

    let (k1, k2) = shape.coefficients();
    let inductance_h = k1 * MU_0 * n * n * d_avg_m / (1.0 + k2 * rho);
    let inductance_nh = inductance_h * 1e9;

    Ok(InductorResult {
        din_mils,
        rho,
        d_avg_mils,
        inductance_nh,
    })
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    // Saturn PDF page 30: n=5, w=10 mils, s=10 mils, dout=350 mils, Square
    // din=170, ρ=0.3462, L=248.5936 nH
    #[test]
    fn saturn_page30_square() {
        let result = planar_spiral(5, 10.0, 10.0, 350.0, SpiralShape::Square).unwrap();
        assert_relative_eq!(result.din_mils, 170.0, epsilon = 1e-10);
        assert_relative_eq!(result.rho, 0.34615, epsilon = 1e-4);
        assert_relative_eq!(result.inductance_nh, 248.59, epsilon = 0.2);
    }

    #[test]
    fn derived_din_matches_spec() {
        // din = 350 - 2×5×(10+10) + 2×10 = 350 - 200 + 20 = 170
        let result = planar_spiral(5, 10.0, 10.0, 350.0, SpiralShape::Square).unwrap();
        assert_relative_eq!(result.din_mils, 170.0, epsilon = 1e-10);
    }

    #[test]
    fn error_on_zero_turns() {
        assert!(planar_spiral(0, 10.0, 10.0, 350.0, SpiralShape::Square).is_err());
    }

    #[test]
    fn error_on_din_negative() {
        // Very large n will make din negative
        assert!(planar_spiral(50, 10.0, 10.0, 350.0, SpiralShape::Square).is_err());
    }

    #[test]
    fn hexagonal_shape_accepted() {
        assert!(planar_spiral(3, 10.0, 10.0, 200.0, SpiralShape::Hexagonal).is_ok());
    }
}
