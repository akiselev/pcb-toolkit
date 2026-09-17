//! Ohm's law and basic electrical calculators.
//!
//! Sub-calculators:
//! 1. E-I-R (V = IR, P = VI)
//! 2. LED bias resistor
//! 3. Resistor series/parallel
//! 4. Pi-pad attenuator
//! 5. T-pad attenuator
//! 6. Capacitor series/parallel
//! 7. Inductor series/parallel
//!
//! Contract for passive combinations: every component value must be finite
//! and strictly positive. Negative or zero elements are rejected rather than
//! combined, so reciprocal sums can never hit a singularity.

use serde::{Deserialize, Serialize};

use crate::model::{ModelInfo, ModelStatus};
use crate::{CalcError, validate};

/// Model description.
pub const MODEL: ModelInfo = ModelInfo {
    name: "Ideal lumped-element relations (Ohm's law, matched symmetric attenuators)",
    status: ModelStatus::Validated,
    reference: "Exact circuit relations; attenuators verified with an ABCD-matrix oracle",
    validity: "Ideal resistors; attenuation > 0 dB; all component values finite and > 0",
};

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
/// Exactly two of the three options must be `Some`. Voltage and current may
/// carry either sign; resistance must be finite and non-zero when supplied.
///
/// # Errors
/// Returns [`CalcError::InsufficientInputs`] if fewer or more than two values are provided,
/// [`CalcError::NotFinite`] for non-finite values, or [`CalcError::OutOfRange`] if a zero
/// denominator would result.
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
    if let Some(v) = voltage_v {
        validate::finite("voltage_v", v)?;
    }
    if let Some(i) = current_a {
        validate::finite("current_a", i)?;
    }
    if let Some(r) = resistance_ohm {
        validate::finite("resistance_ohm", r)?;
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
        voltage_v: validate::finite_result("voltage_v", v)?,
        current_a: validate::finite_result("current_a", i)?,
        resistance_ohm: validate::finite_result("resistance_ohm", r)?,
        power_w: validate::finite_result("power_w", v * i)?,
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

/// Calculate LED bias resistor: R = (Vs − Vled) / Iled.
///
/// # Errors
/// Returns an error unless `led_v > 0`, `supply_v > led_v` and `led_current_a > 0`.
pub fn led_bias(supply_v: f64, led_v: f64, led_current_a: f64) -> Result<LedBiasResult, CalcError> {
    validate::positive_quantity("led_v", led_v)?;
    validate::finite("supply_v", supply_v)?;
    if supply_v <= led_v {
        return Err(CalcError::OutOfRange {
            name: "supply_v",
            value: supply_v,
            expected: "> led_v",
        });
    }
    validate::positive_quantity("led_current_a", led_current_a)?;

    let v_drop = supply_v - led_v;
    Ok(LedBiasResult {
        resistance_ohm: validate::finite_result("resistance_ohm", v_drop / led_current_a)?,
        power_w: validate::finite_result("power_w", v_drop * led_current_a)?,
    })
}

// ---------------------------------------------------------------------------
// Passive combinations
// ---------------------------------------------------------------------------

fn check_components(name: &'static str, values: &[f64]) -> Result<(), CalcError> {
    if values.is_empty() {
        return Err(CalcError::InsufficientInputs(
            "at least one component value required",
        ));
    }
    for &v in values {
        validate::positive_quantity(name, v)?;
    }
    Ok(())
}

fn sum(name: &'static str, values: &[f64]) -> Result<f64, CalcError> {
    check_components(name, values)?;
    validate::finite_result(name, values.iter().sum())
}

fn reciprocal_sum(name: &'static str, values: &[f64]) -> Result<f64, CalcError> {
    check_components(name, values)?;
    let recip: f64 = values.iter().map(|v| 1.0 / v).sum();
    validate::finite_result(name, 1.0 / recip)
}

/// Result of a resistor series/parallel combination.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResistorCombinationResult {
    /// Combined resistance in Ohms.
    pub resistance_ohm: f64,
}

/// Sum resistors in series: R_total = R1 + R2 + … + Rn.
pub fn resistors_series(values: &[f64]) -> Result<ResistorCombinationResult, CalcError> {
    Ok(ResistorCombinationResult {
        resistance_ohm: sum("resistor value", values)?,
    })
}

/// Combine resistors in parallel: 1/R_total = 1/R1 + … + 1/Rn.
pub fn resistors_parallel(values: &[f64]) -> Result<ResistorCombinationResult, CalcError> {
    Ok(ResistorCombinationResult {
        resistance_ohm: reciprocal_sum("resistor value", values)?,
    })
}

/// Result of a capacitor series/parallel combination.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapacitorCombinationResult {
    /// Combined capacitance in Farads.
    pub capacitance_f: f64,
}

