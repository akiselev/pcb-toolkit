//! Conductor current capacity calculator.
//!
//! Supports:
//! - IPC-2221A (legacy empirical formula)
//! - IPC-2152 (with modifier charts — table data needed)
//!
//! Also computes: DC resistance, voltage drop, power dissipation,
//! skin depth, current density.
//!
//! # TODO
//! - IPC-2152 table data for more accurate results
//! - Solve-for-width mode (given target current, find required width)
//! - Temperature-adjusted resistivity
//! - AC resistance model (skin effect + proximity)

use serde::{Deserialize, Serialize};

use crate::CalcError;
use crate::copper::EtchFactor;

/// IPC-2221A constant for external (surface) layers.
const K_EXTERNAL: f64 = 0.048;

/// IPC-2221A constant for internal layers.
const K_INTERNAL: f64 = 0.024;

/// Copper resistivity in ohm-mil units (6.787e-4 Ω·mil).
///
/// Derived from 1.724e-6 Ω·cm × (1 cm / 393.7 mil) = 6.787e-7 Ω·mil …
/// but for R = ρL/A with L in mils and A in mil², we need the factor as shown.
/// Saturn uses R = 6.787e-4 × L / A for L,A in mils/mil².
const RESISTIVITY_OHM_MIL: f64 = 6.787e-4;

/// Copper resistivity in Ω·m (used for skin depth calculation).
const COPPER_RESISTIVITY_OHM_M: f64 = 1.724e-8;

/// Permeability of free space µ₀ (H/m).
const MU_0: f64 = 1.256_637_061_435_9e-6;

/// Inputs for conductor current capacity calculation.
pub struct CurrentInput {
    /// Trace width (mils).
    pub width: f64,
    /// Copper thickness (mils).
    pub thickness: f64,
    /// Trace length (mils). Used for resistance and voltage drop.
    pub length: f64,
    /// Allowed temperature rise above ambient (°C).
    pub temperature_rise: f64,
    /// Ambient temperature (°C). Not currently used in IPC-2221A, reserved.
    pub ambient_temp: f64,
    /// Frequency (Hz). Used for skin depth calculation. 0 = DC only.
    pub frequency: f64,
    /// Etch factor affecting cross-section geometry.
    pub etch_factor: EtchFactor,
    /// Whether the trace is on an internal layer (halves the current capacity).
    pub is_internal: bool,
}

/// Result of a conductor current capacity calculation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentResult {
    /// Maximum current capacity (A) per IPC-2221A.
    pub current_capacity: f64,
    /// Conductor cross-sectional area (sq mils).
    pub cross_section: f64,
    /// DC resistance (Ohms).
    pub resistance_dc: f64,
    /// Voltage drop at maximum current (V).
    pub voltage_drop: f64,
    /// Power dissipation at maximum current (W).
    pub power_dissipation: f64,
    /// Current density at maximum current (A/mil²).
    pub current_density: f64,
    /// Skin depth at the given frequency (mils). 0 if frequency is 0.
    pub skin_depth_mils: f64,
}

