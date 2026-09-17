//! Conductor current capacity calculator.
//!
//! * [`calculate`] — IPC-2221A power-law formula (legacy; the basis of most
//!   trace-width calculators and of Saturn's "IPC-2221" mode).
//! * [`calculate_ipc2152_estimate`] — an **experimental** estimate that
//!   scales the IPC-2221A result with modifier curves reconstructed from
//!   Saturn PCB Toolkit. It is **not** an implementation of the IPC-2152
//!   charts and must not be presented as one; see the function docs.
//!
//! Both also compute DC resistance (temperature corrected), voltage drop,
//! power dissipation, current density and skin depth.

use serde::{Deserialize, Serialize};

use crate::copper::EtchFactor;
use crate::model::{ModelInfo, ModelStatus};
use crate::tables::interpolate::lerp;
use crate::{CalcError, constants, validate};

/// IPC-2221A model description.
pub const MODEL: ModelInfo = ModelInfo {
    name: "IPC-2221A conductor current capacity, I = k·ΔT^0.44·A^0.725",
    status: ModelStatus::Compatibility,
    reference: "IPC-2221A (2003) §6.2, Fig. 6-4; Saturn PCB Toolkit IPC-2221 mode",
    validity: "Published chart range: A ≤ ~700 mil², ΔT 10–100 °C, external k = 0.048, internal k = 0.024; ±10–20% vs. IPC-2152 data",
};

/// IPC-2152 estimate model description.
pub const MODEL_IPC2152_ESTIMATE: ModelInfo = ModelInfo {
    name: "IPC-2221A scaled by Saturn-reconstructed area/ΔT/board modifiers (IPC-2152-style estimate)",
    status: ModelStatus::Experimental,
    reference: "Saturn PCB Toolkit v8.44 modifier tables (partially extracted); not IPC-2152 chart data",
    validity: "Estimate only. Modifier curves are linearly interpolated between reconstructed points; no independent validation against IPC-2152 charts",
};

/// IPC-2221A constant for external (surface) layers.
const K_EXTERNAL: f64 = 0.048;

/// IPC-2221A constant for internal layers.
const K_INTERNAL: f64 = 0.024;

/// Inputs for conductor current capacity calculation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CurrentInput {
    /// Trace width (mils).
    pub width: f64,
    /// Copper thickness (mils).
    pub thickness: f64,
    /// Trace length (mils). Used for resistance and voltage drop.
    pub length: f64,
    /// Allowed temperature rise above ambient (°C).
    pub temperature_rise: f64,
    /// Ambient (board) temperature (°C). Used for the resistance temperature
    /// correction; the IPC-2221A current formula itself depends only on ΔT.
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
    /// DC resistance (Ohms) at the ambient temperature.
    pub resistance_dc: f64,
    /// DC resistance (Ohms) at ambient + temperature rise (conductor operating temperature).
    pub resistance_dc_at_rise: f64,
    /// Voltage drop at maximum current (V), using the operating-temperature resistance.
    pub voltage_drop: f64,
    /// Power dissipation at maximum current (W), using the operating-temperature resistance.
    pub power_dissipation: f64,
    /// Current density at maximum current (A/mil²).
    pub current_density: f64,
    /// Skin depth at the given frequency (mils). 0 if frequency is 0.
    pub skin_depth_mils: f64,
}

/// Copper resistivity in Ω·mil at temperature `temp_c`, from the 20 °C value
/// and the linear temperature coefficient (applied exactly once).
pub fn copper_resistivity_ohm_mil(temp_c: f64) -> Result<f64, CalcError> {
    validate::finite("temperature", temp_c)?;
    let factor =
        1.0 + constants::COPPER_TEMP_COEFF * (temp_c - constants::COPPER_RESISTIVITY_REF_C);
    if factor <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "temperature",
            value: temp_c,
            expected: "> −234 °C (linear resistivity model)",
        });
    }
    Ok(constants::COPPER_RESISTIVITY_OHM_MIL * factor)
}

/// Skin depth in copper (mils) at `frequency` Hz; 0 for DC.
pub fn skin_depth_mils(frequency: f64) -> Result<f64, CalcError> {
    validate::non_negative("frequency", frequency)?;
    if frequency == 0.0 {
        return Ok(0.0);
    }
    let delta_m = (constants::COPPER_RESISTIVITY_OHM_M
        / (std::f64::consts::PI * frequency * constants::MU_0))
        .sqrt();
    validate::finite_result("skin_depth", delta_m / constants::MIL_TO_M)
}

