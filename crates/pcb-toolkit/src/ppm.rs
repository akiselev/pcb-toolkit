//! PPM / frequency conversion and crystal load capacitor calculator.
//!
//! Sub-calculators:
//! 1. XTAL load capacitor value: C_load = (C1×C2)/(C1+C2) + C_stray
//! 2. Hz to PPM: PPM = (variation / center_freq) × 1,000,000
//! 3. PPM to Hz: variation = center_freq × PPM / 1,000,000

use serde::{Deserialize, Serialize};

use crate::CalcError;

/// Result of a Hz→PPM conversion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HzToPpmResult {
    /// Frequency variation in Hz (max_hz - center_hz).
    pub variation_hz: f64,
    /// Variation expressed in parts per million.
    pub ppm: f64,
}

/// Result of a PPM→Hz conversion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpmToHzResult {
    /// Frequency variation in Hz.
    pub variation_hz: f64,
    /// Upper frequency limit: center + variation.
    pub max_hz: f64,
    /// Lower frequency limit: center - variation.
    pub min_hz: f64,
}

/// Result of a crystal load capacitor calculation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XtalLoadResult {
    /// Calculated load capacitance: (C1×C2)/(C1+C2) + C_stray, in Farads.
    pub c_load_calc_f: f64,
    /// Rule-of-thumb load capacitance: (C1 + C2) / 2, in Farads.
    pub c_load_rule_of_thumb_f: f64,
}

/// Convert frequency deviation to PPM.
///
/// # Arguments
/// - `center_hz` — nominal (center) frequency in Hz (must be > 0)
/// - `max_hz` — upper frequency limit in Hz (must be > center_hz)
///
/// # Errors
/// Returns [`CalcError::OutOfRange`] if inputs are invalid.
pub fn hz_to_ppm(center_hz: f64, max_hz: f64) -> Result<HzToPpmResult, CalcError> {
    if center_hz <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "center_hz",
            value: center_hz,
            expected: "> 0",
        });
    }
    if max_hz <= center_hz {
        return Err(CalcError::OutOfRange {
            name: "max_hz",
            value: max_hz,
            expected: "> center_hz",
        });
    }

    let variation_hz = max_hz - center_hz;
    let ppm = (variation_hz / center_hz) * 1_000_000.0;

    Ok(HzToPpmResult { variation_hz, ppm })
}

/// Convert PPM to frequency deviation.
///
/// # Arguments
/// - `center_hz` — nominal (center) frequency in Hz (must be > 0)
/// - `ppm` — parts-per-million deviation (must be > 0)
///
/// # Errors
/// Returns [`CalcError::OutOfRange`] if inputs are invalid.
pub fn ppm_to_hz(center_hz: f64, ppm: f64) -> Result<PpmToHzResult, CalcError> {
    if center_hz <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "center_hz",
            value: center_hz,
            expected: "> 0",
        });
    }
    if ppm <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "ppm",
            value: ppm,
            expected: "> 0",
        });
    }

    let variation_hz = center_hz * ppm / 1_000_000.0;

    Ok(PpmToHzResult {
        variation_hz,
        max_hz: center_hz + variation_hz,
        min_hz: center_hz - variation_hz,
    })
}

/// Calculate crystal load capacitance.
///
/// # Arguments
/// - `c_load_spec_f` — specified load capacitance from the crystal datasheet, in Farads
/// - `c_stray_f` — stray PCB capacitance in Farads (must be ≥ 0)
/// - `c1_f` — load capacitor 1 in Farads (must be > 0)
/// - `c2_f` — load capacitor 2 in Farads (must be > 0)
///
/// # Errors
/// Returns [`CalcError::OutOfRange`] if inputs are invalid.
pub fn xtal_load(
    c_stray_f: f64,
    c1_f: f64,
    c2_f: f64,
) -> Result<XtalLoadResult, CalcError> {
    if c_stray_f < 0.0 {
        return Err(CalcError::OutOfRange {
            name: "c_stray_f",
            value: c_stray_f,
            expected: ">= 0",
        });
    }
    if c1_f <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "c1_f",
            value: c1_f,
            expected: "> 0",
        });
    }
    if c2_f <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "c2_f",
            value: c2_f,
            expected: "> 0",
        });
    }

    let c_series = (c1_f * c2_f) / (c1_f + c2_f);
    let c_load_calc_f = c_series + c_stray_f;
    let c_load_rule_of_thumb_f = (c1_f + c2_f) / 2.0;

    Ok(XtalLoadResult {
        c_load_calc_f,
        c_load_rule_of_thumb_f,
    })
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    // Saturn PDF page 32: Hz→PPM
    // center=32000 Hz, max=32001 Hz → variation=1 Hz, PPM=31.25
    #[test]
    fn saturn_hz_to_ppm() {
        let result = hz_to_ppm(32000.0, 32001.0).unwrap();
        assert_relative_eq!(result.variation_hz, 1.0, epsilon = 1e-10);
        assert_relative_eq!(result.ppm, 31.25, epsilon = 1e-6);
    }

    // Saturn PDF page 32: PPM→Hz
    // center=50 MHz, PPM=25 → variation=1250 Hz, max=50001250, min=49998750
    #[test]
    fn saturn_ppm_to_hz() {
        let result = ppm_to_hz(50e6, 25.0).unwrap();
        assert_relative_eq!(result.variation_hz, 1250.0, epsilon = 1e-6);
        assert_relative_eq!(result.max_hz, 50_001_250.0, epsilon = 1e-4);
        assert_relative_eq!(result.min_hz, 49_998_750.0, epsilon = 1e-4);
    }

    // Saturn PDF page 32: XTAL load caps
    // C_stray=3pF, C1=14pF, C2=14pF → calc=10pF, rule_of_thumb=14pF
    #[test]
    fn saturn_xtal_load() {
        let result = xtal_load(3e-12, 14e-12, 14e-12).unwrap();
        assert_relative_eq!(result.c_load_calc_f, 10e-12, epsilon = 1e-14);
        assert_relative_eq!(result.c_load_rule_of_thumb_f, 14e-12, epsilon = 1e-14);
    }

    #[test]
    fn error_on_zero_center_freq() {
        assert!(hz_to_ppm(0.0, 100.0).is_err());
        assert!(ppm_to_hz(0.0, 10.0).is_err());
    }

    #[test]
    fn error_on_max_not_greater_than_center() {
        assert!(hz_to_ppm(1000.0, 999.0).is_err());
        assert!(hz_to_ppm(1000.0, 1000.0).is_err());
    }
}