/// Calculate conductor current capacity and related electrical properties.
///
/// Uses the IPC-2221A empirical formula:
///   I = k × ΔT^0.44 × A^0.725
/// where k = 0.048 (external) or 0.024 (internal), ΔT in °C, A in mil².
pub fn calculate(input: &CurrentInput) -> Result<CurrentResult, CalcError> {
    let CurrentInput {
        width,
        thickness,
        length,
        temperature_rise,
        etch_factor,
        is_internal,
        frequency,
        ..
    } = *input;

    if width <= 0.0 {
        return Err(CalcError::NegativeDimension { name: "width", value: width });
    }
    if thickness <= 0.0 {
        return Err(CalcError::NegativeDimension { name: "thickness", value: thickness });
    }
    if length <= 0.0 {
        return Err(CalcError::NegativeDimension { name: "length", value: length });
    }
    if temperature_rise <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "temperature_rise",
            value: temperature_rise,
            expected: "> 0",
        });
    }
    if frequency < 0.0 {
        return Err(CalcError::OutOfRange {
            name: "frequency",
            value: frequency,
            expected: ">= 0",
        });
    }

    // Cross-sectional area (sq mils), accounting for etch factor
    let cross_section = etch_factor.cross_section_sq_mils(width, thickness);

    // IPC-2221A current capacity
    let k = if is_internal { K_INTERNAL } else { K_EXTERNAL };
    let current_capacity = k * temperature_rise.powf(0.44) * cross_section.powf(0.725);

    // DC resistance: R = ρ × L / A  (in ohm-mil units)
    let resistance_dc = RESISTIVITY_OHM_MIL * length / cross_section;

    // Voltage drop and power at max current
    let voltage_drop = current_capacity * resistance_dc;
    let power_dissipation = current_capacity * voltage_drop;

    // Current density (A/mil²)
    let current_density = current_capacity / cross_section;

    // Skin depth: δ = √(ρ / (π × f × µ₀))
    let skin_depth_mils = if frequency > 0.0 {
        let delta_m = (COPPER_RESISTIVITY_OHM_M
            / (std::f64::consts::PI * frequency * MU_0))
            .sqrt();
        // Convert m to mils: 1 m = 1 / 2.54e-5 mils = 39370.079 mils
        delta_m / crate::constants::MIL_TO_M
    } else {
        0.0
    };

    Ok(CurrentResult {
        current_capacity,
        cross_section,
        resistance_dc,
        voltage_drop,
        power_dissipation,
        current_density,
        skin_depth_mils,
    })
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    // Saturn test vector: 1MHz skin depth = 2.599 mil
    #[test]
    fn skin_depth_1mhz() {
        let result = calculate(&CurrentInput {
            width: 10.0,
            thickness: 1.4,
            length: 1000.0,
            temperature_rise: 10.0,
            ambient_temp: 25.0,
            frequency: 1_000_000.0,
            etch_factor: EtchFactor::None,
            is_internal: false,
        })
        .unwrap();

        assert_relative_eq!(result.skin_depth_mils, 2.599, max_relative = 0.005);
    }

    // IPC-2221A formula verification [docs/notes/17-test-vectors.md]:
    // External, dT=10°C, A=100 sq.mils → I = 0.048 × 10^0.44 × 100^0.725 = 3.73 A
    //
    // Note: Saturn PDF page 6/46 vectors use IPC-2152 mode (5.076A / 3.723A),
    // which requires table data not yet implemented.
    #[test]
    fn ipc2221a_external_a100() {
        // W=50, T=2.0, no etch → A = 100 sq mils
        let result = calculate(&CurrentInput {
            width: 50.0,
            thickness: 2.0,
            length: 1000.0,
            temperature_rise: 10.0,
            ambient_temp: 25.0,
            frequency: 0.0,
            etch_factor: EtchFactor::None,
            is_internal: false,
        })
        .unwrap();

        assert_relative_eq!(result.cross_section, 100.0, max_relative = 1e-10);
        assert_relative_eq!(result.current_capacity, 3.73, max_relative = 0.005);
    }

    // IPC-2221A internal: same area → I = 0.024 × 10^0.44 × 100^0.725 = 1.86 A
    #[test]
    fn ipc2221a_internal_a100() {
        let result = calculate(&CurrentInput {
            width: 50.0,
            thickness: 2.0,
            length: 1000.0,
            temperature_rise: 10.0,
            ambient_temp: 25.0,
            frequency: 0.0,
            etch_factor: EtchFactor::None,
            is_internal: true,
        })
        .unwrap();

        assert_relative_eq!(result.current_capacity, 1.86, max_relative = 0.005);
    }

    // Internal layers use k=0.024 → half the current of external
    #[test]
    fn internal_lower_than_external() {
        let ext = calculate(&CurrentInput {
            width: 10.0,
            thickness: 1.4,
            length: 1000.0,
            temperature_rise: 10.0,
            ambient_temp: 25.0,
            frequency: 0.0,
            etch_factor: EtchFactor::None,
            is_internal: false,
        })
        .unwrap();

        let int = calculate(&CurrentInput {
            width: 10.0,
            thickness: 1.4,
            length: 1000.0,
            temperature_rise: 10.0,
            ambient_temp: 25.0,
            frequency: 0.0,
            etch_factor: EtchFactor::None,
            is_internal: true,
        })
        .unwrap();

        // k_int / k_ext = 0.024 / 0.048 = 0.5
        assert_relative_eq!(int.current_capacity / ext.current_capacity, 0.5, max_relative = 1e-10);
    }

    #[test]
    fn resistance_and_power() {
        let result = calculate(&CurrentInput {
            width: 10.0,
            thickness: 1.4,
            length: 1000.0,
            temperature_rise: 10.0,
            ambient_temp: 25.0,
            frequency: 0.0,
            etch_factor: EtchFactor::None,
            is_internal: false,
        })
        .unwrap();

        // R = 6.787e-4 × 1000 / 14 ≈ 0.04848 Ω
        let expected_r = RESISTIVITY_OHM_MIL * 1000.0 / 14.0;
        assert_relative_eq!(result.resistance_dc, expected_r, max_relative = 1e-10);

        // V = I × R
        assert_relative_eq!(
            result.voltage_drop,
            result.current_capacity * result.resistance_dc,
            max_relative = 1e-10
        );

        // P = I × V
        assert_relative_eq!(
            result.power_dissipation,
            result.current_capacity * result.voltage_drop,
            max_relative = 1e-10
        );
    }

    #[test]
    fn rejects_negative_width() {
        let result = calculate(&CurrentInput {
            width: -1.0,
            thickness: 1.4,
            length: 1000.0,
            temperature_rise: 10.0,
            ambient_temp: 25.0,
            frequency: 0.0,
            etch_factor: EtchFactor::None,
            is_internal: false,
        });
        assert!(result.is_err());
    }

    #[test]
    fn rejects_zero_temperature_rise() {
        let result = calculate(&CurrentInput {
            width: 10.0,
            thickness: 1.4,
            length: 1000.0,
            temperature_rise: 0.0,
            ambient_temp: 25.0,
            frequency: 0.0,
            etch_factor: EtchFactor::None,
            is_internal: false,
        });
        assert!(result.is_err());
    }
}