struct Common {
    cross_section: f64,
    resistance_dc: f64,
    resistance_dc_at_rise: f64,
    skin_depth_mils: f64,
}

fn common(
    width: f64,
    thickness: f64,
    length: f64,
    temperature_rise: f64,
    ambient_temp: f64,
    frequency: f64,
    etch_factor: EtchFactor,
) -> Result<Common, CalcError> {
    validate::positive("length", length)?;
    validate::positive_quantity("temperature_rise", temperature_rise)?;
    validate::finite("ambient_temp", ambient_temp)?;
    let cross_section = etch_factor.cross_section_sq_mils(width, thickness)?;
    let rho_ambient = copper_resistivity_ohm_mil(ambient_temp)?;
    let rho_hot = copper_resistivity_ohm_mil(ambient_temp + temperature_rise)?;
    Ok(Common {
        cross_section,
        resistance_dc: validate::finite_result(
            "resistance_dc",
            rho_ambient * length / cross_section,
        )?,
        resistance_dc_at_rise: validate::finite_result(
            "resistance_dc_at_rise",
            rho_hot * length / cross_section,
        )?,
        skin_depth_mils: skin_depth_mils(frequency)?,
    })
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
        ambient_temp,
        frequency,
        etch_factor,
        is_internal,
    } = *input;

    let c = common(
        width,
        thickness,
        length,
        temperature_rise,
        ambient_temp,
        frequency,
        etch_factor,
    )?;

    let k = if is_internal { K_INTERNAL } else { K_EXTERNAL };
    let current_capacity = validate::finite_result(
        "current_capacity",
        k * temperature_rise.powf(0.44) * c.cross_section.powf(0.725),
    )?;

    let voltage_drop = current_capacity * c.resistance_dc_at_rise;
    Ok(CurrentResult {
        current_capacity,
        cross_section: c.cross_section,
        resistance_dc: c.resistance_dc,
        resistance_dc_at_rise: c.resistance_dc_at_rise,
        voltage_drop,
        power_dissipation: current_capacity * voltage_drop,
        current_density: current_capacity / c.cross_section,
        skin_depth_mils: c.skin_depth_mils,
    })
}

/// Inputs for the IPC-2152-style current capacity estimate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ipc2152Input {
    /// Trace width (mils).
    pub width: f64,
    /// Copper thickness (mils).
    pub thickness: f64,
    /// Trace length (mils).
    pub length: f64,
    /// Allowed temperature rise above ambient (°C).
    pub temperature_rise: f64,
    /// Ambient (board) temperature (°C).
    pub ambient_temp: f64,
    /// Frequency (Hz). 0 = DC only.
    pub frequency: f64,
    /// Etch factor.
    pub etch_factor: EtchFactor,
    /// Whether the trace is on an internal layer (applies the IPC-2221A
    /// factor of two; IPC-2152 itself uses a common baseline for both).
    pub is_internal: bool,
    /// Board thickness (mils).
    pub board_thickness_mils: f64,
    /// Whether the board has an adjacent copper plane.
    pub has_copper_plane: bool,
    /// Material thermal conductivity modifier (default 1.0). Must be finite and > 0.
    pub material_modifier: f64,
    /// User-supplied modifier (default 1.0). Must be finite and > 0.
    pub user_modifier: f64,
}

/// Result of an IPC-2152-style estimate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ipc2152Result {
    /// Estimated current capacity with all modifiers (A).
    pub current_capacity: f64,
    /// Conductor cross-sectional area (sq mils).
    pub cross_section: f64,
    /// DC resistance (Ohms) at the ambient temperature.
    pub resistance_dc: f64,
    /// DC resistance (Ohms) at ambient + temperature rise.
    pub resistance_dc_at_rise: f64,
    /// Voltage drop at the estimated current (V), operating-temperature resistance.
    pub voltage_drop: f64,
    /// Power dissipation (W), operating-temperature resistance.
    pub power_dissipation: f64,
    /// Current density (A/mil²).
    pub current_density: f64,
    /// Skin depth (mils). 0 if frequency is 0.
    pub skin_depth_mils: f64,
    /// Area modifier applied.
    pub m_area: f64,
    /// Temperature rise modifier applied.
    pub m_temp: f64,
    /// Board thickness modifier applied.
    pub m_board: f64,
}

