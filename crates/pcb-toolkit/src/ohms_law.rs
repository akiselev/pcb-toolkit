//! Ohm's law and basic electrical calculators.
//!
//! Sub-calculators:
//! 1. E-I-R (V = IR, P = VI)
//! 2. LED Bias resistor
//! 3. Resistor series/parallel
//! 4. Pi-pad attenuator
//! 5. T-pad attenuator
//! 6. Capacitor series/parallel
//! 7. Inductor series/parallel

use serde::{Deserialize, Serialize};

use crate::CalcError;

// ---------------------------------------------------------------------------
// E-I-R
// ---------------------------------------------------------------------------

/// Result of an E-I-R (voltage-current-resistance) calculation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EirResult {
    /// Voltage in Volts.
    pub voltage_v: f64,
    /// Current in Amperes.
    pub current_a: f64,
    /// Resistance in Ohms.
    pub resistance_ohm: f64,
    /// Power in Watts (P = V × I).
    pub power_w: f64,
}

/// Calculate voltage, current, resistance, and power given any two of V, I, R.
///
/// Exactly two of the three options must be `Some`.
///
/// # Errors
/// Returns [`CalcError::InsufficientInputs`] if fewer or more than two values are provided,
/// or [`CalcError::OutOfRange`] if a zero denominator would result.
pub fn eir(
    voltage_v: Option<f64>,
    current_a: Option<f64>,
    resistance_ohm: Option<f64>,
) -> Result<EirResult, CalcError> {
    let provided = [voltage_v, current_a, resistance_ohm]
        .iter()
        .filter(|v| v.is_some())
        .count();
    if provided != 2 {
        return Err(CalcError::InsufficientInputs(
            "exactly 2 of voltage_v, current_a, resistance_ohm must be provided",
        ));
    }

    let (v, i, r) = match (voltage_v, current_a, resistance_ohm) {
        (Some(v), Some(i), None) => {
            if i == 0.0 {
                return Err(CalcError::OutOfRange {
                    name: "current_a",
                    value: i,
                    expected: "!= 0 when computing resistance",
                });
            }
            (v, i, v / i)
        }
        (Some(v), None, Some(r)) => {
            if r == 0.0 {
                return Err(CalcError::OutOfRange {
                    name: "resistance_ohm",
                    value: r,
                    expected: "!= 0 when computing current",
                });
            }
            (v, v / r, r)
        }
        (None, Some(i), Some(r)) => (i * r, i, r),
        _ => unreachable!(),
    };

    Ok(EirResult {
        voltage_v: v,
        current_a: i,
        resistance_ohm: r,
        power_w: v * i,
    })
}

// ---------------------------------------------------------------------------
// LED bias resistor
// ---------------------------------------------------------------------------

/// Result of an LED bias resistor calculation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LedBiasResult {
    /// Required series resistor value in Ohms.
    pub resistance_ohm: f64,
    /// Power dissipated by the resistor in Watts.
    pub power_w: f64,
}

/// Calculate LED bias resistor.
///
/// R = (Vs − Vled) / Iled
///
/// # Arguments
/// - `supply_v` — supply voltage in Volts (must be > led_v)
/// - `led_v` — LED forward voltage in Volts (must be > 0)
/// - `led_current_a` — desired LED current in Amperes (must be > 0)
///
/// # Errors
/// Returns [`CalcError::OutOfRange`] if inputs are invalid.
pub fn led_bias(supply_v: f64, led_v: f64, led_current_a: f64) -> Result<LedBiasResult, CalcError> {
    if led_v <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "led_v",
            value: led_v,
            expected: "> 0",
        });
    }
    if supply_v <= led_v {
        return Err(CalcError::OutOfRange {
            name: "supply_v",
            value: supply_v,
            expected: "> led_v",
        });
    }
    if led_current_a <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "led_current_a",
            value: led_current_a,
            expected: "> 0",
        });
    }

    let v_drop = supply_v - led_v;
    let resistance_ohm = v_drop / led_current_a;
    let power_w = v_drop * led_current_a;

    Ok(LedBiasResult {
        resistance_ohm,
        power_w,
    })
}

