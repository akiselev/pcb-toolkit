//! PPM / frequency conversion and crystal load capacitor calculator.
//!
//! Sub-calculators:
//! 1. XTAL load capacitance: C_load = (C1×C2)/(C1+C2) + C_stray
//! 2. Hz to PPM: PPM = (variation / center_freq) × 1,000,000
//! 3. PPM to Hz: variation = center_freq × PPM / 1,000,000

use serde::{Deserialize, Serialize};

use crate::model::{ModelInfo, ModelStatus};
use crate::{CalcError, validate};

/// Model description.
pub const MODEL: ModelInfo = ModelInfo {
    name: "PPM conversion and Pierce-oscillator crystal load capacitance",
    status: ModelStatus::Validated,
    reference: "TI SWRA372 (AN100) §Load capacitance; Saturn PCB Toolkit help p.32",
    validity: "Exact relations; stray capacitance modelled as one lumped value across the crystal terminals",
};

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
    /// Load capacitance seen by the crystal: (C1×C2)/(C1+C2) + C_stray, in Farads.
    /// Compare this with the crystal datasheet's specified C_L.
    pub c_load_f: f64,
    /// Arithmetic mean of the two external capacitors, (C1 + C2)/2, in Farads.
    ///
    /// This is the value Saturn PCB Toolkit displays as "rule of thumb". It is
    /// **not** the crystal load capacitance (it ignores the series
    /// combination and the stray term) and is provided only for
    /// compatibility with that display.
    pub c_external_average_f: f64,
}

/// Convert frequency deviation to PPM.
///
/// # Errors
/// Returns an error unless `center_hz > 0` and `max_hz > center_hz`.
pub fn hz_to_ppm(center_hz: f64, max_hz: f64) -> Result<HzToPpmResult, CalcError> {
    validate::positive_quantity("center_hz", center_hz)?;
    validate::finite("max_hz", max_hz)?;
    if max_hz <= center_hz {
        return Err(CalcError::OutOfRange {
            name: "max_hz",
            value: max_hz,
            expected: "> center_hz",
        });
    }
    let variation_hz = max_hz - center_hz;
    Ok(HzToPpmResult {
        variation_hz,
        ppm: validate::finite_result("ppm", (variation_hz / center_hz) * 1_000_000.0)?,
    })
}

/// Convert PPM to frequency deviation.
///
/// # Errors
/// Returns an error unless `center_hz > 0` and `ppm > 0`.
pub fn ppm_to_hz(center_hz: f64, ppm: f64) -> Result<PpmToHzResult, CalcError> {
    validate::positive_quantity("center_hz", center_hz)?;
    validate::positive_quantity("ppm", ppm)?;
    let variation_hz = validate::finite_result("variation_hz", center_hz * ppm / 1_000_000.0)?;
    Ok(PpmToHzResult {
        variation_hz,
        max_hz: center_hz + variation_hz,
        min_hz: center_hz - variation_hz,
    })
}

/// Calculate the load capacitance presented to a crystal in a Pierce
/// oscillator with external capacitors `c1_f` and `c2_f` from each crystal
/// pin to ground.
///
/// `c_stray_f` is a single lumped stray capacitance across the crystal
/// terminals (pins, package, PCB), added to the series combination of C1 and
/// C2. If stray capacitance is known per leg instead, add it to `c1_f` and
/// `c2_f` and pass 0 here.
///
/// # Errors
/// Returns an error unless `c_stray_f ≥ 0`, `c1_f > 0` and `c2_f > 0`.
pub fn xtal_load(c_stray_f: f64, c1_f: f64, c2_f: f64) -> Result<XtalLoadResult, CalcError> {
    validate::non_negative("c_stray_f", c_stray_f)?;
    validate::positive_quantity("c1_f", c1_f)?;
    validate::positive_quantity("c2_f", c2_f)?;

    let c_series = (c1_f * c2_f) / (c1_f + c2_f);
    Ok(XtalLoadResult {
        c_load_f: validate::finite_result("c_load_f", c_series + c_stray_f)?,
        c_external_average_f: validate::finite_result("c_external_average_f", (c1_f + c2_f) / 2.0)?,
    })
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    // Saturn PDF page 32: center=32000 Hz, max=32001 Hz → variation=1 Hz, PPM=31.25
    #[test]
    fn saturn_hz_to_ppm() {
        let result = hz_to_ppm(32000.0, 32001.0).unwrap();
        assert_relative_eq!(result.variation_hz, 1.0, epsilon = 1e-10);
        assert_relative_eq!(result.ppm, 31.25, epsilon = 1e-6);
    }

    // Saturn PDF page 32: center=50 MHz, PPM=25 → variation=1250 Hz
    #[test]
    fn saturn_ppm_to_hz() {
        let result = ppm_to_hz(50e6, 25.0).unwrap();
        assert_relative_eq!(result.variation_hz, 1250.0, epsilon = 1e-6);
        assert_relative_eq!(result.max_hz, 50_001_250.0, epsilon = 1e-4);
        assert_relative_eq!(result.min_hz, 49_998_750.0, epsilon = 1e-4);
    }

    // Saturn PDF page 32: C_stray=3pF, C1=14pF, C2=14pF → load 10pF; Saturn "rule of thumb" 14pF
    #[test]
    fn saturn_xtal_load() {
        let result = xtal_load(3e-12, 14e-12, 14e-12).unwrap();
        assert_relative_eq!(result.c_load_f, 10e-12, epsilon = 1e-24);
        assert_relative_eq!(result.c_external_average_f, 14e-12, epsilon = 1e-24);
    }

    #[test]
    fn errors() {
        assert!(hz_to_ppm(0.0, 100.0).is_err());
        assert!(ppm_to_hz(0.0, 10.0).is_err());
        assert!(hz_to_ppm(1000.0, 999.0).is_err());
        assert!(hz_to_ppm(1000.0, 1000.0).is_err());
        assert!(hz_to_ppm(f64::NAN, 1000.0).is_err());
        assert!(xtal_load(-1e-12, 14e-12, 14e-12).is_err());
        assert!(xtal_load(0.0, f64::INFINITY, 14e-12).is_err());
    }
}
