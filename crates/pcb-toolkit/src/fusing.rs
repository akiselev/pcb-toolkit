//! Fusing current calculator (Onderdonk's equation).
//!
//! Calculates the current required to melt a copper conductor
//! in a given time period.
//!
//! Reference: Onderdonk's equation adapted for PCB traces.

use serde::{Deserialize, Serialize};

use crate::copper::{CopperWeight, EtchFactor, PlatingThickness};
use crate::CalcError;

/// Copper melting temperature in °C (pure copper, 1084.62 °C).
pub const COPPER_MELTING_TEMP_C: f64 = 1084.62;

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
/// I = A_circ × √( log₁₀(1 + (Tm − Ta) / (234 + Ta)) / (33 × t) )
///
/// # Arguments
/// - `area_circular_mils` — conductor cross-section in circular mils (must be > 0)
/// - `time_s` — pulse duration in seconds (must be > 0)
/// - `ambient_c` — ambient temperature in °C
/// - `melting_temp_c` — melting temperature of conductor in °C (must be > ambient_c)
///
/// # Errors
/// Returns [`CalcError::OutOfRange`] if inputs are invalid.
pub fn fusing_current(
    area_circular_mils: f64,
    time_s: f64,
    ambient_c: f64,
    melting_temp_c: f64,
) -> Result<f64, CalcError> {
    if area_circular_mils <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "area_circular_mils",
            value: area_circular_mils,
            expected: "> 0",
        });
    }
    if time_s <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "time_s",
            value: time_s,
            expected: "> 0",
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
    let current = area_circular_mils * (log_term / (33.0 * time_s)).sqrt();

    Ok(current)
}

/// Calculate the fusing current for a PCB trace from physical dimensions.
///
/// # Arguments
/// - `width_mils` — trace width in mils (must be > 0)
/// - `base_copper` — base copper weight
/// - `plating` — plating thickness (use `PlatingThickness::Bare` for bare board)
/// - `etch_factor` — etch profile affecting the cross-section shape
/// - `time_s` — pulse duration in seconds (must be > 0)
/// - `ambient_c` — ambient temperature in °C
///
/// # Errors
/// Returns [`CalcError::OutOfRange`] or [`CalcError::NegativeDimension`] if inputs are invalid.
pub fn fusing_current_trace(
    width_mils: f64,
    base_copper: CopperWeight,
    plating: PlatingThickness,
    etch_factor: EtchFactor,
    time_s: f64,
    ambient_c: f64,
) -> Result<FusingResult, CalcError> {
    if width_mils <= 0.0 {
        return Err(CalcError::NegativeDimension {
            name: "width_mils",
            value: width_mils,
        });
    }

    let copper_thickness_mils =
        base_copper.thickness_mils() + plating.thickness_mils();
    let area_sq_mils = etch_factor.cross_section_sq_mils(width_mils, copper_thickness_mils);
    let area_circular_mils = area_sq_mils * (4.0 / std::f64::consts::PI);

    let fusing_current_a = fusing_current(
        area_circular_mils,
        time_s,
        ambient_c,
        COPPER_MELTING_TEMP_C,
    )?;

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
    // Copper melting point: 1084.62 °C (correct physical value; the spec note of
    // 1064.62 is a transcription error — reverse-engineering confirms 1084.62).
    #[test]
    fn saturn_fusing_current_from_area() {
        let current = fusing_current(23.93, 1.0, 22.0, COPPER_MELTING_TEMP_C).unwrap();
        assert_relative_eq!(current, 3.5147, max_relative = 0.001);
    }

    // Verify cross-section math: A_sq=18.79 ↔ A_circ=23.93
    #[test]
    fn circular_mils_conversion() {
        let a_sq = 18.79_f64;
        let a_circ = a_sq * (4.0 / std::f64::consts::PI);
        assert_relative_eq!(a_circ, 23.93, epsilon = 0.01);
    }

    // Verify EtchFactor::TwoToOne geometry: W=10, T=2.10 → A_sq≈18.79
    #[test]
    fn etch_two_to_one_area() {
        // T=2.10 mils is CopperWeight::Oz15 (1.5 oz) bare
        let area = EtchFactor::TwoToOne.cross_section_sq_mils(10.0, 2.10);
        assert_relative_eq!(area, 18.795, epsilon = 0.01);
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
    fn error_on_zero_time() {
        assert!(fusing_current(23.93, 0.0, 22.0, COPPER_MELTING_TEMP_C).is_err());
    }

    #[test]
    fn error_on_ambient_above_melting() {
        assert!(fusing_current(23.93, 1.0, 1100.0, COPPER_MELTING_TEMP_C).is_err());
    }
}