// ---------------------------------------------------------------------------
// Resistor combinations
// ---------------------------------------------------------------------------

/// Result of a resistor series/parallel combination.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResistorCombinationResult {
    /// Combined resistance in Ohms.
    pub resistance_ohm: f64,
}

/// Sum resistors in series.
///
/// R_total = R1 + R2 + … + Rn
pub fn resistors_series(values: &[f64]) -> Result<ResistorCombinationResult, CalcError> {
    if values.is_empty() {
        return Err(CalcError::InsufficientInputs("at least one resistor required"));
    }
    Ok(ResistorCombinationResult {
        resistance_ohm: values.iter().sum(),
    })
}

/// Combine resistors in parallel.
///
/// 1/R_total = 1/R1 + 1/R2 + … + 1/Rn
pub fn resistors_parallel(values: &[f64]) -> Result<ResistorCombinationResult, CalcError> {
    if values.is_empty() {
        return Err(CalcError::InsufficientInputs("at least one resistor required"));
    }
    for (idx, &r) in values.iter().enumerate() {
        if r == 0.0 {
            return Err(CalcError::OutOfRange {
                name: "resistor value",
                value: r,
                expected: "!= 0 for parallel combination",
            });
        }
        let _ = idx;
    }
    let reciprocal_sum: f64 = values.iter().map(|r| 1.0 / r).sum();
    Ok(ResistorCombinationResult {
        resistance_ohm: 1.0 / reciprocal_sum,
    })
}

// ---------------------------------------------------------------------------
// Capacitor combinations
// ---------------------------------------------------------------------------

/// Result of a capacitor series/parallel combination.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapacitorCombinationResult {
    /// Combined capacitance in Farads.
    pub capacitance_f: f64,
}

/// Sum capacitors in parallel (C_total = C1 + C2 + … + Cn).
pub fn capacitors_parallel(values: &[f64]) -> Result<CapacitorCombinationResult, CalcError> {
    if values.is_empty() {
        return Err(CalcError::InsufficientInputs("at least one capacitor required"));
    }
    Ok(CapacitorCombinationResult {
        capacitance_f: values.iter().sum(),
    })
}

/// Combine capacitors in series (1/C_total = 1/C1 + 1/C2 + … + 1/Cn).
pub fn capacitors_series(values: &[f64]) -> Result<CapacitorCombinationResult, CalcError> {
    if values.is_empty() {
        return Err(CalcError::InsufficientInputs("at least one capacitor required"));
    }
    for &c in values.iter() {
        if c == 0.0 {
            return Err(CalcError::OutOfRange {
                name: "capacitor value",
                value: c,
                expected: "!= 0 for series combination",
            });
        }
    }
    let reciprocal_sum: f64 = values.iter().map(|c| 1.0 / c).sum();
    Ok(CapacitorCombinationResult {
        capacitance_f: 1.0 / reciprocal_sum,
    })
}

// ---------------------------------------------------------------------------
// Inductor combinations
// ---------------------------------------------------------------------------

/// Result of an inductor series/parallel combination.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InductorCombinationResult {
    /// Combined inductance in Henries.
    pub inductance_h: f64,
}

/// Sum inductors in series (L_total = L1 + L2 + … + Ln, no mutual coupling).
pub fn inductors_series(values: &[f64]) -> Result<InductorCombinationResult, CalcError> {
    if values.is_empty() {
        return Err(CalcError::InsufficientInputs("at least one inductor required"));
    }
    Ok(InductorCombinationResult {
        inductance_h: values.iter().sum(),
    })
}

/// Combine inductors in parallel (1/L_total = 1/L1 + … + 1/Ln, no mutual coupling).
pub fn inductors_parallel(values: &[f64]) -> Result<InductorCombinationResult, CalcError> {
    if values.is_empty() {
        return Err(CalcError::InsufficientInputs("at least one inductor required"));
    }
    for &l in values.iter() {
        if l == 0.0 {
            return Err(CalcError::OutOfRange {
                name: "inductor value",
                value: l,
                expected: "!= 0 for parallel combination",
            });
        }
    }
    let reciprocal_sum: f64 = values.iter().map(|l| 1.0 / l).sum();
    Ok(InductorCombinationResult {
        inductance_h: 1.0 / reciprocal_sum,
    })
}

