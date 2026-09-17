//! Fusing current calculator (Onderdonk's equation).
//!
//! Estimates the current that raises a copper conductor from ambient to its
//! melting point in a given time, assuming adiabatic heating (no conduction,
//! convection or radiation losses):
//!
//! ```text
//! I = A_cmil · √( log10(1 + (Tm − Ta)/(234 + Ta)) / (33 · t) )
//! ```
//!
//! This is a melting-onset estimate only. It is not a fuse time-current
//! curve, does not model arcing, opening, heat sinking into the laminate or
//! adjacent copper, and becomes increasingly conservative for times beyond
//! a few seconds where heat loss is significant.

use serde::{Deserialize, Serialize};

use crate::copper::{CopperWeight, EtchFactor, PlatingThickness};
use crate::model::{ModelInfo, ModelStatus};
use crate::{CalcError, constants, validate};

/// Model description.
pub const MODEL: ModelInfo = ModelInfo {
    name: "Onderdonk adiabatic fusing current",
    status: ModelStatus::Compatibility,
    reference: "Onderdonk (1928) as tabulated by Preece/IPC; Saturn PCB Toolkit help p.16",
    validity: "Adiabatic (t ≲ 5 s); −234 °C < Ta < Tm; melting onset only, not a protection model",
};

/// Copper melting temperature in °C. Re-exported from [`constants`].
pub const COPPER_MELTING_TEMP_C: f64 = constants::COPPER_MELTING_POINT_C;

/// Result of a fusing current calculation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FusingResult {
    /// Total copper thickness in mils.
    pub copper_thickness_mils: f64,
    /// Conductor cross-section in square mils (trapezoidal for etched profiles).
    pub area_sq_mils: f64,
    /// Conductor cross-section in circular mils (= area_sq_mils × 4/π).
    pub area_circular_mils: f64,
    /// Fusing (melting) current in Amperes.
    pub fusing_current_a: f64,
    /// Copper melting temperature used (°C).
    pub melting_temp_c: f64,
}

/// Calculate fusing current from cross-sectional area using Onderdonk's equation.
///
/// # Arguments
/// - `area_circular_mils` — conductor cross-section in circular mils (must be > 0)
/// - `time_s` — pulse duration in seconds (must be > 0)
/// - `ambient_c` — ambient temperature in °C (must be > −234 °C)
/// - `melting_temp_c` — melting temperature of conductor in °C (must be > ambient_c)
///
/// # Errors
/// Returns an error for non-finite inputs or inputs outside the equation's domain.
pub fn fusing_current(
    area_circular_mils: f64,
    time_s: f64,
    ambient_c: f64,
    melting_temp_c: f64,
) -> Result<f64, CalcError> {
    validate::positive_quantity("area_circular_mils", area_circular_mils)?;
    validate::positive_quantity("time_s", time_s)?;
    validate::finite("ambient_c", ambient_c)?;
    validate::finite("melting_temp_c", melting_temp_c)?;
    if ambient_c <= -234.0 {
        return Err(CalcError::OutOfRange {
            name: "ambient_c",
            value: ambient_c,
            expected: "> −234 °C (resistivity model domain)",
        });
    }
    if melting_temp_c <= ambient_c {
        return Err(CalcError::OutOfRange {
            name: "melting_temp_c",
            value: melting_temp_c,
            expected: "> ambient_c",
        });
    }

    let delta_t = melting_temp_c - ambient_c;
    let log_term = (1.0 + delta_t / (234.0 + ambient_c)).log10();
    validate::finite_result(
        "fusing_current",
        area_circular_mils * (log_term / (33.0 * time_s)).sqrt(),
    )
}

/// Calculate the fusing current for a PCB trace from physical dimensions.
///
/// # Errors
/// Returns an error for invalid dimensions, an impossible etched
/// cross-section, or inputs outside the equation's domain.
pub fn fusing_current_trace(
    width_mils: f64,
    base_copper: CopperWeight,
    plating: PlatingThickness,
    etch_factor: EtchFactor,
    time_s: f64,
    ambient_c: f64,
) -> Result<FusingResult, CalcError> {
    let copper_thickness_mils = base_copper.thickness_mils() + plating.thickness_mils();
    let area_sq_mils = etch_factor.cross_section_sq_mils(width_mils, copper_thickness_mils)?;
    let area_circular_mils = area_sq_mils * constants::FOUR_OVER_PI;

    let fusing_current_a =
        fusing_current(area_circular_mils, time_s, ambient_c, COPPER_MELTING_TEMP_C)?;

    Ok(FusingResult {
        copper_thickness_mils,
        area_sq_mils,
        area_circular_mils,
        fusing_current_a,
        melting_temp_c: COPPER_MELTING_TEMP_C,
    })
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    // Saturn PDF page 16: A_circ=23.93, t=1s, Ta=22°C → I=3.5147 A
    #[test]
    fn saturn_fusing_current_from_area() {
        let current = fusing_current(23.93, 1.0, 22.0, COPPER_MELTING_TEMP_C).unwrap();
        assert_relative_eq!(current, 3.5147, max_relative = 0.001);
    }

    #[test]
    fn shared_constant_is_used() {
        assert_relative_eq!(COPPER_MELTING_TEMP_C, 1084.62, epsilon = 1e-9);
        assert_relative_eq!(
            COPPER_MELTING_TEMP_C,
            constants::COPPER_MELTING_POINT_C,
            epsilon = 0.0
        );
    }

    // Full trace calculation with 1.5oz bare, 2:1 etch, W=10 mil, t=1s, Ta=22°C
    #[test]
    fn saturn_full_trace_oz15_bare() {
        let result = fusing_current_trace(
            10.0,
            CopperWeight::Oz15,
            PlatingThickness::Bare,
            EtchFactor::TwoToOne,
            1.0,
            22.0,
        )
        .unwrap();
        assert_relative_eq!(result.copper_thickness_mils, 2.10, epsilon = 1e-10);
        assert_relative_eq!(result.area_sq_mils, 18.795, epsilon = 0.01);
        assert_relative_eq!(result.area_circular_mils, 23.93, epsilon = 0.02);
        assert_relative_eq!(result.fusing_current_a, 3.5147, max_relative = 0.001);
    }

    #[test]
    fn domain_errors() {
        assert!(fusing_current(23.93, 0.0, 22.0, COPPER_MELTING_TEMP_C).is_err());
        assert!(fusing_current(23.93, 1.0, 1100.0, COPPER_MELTING_TEMP_C).is_err());
        // −250 °C satisfies "melting above ambient" but breaks the logarithm (audit A24).
        assert!(fusing_current(23.93, 1.0, -250.0, COPPER_MELTING_TEMP_C).is_err());
        assert!(fusing_current(f64::NAN, 1.0, 22.0, COPPER_MELTING_TEMP_C).is_err());
        // Impossible trapezoid.
        assert!(
            fusing_current_trace(
                1.0,
                CopperWeight::Oz1,
                PlatingThickness::Bare,
                EtchFactor::OneToOne,
                1.0,
                22.0
            )
            .is_err()
        );
    }
}