/// Sum capacitors in parallel (C_total = C1 + C2 + … + Cn).
pub fn capacitors_parallel(values: &[f64]) -> Result<CapacitorCombinationResult, CalcError> {
    Ok(CapacitorCombinationResult {
        capacitance_f: sum("capacitor value", values)?,
    })
}

/// Combine capacitors in series (1/C_total = 1/C1 + … + 1/Cn).
pub fn capacitors_series(values: &[f64]) -> Result<CapacitorCombinationResult, CalcError> {
    Ok(CapacitorCombinationResult {
        capacitance_f: reciprocal_sum("capacitor value", values)?,
    })
}

/// Result of an inductor series/parallel combination.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InductorCombinationResult {
    /// Combined inductance in Henries.
    pub inductance_h: f64,
}

/// Sum inductors in series (no mutual coupling).
pub fn inductors_series(values: &[f64]) -> Result<InductorCombinationResult, CalcError> {
    Ok(InductorCombinationResult {
        inductance_h: sum("inductor value", values)?,
    })
}

/// Combine inductors in parallel (no mutual coupling).
pub fn inductors_parallel(values: &[f64]) -> Result<InductorCombinationResult, CalcError> {
    Ok(InductorCombinationResult {
        inductance_h: reciprocal_sum("inductor value", values)?,
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
    /// Series resistor (Ω): the single centre element of a Pi-pad, or each of
    /// the two outer elements of a T-pad.
    pub r_series_ohm: f64,
    /// Shunt resistor (Ω): each of the two outer elements of a Pi-pad, or the
    /// single centre element of a T-pad.
    pub r_shunt_ohm: f64,
}

fn attenuator_inputs(attenuation_db: f64, z_ohm: f64) -> Result<f64, CalcError> {
    validate::positive_quantity("attenuation_db", attenuation_db)?;
    validate::positive_quantity("z_ohm", z_ohm)?;
    Ok(10.0_f64.powf(attenuation_db / 20.0))
}

/// Calculate a symmetric Pi-pad attenuator.
///
/// ```text
/// in ─┬── R_series ──┬─ out
///     R_shunt        R_shunt
///     GND            GND
/// ```
///
/// Formulas (matched, Z_in = Z_out = Z):
/// - K = 10^(dB/20)
/// - R_shunt  = Z × (K + 1) / (K − 1)
/// - R_series = Z × (K² − 1) / (2K)
///
/// # Errors
/// Returns [`CalcError::OutOfRange`] if `attenuation_db` ≤ 0 or `z_ohm` ≤ 0.
pub fn pi_pad(attenuation_db: f64, z_ohm: f64) -> Result<AttenuatorResult, CalcError> {
    let k = attenuator_inputs(attenuation_db, z_ohm)?;
    Ok(AttenuatorResult {
        attenuation_db,
        k,
        r_series_ohm: validate::finite_result("r_series_ohm", z_ohm * (k * k - 1.0) / (2.0 * k))?,
        r_shunt_ohm: validate::finite_result("r_shunt_ohm", z_ohm * (k + 1.0) / (k - 1.0))?,
    })
}

/// Calculate a symmetric T-pad attenuator.
///
/// ```text
/// in ── R_series ──┬── R_series ── out
///                  R_shunt
///                  GND
/// ```
///
/// Formulas (matched, Z_in = Z_out = Z):
/// - K = 10^(dB/20)
/// - R_series = Z × (K − 1) / (K + 1)
/// - R_shunt  = Z × 2K / (K² − 1)
///
/// # Errors
/// Returns [`CalcError::OutOfRange`] if `attenuation_db` ≤ 0 or `z_ohm` ≤ 0.
pub fn t_pad(attenuation_db: f64, z_ohm: f64) -> Result<AttenuatorResult, CalcError> {
    let k = attenuator_inputs(attenuation_db, z_ohm)?;
    Ok(AttenuatorResult {
        attenuation_db,
        k,
        r_series_ohm: validate::finite_result("r_series_ohm", z_ohm * (k - 1.0) / (k + 1.0))?,
        r_shunt_ohm: validate::finite_result("r_shunt_ohm", z_ohm * 2.0 * k / (k * k - 1.0))?,
    })
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    /// Input impedance and insertion loss of a two-port from its ABCD matrix
    /// terminated in `z` at both ends.
    fn abcd_check(a: f64, b: f64, c: f64, d: f64, z: f64) -> (f64, f64) {
        let z_in = (a * z + b) / (c * z + d);
        let s21 = 2.0 / (a + b / z + c * z + d);
        (z_in, -20.0 * s21.log10())
    }

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
    }

    // Saturn PDF page 21: LED: Vs=12V, Vled=2V, Iled=10mA → R=1000Ω, P=0.1W
    #[test]
    fn saturn_led_bias() {
        let result = led_bias(12.0, 2.0, 0.01).unwrap();
        assert_relative_eq!(result.resistance_ohm, 1000.0, epsilon = 1e-6);
        assert_relative_eq!(result.power_w, 0.1, epsilon = 1e-8);
    }

    #[test]
    fn pi_pad_is_matched_and_delivers_requested_attenuation() {
        for db in [0.5, 1.0, 3.0, 6.0, 10.0, 20.0, 40.0] {
            let pad = pi_pad(db, 50.0).unwrap();
            let r = pad.r_series_ohm;
            let y = 1.0 / pad.r_shunt_ohm;
            // shunt(Y) · series(R) · shunt(Y)
            let (a, b, c, d) = (1.0 + r * y, r, 2.0 * y + r * y * y, 1.0 + r * y);
            let (z_in, loss) = abcd_check(a, b, c, d, 50.0);
            assert_relative_eq!(z_in, 50.0, max_relative = 1e-10);
            assert_relative_eq!(loss, db, max_relative = 1e-10);
        }
    }

    #[test]
    fn t_pad_is_matched_and_delivers_requested_attenuation() {
        for db in [0.5, 1.0, 3.0, 6.0, 10.0, 20.0, 40.0] {
            let pad = t_pad(db, 50.0).unwrap();
            let r = pad.r_series_ohm;
            let y = 1.0 / pad.r_shunt_ohm;
            // series(R) · shunt(Y) · series(R)
            let (a, b, c, d) = (1.0 + r * y, 2.0 * r + r * r * y, y, 1.0 + r * y);
            let (z_in, loss) = abcd_check(a, b, c, d, 50.0);
            assert_relative_eq!(z_in, 50.0, max_relative = 1e-10);
            assert_relative_eq!(loss, db, max_relative = 1e-10);
        }
    }

    #[test]
    fn pi_pad_reference_values() {
        // 20 dB / 50 Ω: series 247.5 Ω, shunts 61.11 Ω; 1 dB: series 5.769 Ω, shunts 869.5 Ω.
        let p20 = pi_pad(20.0, 50.0).unwrap();
        assert_relative_eq!(p20.r_series_ohm, 247.5, max_relative = 1e-12);
        assert_relative_eq!(p20.r_shunt_ohm, 61.1111111, max_relative = 1e-7);
        let p1 = pi_pad(1.0, 50.0).unwrap();
        assert_relative_eq!(p1.r_series_ohm, 5.76919, max_relative = 1e-5);
        assert_relative_eq!(p1.r_shunt_ohm, 869.548, max_relative = 1e-5);
    }

    #[test]
    fn t_pad_10db_50ohm() {
        let result = t_pad(10.0, 50.0).unwrap();
        assert_relative_eq!(result.r_series_ohm, 25.97, epsilon = 0.02);
        assert_relative_eq!(result.r_shunt_ohm, 35.14, epsilon = 0.02);
    }

    #[test]
    fn combinations() {
        assert_relative_eq!(
            resistors_series(&[100.0, 200.0]).unwrap().resistance_ohm,
            300.0,
            epsilon = 1e-10
        );
        assert_relative_eq!(
            resistors_parallel(&[100.0, 100.0]).unwrap().resistance_ohm,
            50.0,
            epsilon = 1e-10
        );
        assert_relative_eq!(
            capacitors_parallel(&[10e-12, 10e-12])
                .unwrap()
                .capacitance_f,
            20e-12,
            epsilon = 1e-22
        );
        assert_relative_eq!(
            capacitors_series(&[10e-12, 10e-12]).unwrap().capacitance_f,
            5e-12,
            epsilon = 1e-22
        );
        assert_relative_eq!(
            inductors_series(&[1e-6, 2e-6]).unwrap().inductance_h,
            3e-6,
            epsilon = 1e-16
        );
        assert_relative_eq!(
            inductors_parallel(&[10e-6, 10e-6]).unwrap().inductance_h,
            5e-6,
            epsilon = 1e-16
        );
    }

    #[test]
    fn combinations_reject_nonpositive_and_nonfinite() {
        assert!(resistors_parallel(&[100.0, 0.0]).is_err());
        assert!(resistors_parallel(&[100.0, -100.0]).is_err());
        assert!(resistors_series(&[100.0, f64::NAN]).is_err());
        assert!(capacitors_series(&[1e-12, f64::INFINITY]).is_err());
        assert!(inductors_parallel(&[]).is_err());
    }

    #[test]
    fn error_on_zero_attenuation_and_nonfinite() {
        assert!(pi_pad(0.0, 50.0).is_err());
        assert!(t_pad(0.0, 50.0).is_err());
        assert!(pi_pad(f64::NAN, 50.0).is_err());
        assert!(t_pad(3.0, f64::INFINITY).is_err());
    }

    #[test]
    fn error_on_insufficient_or_nonfinite_eir_inputs() {
        assert!(eir(Some(12.0), None, None).is_err());
        assert!(eir(Some(12.0), Some(1.0), Some(12.0)).is_err());
        assert!(eir(Some(f64::NAN), Some(1.0), None).is_err());
        assert!(eir(Some(12.0), Some(0.0), None).is_err());
    }
}