// ---------------------------------------------------------------------------
// Attenuators
// ---------------------------------------------------------------------------

/// Result of a symmetric Pi-pad or T-pad attenuator calculation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttenuatorResult {
    /// Attenuation in dB.
    pub attenuation_db: f64,
    /// Voltage ratio K = 10^(dB/20).
    pub k: f64,
    /// Series resistor (Ω): two outer elements for Pi-pad, two outer elements for T-pad.
    pub r_series_ohm: f64,
    /// Shunt resistor (Ω): centre element for Pi-pad, centre element for T-pad.
    pub r_shunt_ohm: f64,
}

/// Calculate a symmetric Pi-pad attenuator.
///
/// Topology: Rshunt — Rseries — Rshunt (shunt to ground at input and output).
///
/// ```text
/// in ─┬── R_series ──┬─ out
///     R_shunt        R_shunt
///     GND            GND
/// ```
///
/// Formulas (symmetric, Z_in = Z_out = z_ohm):
/// - K = 10^(dB/20)
/// - R_shunt = Z × (K + 1) / (K − 1)
/// - R_series = Z × (K − 1) / (K + 1)
///
/// # Errors
/// Returns [`CalcError::OutOfRange`] if `attenuation_db` ≤ 0 or `z_ohm` ≤ 0.
pub fn pi_pad(attenuation_db: f64, z_ohm: f64) -> Result<AttenuatorResult, CalcError> {
    if attenuation_db <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "attenuation_db",
            value: attenuation_db,
            expected: "> 0",
        });
    }
    if z_ohm <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "z_ohm",
            value: z_ohm,
            expected: "> 0",
        });
    }

    let k = 10.0_f64.powf(attenuation_db / 20.0);
    let r_shunt_ohm = z_ohm * (k + 1.0) / (k - 1.0);
    let r_series_ohm = z_ohm * (k - 1.0) / (k + 1.0);

    Ok(AttenuatorResult {
        attenuation_db,
        k,
        r_series_ohm,
        r_shunt_ohm,
    })
}

