//! Capacitive/inductive reactance and resonant frequency calculator.
//!
//! Xc = 1 / (2πfC)
//! Xl = 2πfL
//! f_res = 1 / (2π√(LC))

use serde::{Deserialize, Serialize};

use crate::CalcError;

/// Result of a reactance calculation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReactanceResult {
    /// Capacitive reactance in Ohms. None if capacitance was not provided.
    pub xc_ohms: Option<f64>,
    /// Inductive reactance in Ohms. None if inductance was not provided.
    pub xl_ohms: Option<f64>,
    /// Resonant frequency in Hz. None if both L and C were not provided.
    pub f_res_hz: Option<f64>,
}

/// Calculate capacitive reactance, inductive reactance, and resonant frequency.
///
/// # Arguments
/// - `freq_hz` — frequency in Hz (must be > 0)
/// - `capacitance_f` — capacitance in Farads (None to skip Xc and f_res)
/// - `inductance_h` — inductance in Henries (None to skip Xl and f_res)
///
/// # Errors
/// Returns [`CalcError::OutOfRange`] if `freq_hz` ≤ 0 or either value is ≤ 0.
pub fn reactance(
    freq_hz: f64,
    capacitance_f: Option<f64>,
    inductance_h: Option<f64>,
) -> Result<ReactanceResult, CalcError> {
    if freq_hz <= 0.0 {
        return Err(CalcError::OutOfRange {
            name: "freq_hz",
            value: freq_hz,
            expected: "> 0",
        });
    }

    if let Some(c) = capacitance_f {
        if c <= 0.0 {
            return Err(CalcError::OutOfRange {
                name: "capacitance_f",
                value: c,
                expected: "> 0",
            });
        }
    }

    if let Some(l) = inductance_h {
        if l <= 0.0 {
            return Err(CalcError::OutOfRange {
                name: "inductance_h",
                value: l,
                expected: "> 0",
            });
        }
    }

    let two_pi_f = 2.0 * std::f64::consts::PI * freq_hz;

    let xc_ohms = capacitance_f.map(|c| 1.0 / (two_pi_f * c));
    let xl_ohms = inductance_h.map(|l| two_pi_f * l);

    let f_res_hz = match (capacitance_f, inductance_h) {
        (Some(c), Some(l)) => Some(1.0 / (2.0 * std::f64::consts::PI * (l * c).sqrt())),
        _ => None,
    };

    Ok(ReactanceResult {
        xc_ohms,
        xl_ohms,
        f_res_hz,
    })
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    // Saturn PDF page 41: f=1 MHz, C=1 µF, L=1 mH
    // Xc=0.1592 Ω, Xl=6283.1800 Ω, f_res=5032.9255 Hz
    #[test]
    fn saturn_page41_vector() {
        let result = reactance(1e6, Some(1e-6), Some(1e-3)).unwrap();
        assert_relative_eq!(result.xc_ohms.unwrap(), 0.15915494, epsilon = 1e-4);
        assert_relative_eq!(result.xl_ohms.unwrap(), 6283.1853, epsilon = 1e-1);
        assert_relative_eq!(result.f_res_hz.unwrap(), 5032.9255, epsilon = 1e-2);
    }

    // C=10nF, f=1kHz → Xc = 15915 Ω
    #[test]
    fn xc_10nf_1khz() {
        let result = reactance(1e3, Some(10e-9), None).unwrap();
        assert_relative_eq!(result.xc_ohms.unwrap(), 15915.494, epsilon = 1e-1);
    }

    // L=100µH, f=1kHz → Xl = 0.6283 Ω
    #[test]
    fn xl_100uh_1khz() {
        let result = reactance(1e3, None, Some(100e-6)).unwrap();
        assert_relative_eq!(result.xl_ohms.unwrap(), 0.6283185, epsilon = 1e-4);
    }

    #[test]
    fn error_on_zero_freq() {
        assert!(reactance(0.0, Some(1e-6), None).is_err());
    }

    #[test]
    fn error_on_negative_capacitance() {
        assert!(reactance(1e6, Some(-1e-6), None).is_err());
    }
}