/// Temperature-rise modifier knots (ΔT °C, modifier), reconstructed from Saturn.
const M_TEMP: &[(f64, f64)] = &[
    (10.0, 0.40),
    (20.0, 0.48),
    (30.0, 0.58),
    (40.0, 0.67),
    (50.0, 0.75),
    (60.0, 0.85),
    (70.0, 0.95),
    (80.0, 1.00),
    (90.0, 1.10),
    (100.0, 1.20),
    (110.0, 1.30),
];

/// Board-thickness modifier knots with an adjacent plane (mils, modifier).
const M_BOARD_PLANE: &[(f64, f64)] = &[
    (10.0, 1.63),
    (20.0, 1.59),
    (30.0, 1.56),
    (40.0, 1.52),
    (50.0, 1.49),
    (60.0, 1.46),
    (70.0, 1.43),
    (80.0, 1.41),
    (90.0, 1.37),
    (100.0, 1.34),
    (110.0, 1.24),
];

/// Board-thickness modifier knots without an adjacent plane (mils, modifier).
const M_BOARD_NO_PLANE: &[(f64, f64)] = &[
    (10.0, 1.59),
    (20.0, 1.55),
    (30.0, 1.52),
    (40.0, 1.48),
    (50.0, 1.45),
    (60.0, 1.42),
    (70.0, 1.39),
    (80.0, 1.37),
    (90.0, 1.33),
    (100.0, 1.30),
    (110.0, 1.20),
];

fn m_temp_lookup(dt: f64) -> Result<f64, CalcError> {
    lerp(M_TEMP, dt)
}

fn m_board_lookup(thickness_mils: f64, has_plane: bool) -> Result<f64, CalcError> {
    lerp(
        if has_plane {
            M_BOARD_PLANE
        } else {
            M_BOARD_NO_PLANE
        },
        thickness_mils,
    )
}

/// Area modifier: piecewise power-law segments joined by linear blending
/// over ±5% of each segment boundary so the modifier is continuous.
fn m_area_lookup(area: f64) -> f64 {
    let seg = |a: f64| -> f64 {
        if a <= 20.0 {
            3.0364 * a.powf(-0.145)
        } else if a <= 60.0 {
            2.9143 * a.powf(-0.129)
        } else if a <= 100.0 {
            2.7877 * a.powf(-0.114)
        } else {
            2.801 * a.powf(-0.111)
        }
    };
    for boundary in [20.0, 60.0, 100.0] {
        let lo = boundary * 0.95;
        let hi = boundary * 1.05;
        if area > lo && area < hi {
            let t = (area - lo) / (hi - lo);
            return (1.0 - t) * seg(lo) + t * seg(hi);
        }
    }
    seg(area)
}