/// Calculate a symmetric T-pad attenuator.
///
/// Topology: Rseries — Rshunt — Rseries (shunt to ground in the centre).
///
/// ```text
/// in ── R_series ──┬── R_series ── out
///                  R_shunt
///                  GND
/// ```
///
/// Formulas (symmetric, Z_in = Z_out = z_ohm):
/// - K = 10^(dB/20)
/// - R_series = Z × (K − 1) / (K + 1)
/// - R_shunt  = Z × 2K / (K² − 1)
///
/// # Errors
/// Returns [`CalcError::OutOfRange`] if `attenuation_db` ≤ 0 or `z_ohm` ≤ 0.
pub fn t_pad(attenuation_db: f64, z_ohm: f64) -> Result<AttenuatorResult, CalcError> {
    if attenuation_db <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "attenuation_db",
            value: attenuation_db,
            expected: "> 0",
        });
    }
    if z_ohm <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "z_ohm",
            value: z_ohm,
            expected: "> 0",
        });
    }

    let k = 10.0_f64.powf(attenuation_db / 20.0);
    let r_series_ohm = z_ohm * (k - 1.0) / (k + 1.0);
    let r_shunt_ohm = z_ohm * 2.0 * k / (k * k - 1.0);

    Ok(AttenuatorResult {
        attenuation_db,
        k,
        r_series_ohm,
        r_shunt_ohm,
    })
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    // Saturn PDF page 21: V=12V, I=1A, R=12Ω → P=12W
    #[test]
    fn saturn_eir_vi() {
        let result = eir(Some(12.0), Some(1.0), None).unwrap();
        assert_relative_eq!(result.resistance_ohm, 12.0, epsilon = 1e-10);
        assert_relative_eq!(result.power_w, 12.0, epsilon = 1e-10);
    }

    #[test]
    fn eir_solve_voltage() {
        let result = eir(None, Some(1.0), Some(12.0)).unwrap();
        assert_relative_eq!(result.voltage_v, 12.0, epsilon = 1e-10);
        assert_relative_eq!(result.power_w, 12.0, epsilon = 1e-10);
    }

    // Saturn PDF page 21: LED: Vs=12V, Vled=2V, Iled=10mA → R=1000Ω, P=0.1W
    #[test]
    fn saturn_led_bias() {
        let result = led_bias(12.0, 2.0, 0.01).unwrap();
        assert_relative_eq!(result.resistance_ohm, 1000.0, epsilon = 1e-6);
        assert_relative_eq!(result.power_w, 0.1, epsilon = 1e-8);
    }

    // Pi-pad 1dB, 50Ω: R_series≈2.9Ω, R_shunt≈870Ω
    #[test]
    fn pi_pad_1db_50ohm() {
        let result = pi_pad(1.0, 50.0).unwrap();
        assert_relative_eq!(result.r_series_ohm, 2.88, epsilon = 0.01);
        assert_relative_eq!(result.r_shunt_ohm, 869.5, epsilon = 0.5);
    }

    // Pi-pad 10dB, 50Ω: R_series≈26.0Ω, R_shunt≈96.2Ω
    #[test]
    fn pi_pad_10db_50ohm() {
        let result = pi_pad(10.0, 50.0).unwrap();
        assert_relative_eq!(result.r_series_ohm, 25.97, epsilon = 0.02);
        assert_relative_eq!(result.r_shunt_ohm, 96.25, epsilon = 0.05);
    }

    // T-pad 10dB, 50Ω: R_series≈26.0Ω, R_shunt≈35.1Ω
    #[test]
    fn t_pad_10db_50ohm() {
        let result = t_pad(10.0, 50.0).unwrap();
        assert_relative_eq!(result.r_series_ohm, 25.97, epsilon = 0.02);
        assert_relative_eq!(result.r_shunt_ohm, 35.14, epsilon = 0.02);
    }

    #[test]
    fn resistors_series_two() {
        let result = resistors_series(&[100.0, 200.0]).unwrap();
        assert_relative_eq!(result.resistance_ohm, 300.0, epsilon = 1e-10);
    }

    #[test]
    fn resistors_parallel_two_equal() {
        let result = resistors_parallel(&[100.0, 100.0]).unwrap();
        assert_relative_eq!(result.resistance_ohm, 50.0, epsilon = 1e-10);
    }

    #[test]
    fn capacitors_parallel_two() {
        let result = capacitors_parallel(&[10e-12, 10e-12]).unwrap();
        assert_relative_eq!(result.capacitance_f, 20e-12, epsilon = 1e-22);
    }

    #[test]
    fn capacitors_series_two_equal() {
        let result = capacitors_series(&[10e-12, 10e-12]).unwrap();
        assert_relative_eq!(result.capacitance_f, 5e-12, epsilon = 1e-22);
    }

    #[test]
    fn inductors_series_two() {
        let result = inductors_series(&[1e-6, 2e-6]).unwrap();
        assert_relative_eq!(result.inductance_h, 3e-6, epsilon = 1e-16);
    }

    #[test]
    fn inductors_parallel_two_equal() {
        let result = inductors_parallel(&[10e-6, 10e-6]).unwrap();
        assert_relative_eq!(result.inductance_h, 5e-6, epsilon = 1e-16);
    }

    #[test]
    fn error_on_zero_attenuation() {
        assert!(pi_pad(0.0, 50.0).is_err());
        assert!(t_pad(0.0, 50.0).is_err());
    }

    #[test]
    fn error_on_insufficient_eir_inputs() {
        assert!(eir(Some(12.0), None, None).is_err());
        assert!(eir(Some(12.0), Some(1.0), Some(12.0)).is_err());
    }
}
