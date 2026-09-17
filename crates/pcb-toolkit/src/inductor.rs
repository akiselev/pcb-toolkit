//! Planar spiral inductor calculator.
//!
//! Reference: Mohan, Hershenson, Boyd, Lee, "Simple Accurate Expressions
//! for Planar Spiral Inductances", IEEE JSSC 34(10), October 1999.
//!
//! * Square, hexagonal, octagonal: modified Wheeler expression (paper eq. 2,
//!   Table I coefficients).
//! * Circular: current-sheet expression (paper eq. 3, Table II coefficients),
//!   because Table I gives no circular coefficients.
//!
//! The paper reports typical errors of 2–3% (max ~8%) against field-solver
//! and measured on-chip spirals with `ρ` (fill ratio) ≳ 0.1 and s ≤ 3w. It
//! is a free-space, low-frequency (below self-resonance) inductance; ground
//! planes close to the spiral, substrate eddy currents and the distributed
//! capacitance that sets the self-resonant frequency are not modelled.

use serde::{Deserialize, Serialize};

use crate::model::{ModelInfo, ModelStatus};
use crate::{CalcError, constants, validate};

/// Model description.
pub const MODEL: ModelInfo = ModelInfo {
    name: "Mohan et al. 1999 planar spiral inductance (modified Wheeler / current sheet)",
    status: ModelStatus::Validated,
    reference: "Mohan, Hershenson, Boyd & Lee, IEEE JSSC 34(10) 1999, eq. 2–3, Tables I–II",
    validity: "Fill ratio ρ ≳ 0.1, s ≤ 3w; ±3% typical vs. field solver; free-space DC inductance, no nearby ground plane",
};

/// Mils to meters conversion factor.
const MILS_TO_METERS: f64 = constants::MIL_TO_M;

/// Spiral geometry shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpiralShape {
    Square,
    Hexagonal,
    Octagonal,
    Circle,
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

/// Calculate the inductance of a planar spiral.
///
/// # Arguments
/// - `n_turns` — number of turns (must be ≥ 1)
/// - `width_mils` — trace width in mils (must be > 0)
/// - `spacing_mils` — inter-turn spacing in mils (must be > 0)
/// - `dout_mils` — outer diameter in mils (must be > 0)
/// - `shape` — spiral geometry
///
/// The inner diameter is derived as `din = dout − 2·n·(w + s) + 2·s`.
///
/// # Errors
/// Returns an error for invalid inputs or a geometry whose inner diameter
/// would not be positive.
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
            value: 0.0,
            expected: ">= 1",
        });
    }
    validate::positive("width_mils", width_mils)?;
    validate::positive("spacing_mils", spacing_mils)?;
    validate::positive("dout_mils", dout_mils)?;

    let n = f64::from(n_turns);
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

    let inductance_h = match shape {
        // Modified Wheeler (eq. 2): L = K1·μ0·n²·d_avg / (1 + K2·ρ), Table I.
        SpiralShape::Square => wheeler(2.34, 2.75, n, d_avg_m, rho),
        SpiralShape::Hexagonal => wheeler(2.33, 3.82, n, d_avg_m, rho),
        SpiralShape::Octagonal => wheeler(2.25, 3.55, n, d_avg_m, rho),
        // Current sheet (eq. 3): L = μ0·n²·d_avg·c1/2·[ln(c2/ρ) + c3·ρ + c4·ρ²], Table II circle.
        SpiralShape::Circle => {
            let (c1, c2, c3, c4) = (1.00, 2.46, 0.00, 0.20);
            constants::MU_0 * n * n * d_avg_m * c1 / 2.0
                * ((c2 / rho).ln() + c3 * rho + c4 * rho * rho)
        }
    };

    Ok(InductorResult {
        din_mils,
        rho,
        d_avg_mils,
        inductance_nh: validate::finite_result("inductance_nh", inductance_h * 1e9)?,
    })
}

fn wheeler(k1: f64, k2: f64, n: f64, d_avg_m: f64, rho: f64) -> f64 {
    k1 * constants::MU_0 * n * n * d_avg_m / (1.0 + k2 * rho)
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
    fn circle_uses_current_sheet_table_ii() {
        // n=5, d_avg=260 mil = 6.604 mm, ρ=0.34615:
        // L = μ0·25·6.604e-3·0.5·[ln(2.46/0.34615) + 0.2·0.34615²] = 208.6 nH
        let r = planar_spiral(5, 10.0, 10.0, 350.0, SpiralShape::Circle).unwrap();
        let expected = constants::MU_0
            * 25.0
            * 6.604e-3
            * 0.5
            * ((2.46_f64 / 0.346_153_846).ln() + 0.2 * 0.346_153_846_f64.powi(2))
            * 1e9;
        assert_relative_eq!(r.inductance_nh, expected, max_relative = 1e-6);
        // A circle encloses less area than a square of the same diameter: lower L.
        let sq = planar_spiral(5, 10.0, 10.0, 350.0, SpiralShape::Square).unwrap();
        assert!(r.inductance_nh < sq.inductance_nh);
    }

    #[test]
    fn shapes_are_ordered_by_enclosed_area() {
        let l = |s| {
            planar_spiral(3, 10.0, 10.0, 200.0, s)
                .unwrap()
                .inductance_nh
        };
        assert!(l(SpiralShape::Square) > l(SpiralShape::Octagonal));
        assert!(l(SpiralShape::Octagonal) > l(SpiralShape::Circle));
    }

    #[test]
    fn errors() {
        assert!(planar_spiral(0, 10.0, 10.0, 350.0, SpiralShape::Square).is_err());
        assert!(planar_spiral(50, 10.0, 10.0, 350.0, SpiralShape::Square).is_err());
        assert!(planar_spiral(5, f64::NAN, 10.0, 350.0, SpiralShape::Square).is_err());
    }
}