/// IPC-2152-style current capacity **estimate** (experimental).
///
/// Starts from the IPC-2221A power law and multiplies by area, temperature
/// rise and board-thickness modifiers reconstructed from Saturn PCB Toolkit.
/// The modifier data are partial and were not extracted from the IPC-2152
/// charts themselves; the result is an estimate with no established error
/// bound. Modifiers are interpolated so the estimate is continuous in every
/// input.
pub fn calculate_ipc2152_estimate(input: &Ipc2152Input) -> Result<Ipc2152Result, CalcError> {
    let Ipc2152Input {
        width,
        thickness,
        length,
        temperature_rise,
        ambient_temp,
        frequency,
        etch_factor,
        is_internal,
        board_thickness_mils,
        has_copper_plane,
        material_modifier,
        user_modifier,
    } = *input;

    validate::positive("board_thickness_mils", board_thickness_mils)?;
    validate::positive_quantity("material_modifier", material_modifier)?;
    validate::positive_quantity("user_modifier", user_modifier)?;

    let c = common(
        width,
        thickness,
        length,
        temperature_rise,
        ambient_temp,
        frequency,
        etch_factor,
    )?;

    let k = if is_internal { K_INTERNAL } else { K_EXTERNAL };
    let i_base = k * temperature_rise.powf(0.44) * c.cross_section.powf(0.725);

    let m_area = m_area_lookup(c.cross_section);
    let m_temp = m_temp_lookup(temperature_rise)?;
    let m_board = m_board_lookup(board_thickness_mils, has_copper_plane)?;

    let current_capacity = validate::finite_result(
        "current_capacity",
        i_base * m_area * m_temp * m_board * material_modifier * user_modifier,
    )?;

    let voltage_drop = current_capacity * c.resistance_dc_at_rise;
    Ok(Ipc2152Result {
        current_capacity,
        cross_section: c.cross_section,
        resistance_dc: c.resistance_dc,
        resistance_dc_at_rise: c.resistance_dc_at_rise,
        voltage_drop,
        power_dissipation: current_capacity * voltage_drop,
        current_density: current_capacity / c.cross_section,
        skin_depth_mils: c.skin_depth_mils,
        m_area,
        m_temp,
        m_board,
    })
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    fn base() -> CurrentInput {
        CurrentInput {
            width: 10.0,
            thickness: 1.4,
            length: 1000.0,
            temperature_rise: 10.0,
            ambient_temp: 25.0,
            frequency: 0.0,
            etch_factor: EtchFactor::None,
            is_internal: false,
        }
    }

    fn ipc2152(ambient_temp: f64, temperature_rise: f64) -> Ipc2152Input {
        Ipc2152Input {
            width: 100.0,
            thickness: 1.0,
            length: 1000.0,
            temperature_rise,
            ambient_temp,
            frequency: 0.0,
            etch_factor: EtchFactor::None,
            is_internal: false,
            board_thickness_mils: 62.0,
            has_copper_plane: false,
            material_modifier: 1.0,
            user_modifier: 1.0,
        }
    }

    // Saturn test vector: 1MHz skin depth = 2.599 mil
    #[test]
    fn skin_depth_1mhz() {
        let result = calculate(&CurrentInput {
            frequency: 1e6,
            ..base()
        })
        .unwrap();
        assert_relative_eq!(result.skin_depth_mils, 2.599, max_relative = 0.005);
    }

    // External, dT=10°C, A=100 sq.mils → I = 0.048 × 10^0.44 × 100^0.725 = 3.73 A
    #[test]
    fn ipc2221a_external_a100() {
        let result = calculate(&CurrentInput {
            width: 50.0,
            thickness: 2.0,
            ..base()
        })
        .unwrap();
        assert_relative_eq!(result.cross_section, 100.0, max_relative = 1e-10);
        assert_relative_eq!(result.current_capacity, 3.73, max_relative = 0.005);
    }

    #[test]
    fn ipc2221a_internal_a100() {
        let result = calculate(&CurrentInput {
            width: 50.0,
            thickness: 2.0,
            is_internal: true,
            ..base()
        })
        .unwrap();
        assert_relative_eq!(result.current_capacity, 1.86, max_relative = 0.005);
    }

    #[test]
    fn internal_lower_than_external() {
        let ext = calculate(&base()).unwrap();
        let int = calculate(&CurrentInput {
            is_internal: true,
            ..base()
        })
        .unwrap();
        assert_relative_eq!(
            int.current_capacity / ext.current_capacity,
            0.5,
            max_relative = 1e-10
        );
    }

    #[test]
    fn resistance_uses_ambient_temperature_once() {
        // 20 °C ambient: R = 6.787e-4 × 1000 / 14.
        let r20 = calculate(&CurrentInput {
            ambient_temp: 20.0,
            ..base()
        })
        .unwrap();
        assert_relative_eq!(
            r20.resistance_dc,
            6.787e-4 * 1000.0 / 14.0,
            max_relative = 1e-4
        );
        // 60 °C ambient: one linear correction, 15.7% higher.
        let r60 = calculate(&CurrentInput {
            ambient_temp: 60.0,
            ..base()
        })
        .unwrap();
        assert_relative_eq!(
            r60.resistance_dc,
            r20.resistance_dc * (1.0 + 0.00393 * 40.0),
            max_relative = 1e-12
        );
        // Operating temperature adds the rise.
        assert_relative_eq!(
            r60.resistance_dc_at_rise,
            r20.resistance_dc * (1.0 + 0.00393 * 50.0),
            max_relative = 1e-12
        );
        // V = I × R_hot, P = I × V.
        assert_relative_eq!(
            r60.voltage_drop,
            r60.current_capacity * r60.resistance_dc_at_rise,
            max_relative = 1e-12
        );
        assert_relative_eq!(
            r60.power_dissipation,
            r60.current_capacity * r60.voltage_drop,
            max_relative = 1e-12
        );
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert!(
            calculate(&CurrentInput {
                width: -1.0,
                ..base()
            })
            .is_err()
        );
        assert!(
            calculate(&CurrentInput {
                width: f64::NAN,
                ..base()
            })
            .is_err()
        );
        assert!(
            calculate(&CurrentInput {
                temperature_rise: 0.0,
                ..base()
            })
            .is_err()
        );
        assert!(
            calculate(&CurrentInput {
                ambient_temp: f64::INFINITY,
                ..base()
            })
            .is_err()
        );
        assert!(
            calculate(&CurrentInput {
                ambient_temp: -300.0,
                ..base()
            })
            .is_err()
        );
        assert!(
            calculate(&CurrentInput {
                frequency: -1.0,
                ..base()
            })
            .is_err()
        );
        // Impossible etched trapezoids (audit A08).
        assert!(
            calculate(&CurrentInput {
                width: 1.0,
                etch_factor: EtchFactor::OneToOne,
                ..base()
            })
            .is_err()
        );
        assert!(
            calculate(&CurrentInput {
                width: 2.0,
                etch_factor: EtchFactor::OneToOne,
                ..base()
            })
            .is_err()
        );
    }

    #[test]
    fn modifiers_are_interpolated_and_continuous() {
        assert_relative_eq!(m_temp_lookup(10.0).unwrap(), 0.40, epsilon = 1e-12);
        assert_relative_eq!(m_temp_lookup(15.0).unwrap(), 0.44, epsilon = 1e-12);
        assert_relative_eq!(m_temp_lookup(5.0).unwrap(), 0.40, epsilon = 1e-12);
        assert_relative_eq!(m_temp_lookup(200.0).unwrap(), 1.30, epsilon = 1e-12);
        assert_relative_eq!(m_board_lookup(10.0, false).unwrap(), 1.59, epsilon = 1e-12);
        assert_relative_eq!(
            m_board_lookup(62.0, false).unwrap(),
            1.42 + (1.39 - 1.42) * 0.2,
            epsilon = 1e-12
        );
        assert_relative_eq!(m_board_lookup(50.0, true).unwrap(), 1.49, epsilon = 1e-12);
        for a in [19.0, 20.0, 21.0, 57.0, 60.0, 63.0, 95.0, 100.0, 105.0] {
            let d = (m_area_lookup(a + 1e-6) - m_area_lookup(a - 1e-6)).abs();
            assert!(d < 1e-4, "m_area step at {a}: {d}");
        }
    }

    #[test]
    fn ipc2152_estimate_is_continuous_in_temperature_rise_and_ambient() {
        let a = calculate_ipc2152_estimate(&ipc2152(20.0, 10.0)).unwrap();
        let b = calculate_ipc2152_estimate(&ipc2152(20.0, 10.000001)).unwrap();
        assert_relative_eq!(b.current_capacity, a.current_capacity, max_relative = 1e-4);
        let c = calculate_ipc2152_estimate(&ipc2152(20.000001, 10.0)).unwrap();
        assert_relative_eq!(c.resistance_dc, a.resistance_dc, max_relative = 1e-5);
    }

    #[test]
    fn ipc2152_estimate_resistance_does_not_double_count_temperature() {
        let r = calculate_ipc2152_estimate(&ipc2152(60.0, 10.0)).unwrap();
        let expected = 6.787e-4 * (1.0 + 0.00393 * 40.0) * 1000.0 / 100.0;
        assert_relative_eq!(r.resistance_dc, expected, max_relative = 1e-4);
    }

    #[test]
    fn ipc2152_estimate_applies_modifiers() {
        let r = calculate_ipc2152_estimate(&Ipc2152Input {
            width: 50.0,
            thickness: 2.0,
            ambient_temp: 25.0,
            ..ipc2152(25.0, 10.0)
        })
        .unwrap();
        let i_base = K_EXTERNAL * 10.0_f64.powf(0.44) * 100.0_f64.powf(0.725);
        assert_relative_eq!(
            r.current_capacity,
            i_base * r.m_area * r.m_temp * r.m_board,
            max_relative = 1e-12
        );
        assert_relative_eq!(r.m_temp, 0.40, epsilon = 1e-12);
        assert!(r.m_area > 0.0 && r.m_board > 1.0);
    }

    #[test]
    fn ipc2152_estimate_rejects_bad_modifiers() {
        assert!(
            calculate_ipc2152_estimate(&Ipc2152Input {
                user_modifier: -1.0,
                ..ipc2152(25.0, 10.0)
            })
            .is_err()
        );
        assert!(
            calculate_ipc2152_estimate(&Ipc2152Input {
                material_modifier: f64::NAN,
                ..ipc2152(25.0, 10.0)
            })
            .is_err()
        );
        assert!(
            calculate_ipc2152_estimate(&Ipc2152Input {
                board_thickness_mils: 0.0,
                ..ipc2152(25.0, 10.0)
            })
            .is_err()
        );
    }
}
